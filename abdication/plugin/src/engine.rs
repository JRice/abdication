//! The polyphonic engine: eight copies of the Faust voice, note
//! allocation, the knobs' modulators (modulation.rs; each voice moves each
//! knob its own way, once a block), and mixing. No nih-plug types here, so
//! it can be tested (and later reused) on its own.

use std::sync::Arc;

use faust_types::FaustDsp;

use crate::callbus_dsp::CallBusDsp;
use crate::modulation::{self, ModSettings, ModState, NoteClock, TARGETS};
use crate::params::{Dsp, SynthParams, Targets, VoiceSlots};
use crate::scope::{LiveBlock, LiveScope, MOVING, OUTPUTS};
pub use crate::voice::MAX_BLOCK;
use crate::voice::{Unison, VoiceUnit};

pub const NUM_VOICES: usize = 8;
/// The voice mono mode plays.
const MONO: usize = 0;

/// Notes held in mono mode, newest last (last-note priority). Fixed size,
/// so the audio thread never allocates; the oldest note drops off if
/// more than this many are held.
#[derive(Default)]
struct NoteStack {
    notes: [u8; 16],
    len: usize,
}

impl NoteStack {
    fn push(&mut self, note: u8) {
        self.remove(note);
        if self.len == self.notes.len() {
            self.notes.copy_within(1.., 0);
            self.len -= 1;
        }
        self.notes[self.len] = note;
        self.len += 1;
    }
    fn remove(&mut self, note: u8) {
        if let Some(i) = self.notes[..self.len].iter().position(|&n| n == note) {
            self.notes.copy_within(i + 1..self.len, i);
            self.len -= 1;
        }
    }
    fn last(&self) -> Option<u8> {
        self.notes[..self.len].last().copied()
    }
    fn clear(&mut self) {
        self.len = 0;
    }
}

/// Equal-tempered pitch of a MIDI note (A4 = 440 Hz).
pub fn note_freq(note: u8) -> f32 {
    440.0 * 2f32.powf((note as f32 - 69.0) / 12.0)
}

/// A released voice whose output stays below this for `FREE_AFTER` blocks
/// is considered finished and stops being computed.
const SILENCE: f32 = 1e-5;
const FREE_AFTER: u32 = 32;

/// The calls bus stops being computed once the calls are off and its output
/// has stayed below this for this many blocks (a few seconds: the echo's long
/// feedback can leave quiet stretches between repeats).
const BUS_SILENCE: f32 = 1e-6;
const BUS_FREE_AFTER: u32 = 3000;

/// Calls Level at or below this is off (the bottom of its range).
const CALLS_OFF_DB: f32 = -80.0;

/// Blocks the bus keeps running after Mute while it fades (about 20 ms per
/// e-fold), before what's left in it is cleared: 270 ms.
const BUS_FADE_BLOCKS: u32 = 200;

/// The mod wheel and aftertouch, as the modulators see them, follow their
/// controllers over this long, so their 128 steps don't zip.
const CONTROLLER_S: f32 = 0.012;

struct Voice {
    dsp: VoiceUnit,
    channel: u8,
    note: u8,
    freq: f32,
    started: u64,
    gated: bool,
    active: bool,
    quiet_blocks: u32,
    /// Each modulatable knob's modulator in this voice, and where it put the
    /// knob last block (its travel, for the panel).
    mods: [ModState; modulation::COUNT],
    moved: [f32; modulation::COUNT],
    clock: NoteClock,
    velocity: f32,
    /// Polyphonic pressure on this voice's key, and the aftertouch it feels (smoothed).
    poly_pressure: f32,
    touch: f32,
    /// The key in semitones (gliding with the portamento), for key tracking.
    key: f32,
    key_target: f32,
    /// The note started since the last block (see `render`).
    fresh: bool,
}

pub struct Engine {
    voices: Vec<Voice>,
    slots: VoiceSlots,
    clock: u64,
    rng: u32,
    /// Last channel pressure, the starting aftertouch for new notes.
    pressure: f32,
    /// Every output of the voice being computed: the voice, then the
    /// panel's taps and moving values.
    outputs: [[f32; MAX_BLOCK]; OUTPUTS],
    /// The newest voice's taps and moving values go here, for the editor.
    live: Option<Arc<LiveScope>>,
    scope_voice: Option<usize>,
    sample_rate: f32,
    /// Portamento seconds; 0 = polyphonic.
    glide: f32,
    held: NoteStack,
    /// The song position in beats, for the synced LFO: the host's while
    /// it plays, otherwise counted on at the tempo.
    beat_clock: f64,
    /// The calls' echo and reverb, shared by every voice.
    call_bus: Box<CallBusDsp>,
    /// The voices' calls for the current block, left and right.
    call_input: [[f32; MAX_BLOCK]; 2],
    calls_on: bool,
    /// Blocks the bus has been quiet with the calls off.
    bus_quiet: u32,
    /// Events Depth above 0, Calls Level above its bottom, and their mutes.
    events_up: bool,
    events_muted: bool,
    calls_up: bool,
    calls_muted: bool,
    /// Blocks since the calls were muted.
    muted_blocks: u32,
    /// Every modulatable knob's modulator settings and its own (plain) value.
    mods: [ModSettings; modulation::COUNT],
    base: [f32; modulation::COUNT],
    /// The mod wheel as the modulators see it (smoothed), and where it's going.
    wheel: f32,
    wheel_target: f32,
    tempo: f64,
}

impl Engine {
    pub fn new(slots: VoiceSlots) -> Self {
        let voices = (0..NUM_VOICES)
            .map(|i| Voice {
                dsp: VoiceUnit::default(),
                channel: 0,
                note: 0,
                freq: 0.0,
                started: 0,
                gated: false,
                active: false,
                quiet_blocks: 0,
                mods: std::array::from_fn(|t| ModState::new(1 + 97 * i as u32 + 7919 * t as u32)),
                moved: [0.0; modulation::COUNT],
                clock: NoteClock::default(),
                velocity: 0.0,
                poly_pressure: 0.0,
                touch: 0.0,
                key: 60.0,
                key_target: 60.0,
                fresh: false,
            })
            .collect();
        let base = std::array::from_fn(|t| slots.mod_travel[t].min);
        Self { voices, slots, clock: 0, rng: 0x9E37_79B9, pressure: 0.0,
               outputs: [[0.0; MAX_BLOCK]; OUTPUTS], live: None, scope_voice: None, sample_rate: 48_000.0,
               glide: 0.0, held: NoteStack::default(), beat_clock: 0.0,
               call_bus: Box::new(CallBusDsp::new()), call_input: [[0.0; MAX_BLOCK]; 2], calls_on: false,
               bus_quiet: BUS_FREE_AFTER, events_up: false, events_muted: false, calls_up: false,
               calls_muted: false, muted_blocks: 0, mods: [ModSettings::default(); modulation::COUNT], base,
               wheel: 0.0, wheel_target: 0.0, tempo: 120.0 }
    }

    /// Every parameter as `params` has it: the Faust controls and the
    /// modulators (the plugin does this each block; tests once).
    pub fn load(&mut self, params: &SynthParams) {
        for (index, value) in params.faust_values() {
            self.set_param(index, value);
        }
        for (t, settings) in params.mod_settings().enumerate() {
            self.set_mod(t, settings);
        }
    }

    /// Modulatable knob `t`'s modulator settings (modulation::TARGETS order).
    pub fn set_mod(&mut self, t: usize, settings: ModSettings) {
        self.mods[t] = settings;
    }

    /// Set one parameter by id, in plain units: a Faust control, or a
    /// modulator field ("mod.filter_cutoff.env"; menus by index, switches
    /// 0/1). For tests and tools; the plugin goes through `load`.
    pub fn set_by_id(&mut self, params: &SynthParams, id: &str, value: f32) {
        let Some((knob, field)) = id.strip_prefix("mod.").and_then(|r| r.rsplit_once('.')) else {
            let targets = params.faust_index(id).unwrap_or_else(|| panic!("no parameter '{id}'"));
            return self.set_param(targets, value);
        };
        let t = TARGETS.iter().position(|t| crate::params::mod_slug(t.id) == knob)
            .unwrap_or_else(|| panic!("no modulatable knob '{knob}'"));
        let m = &mut self.mods[t];
        match field {
            "shape" => m.shape = modulation::Shape::from_index(value as usize),
            "rate" => m.hz = value,
            "sync" => m.sync = (value > 0.5).then_some(modulation::DIVISIONS[modulation::DEFAULT_DIVISION].0),
            "division" => m.sync = m.sync.map(|_| modulation::DIVISIONS[(value as usize).min(14)].0),
            "retrig" => m.retrig = value > 0.5,
            "lfo" => m.lfo = value,
            "attack" => m.attack = value,
            "decay" => m.decay = value,
            "sustain" => m.sustain = value,
            "release" => m.release = value,
            "attack_curve" => m.attack_curve = value,
            "decay_curve" => m.decay_curve = value,
            "release_curve" => m.release_curve = value,
            "env" => m.env = value,
            "vel" => m.vel = value,
            "touch" => m.touch = value,
            "wheel" => m.wheel = value,
            "key" => m.key = value,
            "vel_to" => m.vel_to = modulation::Dest::from_index(value as usize),
            "touch_to" => m.touch_to = modulation::Dest::from_index(value as usize),
            "wheel_to" => m.wheel_to = modulation::Dest::from_index(value as usize),
            "key_to" => m.key_to = modulation::Dest::from_index(value as usize),
            _ => panic!("no modulator field '{field}'"),
        }
    }

    /// Where the newest voice's taps and moving values are copied (for the
    /// editor's live screens).
    pub fn set_live_scope(&mut self, live: Arc<LiveScope>) {
        self.live = Some(live);
    }

    pub fn init(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.call_bus.init(sample_rate as i32);
        let slot = self.slots.slot;
        for (i, v) in self.voices.iter_mut().enumerate() {
            v.dsp.init(sample_rate, &self.slots);
            // Each voice's identity for Vintage's per-voice offsets.
            v.dsp.set(slot, (i + 1) as f32);
            v.active = false;
            v.gated = false;
        }
    }

    pub fn reset(&mut self) {
        for v in &mut self.voices {
            v.dsp.instance_clear();
            v.dsp.set(self.slots.gate, 0.0);
            v.active = false;
            v.gated = false;
        }
        self.call_bus.instance_clear();
        self.bus_quiet = BUS_FREE_AFTER;
    }

    /// Set a shared (non-voice) Faust control on every voice. A modulatable
    /// knob's value is its modulators' starting point; playing voices get it
    /// moved, in `render`.
    pub fn set_param(&mut self, param: Targets, value: f32) {
        if let Some(t) = param.modulated {
            self.base[t as usize] = value;
        }
        let unison = (param == self.slots.unison).then(|| Unison::from_value(value));
        if param == self.slots.events_depth {
            self.events_up = value > 0.0;
        } else if param == self.slots.events_mute {
            self.events_muted = value > 0.5;
        } else if param == self.slots.calls_level {
            self.calls_up = value > CALLS_OFF_DB;
        } else if param == self.slots.calls_mute {
            self.calls_muted = value > 0.5;
        }
        let events_on = self.events_up && !self.events_muted;
        self.calls_on = self.calls_up && !self.calls_muted;
        let calls_on = self.calls_on;
        for v in &mut self.voices {
            v.dsp.set(param, value);
            if let Some(mode) = unison {
                v.dsp.unison = mode;
            }
            v.dsp.events_on = events_on;
            v.dsp.calls_on = calls_on;
        }
        for t in param.iter().filter(|t| t.dsp == Dsp::CallBus) {
            self.call_bus.set_param(t.index, value);
        }
    }

