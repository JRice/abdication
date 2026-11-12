//! Every knob's own modulators (after Raycaster): per voice, an LFO and an
//! ADSR envelope, plus how far velocity, aftertouch, the mod wheel and the
//! key move it, or (each on its own) the LFO's or the envelope's amount. The engine runs them once a block and moves each knob within
//! its own travel (the host's normalized value, so a log knob moves evenly
//! in octaves): an amount of 1 sweeps it across its whole range. The voice
//! then plays the knob where they put it, drawing a straight line to each
//! new value over the block (voice.lib's `ramped`).
//!
//! No nih-plug types here: the engine and its tests use this on its own.

/// A knob that has modulators.
#[derive(Clone, Copy, Debug)]
pub struct Target {
    /// The knob's parameter id.
    pub id: &'static str,
    /// Its name in the panel's list of active modulations.
    pub short: &'static str,
    /// Off at the bottom of its range (a mix fader, Events Depth, Calls
    /// Level): it stays off while the knob is there, whatever moves it, and
    /// modulation never quite reaches the bottom from above.
    pub off_at_bottom: bool,
    /// Key tracking in octaves of the knob per octave of the note (the
    /// cutoff), rather than in its travel.
    pub key_in_octaves: bool,
}

const fn target(id: &'static str, short: &'static str) -> Target {
    Target { id, short, off_at_bottom: false, key_in_octaves: false }
}
const fn gated(id: &'static str, short: &'static str) -> Target {
    Target { id, short, off_at_bottom: true, key_in_octaves: false }
}

/// Every modulatable knob, in panel order (top bar, then row by row). Its
/// position here is its modulators' place in a saved patch: add new ones
/// at the end.
pub const TARGETS: [Target; 50] = [
    target("vintage.amount", "vintage"),
    target("unison.detune", "detune"),
    target("drive.amount", "drive"),
    target("dual_1.a", "dual1 a"),
    target("dual_1.b", "dual1 b"),
    target("wt_1.position", "wt1 pos"),
    target("wt_1.oscmod_depth", "wt1 oscmod"),
    target("dual_2.a", "dual2 a"),
    target("dual_2.b", "dual2 b"),
    target("wt_2.position", "wt2 pos"),
    target("wt_2.oscmod_depth", "wt2 oscmod"),
    target("noise.crackle_density", "crackle"),
    target("noise.tilt", "tilt"),
    target("transient.decay", "trans dec"),
    target("transient.pitch", "trans pitch"),
    gated("mix.osc_1", "osc1 amp"),
    gated("mix.osc_2", "osc2 amp"),
    gated("mix.noise", "noise amp"),
    gated("transient.level", "trans amp"),
    Target { id: "filter.cutoff", short: "filter", off_at_bottom: false, key_in_octaves: true },
    target("filter.resonance", "reso"),
    target("filter.morph_lp_notch_hp", "morph"),
    target("filter.bandpass", "bandpass"),
    target("filter.utility_hpf", "hpf"),
    target("events.density", "ev density"),
    gated("events.depth", "ev depth"),
    target("events.life", "ev life"),
    target("events.sharp", "ev sharp"),
    gated("calls.level", "calls"),
    target("calls.activity", "activity"),
    target("calls.rate", "calls rate"),
    target("noise.move", "move"),
    target("noise.rate", "move rate"),
    target("perform.global_pitch", "pitch"),
    // Ray (added last, so the earlier knobs keep their modulators in saved patches).
    target("ray_1.squish", "ray1 squish"),
    target("ray_1.skew", "ray1 skew"),
    target("ray_1.tilt", "ray1 tilt"),
    target("ray_1.center", "ray1 center"),
    target("ray_1.ray", "ray1 ray"),
    target("ray_1.orbit", "ray1 orbit"),
    target("ray_1.orbit_rate", "ray1 rate"),
    target("ray_1.fine", "ray1 fine"),
    target("ray_2.squish", "ray2 squish"),
    target("ray_2.skew", "ray2 skew"),
    target("ray_2.tilt", "ray2 tilt"),
    target("ray_2.center", "ray2 center"),
    target("ray_2.ray", "ray2 ray"),
    target("ray_2.orbit", "ray2 orbit"),
    target("ray_2.orbit_rate", "ray2 rate"),
    target("ray_2.fine", "ray2 fine"),
];
pub const COUNT: usize = TARGETS.len();

