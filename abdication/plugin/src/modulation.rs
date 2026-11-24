//! The modulators (Nov 2026, after Elektron-style assignment): ten sources
//! shared by every knob, each with a depth per knob.
//!
//! - Envelopes A, B and C: ADSRs with an attack curve and a decay/release
//!   curve. A is also the amp (at full depth, in the Faust voice), B is
//!   wired to the cutoff by the Filter plate's Env B fader, C is free.
//! - LFOs X, Y and Z: a shape (the basic waves, Noise, Brown, S&H, or any
//!   library wave), a rate (Hz or synced), Free (runs on through notes)
//!   or restarted with each. Z is also wired to both oscillators' pitch.
//! - The mod wheel, aftertouch, the key (bipolar about C4) and velocity.
//!
//! The engine runs them once a block per voice and moves each knob within
//! its own travel (the host's normalized value, so a log knob moves evenly
//! in octaves): a depth of 1 sweeps it across its whole range. The voice
//! then plays the knob where they put it, drawing a straight line to each
//! new value over the block (voice.lib's `ramped`).
//!
//! No nih-plug types here: the engine and its tests use this on its own.

/// A modulation source.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Source {
    /// Its part of a depth's parameter id ("mod.filter_cutoff.x").
    pub id: &'static str,
    /// On its button and in the readout.
    pub label: &'static str,
    pub name: &'static str,
    /// Swings both ways (the LFOs, the key) rather than 0..1.
    pub bipolar: bool,
}

const fn source(id: &'static str, label: &'static str, name: &'static str, bipolar: bool) -> Source {
    Source { id, label, name, bipolar }
}

pub const SOURCES: [Source; 10] = [
    source("a", "A", "Env A", false),
    source("b", "B", "Env B", false),
    source("c", "C", "Env C", false),
    source("x", "X", "LFO X", true),
    source("y", "Y", "LFO Y", true),
    source("z", "Z", "LFO Z", true),
    source("mw", "MW", "Mod Wheel", false),
    source("at", "AT", "Aftertouch", false),
    source("key", "Key", "Key", true),
    source("vel", "Vel", "Velocity", false),
];
pub const SOURCE_COUNT: usize = SOURCES.len();
pub const ENV_A: usize = 0;
pub const ENV_B: usize = 1;
pub const ENV_C: usize = 2;
pub const LFO_X: usize = 3;
pub const LFO_Z: usize = 5;
pub const WHEEL: usize = 6;
pub const TOUCH: usize = 7;
pub const KEY: usize = 8;
pub const VELOCITY: usize = 9;

pub fn source_index(id: &str) -> Option<usize> {
    SOURCES.iter().position(|s| s.id == id)
}

/// A knob the sources can move.
#[derive(Clone, Copy, Debug)]
pub struct Target {
    /// The knob's parameter id.
    pub id: &'static str,
    /// Its name in the readout.
    pub short: &'static str,
    /// Off at the bottom of its range (the Noise and Trans faders): it stays
    /// off while the knob is there, whatever moves it, and modulation never
    /// quite reaches the bottom from above.
    pub off_at_bottom: bool,
    /// The key moves it in octaves of the knob per octave of the note (the
    /// cutoff), rather than in its travel.
    pub key_in_octaves: bool,
}

const fn target(id: &'static str, short: &'static str) -> Target {
    Target { id, short, off_at_bottom: false, key_in_octaves: false }
}
const fn gated(id: &'static str, short: &'static str) -> Target {
    Target { id, short, off_at_bottom: true, key_in_octaves: false }
}

