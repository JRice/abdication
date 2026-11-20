//! The small filters: a one-pole for bandwidth and damping, a DC blocker for the
//! tank, and a state-variable filter for Color.

use std::f32::consts::{PI, TAU};

/// The coefficient of a one-pole lowpass at `hz`; 0 passes everything.
pub fn one_pole_coefficient(hz: f32, sample_rate: f32) -> f32 {
    (-TAU * hz.min(0.49 * sample_rate) / sample_rate).exp()
}

#[derive(Default)]
pub struct OnePole {
    state: f32,
}

impl OnePole {
    #[inline]
    pub fn process(&mut self, x: f32, coefficient: f32) -> f32 {
        self.state = x + coefficient * (self.state - x);
        self.state
    }

    pub fn clear(&mut self) {
        self.state = 0.0;
    }
}

/// Takes out what's below a few hertz, so nothing can creep up the tank forever.
pub struct DcBlock {
    pole: f32,
    x1: f32,
    y1: f32,
}

impl DcBlock {
    pub fn new(hz: f32, sample_rate: f32) -> Self {
        Self { pole: 1.0 - TAU * hz / sample_rate, x1: 0.0, y1: 0.0 }
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        self.y1 = x - self.x1 + self.pole * self.y1;
        self.x1 = x;
        self.y1
    }

    pub fn clear(&mut self) {
        (self.x1, self.y1) = (0.0, 0.0);
    }
}

/// Coefficients for a topology-preserving state-variable filter.
#[derive(Clone, Copy)]
pub struct SvfShape {
    k: f32,
    a1: f32,
    a2: f32,
    a3: f32,
}

impl SvfShape {
    const BUTTERWORTH_Q: f32 = std::f32::consts::FRAC_1_SQRT_2;

    pub fn new(hz: f32, sample_rate: f32) -> Self {
        Self::resonant(hz, Self::BUTTERWORTH_Q, sample_rate)
    }

    pub fn resonant(hz: f32, q: f32, sample_rate: f32) -> Self {
        let g = (PI * hz.clamp(10.0, 0.45 * sample_rate) / sample_rate).tan();
        let k = 1.0 / q;
        let a1 = 1.0 / (1.0 + g * (g + k));
        Self { k, a1, a2: g * a1, a3: g * g * a1 }
    }
}

#[derive(Default)]
pub struct Svf {
    ic1: f32,
    ic2: f32,
}

impl Svf {
    /// One step; returns the band and low outputs.
    #[inline(always)]
    fn step(&mut self, x: f32, s: &SvfShape) -> (f32, f32) {
        let v3 = x - self.ic2;
        let v1 = s.a1 * self.ic1 + s.a2 * v3;
        let v2 = self.ic2 + s.a2 * self.ic1 + s.a3 * v3;
        self.ic1 = 2.0 * v1 - self.ic1;
        self.ic2 = 2.0 * v2 - self.ic2;
        (v1, v2)
    }

    /// Returns (lowpass, highpass).
    #[inline]
    pub fn process(&mut self, x: f32, s: &SvfShape) -> (f32, f32) {
        let (band, low) = self.step(x, s);
        (low, x - s.k * band - low)
    }

    /// A bandpass with unity gain at its centre.
    #[inline]
    pub fn band(&mut self, x: f32, s: &SvfShape) -> f32 {
        s.k * self.step(x, s).0
    }

    pub fn clear(&mut self) {
        (self.ic1, self.ic2) = (0.0, 0.0);
    }
}

/// Color on one channel: a highpass, then a lowpass.
#[derive(Default)]
pub struct Tone {
    low_cut: Svf,
    high_cut: Svf,
}

impl Tone {
    #[inline]
    pub fn process(&mut self, x: f32, low_cut: &SvfShape, high_cut: &SvfShape) -> f32 {
        let (_, x) = self.low_cut.process(x, low_cut);
        self.high_cut.process(x, high_cut).0
    }

    pub fn clear(&mut self) {
        self.low_cut.clear();
        self.high_cut.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SR: f32 = 48_000.0;

    fn gain_at(hz: f32, mut f: impl FnMut(f32) -> f32) -> f32 {
        let mut peak: f32 = 0.0;
        for n in 0..48_000 {
            let y = f((TAU * hz * n as f32 / SR).sin());
            if n > 24_000 {
                peak = peak.max(y.abs());
            }
        }
        peak
    }

    #[test]
    fn the_svf_cuts_where_asked() {
        let shape = SvfShape::new(1_000.0, SR);
        let mut svf = Svf::default();
        assert!((gain_at(1_000.0, |x| svf.process(x, &shape).0) - 0.707).abs() < 0.02);
        let mut svf = Svf::default();
        assert!(gain_at(100.0, |x| svf.process(x, &shape).0) > 0.99);
        let mut svf = Svf::default();
        assert!(gain_at(100.0, |x| svf.process(x, &shape).1) < 0.02);
    }

    #[test]
    fn the_band_peaks_at_unity() {
        let shape = SvfShape::resonant(1_000.0, 8.0, SR);
        let mut svf = Svf::default();
        assert!((gain_at(1_000.0, |x| svf.band(x, &shape)) - 1.0).abs() < 0.02);
        let mut svf = Svf::default();
        assert!(gain_at(2_000.0, |x| svf.band(x, &shape)) < 0.1);
    }

    #[test]
    fn the_dc_block_passes_audio() {
        let mut dc = DcBlock::new(5.0, SR);
        assert!(gain_at(100.0, |x| dc.process(x + 0.5)) < 1.01);
        let mut dc = DcBlock::new(5.0, SR);
        assert!(gain_at(100.0, |x| dc.process(x)) > 0.99);
    }
}
