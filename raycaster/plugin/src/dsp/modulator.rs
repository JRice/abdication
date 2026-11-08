//! The dynamic modulator: every modulatable control has its own, per voice. Each is an LFO
//! (seven shapes, free or synced to the host, optionally restarted on each note) plus a
//! simple attack-decay envelope started by the note. Both move the control within its own
//! travel: an amount of 1 sweeps it across its whole range.

use nih_plug::prelude::Enum;

use super::envelope::along;
use super::random::Rng;

#[derive(Enum, Clone, Copy, PartialEq, Eq, Debug)]
pub enum LfoShape {
    #[id = "sine"]
    Sine,
    #[id = "saw"]
    Saw,
    #[id = "ramp"]
    Ramp,
    #[id = "square"]
    Square,
    #[id = "noise"]
    Noise,
    #[id = "brown"]
    #[name = "Brown"]
    Brown,
    #[id = "sh"]
    #[name = "S&H"]
    SampleHold,
}

pub const SHAPES: [LfoShape; 7] =
    [LfoShape::Sine, LfoShape::Saw, LfoShape::Ramp, LfoShape::Square, LfoShape::Noise, LfoShape::Brown, LfoShape::SampleHold];

impl LfoShape {
    pub fn label(self) -> &'static str {
        match self {
            LfoShape::Sine => "Sine",
            LfoShape::Saw => "Saw",
            LfoShape::Ramp => "Ramp",
            LfoShape::Square => "Square",
            LfoShape::Noise => "Noise",
            LfoShape::Brown => "Brown",
            LfoShape::SampleHold => "S&H",
        }
    }
}

/// Synced lengths, in quarter-note beats.
#[derive(Enum, Clone, Copy, PartialEq, Eq, Debug)]
pub enum Division {
    #[id = "4bar"]
    #[name = "4 bars"]
    FourBars,
    #[id = "2bar"]
    #[name = "2 bars"]
    TwoBars,
    #[id = "1bar"]
    #[name = "1 bar"]
    OneBar,
    #[id = "2"]
    #[name = "1/2"]
    Half,
    #[id = "2d"]
    #[name = "1/2."]
    HalfDotted,
    #[id = "2t"]
    #[name = "1/2T"]
    HalfTriplet,
    #[id = "4"]
    #[name = "1/4"]
    Quarter,
    #[id = "4d"]
    #[name = "1/4."]
    QuarterDotted,
    #[id = "4t"]
    #[name = "1/4T"]
    QuarterTriplet,
    #[id = "8"]
    #[name = "1/8"]
    Eighth,
    #[id = "8d"]
    #[name = "1/8."]
    EighthDotted,
    #[id = "8t"]
    #[name = "1/8T"]
    EighthTriplet,
    #[id = "16"]
    #[name = "1/16"]
    Sixteenth,
    #[id = "16d"]
    #[name = "1/16."]
    SixteenthDotted,
    #[id = "16t"]
    #[name = "1/16T"]
    SixteenthTriplet,
    #[id = "32"]
    #[name = "1/32"]
    ThirtySecond,
}

impl Division {
    pub fn beats(self) -> f64 {
        match self {
            Division::FourBars => 16.0,
            Division::TwoBars => 8.0,
            Division::OneBar => 4.0,
            Division::Half => 2.0,
            Division::HalfDotted => 3.0,
            Division::HalfTriplet => 4.0 / 3.0,
            Division::Quarter => 1.0,
            Division::QuarterDotted => 1.5,
            Division::QuarterTriplet => 2.0 / 3.0,
            Division::Eighth => 0.5,
            Division::EighthDotted => 0.75,
            Division::EighthTriplet => 1.0 / 3.0,
            Division::Sixteenth => 0.25,
            Division::SixteenthDotted => 0.375,
            Division::SixteenthTriplet => 1.0 / 6.0,
            Division::ThirtySecond => 0.125,
        }
    }
}

/// One control's modulator settings, as the knobs set them.
#[derive(Clone, Copy, Debug)]
pub struct ModSettings {
    pub shape: LfoShape,
    pub hz: f32,
    /// Beats per cycle, when synced.
    pub sync: Option<f64>,
    pub retrig: bool,
    /// -1..1 of the control's travel.
    pub lfo: f32,
    pub attack_s: f32,
    pub decay_s: f32,
    pub env: f32,
    /// Bows of the envelope's attack and decay (see `envelope::curve`).
    pub attack_curve: f32,
    pub decay_curve: f32,
    /// How far the mod wheel and aftertouch move the control at full travel, either way.
    pub wheel: f32,
    pub touch: f32,
}