/// Every modulatable knob, in panel order (top bar, then plate by plate).
/// Its position here is its depths' place in the engine (their parameter
/// ids carry the knob's id, so saved patches don't depend on the order).
pub const TARGETS: [Target; 81] = [
    target("vintage.amount", "vintage"),
    target("unison.detune", "detune"),
    target("unison.spread", "spread"),
    target("dual_1.a", "I dual a"),
    target("dual_1.b", "I dual b"),
    target("wt_1.oscmod_depth", "I oscmod"),
    target("wt_1.speed", "I speed"),
    target("ray_1.squish", "I squish"),
    target("ray_1.skew", "I skew"),
    target("ray_1.tilt", "I tilt"),
    target("ray_1.center", "I center"),
    target("ray_1.ray", "I ray"),
    target("ray_1.orbit", "I orbit"),
    target("ray_1.orbit_rate", "I orbit rate"),
    target("ray_1.fine", "I fine"),
    target("ortho_1.top_position", "I top"),
    target("ortho_1.bottom_position", "I bottom"),
    target("ortho_1.mix", "I ortho mix"),
    target("windows_1.int_1_x", "I 1 x"),
    target("windows_1.int_1_phase", "I 1 phase"),
    target("windows_1.int_1_len", "I 1 len"),
    target("windows_1.int_1_pitch", "I 1 pitch"),
    target("windows_1.int_2_x", "I 2 x"),
    target("windows_1.int_2_phase", "I 2 phase"),
    target("windows_1.int_2_len", "I 2 len"),
    target("windows_1.int_2_pitch", "I 2 pitch"),
    target("windows_1.int_3_x", "I 3 x"),
    target("windows_1.int_3_phase", "I 3 phase"),
    target("windows_1.int_3_len", "I 3 len"),
    target("windows_1.int_3_pitch", "I 3 pitch"),
    target("dual_2.a", "II dual a"),
    target("dual_2.b", "II dual b"),
    target("wt_2.oscmod_depth", "II oscmod"),
    target("wt_2.speed", "II speed"),
    target("ray_2.squish", "II squish"),
    target("ray_2.skew", "II skew"),
    target("ray_2.tilt", "II tilt"),
    target("ray_2.center", "II center"),
    target("ray_2.ray", "II ray"),
    target("ray_2.orbit", "II orbit"),
    target("ray_2.orbit_rate", "II orbit rate"),
    target("ray_2.fine", "II fine"),
    target("ortho_2.top_position", "II top"),
    target("ortho_2.bottom_position", "II bottom"),
    target("ortho_2.mix", "II ortho mix"),
    target("windows_2.int_1_x", "II 1 x"),
    target("windows_2.int_1_phase", "II 1 phase"),
    target("windows_2.int_1_len", "II 1 len"),
    target("windows_2.int_1_pitch", "II 1 pitch"),
    target("windows_2.int_2_x", "II 2 x"),
    target("windows_2.int_2_phase", "II 2 phase"),
    target("windows_2.int_2_len", "II 2 len"),
    target("windows_2.int_2_pitch", "II 2 pitch"),
    target("windows_2.int_3_x", "II 3 x"),
    target("windows_2.int_3_phase", "II 3 phase"),
    target("windows_2.int_3_len", "II 3 len"),
    target("windows_2.int_3_pitch", "II 3 pitch"),
    target("osc_1.pitch", "I pitch"),
    target("osc_2.pitch", "II pitch"),
    target("mix.balance", "balance"),
    gated("mix.noise", "noise level"),
    gated("transient.level", "trans level"),
    target("noise.crackle_density", "crackle"),
    target("noise.tilt", "noise tilt"),
    target("transient.decay", "trans decay"),
    target("transient.pitch", "trans pitch"),
    target("transient.depth", "trans depth"),
    target("drive.amount", "drive"),
    Target { id: "filter.cutoff", short: "cutoff", off_at_bottom: false, key_in_octaves: true },
    target("filter.resonance", "reso"),
    target("filter.a", "filter a"),
    target("filter.b", "filter b"),
    target("filter.hpf", "hpf"),
    target("filter.hpf_peak", "hpf peak"),
    target("harmonic.shift", "harm shift"),
    target("harmonic.tilt", "harm tilt"),
    target("harmonic.depth", "harm depth"),
    target("harmonic.odd_even", "odd/even"),
    target("lfo_x.rate", "X rate"),
    target("lfo_y.rate", "Y rate"),
    target("lfo_z.rate", "Z rate"),
];
pub const COUNT: usize = TARGETS.len();

pub fn target_index(id: &str) -> Option<usize> {
    TARGETS.iter().position(|t| t.id == id)
}

const fn index_of(id: &str) -> usize {
    let mut i = 0;
    while i < COUNT {
        if const_eq(TARGETS[i].id, id) {
            return i;
        }
        i += 1;
    }
    panic!("not a target")
}

const fn const_eq(a: &str, b: &str) -> bool {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}

/// The cutoff: envelope B's depth on it is the Filter plate's Env B fader.
pub const CUTOFF: usize = index_of("filter.cutoff");
/// The LFOs' rates (X, Y, Z) and the wavetables' Speeds (I, II).
pub const LFO_RATES: [usize; 3] = [index_of("lfo_x.rate"), index_of("lfo_y.rate"), index_of("lfo_z.rate")];
pub const WT_SPEEDS: [usize; 2] = [index_of("wt_1.speed"), index_of("wt_2.speed")];
/// The parameter that is envelope B's depth on the cutoff.
pub const ENV_B_DEPTH: &str = "filter.env_b";

/// A depth's parameter id: "mod.filter_cutoff.x". The one wired on the
/// panel (envelope B on the cutoff) is its fader's parameter.
pub fn depth_id(t: usize, s: usize) -> String {
    if t == CUTOFF && s == ENV_B {
        return ENV_B_DEPTH.to_string();
    }
    format!("mod.{}.{}", TARGETS[t].id.replace('.', "_"), SOURCES[s].id)
}

