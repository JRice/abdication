//! The drum engine: a few copies of the Faust voice taking hits in turn, so
//! a new hit never cuts off a ringing one (which would click); one copy of
//! the effects on their sum (so a compressor ducks the hit still ringing and
//! Drive glues the hits together); choke; and an output limiter. No nih-plug
//! types here, so it can be tested on its own.
//!
//! Each voice's Body and Noise are their own DSPs (plugin-body.dsp,
//! plugin-noise.dsp): the voice works out their inputs, and each is only
//! computed while it sounds. The Body takes its settings every 16 samples, so
//! its 24 modes' coefficients aren't recomputed per sample: it was 90 % of a
//! voice's CPU.

use faust_types::{FaustDsp, ParamIndex};

use crate::body::BodyDsp;
use crate::dsp::VoiceDsp;
use crate::noise::NoiseDsp;
use crate::fx::FxDsp;
use crate::params::{Target, VoiceSlots};
use crate::scope::{Recorder, PARTS};

/// Hits ring on independently up to this many deep; the next hit takes the
/// voice that has been ringing longest. Five, not four: with four, in 4/4
/// the hit whose tail gets cut fell on the same beat every bar.
pub const NUM_VOICES: usize = 5;
pub const MAX_BLOCK: usize = 64;

/// A voice whose output stays below this for `FREE_AFTER` blocks is
/// finished and stops being computed.
const SILENCE: f32 = 1e-5;
const FREE_AFTER: u32 = 32;
/// The voice's outputs (voice.lib): Tone, Sample 1, Sample 2; the Body's
/// exciter and level; the Noise's level; the Body's settings (pitch,
/// material, decay, brightness, position); the Noise's inputs; then the
/// Body's Clip, and the Noise's Bite, envelope, part level, Spread and whether it's set.
const VOICE_OUTS: usize = 27;
const TONE: usize = 0;
const SAMPLE_1: usize = 1;
const SAMPLE_2: usize = 2;
const EXCITER: usize = 3;
const BODY_LEVEL: usize = 4;
const NOISE_LEVEL: usize = 5;
const BODY_SETTINGS: usize = 6;
const NOISE_INPUTS: usize = 11;
/// The Noise's inputs from the voice (NOISE_INPUTS on): ten, then (after the
/// Body's Clip) its Bite, envelope, part level, Spread and its flag; its level goes last.
const NOISE_FIRST: usize = 10;
const BODY_CLIP: usize = 21;
const NOISE_LATER: usize = 22;
const NOISE_DSP_INPUTS: usize = 16;
/// Below this level the Body or Noise is silent and isn't computed.
const PART_ON: f32 = 1e-6;
/// The Body's settings are taken (and its coefficients computed) this often, in samples.
pub const BODY_STEP: usize = 16;
/// A choke fades the ringing hits (and the effects' tails) out over about this long.
const CHOKE_S: f32 = 0.005;
/// With Keytrack on, this note plays at Tone's Pitch (as in voice.lib).
const KEYTRACK_REF_HZ: f32 = 261.6256;

/// Equal-tempered pitch of a MIDI note (A4 = 440 Hz).
pub fn note_freq(note: u8) -> f32 {
    440.0 * 2f32.powf((note as f32 - 69.0) / 12.0)
}

struct Voice {
    dsp: Box<VoiceDsp>,
    body: Box<BodyDsp>,
    noise: Box<NoiseDsp>,
    /// The Body was computed last block (else it starts clean when it's next needed).
    body_running: bool,
    /// The Body's next block starts a hit: its coefficients jump rather than glide.
    body_snap: bool,
    started: u64,
    /// The gate is held for one sub-block after a hit: drums ignore its
    /// length, but the Faust voice needs a rising edge to start.
    gate_open: bool,
    active: bool,
    quiet_blocks: u32,
    /// 1 while playing; falls towards 0 once choked.
    level: f32,
    choked: bool,
}

pub struct Engine {
    voices: Vec<Voice>,
    fx: Box<FxDsp>,
    slots: VoiceSlots,
    clock: u64,
    rng: u32,
    /// The voice's outputs, the Body's and the Noise's.
    scratch: [[f32; MAX_BLOCK]; VOICE_OUTS],
    body_out: [f32; MAX_BLOCK],
    noise_out: [f32; MAX_BLOCK],
    /// The latest hit's parts, for the panel's scopes.
    pub recorder: Recorder,
    mix: [f32; MAX_BLOCK],
    choke_fall: f32,
    /// Tone's Pitch and Keytrack, and the latest note: the effects' tuning.
    tone_pitch: f32,
    keytrack: bool,
    last_note_hz: f32,
    /// The effects' hit pulse is held for one sub-block, like a voice's gate.
    hit_open: bool,
    /// The effects' output level: 1, falling once choked (their tails ring on).
    fx_level: f32,
    fx_choked: bool,
}

impl Engine {
    pub fn new(slots: VoiceSlots) -> Self {
        let voices = (0..NUM_VOICES)
            .map(|_| Voice {
                dsp: Box::new(VoiceDsp::new()),
                body: Box::new(BodyDsp::new()),
                noise: Box::new(NoiseDsp::new()),
                body_running: false,
                body_snap: false,
                started: 0,
                gate_open: false,
                active: false,
                quiet_blocks: 0,
                level: 1.0,
                choked: false,
            })
            .collect();
        Self {
            voices,
            fx: Box::new(FxDsp::new()),
            slots,
            clock: 0,
            rng: 0x9E37_79B9,
            scratch: [[0.0; MAX_BLOCK]; VOICE_OUTS],
            body_out: [0.0; MAX_BLOCK],
            noise_out: [0.0; MAX_BLOCK],
            recorder: Recorder::default(),
            mix: [0.0; MAX_BLOCK],
            choke_fall: 0.0,
            tone_pitch: 50.0,
            keytrack: false,
            last_note_hz: KEYTRACK_REF_HZ,
            hit_open: false,
            fx_level: 1.0,
            fx_choked: false,
        }
    }

    pub fn init(&mut self, sample_rate: f32) {
        self.choke_fall = (-6.9 / (CHOKE_S * sample_rate)).exp();
        for v in &mut self.voices {
            v.dsp.init(sample_rate as i32);
            v.body.init(sample_rate as i32);
            v.noise.init(sample_rate as i32);
        }
        self.fx.init(sample_rate as i32);
        self.recorder.init(sample_rate);
        self.reset();
    }

    pub fn reset(&mut self) {
        for v in &mut self.voices {
            v.dsp.instance_clear();
            v.body.instance_clear();
            v.noise.instance_clear();
            v.body_running = false;
            v.dsp.set_param(self.slots.gate, 0.0);
            v.active = false;
            v.gate_open = false;
            v.choked = false;
            v.level = 1.0;
        }
        self.fx.instance_clear();
        self.fx.set_param(self.slots.hit, 0.0);
        self.hit_open = false;
        self.fx_level = 1.0;
        self.fx_choked = false;
        self.recorder.stop();
    }

    /// Set a host parameter's Faust control: on every voice, or on the effects.
    pub fn set_param(&mut self, target: Target, param: ParamIndex, value: f32) {
        match target {
            Target::Voice => self.voices.iter_mut().for_each(|v| v.dsp.set_param(param, value)),
            Target::Fx => self.fx.set_param(param, value),
            Target::Body => {} // fed by the voice, not by host parameters
        }
    }

    /// Everything the engine takes from the parameters, as the plugin does
    /// every block: every Faust control, the effects' tuning and envelope.
    pub fn apply(&mut self, params: &crate::params::DrumParams) {
        for (target, index, value) in params.faust_values() {
            self.set_param(target, index, value);
        }
        self.set_tone(params.value("tone.pitch").unwrap_or(50.0), params.value("tone.keytrack").unwrap_or(0.0) > 0.5);
        self.set_envelope(params.envelope());
    }

    /// The voice's Envelope settings, passed on to the effects' sends.
    pub fn set_envelope(&mut self, settings: [f32; 4]) {
        for (index, value) in self.slots.envelope.into_iter().zip(settings) {
            self.fx.set_param(index, value);
        }
    }

    /// Tone's Pitch and Keytrack, which tune Ring mod and Comb.
    pub fn set_tone(&mut self, pitch: f32, keytrack: bool) {
        self.tone_pitch = pitch;
        self.keytrack = keytrack;
    }

    /// The pitch the effects are tuned to: Tone's, keytracked to the latest note.
    fn tune(&self) -> f32 {
        if self.keytrack { self.tone_pitch * self.last_note_hz / KEYTRACK_REF_HZ } else { self.tone_pitch }
    }

    /// A hit: the note sets the pitch only when Tone's Keytrack is on.
    pub fn note_on(&mut self, note: u8, velocity: f32) {
        self.clock += 1;
        let (slots, seed, i) = (self.slots, self.random(), self.pick_voice());
        let v = &mut self.voices[i];
        // A voice taken while choked starts clean, not from its faded state.
        if v.choked {
            v.dsp.instance_clear();
            v.body.instance_clear();
            v.noise.instance_clear();
            v.choked = false;
        }
        v.body_snap = true;
        v.dsp.set_param(slots.freq, note_freq(note));
        v.dsp.set_param(slots.gain, velocity);
        v.dsp.set_param(slots.seed, seed);
        v.dsp.set_param(slots.gate, 1.0);
        v.started = self.clock;
        v.gate_open = true;
        v.active = true;
        v.quiet_blocks = 0;
        v.level = 1.0;
        self.recorder.start(i);
        // The effects restart their per-hit parts (Ring mod's carrier) and,
        // if a choke silenced them, start clean.
        if self.fx_choked {
            self.fx.instance_clear();
            self.fx_choked = false;
        }
        self.fx_level = 1.0;
        self.last_note_hz = note_freq(note);
        self.fx.set_param(slots.hit, 1.0);
        self.fx.set_param(slots.vel, velocity);
        self.hit_open = true;
    }