pub fn target_index(id: &str) -> Option<usize> {
    TARGETS.iter().position(|t| t.id == id)
}

/// The cutoff: its modulator starts as the old filter envelope and keytrack.
pub const CUTOFF: usize = 19;
/// Global Pitch: its modulator starts as vibrato on the mod wheel.
pub const PITCH: usize = 33;

/// What a played source (velocity, aftertouch, mod wheel, key) moves: the
/// knob, or its LFO's or envelope's amount (the mod wheel deepening a
/// vibrato, velocity a filter envelope).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dest {
    Knob,
    LfoAmount,
    EnvAmount,
}

pub const DEST_NAMES: [&str; 3] = ["Knob", "LFO Amt", "Env Amt"];

impl Dest {
    pub fn from_index(i: usize) -> Self {
        [Dest::Knob, Dest::LfoAmount, Dest::EnvAmount][i.min(2)]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shape {
    Sine,
    Triangle,
    Saw,
    Ramp,
    Square,
    /// A new random value each cycle, eased into.
    Noise,
    /// A random walk.
    Brown,
    /// Sample and hold: a new random value each cycle, held.
    SampleHold,
}

pub const SHAPES: [Shape; 8] = [Shape::Sine, Shape::Triangle, Shape::Saw, Shape::Ramp, Shape::Square, Shape::Noise,
                                Shape::Brown, Shape::SampleHold];
pub const SHAPE_NAMES: [&str; 8] = ["Sine", "Triangle", "Saw", "Ramp", "Square", "Noise", "Brown", "S&H"];

impl Shape {
    pub fn from_index(i: usize) -> Self {
        SHAPES[i.min(SHAPES.len() - 1)]
    }
}

/// Synced LFO lengths, in beats, longest first, and their names.
pub const DIVISIONS: [(f64, &str); 15] = [
    (16.0, "4 bars"), (8.0, "2 bars"), (4.0, "1 bar"), (3.0, "1/2."), (2.0, "1/2"), (1.5, "1/4."),
    (4.0 / 3.0, "1/2T"), (1.0, "1/4"), (0.75, "1/8."), (2.0 / 3.0, "1/4T"), (0.5, "1/8"),
    (1.0 / 3.0, "1/8T"), (0.25, "1/16"), (1.0 / 6.0, "1/16T"), (0.125, "1/32"),
];
/// 1/4.
pub const DEFAULT_DIVISION: usize = 7;

/// One knob's modulator settings, as the engine reads them each block.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ModSettings {
    pub shape: Shape,
    pub hz: f32,
    /// Synced: the cycle's length in beats (the rate is then ignored).
    pub sync: Option<f64>,
    /// Restart the LFO (at 0) with each note; otherwise it runs free.
    pub retrig: bool,
    pub lfo: f32,
    pub attack: f32,
    pub decay: f32,
    pub sustain: f32,
    pub release: f32,
    /// Each segment's bow (see `curve`).
    pub attack_curve: f32,
    pub decay_curve: f32,
    pub release_curve: f32,
    pub env: f32,
    pub vel: f32,
    pub touch: f32,
    pub wheel: f32,
    /// Travel per 5 octaves from middle C, or (the cutoff, moving the knob)
    /// octaves per octave.
    pub key: f32,
    /// What each played source moves.
    pub vel_to: Dest,
    pub touch_to: Dest,
    pub wheel_to: Dest,
    pub key_to: Dest,
}

impl Default for ModSettings {
    fn default() -> Self {
        Self { shape: Shape::Sine, hz: 1.0, sync: None, retrig: true, lfo: 0.0, attack: 0.005, decay: 0.4,
               sustain: 0.0, release: 0.5, attack_curve: DEFAULT_ATTACK_BOW, decay_curve: DEFAULT_BOW,
               release_curve: DEFAULT_BOW, env: 0.0, vel: 0.0, touch: 0.0, wheel: 0.0, key: 0.0,
               vel_to: Dest::Knob, touch_to: Dest::Knob, wheel_to: Dest::Knob, key_to: Dest::Knob }
    }
}

/// The played sources as they are in one voice (each 0..1; the key in
/// semitones), for `ModSettings::played`.
#[derive(Clone, Copy, Debug, Default)]
pub struct Played {
    pub velocity: f32,
    pub touch: f32,
    pub wheel: f32,
    pub key: f32,
}

const ON: f32 = 1e-4;

impl ModSettings {
    /// The LFO or the envelope moves the knob (or could, once a played
    /// source raises its amount).
    pub fn moving(&self) -> bool {
        let routed = |amount: f32, to: Dest| amount.abs() > ON && to != Dest::Knob;
        self.lfo.abs() > ON || self.env.abs() > ON || routed(self.vel, self.vel_to)
            || routed(self.touch, self.touch_to) || routed(self.wheel, self.wheel_to) || routed(self.key, self.key_to)
    }