    /// Portamento time in seconds (from the plugin's parameter, every
    /// block). 0 = polyphonic; above 0 = mono (one voice, last-note
    /// priority) with SH-101-style auto glide: only overlapping (legato)
    /// notes glide, over that time; detached notes start on pitch.
    pub fn set_portamento(&mut self, seconds: f32) {
        if (seconds > 0.0) != self.is_mono() {
            self.held.clear();
        }
        self.glide = seconds;
    }

    /// The host's tempo and, while it plays, its position in beats, for
    /// the synced LFO; call before rendering each block of `samples`.
    /// Stopped (or without a position), the clock counts on at the tempo,
    /// so a synced LFO keeps moving.
    pub fn set_transport(&mut self, tempo: f64, playing_at: Option<f64>, samples: usize) {
        let tempo = if tempo > 0.0 { tempo } else { 120.0 };
        self.beat_clock = match playing_at {
            Some(beats) => beats,
            None => self.beat_clock + samples as f64 * tempo / 60.0 / self.sample_rate as f64,
        };
        self.tempo = tempo;
    }

    fn is_mono(&self) -> bool {
        self.glide > 0.0
    }

    pub fn note_on(&mut self, channel: u8, note: u8, velocity: f32) {
        if !self.is_mono() {
            let i = self.pick_voice();
            return self.start_voice(i, channel, note, velocity, 0.0);
        }
        self.held.push(note);
        if self.voices[MONO].gated {
            // Legato: glide to the new note without restarting anything.
            self.retune_mono(channel, note);
        } else {
            // Auto glide (SH-101 style): a detached note starts on pitch.
            self.start_voice(MONO, channel, note, velocity, 0.0);
        }
    }

    /// Start voice `i` on a note: a fresh gate edge restarts the envelopes.
    fn start_voice(&mut self, i: usize, channel: u8, note: u8, velocity: f32, glide: f32) {
        self.clock += 1;
        let slots = self.slots;
        let freq = self.freq(note);
        let seed = self.random();
        let mods = self.mods;
        let v = &mut self.voices[i];
        // The modulators start their envelopes from wherever they are (a
        // stolen voice's), and restart their LFOs if they're set to.
        for (state, settings) in v.mods.iter_mut().zip(&mods) {
            state.note_on(settings, &v.clock);
        }
        v.clock = NoteClock { gated: true, t_on: 0.0, t_off: 0.0 };
        v.velocity = velocity;
        v.poly_pressure = 0.0;
        v.key_target = note as f32;
        v.key = note as f32;
        v.fresh = true;

        // The envelope starts on a rising gate edge. A voice that's still
        // gated (stolen) needs one sample of gate-off first.
        if v.gated {
            v.dsp.set(slots.gate, 0.0);
            let mut one = [[0.0f32; 1]; OUTPUTS];
            v.dsp.compute(&mut one.each_mut().map(|o| &mut o[..]));
        }
        v.dsp.set(slots.glide, glide);
        v.dsp.set(slots.freq, freq);
        v.freq = freq;
        v.dsp.set(slots.gain, velocity);
        v.dsp.set(slots.seed, seed);
        v.dsp.set(slots.gate, 1.0);
        v.channel = channel;
        v.note = note;
        v.started = self.clock;
        v.gated = true;
        v.active = true;
        v.quiet_blocks = 0;
        self.scope_voice = Some(i);
    }

    /// Mono: move the sounding voice to `note`, gliding, gate untouched.
    fn retune_mono(&mut self, channel: u8, note: u8) {
        let (glide, freq) = (self.glide, self.freq(note));
        let slots = self.slots;
        let v = &mut self.voices[MONO];
        v.dsp.set(slots.glide, glide);
        v.dsp.set(slots.freq, freq);
        v.freq = freq;
        v.key_target = note as f32;
        v.channel = channel;
        v.note = note;
        self.scope_voice = Some(MONO);
    }

    pub fn note_off(&mut self, channel: u8, note: u8) {
        let slots = self.slots;
        if self.is_mono() {
            self.held.remove(note);
            let mono = &self.voices[MONO];
            if mono.gated && mono.note == note {
                match self.held.last() {
                    // Back to the most recent note still held.
                    Some(previous) => self.retune_mono(channel, previous),
                    None => {
                        let mods = self.mods;
                        let mono = &mut self.voices[MONO];
                        mono.dsp.set(slots.gate, 0.0);
                        mono.gated = false;
                        release(mono, &mods);
                    }
                }
            }
        }
        // Poly voices (and any still held from before switching to mono).
        let skip = if self.is_mono() { Some(MONO) } else { None };
        let mods = self.mods;
        for (_, v) in self.voices.iter_mut().enumerate()
            .filter(|(i, v)| Some(*i) != skip && v.gated && v.channel == channel && v.note == note)
        {
            v.dsp.set(slots.gate, 0.0);
            v.gated = false;
            release(v, &mods);
        }
    }

    /// Bend position -1..1; what it does is the voice's Bend Mode.
    pub fn pitch_bend(&mut self, amount: f32) {
        self.set_param(self.slots.bend, amount.clamp(-1.0, 1.0));
    }

    /// Mod wheel position 0..1: every knob's Mod Wheel amount.
    pub fn mod_wheel(&mut self, amount: f32) {
        self.wheel_target = amount.clamp(0.0, 1.0);
    }

    /// Channel pressure 0..1: every voice.
    pub fn channel_pressure(&mut self, amount: f32) {
        self.pressure = amount.clamp(0.0, 1.0);
    }

    /// Polyphonic pressure 0..1: only the voice(s) playing that note.
    pub fn poly_pressure(&mut self, channel: u8, note: u8, amount: f32) {
        for v in self.voices.iter_mut().filter(|v| v.gated && v.channel == channel && v.note == note) {
            v.poly_pressure = amount.clamp(0.0, 1.0);
        }
    }

    /// Move every modulatable knob of voice `i` for the `n` samples about to
    /// be rendered: its modulators advance to the block's end, and the voice
    /// draws a line there over the block.
    fn modulate(&mut self, i: usize, n: usize) {
        let dt = n as f32 / self.sample_rate;
        let follow = 1.0 - (-dt / CONTROLLER_S).exp();
        let (glide, tempo, beats, wheel) = (self.glide, self.tempo, self.beat_clock, self.wheel);
        let (slots, mods, base, pressure) = (&self.slots, &self.mods, &self.base, self.pressure);
        let v = &mut self.voices[i];
        if v.clock.gated {
            v.clock.t_on += dt;
        } else {
            v.clock.t_off += dt;
        }
        // The key glides as the pitch does (voice.lib: a third of the glide time per e-fold).
        v.key = if glide > 0.0 { v.key + (v.key_target - v.key) * (1.0 - (-3.0 * dt / glide).exp()) } else { v.key_target };
        v.touch += (pressure.max(v.poly_pressure) - v.touch) * follow;
        let played = modulation::Played { velocity: v.velocity, touch: v.touch, wheel, key: v.key };
        for t in 0..modulation::COUNT {
            let s = &mods[t];
            let shift = s.played(&TARGETS[t], &played);
            let moving = if s.moving() {
                // The played sources routed to the LFO's or envelope's amount move it.
                let s = ModSettings { lfo: s.lfo + shift.lfo, env: s.env + shift.env, ..*s };
                v.mods[t].step(&s, dt, &v.clock, tempo, beats)
            } else {
                0.0
            };
            let (plain, travel) = modulation::apply(&TARGETS[t], &slots.mod_travel[t], base[t], moving, &shift);
            v.dsp.set(slots.mod_targets[t], plain);
            v.moved[t] = travel;
        }
    }

    /// Render up to MAX_BLOCK samples of the mono voice mix into `out`
    /// (overwritten, not summed).
    pub fn render(&mut self, out: &mut [f32]) {
        let n = out.len().min(MAX_BLOCK);
        out[..n].fill(0.0);
        self.call_input.iter_mut().for_each(|c| c[..n].fill(0.0));
        if n == 0 {
            return;
        }
        self.wheel += (self.wheel_target - self.wheel) * (1.0 - (-(n as f32) / self.sample_rate / CONTROLLER_S).exp());
        for i in 0..NUM_VOICES {
            if !self.voices[i].active {
                continue;
            }
            // A new note's first sample plays at its modulators' starting
            // point (the knobs jump there on note-on), and the line to the
            // block's end starts from it. Otherwise the note would start a
            // block into its envelopes.
            let from = if std::mem::take(&mut self.voices[i].fresh) {
                self.modulate(i, 0);
                self.voices[i].dsp.compute(&mut self.outputs.each_mut().map(|o| &mut o[..1]));
                1
            } else {
                0
            };
            if from < n {
                self.modulate(i, n - from);
                self.voices[i].dsp.compute(&mut self.outputs.each_mut().map(|o| &mut o[from..n]));
            }
            let v = &mut self.voices[i];
            let [buf, osc1, osc2, moving @ ..] = &self.outputs;
            if let (Some(live), Some(true)) = (&self.live, self.scope_voice.map(|s| s == i)) {
                let moving: [f32; MOVING] = std::array::from_fn(|k| moving[k][n - 1]);
                live.push(LiveBlock { osc1: &osc1[..n], osc2: &osc2[..n], moving, moved: v.moved,
                                      note_hz: v.freq, sample_rate: self.sample_rate });
            }
            if self.calls_on {
                v.dsp.add_calls(n, &mut self.call_input);
            }
            let mut peak = 0.0f32;
            for (o, s) in out[..n].iter_mut().zip(buf[..n].iter()) {
                *o += *s;
                peak = peak.max(s.abs());
            }
            if !v.gated && peak < SILENCE {
                v.quiet_blocks += 1;
                if v.quiet_blocks >= FREE_AFTER {
                    v.active = false;
                }
            } else {
                v.quiet_blocks = 0;
            }
        }
    }

    /// Render the calls bus for the block `render` just rendered, into
    /// `left` and `right` (overwritten; the same length as that block). The
    /// bus takes every voice's calls, and only its output is heard.
    pub fn render_calls(&mut self, left: &mut [f32], right: &mut [f32]) {
        self.muted_blocks = if self.calls_muted { self.muted_blocks.saturating_add(1) } else { 0 };
        if self.muted_blocks == BUS_FADE_BLOCKS {
            self.call_bus.instance_clear();
            self.bus_quiet = BUS_FREE_AFTER;
        }
        if !self.calls_on && self.bus_quiet >= BUS_FREE_AFTER {
            left.fill(0.0);
            right.fill(0.0);
            return;
        }
        let n = left.len();
        let [in_left, in_right] = &self.call_input;
        self.call_bus.compute(n as i32, &[&in_left[..n], &in_right[..n]], &mut [&mut *left, &mut *right]);
        let peak = left.iter().chain(right.iter()).fold(0.0f32, |p, s| p.max(s.abs()));
        self.bus_quiet = if peak < BUS_SILENCE { self.bus_quiet + 1 } else { 0 };
    }

