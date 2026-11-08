//! The analog parts around the converters. They don't care what the clock is doing,
//! which is the whole trick: slow it down and the fixed filter stops hiding anything.

use std::f64::consts::PI;

#[derive(Clone, Copy, Default)]
struct Biquad {
    b: [f64; 3],
    a: [f64; 2],
    z: [f64; 2],
}

impl Biquad {
    #[inline(always)]
    fn process(&mut self, x: f64) -> f64 {
        let y = self.b[0] * x + self.z[0];
        self.z[0] = self.b[1] * x - self.a[0] * y + self.z[1];
        self.z[1] = self.b[2] * x - self.a[1] * y;
        y
    }
}

/// An 8-pole Chebyshev lowpass, the cliff measured in the recording.
#[derive(Clone)]
pub struct Cliff {
    sections: [Biquad; Cliff::SECTIONS],
}

impl Cliff {
    const SECTIONS: usize = 4;
    const RIPPLE_DB: f64 = 0.5;

    pub fn new(cutoff_hz: f32, sample_rate: f32) -> Self {
        let order = (Self::SECTIONS * 2) as f64;
        let epsilon = (10f64.powf(Self::RIPPLE_DB / 10.0) - 1.0).sqrt();
        let spread = (1.0 / epsilon).asinh() / order;
        let warped = (PI * cutoff_hz as f64 / sample_rate as f64).tan();
        let sections = std::array::from_fn(|k| {
            let angle = PI * (2 * k + 1) as f64 / (2.0 * order);
            let (re, im) = (-spread.sinh() * angle.sin() * warped, spread.cosh() * angle.cos() * warped);
            let (magnitude, twice_re) = (re * re + im * im, 2.0 * re);
            // Bilinear transform of 1 / (s^2 - 2 re s + |p|^2).
            let norm = 1.0 - twice_re + magnitude;
            let gain = magnitude / norm; // unity at DC
            Biquad {
                b: [gain, 2.0 * gain, gain],
                a: [2.0 * (magnitude - 1.0) / norm, (1.0 + twice_re + magnitude) / norm],
                z: [0.0; 2],
            }
        });
        Self { sections }
    }

    #[inline(always)]
    pub fn process(&mut self, x: f32) -> f32 {
        self.sections.iter_mut().fold(x as f64, |y, s| s.process(y)) as f32
    }
}

/// A first-order shelf between two time constants: hiss protection on the way into the
/// converter and its exact opposite on the way out.
#[derive(Clone)]
pub struct Shelf {
    b: [f32; 2],
    a: f32,
    x1: f32,
    y1: f32,
}

impl Shelf {
    const ZERO_S: f32 = 50e-6;
    const POLE_S: f32 = 15e-6;

    fn between(zero: f32, pole: f32, sample_rate: f32) -> Self {
        let k = 2.0 * sample_rate;
        let norm = 1.0 + k * pole;
        Self { b: [(1.0 + k * zero) / norm, (1.0 - k * zero) / norm], a: (1.0 - k * pole) / norm, x1: 0.0, y1: 0.0 }
    }

    pub fn emphasis(sample_rate: f32) -> Self {
        Self::between(Self::ZERO_S, Self::POLE_S, sample_rate)
    }

    pub fn de_emphasis(sample_rate: f32) -> Self {
        Self::between(Self::POLE_S, Self::ZERO_S, sample_rate)
    }

    #[inline(always)]
    pub fn process(&mut self, x: f32) -> f32 {
        let y = self.b[0] * x + self.b[1] * self.x1 - self.a * self.y1;
        self.x1 = x;
        self.y1 = y;
        y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gain_db(mut f: impl FnMut(f32) -> f32, hz: f32, rate: f32) -> f32 {
        let omega = 2.0 * PI * hz as f64 / rate as f64;
        let n = rate as usize / 4;
        let mut peak = 0.0f32;
        for i in 0..n {
            let y = f((i as f64 * omega).sin() as f32);
            if i > n / 2 {
                peak = peak.max(y.abs());
            }
        }
        20.0 * peak.log10()
    }

    #[test]
    fn the_cliff_passes_the_band_and_falls_off_hard() {
        let rate = 192_000.0;
        let at = |hz| {
            let mut c = Cliff::new(14_200.0, rate);
            gain_db(|x| c.process(x), hz, rate)
        };
        assert!(at(1_000.0).abs() < 0.8, "{}", at(1_000.0));
        assert!(at(10_000.0).abs() < 0.8, "{}", at(10_000.0));
        assert!(at(20_000.0) < -40.0, "{}", at(20_000.0));
        assert!(at(40_000.0) < -90.0, "{}", at(40_000.0));
    }

    #[test]
    fn emphasis_and_de_emphasis_cancel() {
        let rate = 192_000.0;
        for hz in [100.0, 3_000.0, 12_000.0] {
            let (mut up, mut down) = (Shelf::emphasis(rate), Shelf::de_emphasis(rate));
            let gain = gain_db(|x| down.process(up.process(x)), hz, rate);
            assert!(gain.abs() < 0.05, "{hz}: {gain}");
        }
    }

    #[test]
    fn emphasis_boosts_the_top() {
        let rate = 192_000.0;
        let mut up = Shelf::emphasis(rate);
        let gain = gain_db(|x| up.process(x), 12_000.0, rate);
        assert!(gain > 6.0 && gain < 10.5, "{gain}");
    }
}