    /// Choke (another pad in the Drum Machine's choke group, or the host):
    /// every ringing hit, and the effects' tails, fade out over a few milliseconds.
    pub fn choke(&mut self) {
        for v in self.voices.iter_mut().filter(|v| v.active) {
            v.choked = true;
        }
        self.fx_choked = true;
    }

    /// Render up to MAX_BLOCK samples of the pad (voices, then effects) into
    /// `out` (overwritten, not summed).
    pub fn render(&mut self, out: &mut [f32]) {
        let n = out.len().min(MAX_BLOCK);
        let (gate, fall, slots) = (self.slots.gate, self.choke_fall, self.slots);
        let mix = &mut self.mix[..n];
        mix.fill(0.0);
        for (index, v) in self.voices.iter_mut().enumerate().filter(|(_, v)| v.active) {
            v.dsp.compute(n as i32, &[], &mut self.scratch.each_mut().map(|b| &mut b[..n]));
            if v.gate_open {
                v.dsp.set_param(gate, 0.0);
                v.gate_open = false;
            }
            Self::run_body(v, &slots, &self.scratch, &mut self.body_out[..n]);
            Self::run_noise(v, &self.scratch, &mut self.noise_out[..n]);
            // A voice that ever produces a non-finite sample is cleared on the
            // spot, so a numerical fault can cost a click, never a dead voice.
            let faulty = |b: &[f32]| b.iter().any(|s| !s.is_finite());
            if self.scratch.iter().any(|p| faulty(&p[..n])) || faulty(&self.body_out[..n]) || faulty(&self.noise_out[..n]) {
                v.dsp.instance_clear();
                v.body.instance_clear();
                v.noise.instance_clear();
                self.scratch.iter_mut().for_each(|p| p[..n].fill(0.0));
                self.body_out[..n].fill(0.0);
                self.noise_out[..n].fill(0.0);
            }
            let recording = self.recorder.voice == Some(index);
            let mut peak = 0.0f32;
            for (k, o) in mix.iter_mut().enumerate() {
                if v.choked {
                    v.level *= fall;
                }
                // Tone, Body, Noise, Sample 1, Sample 2.
                let parts = [self.scratch[TONE][k], self.body_out[k], self.noise_out[k], self.scratch[SAMPLE_1][k], self.scratch[SAMPLE_2][k]];
                let part: [f32; PARTS] = parts.map(|s| s * v.level);
                let y: f32 = part.iter().sum();
                if recording {
                    self.recorder.push(part);
                }
                *o += y;
                peak = peak.max(y.abs());
            }
            if peak < SILENCE {
                v.quiet_blocks += 1;
                if v.quiet_blocks >= FREE_AFTER {
                    v.active = false;
                    if recording {
                        self.recorder.stop();
                    }
                }
            } else {
                v.quiet_blocks = 0;
            }
        }
        self.recorder.publish();

        // The effects always run: their tails (Comb, Compressor release)
        // outlast the voices.
        self.fx.set_param(self.slots.tune, self.tune());
        self.fx.compute(n as i32, &[&self.mix[..n]], &mut [&mut out[..n]]);
        if self.hit_open {
            self.fx.set_param(self.slots.hit, 0.0);
            self.hit_open = false;
        }
        if out[..n].iter().any(|s| !s.is_finite()) {
            self.fx.instance_clear();
            out[..n].fill(0.0);
        }
        if self.fx_choked {
            for o in out[..n].iter_mut() {
                self.fx_level *= fall;
                *o *= self.fx_level;
            }
        }
    }

    /// The voice's Body for this block, from the voice's outputs: computed
    /// only while its level is up (starting clean after a rest), with its
    /// settings taken from the block's last sample.
    fn run_body(v: &mut Voice, slots: &VoiceSlots, voice_out: &[[f32; MAX_BLOCK]; VOICE_OUTS], out: &mut [f32]) {
        let n = out.len();
        if !voice_out[BODY_LEVEL][..n].iter().any(|g| g.abs() > PART_ON) {
            v.body_running = false;
            out.fill(0.0);
            return;
        }
        if !v.body_running {
            v.body.instance_clear();
            v.body_snap = true;
        }
        for (step, chunk) in out.chunks_mut(BODY_STEP).enumerate() {
            let (from, to) = (step * BODY_STEP, step * BODY_STEP + chunk.len());
            for (k, index) in slots.body.iter().enumerate() {
                v.body.set_param(*index, voice_out[BODY_SETTINGS + k][to - 1]);
            }
            v.body.set_param(slots.body_clip, voice_out[BODY_CLIP][to - 1]);
            v.body.set_param(slots.body_gate, if v.body_snap { 1.0 } else { 0.0 });
            v.body.compute(chunk.len() as i32, &[&voice_out[EXCITER][from..to], &voice_out[BODY_LEVEL][from..to]], &mut [chunk]);
            if v.body_snap {
                v.body.set_param(slots.body_gate, 0.0);
                v.body_snap = false;
            }
        }
        v.body_running = true;
    }

    /// The voice's Noise for this block, computed only while it sounds. It
    /// picks up where it left off (never cleared on the way back in, or every
    /// hit would play the same noise).
    fn run_noise(v: &mut Voice, voice_out: &[[f32; MAX_BLOCK]; VOICE_OUTS], out: &mut [f32]) {
        let n = out.len();
        if !voice_out[NOISE_LEVEL][..n].iter().any(|g| g.abs() > PART_ON) {
            out.fill(0.0);
            return;
        }
        let inputs: [&[f32]; NOISE_DSP_INPUTS] = std::array::from_fn(|k| match k {
            k if k < NOISE_FIRST => &voice_out[NOISE_INPUTS + k][..n],
            k if k < NOISE_DSP_INPUTS - 1 => &voice_out[NOISE_LATER + k - NOISE_FIRST][..n],
            _ => &voice_out[NOISE_LEVEL][..n],
        });
        v.noise.compute(n as i32, &inputs, &mut [out]);
    }

    pub fn active_voices(&self) -> usize {
        self.voices.iter().filter(|v| v.active).count()
    }

    /// Restart the per-hit random values from `state` (repeatable offline renders).
    pub fn reseed(&mut self, state: u32) {
        self.rng = state.max(1);
    }

    /// Uniform 0..1 from a xorshift generator (a fresh seed per hit).
    fn random(&mut self) -> f32 {
        let mut x = self.rng;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.rng = x;
        (x >> 8) as f32 / (1u32 << 24) as f32
    }

    /// A free voice first, else the one that has been ringing longest.
    fn pick_voice(&self) -> usize {
        let oldest = |pred: &dyn Fn(&Voice) -> bool| {
            self.voices.iter().enumerate().filter(|(_, v)| pred(v)).min_by_key(|(_, v)| v.started).map(|(i, _)| i)
        };
        oldest(&|v| !v.active).or_else(|| oldest(&|_| true)).expect("at least one voice")
    }
}

/// Basic peak limiter for the output (Abdication's): instant attack,
/// exponential release, so the output can't exceed the ceiling.
pub struct Limiter {
    ceiling: f32,
    release: f32,
    env: f32,
}

impl Default for Limiter {
    fn default() -> Self {
        Self::new()
    }
}

impl Limiter {
    pub const CEILING_DB: f32 = -0.5;
    pub const RELEASE_S: f32 = 0.05;

    pub fn new() -> Self {
        Self { ceiling: 10f32.powf(Self::CEILING_DB / 20.0), release: 0.0, env: 0.0 }
    }

    pub fn init(&mut self, sample_rate: f32) {
        self.release = (-1.0 / (Self::RELEASE_S * sample_rate)).exp();
        self.env = 0.0;
    }