    pub fn active_voices(&self) -> usize {
        self.voices.iter().filter(|v| v.active).count()
    }

    /// Uniform 0..1 from a xorshift generator (per-note randomness).
    fn random(&mut self) -> f32 {
        let mut x = self.rng;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.rng = x;
        (x >> 8) as f32 / (1u32 << 24) as f32
    }

    fn freq(&self, note: u8) -> f32 {
        note_freq(note)
    }

    /// Free voice first, then the oldest released one, then the oldest.
    fn pick_voice(&self) -> usize {
        let oldest = |pred: &dyn Fn(&Voice) -> bool| {
            self.voices
                .iter()
                .enumerate()
                .filter(|(_, v)| pred(v))
                .min_by_key(|(_, v)| v.started)
                .map(|(i, _)| i)
        };
        oldest(&|v| !v.active)
            .or_else(|| oldest(&|v| !v.gated))
            .or_else(|| oldest(&|_| true))
            .expect("at least one voice")
    }
}

/// A voice's note ends: its modulators' envelopes start their release.
fn release(v: &mut Voice, mods: &[ModSettings; modulation::COUNT]) {
    for (state, settings) in v.mods.iter_mut().zip(mods) {
        state.note_off(settings, &v.clock);
    }
    v.clock.gated = false;
    v.clock.t_off = 0.0;
}

