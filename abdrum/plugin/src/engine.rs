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
/// material, decay, brightness, position); the Noise's other inputs.
const VOICE_OUTS: usize = 21;
const TONE: usize = 0;
const SAMPLE_1: usize = 1;
const SAMPLE_2: usize = 2;
const EXCITER: usize = 3;
const BODY_LEVEL: usize = 4;
const NOISE_LEVEL: usize = 5;
const BODY_SETTINGS: usize = 6;
const NOISE_INPUTS: usize = 11;
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
        let mut inputs: [&[f32]; 11] = std::array::from_fn(|k| &voice_out[NOISE_INPUTS + k.min(9)][..n]);
        inputs[10] = &voice_out[NOISE_LEVEL][..n];
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
        for kind in 0..3 {
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
                        ("compressor.threshold", 0.0)] {
            set(&mut b, &params, id, v);
        }
        for e in [&mut a, &mut b] {
            e.note_on(60, 1.0);
            e.voices.iter_mut().for_each(|v| v.dsp.set_param(e.slots.seed, 0.5));
        }
        assert_eq!(run(&mut a, 22_050), run(&mut b, 22_050));
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
            for (id, v) in [("crush.amount", 0.2), ("drive.amount", 0.5), ("ring_mod.amount", 0.3), ("comb.amount", 0.3), ("compressor.threshold", -12.0)] {
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