/// The (target, source) a "mod.<knob>.<source>" id names.
pub fn parse_depth_id(id: &str) -> Option<(usize, usize)> {
    let (knob, src) = id.strip_prefix("mod.")?.rsplit_once('.')?;
    let t = TARGETS.iter().position(|t| t.id.replace('.', "_") == knob)?;
    Some((t, source_index(src)?))
}

/// The depths a fresh patch has: the key moves the cutoff half an octave
/// per octave (B's own, 21 %, is its fader's default).
pub fn default_depth(t: usize, s: usize) -> f32 {
    if t == CUTOFF && s == KEY { 0.5 } else { 0.0 }
}

// ---- The settings the engine reads ----------------------------------------------

/// The modulators' own controls the engine reads (Faust controls, declared
/// in voice.lib's `modControls`), in this order. params.rs marks each so
/// the engine catches its value as it's loaded.
pub const FIELDS: [&str; 44] = [
    "env_a.attack", "env_a.decay", "env_a.sustain", "env_a.release", "env_a.attack_curve", "env_a.dr_curve",
    "env_b.attack", "env_b.decay", "env_b.sustain", "env_b.release", "env_b.attack_curve", "env_b.dr_curve",
    "env_c.attack", "env_c.decay", "env_c.sustain", "env_c.release", "env_c.attack_curve", "env_c.dr_curve",
    "lfo_x.shape", "lfo_x.rate", "lfo_x.sync", "lfo_x.division", "lfo_x.free",
    "lfo_y.shape", "lfo_y.rate", "lfo_y.sync", "lfo_y.division", "lfo_y.free",
    "lfo_z.shape", "lfo_z.rate", "lfo_z.sync", "lfo_z.division", "lfo_z.free",
    "lfo_z.depth", "lfo_z.mode",
    "wt_1.speed", "wt_1.speed_sync", "wt_1.speed_division",
    "wt_2.speed", "wt_2.speed_sync", "wt_2.speed_division",
    ENV_B_DEPTH, "osc_1.mode", "osc_2.mode",
];
pub const FIELD_COUNT: usize = FIELDS.len();
const ENV_FIELDS: usize = 0;
const LFO_FIELDS: usize = 18;
const VIBRATO_FIELDS: usize = 33;
const SPEED_FIELDS: usize = 35;
const ENV_B_FIELD: usize = 41;
const MODE_FIELDS: usize = 42;

pub fn field_index(id: &str) -> Option<usize> {
    FIELDS.iter().position(|f| *f == id)
}

/// The values of FIELDS, as loaded.
#[derive(Clone, Copy, Debug)]
pub struct Fields(pub [f32; FIELD_COUNT]);

impl Default for Fields {
    fn default() -> Self {
        let mut f = [0.0; FIELD_COUNT];
        for e in 0..3 {
            f[ENV_FIELDS + 6 * e..ENV_FIELDS + 6 * e + 6].copy_from_slice(&[0.005, 0.4, [0.6, 0.3, 0.0][e], 0.5, 0.6, -0.6]);
        }
        for (l, hz) in [0.5, 2.0, 5.0].into_iter().enumerate() {
            f[LFO_FIELDS + 5 * l..LFO_FIELDS + 5 * l + 5].copy_from_slice(&[0.0, hz, 0.0, DEFAULT_DIVISION as f32, 0.0]);
        }
        f[VIBRATO_FIELDS] = 50.0;
        f[VIBRATO_FIELDS + 1] = 1.0;
        for s in 0..2 {
            f[SPEED_FIELDS + 3 * s..SPEED_FIELDS + 3 * s + 3].copy_from_slice(&[0.5, 0.0, 8.0]);
        }
        f[ENV_B_FIELD] = 0.21;
        Self(f)
    }
}

impl Fields {
    pub fn env(&self, e: usize) -> EnvSettings {
        let f = &self.0[ENV_FIELDS + 6 * e..];
        EnvSettings { attack: f[0], decay: f[1], sustain: f[2], release: f[3], attack_curve: f[4], dr_curve: f[5] }
    }

    pub fn lfo(&self, l: usize) -> LfoSettings {
        let f = &self.0[LFO_FIELDS + 5 * l..];
        LfoSettings { shape: LfoShape::from_menu(f[0] as usize), hz: f[1], sync: f[2] > 0.5,
                      division: division_index(f[3]), free: f[4] > 0.5 }
    }