impl Default for ModSettings {
    fn default() -> Self {
        Self {
            shape: LfoShape::Sine,
            hz: 1.0,
            sync: None,
            retrig: false,
            lfo: 0.0,
            attack_s: 0.005,
            decay_s: 0.4,
            env: 0.0,
            attack_curve: 0.0,
            decay_curve: DEFAULT_DECAY_BOW,
            wheel: 0.0,
            touch: 0.0,
        }
    }
}

impl ModSettings {
    pub fn active(&self) -> bool {
        self.lfo.abs() > 1e-4 || self.env.abs() > 1e-4
    }
}

/// The host's tempo and, while it plays, where it is.
#[derive(Clone, Copy, Debug)]
pub struct Clock {
    pub tempo: f64,
    /// Song position in quarter notes at the start of this block, if playing.
    pub beats: Option<f64>,
}

impl Default for Clock {
    fn default() -> Self {
        Self { tempo: 120.0, beats: None }
    }
}

/// Envelope decays to -40 dB at the end of its decay time.
/// The modulator envelope's decay as it was before curves: close to exponential.
pub const DEFAULT_DECAY_BOW: f32 = -0.9;

#[derive(Clone)]
pub struct ModState {
    phase: f64,
    /// Smooth random: from one value to the next across a cycle.
    from: f32,
    to: f32,
    held: f32,
    walk: f32,
    env_time: f32,
    env_running: bool,
    rng: Rng,
}

impl ModState {
    pub fn new(seed: u32) -> Self {
        let mut rng = Rng::new(seed);
        let phase = rng.unit() as f64;
        let (from, to, held) = (rng.signed(), rng.signed(), rng.signed());
        Self { phase, from, to, held, walk: 0.0, env_time: 0.0, env_running: false, rng }
    }

