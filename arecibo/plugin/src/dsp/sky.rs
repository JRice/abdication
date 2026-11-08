//! The ionosphere, more or less: the standard HF model, in which a signal arrives by
//! several paths, each slowly fading in strength and phase as a Rayleigh process, plus
//! what the atmosphere adds of its own.

use super::filters::rate_for;
use super::random::Rng;

/// Fading only moves slowly, so it's worked out this far apart and interpolated.
const FADING_STEP: usize = 64;

/// A complex gain that wanders like one sky path: unit power, Rayleigh in strength, moving
/// at about the Doppler spread.
#[derive(Clone)]
pub struct Fading {
    stages: [(f32, f32); 2],
    previous: (f32, f32),
    current: (f32, f32),
    position: usize,
    pole: f32,
    normal: f32,
    step_rate: f32,
    rng: Rng,
}

impl Fading {
    pub fn new(sample_rate: f32, seed: u32) -> Self {
        let mut fading = Self {
            stages: [(0.0, 0.0); 2],
            previous: (1.0, 0.0),
            current: (1.0, 0.0),
            position: 0,
            pole: 0.0,
            normal: 1.0,
            step_rate: sample_rate / FADING_STEP as f32,
            rng: Rng::new(seed),
        };
        fading.set_spread(0.5);
        // Start somewhere in its distribution, rather than building up from nothing.
        let rate = 1.0 / (1.0 - fading.pole);
        for _ in 0..(rate * 8.0) as usize {
            fading.advance();
        }
        fading
    }

    /// The Doppler spread in Hz. A spread of 0.1 is a quiet day, 1 a poor one, 10 flutter.
    pub fn set_spread(&mut self, hz: f32) {
        self.pole = 1.0 - rate_for(1.0 / (std::f32::consts::TAU * hz.max(1e-3)), self.step_rate);
        // Two identical one-poles in a row. Their combined power gain has this closed form.
        let a = self.pole;
        let g = 1.0 - a;
        self.normal = ((1.0 - a * a).powi(3) / (g.powi(4) * (1.0 + a * a))).sqrt();
    }

    fn advance(&mut self) {
        let (g, a) = (1.0 - self.pole, self.pole);
        // Each part has half the power.
        let noise = (self.rng.gaussian() * std::f32::consts::FRAC_1_SQRT_2, self.rng.gaussian() * std::f32::consts::FRAC_1_SQRT_2);
        let first = (a * self.stages[0].0 + g * noise.0, a * self.stages[0].1 + g * noise.1);
        let second = (a * self.stages[1].0 + g * first.0, a * self.stages[1].1 + g * first.1);
        self.stages = [first, second];
        self.previous = self.current;
        self.current = (second.0 * self.normal, second.1 * self.normal);
    }

    #[inline(always)]
    pub fn gain(&mut self) -> (f32, f32) {
        if self.position == 0 {
            self.advance();
            self.position = FADING_STEP;
        }
        self.position -= 1;
        let along = 1.0 - self.position as f32 / FADING_STEP as f32;
        (self.previous.0 + (self.current.0 - self.previous.0) * along, self.previous.1 + (self.current.1 - self.previous.1) * along)
    }
}

/// Lightning a long way off: a hiss, plus crashes that arrive at random and die quickly.
#[derive(Clone)]
pub struct Atmosphere {
    crash: f32,
    crash_decay: f32,
    per_sample: f32,
    rng: Rng,
    /// Seconds into the whistler now falling, if there is one.
    whistler: Option<f32>,
    whistler_phase: f32,
}

impl Atmosphere {
    const CRASH_S: f32 = 0.006;
    const CRASH_LEVEL: f32 = 0.3;
    const HISS_LEVEL: f32 = 0.012;
    /// Some lightning's energy runs out along the Earth's magnetic field and back, the high
    /// frequencies first, and arrives as a long falling tone.
    const WHISTLER_CHANCE: f32 = 0.08;
    const WHISTLER_TOP_HZ: f32 = 6_000.0;
    const WHISTLER_FALL_S: f32 = 0.09;
    const WHISTLER_S: f32 = 1.8;
    const WHISTLER_LEVEL: f32 = 0.09;

    pub fn new(sample_rate: f32, seed: u32) -> Self {
        Self { crash: 0.0, crash_decay: (-1.0 / (Self::CRASH_S * sample_rate)).exp(), per_sample: 1.0 / sample_rate, rng: Rng::new(seed), whistler: None, whistler_phase: 0.0 }
    }

    /// `activity` 0..1. A complex noise sample.
    #[inline(always)]
    pub fn next(&mut self, activity: f32) -> (f32, f32) {
        let crashes_per_second = 9.0 * activity * activity;
        if self.rng.chance(crashes_per_second * self.per_sample) {
            self.crash = Self::CRASH_LEVEL * self.rng.exponential(1.0).min(4.0) * activity;
            if self.whistler.is_none() && self.rng.chance(Self::WHISTLER_CHANCE) {
                self.whistler = Some(0.0);
            }
        }
        self.crash *= self.crash_decay;
        let level = Self::HISS_LEVEL * activity + self.crash;
        let tone = self.next_whistler();
        (self.rng.gaussian() * level + tone.0, self.rng.gaussian() * level + tone.1)
    }
}