    /// LFO Z's pitch depth in cents, and what scales it (0 on, 1 the mod
    /// wheel, 2 aftertouch).
    pub fn vibrato(&self) -> (f32, usize) {
        (self.0[VIBRATO_FIELDS], self.0[VIBRATO_FIELDS + 1].round() as usize)
    }

    /// Wavetable `slot`'s Speed setting: synced, and its division.
    pub fn speed_sync(&self, slot: usize) -> Option<usize> {
        let f = &self.0[SPEED_FIELDS + 3 * slot..];
        (f[1] > 0.5).then(|| division_index(f[2]))
    }

    pub fn env_b_depth(&self) -> f32 {
        self.0[ENV_B_FIELD]
    }

    /// Oscillator slot `slot`'s engine (layout's SLOT_* numbers).
    pub fn slot_mode(&self, slot: usize) -> usize {
        self.0[MODE_FIELDS + slot].round() as usize
    }
}

// ---- Envelopes ---------------------------------------------------------------------

/// An envelope's settings: seconds, sustain 0..1, and the bows (`curve`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EnvSettings {
    pub attack: f32,
    pub decay: f32,
    pub sustain: f32,
    pub release: f32,
    pub attack_curve: f32,
    /// The decay's and release's.
    pub dr_curve: f32,
}

/// How far a curve of ±1 bends: the exponent at full bow (Raycaster's).
pub const CURVE_DEPTH: f32 = 5.0;

/// The fraction of the way along a segment at `progress` (0..1), for a
/// segment that rises: `bow` 0 is straight, positive bows the line up (fast
/// first), negative down.
pub fn curve(progress: f32, bow: f32) -> f32 {
    let k = bow * CURVE_DEPTH;
    if k.abs() < 1e-3 { progress } else { (1.0 - (-k * progress).exp()) / (1.0 - (-k).exp()) }
}

/// A segment's value at `progress`, going from `from` to `to`, bowed up
/// (positive) or down as it looks on a graph, whichever way it runs.
pub fn along(from: f32, to: f32, progress: f32, bow: f32) -> f32 {
    let bow = if to >= from { bow } else { -bow };
    from + (to - from) * curve(progress.clamp(0.0, 1.0), bow)
}

impl EnvSettings {
    /// The level `t_on` seconds into a note that started from `start`.
    fn held(&self, t_on: f32, start: f32) -> f32 {
        let a = self.attack.max(1e-4);
        if t_on < a {
            along(start, 1.0, t_on / a, self.attack_curve)
        } else {
            along(1.0, self.sustain, (t_on - a) / self.decay.max(1e-4), self.dr_curve)
        }
    }