    pub fn process(&mut self, x: f32) -> f32 {
        // A NaN or infinity would poison the envelope for good; drop it.
        if !x.is_finite() {
            return 0.0;
        }
        self.env = x.abs().max(self.env * self.release);
        if self.env > self.ceiling {
            x * self.ceiling / self.env
        } else {
            x
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::params::DrumParams;

    fn engine() -> (Engine, DrumParams) {
        let params = DrumParams::from_faust();
        let mut e = Engine::new(params.slots());
        e.init(44_100.0);
        apply_all(&mut e, &params);
        (e, params)
    }

    /// What the plugin does every block: every parameter, plus the effects' tuning.
    pub(super) fn apply_all(e: &mut Engine, params: &DrumParams) {
        e.apply(params);
    }

    /// Set one parameter by id, as the host would (and refresh the tuning).
    pub(super) fn set(e: &mut Engine, params: &DrumParams, id: &str, value: f32) {
        let (target, index) = params.faust_index(id).unwrap_or_else(|| panic!("no param {id}"));
        e.set_param(target, index, value);
        if id == "tone.pitch" {
            e.tone_pitch = value;
        }
        if id == "tone.keytrack" {
            e.keytrack = value > 0.5;
        }
        if let Some(k) = crate::params::ENVELOPE_IDS.iter().position(|e| *e == id) {
            let index = e.slots.envelope[k];
            e.fx.set_param(index, value);
        }
    }

    fn run(e: &mut Engine, samples: usize) -> Vec<f32> {
        let mut out = vec![0.0; samples];
        for chunk in out.chunks_mut(MAX_BLOCK) {
            e.render(chunk);
        }
        out
    }

    fn peak(x: &[f32]) -> f32 {
        x.iter().fold(0.0f32, |m, s| m.max(s.abs()))
    }

    #[test]
    fn silent_until_hit_then_sounds_then_frees_itself() {
        let (mut e, _) = engine();
        assert_eq!(peak(&run(&mut e, 4410)), 0.0);
        e.note_on(36, 1.0);
        let hit = run(&mut e, 44_100);
        assert!(peak(&hit) > 0.1, "default kick too quiet: {}", peak(&hit));
        assert!(hit.iter().all(|s| s.is_finite()));
        run(&mut e, 44_100 * 6);
        assert_eq!(e.active_voices(), 0, "a finished hit should stop computing");
    }

    #[test]
    fn hits_overlap_up_to_the_voice_count() {
        let (mut e, _) = engine();
        for _ in 0..NUM_VOICES + 2 {
            e.note_on(36, 0.8);
            run(&mut e, 441);
        }
        assert_eq!(e.active_voices(), NUM_VOICES);
    }

    #[test]
    fn choke_silences_quickly() {
        let (mut e, _) = engine();
        e.note_on(36, 1.0);
        run(&mut e, 2205);
        e.choke();
        let after = run(&mut e, 2205);
        assert!(peak(&after[1323..]) < 1e-3, "still sounding 30 ms after a choke");
    }

    #[test]
    fn sample_layers_play_wrap_within_their_set_and_strike_the_body() {
        let (mut e, params) = engine();
        let set = |e: &mut Engine, id: &str, v: f32| set(e, &params, id, v);
        let hit = |e: &mut Engine| {
            e.reset();
            e.note_on(60, 1.0);
            e.voices.iter_mut().for_each(|v| v.dsp.set_param(e.slots.seed, 0.5));
            run(e, 22_050)
        };
        set(&mut e, "tone.level", 0.0);
        set(&mut e, "sample_1.level", 1.0);
        set(&mut e, "sample_1.set", 1.0); // knock
        set(&mut e, "sample_1.sample", 1.0);
        let first = hit(&mut e);
        assert!(peak(&first) > 0.3, "the layer should sound: {}", peak(&first));
        let knocks = crate::samplelib::HIT_NAMES.iter().filter(|n| n.starts_with("Knock")).count();
        set(&mut e, "sample_1.sample", (knocks + 1) as f32);
        assert_eq!(first, hit(&mut e), "Sample {} should wrap to 1 in the knock set", knocks + 1);

        set(&mut e, "sample_1.level", 0.0);
        set(&mut e, "body.level", 1.0);
        set(&mut e, "body.decay", 400.0);
        let mallet = peak(&hit(&mut e)[4410..]);
        set(&mut e, "sample_1.body_send", 1.0);
        let sent = peak(&hit(&mut e)[4410..]);
        assert!(sent > 0.01 && (sent / mallet).log10().abs() < 1.5, "a full send should ring the Body like the mallet: {sent} vs {mallet}");
    }

    /// Regression: a NaN in Tube's DC filter once silenced a voice for good,
    /// so every fourth hit dropped. Every type, driven hard on a hot, layered
    /// hit, must sound on every voice.
    #[test]
    fn every_drive_type_sounds_on_every_voice() {
        let (mut e, params) = engine();
        let set = |e: &mut Engine, id: &str, v: f32| set(e, &params, id, v);
        for (id, v) in [("noise.level", 1.0), ("body.level", 1.0), ("sample_1.level", 1.0), ("sample_2.level", 1.0),
                        ("body.strike", 1.0), ("drive.amount", 1.0)] {
            set(&mut e, id, v);
        }
        for kind in 0..4 {
            set(&mut e, "drive.type", kind as f32);
            e.reset();
            for hit in 0..3 * NUM_VOICES {
                e.note_on(60, 1.0);
                let x = run(&mut e, 4410);
                assert!(x.iter().all(|s| s.is_finite()), "type {kind} hit {hit}: non-finite output");
                assert!(peak(&x) > 0.05, "type {kind} hit {hit}: silent ({})", peak(&x));
            }
        }
    }

    #[test]
    fn the_latest_hit_reaches_the_panels_scopes_part_by_part() {
        let (mut e, params) = engine();
        let share = std::sync::Arc::new(crate::scope::HitScope::default());
        e.recorder.share(share.clone());
        e.init(44_100.0);
        apply_all(&mut e, &params);
        set(&mut e, &params, "noise.level", 0.5);
        e.note_on(60, 1.0);
        run(&mut e, 4410);
        let (seen, traces) = share.traces_since((0, 0), 100).expect("the hit was shared");
        assert_eq!(seen.1, 4410);
        let on: Vec<bool> = traces.iter().map(|t| !t.columns.is_empty()).collect();
        assert_eq!(on, [true, false, true, false, false], "Tone and Noise sound; the rest are off");
        // The parts add up to what the pad played (before the effects, all off here).
        e.note_on(60, 1.0);
        let played = run(&mut e, 64);
        let capture_sum = {
            let (_, t) = share.traces_since(seen, 64).unwrap();
            t.iter().map(|t| t.peak).fold(0.0f32, f32::max)
        };
        assert!(peak(&played) > 0.0 && capture_sum > 0.0);
    }

    /// Regression: turning a crush back off left it engaged at a sliver
    /// above 0 (smoothing never lands exactly on 0), where its sample rate
    /// rounds to the host's and the sample-and-hold froze: silence.
    #[test]
    fn crush_turned_back_off_lets_the_pad_sound() {
        for id in ["crush.amount", "noise.crush"] {
            let (mut e, params) = engine();
            set(&mut e, &params, "noise.level", 1.0);
            set(&mut e, &params, id, 0.5);
            e.note_on(60, 1.0);
            run(&mut e, 22_050);
            set(&mut e, &params, id, 0.0);
            run(&mut e, 44_100 * 4);
            e.note_on(60, 1.0);
            let x = run(&mut e, 4410);
            let changes = x.windows(2).filter(|w| w[0] != w[1]).count();
            assert!(peak(&x) > 0.1 && changes > 4000, "{id}: the pad froze after Crush went back to 0 ({changes} changes)");
        }
    }

    /// A ring sends the envelope to its control, in the voice and in the
    /// effects: here each one pulls its control down hard at the hit and lets
    /// it come back as the envelope decays.
    #[test]
    fn rings_send_the_envelope_to_their_controls() {
        let hit = |rings: &[(&str, f32)]| {
            let (mut e, params) = engine();
            for (id, v) in [("voice.variation", 0.0), ("envelope.decay", 60.0), ("envelope.curve", 0.0), ("envelope.velocity", 0.0),
                            ("noise.level", 0.5), ("noise.decay", 1000.0)] {
                set(&mut e, &params, id, v);
            }
            for &(id, v) in rings {
                set(&mut e, &params, id, v);
            }
            e.note_on(60, 1.0);
            e.voices.iter_mut().for_each(|v| v.dsp.set_param(e.slots.seed, 0.5));
            run(&mut e, 22_050)
        };
        let plain = hit(&[]);
        let differs = |a: &[f32], b: &[f32]| a.iter().zip(b).fold(0.0f32, |m, (x, y)| m.max((x - y).abs())) / peak(b).max(1e-9);
        // Voice: Noise's Bandpass ring at -1 closes the filter at the hit, then lets it back open.
        let swept = hit(&[("noise.bandpass_mod", -1.0)]);
        assert!(differs(&swept[..441], &plain[..441]) > 0.3, "the send should change the start");
        assert!(differs(&swept[13_230..], &plain[13_230..]) < 0.05, "and let go once the envelope is over");
        // Effects: Crush's ring crushes the start only.
        let crushed = hit(&[("crush.amount_mod", 1.0)]);
        assert!(differs(&crushed[..441], &plain[..441]) > 0.3, "the start should be crushed");
        assert!(differs(&crushed[13_230..], &plain[13_230..]) < 0.05, "and the tail clean");
    }

    #[test]
    fn choke_silences_the_effects_tails_too() {
        let (mut e, params) = engine();
        set(&mut e, &params, "tone.pitch", 200.0);
        set(&mut e, &params, "comb.amount", 1.0);
        e.note_on(60, 1.0);
        run(&mut e, 4410);
        e.choke();
        let after = run(&mut e, 4410);
        assert!(peak(&after[1323..]) < 1e-3, "the comb rang on after a choke: {}", peak(&after[1323..]));
        // ...and the next hit plays normally.
        e.note_on(60, 1.0);
        assert!(peak(&run(&mut e, 4410)) > 0.1);
    }

    #[test]
    fn every_effect_at_zero_is_a_true_bypass() {
        let (mut a, params) = engine();
        let (mut b, _) = engine();
        // b sets every effect explicitly to its off position; a keeps defaults.
        for (id, v) in [("crush.amount", 0.0), ("drive.amount", 0.0), ("ring_mod.amount", 0.0), ("comb.amount", 0.0),
                        ("filter.cutoff", 20_000.0), ("filter.position", 1.0), ("compressor.threshold", 0.0),
                        ("chop.hold", 2000.0), ("chop.rate", 0.0)] {
            set(&mut b, &params, id, v);
        }
        for e in [&mut a, &mut b] {
            e.note_on(60, 1.0);
            e.voices.iter_mut().for_each(|v| v.dsp.set_param(e.slots.seed, 0.5));
        }
        assert_eq!(run(&mut a, 22_050), run(&mut b, 22_050));
    }

    /// A long, plain tone (no pitch drop, 2 s decay, no Variation) through `sets`.
    fn long_hit(sets: &[(&str, f32)], samples: usize) -> Vec<f32> {
        let (mut e, params) = engine();
        for &(id, v) in [("voice.variation", 0.0), ("tone.drop", 0.0), ("tone.pitch", 110.0), ("tone.decay", 2000.0), ("tone.snap", 0.0)]
            .iter().chain(sets)
        {
            set(&mut e, &params, id, v);
        }
        e.note_on(60, 1.0);
        e.voices.iter_mut().for_each(|v| v.dsp.set_param(e.slots.seed, 0.5));
        run(&mut e, samples)
    }

    /// With nothing between its two places, the filter sounds the same Pre
    /// and Post; with Drive between them, Pre drives the filter's ring.
    #[test]
    fn the_filter_runs_before_drive_or_after_comb() {
        let filter = [("filter.cutoff", 900.0), ("filter.resonance", 0.95), ("noise.level", 0.6), ("noise.decay", 40.0)];
        let at = |pre: f32, more: &[(&str, f32)]| {
            let sets: Vec<_> = filter.iter().copied().chain([("filter.position", pre)]).chain(more.iter().copied()).collect();
            long_hit(&sets, 22_050)
        };
        assert_eq!(at(0.0, &[]), at(1.0, &[]), "nothing between: the same either way");
        let drive = [("drive.type", 2.0), ("drive.amount", 0.9)];
        let (post, pre) = (at(0.0, &drive), at(1.0, &drive));
        let differs = post.iter().zip(&pre).fold(0.0f32, |m, (a, b)| m.max((a - b).abs()));
        assert!(differs > 0.1, "Drive between them: Pre should sound different ({differs})");
        assert!(pre.iter().all(|s| s.is_finite()) && peak(&pre) <= 1.0);
    }

    /// Fuzz: the starved bias drops a tail once it's quiet enough, where
    /// Clip would hold it up; loud parts stay loud; nothing goes non-finite.
    #[test]
    fn fuzz_cuts_quiet_tails_and_stays_finite() {
        let tail = |kind: f32| {
            // (The tone falls below the fuzz's bias, about -38 dBFS, at 1.25 s.)
            let x = long_hit(&[("drive.type", kind), ("drive.amount", 1.0)], 88_200);
            (peak(&x[..4410]), peak(&x[66_150..]))
        };
        let (fuzz_start, fuzz_tail) = tail(3.0);
        let (_, clip_tail) = tail(2.0);
        assert!(fuzz_start > 0.5, "the hit should be loud through the fuzz: {fuzz_start}");
        assert!(clip_tail > 0.1, "Clip holds the tail up: {clip_tail}");
        assert!(fuzz_tail < 0.01, "the fuzz should have dropped the tail: {fuzz_tail}");
        // Big jumps between samples (a crushed, layered hit) stay finite.
        let x = long_hit(&[("drive.type", 3.0), ("drive.amount", 1.0), ("crush.amount", 1.0), ("noise.level", 1.0),
                           ("sample_1.level", 1.0)], 22_050);
        assert!(x.iter().all(|s| s.is_finite()) && peak(&x) > 0.1);
    }

    /// Chop's Hold cuts each hit dead once it's over; its Rate chops at the
    /// rate its knob says, starting open, the same on every hit.
    #[test]
    fn chop_holds_then_cuts_and_chops_at_its_rate() {
        let x = long_hit(&[("chop.hold", 50.0)], 22_050);
        assert!(peak(&x[..2000]) > 0.3, "open during the hold");
        assert!(peak(&x[(0.052 * 44_100.0) as usize..]) == 0.0, "shut after it");

        // Rate 0.6: (0.6 - 0.2) / 0.8 of the way from 10 to 400 Hz, full depth.
        let hz = 10.0 * 40f32.powf(0.5);
        let x = long_hit(&[("chop.rate", 0.6)], 44_100);
        let closed: Vec<bool> = x.chunks(16).map(|c| peak(c) < 1e-4).collect();
        let shuts = closed.windows(2).filter(|w| !w[0] && w[1]).count() as f32;
        assert!((shuts / hz - 1.0).abs() < 0.05, "{shuts} chops in a second, for {hz} Hz");
        assert!(!closed[0], "every hit starts open");

        // Half-way up its fade-in, Rate is a tremolo: never fully shut.
        let x = long_hit(&[("chop.rate", 0.1)], 22_050);
        assert!(x[..11_025].chunks(64).all(|c| peak(c) > 0.05), "a half-depth chop never shuts");
    }

    /// Ramp sounds, stays finite and sits near white noise's level.
    #[test]
    fn ramp_noise_sounds_near_white_noise_level() {
        let rms = |x: &[f32]| (x.iter().map(|v| v * v).sum::<f32>() / x.len() as f32).sqrt();
        let noise = |generator: f32| {
            let x = long_hit(&[("tone.level", 0.0), ("noise.level", 1.0), ("noise.generator", generator), ("noise.decay", 4000.0),
                               ("noise.snap", 0.0), ("noise.highpass", 20.0), ("noise.q", 0.0)], 22_050);
            assert!(x.iter().all(|s| s.is_finite()));
            rms(&x[2205..])
        };
        let db = 20.0 * (noise(6.0) / noise(0.0)).log10();
        assert!(db.abs() < 2.0, "Ramp sits {db:.1} dB from white noise");
    }

    /// How alike two stretches are, whatever their levels (1 = the same shape).
    fn likeness(a: &[f32], b: &[f32]) -> f32 {
        let dot = |x: &[f32], y: &[f32]| x.iter().zip(y).map(|(p, q)| p * q).sum::<f32>();
        dot(a, b) / (dot(a, a) * dot(b, b)).sqrt().max(1e-20)
    }

    /// Magnitudes of a stretch's spectrum (Hann window), 10 Hz apart up to 4 kHz.
    fn spectrum(x: &[f32]) -> Vec<f32> {
        let n = x.len() as f32;
        (1..400).map(|bin| {
            let w = 2.0 * std::f32::consts::PI * bin as f32 * 10.0 / 44_100.0;
            let (re, im) = x.iter().enumerate().fold((0.0, 0.0), |(re, im), (i, v)| {
                let hann = 0.5 - 0.5 * (2.0 * std::f32::consts::PI * i as f32 / n).cos();
                (re + v * hann * (w * i as f32).cos(), im + v * hann * (w * i as f32).sin())
            });
            (re * re + im * im).sqrt()
        }).collect()
    }

    /// Bite changes the hit where its envelope is high and leaves the
    /// decayed tail clean, at about the clean hit's level.
    #[test]
    fn bite_grits_the_start_and_leaves_the_tail_clean() {
        let rms = |x: &[f32]| (x.iter().map(|v| v * v).sum::<f32>() / x.len() as f32).sqrt();
        let hit = |bite: f32| long_hit(&[("tone.level", 0.0), ("noise.level", 1.0), ("noise.decay", 300.0), ("noise.snap", 0.0),
                                         ("noise.bandpass", 6000.0), ("noise.highpass", 2000.0), ("noise.bite", bite)], 22_050);
        let (clean, bitten) = (hit(0.0), hit(1.0));
        assert!(bitten.iter().all(|s| s.is_finite()));
        let (start, tail) = (0..1323, 8820..13230);
        let db = 20.0 * (rms(&bitten[start.clone()]) / rms(&clean[start.clone()])).log10();
        assert!(db.abs() < 3.0, "the start should stay near the clean level: {db:.1} dB");
        let (at_start, at_tail) = (likeness(&bitten[start], &clean[0..1323]), likeness(&bitten[tail.clone()], &clean[tail]));
        assert!(at_start < 0.95, "the start should sound different: {at_start}");
        assert!(at_tail > 0.99, "the tail should be clean: {at_tail}");
    }

    /// Body Clip overloads loud rings (they change shape) and leaves quiet
    /// ones as they were; it holds the level, stays bounded and finite.
    #[test]
    fn body_clip_overloads_loud_rings_only() {
        let body = |clip: f32| {
            let (mut e, params) = engine();
            for (id, v) in [("voice.variation", 0.0), ("tone.level", 0.0), ("body.level", 1.0), ("tone.pitch", 80.0),
                            ("body.decay", 600.0), ("body.clip", clip)] {
                set(&mut e, &params, id, v);
            }
            e.note_on(60, 1.0);
            e.voices.iter_mut().for_each(|v| v.dsp.set_param(e.slots.seed, 0.5));
            run(&mut e, 44_100)
        };
        let (clean, clipped) = (body(0.0), body(1.0));
        assert!(clipped.iter().all(|s| s.is_finite()) && peak(&clipped) < 2.0 * peak(&clean));
        // (By spectrum: the overload shifts the ring's phase, so the waveforms drift apart.)
        let (start, tail) = (0..4410, 30_870..35_280);
        let alike = |r: std::ops::Range<usize>| likeness(&spectrum(&clipped[r.clone()]), &spectrum(&clean[r]));
        let (at_start, at_tail) = (alike(start), alike(tail));
        assert!(at_start < 0.9, "a loud ring should change shape: {at_start}");
        assert!(at_tail > at_start + 0.05, "and come cleaner as it decays: {at_start} -> {at_tail}");
    }

    /// A Tone-only hit with these settings at this velocity (no Variation).
    fn tone_hit(sets: &[(&str, f32)], velocity: f32, samples: usize) -> Vec<f32> {
        let (mut e, params) = engine();
        for &(id, v) in [("voice.variation", 0.0), ("tone.decay", 2000.0), ("tone.snap", 0.0), ("tone.vel_pitch", 0.0)].iter().chain(sets) {
            set(&mut e, &params, id, v);
        }
        e.note_on(60, velocity);
        e.voices.iter_mut().for_each(|v| v.dsp.set_param(e.slots.seed, 0.5));
        run(&mut e, samples)
    }

    /// The frequency over ms t0..t1, from its up-crossings.
    fn hz_between(x: &[f32], t0: f32, t1: f32) -> f32 {
        let (a, b) = ((t0 * 44.1) as usize, (t1 * 44.1) as usize);
        let ups: Vec<f32> = (a..b - 1).filter(|&i| x[i] < 0.0 && x[i + 1] >= 0.0)
            .map(|i| i as f32 + x[i] / (x[i] - x[i + 1])).collect();
        if ups.len() < 2 { return 0.0; }
        44_100.0 * (ups.len() - 1) as f32 / (ups[ups.len() - 1] - ups[0])
    }

    /// Hold keeps the pitch at the top before the drop starts.
    #[test]
    fn hold_delays_the_drop() {
        let sets = |hold: f32| tone_hit(&[("tone.pitch", 100.0), ("tone.drop", 24.0), ("tone.split", 0.0),
                                          ("tone.slow_time", 60.0), ("tone.hold", hold)], 1.0, 8820);
        let (plain, held) = (sets(0.0), sets(30.0));
        let (p, h) = (hz_between(&plain, 15.0, 25.0), hz_between(&held, 15.0, 25.0));
        assert!((h / 400.0 - 1.0).abs() < 0.03, "held at the top (400 Hz): {h}");
        assert!(p < 0.75 * h, "without Hold it has already fallen: {p} vs {h}");
        // After the hold it falls the same way, 30 ms later.
        let (p2, h2) = (hz_between(&plain, 40.0, 70.0), hz_between(&held, 70.0, 100.0));
        assert!((p2 / h2 - 1.0).abs() < 0.03, "{p2} vs {h2}");
    }

    /// Vel Time: softer hits drop quicker; full velocity keeps the knob's time.
    #[test]
    fn vel_time_shortens_soft_hits_drops() {
        let sets = |vel_time: f32, velocity: f32| tone_hit(&[("tone.pitch", 60.0), ("tone.drop", 24.0), ("tone.split", 0.0),
                                                           ("tone.slow_time", 200.0), ("tone.vel_time", vel_time)], velocity, 13230);
        let at = |x: &[f32]| hz_between(x, 60.0, 90.0);
        let (hard_off, hard_on, soft_on) = (at(&sets(0.0, 1.0)), at(&sets(1.0, 1.0)), at(&sets(1.0, 0.3)));
        assert!((hard_on / hard_off - 1.0).abs() < 0.01, "full velocity keeps the time: {hard_on} vs {hard_off}");
        assert!(soft_on < 0.8 * hard_on, "a soft hit has fallen further: {soft_on} vs {hard_on}");
    }

    /// Laser: a deep Drop starts above Nyquist and folds down (energy high
    /// in the first milliseconds), stays finite, and lands at the pitch.
    #[test]
    fn laser_folds_down_from_above_nyquist() {
        let x = tone_hit(&[("tone.pitch", 40.0), ("tone.drop", 118.0), ("tone.split", 0.0), ("tone.slow_time", 150.0)], 1.0, 44_100);
        assert!(x.iter().all(|s| s.is_finite()));
        let early = hz_between(&x, 1.0, 4.0);
        assert!(early > 3000.0, "starts high, folded: {early} Hz");
        let late = hz_between(&x, 800.0, 950.0);
        assert!((late / 40.0 - 1.0).abs() < 0.05, "lands at Pitch: {late}");
    }

    /// Spectral centroid (Hz) of a stretch, from its 10 Hz spectrum.
    fn centroid(x: &[f32]) -> f32 {
        let sp = spectrum(x);
        let (num, den) = sp.iter().enumerate().fold((0.0, 0.0), |(n, d), (i, m)| (n + (i + 1) as f32 * 10.0 * m * m, d + m * m));
        num / den.max(1e-20)
    }

    /// Osc 2: every mode changes Tone's spectrum, holds its level within a
    /// few dB and stays finite; FM's Feedback turns it toward noise.
    #[test]
    fn osc2_modes_color_tone_at_its_level() {
        let rms = |x: &[f32]| (x.iter().map(|v| v * v).sum::<f32>() / x.len() as f32).sqrt();
        let hit = |more: &[(&str, f32)]| tone_hit(&[("tone.pitch", 110.0), ("tone.drop", 0.0), ("tone.decay", 600.0)].iter().copied()
                                                  .chain(more.iter().copied()).collect::<Vec<_>>(), 1.0, 8820);
        let plain = hit(&[]);
        for mode in 0..4 {
            let x = hit(&[("tone.osc_2", mode as f32), ("tone.amount", 1.0), ("tone.ratio", 2.7), ("tone.morph", 1.0)]);
            assert!(x.iter().all(|s| s.is_finite()), "mode {mode}");
            let db = 20.0 * (rms(&x[..4410]) / rms(&plain[..4410])).log10();
            assert!(db.abs() < 4.0, "mode {mode} level {db:.1} dB");
            assert!(centroid(&x[..2205]) > 1.5 * centroid(&plain[..2205]), "mode {mode} should brighten Tone");
        }
        let flat = |x: &[f32]| { let sp = spectrum(x); let n = sp.len() as f32;
            let g = (sp.iter().map(|m| (m * m + 1e-20).ln()).sum::<f32>() / n).exp(); g / (sp.iter().map(|m| m * m).sum::<f32>() / n) };
        let noisy = hit(&[("tone.feedback", 1.0)]);
        assert!(flat(&noisy[2205..6615]) > 0.1, "full Feedback is noisy: {}", flat(&noisy[2205..6615]));
    }

    /// Rattle keeps Tone's level and spreads its spectrum; Damp darkens the
    /// fading tail and leaves the start.
    #[test]
    fn rattle_and_damp() {
        let rms = |x: &[f32]| (x.iter().map(|v| v * v).sum::<f32>() / x.len() as f32).sqrt();
        let hit = |more: &[(&str, f32)]| tone_hit(&[("tone.pitch", 110.0), ("tone.drop", 0.0), ("tone.decay", 600.0), ("tone.shape", 0.8)]
                                                  .iter().copied().chain(more.iter().copied()).collect::<Vec<_>>(), 1.0, 22_050);
        let plain = hit(&[]);
        let rattled = hit(&[("tone.rattle", 1.0)]);
        let db = 20.0 * (rms(&rattled[..4410]) / rms(&plain[..4410])).log10();
        assert!(db.abs() < 2.0 && centroid(&rattled[..2205]) > 2.0 * centroid(&plain[..2205]), "Rattle: {db:.1} dB");
        let damped = hit(&[("tone.damp", 0.5)]);
        assert!((centroid(&damped[..2205]) / centroid(&plain[..2205]) - 1.0).abs() < 0.1, "the start stays open");
        assert!(centroid(&damped[13230..17640]) < 0.6 * centroid(&plain[13230..17640]), "the tail darkens");
    }

    /// Spread moves Metal's ratios (a different spectrum, the same level).
    #[test]
    fn spread_moves_the_metal_ratios() {
        let rms = |x: &[f32]| (x.iter().map(|v| v * v).sum::<f32>() / x.len() as f32).sqrt();
        let metal = |spread: f32| long_hit(&[("tone.level", 0.0), ("noise.level", 1.0), ("noise.generator", 4.0), ("noise.decay", 4000.0),
                                            ("noise.snap", 0.0), ("noise.highpass", 20.0), ("noise.q", 0.0), ("tone.pitch", 300.0), ("noise.spread", spread)], 13_230);
        let m0 = metal(0.0);
        for sp in [-1.0f32, 1.0] {
            let x = metal(sp);
            let db = 20.0 * (rms(&x[2205..]) / rms(&m0[2205..])).log10();
            assert!(db.abs() < 2.0, "Spread {sp}: {db:.1} dB");
            assert!(likeness(&spectrum(&x[4410..8820]), &spectrum(&m0[4410..8820])) < 0.8, "Spread {sp} moves the spectrum");
        }
    }

    /// Measures the Battalion-inspired additions for tuning:
    /// `cargo test --release -- --ignored --nocapture measure_battalion`
    #[test]
    #[ignore]
    fn measure_battalion() {
        let rms = |x: &[f32]| (x.iter().map(|v| v * v).sum::<f32>() / x.len().max(1) as f32).sqrt();
        let db = |a: f32, b: f32| 20.0 * (a / b).log10();
        let base = [("tone.pitch", 110.0), ("tone.drop", 0.0), ("tone.decay", 600.0)];
        let hit = |more: &[(&str, f32)]| tone_hit(&base.iter().copied().chain(more.iter().copied()).collect::<Vec<_>>(), 1.0, 22_050);
        let plain = hit(&[]);
        let report = |name: &str, x: &[f32]| println!("{name:28} {:+5.1} dB, peak {:.2}, centroid 0-50 ms {:5.0} Hz, 300-400 ms {:5.0} Hz, finite {}",
            db(rms(&x[..4410]), rms(&plain[..4410])), peak(x), centroid(&x[..2205]), centroid(&x[13230..17640]), x.iter().all(|s| s.is_finite()));
        report("plain", &plain);
        for (mode, name) in [(0.0, "FM"), (1.0, "Tangle"), (2.0, "Shame"), (3.0, "VOSIM")] {
            for amount in [0.5f32, 1.0] {
                for morph in if mode == 1.0 || mode == 2.0 { vec![0.0f32, 0.4, 1.0] } else { vec![0.0] } {
                    let x = hit(&[("tone.osc_2", mode), ("tone.amount", amount), ("tone.morph", morph), ("tone.ratio", 2.7)]);
                    report(&format!("{name} a{amount} m{morph}"), &x);
                }
            }
        }
        // Flatness of the spectrum (geometric over arithmetic mean of power): ~0 a tone, toward 1 noise.
        let flat = |x: &[f32]| { let sp = spectrum(x); let n = sp.len() as f32;
            let g = (sp.iter().map(|m| (m * m + 1e-20).ln()).sum::<f32>() / n).exp(); g / (sp.iter().map(|m| m * m).sum::<f32>() / n) };
        for fb in [0.3f32, 0.6, 1.0] {
            let x = hit(&[("tone.feedback", fb)]);
            report(&format!("FM feedback {fb}"), &x);
            println!("   flatness {:.3} (plain {:.3})", flat(&x[2205..6615]), flat(&plain[2205..6615]));
        }
        report("Rattle 1", &hit(&[("tone.rattle", 1.0)]));
        report("Damp 0.5", &hit(&[("tone.damp", 0.5), ("tone.shape", 0.8)]));
        report("Damp 0 (shape 0.8)", &hit(&[("tone.shape", 0.8)]));
        let metal = |spread: f32| long_hit(&[("tone.level", 0.0), ("noise.level", 1.0), ("noise.generator", 4.0), ("noise.decay", 4000.0),
                                            ("noise.snap", 0.0), ("noise.highpass", 20.0), ("noise.q", 0.0), ("tone.pitch", 300.0), ("noise.spread", spread)], 22_050);
        let m0 = metal(0.0);
        for sp in [-1.0f32, -0.5, 0.5, 1.0] {
            let x = metal(sp);
            println!("metal spread {sp:+}: {:+.1} dB, likeness {:.3}", db(rms(&x[2205..]), rms(&m0[2205..])), likeness(&spectrum(&x[4410..8820]), &spectrum(&m0[4410..8820])));
        }
    }

    /// Measures the Nithonat-inspired additions (Ramp, Bite, Body Clip) for tuning:
    /// `cargo test --release -- --ignored --nocapture measure_additions`
    #[test]
    #[ignore]
    fn measure_additions() {
        let rms = |x: &[f32]| (x.iter().map(|v| v * v).sum::<f32>() / x.len().max(1) as f32).sqrt();
        let db = |a: f32, b: f32| 20.0 * (a / b).log10();
        let noise = |more: &[(&str, f32)]| {
            let base = [("tone.level", 0.0), ("noise.level", 1.0), ("noise.decay", 4000.0), ("noise.snap", 0.0),
                        ("noise.highpass", 20.0), ("noise.q", 0.0)];
            long_hit(&base.iter().copied().chain(more.iter().copied()).collect::<Vec<_>>(), 22_050)
        };
        let white = noise(&[]);
        let ramp = noise(&[("noise.generator", 6.0)]);
        println!("ramp vs white: {:.2} dB, peak {:.2}", db(rms(&ramp[2205..]), rms(&white[2205..])), peak(&ramp));
        // Bite on a decaying hat-like noise: level against clean, early vs late.
        let hat = [("noise.decay", 300.0), ("noise.snap", 0.0), ("noise.bandpass", 6000.0), ("noise.highpass", 2000.0), ("noise.q", 0.3)];
        let hit = |bite: f32| {
            let sets: Vec<_> = [("tone.level", 0.0), ("noise.level", 1.0)].iter().copied().chain(hat).chain([("noise.bite", bite)]).collect();
            long_hit(&sets, 22_050)
        };
        let clean = hit(0.0);
        for b in [0.25f32, 0.5, 0.75, 1.0] {
            let x = hit(b);
            let (e, l) = (0..1323, 8820..13230);
            let rel = |r: std::ops::Range<usize>| {
                let d: Vec<f32> = x[r.clone()].iter().zip(&clean[r.clone()]).map(|(a, c)| a - c).collect();
                rms(&d) / rms(&clean[r])
            };
            println!("bite {b}: early {:+.1} dB, late {:+.1} dB vs clean; difference early {:.2}, late {:.2}; peak {:.2}",
                     db(rms(&x[e.clone()]), rms(&clean[e.clone()])), db(rms(&x[l.clone()]), rms(&clean[l.clone()])), rel(e), rel(l), peak(&x));
        }
        // Body Clip on a membrane kick body.
        let body = |clip: f32, vel: f32| {
            let (mut e, params) = engine();
            for (id, v) in [("voice.variation", 0.0), ("tone.level", 0.0), ("body.level", 1.0), ("tone.pitch", 80.0),
                            ("body.decay", 600.0), ("body.clip", clip)] {
                set(&mut e, &params, id, v);
            }
            e.note_on(60, vel);
            e.voices.iter_mut().for_each(|v| v.dsp.set_param(e.slots.seed, 0.5));
            run(&mut e, 44_100)
        };
        let ref0 = body(0.0, 1.0);
        let zc = |x: &[f32]| x.windows(2).filter(|w| w[0] < 0.0 && w[1] >= 0.0).count();
        for (c, v) in [(0.0f32, 1.0f32), (0.5, 1.0), (1.0, 1.0), (1.0, 0.3)] {
            let x = body(c, v);
            let mean = x[..4410].iter().sum::<f32>() / 4410.0;
            println!("clip {c} vel {v}: peak {:.3} (clean {:.3}), 0-50 ms {:+.1} dB, 300-500 ms {:+.1} dB, mean {:.4}, crossings 0-100 ms {} (clean {}), finite {}",
                     peak(&x), peak(&ref0), db(rms(&x[..2205]), rms(&ref0[..2205])), db(rms(&x[13230..22050]), rms(&ref0[13230..22050])),
                     mean, zc(&x[..4410]), zc(&ref0[..4410]), x.iter().all(|s| s.is_finite()));
        }
    }

    /// RF sounds, stays finite and sits near white noise's level.
    #[test]
    fn rf_noise_sounds_near_white_noise_level() {
        let rms = |x: &[f32]| (x.iter().map(|v| v * v).sum::<f32>() / x.len() as f32).sqrt();
        let noise = |generator: f32| {
            let x = long_hit(&[("tone.level", 0.0), ("noise.level", 1.0), ("noise.generator", generator), ("noise.decay", 4000.0),
                               ("noise.snap", 0.0), ("noise.highpass", 20.0), ("noise.q", 0.0), ("tone.pitch", 217.0)], 22_050);
            assert!(x.iter().all(|s| s.is_finite()));
            rms(&x[2205..])
        };
        let (white, rf) = (noise(0.0), noise(5.0));
        let db = 20.0 * (rf / white).log10();
        assert!(db.abs() < 2.0, "RF sits {db:.1} dB from white noise");
    }

    #[test]
    fn keytrack_follows_the_note_only_when_on() {
        let (mut e, params) = engine();
        let pitch_of = |e: &mut Engine, note: u8| {
            e.reset();
            e.note_on(note, 1.0);
            let x = run(e, 44_100);
            // zero crossings in the steady part, 0.3..0.8 s
            x[13_230..35_280].windows(2).filter(|w| w[0] <= 0.0 && w[1] > 0.0).count()
        };
        // Note 60 plays at the Pitch knob (50 Hz by default) when Keytrack is on.
        assert_eq!(pitch_of(&mut e, 60), pitch_of(&mut e, 72), "Keytrack off: every note plays at Pitch");
        set(&mut e, &params, "tone.keytrack", 1.0);
        let (low, high) = (pitch_of(&mut e, 60), pitch_of(&mut e, 72));
        assert!((high as f32 / low as f32 - 2.0).abs() < 0.1, "an octave up should double: {low} -> {high}");
    }
}

/// The pad filter (the SH-101's) on its own, in the effects DSP with every
/// other effect off.
#[cfg(test)]
mod filter {
    use super::*;
    use crate::params::DrumParams;
    use faust_types::FaustDsp;

    const SR: f32 = 44_100.0;

    fn fx(cutoff: f32, resonance: f32) -> Box<FxDsp> {
        let params = DrumParams::from_faust();
        let mut fx = Box::new(FxDsp::new());
        fx.init(SR as i32);
        for (id, v) in [("filter.cutoff", cutoff), ("filter.resonance", resonance)] {
            fx.set_param(params.faust_index(id).unwrap().1, v);
        }
        fx.compute(4410, &[&vec![0.0; 4410]], &mut [&mut vec![0.0; 4410]]);   // let the controls settle
        fx
    }

    fn process(fx: &mut FxDsp, x: &[f32]) -> Vec<f32> {
        let mut y = vec![0.0; x.len()];
        fx.compute(x.len() as i32, &[x], &mut [&mut y]);
        y
    }

    fn rms(x: &[f32]) -> f32 {
        (x.iter().map(|v| v * v).sum::<f32>() / x.len() as f32).sqrt()
    }

    /// Gain (dB) for a quiet sine at hz.
    fn gain_db(cutoff: f32, resonance: f32, hz: f32) -> f32 {
        let mut f = fx(cutoff, resonance);
        let x: Vec<f32> = (0..88_200).map(|n| 0.01 * (2.0 * std::f32::consts::PI * hz * n as f32 / SR).sin()).collect();
        let y = process(&mut f, &x);
        20.0 * (rms(&y[44_100..]) / rms(&x[44_100..])).log10()
    }

    /// The impulse response's ring: its frequency (zero crossings) and T60
    /// (from the level 50 ms in and 250 ms later).
    fn ring(cutoff: f32, resonance: f32) -> (f32, f32) {
        let mut f = fx(cutoff, resonance);
        let mut x = vec![0.0; 4 * 44_100];
        x[0] = 0.01;
        let y = process(&mut f, &x);
        let window = |t: f32| rms(&y[(t * SR) as usize..((t + 0.02) * SR) as usize]);
        let (a, b) = (window(0.05), window(0.30));
        let t60 = 0.25 * 60.0 / (20.0 * (a / b).log10());
        let crossings = y[2205..13_230].windows(2).filter(|w| w[0] <= 0.0 && w[1] > 0.0).count();
        (crossings as f32 / 0.25, t60)
    }

    fn kick() -> Vec<f32> {
        (0..44_100).map(|n| { let t = n as f32 / SR; 0.8 * (-t / 0.1).exp() * (2.0 * std::f32::consts::PI * 60.0 * t).sin() }).collect()
    }

    fn burst() -> Vec<f32> {
        let mut seed = 1u32;
        (0..44_100).map(|n| {
            seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            if n < 2205 { 0.8 * ((seed >> 8) as f32 / 8_388_608.0 - 1.0) } else { 0.0 }
        }).collect()
    }

    /// 24 dB/oct, -12 dB at the cutoff with no resonance (four equal poles);
    /// resonance costs 4.4 dB of bass at the top (the 101's output mix),
    /// rings on the cutoff, for about 2 s at the top whatever the cutoff,
    /// and loud hits stay within a few dB of full scale.
    #[test]
    fn sh101_filter_tracks_rings_and_stays_bounded() {
        assert!((gain_db(1000.0, 0.0, 1000.0) + 12.0).abs() < 0.5);
        let slope = gain_db(1000.0, 0.0, 2000.0) - gain_db(1000.0, 0.0, 4000.0);
        assert!(slope > 21.0 && slope < 26.0, "slope {slope} dB/oct");
        let bass = gain_db(1000.0, 1.0, 50.0);
        assert!(bass < -3.5 && bass > -5.0, "bass at full resonance {bass} dB");
        for hz in [60.0, 1000.0, 5000.0] {
            let (f, t60) = ring(hz, 1.0);
            assert!((f / hz - 1.0).abs() < 0.02, "rang at {f} Hz for a {hz} Hz cutoff");
            assert!(t60 > 1.6 && t60 < 2.6, "T60 {t60} s at {hz} Hz");
        }
        for hz in [80.0, 1000.0, 4000.0] {
            for x in [kick(), burst()] {
                let mut f = fx(hz, 1.0);
                let peak = process(&mut f, &x).iter().fold(0f32, |m, v| m.max(v.abs()));
                assert!(peak.is_finite() && peak < 2.0, "peak {peak} at {hz} Hz");
            }
        }
    }

    /// Prints the filter's measurements: cargo test --release -- --ignored --nocapture filter_measure
    #[test]
    #[ignore]
    fn filter_measure() {
        for r in [0.0, 0.5, 0.8, 1.0] {
            let db: Vec<String> = [50.0, 250.0, 500.0, 1000.0, 2000.0, 4000.0]
                .iter().map(|hz| format!("{:6.1}", gain_db(1000.0, r, *hz))).collect();
            println!("cutoff 1 kHz, res {r:.1}: 50/250/500/1k/2k/4k Hz: {}", db.join(" "));
        }
        // Loud hits: a 0.8 kick (60 Hz, 100 ms decay) and a 0.8 noise burst (50 ms).
        let (kick, burst) = (kick(), burst());
        for hz in [80.0, 300.0, 1000.0, 4000.0] {
            for r in [0.0, 0.5, 0.8, 1.0] {
                let pk = |x: &[f32]| { let mut f = fx(hz, r); 20.0 * process(&mut f, x).iter().fold(0f32, |m, v| m.max(v.abs())).log10() };
                println!("cutoff {hz} res {r:.1}: kick peak {:5.1} dBFS, burst peak {:5.1} dBFS", pk(&kick), pk(&burst));
            }
        }
        for hz in [60.0, 200.0, 1000.0, 5000.0] {
            for r in [0.6, 0.8, 0.9, 1.0] {
                let (f, t60) = ring(hz, r);
                println!("cutoff {hz} Hz res {r:.1}: rings at {f:.0} Hz, T60 {t60:.2} s");
            }
        }
    }
}

/// Cross-check against the offline C++ renderer (tools/compare-backends.py
/// runs this): ABDRUM_COMPARE=<dir> cargo test --release -- --ignored compare
#[cfg(test)]
mod compare {
    use super::*;
    use crate::params::DrumParams;

    #[test]
    #[ignore]
    fn compare() {
        let dir = std::env::var("ABDRUM_COMPARE").expect("set ABDRUM_COMPARE");
        let params = DrumParams::from_faust();
        for entry in std::fs::read_dir(&dir).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().map_or(true, |e| e != "txt") {
                continue;
            }
            let mut e = Engine::new(params.slots());
            e.init(44_100.0);
            super::tests::apply_all(&mut e, &params);
            let (mut seed, mut seconds) = (0.25f32, 1.0f32);
            for line in std::fs::read_to_string(&path).unwrap().lines() {
                let mut parts = line.split_whitespace();
                let (id, value): (&str, f32) = (parts.next().unwrap(), parts.next().unwrap().parse().unwrap());
                match id {
                    "seed" => seed = value,
                    "seconds" => seconds = value,
                    _ => super::tests::set(&mut e, &params, id, value),
                }
            }
            e.note_on(60, 1.0);
            e.voices[0].dsp.set_param(e.slots.seed, seed);
            let mut out = vec![0.0f32; (seconds * 44_100.0) as usize];
            for chunk in out.chunks_mut(MAX_BLOCK) {
                e.render(chunk);
            }
            let bytes: Vec<u8> = out.iter().flat_map(|s| s.to_le_bytes()).collect();
            std::fs::write(path.with_extension("rust.raw"), bytes).unwrap();
        }
    }
}


/// CPU benchmark: `cargo test --release -- --ignored --nocapture bench_cpu`
#[cfg(test)]
mod bench {
    use super::*;
    use crate::params::DrumParams;