    /// A new note. `retrigger` is false for a legato note in mono, which keeps everything.
    pub fn note_on(&mut self, s: &ModSettings, retrigger: bool) {
        if !retrigger {
            return;
        }
        self.env_time = 0.0;
        self.env_running = true;
        if s.retrig {
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

    /// Moves on by `dt` seconds and returns the offset, in the control's normalized travel.
    pub fn step(&mut self, s: &ModSettings, dt: f32, clock: &Clock) -> f32 {
        let cycles_per_second = match s.sync {
            Some(beats) => clock.tempo / 60.0 / beats.max(1e-3),
            None => s.hz as f64,
        };
        let locked = s.sync.zip(clock.beats).filter(|_| !s.retrig);
        let next = match locked {
            Some((beats, position)) => (position / beats).rem_euclid(1.0),
            None => self.phase + cycles_per_second * dt as f64,
        };
        let wrapped = next >= 1.0 || next < self.phase;
        self.phase = next.rem_euclid(1.0);
        if wrapped {
            self.new_cycle();
        }
        let phase = self.phase as f32;
        let lfo = if s.lfo.abs() > 1e-4 {
            match s.shape {
                LfoShape::Sine => (std::f32::consts::TAU * phase).sin(),
                LfoShape::Saw => 1.0 - 2.0 * phase,
                LfoShape::Ramp => 2.0 * phase - 1.0,
                LfoShape::Square => {
                    if phase < 0.5 {
                        1.0
                    } else {
                        -1.0
                    }
                }
                LfoShape::Noise => {
                    let blend = 0.5 - 0.5 * (std::f32::consts::PI * phase).cos();
                    self.from + (self.to - self.from) * blend
                }
                LfoShape::Brown => {
                    // A random walk that covers about the full range once a cycle.
                    let step = 2.0 * self.rng.gaussian() * ((cycles_per_second as f32) * dt).sqrt();
                    self.walk += step;
                    if self.walk > 1.0 {
                        self.walk = 2.0 - self.walk;
                    } else if self.walk < -1.0 {
                        self.walk = -2.0 - self.walk;
                    }
                    self.walk = self.walk.clamp(-1.0, 1.0);
                    self.walk
                }
                LfoShape::SampleHold => self.held,
            }
        } else {
            0.0
        };
        let env = if self.env_running {
            self.env_time += dt;
            let (a, d) = (s.attack_s.max(1e-4), s.decay_s.max(1e-4));
            if self.env_time < a {
                along(0.0, 1.0, self.env_time / a, s.attack_curve)
            } else {
                let progress = (self.env_time - a) / d;
                if progress >= 1.0 {
                    self.env_running = false;
                }
                along(1.0, 0.0, progress, s.decay_curve)
            }
        } else {
            0.0
        };
        lfo * s.lfo + env * s.env
    }
}

/// One cycle of a shape, for drawing (random shapes drawn from a fixed seed).
pub fn sketch(shape: LfoShape, points: usize) -> Vec<f32> {
    let mut rng = Rng::new(7);
    let mut walk = 0.0f32;
    let holds: Vec<f32> = (0..5).map(|_| rng.signed()).collect();
    (0..points)
        .map(|i| {
            let p = i as f32 / points as f32;
            match shape {
                LfoShape::Sine => (std::f32::consts::TAU * p).sin(),
                LfoShape::Saw => 1.0 - 2.0 * p,
                LfoShape::Ramp => 2.0 * p - 1.0,
                LfoShape::Square => {
                    if p < 0.5 {
                        1.0
                    } else {
                        -1.0
                    }
                }
                LfoShape::Noise => {
                    let x = p * 4.0;
                    let k = x.floor() as usize;
                    let f = x - k as f32;
                    let blend = 0.5 - 0.5 * (std::f32::consts::PI * f).cos();
                    holds[k] + (holds[k + 1] - holds[k]) * blend
                }
                LfoShape::Brown => {
                    walk = (walk + 0.25 * rng.gaussian()).clamp(-1.0, 1.0);
                    walk
                }
                LfoShape::SampleHold => holds[(p * 4.0) as usize],
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(s: &ModSettings, seconds: f32, clock: &Clock) -> Vec<f32> {
        let mut m = ModState::new(3);
        m.note_on(s, true);
        let dt = 16.0 / 48_000.0;
        (0..(seconds / dt) as usize).map(|_| m.step(s, dt, clock)).collect()
    }

    #[test]
    fn every_shape_stays_in_range() {
        for shape in SHAPES {
            let s = ModSettings { shape, hz: 7.0, lfo: 1.0, ..ModSettings::default() };
            let out = run(&s, 3.0, &Clock::default());
            assert!(out.iter().all(|v| v.abs() <= 1.0001), "{shape:?}");
            let spread = out.iter().fold(f32::MIN, |a, b| a.max(*b)) - out.iter().fold(f32::MAX, |a, b| a.min(*b));
            assert!(spread > 0.8, "{shape:?} only moved {spread}");
        }
    }

    #[test]
    fn a_retriggered_sine_starts_at_zero_and_a_synced_one_follows_tempo() {
        let s = ModSettings { lfo: 1.0, retrig: true, sync: Some(1.0), ..ModSettings::default() };
        // 120 BPM, one cycle a beat = 2 Hz: a quarter cycle in 125 ms.
        let out = run(&s, 0.125, &Clock { tempo: 120.0, beats: None });
        assert!((out.last().unwrap() - 1.0).abs() < 0.02, "{:?}", out.last());
    }

    #[test]
    fn locked_to_the_host_when_not_retriggered() {
        let s = ModSettings { lfo: 1.0, sync: Some(4.0), shape: LfoShape::Ramp, ..ModSettings::default() };
        let mut m = ModState::new(9);
        let v = m.step(&s, 0.001, &Clock { tempo: 120.0, beats: Some(10.0) });
        // 10 beats into a 4-beat cycle: halfway, so a ramp reads 0.
        assert!(v.abs() < 1e-3, "{v}");
    }

    #[test]
    fn the_envelope_rises_and_falls() {
        let s = ModSettings { env: 1.0, attack_s: 0.1, decay_s: 0.5, ..ModSettings::default() };
        let out = run(&s, 1.0, &Clock::default());
        let peak = out.iter().cloned().fold(0.0, f32::max);
        assert!(peak > 0.95);
        assert!(*out.last().unwrap() < 0.01);
    }
}
