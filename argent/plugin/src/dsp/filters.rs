//! The small filters everything here is built from.

use std::f32::consts::PI;

/// A state-variable filter, stable under modulation.
#[derive(Clone, Copy, Default)]
pub struct Svf {
    ic1: f32,
    ic2: f32,
    g: f32,
    k: f32,
    a1: f32,
    a2: f32,
}

pub struct SvfOut {
    pub low: f32,
    pub band: f32,
    pub high: f32,
}

impl Svf {
    /// `q` above 0.5 peaks.
    pub fn set(&mut self, hz: f32, q: f32, rate: f32) {
        self.g = (PI * hz.clamp(1.0, 0.47 * rate) / rate).tan();
        self.k = 1.0 / q;
        self.a1 = 1.0 / (1.0 + self.g * (self.g + self.k));
        self.a2 = self.g * self.a1;
    }

    #[inline(always)]
    pub fn process(&mut self, x: f32) -> SvfOut {
        let v3 = x - self.ic2;
        let v1 = self.a1 * self.ic1 + self.a2 * v3;
        let v2 = self.ic2 + self.g * v1;
        self.ic1 = 2.0 * v1 - self.ic1;
        self.ic2 = 2.0 * v2 - self.ic2;
        SvfOut { low: v2, band: v1, high: x - self.k * v1 - v2 }
    }

    pub fn clear(&mut self) {
        (self.ic1, self.ic2) = (0.0, 0.0);
    }
}

/// How far a one-pole moves per sample to have this time constant.
pub fn rate_for(seconds: f32, sample_rate: f32) -> f32 {
    1.0 - (-1.0 / (seconds.max(1e-6) * sample_rate)).exp()
}

/// Removes what never changes, which a loop would otherwise add up.
#[derive(Clone, Copy, Default)]
pub struct DcBlock {
    x1: f32,
    y1: f32,
}

impl DcBlock {
    const POLE: f32 = 0.9995;

    #[inline(always)]
    pub fn process(&mut self, x: f32) -> f32 {
        let y = x - self.x1 + Self::POLE * self.y1;
        self.x1 = x;
        self.y1 = y;
        y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn peak_gain(mut f: impl FnMut(f32) -> f32, hz: f32, rate: f32) -> f32 {
        let omega = 2.0 * std::f64::consts::PI * hz as f64 / rate as f64;
        let n = rate as usize / 4;
        (0..n)
            .map(|i| f((i as f64 * omega).sin() as f32).abs())
            .skip(n / 2)
            .fold(0.0, f32::max)
    }

    #[test]
    fn the_filter_peaks_where_it_is_told() {
        let rate = 96_000.0;
        let at = |hz: f32| {
            let mut svf = Svf::default();
            svf.set(1_000.0, 8.0, rate);
            peak_gain(|x| svf.process(x).band, hz, rate)
        };
        assert!(at(1_000.0) > at(500.0) * 3.0 && at(1_000.0) > at(2_000.0) * 3.0);
    }

    #[test]
    fn dc_goes_and_audio_stays() {
        let mut block = DcBlock::default();
        let mut last = 0.0;
        for _ in 0..200_000 {
            last = block.process(0.5);
        }
        assert!(last.abs() < 1e-3);
        assert!((peak_gain(|x| block.process(x), 1_000.0, 96_000.0) - 1.0).abs() < 0.01);
    }
}
