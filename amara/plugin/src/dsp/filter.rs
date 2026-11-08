//! The steep lowpass that follows the codec back up to the host's rate.

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

/// An 8-pole Chebyshev lowpass: the band edge of the line.
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