    fn time(name: &str, sets: &[(&str, f32)], fx_on: bool) {
        let params = DrumParams::from_faust();
        let mut e = Engine::new(params.slots());
        e.init(48_000.0);
        e.apply(&params);
        for &(id, v) in sets {
            super::tests::set(&mut e, &params, id, v);
        }
        if fx_on {
            for (id, v) in [("crush.amount", 0.2), ("drive.amount", 0.5), ("ring_mod.amount", 0.3), ("comb.amount", 0.3), ("filter.cutoff", 800.0),
                            ("filter.resonance", 0.6), ("compressor.threshold", -12.0)] {
                super::tests::set(&mut e, &params, id, v);
            }
        }
        let mut out = [0.0f32; MAX_BLOCK];
        // Re-hit every 0.5 s of a 2 s run; the best of five runs (this is a shared machine).
        let blocks = 48_000 / MAX_BLOCK;
        let per_second = (0..5)
            .map(|_| {
                let start = std::time::Instant::now();
                for _ in 0..2 {
                    for b in 0..blocks {
                        if b == 0 || b == blocks / 2 {
                            e.note_on(60, 0.9);
                        }
                        e.render(&mut out);
                    }
                }
                start.elapsed().as_secs_f64() / 2.0
            })
            .fold(f64::MAX, f64::min);
        println!("{name:28} {:6.2} % of one core (per second of audio, ~2 voices ringing)", per_second * 100.0);
    }