/// Basic peak limiter for the master output: instant attack, exponential
/// release. The envelope never falls below the current sample's level, so
/// the output can't exceed the ceiling.
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
    pub const CEILING_DB: f32 = -1.5;
    pub const RELEASE_S: f32 = 0.05;

    pub fn new() -> Self {
        Self { ceiling: 10f32.powf(Self::CEILING_DB / 20.0), release: 0.0, env: 0.0 }
    }

    pub fn init(&mut self, sample_rate: f32) {
        self.release = (-1.0 / (Self::RELEASE_S * sample_rate)).exp();
        self.env = 0.0;
    }

    /// A stereo pair, reduced together so that the image holds.
    pub fn process_stereo(&mut self, left: f32, right: f32) -> (f32, f32) {
        if !left.is_finite() || !right.is_finite() {
            return (0.0, 0.0);
        }
        self.env = left.abs().max(right.abs()).max(self.env * self.release);
        if self.env > self.ceiling {
            let scale = self.ceiling / self.env;
            (left * scale, right * scale)
        } else {
            (left, right)
        }
    }

    pub fn process(&mut self, x: f32) -> f32 {
        // A NaN or infinity would poison the envelope (and the output)
        // for good; drop it instead.
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
    use crate::params::SynthParams;

    fn engine() -> (Engine, SynthParams) {
        let params = SynthParams::from_faust();
        let mut e = Engine::new(params.slots());
        e.init(48_000.0);
        e.load(&params);
        (e, params)
    }

    fn run(e: &mut Engine, blocks: usize) -> (f32, f32, bool) {
        let (mut peak, mut energy, mut bad) = (0.0f32, 0.0f32, false);
        let mut buf = [0.0; MAX_BLOCK];
        for _ in 0..blocks {
            e.render(&mut buf);
            for s in buf {
                bad |= !s.is_finite();
                peak = peak.max(s.abs());
                energy += s * s;
            }
        }
        (peak, (energy / (blocks * MAX_BLOCK) as f32).sqrt(), bad)
    }

    #[test]
    fn silent_until_played() {
        let (mut e, _) = engine();
        let (peak, _, bad) = run(&mut e, 100);
        assert!(!bad);
        assert_eq!(peak, 0.0);
    }

    #[test]
    fn chord_sounds_and_releases() {
        let (mut e, _) = engine();
        for n in [48, 52, 55, 59] {
            e.note_on(0, n, 0.8);
        }
        let (peak, rms, bad) = run(&mut e, 750); // 1 s
        assert!(!bad);
        assert!(rms > 0.01 && peak < 4.0, "rms {rms}, peak {peak}");
        assert_eq!(e.active_voices(), 4);

        for n in [48, 52, 55, 59] {
            e.note_off(0, n);
        }
        run(&mut e, 3000); // 4 s, well past the release
        assert_eq!(e.active_voices(), 0, "released voices should be freed");
    }

    #[test]
    fn unison_keeps_level_and_randomizes_phase() {
        let (mut e, params) = engine();
        let on = params.faust_index("unison.mode").unwrap();
        let random = params.faust_index("unison.random_phase").unwrap();
        // Both oscillators: Osc 2 defaults to silent.
        e.set_param(params.faust_index("mix.osc_2").unwrap(), 0.5);

        let level = |e: &mut Engine, unison: f32| {
            e.set_param(on, unison);
            e.note_on(0, 45, 0.8);
            let (_, rms, bad) = run(e, 375);
            e.note_off(0, 45);
            run(e, 3000);
            assert!(!bad);
            rms
        };
        let single = level(&mut e, 0.0);
        for (mode, name) in [(1.0, "U5"), (2.0, "U7")] {
            let ratio_db = 20.0 * (level(&mut e, mode) / single).log10();
            assert!(ratio_db.abs() < 4.0, "{name} changed level by {ratio_db:.1} dB");
        }

        // With random phase on, two hits of the same note start differently.
        e.set_param(random, 1.0);
        let mut first = [0.0; MAX_BLOCK];
        let mut second = [0.0; MAX_BLOCK];
        e.note_on(0, 45, 0.8);
        e.render(&mut first);
        e.note_off(0, 45);
        run(&mut e, 3000);
        e.note_on(0, 45, 0.8);
        e.render(&mut second);
        assert_ne!(first, second);
    }

    /// Worst case: 8 notes, unison on. Run with
    /// `cargo test --release -- --ignored --nocapture cpu`.
    #[test]
    #[ignore]
    fn cpu_worst_case() {
        let (mut e, params) = engine();
        // ABD_UNISON=0 measures without unison, ABD_UNISON=5 with U5.
        let copies = match std::env::var("ABD_UNISON").as_deref() {
            Ok("0") => 1,
            Ok("5") => 5,
            _ => 7,
        };
        let mode = match copies { 1 => 0.0, 5 => 1.0, _ => 2.0 };
        e.set_param(params.faust_index("unison.mode").unwrap(), mode);
        e.set_param(params.faust_index("mix.osc_2").unwrap(), 1.0);
        // ABD_OSC2=0 plays a Dual in Osc 2 as well; ABD_OSC1=1 a Wavetable in Osc 1.
        if std::env::var("ABD_OSC2").as_deref() == Ok("0") {
            e.set_param(params.faust_index("osc_2.mode").unwrap(), 0.0);
        }
        if std::env::var("ABD_OSC1").as_deref() == Ok("1") {
            e.set_param(params.faust_index("osc_1.mode").unwrap(), 1.0);
        }
        // ABD_DUAL_ALGO=<index> measures a particular Dual algorithm.
        if let Some(algo) = std::env::var("ABD_DUAL_ALGO").ok().and_then(|a| a.parse::<f32>().ok()) {
            e.set_param(params.faust_index("dual_1.algorithm").unwrap(), algo);
            e.set_param(params.faust_index("dual_2.algorithm").unwrap(), algo);
        }
        // ABD_MODS=1: every knob's LFO and envelope running, and every played source set.
        if std::env::var("ABD_MODS").is_ok() {
            for m in e.mods.iter_mut() {
                (m.lfo, m.env, m.vel, m.touch, m.wheel) = (0.05, 0.05, 0.05, 0.05, 0.05);
                m.shape = modulation::Shape::Brown;
            }
        }
        for n in 48..56 {
            e.note_on(0, n, 0.8);
        }
        let seconds: f32 = std::env::var("ABD_SECONDS").ok().and_then(|s| s.parse().ok()).unwrap_or(10.0);
        let blocks = (48_000.0 * seconds) as usize / MAX_BLOCK;
        let t = std::time::Instant::now();
        run(&mut e, blocks);
        let used = t.elapsed().as_secs_f32();
        println!("8 voices x {copies} unison: {:.1}% of one core in real time", 100.0 * used / seconds);
    }

    /// A sustained, filter-open chord on a saw; each `take` renders it afresh.
    fn texture_engine() -> (Engine, SynthParams) {
        let (mut e, params) = engine();
        for (id, v) in [("mix.noise", 0.0), ("filter.cutoff", 8000.0), ("mod.filter_cutoff.env", 0.0),
                        ("drive.amount", 0.0), ("amp.sustain", 1.0), ("amp.sustain", 1.0)] {
            e.set_by_id(&params, id, v);
        }
        (e, params)
    }

    fn set(e: &mut Engine, params: &SynthParams, id: &str, v: f32) {
        e.set_by_id(&params, id, v);
    }

    /// Events at Depth 0 (the default) leave a voice alone: the texture DSP
    /// isn't even run. Raised, they change the sound but not its average level.
    #[test]
    fn events_move_the_spectrum_and_hold_the_level() {
        let take = |depth: f32| {
            let (mut e, params) = texture_engine();
            set(&mut e, &params, "events.depth", depth);
            for n in [36, 43, 48, 55] {
                e.note_on(0, n, 0.8);
            }
            run(&mut e, 750 * 6); // let the leveler settle
            let mut out = Vec::new();
            let mut buf = [0.0; MAX_BLOCK];
            for _ in 0..750 * 10 {
                e.render(&mut buf);
                out.extend_from_slice(&buf);
            }
            out
        };
        let rms = |x: &[f32]| (x.iter().map(|s| s * s).sum::<f32>() / x.len() as f32).sqrt();
        let (off, on, deep) = (take(0.0), take(6.0), take(12.0));
        assert!(off.iter().chain(&on).chain(&deep).all(|s| s.is_finite()));
        assert_ne!(off, on, "events change nothing");
        for (name, x) in [("6 dB", &on), ("12 dB", &deep)] {
            let change = 20.0 * (rms(x) / rms(&off)).log10();
            println!("depth {name}: level {change:+.2} dB");
            assert!(change.abs() < 1.5, "depth {name} moved the level by {change:.2} dB");
        }
    }

    /// Calls Level at the bottom is silent; raised, the calls come out of the
    /// bus and stay in it after the notes end. The bus then stops.
    #[test]
    fn calls_sound_through_the_bus_and_ring_on() {
        let (mut e, params) = texture_engine();
        let mut buf = [0.0; MAX_BLOCK];
        let (mut left, mut right) = ([0.0; MAX_BLOCK], [0.0; MAX_BLOCK]);
        let mut peak_of = |e: &mut Engine, blocks: usize| {
            let (mut peak, mut differ) = (0.0f32, false);
            for _ in 0..blocks {
                e.render(&mut buf);
                e.render_calls(&mut left, &mut right);
                for (l, r) in left.iter().zip(&right) {
                    assert!(l.is_finite() && r.is_finite());
                    peak = peak.max(l.abs()).max(r.abs());
                    differ |= l != r;
                }
            }
            (peak, differ)
        };
        e.note_on(0, 48, 0.8);
        assert_eq!(peak_of(&mut e, 750 * 2).0, 0.0, "calls are off by default");

        set(&mut e, &params, "calls.level", -30.0);
        set(&mut e, &params, "calls.activity", 1.0);
        set(&mut e, &params, "calls.echo", 1.0);
        let (peak, stereo) = peak_of(&mut e, 750 * 20);
        println!("calls at -30 dB: peak {peak:.4}");
        assert!(peak > 1e-3 && peak < 1.0, "peak {peak}");
        assert!(stereo, "the bus should be stereo");

        e.note_off(0, 48);
        set(&mut e, &params, "calls.level", -80.0);
        run(&mut e, 750 * 3);
        let (tail, _) = peak_of(&mut e, 750);
        assert!(tail > 1e-5, "the echo should ring on after the calls stop: {tail}");

        peak_of(&mut e, 750 * 120);
        assert_eq!(peak_of(&mut e, 10).0, 0.0, "the bus should stop in the end");
        assert!(e.bus_quiet >= BUS_FREE_AFTER);
    }

    /// A held chord with the events up, muted or not, as `render` gives it.
    fn events_chord(mute: bool, depth: f32, unmute_after: Option<usize>) -> Vec<f32> {
        let (mut e, params) = texture_engine();
        set(&mut e, &params, "events.depth", depth);
        set(&mut e, &params, "events.mute", if mute { 1.0 } else { 0.0 });
        for n in [36, 43, 48, 55] {
            e.note_on(0, n, 0.8);
        }
        let mut out = Vec::new();
        let mut buf = [0.0; MAX_BLOCK];
        for b in 0..750 * 3 {
            if Some(b) == unmute_after {
                set(&mut e, &params, "events.mute", if mute { 0.0 } else { 1.0 });
            }
            e.render(&mut buf);
            out.extend_from_slice(&buf);
        }
        out
    }

    /// Muted events are the dry voice, sample for sample, and muting them
    /// partway crossfades to it without a click.
    #[test]
    fn events_mute_is_the_dry_voice() {
        let dry = events_chord(false, 0.0, None);
        assert_eq!(dry, events_chord(true, 12.0, None), "muted from the start");

        let at = 750; // a second in
        let muted_late = events_chord(false, 12.0, Some(at));
        let settled = (at + 300) * MAX_BLOCK;
        assert_eq!(&muted_late[settled..], &dry[settled..], "after the fade");
        let jump = |x: &[f32]| x.windows(2).fold(0.0f32, |j, w| j.max((w[1] - w[0]).abs()));
        let around = &muted_late[(at - 8) * MAX_BLOCK..(at + 300) * MAX_BLOCK];
        let before = &muted_late[(at - 158) * MAX_BLOCK..(at - 8) * MAX_BLOCK];
        println!("largest step: {:.4} before muting, {:.4} across it", jump(before), jump(around));
        assert!(jump(around) < 1.5 * jump(before), "a click at the mute");
    }

    /// Mute silences the calls and their tail, and nothing old comes back.
    #[test]
    fn calls_mute_takes_the_tail_with_it() {
        let (mut e, params) = texture_engine();
        let (mut buf, mut left, mut right) = ([0.0; MAX_BLOCK], [0.0; MAX_BLOCK], [0.0; MAX_BLOCK]);
        let mut peak_of = |e: &mut Engine, blocks: usize| {
            let mut peak = 0.0f32;
            for _ in 0..blocks {
                e.render(&mut buf);
                e.render_calls(&mut left, &mut right);
                peak = left.iter().chain(&right).fold(peak, |p, s| p.max(s.abs()));
            }
            peak
        };
        set(&mut e, &params, "calls.level", -30.0);
        set(&mut e, &params, "calls.activity", 1.0);
        set(&mut e, &params, "calls.echo", 1.0);
        e.note_on(0, 48, 0.8);
        assert!(peak_of(&mut e, 750 * 20) > 1e-3);
        e.note_off(0, 48);
        peak_of(&mut e, 750 * 3); // the voice is gone; its echoes ring on
        assert!(peak_of(&mut e, 750) > 1e-5, "the tail should still be ringing");

        set(&mut e, &params, "calls.mute", 1.0);
        peak_of(&mut e, BUS_FADE_BLOCKS as usize);
        assert_eq!(peak_of(&mut e, 750), 0.0, "muted");

        set(&mut e, &params, "calls.mute", 0.0);
        assert!(peak_of(&mut e, 750) < 1e-9, "the old tail came back");

        // Playing again needs nothing else.
        e.note_on(0, 48, 0.8);
        assert!(peak_of(&mut e, 750 * 20) > 1e-3, "calls should sound after unmuting");
    }

    /// Noise's Mute silences it, and the rest of the mix holds its level.
    #[test]
    fn noise_mute_leaves_the_balance_alone() {
        let take = |osc: f32, noise_mute: f32| {
            let (mut e, params) = texture_engine();
            set(&mut e, &params, "mix.osc_1", osc);
            set(&mut e, &params, "mix.noise", 1.0);
            set(&mut e, &params, "noise.type", 0.0);
            set(&mut e, &params, "noise.mute", noise_mute);
            e.note_on(0, 48, 0.8);
            run(&mut e, 750).1
        };
        assert!(take(0.0, 1.0) < 1e-6, "muted noise is silent");
        assert!(take(0.0, 0.0) > 1e-3, "noise should sound unmuted");
        let (with_noise_up, muted) = (take(1.0, 0.0), take(1.0, 1.0));
        println!("rms: noise up {with_noise_up:.3}, muted {muted:.3}");
        assert!(muted > 0.3 * with_noise_up, "muting noise shouldn't change the balance of the rest");
    }

    /// Noise Move wanders the noise's level; at 0 it's the plain noise.
    #[test]
    fn noise_move_wanders() {
        let spread = |move_amount: f32| {
            let (mut e, params) = texture_engine();
            set(&mut e, &params, "mix.osc_1", 0.0);
            set(&mut e, &params, "mix.noise", 1.0);
            set(&mut e, &params, "noise.type", 0.0);
            set(&mut e, &params, "noise.move", move_amount);
            e.note_on(0, 48, 0.8);
            run(&mut e, 750);
            let mut buf = [0.0; MAX_BLOCK];
            let mut levels = Vec::new();
            for _ in 0..100 {
                let mut energy = 0.0;
                for _ in 0..75 {
                    e.render(&mut buf);
                    energy += buf.iter().map(|s| s * s).sum::<f32>();
                }
                levels.push(energy);
            }
            let (lo, hi) = levels.iter().fold((f32::MAX, 0f32), |(l, h), &x| (l.min(x), h.max(x)));
            10.0 * (hi / lo).log10()
        };
        let (still, moving) = (spread(0.0), spread(1.0));
        println!("noise level range over 10 s: still {still:.1} dB, moving {moving:.1} dB");
        assert!(still < 3.0 && moving > 9.0);
    }

    #[test]
    fn stereo_limiter_holds_the_ceiling_and_the_image() {
        let mut l = Limiter::new();
        l.init(48_000.0);
        let ceiling = 10f32.powf(Limiter::CEILING_DB / 20.0);
        for i in 0..48_000 {
            let x = 4.0 * (i as f32 * 0.05).sin();
            let (a, b) = l.process_stereo(x, x * 0.5);
            assert!(a.abs() <= ceiling + 1e-6 && b.abs() <= ceiling + 1e-6);
            assert!((b - a * 0.5).abs() < 1e-5);
        }
    }

    /// One held note's output, as a sustained sine through an open filter.
    fn vintage_sine(e: &mut Engine, params: &SynthParams, vintage: f32, note: u8, blocks: usize) -> Vec<f32> {
        set(e, params, "vintage.amount", vintage);
        set(e, params, "dual_1.a", 0.0);
        set(e, params, "filter.cutoff", 18000.0);
        e.note_on(0, note, 0.8);
        run(e, 750); // past the attack
        let mut out = Vec::with_capacity(blocks * MAX_BLOCK);
        let mut buf = [0.0; MAX_BLOCK];
        for _ in 0..blocks {
            e.render(&mut buf);
            out.extend_from_slice(&buf);
        }
        out
    }

    /// Short-window level of a sine, in dB, one value per `window` samples.
    fn levels_db(x: &[f32], window: usize) -> Vec<f32> {
        x.chunks_exact(window)
            .map(|c| 10.0 * (c.iter().map(|s| s * s).sum::<f32>() / window as f32).log10())
            .collect()
    }

    /// Vintage's noise on level: none at 0, a little at the top, and each
    /// voice's its own. Its noise on pitch is measured from dumps: see below.
    #[test]
    fn vintage_noise_moves_level_per_voice() {
        let spread = |x: &[f32]| {
            let mean = x.iter().sum::<f32>() / x.len() as f32;
            (x.iter().map(|v| (v - mean) * (v - mean)).sum::<f32>() / x.len() as f32).sqrt()
        };
        let (mut e, params) = texture_engine();
        let window = 960; // 20 ms: 4.4 cycles of the note, which is held still enough
        let still = levels_db(&vintage_sine(&mut e, &params, 0.0, 69, 750 * 4), window);
        let (mut e, params) = texture_engine();
        let moved = levels_db(&vintage_sine(&mut e, &params, 1.0, 69, 750 * 4), window);
        println!("level spread: Vintage 0 {:.3} dB, Vintage 1 {:.3} dB", spread(&still), spread(&moved));
        assert!(spread(&still) < 0.1);
        assert!(spread(&moved) > 0.2 && spread(&moved) < 1.2);
    }

    /// Diagnostic: dumps sines at Vintage 0 and 1, two voices at once, for
    /// pitch and level analysis. `ABD_DUMP=<dir> cargo test --release -- --ignored dump_vintage`
    #[test]
    #[ignore]
    fn dump_vintage_noise() {
        let dir = std::env::var("ABD_DUMP").expect("set ABD_DUMP");
        for (name, note, vintage) in [("v0_a2", 45u8, 0.0), ("v1_a2", 45, 1.0), ("v03_a2", 45, 0.3), ("v1_a5", 81, 1.0)] {
            let (mut e, params) = texture_engine();
            let out = vintage_sine(&mut e, &params, vintage, note, 750 * 30);
            let bytes: Vec<u8> = out.iter().flat_map(|v| v.to_le_bytes()).collect();
            std::fs::write(format!("{dir}/{name}.f32"), bytes).unwrap();
        }
        // Each voice alone, on one note: voices 0 and 1 of the same engine.
        let (mut e, params) = texture_engine();
        set(&mut e, &params, "vintage.amount", 1.0);
        set(&mut e, &params, "dual_1.a", 0.0);
        set(&mut e, &params, "filter.cutoff", 18000.0);
        for (v, name) in [(0usize, "voice0"), (1, "voice1")] {
            e.voices[v].dsp.set(e.slots.freq, 440.0);
            e.voices[v].dsp.set(e.slots.gain, 0.8);
            e.voices[v].dsp.set(e.slots.gate, 1.0);
            let mut out = Vec::new();
            let mut buf = [[0.0f32; MAX_BLOCK]; OUTPUTS];
            for _ in 0..750 * 30 {
                e.voices[v].dsp.compute(&mut buf.each_mut().map(|o| &mut o[..]));
                out.extend_from_slice(&buf[0]);
            }
            let bytes: Vec<u8> = out.iter().flat_map(|s| s.to_le_bytes()).collect();
            std::fs::write(format!("{dir}/{name}.f32"), bytes).unwrap();
        }
    }

    #[test]
    fn limiter_holds_the_ceiling() {
        let mut l = Limiter::new();
        l.init(48_000.0);
        let ceiling = 10f32.powf(Limiter::CEILING_DB / 20.0);
        let mut peak = 0.0f32;
        for i in 0..48_000 {
            let x = 4.0 * (i as f32 * 0.05).sin();
            peak = peak.max(l.process(x).abs());
        }
        assert!(peak <= ceiling + 1e-6, "peak {peak} over ceiling {ceiling}");
        // Quiet signals pass untouched once the release has recovered.
        for _ in 0..48_000 {
            l.process(0.0);
        }
        assert_eq!(l.process(0.25), 0.25);
    }

    #[test]
    fn mod_wheel_and_bend_change_the_sound() {
        let (mut e, _) = engine();
        let take = |e: &mut Engine| {
            e.note_on(0, 57, 0.8);
            run(e, 100);
            let mut buf = [0.0; MAX_BLOCK];
            e.render(&mut buf);
            e.note_off(0, 57);
            run(e, 3000);
            buf
        };
        let plain = take(&mut e);
        e.pitch_bend(1.0);
        let bent = take(&mut e);
        e.pitch_bend(0.0);
        e.mod_wheel(1.0);
        let wheeled = take(&mut e);
        assert_ne!(plain, bent);
        assert_ne!(plain, wheeled);
    }

    /// Out of the box, the mod wheel brings in vibrato: Global Pitch's LFO
    /// (5 Hz) deepens to 25 cents with the wheel at the top, and sits still
    /// with it down.
    #[test]
    fn mod_wheel_brings_in_vibrato() {
        let swing = |wheel: f32| {
            let (mut e, params) = engine();
            let live = Arc::new(LiveScope::default());
            e.set_live_scope(live.clone());
            e.mod_wheel(wheel);
            e.note_on(0, 60, 1.0);
            let travel = params.slots().mod_travel[modulation::PITCH];
            let mut buf = [0.0; MAX_BLOCK];
            let cents: Vec<f32> = (0..750).map(|_| {
                e.render(&mut buf);
                100.0 * travel.plain(live.snapshot_since(0).unwrap().moved[modulation::PITCH])
            }).collect();
            let (lo, hi) = cents[100..].iter().fold((f32::MAX, f32::MIN), |(lo, hi), c| (lo.min(*c), hi.max(*c)));
            // Upward zero crossings in the last second: the rate.
            let crossings = cents.windows(2).filter(|w| w[0] < 0.0 && w[1] >= 0.0).count();
            (lo, hi, crossings)
        };
        let (lo, hi, _) = swing(0.0);
        assert!(lo.abs() < 0.01 && hi.abs() < 0.01, "pitch moves with the wheel down: {lo}..{hi} cents");
        let (lo, hi, crossings) = swing(1.0);
        assert!((hi - 25.0).abs() < 1.5 && (lo + 25.0).abs() < 1.5, "vibrato {lo}..{hi} cents");
        assert!((4..=6).contains(&crossings), "{crossings} cycles a second");
    }

    /// Velocity, aftertouch, the mod wheel and the key each move a knob by
    /// their own amount, per voice: the cutoff here.
    #[test]
    fn played_sources_move_their_knobs() {
        use crate::scope::Moving;
        let cutoff_after = |sets: &[(&str, f32)], play: &dyn Fn(&mut Engine)| {
            let (mut e, params) = engine();
            e.set_by_id(&params, "mod.filter_cutoff.env", 0.0);
            e.set_by_id(&params, "mod.filter_cutoff.key", 0.0);
            for (id, v) in sets {
                e.set_by_id(&params, id, *v);
            }
            let live = Arc::new(LiveScope::default());
            e.set_live_scope(live.clone());
            play(&mut e);
            run(&mut e, 40);
            live.snapshot_since(0).unwrap().get(Moving::Cutoff)
        };
        let plain = cutoff_after(&[], &|e| e.note_on(0, 60, 1.0));
        assert!((plain / 1200.0 - 1.0).abs() < 0.01, "{plain}");
        let soft = cutoff_after(&[("mod.filter_cutoff.vel", 0.3)], &|e| e.note_on(0, 60, 0.2));
        let hard = cutoff_after(&[("mod.filter_cutoff.vel", 0.3)], &|e| e.note_on(0, 60, 1.0));
        assert!(hard > soft * 2.0 && soft > plain, "velocity: {soft} .. {hard}");
        let pressed = cutoff_after(&[("mod.filter_cutoff.touch", -0.3)], &|e| { e.channel_pressure(1.0); e.note_on(0, 60, 1.0) });
        assert!(pressed < plain * 0.5, "aftertouch: {pressed}");
        let wheeled = cutoff_after(&[("mod.filter_cutoff.wheel", 0.3)], &|e| { e.mod_wheel(1.0); e.note_on(0, 60, 1.0) });
        assert!(wheeled > plain * 2.0, "mod wheel: {wheeled}");
        // The cutoff's key tracking is in octaves: one per octave at 100 %.
        let up = cutoff_after(&[("mod.filter_cutoff.key", 1.0)], &|e| e.note_on(0, 72, 1.0));
        assert!((up / plain - 2.0).abs() < 0.02, "key: {up}");
    }

    /// Diagnostic: renders wavetable-only notes to raw f32 files for
    /// analysis. `ABD_DUMP=<dir> cargo test --release -- --ignored dump`
    #[test]
    #[ignore]
    fn dump_wavetable_notes() {
        let dir = std::env::var("ABD_DUMP").expect("set ABD_DUMP");
        let (mut e, params) = engine();
        let set = |e: &mut Engine, id: &str, v: f32| e.set_by_id(&params, id, v);
        set(&mut e, "mix.osc_1", 0.0);
        set(&mut e, "mix.noise", 0.0);
        set(&mut e, "mix.osc_2", 1.0);
        set(&mut e, "filter.cutoff", 18000.0);
        set(&mut e, "mod.filter_cutoff.env", 0.0);
        set(&mut e, "drive.amount", 0.0);
        set(&mut e, "amp.sustain", 1.0);
        // One wave per take, in all four slots.
        for (name, note, wave) in [("c2_a8_002", 36, 1.0), ("c5_a8_002", 72, 1.0), ("c7_a8_002", 96, 1.0),
                                   ("c3_m8_03", 48, 120.0), ("c3_m16_31", 48, 200.0)] {
            for slot in ["wt_2.wave_1", "wt_2.wave_2", "wt_2.wave_3", "wt_2.wave_4"] {
                set(&mut e, slot, wave);
            }
            run(&mut e, 2000);
            e.note_on(0, note, 1.0);
            let mut out = Vec::new();
            let mut buf = [0.0f32; MAX_BLOCK];
            for _ in 0..1500 {
                e.render(&mut buf);
                out.extend_from_slice(&buf);
            }
            e.note_off(0, note);
            run(&mut e, 4000);
            let bytes: Vec<u8> = out.iter().flat_map(|v| v.to_le_bytes()).collect();
            std::fs::write(format!("{dir}/{name}.f32"), bytes).unwrap();
        }
    }

    /// Every Dual algorithm with every pair of waves renders finite, audible
    /// output, and A, B and the waves each change the sound. Prints levels
    /// with `--nocapture`.
    #[test]
    fn dual_algorithms_behave() {
        let (mut e, params) = engine();
        let set = |e: &mut Engine, id: &str, v: f32| e.set_by_id(&params, id, v);
        for (id, v) in [("mix.noise", 0.0),
                        ("filter.cutoff", 18000.0), ("mod.filter_cutoff.env", 0.0), ("drive.amount", 0.0),
                        ("amp.sustain", 1.0)] {
            set(&mut e, id, v);
        }
        let render = |e: &mut Engine, algo: usize, (w1, w2): (usize, usize), a: f32, b: f32| {
            set(e, "dual_1.algorithm", algo as f32);
            set(e, "dual_1.wave_1", w1 as f32);
            set(e, "dual_1.wave_2", w2 as f32);
            set(e, "dual_1.a", a);
            set(e, "dual_1.b", b);
            e.reset();
            run(e, 300);
            e.note_on(0, 48, 1.0);
            let mut out = Vec::new();
            let mut buf = [0.0f32; MAX_BLOCK];
            for _ in 0..150 {
                e.render(&mut buf);
                out.extend_from_slice(&buf);
            }
            e.note_off(0, 48);
            run(e, 3000);
            out
        };
        let rms = |x: &[f32]| (x.iter().map(|v| v * v).sum::<f32>() / x.len() as f32).sqrt();
        let differs = |x: &[f32], y: &[f32]| {
            let d: Vec<f32> = x.iter().zip(y).map(|(p, q)| p - q).collect();
            rms(&d) > 0.02 * rms(y).max(1e-6)
        };
        let names = ["VA Sweep", "Spread", "PWM", "Sync", "Fractal", "Window Amp Sync", "Metal", "Ring Mod",
                     "FM", "Noise AM", "Chaos", "Folding"];
        for (algo, name) in names.iter().enumerate() {
            let mut levels = Vec::new();
            for pair in (0..16).map(|k| (k / 4, k % 4)) {
                let out = render(&mut e, algo, pair, 0.5, 0.5);
                assert!(out.iter().all(|v| v.is_finite()), "{name} {pair:?}: non-finite");
                assert!(rms(&out) > 0.01, "{name} {pair:?}: silent ({})", rms(&out));
                levels.push(format!("{:.2}", rms(&out)));
            }
            println!("{name:16} rms by wave pair: {}", levels.join(" "));
            let base = render(&mut e, algo, (2, 2), 0.5, 0.5);
            assert!(differs(&render(&mut e, algo, (2, 2), 0.95, 0.5), &base), "{name}: A does nothing");
            assert!(differs(&render(&mut e, algo, (2, 2), 0.5, 0.95), &base), "{name}: B does nothing");
            let (first, second) = (differs(&render(&mut e, algo, (0, 2), 0.5, 0.5), &base),
                                   differs(&render(&mut e, algo, (2, 0), 0.5, 0.5), &base));
            println!("{name:16} wave 1 {}, wave 2 {}", if first { "used" } else { "unused" },
                     if second { "used" } else { "unused" });
            assert!(first || second, "{name}: the waves do nothing");
        }
    }

    /// The wavetable's Position morphs through its four waves: 0, 1/3, 2/3
    /// and 1 are waves 1..4 alone, and between them it blends two.
    #[test]
    fn wavetable_position_morphs_four_waves() {
        let (mut e, params) = engine();
        let set = |e: &mut Engine, id: &str, v: f32| e.set_by_id(&params, id, v);
        for (id, v) in [("mix.osc_1", 0.0), ("mix.noise", 0.0), ("mix.osc_2", 1.0),
                        ("filter.cutoff", 18000.0), ("mod.filter_cutoff.env", 0.0), ("drive.amount", 0.0),
                        ("amp.attack", 0.001), ("amp.sustain", 1.0)] {
            set(&mut e, id, v);
        }
        let take = |e: &mut Engine, waves: [f32; 4], position: f32| {
            for (k, w) in waves.iter().enumerate() {
                set(e, &format!("wt_2.wave_{}", k + 1), *w);
            }
            set(e, "wt_2.position", position);
            e.reset();
            run(e, 300);
            e.note_on(0, 48, 1.0);
            let mut out = Vec::new();
            let mut buf = [0.0f32; MAX_BLOCK];
            for _ in 0..(48_000 / 5 / MAX_BLOCK) {                         // 0.2 s
                e.render(&mut buf);
                out.extend_from_slice(&buf);
            }
            e.note_off(0, 48);
            run(e, 3000);
            out
        };
        let rms = |x: &[f32]| (x.iter().map(|v| v * v).sum::<f32>() / x.len() as f32).sqrt();
        let diff = |a: &[f32], b: &[f32]| {
            let d: Vec<f32> = a.iter().zip(b).map(|(x, y)| x - y).collect();
            rms(&d) / rms(b).max(1e-9)
        };
        let waves = [217.0, 218.0, 220.0, 219.0];                          // Sine, Triangle, Square, Saw
        let alone: Vec<Vec<f32>> = waves.iter().map(|w| take(&mut e, [*w; 4], 0.0)).collect();
        for (k, position) in [0.0, 1.0 / 3.0, 2.0 / 3.0, 1.0].iter().enumerate() {
            let d = diff(&take(&mut e, waves, *position), &alone[k]);
            assert!(d < 0.01, "position {position:.2} isn't wave {} alone ({d:.4})", k + 1);
        }
        let between = take(&mut e, waves, 0.5);
        assert!(diff(&between, &alone[1]) > 0.1 && diff(&between, &alone[2]) > 0.1, "0.5 should blend waves 2 and 3");
    }

    /// The multidrive sweeps soft -> hard -> tube -> fold without jumps:
    /// neighbouring settings sound close, the level stays in a sane range,
    /// and each quarter of the knob sounds different from the others.
    #[test]
    fn multidrive_sweeps_smoothly() {
        let (mut e, params) = engine();
        let set = |e: &mut Engine, id: &str, v: f32| e.set_by_id(&params, id, v);
        for (id, v) in [("filter.cutoff", 18000.0), ("mod.filter_cutoff.env", 0.0), ("amp.sustain", 1.0),
                        ("mix.noise", 0.0)] {
            set(&mut e, id, v);
        }
        let take = |e: &mut Engine, drive: f32| {
            set(e, "drive.amount", drive);
            e.reset();
            run(e, 300);
            e.note_on(0, 45, 1.0);
            let mut out = Vec::new();
            let mut buf = [0.0f32; MAX_BLOCK];
            for _ in 0..150 {
                e.render(&mut buf);
                out.extend_from_slice(&buf);
            }
            e.note_off(0, 45);
            run(e, 3000);
            out[MAX_BLOCK * 50..].to_vec()
        };
        let rms = |x: &[f32]| (x.iter().map(|v| v * v).sum::<f32>() / x.len() as f32).sqrt();
        let db = |x: f32| 20.0 * x.log10();
        let takes: Vec<Vec<f32>> = (0..=40).map(|i| take(&mut e, i as f32 / 40.0)).collect();
        let reference = rms(&takes[0]);
        for (i, pair) in takes.windows(2).enumerate() {
            assert!(pair[1].iter().all(|v| v.is_finite()));
            let step = (db(rms(&pair[1])) - db(rms(&pair[0]))).abs();
            // The fold's onset is steep on a bright saw (about 0.9 dB per
            // 1/80 of the knob there, halving with the step: a slope, not a jump).
            assert!(step < 2.0, "level jumps {step:.2} dB between {} and {}", i, i + 1);
            let level = db(rms(&pair[1])) - db(reference);
            assert!(level.abs() < 8.0, "drive {} is {level:.1} dB off", i + 1);
        }
        let shape_diff = |a: &[f32], b: &[f32]| {
            let d: Vec<f32> = a.iter().zip(b).map(|(x, y)| x / rms(a) - y / rms(b)).collect();
            rms(&d)
        };
        // Quarter centres (soft, hard, tube, fold) differ in shape.
        for (a, b) in [(5, 15), (15, 25), (25, 35)] {
            assert!(shape_diff(&takes[a], &takes[b]) > 0.05, "settings {a} and {b} sound alike");
        }
    }

    /// The newest voice's Osc 2 (a Wavetable) output reaches the live scope while it
    /// plays, and stops arriving once the voice has finished.
    #[test]
    fn live_scope_follows_the_newest_voice() {
        let (mut e, params) = engine();
        e.set_param(params.faust_index("mix.osc_2").unwrap(), 1.0); // at 0 it isn't computed
        let live = Arc::new(LiveScope::default());
        e.set_live_scope(live.clone());
        assert!(live.snapshot_since(0).is_none(), "nothing before a note");
        e.note_on(0, 45, 1.0);
        run(&mut e, 200);
        let snapshot = live.snapshot_since(0).expect("live samples while playing");
        assert!((snapshot.note_hz - 110.0).abs() < 0.01);
        let trace = snapshot.trace(crate::scope::Tap::Osc2, 110.0, 1.0);
        assert!(trace.iter().fold(0.0f32, |m, v| m.max(v.abs())) > 0.05, "live trace is silent");
        e.note_off(0, 45);
        run(&mut e, 20_000);                     // release, then the voice is freed
        let seen = live.snapshot_since(0).unwrap().written;
        run(&mut e, 100);
        assert!(live.snapshot_since(seen).is_none(), "a finished voice still feeds the scope");
    }

    /// Portamento above 0: one voice, legato notes glide (no retrigger),
    /// releasing the top note glides back to the one still held, a detached
    /// note starts on pitch; at 0 the engine is polyphonic again.
    #[test]
    fn mono_mode_glides_on_one_voice() {
        let (mut e, params) = engine();
        let set = |e: &mut Engine, id: &str, v: f32| e.set_by_id(&params, id, v);
        for (id, v) in [("dual_1.a", 0.0), ("mix.noise", 0.0), ("filter.cutoff", 18000.0),
                        ("mod.filter_cutoff.env", 0.0), ("amp.sustain", 1.0), ("drive.amount", 0.0)] {
            set(&mut e, id, v);
        }
        let take = |e: &mut Engine, seconds: f32| {
            let mut out = Vec::new();
            let mut buf = [0.0f32; MAX_BLOCK];
            for _ in 0..(seconds * 48_000.0 / MAX_BLOCK as f32) as usize {
                e.render(&mut buf);
                out.extend_from_slice(&buf);
            }
            out
        };
        // Average pitch from the spacing of rising zero crossings (the mix
        // is a lone sine).
        let pitch = |x: &[f32]| {
            let at: Vec<usize> = x.windows(2).enumerate().filter(|(_, w)| w[0] <= 0.0 && w[1] > 0.0).map(|(i, _)| i).collect();
            (at.len() - 1) as f32 / ((at[at.len() - 1] - at[0]) as f32 / 48_000.0)
        };

        e.set_portamento(0.1);
        e.note_on(0, 45, 1.0);                                       // A2, 110 Hz
        let first = take(&mut e, 0.3);
        assert!((pitch(&first[4800..]) - 110.0).abs() < 4.0, "first note doesn't glide in from nowhere");
        e.note_on(0, 57, 1.0);                                       // A3, legato
        let gliding = take(&mut e, 0.05);
        let arrived = take(&mut e, 0.3);
        assert_eq!(e.active_voices(), 1, "mono uses one voice");
        let mid = pitch(&gliding);
        assert!(mid > 115.0 && mid < 210.0, "no glide: {mid:.0} Hz on average over the first 50 ms");
        assert!((pitch(&arrived[4800..]) - 220.0).abs() < 4.0, "didn't reach the new note");
        e.note_off(0, 57);                                           // back to the held A2
        let back = take(&mut e, 0.3);
        assert!((pitch(&back[4800..]) - 110.0).abs() < 4.0, "didn't return to the held note");
        e.note_off(0, 45);
        take(&mut e, 0.1);

        // A detached note (played after letting go) starts on pitch.
        e.note_on(0, 69, 1.0);                                       // A4, 440 Hz
        let detached = take(&mut e, 0.05);
        assert!((pitch(&detached) - 440.0).abs() < 8.0, "a detached note glided: {:.0} Hz", pitch(&detached));
        e.note_off(0, 69);
        take(&mut e, 2.0);

        e.set_portamento(0.0);
        for n in [48, 52, 55] {
            e.note_on(0, n, 1.0);
        }
        take(&mut e, 0.05);
        assert_eq!(e.active_voices(), 3, "Portamento off should be polyphonic");
    }

    /// Vintage 0: every voice exactly in tune. Vintage 1: voices spread
    /// (per-voice offsets and drift), but by cents, not semitones.
    #[test]
    fn vintage_spreads_the_voices() {
        let (mut e, params) = engine();
        let set = |e: &mut Engine, id: &str, v: f32| e.set_by_id(&params, id, v);
        for (id, v) in [("dual_1.a", 0.0), ("mix.noise", 0.0), ("filter.cutoff", 18000.0),
                        ("mod.filter_cutoff.env", 0.0), ("amp.sustain", 1.0), ("drive.amount", 0.0)] {
            set(&mut e, id, v);
        }
        let pitch = |x: &[f32]| {
            let at: Vec<usize> = x.windows(2).enumerate().filter(|(_, w)| w[0] <= 0.0 && w[1] > 0.0).map(|(i, _)| i).collect();
            (at.len() - 1) as f32 / ((at[at.len() - 1] - at[0]) as f32 / 48_000.0)
        };
        // Each voice in turn plays A2 alone; returns its pitch in cents off 110 Hz.
        let spread = |e: &mut Engine| -> Vec<f32> {
            (0..NUM_VOICES)
                .map(|_| {
                    e.note_on(0, 45, 1.0);
                    let mut out = Vec::new();
                    let mut buf = [0.0f32; MAX_BLOCK];
                    for _ in 0..375 {                                // 0.5 s
                        e.render(&mut buf);
                        out.extend_from_slice(&buf);
                    }
                    e.note_off(0, 45);
                    run(e, 3000);
                    1200.0 * (pitch(&out[4800..]) / 110.0).log2()
                })
                .collect()
        };
        let range = |c: &[f32]| c.iter().fold((f32::MAX, f32::MIN), |(lo, hi), &x| (lo.min(x), hi.max(x)));
        set(&mut e, "vintage.amount", 0.0);
        let clean = spread(&mut e);
        let (lo, hi) = range(&clean);
        assert!(hi - lo < 0.1 && lo.abs() < 1.0, "Vintage 0 isn't in tune: {clean:?}");
        set(&mut e, "vintage.amount", 1.0);
        let worn = spread(&mut e);
        let (lo, hi) = range(&worn);
        println!("cents per voice at Vintage 1: {worn:.1?}");
        assert!(hi - lo > 3.0, "Vintage 1 barely spreads the voices ({:.1} cents)", hi - lo);
        assert!(lo > -60.0 && hi < 60.0, "Vintage 1 goes too far: {lo:.1}..{hi:.1} cents");
    }

    /// Every OscMod renders finite audio; at depth 1 it changes the sound.
    /// Prints a table with `--nocapture`.
    #[test]
    fn osc_mods_behave() {
        let (mut e, params) = engine();
        let set = |e: &mut Engine, id: &str, v: f32| e.set_by_id(&params, id, v);
        for (id, v) in [("mix.osc_1", 0.0), ("mix.noise", 0.0), ("mix.osc_2", 1.0),
                        ("filter.cutoff", 18000.0), ("mod.filter_cutoff.env", 0.0), ("drive.amount", 0.0),
                        ("amp.sustain", 1.0), ("wt_2.wave_1", 1.0)] {
            set(&mut e, id, v);
        }
        // Reset before each render so every take starts from identical voice
        // state (the drive's noise would otherwise differ between voices).
        let render = |e: &mut Engine, kind: f32, depth: f32| {
            set(e, "wt_2.oscmod", kind);
            set(e, "wt_2.oscmod_depth", depth);
            e.reset();
            run(e, 300);
            e.note_on(0, 48, 1.0);
            let mut out = Vec::new();
            let mut buf = [0.0f32; MAX_BLOCK];
            for _ in 0..200 {
                e.render(&mut buf);
                out.extend_from_slice(&buf);
            }
            e.note_off(0, 48);
            run(e, 3000);
            out
        };
        let rms = |x: &[f32]| (x.iter().map(|v| v * v).sum::<f32>() / x.len() as f32).sqrt();
        let diff = |a: &[f32], b: &[f32]| {
            let d: Vec<f32> = a.iter().zip(b).map(|(x, y)| x - y).collect();
            rms(&d) / rms(b).max(1e-9)
        };
        let dry = render(&mut e, 0.0, 0.0);
        let names = ["Off", "Bits 4", "SR 11k", "SR 8k", "SR 4k", "Simplify 6k",
                     "Simplify 1.5k", "Simplify 800", "Rectify", "Tri Fold", "Sine Fold", "Squish",
                     "Ring", "Mirror", "Sign Swap"];
        for (k, name) in names.iter().enumerate().skip(1) {
            let zero = render(&mut e, k as f32, 0.0);
            let full = render(&mut e, k as f32, 1.0);
            assert!(full.iter().all(|v| v.is_finite()), "{name} produced non-finite samples");
            let (d0, d1) = (diff(&zero, &dry), diff(&full, &dry));
            println!("{name:14} depth 0 differs from dry by {:5.1}%, depth 1 by {:6.1}%, level {:.3}",
                     100.0 * d0, 100.0 * d1, rms(&full));
            assert!(d1 > 0.002, "{name} at depth 1 barely changes the sound ({d1})");
            // Simplify's low-pass starts near Nyquist: a little phase shift remains.
            let allowed = if name.starts_with("Simplify") { 0.05 } else { 0.02 };
            assert!(d0 < allowed, "{name} at depth 0 should be (nearly) dry, differs by {d0}");
        }
    }

    /// Synthesized and sampled transients sound, stay finite, carry no DC,
    /// and Pitch changes every type.
    #[test]
    fn transients_play() {
        let (mut e, params) = engine();
        let set = |e: &mut Engine, id: &str, v: f32| e.set_by_id(&params, id, v);
        set(&mut e, "transient.post_filter", 1.0);
        set(&mut e, "transient.decay", 200.0);
        let take = |e: &mut Engine, kind: f32, level: f32, pitch: f32| {
            set(e, "transient.type", kind);
            set(e, "transient.level", level);
            set(e, "transient.pitch", pitch);
            e.reset();
            run(e, 2000);                       // let the level smoother settle
            e.note_on(0, 48, 1.0);
            let mut out = Vec::new();
            let mut buf = [0.0f32; MAX_BLOCK];
            for _ in 0..120 {
                e.render(&mut buf);
                out.extend_from_slice(&buf);
            }
            e.note_off(0, 48);
            run(e, 3000);
            out
        };
        let last = (crate::translib::LENS.len() + 4) as f32;
        // Click (0) is 1.5 ms long: it only survives if smoothers snap on note-on.
        for kind in [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, last] {
            let without = take(&mut e, kind, 0.0, 0.0);
            let with = take(&mut e, kind, 1.0, 0.0);
            let layer: Vec<f32> = with.iter().zip(&without).map(|(a, b)| a - b).collect();
            assert!(with.iter().all(|v| v.is_finite()), "type {kind}: non-finite output");
            let peak = layer.iter().fold(0.0f32, |m, v| m.max(v.abs()));
            let mean = layer.iter().sum::<f32>() / layer.len() as f32;
            println!("type {kind:3}: peak {peak:.3}, mean {mean:+.5}");
            assert!(peak > 0.05, "type {kind}: transient inaudible (peak {peak})");
            assert!(mean.abs() < 0.01, "type {kind}: DC offset {mean}");
            if kind != 1.0 {
                let up = take(&mut e, kind, 1.0, 12.0);
                assert_ne!(with, up, "type {kind}: Pitch had no effect");
            }
        }
    }

    #[test]
    fn voice_stealing_keeps_working() {
        let (mut e, _) = engine();
        for n in 40..60 {
            e.note_on(0, n, 1.0);
            let (_, _, bad) = run(&mut e, 10);
            assert!(!bad);
        }
        assert_eq!(e.active_voices(), NUM_VOICES);
    }

    /// Play a note and collect one moving value at the end of every block.
    fn moving_over(e: &mut Engine, value: crate::scope::Moving, blocks: usize) -> Vec<f32> {
        let live = Arc::new(LiveScope::default());
        e.set_live_scope(live.clone());
        e.note_on(0, 57, 1.0);
        let mut buf = [0.0; MAX_BLOCK];
        (0..blocks)
            .map(|_| {
                e.render(&mut buf);
                live.snapshot_since(0).unwrap().get(value)
            })
            .collect()
    }

    /// A slot on Ray sounds with every element (Chaos too), stays finite and
    /// centred, and a later note on the same voice starts clean.
    #[test]
    fn ray_slots_sound() {
        for element in 0..7 {
            let (mut e, params) = engine();
            let set = |e: &mut Engine, id: &str, v: f32| e.set_by_id(&params, id, v);
            set(&mut e, "osc_1.mode", 2.0);
            set(&mut e, "ray_1.element", element as f32);
            set(&mut e, "ray_1.center", 0.4);
            set(&mut e, "ray_1.orbit", 0.2);
            for note in [57, 64] {
                e.note_on(0, note, 1.0);
                let (peak, rms, bad) = run(&mut e, 300);
                assert!(!bad, "element {element}: not finite");
                assert!(peak > 0.01 && peak < 1.5, "element {element}: peak {peak}");
                assert!(rms > 0.003, "element {element}: rms {rms}");
                e.note_off(0, note);
                run(&mut e, 600);
            }
        }
    }

    /// A fader at exactly 0 silences its source even with its modulators
    /// pushing it up; just above 0, they open it.
    #[test]
    fn mix_fader_at_zero_mutes() {
        let (mut e, params) = engine();
        let set = |e: &mut Engine, id: &str, v: f32| e.set_by_id(&params, id, v);
        for id in ["mix.osc_1", "mix.osc_2", "mix.noise", "transient.level"] {
            set(&mut e, id, 0.0);
        }
        set(&mut e, "mod.mix_osc_1.lfo", 1.0);
        set(&mut e, "mod.mix_osc_1.env", 1.0);
        set(&mut e, "mod.mix_osc_1.sustain", 1.0);
        e.note_on(0, 57, 1.0);
        let (peak, _, bad) = run(&mut e, 400);
        assert!(!bad);
        assert!(peak < 1e-6, "muted mix still sounds: {peak}");
        set(&mut e, "mix.osc_1", 0.01);
        let (peak, _, _) = run(&mut e, 400);
        assert!(peak > 0.01, "the modulators didn't open the fader: {peak}");
    }

    /// A knob's modulators move it: the LFO swings the cutoff both ways, the
    /// envelope lifts it at the start of the note and lets it settle.
    #[test]
    fn modulators_move_their_knobs() {
        use crate::scope::Moving;
        let (mut e, params) = engine();
        let set = |e: &mut Engine, id: &str, v: f32| e.set_by_id(&params, id, v);
        set(&mut e, "mod.filter_cutoff.env", 0.0);
        set(&mut e, "mod.filter_cutoff.rate", 5.0);
        set(&mut e, "mod.filter_cutoff.lfo", 0.5);
        let cutoff = moving_over(&mut e, Moving::Cutoff, 1500); // 2 s
        let (lo, hi) = cutoff.iter().fold((f32::MAX, 0.0f32), |(lo, hi), v| (lo.min(*v), hi.max(*v)));
        assert!(hi / lo > 30.0, "the LFO barely moves the cutoff: {lo}..{hi}");
        // The panel's mark goes where the voice plays it.
        let live = Arc::new(LiveScope::default());
        e.set_live_scope(live.clone());
        run(&mut e, 3);
        let snapshot = live.snapshot_since(0).unwrap();
        let travel = params.slots().mod_travel[modulation::CUTOFF];
        let drawn = travel.plain(snapshot.moved[modulation::CUTOFF]);
        let played = snapshot.get(Moving::Cutoff);
        assert!((drawn / played - 1.0).abs() < 0.01, "drawn at {drawn} Hz, played at {played} Hz");

        let (mut e, _) = engine();
        set(&mut e, "filter.cutoff", 200.0);
        set(&mut e, "mod.filter_cutoff.sustain", 0.0);
        set(&mut e, "mod.filter_cutoff.decay", 0.2);
        set(&mut e, "mod.filter_cutoff.env", 1.0);
        let cutoff = moving_over(&mut e, Moving::Cutoff, 1500);
        let peak = cutoff[..20].iter().fold(0.0f32, |m, v| m.max(*v));
        assert!(peak > 12_000.0, "the envelope didn't open the cutoff: {peak}");
        let resting = 200.0 * (note_freq(57) / 261.63f32).sqrt(); // key tracking 0.5
        assert!((cutoff[1499] / resting - 1.0).abs() < 0.02, "cutoff didn't settle: {}", cutoff[1499]);
    }

    /// Synced, the LFO follows the song position: a sine at 1/4 notes
    /// peaks a sixteenth into every beat, whenever the note started.
    #[test]
    fn synced_lfo_follows_the_song_position() {
        use crate::scope::Moving;
        let (mut e, params) = engine();
        let set = |e: &mut Engine, id: &str, v: f32| e.set_by_id(&params, id, v);
        // The Bandpass knob (linear, 0..1) at 0.5, swung by +-0.4: a sine
        // at 1/4 notes, free of the note.
        set(&mut e, "filter.bandpass", 0.5);
        set(&mut e, "mod.filter_bandpass.sync", 1.0); // 1/4
        set(&mut e, "mod.filter_bandpass.retrig", 0.0);
        set(&mut e, "mod.filter_bandpass.lfo", 0.4);
        let live = Arc::new(LiveScope::default());
        e.set_live_scope(live.clone());
        let mut buf = [0.0; MAX_BLOCK];
        let beats_per_block = 120.0 / 60.0 * MAX_BLOCK as f64 / 48_000.0;
        let mut beats = 3.3; // the song is already playing
        for block in 0..2000 {
            if block == 100 {
                e.note_on(0, 57, 1.0);
            }
            e.set_transport(120.0, Some(beats), MAX_BLOCK);
            e.render(&mut buf);
            beats += beats_per_block;
            if block > 100 {
                // The modulators run at the block's start, at the song position then.
                let at = beats - beats_per_block;
                let expected = 0.5 + 0.4 * (2.0 * std::f64::consts::PI * at).sin() as f32;
                let got = live.snapshot_since(0).unwrap().get(Moving::Bandpass);
                assert!((got - expected).abs() < 0.03, "beat {at:.3}: bandpass {got}, expected {expected}");
            }
        }
    }
}