    /// Where a note is in it: the stage and how far through it (0..1; the
    /// sustain's seconds so far), as the wavetables' paths follow it.
    pub fn stage(&self, clock: &NoteClock) -> Stage {
        let (a, d) = (self.attack.max(1e-4), self.decay.max(1e-4));
        match clock.gated {
            false => Stage::Release((clock.t_off / self.release.max(1e-4)).min(1.0)),
            true if clock.t_on < a => Stage::Attack(clock.t_on / a),
            true if clock.t_on < a + d => Stage::Decay((clock.t_on - a) / d),
            true => Stage::Sustain(clock.t_on - a - d),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Stage {
    Attack(f32),
    Decay(f32),
    Sustain(f32),
    Release(f32),
}

/// When a voice's note started and ended, shared by its envelopes.
#[derive(Clone, Copy, Debug, Default)]
pub struct NoteClock {
    pub gated: bool,
    /// Seconds since the note began (stops counting at release).
    pub t_on: f32,
    /// Seconds since release.
    pub t_off: f32,
}

/// One envelope in one voice: where it started and was released from.
#[derive(Clone, Copy, Debug, Default)]
pub struct EnvState {
    start: f32,
    released_at: f32,
}

impl EnvState {
    pub fn level(&self, s: &EnvSettings, clock: &NoteClock) -> f32 {
        if clock.gated {
            s.held(clock.t_on, self.start)
        } else {
            along(self.released_at, 0.0, clock.t_off / s.release.max(1e-4), s.dr_curve)
        }
    }

    /// A note starts (or restarts) on this voice; call before the clock
    /// resets. It starts from wherever it is.
    pub fn note_on(&mut self, s: &EnvSettings, clock: &NoteClock) {
        self.start = self.level(s, clock);
    }

    /// The note is released; call before the clock does.
    pub fn note_off(&mut self, s: &EnvSettings, clock: &NoteClock) {
        self.released_at = self.level(s, clock);
    }
}

/// An envelope's shape for the panel: (seconds, level) points through
/// attack, decay, `hold` seconds of sustain, then release.
pub fn env_sketch(s: &EnvSettings, hold: f32, points: usize) -> Vec<(f32, f32)> {
    let gate = s.attack + s.decay + hold;
    let total = gate + s.release;
    let at_gate = s.held(gate, 0.0);
    (0..=points)
        .map(|i| {
            let t = total * i as f32 / points as f32;
            let level = if t < gate { s.held(t, 0.0) } else { along(at_gate, 0.0, (t - gate) / s.release.max(1e-4), s.dr_curve) };
            (t, level)
        })
        .collect()
}

// ---- LFOs ----------------------------------------------------------------------------

/// An LFO's shape: a library wave (the four basic ones first), or one only
/// an LFO has.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LfoShape {
    Wave(usize),
    /// A new random value each cycle, eased into.
    Noise,
    /// A random walk.
    Brown,
    /// Sample and hold: a new random value each cycle, held.
    SampleHold,
}

impl LfoShape {
    /// From the Shape menu (wavelib.lib's lfoShape): the basic waves, the
    /// LFO-only shapes, then the rest of the library.
    pub fn from_menu(k: usize) -> Self {
        let only = crate::wavelib::LFO_ONLY.len();
        match k {
            0..=3 => LfoShape::Wave(k),
            4 => LfoShape::Noise,
            5 => LfoShape::Brown,
            6 => LfoShape::SampleHold,
            _ => LfoShape::Wave((k - only).min(crate::wavelib::WAVE_COUNT - 1)),
        }
    }

    /// The value at phase `p` of a shape that isn't random.
    pub fn wave(self, p: f32) -> Option<f32> {
        let p = p - p.floor();
        match self {
            // The basic waves drawn exactly (the library's are band-limited:
            // a square there ripples); as the library's, they start at 0 and rise.
            LfoShape::Wave(0) => Some((std::f32::consts::TAU * p).sin()),
            LfoShape::Wave(1) => Some(1.0 - 4.0 * ((p + 0.25).fract() - 0.5).abs()),
            LfoShape::Wave(2) => Some(2.0 * (p + 0.5).fract() - 1.0),
            LfoShape::Wave(3) => Some(if p < 0.5 { 1.0 } else { -1.0 }),
            LfoShape::Wave(w) => Some(crate::wavelib::wave(w, p)),
            _ => None,
        }
    }
}

/// An LFO's settings.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LfoSettings {
    pub shape: LfoShape,
    pub hz: f32,
    pub sync: bool,
    /// The synced length (DIVISIONS index).
    pub division: usize,
    /// Runs on through notes (each voice its own); off, each note restarts it.
    pub free: bool,
}

/// Synced lengths in beats, longest first, and their names (the Faust
/// menus' order: voice.lib's Division and Speed Division).
pub const DIVISIONS: [(f64, &str); 21] = [
    (16.0, "4/1"), (12.0, "2/1."), (8.0, "2/1"), (6.0, "1/1."), (16.0 / 3.0, "2/1T"), (4.0, "1/1"), (3.0, "1/2."),
    (8.0 / 3.0, "1/1T"), (2.0, "1/2"), (1.5, "1/4."), (4.0 / 3.0, "1/2T"), (1.0, "1/4"), (0.75, "1/8."),
    (2.0 / 3.0, "1/4T"), (0.5, "1/8"), (0.375, "1/16."), (1.0 / 3.0, "1/8T"), (0.25, "1/16"), (0.1875, "1/32."),
    (1.0 / 6.0, "1/16T"), (0.125, "1/32"),
];
/// 1/4.
pub const DEFAULT_DIVISION: usize = 11;

fn division_index(value: f32) -> usize {
    (value.round().max(0.0) as usize).min(DIVISIONS.len() - 1)
}

/// A synced division moved by a modulator: `shift` of the knob's travel
/// steps through the divisions (the whole travel, all of them); up is faster.
pub fn shifted_division(division: usize, shift: f32) -> usize {
    let steps = (shift * (DIVISIONS.len() - 1) as f32).round() as i64;
    (division as i64 + steps).clamp(0, DIVISIONS.len() as i64 - 1) as usize
}

/// A small, fast generator for the random shapes (xorshift32).
#[derive(Clone, Copy, Debug)]
pub struct Rng(u32);

impl Rng {
    pub fn new(seed: u32) -> Self {
        Self(seed.max(1))
    }
    fn next(&mut self) -> u32 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.0 = x;
        x
    }
    /// Uniform 0..1.
    pub fn unit(&mut self) -> f32 {
        (self.next() >> 8) as f32 / (1u32 << 24) as f32
    }
    /// Uniform -1..1.
    pub fn signed(&mut self) -> f32 {
        2.0 * self.unit() - 1.0
    }
    /// Roughly normal, mean 0, deviation 1 (the sum of four uniforms).
    fn gaussian(&mut self) -> f32 {
        (self.unit() + self.unit() + self.unit() + self.unit() - 2.0) * 1.732
    }
}