    #[test]
    #[ignore]
    fn bench_cpu() {
        let off = [("tone.level", 0.0)];
        time("tone only (default)", &[], false);
        time("body only", &[off[0], ("body.level", 1.0)], false);
        time("noise only", &[off[0], ("noise.level", 1.0)], false);
        time("sample 1 only", &[off[0], ("sample_1.level", 1.0)], false);
        time("everything on", &[("body.level", 1.0), ("noise.level", 1.0), ("sample_1.level", 1.0), ("sample_2.level", 1.0)], false);
        time("everything + effects", &[("body.level", 1.0), ("noise.level", 1.0), ("sample_1.level", 1.0), ("sample_2.level", 1.0)], true);
    }
}

/// Before/after renders of the effects added in Nov 2026 (Filter Pre/Post,
/// Fuzz, Chop, RF): ABDRUM_DEMOS=<dir> cargo test --release -- --ignored demos
/// Each file plays its takes in turn, three hits each, with a gap between.
#[cfg(test)]
mod demos {
    use super::*;
    use crate::params::DrumParams;

    const SR: f32 = 44_100.0;
    type Sets<'a> = &'a [(&'a str, f32)];

    fn engine(params: &DrumParams, sets: &[Sets]) -> Engine {
        let mut e = Engine::new(params.slots());
        e.init(SR);
        e.apply(params);
        for &(id, v) in sets.iter().flat_map(|s| s.iter()) {
            super::tests::set(&mut e, params, id, v);
        }
        e
    }

    /// Hits at `times` (s) for `length` s.
    fn play(e: &mut Engine, times: &[f32], length: f32) -> Vec<f32> {
        let mut out = vec![0.0f32; (length * SR) as usize];
        let mut starts: Vec<usize> = times.iter().map(|t| (t * SR) as usize).collect();
        starts.push(out.len());
        e.render(&mut out[..starts[0]]);
        for w in starts.windows(2) {
            e.note_on(60, 1.0);
            for c in out[w[0]..w[1]].chunks_mut(MAX_BLOCK) {
                e.render(c);
            }
        }
        // The plugin's output stage: its -0.5 dBFS limiter.
        let mut limiter = Limiter::new();
        limiter.init(SR);
        out.iter_mut().for_each(|s| *s = limiter.process(*s));
        out
    }

    /// Takes in turn, each three hits `spacing` s apart, then a gap.
    fn takes(base: Sets, variants: &[Sets], spacing: f32) -> Vec<f32> {
        let params = DrumParams::from_faust();
        let mut out = Vec::new();
        for v in variants {
            let mut e = engine(&params, &[base, v]);
            out.extend(play(&mut e, &[0.0, spacing, 2.0 * spacing], 3.0 * spacing + 0.15));
            out.extend(vec![0.0; (0.5 * SR) as usize]);
        }
        out
    }

    fn write(dir: &str, name: &str, x: &[f32]) {
        std::fs::write(format!("{dir}/{name}.wav"), crate::export::wav(x, SR as u32)).unwrap();
        println!("{name}: {:.1} s", x.len() as f32 / SR);
    }

    const KICK: Sets = &[("tone.pitch", 42.0), ("tone.drop", 54.0), ("tone.split", 0.45), ("tone.fast_time", 3.0), ("tone.slow_time", 110.0),
                         ("tone.decay", 1100.0), ("tone.snap", 0.3), ("tone.tail_level", 0.45), ("noise.level", 0.06),
                         ("noise.bandpass", 1800.0), ("noise.decay", 40.0), ("voice.variation", 0.15)];
    const SNARE: Sets = &[("tone.level", 0.55), ("tone.pitch", 190.0), ("tone.drop", 12.0), ("tone.split", 0.7), ("tone.decay", 200.0),
                          ("noise.level", 0.7), ("noise.bandpass", 4000.0), ("noise.q", 0.3), ("noise.highpass", 900.0),
                          ("noise.decay", 320.0), ("noise.snap", 0.3), ("noise.tail_level", 0.35), ("voice.variation", 0.15)];

    #[test]
    #[ignore]
    fn demos() {
        let dir = std::env::var("ABDRUM_DEMOS").expect("set ABDRUM_DEMOS");
        std::fs::create_dir_all(&dir).unwrap();

        // 1. A resonant 101 pluck on a snare, Clip driven: Post, then Pre.
        let pluck: Sets = &[("filter.cutoff", 700.0), ("filter.resonance", 0.92), ("filter.cutoff_mod", 0.45),
                            ("envelope.decay", 180.0), ("drive.type", 2.0), ("drive.amount", 0.75)];
        write(&dir, "1a Filter Post then Pre (snare)", &takes(&[SNARE, pluck].concat(), &[&[("filter.position", 0.0)], &[("filter.position", 1.0)]], 0.6));
        // ...and on a pitched zap.
        let zap: Sets = &[("tone.pitch", 70.0), ("tone.drop", 30.0), ("tone.decay", 500.0), ("filter.cutoff", 260.0),
                          ("filter.resonance", 0.97), ("filter.cutoff_mod", 0.5), ("envelope.decay", 250.0),
                          ("drive.type", 2.0), ("drive.amount", 0.85)];
        write(&dir, "1b Filter Post then Pre (zap)", &takes(zap, &[&[("filter.position", 0.0)], &[("filter.position", 1.0)]], 0.7));

        // 2. Drive types on a kick and a snare: clean, Clip, Fuzz half, Fuzz full.
        let drives: [Sets; 4] = [&[], &[("drive.type", 2.0), ("drive.amount", 0.8)], &[("drive.type", 3.0), ("drive.amount", 0.6)],
                                 &[("drive.type", 3.0), ("drive.amount", 1.0)]];
        write(&dir, "2a Kick clean, Clip, Fuzz 60, Fuzz 100", &takes(KICK, &drives, 0.75));
        write(&dir, "2b Snare clean, Clip, Fuzz 60, Fuzz 100", &takes(SNARE, &drives, 0.6));

        // 3. Chop's Hold on a hard-driven kick with a long tail: none, 180 ms, 70 ms.
        let driven: Sets = &[("tone.decay", 1600.0), ("drive.type", 0.0), ("drive.amount", 0.9), ("compressor.threshold", -18.0)];
        write(&dir, "3 Driven kick, Hold off, 180 ms, 70 ms", &takes(&[KICK, driven].concat(),
              &[&[], &[("chop.hold", 180.0)], &[("chop.hold", 70.0)]], 0.75));

        // 4. Chop's Rate on a long snare: tremolo, flutter, rip, buzz; then a falling chop (Rate's ring).
        let long: Sets = &[("noise.decay", 700.0), ("tone.decay", 400.0), ("drive.type", 2.0), ("drive.amount", 0.5)];
        write(&dir, "4 Snare Rate 10 Hz half, 21 Hz, 100 Hz, 280 Hz, falling", &takes(&[SNARE, long].concat(), &[
            &[("chop.rate", 0.1)], &[("chop.rate", 0.36)], &[("chop.rate", 0.7)], &[("chop.rate", 0.92)],
            &[("chop.rate", 0.3), ("chop.rate_mod", 0.6), ("envelope.decay", 500.0), ("envelope.curve", 0.3)]], 0.75));

        // 5. RF: raw (217 Hz frames), as a hat against Metal, under a kick.
        let rf: Sets = &[("tone.level", 0.0), ("noise.level", 1.0), ("noise.generator", 5.0), ("noise.decay", 600.0), ("noise.q", 0.2),
                         ("noise.highpass", 150.0), ("tone.pitch", 217.0)];
        write(&dir, "5a RF raw, 217 Hz then 60 Hz frames", &takes(rf, &[&[], &[("tone.pitch", 60.0)]], 0.75));
        let hat: Sets = &[("tone.level", 0.0), ("noise.level", 0.9), ("tone.pitch", 420.0), ("noise.bandpass", 8000.0), ("noise.q", 0.4),
                          ("noise.highpass", 5000.0), ("noise.decay", 160.0), ("noise.snap", 0.3)];
        write(&dir, "5b Hat Metal then RF", &takes(hat, &[&[("noise.generator", 4.0)], &[("noise.generator", 5.0)]], 0.4));
        write(&dir, "5c Kick with RF crackle, Clip", &takes(KICK, &[&[("noise.generator", 5.0), ("noise.level", 0.25), ("noise.bandpass", 2500.0),
            ("noise.decay", 250.0), ("drive.type", 2.0), ("drive.amount", 0.6)]], 0.75));

        // 6. Together, a short beat: fuzzed kick through Pre filter and Hold;
        // snare with RF, clipped and chopped; RF hat.
        let params = DrumParams::from_faust();
        let kick: Sets = &[("drive.type", 3.0), ("drive.amount", 0.7), ("filter.position", 1.0), ("filter.cutoff", 180.0),
                           ("filter.resonance", 0.85), ("filter.cutoff_mod", 0.35), ("envelope.decay", 120.0), ("chop.hold", 220.0)];
        let snare: Sets = &[("noise.generator", 5.0), ("tone.pitch", 217.0), ("drive.type", 2.0), ("drive.amount", 0.8),
                            ("chop.rate", 0.62), ("chop.hold", 260.0), ("compressor.threshold", -16.0)];
        let hat2: Sets = &[("noise.generator", 5.0), ("noise.decay", 70.0), ("chop.hold", 60.0)];
        let step = 60.0 / 100.0 / 4.0; // 16ths at 100 bpm
        let bars = 2.0;
        let at = |pattern: &[usize]| -> Vec<f32> {
            (0..bars as usize).flat_map(|b| pattern.iter().map(move |s| (b * 16 + s) as f32 * step)).collect()
        };
        let length = bars * 16.0 * step + 0.6;
        let mut mix = play(&mut engine(&params, &[KICK, kick]), &at(&[0, 6, 10]), length);
        for (sets, pattern, gain) in [(&[SNARE, snare][..], &[4usize, 12][..], 0.8), (&[hat, hat2][..], &[2, 6, 8, 10, 14, 15][..], 0.35)] {
            let x = play(&mut engine(&params, sets), &at(pattern), length);
            mix.iter_mut().zip(x).for_each(|(m, s)| *m += gain * s);
        }
        let pk = mix.iter().fold(0.0f32, |m, s| m.max(s.abs()));
        mix.iter_mut().for_each(|s| *s *= 0.94 / pk);
        write(&dir, "6 Beat with all four", &mix);
    }
}