#[cfg(test)]
mod ab_render {
    use super::*;
    use crate::params::SynthParams;

    /// Renders a few patches to ABD_DUMP (raw f32), to compare builds.
    #[test]
    #[ignore]
    fn dump_patches() {
        let Ok(path) = std::env::var("ABD_DUMP") else { return };
        let params = SynthParams::from_faust();
        let mut all = Vec::new();
        let patches: &[&[(&str, f32)]] = &[
            &[],
            &[("dual_1.algorithm", 1.0), ("unison.mode", 2.0)],
            &[("mix.osc_2", 1.0), ("wt_2.oscmod", 6.0), ("noise.type", 4.0), ("mix.noise", 0.5)],
            &[("dual_1.a", 0.75), ("filter.4_pole", 1.0), ("drive.amount", 0.8), ("transient.level", 1.0), ("transient.type", 2.0)],
            &[("dual_1.algorithm", 8.0), ("mod.filter_cutoff.lfo", 0.5), ("mod.dual_1_a.env", 0.7), ("transient.level", 1.0), ("transient.type", 20.0)],
            &[("events.depth", 8.0), ("events.density", 50.0), ("calls.level", -40.0)],
            &[("events.depth", 8.0), ("events.density", 50.0)],
            &[("calls.level", -40.0)],
            &[("mix.noise", 0.0), ("dual_1.a", 0.0)],
            &[("mix.noise", 0.0), ("dual_1.a", 0.0), ("drive.amount", 0.0)],
            &[("mix.noise", 0.0), ("dual_1.a", 0.0), ("vintage.amount", 0.5)],
        ];
        for patch in patches {
            let mut e = Engine::new(params.slots());
            e.init(48_000.0);
            e.load(&params);
            for (id, v) in *patch { e.set_by_id(&params, id, *v); }
            // ABD_SET="id=value,...": on top of every patch (e.g. settings.heat=0).
            for pair in std::env::var("ABD_SET").unwrap_or_default().split(',').filter(|p| !p.is_empty()) {
                let (id, x) = pair.split_once('=').unwrap();
                e.set_by_id(&params, id, x.parse().unwrap());
            }
            for n in [48, 55, 60] { e.note_on(0, n, 0.8); }
            let mut buf = [[0.0f32; MAX_BLOCK]; 2];
            for b in 0..1500 {
                if b == 1000 { for n in [48, 55, 60] { e.note_off(0, n); } }
                e.render(&mut buf[0]);
                all.extend_from_slice(&buf[0]);
            }
        }
        let bytes: Vec<u8> = all.iter().flat_map(|v| v.to_le_bytes()).collect();
        std::fs::write(path, bytes).unwrap();
    }
}