/// One LFO in one voice.
#[derive(Clone, Copy, Debug)]
pub struct LfoState {
    phase: f64,
    from: f32,
    to: f32,
    held: f32,
    walk: f32,
    rng: Rng,
}

impl LfoState {
    pub fn new(seed: u32) -> Self {
        let mut rng = Rng::new(seed);
        let phase = rng.unit() as f64;
        let (from, to, held) = (rng.signed(), rng.signed(), rng.signed());
        Self { phase, from, to, held, walk: 0.0, rng }
    }

    /// A note starts on this voice: unless it's free, the LFO restarts.
    pub fn note_on(&mut self, s: &LfoSettings) {
        if !s.free {
            self.phase = 0.0;
            self.new_cycle();
            self.walk = 0.0;
        }
    }

    fn new_cycle(&mut self) {
        self.from = self.to;
        self.to = self.rng.signed();
        self.held = self.rng.signed();
    }

    /// Advance `dt` seconds at `cycles_per_second` and return the value
    /// (-1..1). `locked`: synced and free, so it follows the song position
    /// (`beats` in a cycle of `len` beats): every voice in step.
    pub fn step(&mut self, shape: LfoShape, cycles_per_second: f64, dt: f32, locked: Option<(f64, f64)>) -> f32 {
        let next = match locked {
            Some((beats, len)) => (beats / len.max(1e-6)).rem_euclid(1.0),
            None => self.phase + cycles_per_second * dt as f64,
        };
        let wrapped = next >= 1.0 || next < self.phase;
        self.phase = next.rem_euclid(1.0);
        if wrapped {
            self.new_cycle();
        }
        let p = self.phase as f32;
        if let Some(v) = shape.wave(p) {
            return v;
        }
        match shape {
            LfoShape::Noise => {
                let ease = 0.5 - 0.5 * (std::f32::consts::PI * p).cos();
                self.from + (self.to - self.from) * ease
            }
            LfoShape::Brown => {
                // Steps scaled so it wanders about as far per cycle as the others.
                let cycles = (cycles_per_second as f32 * dt).max(0.0);
                self.walk += 2.0 * self.rng.gaussian() * cycles.sqrt();
                if self.walk > 1.0 {
                    self.walk = 2.0 - self.walk;
                } else if self.walk < -1.0 {
                    self.walk = -2.0 - self.walk;
                }
                self.walk = self.walk.clamp(-1.0, 1.0);
                self.walk
            }
            _ => self.held,
        }
    }
}

/// A sketch of an LFO shape (for the panel), `points` values across one cycle.
pub fn sketch(shape: LfoShape, points: usize) -> Vec<f32> {
    let mut rng = Rng::new(7);
    let holds: Vec<f32> = (0..6).map(|_| rng.signed()).collect();
    let mut walk = 0.0f32;
    (0..points)
        .map(|i| {
            let p = i as f32 / points as f32;
            match shape {
                LfoShape::Noise => {
                    let x = p * 4.0;
                    let k = x.floor() as usize;
                    let ease = 0.5 - 0.5 * (std::f32::consts::PI * (x - k as f32)).cos();
                    holds[k] + (holds[k + 1] - holds[k]) * ease
                }
                LfoShape::Brown => {
                    walk = (walk + 3.0 * rng.gaussian() / (points as f32).sqrt()).clamp(-1.0, 1.0);
                    walk
                }
                LfoShape::SampleHold => holds[(p * 4.0) as usize],
                LfoShape::Wave(_) => shape.wave(p).unwrap_or(0.0),
            }
        })
        .collect()
}

// ---- The wavetables' paths ----------------------------------------------------------

/// Where a wavetable is on its path: `morph` 0..3 (wave 1 to wave 4) and
/// `fade` 0..1 (the release's crossfade to wave 5).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Path {
    pub morph: f32,
    pub fade: f32,
}

/// A wavetable's path in one voice: the sustain's bounce, and where it was
/// when the note was released (the release fades from there).
#[derive(Clone, Copy, Debug, Default)]
pub struct PathState {
    bounce: f64,
    last: Path,
    released_from: f32,
}

impl PathState {
    /// Where it is now: [morph, fade].
    pub fn now(&self) -> [f32; 2] {
        [self.last.morph, self.last.fade]
    }

    pub fn note_on(&mut self) {
        self.bounce = 0.0;
        self.last = Path::default();
    }

