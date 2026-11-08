//! The decoder's far end: a buzz or a hiss through a lattice filter. Nothing here knows
//! about language; it just does what the frames say.

use super::codec::{Frame, ORDER};
use super::random::Rng;

/// Samples between filter updates.
const FILTER_STEP: usize = 16;
const JITTER: f32 = 0.006;
const SOFT_LIMIT: f32 = 1.4;

/// A frame part-way to the next, ready to use.
#[derive(Clone, Copy)]
pub struct Settings {
    reflections: [f32; ORDER],
    /// Keeps the output at the frame's level whatever the filter is doing.
    residual: f32,
    gain: f32,
    voicing: f32,
    rough: f32,
    f0: f32,
}

impl Settings {
    pub fn between(a: &Frame, b: &Frame, along: f32) -> Self {
        let reflections: [f32; ORDER] = std::array::from_fn(|i| ((a.ratios[i] + (b.ratios[i] - a.ratios[i]) * along) * 0.5).tanh());
        let residual = reflections.iter().map(|k| 1.0 - k * k).product::<f32>().max(1e-6).sqrt();
        Self {
            reflections,
            residual,
            gain: a.gain + (b.gain - a.gain) * along,
            voicing: a.voicing + (b.voicing - a.voicing) * along,
            rough: a.rough + (b.rough - a.rough) * along,
            f0: a.f0 * (b.f0 / a.f0.max(1.0)).powf(along),
        }
    }

    pub const fn step() -> usize {
        FILTER_STEP
    }
}

pub struct Voice {
    backward: [f32; ORDER + 1],
    until_pulse: f32,
    /// Vocal folds out of step alternate long and short, strong and weak.
    alternate: bool,
    rng: Rng,
}

impl Voice {
    pub fn new(seed: u32) -> Self {
        Self { backward: [0.0; ORDER + 1], until_pulse: 0.0, alternate: false, rng: Rng::new(seed) }
    }

    #[inline(always)]
    fn buzz(&mut self, settings: &Settings, pitch_ratio: f32, sample_rate: f32) -> f32 {
        self.until_pulse -= 1.0;
        if self.until_pulse > 0.0 {
            return 0.0;
        }
        self.alternate = !self.alternate;
        let rough = settings.rough;
        let side = if self.alternate { 1.0 } else { -1.0 };
        let wobble = 1.0 + side * 0.2 * rough + self.rng.signed() * (JITTER + 0.05 * rough);
        let period = (sample_rate / (settings.f0 * pitch_ratio).max(10.0)).max(2.0);
        self.until_pulse += period * wobble;
        // Unit power over the period.
        period.sqrt() * (1.0 + side * 0.6 * rough)
    }

    /// One sample of the voice.
    #[inline(always)]
    pub fn process(&mut self, settings: &Settings, pitch_ratio: f32, sample_rate: f32) -> f32 {
        let buzz = self.buzz(settings, pitch_ratio, sample_rate);
        let noise = self.rng.signed() * 1.732;
        let excitation = settings.voicing.sqrt() * buzz + (1.0 - settings.voicing).sqrt() * noise;

        let mut forward = excitation * settings.gain * settings.residual;
        for i in (0..ORDER).rev() {
            forward -= settings.reflections[i] * self.backward[i];
            self.backward[i + 1] = self.backward[i] + settings.reflections[i] * forward;
        }
        self.backward[0] = forward;
        // Some patterns on a bad line get hot. The cone gives out before anything else.
        SOFT_LIMIT * (forward / SOFT_LIMIT).tanh()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dsp::codec::{decode, encode};
    use crate::dsp::speaker::Target;

    const RATE: f32 = 16_000.0;

    fn render(target: &Target, seconds: f32) -> Vec<f32> {
        let frame = decode(&encode(target, RATE));
        let settings = Settings::between(&frame, &frame, 0.0);
        let mut voice = Voice::new(3);
        (0..(seconds * RATE) as usize).map(|_| voice.process(&settings, 1.0, RATE)).collect()
    }

    fn power_at(signal: &[f32], hz: f32) -> f32 {
        let omega = 2.0 * std::f32::consts::PI * hz / RATE;
        let (mut re, mut im) = (0.0, 0.0);
        for (n, x) in signal.iter().enumerate() {
            re += x * (omega * n as f32).cos();
            im += x * (omega * n as f32).sin();
        }
        (re * re + im * im) / signal.len() as f32
    }

    fn target(f1: f32, f2: f32) -> Target {
        Target {
            formants: [f1, f2, 2500.0, 3500.0, 4500.0, 5500.0],
            bandwidths: [60.0, 80.0, 120.0, 175.0, 280.0, 400.0],
            tilt: 0.9,
            f0: 100.0,
            voicing: 1.0,
            gain: 0.2,
            rough: 0.0,
        }
    }

    #[test]
    fn the_filter_puts_the_energy_on_the_formants() {
        // Harmonics of 100 Hz, near and away from F1.
        let open = render(&target(700.0, 1300.0), 1.0);
        let close = render(&target(300.0, 2200.0), 1.0);
        assert!(power_at(&open, 700.0) > power_at(&close, 700.0) * 3.0);
        assert!(power_at(&close, 2200.0) > power_at(&open, 2200.0) * 3.0);
    }

    #[test]
    fn the_level_follows_the_frame_not_the_filter() {
        let rms = |t: Target| {
            let s = render(&t, 1.0);
            (s.iter().map(|x| x * x).sum::<f32>() / s.len() as f32).sqrt()
        };
        let (a, b) = (rms(target(700.0, 1300.0)), rms(target(300.0, 2200.0)));
        assert!((a / b).log2().abs() < 1.0, "{a} {b}");
        assert!((a / 0.2).log2().abs() < 1.0, "{a}");
    }

    #[test]
    fn roughness_adds_a_subharmonic() {
        let smooth = render(&target(700.0, 1300.0), 1.0);
        let rough = render(&Target { rough: 1.0, ..target(700.0, 1300.0) }, 1.0);
        assert!(power_at(&rough, 50.0) > power_at(&smooth, 50.0) * 2.0);
    }
}