#[cfg(test)]
mod note_series {
    use super::*;
    use crate::params::SynthParams;

    /// Plays 16 notes one after another and prints each note's RMS and DC:
    /// `ABD_SET=... cargo test --release note_series -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn note_series() {
        let params = SynthParams::from_faust();
        let mut e = Engine::new(params.slots());
        e.init(48_000.0);
        e.load(&params);
        for pair in std::env::var("ABD_SET").unwrap_or_default().split(',').filter(|p| !p.is_empty()) {
            let (id, x) = pair.split_once('=').unwrap();
            e.set_by_id(&params, id, x.parse().unwrap());
        }
        let live = Arc::new(LiveScope::default());
        e.set_live_scope(live.clone());
        let mut buf = [0.0f32; MAX_BLOCK];
        let mut line = String::new();
        for k in 0..24u8 {
            let note = 48 + (k % 3) * 4;
            e.note_on(0, note, 0.8);
            let (mut sum, mut sq, mut n) = (0.0f64, 0.0f64, 0usize);
            let hold: usize = std::env::var("ABD_HOLD").ok().and_then(|g| g.parse().ok()).unwrap_or(300);
            for b in 0..hold {
                e.render(&mut buf);
                if b >= hold / 6 { for s in buf { sum += s as f64; sq += (s * s) as f64; n += 1; } }
            }
            e.note_off(0, note);
            let gap: usize = std::env::var("ABD_GAP").ok().and_then(|g| g.parse().ok()).unwrap_or(200);
            for _ in 0..gap { e.render(&mut buf); }
            let snap = live.snapshot_since(0).unwrap();
            let dc = |v: &[f32]| v.iter().map(|x| *x as f64).sum::<f64>() / v.len() as f64;
            let pk = |v: &[f32]| v.iter().fold(0.0f32, |m, x| m.max(x.abs()));
            line += &format!("{k}: rms {:.3} dc {:+.3} dual {:+.2}/{:.2} wt {:+.2}/{:.2} | ", (sq / n as f64).sqrt(), sum / n as f64,
                             dc(&snap.osc1), pk(&snap.osc1), dc(&snap.osc2), pk(&snap.osc2));
        }
        println!("{line}");
    }
}