    /// Advance `dt` seconds: envelope A is at `stage`; the sustain bounces
    /// 3 -> 4 -> 3 `round_trips` times a second.
    pub fn step(&mut self, stage: Stage, round_trips: f64, dt: f32) -> Path {
        let path = match stage {
            Stage::Attack(p) => Path { morph: p, fade: 0.0 },
            Stage::Decay(p) => Path { morph: 1.0 + p, fade: 0.0 },
            Stage::Sustain(_) => {
                self.bounce = (self.bounce + round_trips * dt as f64).rem_euclid(1.0);
                let tri = 1.0 - (2.0 * self.bounce as f32 - 1.0).abs();
                Path { morph: 2.0 + tri, fade: 0.0 }
            }
            Stage::Release(p) => {
                if self.last.fade == 0.0 {
                    self.released_from = self.last.morph;
                }
                Path { morph: self.released_from, fade: p.max(1e-6) }
            }
        };
        self.last = path;
        path
    }
}

// ---- Moving a knob -------------------------------------------------------------------

/// A knob's travel: `min..max`, curved by `factor` exactly as nih-plug's
/// skewed range (normalized = ((v - min) / (max - min)) ^ factor).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Travel {
    pub min: f32,
    pub max: f32,
    pub factor: f32,
}

impl Default for Travel {
    fn default() -> Self {
        Self { min: 0.0, max: 1.0, factor: 1.0 }
    }
}

impl Travel {
    pub fn normalize(&self, plain: f32) -> f32 {
        let t = ((plain - self.min) / (self.max - self.min)).clamp(0.0, 1.0);
        if self.factor == 1.0 { t } else { t.powf(self.factor) }
    }
    pub fn plain(&self, normalized: f32) -> f32 {
        let t = normalized.clamp(0.0, 1.0);
        let t = if self.factor == 1.0 { t } else { t.powf(1.0 / self.factor) };
        self.min + (self.max - self.min) * t
    }
}

/// The sources as they are in one voice this block (each -1..1 or 0..1;
/// the key as its part of five octaves from C4).
pub type SourceValues = [f32; SOURCE_COUNT];

/// How far the sources move a knob, from its depths: (travel, octaves).
/// The key moves a `key_in_octaves` knob in octaves per octave instead.
pub fn shift(target: &Target, depths: &[f32; SOURCE_COUNT], values: &SourceValues) -> (f32, f32) {
    let mut travel = 0.0;
    let mut octaves = 0.0;
    for (s, (d, v)) in depths.iter().zip(values).enumerate() {
        if *d == 0.0 {
            continue;
        }
        if s == KEY && target.key_in_octaves {
            octaves = d * v * 5.0;
        } else {
            travel += d * v;
        }
    }
    (travel, octaves)
}

/// What the knob plays at in one voice: its base (plain) moved by `travel`
/// (a share of its whole travel) and `octaves`: (plain value, travel).
pub fn apply(target: &Target, range: &Travel, base: f32, travel: f32, octaves: f32) -> (f32, f32) {
    if target.off_at_bottom && base <= range.min {
        return (range.min, 0.0);
    }
    let normalized = (range.normalize(base) + travel).clamp(0.0, 1.0);
    let mut plain = range.plain(normalized);
    if octaves != 0.0 {
        plain = (plain * octaves.exp2()).clamp(range.min, range.max);
    }
    if target.off_at_bottom {
        // Modulation brings it close to off, never quite there.
        plain = plain.max(range.min + 1e-4 * (range.max - range.min));
    }
    (plain, range.normalize(plain))
}