impl Atmosphere {
    #[inline(always)]
    fn next_whistler(&mut self) -> (f32, f32) {
        let Some(t) = self.whistler else { return (0.0, 0.0) };
        let hz = Self::WHISTLER_TOP_HZ / (1.0 + t / Self::WHISTLER_FALL_S).powi(2);
        self.whistler_phase += std::f32::consts::TAU * hz * self.per_sample;
        if self.whistler_phase > std::f32::consts::TAU {
            self.whistler_phase -= std::f32::consts::TAU;
        }
        let progress = t / Self::WHISTLER_S;
        self.whistler = (progress < 1.0).then_some(t + self.per_sample);
        let level = Self::WHISTLER_LEVEL * (std::f32::consts::PI * progress.min(1.0)).sin();
        let (sin, cos) = self.whistler_phase.sin_cos();
        (cos * level, sin * level)
    }
}

/// A fixed delay for a complex signal, read between samples.
#[derive(Clone)]
pub struct Path {
    re: Vec<f32>,
    im: Vec<f32>,
    head: usize,
    mask: usize,
}

impl Path {
    pub fn new(longest_samples: usize) -> Self {
        let size = (longest_samples + 2).next_power_of_two();
        Self { re: vec![0.0; size], im: vec![0.0; size], head: 0, mask: size - 1 }
    }

    #[inline(always)]
    pub fn write(&mut self, z: (f32, f32)) {
        self.head = (self.head + 1) & self.mask;
        self.re[self.head] = z.0;
        self.im[self.head] = z.1;
    }

    /// What was written this many samples ago.
    #[inline(always)]
    pub fn read(&self, samples: f32) -> (f32, f32) {
        let whole = samples as usize;
        let part = samples - whole as f32;
        let (a, b) = ((self.head + self.mask + 1 - whole) & self.mask, (self.head + self.mask - whole) & self.mask);
        (self.re[a] + (self.re[b] - self.re[a]) * part, self.im[a] + (self.im[b] - self.im[a]) * part)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RATE: f32 = 96_000.0;

    #[test]
    fn a_sky_path_is_rayleigh_with_unit_power() {
        let mut fading = Fading::new(RATE, 3);
        fading.set_spread(20.0);
        let mut powers = Vec::new();
        for i in 0..(RATE as usize * 400) {
            let (re, im) = fading.gain();
            if i % 480 == 0 {
                powers.push(re * re + im * im);
            }
        }
        let mean = powers.iter().sum::<f32>() / powers.len() as f32;
        assert!((mean - 1.0).abs() < 0.12, "{mean}");
        // For Rayleigh fading, power is exponentially distributed: this far down, 1 - e^-x.
        let share_below = |x: f32| powers.iter().filter(|p| **p < x).count() as f32 / powers.len() as f32;
        for (x, expected) in [(0.1, 0.095), (0.5, 0.393), (2.0, 0.865)] {
            assert!((share_below(x) - expected).abs() < 0.04, "{x}: {} vs {expected}", share_below(x));
        }
    }

    #[test]
    fn the_spread_sets_how_fast_it_moves() {
        let moved = |spread: f32| {
            let mut fading = Fading::new(RATE, 5);
            fading.set_spread(spread);
            let mut last = fading.gain();
            let mut total = 0.0;
            for _ in 0..(RATE as usize * 60) {
                let now = fading.gain();
                total += (now.0 - last.0).abs() + (now.1 - last.1).abs();
                last = now;
            }
            total
        };
        assert!(moved(5.0) > moved(0.5) * 3.0);
    }

    #[test]
    fn the_atmosphere_is_quiet_when_calm_and_crashes_when_not() {
        let peak = |activity: f32| {
            let mut sky = Atmosphere::new(RATE, 1);
            (0..(RATE as usize * 10)).map(|_| sky.next(activity).0.abs()).fold(0.0, f32::max)
        };
        assert_eq!(peak(0.0), 0.0);
        assert!(peak(1.0) > peak(0.1) * 5.0);
    }

    #[test]
    fn a_path_delays_by_what_it_says() {
        let mut path = Path::new(100);
        for i in 0..50 {
            path.write((i as f32, -(i as f32)));
        }
        assert_eq!(path.read(10.0), (39.0, -39.0));
        assert_eq!(path.read(10.5), (38.5, -38.5));
    }

    #[test]
    fn a_whistler_falls() {
        let mut sky = Atmosphere::new(RATE, 1);
        sky.whistler = Some(0.0);
        // Count zero crossings early and late: the pitch should have fallen a long way.
        let crossings = |sky: &mut Atmosphere, samples: usize| {
            let (mut count, mut last) = (0, 0.0f32);
            for _ in 0..samples {
                let x = sky.next_whistler().0;
                count += (x.signum() != last.signum()) as usize;
                last = x;
            }
            count
        };
        let early = crossings(&mut sky, RATE as usize / 10);
        let _ = crossings(&mut sky, RATE as usize);
        let late = crossings(&mut sky, RATE as usize / 10);
        assert!(early > late * 4, "{early} {late}");
    }
}
