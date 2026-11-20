//! Oscillator III: a sine carrier into a sine wavefolder. Fold 0 is close to a plain
//! sine; Fold 1 drives it 32 times over, folding it back on itself again and again. The
//! drive follows Fold squared, so the first version's whole range (to 8×) sits in the
//! lower half of the knob. Run at four times the rate and filtered down, so the folds'
//! harmonics alias far less.

use std::f32::consts::{FRAC_PI_2, TAU};

const OVERSAMPLE: usize = 4;
const MAX_DRIVE: f32 = 32.0;

/// A biquad (transposed direct form II).
#[derive(Clone, Copy, Default)]
struct Biquad {
    b: [f32; 3],
    a: [f32; 2],
    z: [f32; 2],
}

impl Biquad {
    fn lowpass(fc: f32, rate: f32, q: f32) -> Self {
        let w = TAU * fc / rate;
        let (s, c) = w.sin_cos();
        let alpha = s / (2.0 * q);
        let a0 = 1.0 + alpha;
        let b1 = (1.0 - c) / a0;
        Self { b: [b1 / 2.0, b1, b1 / 2.0], a: [-2.0 * c / a0, (1.0 - alpha) / a0], z: [0.0; 2] }
    }

    #[inline(always)]
    fn tick(&mut self, x: f32) -> f32 {
        let y = self.b[0] * x + self.z[0];
        self.z[0] = self.b[1] * x - self.a[0] * y + self.z[1];
        self.z[1] = self.b[2] * x - self.a[1] * y;
        y
    }
}

pub struct Folder {
    phase: f32,
    /// A 4th-order Butterworth lowpass at the base rate's 0.42, run at the high rate.
    down: [Biquad; 2],
}

impl Folder {
    pub fn new(rate: f32) -> Self {
        let high = rate * OVERSAMPLE as f32;
        let fc = 0.42 * rate;
        Self { phase: 0.0, down: [Biquad::lowpass(fc, high, 0.541_196), Biquad::lowpass(fc, high, 1.306_563)] }
    }

    /// One sample at `step` cycles per sample, folded by `fold` (0..1).
    #[inline]
    pub fn tick(&mut self, step: f32, fold: f32) -> f32 {
        let f = fold.clamp(0.0, 1.0);
        let drive = 1.0 + (MAX_DRIVE - 1.0) * f * f;
        let sub = step / OVERSAMPLE as f32;
        let mut y = 0.0;
        for _ in 0..OVERSAMPLE {
            self.phase += sub;
            if self.phase >= 1.0 {
                self.phase -= 1.0;
            }
            let x = (FRAC_PI_2 * drive * (TAU * self.phase).sin()).sin();
            let first = self.down[0].tick(x);
            y = self.down[1].tick(first);
        }
        y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn folds_stay_bounded_and_brighten() {
        let rate = 48_000.0;
        let step = 65.4 / rate;
        let crossings = |fold: f32| {
            let mut f = Folder::new(rate);
            let x: Vec<f32> = (0..48_000).map(|_| f.tick(step, fold)).collect();
            assert!(x.iter().all(|v| v.abs() < 1.2));
            x.windows(2).filter(|w| w[0] < 0.0 && w[1] >= 0.0).count()
        };
        let (plain, half, folded) = (crossings(0.0), crossings(0.5), crossings(1.0));
        assert!((60..=70).contains(&plain), "{plain}");
        assert!(half > 3 * plain, "{half}");
        // Full fold crosses zero about four times as often as the old full (8×) did.
        assert!(folded > 3 * half, "{folded} vs {half}");
    }
}