#[cfg(test)]
mod block_rate {
    use super::*;
    use crate::params::SynthParams;

    /// A pluck (and other fast modulation) rendered in blocks of ABD_BLOCK
    /// samples (default 64): with 1, the modulators run every sample, so
    /// comparing the two measures what running them once a block costs.
    /// ABD_CASE picks the patch: pluck, lfo, vel. Writes 1.5 s to ABD_DUMP.
    #[test]
    #[ignore]
    fn render_in_blocks() {
        let Ok(path) = std::env::var("ABD_DUMP") else { return };
        let block: usize = std::env::var("ABD_BLOCK").ok().and_then(|b| b.parse().ok()).unwrap_or(MAX_BLOCK);
        let params = SynthParams::from_faust();
        let mut e = Engine::new(params.slots());
        e.init(48_000.0);
        e.load(&params);
        let mut set = |id: &str, v: f32| e.set_by_id(&params, id, v);
        for (id, v) in [("mix.noise", 0.0), ("dual_1.a", 0.5), ("drive.amount", 0.0), ("amp.sustain", 1.0),
                        ("amp.attack", 0.005), ("settings.drive_grit", 0.0), ("filter.resonance", 2.5),
                        ("mod.filter_cutoff.key", 0.0)] {
            set(id, v);
        }
        match std::env::var("ABD_CASE").as_deref().unwrap_or("pluck") {
            // A snappy filter pluck: 1 ms attack, 120 ms decay to nothing.
            "pluck" => for (id, v) in [("filter.cutoff", 120.0), ("mod.filter_cutoff.env", 0.6),
                                        ("mod.filter_cutoff.attack", 0.001), ("mod.filter_cutoff.decay", 0.12),
                                        ("mod.filter_cutoff.sustain", 0.0)] { set(id, v) },
            // The fastest LFO, deep, on the cutoff.
            "lfo" => for (id, v) in [("filter.cutoff", 1500.0), ("mod.filter_cutoff.env", 0.0),
                                      ("mod.filter_cutoff.rate", 50.0), ("mod.filter_cutoff.lfo", 0.25)] { set(id, v) },
            // An amp pluck on Osc 1's fader (a gated knob).
            _ => for (id, v) in [("filter.cutoff", 18000.0), ("mod.filter_cutoff.env", 0.0),
                                 ("mod.mix_osc_1.env", 1.0), ("mod.mix_osc_1.attack", 0.001),
                                 ("mod.mix_osc_1.decay", 0.05), ("mix.osc_1", 0.05)] { set(id, v) },
        }
        e.note_on(0, 45, 1.0);
        let mut buf = vec![0.0f32; block];
        let mut all = Vec::new();
        while all.len() < 72_000 {
            e.render(&mut buf);
            all.extend_from_slice(&buf);
        }
        std::fs::write(path, all.iter().flat_map(|v| v.to_le_bytes()).collect::<Vec<u8>>()).unwrap();
    }
}

