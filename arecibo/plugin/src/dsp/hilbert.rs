//! Splits a signal into two copies 90 degrees apart, which is what lets a receiver move
//! it up or down in frequency. Olli Niemitalo's allpass pair: flat to a tenth of a degree
//! over nearly the whole band.

const CHAIN_A: [f32; 4] = [0.692_387_8, 0.936_065_4, 0.988_229_5, 0.998_748_8];
const CHAIN_B: [f32; 4] = [0.402_192_1, 0.856_171_1, 0.972_291, 0.995_288_5];

#[derive(Clone, Copy, Default)]
struct Stage {
    c: f32,
    x: [f32; 2],
    y: [f32; 2],
}

impl Stage {
    #[inline(always)]
    fn process(&mut self, input: f32) -> f32 {
        let output = self.c * (input + self.y[1]) - self.x[1];
        self.x = [input, self.x[0]];
        self.y = [output, self.y[0]];
        output
    }
}

#[derive(Clone, Copy, Default)]
pub struct Hilbert {
    a: [Stage; 4],
    b: [Stage; 4],
    delayed: f32,
}

impl Hilbert {
    pub fn new() -> Self {
        let chain = |coefficients: [f32; 4]| coefficients.map(|k| Stage { c: k * k, ..Stage::default() });
        Self { a: chain(CHAIN_A), b: chain(CHAIN_B), delayed: 0.0 }
    }

    /// In-phase and quadrature.
    #[inline(always)]
    pub fn process(&mut self, x: f32) -> (f32, f32) {
        let a = self.a.iter_mut().fold(x, |y, stage| stage.process(y));
        let b = self.b.iter_mut().fold(x, |y, stage| stage.process(y));
        let re = self.delayed;
        self.delayed = a;
        // This way round, a positive shift moves things up.
        (re, -b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::TAU;

    const RATE: f64 = 96_000.0;

    /// Shifts a tone with this pair and measures the power that lands at `listen`.
    fn power_at(tone: f64, shift: f64, listen: f64) -> f64 {
        let mut hilbert = Hilbert::new();
        let (mut re, mut im) = (0.0, 0.0);
        let n = RATE as usize * 2;
        for i in 0..n {
            let t = i as f64 / RATE;
            let (a, b) = hilbert.process((TAU * tone * t).sin() as f32);
            let out = a as f64 * (TAU * shift * t).cos() - b as f64 * (TAU * shift * t).sin();
            if i > n / 2 {
                re += out * (TAU * listen * t).cos();
                im += out * (TAU * listen * t).sin();
            }
        }
        (re * re + im * im) / (n as f64 / 2.0).powi(2)
    }

    #[test]
    fn a_tone_moves_by_exactly_the_shift_and_leaves_almost_no_mirror_image() {
        for tone in [80.0, 440.0, 3_000.0, 12_000.0] {
            let wanted = power_at(tone, 37.0, tone + 37.0);
            let mirror = power_at(tone, 37.0, tone - 37.0);
            let original = power_at(tone, 37.0, tone);
            assert!(wanted > 0.2, "{tone}: {wanted}");
            assert!(mirror < wanted * 1e-3, "{tone}: {mirror} vs {wanted}");
            assert!(original < wanted * 1e-3, "{tone}: {original} vs {wanted}");
        }
    }
}