/// A depth as the readout writes it: "+21 %", or "+1.0 %" below 10 %.
pub fn percent_text(amount: f32) -> String {
    if amount.abs() < 0.0995 { format!("{:+.1} %", amount * 100.0) } else { format!("{:+.0} %", amount * 100.0) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn envelope_runs_attack_decay_sustain_release() {
        let s = EnvSettings { attack: 0.01, decay: 0.1, sustain: 0.5, release: 0.2, attack_curve: 0.0, dr_curve: 0.0 };
        let mut m = EnvState::default();
        let mut clock = NoteClock::default();
        m.note_on(&s, &clock);
        clock = NoteClock { gated: true, t_on: 0.0, t_off: 0.0 };
        assert_eq!(m.level(&s, &clock), 0.0);
        clock.t_on = 0.01;
        assert!((m.level(&s, &clock) - 1.0).abs() < 1e-3);
        clock.t_on = 0.2;
        assert!((m.level(&s, &clock) - 0.5).abs() < 1e-3);
        assert!(matches!(s.stage(&clock), Stage::Sustain(t) if (t - 0.09).abs() < 1e-5));
        m.note_off(&s, &clock);
        clock.gated = false;
        clock.t_off = 0.2;
        assert!(m.level(&s, &clock).abs() < 1e-3);
        // A retrigger starts from where it was, not from 0.
        clock.t_off = 0.05;
        let mid = m.level(&s, &clock);
        m.note_on(&s, &clock);
        clock = NoteClock { gated: true, t_on: 0.0, t_off: 0.0 };
        assert!((m.level(&s, &clock) - mid).abs() < 1e-6);
    }

    #[test]
    fn lfo_shapes_stay_in_range() {
        for k in 0..crate::wavelib::WAVE_COUNT + crate::wavelib::LFO_ONLY.len() {
            let shape = LfoShape::from_menu(k);
            let mut m = LfoState::new(9);
            for _ in 0..500 {
                let v = m.step(shape, 3.0, 0.0013, None);
                assert!(v.is_finite() && v.abs() <= 1.2, "{shape:?}: {v}");
            }
        }
        assert_eq!(LfoShape::from_menu(7), LfoShape::Wave(4));
        // Locked to the song: two voices started apart agree.
        let (mut a, mut b) = (LfoState::new(1), LfoState::new(2));
        let sine = LfoShape::Wave(0);
        assert_eq!(a.step(sine, 2.0, 0.01, Some((2.3, 1.0))), b.step(sine, 2.0, 0.01, Some((2.3, 1.0))));
    }

    #[test]
    fn the_path_follows_the_envelope() {
        let s = EnvSettings { attack: 0.1, decay: 0.1, sustain: 0.5, release: 0.4, attack_curve: 0.0, dr_curve: 0.0 };
        let mut path = PathState::default();
        let at = |t_on: f32| NoteClock { gated: true, t_on, t_off: 0.0 };
        assert!((path.step(s.stage(&at(0.05)), 1.0, 0.0).morph - 0.5).abs() < 1e-4);
        assert!((path.step(s.stage(&at(0.15)), 1.0, 0.0).morph - 1.5).abs() < 1e-4);
        // Sustaining at one round trip a second: wave 4 half a second in, back on 3 at one.
        let mut held = 0.0;
        for _ in 0..50 {
            held = path.step(Stage::Sustain(0.0), 1.0, 0.01).morph;
        }
        assert!((held - 3.0).abs() < 1e-3, "{held}");
        for _ in 0..25 {
            held = path.step(Stage::Sustain(0.0), 1.0, 0.01).morph;
        }
        assert!((held - 2.5).abs() < 1e-3, "{held}");
        // Released: it stays where it was and fades to wave 5.
        let off = NoteClock { gated: false, t_on: 1.0, t_off: 0.2 };
        let p = path.step(s.stage(&off), 1.0, 0.01);
        assert!((p.morph - 2.5).abs() < 1e-3 && (p.fade - 0.5).abs() < 1e-3, "{p:?}");
    }

    #[test]
    fn named_targets_and_depth_ids() {
        assert_eq!(TARGETS[CUTOFF].id, "filter.cutoff");
        assert_eq!(depth_id(CUTOFF, ENV_B), ENV_B_DEPTH);
        assert_eq!(depth_id(CUTOFF, LFO_X), "mod.filter_cutoff.x");
        assert_eq!(parse_depth_id("mod.filter_cutoff.x"), Some((CUTOFF, LFO_X)));
        for (k, f) in FIELDS.iter().enumerate() {
            assert_eq!(field_index(f), Some(k));
        }
        let mut ids: Vec<&str> = TARGETS.iter().map(|t| t.id).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), COUNT, "a target is listed twice");
    }

    #[test]
    fn travel_matches_the_knob_and_gated_knobs_stay_off() {
        let t = Travel { min: 20.0, max: 18000.0, factor: 0.25 };
        for v in [20.0, 440.0, 1200.0, 18000.0] {
            assert!((t.plain(t.normalize(v)) - v).abs() < 0.01 * v);
        }
        let fader = TARGETS[target_index("mix.noise").unwrap()];
        let lin = Travel::default();
        assert_eq!(apply(&fader, &lin, 0.0, 0.8, 0.0).0, 0.0);
        assert!(apply(&fader, &lin, 0.3, -0.8, 0.0).0 > 0.0);
        // The cutoff's key tracking: an octave up, half an octave higher at 0.5.
        let cutoff = TARGETS[CUTOFF];
        let mut depths = [0.0; SOURCE_COUNT];
        depths[KEY] = 0.5;
        let mut values = [0.0; SOURCE_COUNT];
        values[KEY] = 12.0 / 60.0;
        let (travel, octaves) = shift(&cutoff, &depths, &values);
        let up = apply(&cutoff, &t, 1000.0, travel, octaves).0;
        assert!((up / 1000.0 - 2f32.sqrt()).abs() < 1e-3, "{up}");
    }
}
