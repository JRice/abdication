//! The output's safety limiter: instant attack, exponential release, both channels
//! together. Nothing leaves the plugin above the ceiling, and non-finite samples
//! never get as far as the envelope.

const CEILING_DB: f32 = -0.5;
const RELEASE_S: f32 = 0.05;

pub struct Limiter {
    ceiling: f32,
    release: f32,
    envelope: f32,
}

impl Limiter {
    pub fn new(sample_rate: f32) -> Self {
        Self { ceiling: 10f32.powf(CEILING_DB / 20.0), release: (-1.0 / (RELEASE_S * sample_rate)).exp(), envelope: 0.0 }
    }

    #[inline]
    pub fn process(&mut self, input: [f32; 2]) -> [f32; 2] {
        let [l, r] = input.map(|x| if x.is_finite() { x } else { 0.0 });
        self.envelope = l.abs().max(r.abs()).max(self.envelope * self.release);
        let gain = self.gain();
        [l * gain, r * gain]
    }

    #[inline(always)]
    fn gain(&self) -> f32 {
        if self.envelope > self.ceiling {
            self.ceiling / self.envelope
        } else {
            1.0
        }
    }

    /// How far the signal is being pulled down at the moment, in dB.
    #[inline]
    pub fn reduction_db(&self) -> f32 {
        -20.0 * self.gain().log10()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SR: f32 = 48_000.0;

    fn sine(n: usize, amp: f32) -> f32 {
        amp * (n as f32 * 0.05).sin()
    }

    #[test]
    fn never_exceeds_the_ceiling() {
        let mut limiter = Limiter::new(SR);
        let ceiling = 10f32.powf(CEILING_DB / 20.0);
        for n in 0..48_000 {
            let amp = 0.2 + 8.0 * (n % 9_000) as f32 / 9_000.0;
            let [l, r] = limiter.process([sine(n, amp), sine(n + 7, amp * 0.5)]);
            assert!(l.abs() <= ceiling * 1.000_01 && r.abs() <= ceiling * 1.000_01, "{l} {r} at {n}");
        }
    }

    #[test]
    fn leaves_quiet_signals_alone() {
        let mut limiter = Limiter::new(SR);
        for n in 0..4_800 {
            let x = sine(n, 0.5);
            assert_eq!(limiter.process([x, -x]), [x, -x]);
            assert_eq!(limiter.reduction_db(), 0.0);
        }
    }

    #[test]
    fn reports_how_deep_it_limits_and_recovers() {
        let mut limiter = Limiter::new(SR);
        limiter.process([4.0, 0.0]); // 4.0 is 12 dB over 1.0, so 12.5 dB over the ceiling
        assert!((limiter.reduction_db() - 12.54).abs() < 0.05, "{}", limiter.reduction_db());
        for _ in 0..48_000 {
            limiter.process([0.0, 0.0]);
        }
        assert_eq!(limiter.reduction_db(), 0.0);
    }

    #[test]
    fn survives_bad_samples() {
        let mut limiter = Limiter::new(SR);
        let [l, r] = limiter.process([f32::NAN, f32::INFINITY]);
        assert_eq!([l, r], [0.0, 0.0]);
        assert_eq!(limiter.process([0.25, 0.25]), [0.25, 0.25]);
    }
}