    /// Anything moves the knob.
    pub fn active(&self) -> bool {
        self.moving() || [self.vel, self.touch, self.wheel, self.key].iter().any(|a| a.abs() > ON)
    }

    /// The amounts that are set, as (source, amount), for the panel's list:
    /// "mw" moves the knob, "mw>lfo" the LFO's amount.
    pub fn amounts(&self) -> impl Iterator<Item = (&'static str, f32)> {
        let name = |names: [&'static str; 3], to: Dest| names[to as usize];
        [("lfo", self.lfo), ("env", self.env), (name(["vel", "vel>lfo", "vel>env"], self.vel_to), self.vel),
         (name(["at", "at>lfo", "at>env"], self.touch_to), self.touch),
         (name(["mw", "mw>lfo", "mw>env"], self.wheel_to), self.wheel),
         (name(["key", "key>lfo", "key>env"], self.key_to), self.key)]
            .into_iter()
            .filter(|(_, a)| a.abs() > ON)
    }

    /// What the played sources add: to the knob (its travel; the key in
    /// octaves where the knob counts it so, returned apart), and to the
    /// LFO's and envelope's amounts.
    pub fn played(&self, target: &Target, p: &Played) -> PlayedShift {
        let from_c = p.key - 60.0;
        let mut shift = PlayedShift::default();
        for (amount, to, source) in [(self.vel, self.vel_to, p.velocity), (self.touch, self.touch_to, p.touch),
                                     (self.wheel, self.wheel_to, p.wheel), (self.key, self.key_to, from_c / 60.0)] {
            match to {
                Dest::Knob => shift.knob += amount * source,
                Dest::LfoAmount => shift.lfo += amount * source,
                Dest::EnvAmount => shift.env += amount * source,
            }
        }
        if target.key_in_octaves && self.key_to == Dest::Knob {
            shift.knob -= self.key * from_c / 60.0;
            shift.octaves = self.key * from_c / 12.0;
        }
        shift
    }
}

/// An amount as the panel's lists write it: "21%", or "1.0%" below 10 %.
pub fn percent_text(amount: f32) -> String {
    if amount.abs() < 0.0995 { format!("{:.1}%", amount * 100.0) } else { format!("{:.0}%", amount * 100.0) }
}

/// See `ModSettings::played`.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PlayedShift {
    pub knob: f32,
    pub octaves: f32,
    pub lfo: f32,
    pub env: f32,
}

/// How far a curve of ±1 bends: the exponent at full bow (Raycaster's).
pub const CURVE_DEPTH: f32 = 5.0;
/// The bows the segments start at: decays and releases as the envelopes were
/// before they could be bent (the Amp's old Curve at 0.5), falling fast and
/// tailing off; attacks bowed up the same amount, so they arrive sharply.
pub const DEFAULT_BOW: f32 = -0.6;
pub const DEFAULT_ATTACK_BOW: f32 = 0.6;

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

/// The envelope's level `t_on` seconds into a note that started from `start`.
fn held_level(s: &ModSettings, t_on: f32, start: f32) -> f32 {
    let a = s.attack.max(1e-4);
    if t_on < a {
        along(start, 1.0, t_on / a, s.attack_curve)
    } else {
        along(1.0, s.sustain, (t_on - a) / s.decay.max(1e-4), s.decay_curve)
    }
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

/// When a voice's note started and ended, shared by all its knobs' envelopes.
#[derive(Clone, Copy, Debug, Default)]
pub struct NoteClock {
    pub gated: bool,
    /// Seconds since the note began (stops counting at release).
    pub t_on: f32,
    /// Seconds since release.
    pub t_off: f32,
}

/// One knob's modulator in one voice.
#[derive(Clone, Copy, Debug)]
pub struct ModState {
    phase: f64,
    from: f32,
    to: f32,
    held: f32,
    walk: f32,
    rng: Rng,
    /// The envelope's level when the note began and when it was released.
    start: f32,
    released_at: f32,
}

impl ModState {
    pub fn new(seed: u32) -> Self {
        let mut rng = Rng::new(seed);
        let phase = rng.unit() as f64;
        let (from, to, held) = (rng.signed(), rng.signed(), rng.signed());
        Self { phase, from, to, held, walk: 0.0, rng, start: 0.0, released_at: 0.0 }
    }

    /// The envelope's level now.
    pub fn env_level(&self, s: &ModSettings, clock: &NoteClock) -> f32 {
        if clock.gated {
            held_level(s, clock.t_on, self.start)
        } else {
            along(self.released_at, 0.0, clock.t_off / s.release.max(1e-4), s.release_curve)
        }
    }

    /// A note starts (or restarts) on this voice; call before the clock resets.
    pub fn note_on(&mut self, s: &ModSettings, clock: &NoteClock) {
        self.start = self.env_level(s, clock);
        if s.retrig {
            self.phase = 0.0;
            self.new_cycle();
            self.walk = 0.0;
        }
    }

    /// The note is released; call before the clock does.
    pub fn note_off(&mut self, s: &ModSettings, clock: &NoteClock) {
        self.released_at = self.env_level(s, clock);
    }

    fn new_cycle(&mut self) {
        self.from = self.to;
        self.to = self.rng.signed();
        self.held = self.rng.signed();
    }

    /// Advance `dt` seconds (the clock already moved on) and return the
    /// travel the LFO and envelope add. `beats` is the song position, for
    /// a synced LFO.
    pub fn step(&mut self, s: &ModSettings, dt: f32, clock: &NoteClock, tempo: f64, beats: f64) -> f32 {
        let cycles_per_second = match s.sync {
            Some(len) => tempo / 60.0 / len.max(1e-3),
            None => s.hz as f64,
        };
        let next = match s.sync.filter(|_| !s.retrig) {
            // Locked to the song: every voice in step.
            Some(len) => (beats / len).rem_euclid(1.0),
            None => self.phase + cycles_per_second * dt as f64,
        };
        let wrapped = next >= 1.0 || next < self.phase;
        self.phase = next.rem_euclid(1.0);
        let mut out = 0.0;
        if s.lfo.abs() > ON {
            if wrapped {
                self.new_cycle();
            }
            out += s.lfo * self.lfo_value(s.shape, cycles_per_second as f32 * dt);
        }
        if s.env.abs() > ON {
            out += s.env * self.env_level(s, clock);
        }
        out
    }

    fn lfo_value(&mut self, shape: Shape, cycles: f32) -> f32 {
        let p = self.phase as f32;
        match shape {
            Shape::Sine => (std::f32::consts::TAU * p).sin(),
            Shape::Triangle => 1.0 - 4.0 * ((p + 0.25).fract() - 0.5).abs(),
            Shape::Saw => 1.0 - 2.0 * p,
            Shape::Ramp => 2.0 * p - 1.0,
            Shape::Square => if p < 0.5 { 1.0 } else { -1.0 },
            Shape::Noise => {
                let ease = 0.5 - 0.5 * (std::f32::consts::PI * p).cos();
                self.from + (self.to - self.from) * ease
            }
            Shape::Brown => {
                // Steps scaled so it wanders about as far per cycle as the others.
                self.walk += 2.0 * self.rng.gaussian() * cycles.max(0.0).sqrt();
                if self.walk > 1.0 {
                    self.walk = 2.0 - self.walk;
                } else if self.walk < -1.0 {
                    self.walk = -2.0 - self.walk;
                }
                self.walk = self.walk.clamp(-1.0, 1.0);
                self.walk
            }
            Shape::SampleHold => self.held,
        }
    }
}

/// A sketch of an LFO shape (for the panel), `points` values across one cycle.
pub fn sketch(shape: Shape, points: usize) -> Vec<f32> {
    let mut rng = Rng::new(7);
    let holds: Vec<f32> = (0..6).map(|_| rng.signed()).collect();
    let mut walk = 0.0f32;
    (0..points)
        .map(|i| {
            let p = i as f32 / points as f32;
            match shape {
                Shape::Noise => {
                    let x = p * 4.0;
                    let k = x.floor() as usize;
                    let ease = 0.5 - 0.5 * (std::f32::consts::PI * (x - k as f32)).cos();
                    holds[k] + (holds[k + 1] - holds[k]) * ease
                }
                Shape::Brown => {
                    walk = (walk + 3.0 * rng.gaussian() / (points as f32).sqrt()).clamp(-1.0, 1.0);
                    walk
                }
                Shape::SampleHold => holds[(p * 4.0) as usize],
                _ => {
                    let mut state = ModState::new(1);
                    state.phase = p as f64;
                    state.lfo_value(shape, 0.0)
                }
            }
        })
        .collect()
}

/// The envelope's shape for the panel: (seconds, level) points through
/// attack, decay, `hold` seconds of sustain, then release.
pub fn env_sketch(s: &ModSettings, hold: f32, points: usize) -> Vec<(f32, f32)> {
    let gate = s.attack + s.decay + hold;
    let total = gate + s.release;
    let at_gate = held_level(s, gate, 0.0);
    (0..=points)
        .map(|i| {
            let t = total * i as f32 / points as f32;
            let level = if t < gate {
                held_level(s, t, 0.0)
            } else {
                along(at_gate, 0.0, (t - gate) / s.release.max(1e-4), s.release_curve)
            };
            (t, level)
        })
        .collect()
}

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

/// What the knob plays at in one voice: its base (plain) moved by `moving`
/// (the LFO and envelope's travel) and `shift` (the played sources'):
/// (plain value, travel).
pub fn apply(target: &Target, travel: &Travel, base: f32, moving: f32, shift: &PlayedShift) -> (f32, f32) {
    if target.off_at_bottom && base <= travel.min {
        return (travel.min, 0.0);
    }
    let normalized = (travel.normalize(base) + moving + shift.knob).clamp(0.0, 1.0);
    let mut plain = travel.plain(normalized);
    if shift.octaves != 0.0 {
        plain = (plain * shift.octaves.exp2()).clamp(travel.min, travel.max);
    }
    if target.off_at_bottom {
        // Modulation brings it close to off, never quite there.
        plain = plain.max(travel.min + 1e-4 * (travel.max - travel.min));
    }
    (plain, travel.normalize(plain))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn envelope_runs_attack_decay_sustain_release() {
        let s = ModSettings { attack: 0.01, decay: 0.1, sustain: 0.5, release: 0.2, env: 1.0, ..Default::default() };
        let mut m = ModState::new(3);
        let mut clock = NoteClock::default();
        m.note_on(&s, &clock);
        clock = NoteClock { gated: true, t_on: 0.0, t_off: 0.0 };
        assert_eq!(m.env_level(&s, &clock), 0.0);
        clock.t_on = 0.01;
        assert!((m.env_level(&s, &clock) - 1.0).abs() < 1e-3);
        clock.t_on = 0.2;
        assert!((m.env_level(&s, &clock) - 0.5).abs() < 1e-3);
        m.note_off(&s, &clock);
        clock.gated = false;
        clock.t_off = 0.2;
        assert!(m.env_level(&s, &clock).abs() < 1e-3);
        // A retrigger starts from where it was, not from 0.
        clock.t_off = 0.05;
        let mid = m.env_level(&s, &clock);
        m.note_on(&s, &clock);
        clock = NoteClock { gated: true, t_on: 0.0, t_off: 0.0 };
        assert!((m.env_level(&s, &clock) - mid).abs() < 1e-6);
    }

    #[test]
    fn lfo_shapes_stay_in_range_and_sync_locks_voices() {
        for shape in SHAPES {
            let s = ModSettings { shape, lfo: 1.0, hz: 3.0, ..Default::default() };
            let mut m = ModState::new(9);
            let clock = NoteClock::default();
            for _ in 0..2000 {
                let v = m.step(&s, 0.0013, &clock, 120.0, 0.0);
                assert!(v.is_finite() && v.abs() <= 1.0 + 1e-6, "{shape:?}: {v}");
            }
        }
        // Synced and free-running: two voices started apart agree.
        let s = ModSettings { lfo: 1.0, sync: Some(1.0), retrig: false, ..Default::default() };
        let (mut a, mut b) = (ModState::new(1), ModState::new(2));
        let clock = NoteClock::default();
        assert_eq!(a.step(&s, 0.01, &clock, 120.0, 2.3), b.step(&s, 0.01, &clock, 120.0, 2.3));
    }

    #[test]
    fn named_targets_are_where_they_say() {
        assert_eq!(TARGETS[CUTOFF].id, "filter.cutoff");
        assert_eq!(TARGETS[PITCH].id, "perform.global_pitch");
    }

    #[test]
    fn travel_matches_the_knob_and_gated_knobs_stay_off() {
        let t = Travel { min: 20.0, max: 18000.0, factor: 0.25 };
        for v in [20.0, 440.0, 1200.0, 18000.0] {
            assert!((t.plain(t.normalize(v)) - v).abs() < 0.01 * v);
        }
        let fader = TARGETS[target_index("mix.osc_2").unwrap()];
        let lin = Travel::default();
        let none = PlayedShift::default();
        assert_eq!(apply(&fader, &lin, 0.0, 0.8, &none).0, 0.0);
        assert!(apply(&fader, &lin, 0.3, -0.8, &none).0 > 0.0);
        // The cutoff's key tracking: an octave up, half an octave higher at 0.5.
        let cutoff = TARGETS[CUTOFF];
        let s = ModSettings { key: 0.5, ..Default::default() };
        let shift = s.played(&cutoff, &Played { key: 72.0, ..Default::default() });
        let up = apply(&cutoff, &t, 1000.0, 0.0, &shift).0;
        assert!((up / 1000.0 - 2f32.sqrt()).abs() < 1e-3, "{up}");
        // Routed to the LFO's amount, the mod wheel deepens it and leaves the knob.
        let s = ModSettings { wheel: 0.2, wheel_to: Dest::LfoAmount, ..Default::default() };
        let shift = s.played(&TARGETS[PITCH], &Played { wheel: 0.5, key: 60.0, ..Default::default() });
        assert_eq!(shift, PlayedShift { lfo: 0.1, ..Default::default() });
        assert!(s.moving());
    }
}
