//! Delay lines and the allpass built on one. Distances are in samples and may be
//! fractional: the tank's lines move (Size, modulation), so every read interpolates.

/// A ring buffer read at any distance back in time. `read(d)` is the sample written
/// `d` writes ago, so read-then-write gives a delay of exactly `d`.
pub struct Delay {
    buf: Vec<f32>,
    mask: usize,
    pos: usize,
}

impl Delay {
    /// Room for `max_samples` of delay, plus the interpolator's neighbours.
    pub fn new(max_samples: f32) -> Self {
        let len = (max_samples.ceil() as usize + 4).next_power_of_two();
        Self { buf: vec![0.0; len], mask: len - 1, pos: 0 }
    }

    pub fn clear(&mut self) {
        self.buf.fill(0.0);
    }

    #[inline]
    pub fn write(&mut self, x: f32) {
        self.buf[self.pos] = x;
        self.pos = (self.pos + 1) & self.mask;
    }

    #[inline(always)]
    fn at(&self, d: usize) -> f32 {
        self.buf[self.pos.wrapping_sub(d) & self.mask]
    }

    #[inline(always)]
    fn longest(&self) -> f32 {
        (self.mask - 3) as f32
    }

    /// Linear interpolation: for output taps, which are read once and leave.
    #[inline]
    pub fn tap(&self, d: f32) -> f32 {
        let d = d.clamp(1.0, self.longest());
        let i = d as usize;
        let a = self.at(i);
        a + (d - i as f32) * (self.at(i + 1) - a)
    }

    /// Cubic (Hermite) interpolation: for reads inside the loop, where linear
    /// interpolation's dulling would compound on every trip round.
    #[inline]
    pub fn read(&self, d: f32) -> f32 {
        let d = d.clamp(2.0, self.longest());
        let i = d as usize;
        let t = d - i as f32;
        let (xm1, x0, x1, x2) = (self.at(i - 1), self.at(i), self.at(i + 1), self.at(i + 2));
        let c1 = 0.5 * (x1 - xm1);
        let c2 = xm1 - 2.5 * x0 + 2.0 * x1 - 0.5 * x2;
        let c3 = 0.5 * (x2 - xm1) + 1.5 * (x0 - x1);
        ((c3 * t + c2) * t + c1) * t + x0
    }
}

/// A Schroeder allpass in lattice form: flat magnitude, smeared phase. Its line is
/// public to the output taps, which read the reverb from inside it.
pub struct Allpass {
    line: Delay,
}

impl Allpass {
    pub fn new(max_samples: f32) -> Self {
        Self { line: Delay::new(max_samples) }
    }

    pub fn clear(&mut self) {
        self.line.clear();
    }

    pub fn line(&self) -> &Delay {
        &self.line
    }

    #[inline]
    pub fn process(&mut self, x: f32, length: f32, gain: f32) -> f32 {
        let delayed = self.line.read(length);
        let w = x + gain * delayed;
        self.line.write(w);
        delayed - gain * w
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delays_by_exactly_the_distance_asked() {
        let mut line = Delay::new(64.0);
        let mut out = Vec::new();
        for n in 0..40 {
            out.push((line.read(10.0), line.tap(17.0)));
            line.write(if n == 0 { 1.0 } else { 0.0 });
        }
        assert_eq!(out.iter().position(|o| o.0 == 1.0), Some(10));
        assert_eq!(out.iter().position(|o| o.1 == 1.0), Some(17));
    }

    #[test]
    fn interpolates_between_samples() {
        let mut line = Delay::new(16.0);
        for n in 0..16 {
            line.write(n as f32); // a ramp: the value d back is 16 - d
        }
        assert!((line.read(4.5) - 11.5).abs() < 1e-5);
        assert!((line.tap(4.25) - 11.75).abs() < 1e-5);
    }

    #[test]
    fn the_allpass_keeps_energy() {
        let mut ap = Allpass::new(200.0);
        let (mut energy, mut x) = (0.0, 1.0);
        for _ in 0..20_000 {
            let y = ap.process(x, 113.0, 0.7);
            energy += y * y;
            x = 0.0;
        }
        assert!((energy - 1.0).abs() < 1e-3, "{energy}");
    }
}
