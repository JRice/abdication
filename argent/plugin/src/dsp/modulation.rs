//! The motion, at every timescale: a tide over minutes, flares every few seconds, blooms
//! a few times a second inside each flare, a tremolo, and each voice's pitch wandering.
//! Shaped by the Dust Devices recording (see `fizmo-notes.md`).

use std::f32::consts::TAU;

use super::random::Rng;

/// The tremolo's rate and the share of its second harmonic, as measured: 13 % depth at
/// 2.87 Hz with 6 % at twice that.
pub const TREMOR_HZ: f32 = 2.87;
const TREMOR_SECOND: f32 = 0.45;
const TREMOR_SECOND_PHASE: f32 = 0.7;
pub const BLOOM_HZ: f32 = 3.4;
const BLOOM_DRIFT: f32 = 0.2;
/// The shortest bloom is this share of the tallest.
const BLOOM_SPREAD: f32 = 0.6;
const FLARE_DRIFT: f32 = 0.3;
/// The swell's peak is narrow and its foot wide, as the recording's are: cone-shaped
/// on a spectrogram.
const FLARE_SHARPNESS: i32 = 3;
/// The lowest flare reaches this share of the tallest, as the recording's alternate
/// between tall and short.
const FLARE_SPREAD: f32 = 0.55;

/// A smooth random value, -1..1, moving to a new target about `rate` times a second.
#[derive(Clone)]
pub struct Wander {
    rng: Rng,
    from: f32,
    to: f32,
    position: f32,
}

impl Wander {
    pub fn new(seed: u32) -> Self {
        let mut rng = Rng::new(seed);
        let (from, to) = (rng.signed(), rng.signed());
        let position = rng.unit();
        Self { rng, from, to, position }
    }

    /// `step` is the rate divided by the sample rate.
    #[inline(always)]
    pub fn next(&mut self, step: f32) -> f32 {
        self.position += step;
        if self.position >= 1.0 {
            self.position -= self.position.floor();
            self.from = self.to;
            self.to = self.rng.signed();
        }
        let u = self.position * self.position * (3.0 - 2.0 * self.position);
        self.from + (self.to - self.from) * u
    }
}

/// An LFO whose rate wanders by up to `drift` of itself.
#[derive(Clone)]
struct Drifting {
    phase: f32,
    drift: Wander,
    amount: f32,
}

impl Drifting {
    fn new(seed: u32, amount: f32) -> Self {
        Self { phase: 0.0, drift: Wander::new(seed), amount }
    }

    /// Advances; true when a new cycle begins.
    #[inline(always)]
    fn advance(&mut self, hz: f32, sample_rate: f32) -> bool {
        let rate = hz * (1.0 + self.amount * self.drift.next(0.5 * hz / sample_rate));
        self.phase += rate / sample_rate;
        if self.phase >= 1.0 {
            self.phase -= self.phase.floor();
            true
        } else {
            false
        }
    }

    fn hump(&self) -> f32 {
        0.5 - 0.5 * (TAU * self.phase).cos()
    }
}

/// The slow swell: mostly low, rising once a cycle to a height drawn for that cycle.
#[derive(Clone)]
pub struct Flare {
    lfo: Drifting,
    rng: Rng,
    height: f32,
}

impl Flare {
    pub fn new(seed: u32) -> Self {
        Self { lfo: Drifting::new(seed, FLARE_DRIFT), rng: Rng::new(seed ^ 0x2545_f491), height: 1.0 }
    }

    #[inline(always)]
    pub fn next(&mut self, period_s: f32, sample_rate: f32) -> f32 {
        if self.lfo.advance(1.0 / period_s, sample_rate) {
            self.height = 1.0 - FLARE_SPREAD * self.rng.unit();
        }
        self.lfo.hump().powi(FLARE_SHARPNESS) * self.height
    }
}

/// The fast flicker, each bloom drawn its own height.
#[derive(Clone)]
pub struct Bloom {
    lfo: Drifting,
    rng: Rng,
    height: f32,
}

impl Bloom {
    pub fn new(seed: u32) -> Self {
        Self { lfo: Drifting::new(seed, BLOOM_DRIFT), rng: Rng::new(seed ^ 0x5bd1_e995), height: 1.0 }
    }

    #[inline(always)]
    pub fn next(&mut self, sample_rate: f32) -> f32 {
        if self.lfo.advance(BLOOM_HZ, sample_rate) {
            self.height = 1.0 - BLOOM_SPREAD * self.rng.unit();
        }
        self.lfo.hump() * self.height
    }
}

/// A gentle tremolo with a little of its own second harmonic.
#[derive(Clone, Default)]
pub struct Tremor {
    phase: f32,
}

impl Tremor {
    #[inline(always)]
    pub fn next(&mut self, depth: f32, sample_rate: f32) -> f32 {
        let w = TAU * self.phase;
        self.phase = (self.phase + TREMOR_HZ / sample_rate).fract();
        1.0 + depth * (w.sin() + TREMOR_SECOND * (2.0 * w + TREMOR_SECOND_PHASE).sin())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SR: f32 = 48_000.0;

    #[test]
    fn wander_stays_in_range_and_moves_smoothly() {
        let mut w = Wander::new(7);
        let mut last = w.next(0.0);
        for _ in 0..200_000 {
            let v = w.next(10.0 / SR);
            assert!((-1.0..=1.0).contains(&v));
            assert!((v - last).abs() < 0.01);
            last = v;
        }
    }

    #[test]
    fn a_flare_comes_round_about_once_a_period() {
        let mut flare = Flare::new(3);
        // Count each time it rises through the middle.
        let (mut peaks, mut last) = (0, 0.0);
        for _ in 0..(SR * 90.0) as usize {
            let v = flare.next(9.0, SR);
            if last < 0.5 && v >= 0.5 {
                peaks += 1;
            }
            last = v;
        }
        assert!((7..=13).contains(&peaks), "{peaks}");
    }

    #[test]
    fn tremor_depth_is_what_was_asked() {
        let mut t = Tremor::default();
        let gains: Vec<f32> = (0..SR as usize).map(|_| t.next(0.13, SR)).collect();
        let mean = gains.iter().sum::<f32>() / gains.len() as f32;
        assert!((mean - 1.0).abs() < 0.01);
        assert!(gains.iter().all(|g| *g > 0.75 && *g < 1.25));
    }
}