#[cfg(test)]
mod noise_floor {
    use super::*;
    use crate::params::SynthParams;

    /// One held note (MIDI ABD_NOTE, default 57); writes 2 s of output to
    /// ABD_DUMP (raw f32).
    #[test]
    #[ignore]
    fn one_note() {
        let Ok(path) = std::env::var("ABD_DUMP") else { return };
        let params = SynthParams::from_faust();
        let mut e = Engine::new(params.slots());
        e.init(48_000.0);
        e.load(&params);
        for pair in std::env::var("ABD_SET").unwrap_or_default().split(',').filter(|p| !p.is_empty()) {
            let (id, x) = pair.split_once('=').unwrap();
            e.set_by_id(&params, id, x.parse().unwrap());
        }
        let note: u8 = std::env::var("ABD_NOTE").ok().and_then(|n| n.parse().ok()).unwrap_or(57);
        e.note_on(0, note, 0.8);
        let mut buf = [0.0f32; MAX_BLOCK];
        let mut all = Vec::new();
        for _ in 0..1500 { e.render(&mut buf); all.extend_from_slice(&buf); }
        std::fs::write(path, all.iter().flat_map(|v| v.to_le_bytes()).collect::<Vec<u8>>()).unwrap();
    }
}

#[cfg(test)]
mod switches {
    use super::*;
    use crate::params::SynthParams;

    /// Switching what a voice computes while it plays (Dual algorithm,
    /// OscMod, noise type) leaves nothing behind: an unselected branch
    /// outputs 0, not its last value (which was a DC offset).
    #[test]
    fn switching_leaves_no_offset() {
        let params = SynthParams::from_faust();
        let mut e = Engine::new(params.slots());
        e.init(48_000.0);
        e.load(&params);
        let set = |e: &mut Engine, id: &str, v: f32| e.set_by_id(&params, id, v);
        set(&mut e, "mix.osc_2", 1.0);
        let live = Arc::new(LiveScope::default());
        e.set_live_scope(live.clone());
        let mut buf = [0.0f32; MAX_BLOCK];
        e.note_on(0, 57, 0.8);
        let mean = |v: &[f32]| v.iter().map(|x| *x as f64).sum::<f64>() / v.len() as f64;
        for (id, values) in [("dual_1.algorithm", [10.0, 4.0, 11.0, 9.0]), ("wt_2.oscmod", [0.0, 9.0, 12.0, 0.0])] {
            for v in values {
                set(&mut e, id, v);
                for _ in 0..200 { e.render(&mut buf); }
                let snap = live.snapshot_since(0).unwrap();
                let (dual, wt) = (mean(&snap.osc1), mean(&snap.osc2));
                assert!(dual.abs() < 0.05 && wt.abs() < 0.15, "{id} = {v}: offset dual {dual:+.3}, wavetable {wt:+.3}");
            }
        }
    }
}
