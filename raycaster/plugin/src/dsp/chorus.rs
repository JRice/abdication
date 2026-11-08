//! A chorus on each oscillator of each voice, before the drive and filter: a 14 ms delay
//! swept by a 1 Hz sine, the right channel a quarter cycle ahead of the left, mixed half
//! and half with the dry sound. Depth is how far the delay swings, as a share of the 14 ms.
//! At 0 it is a true bypass; the first 5 % of Depth fades the mix in.

const DELAY_S: f32 = 0.014;
const RATE_HZ: f32 = 1.0;
const SPREAD: f32 = 0.25;
const MIX: f32 = 0.5;
const FADE_IN: f32 = 0.05;
/// Room for 28 ms at up to 96 kHz.
const SIZE: usize = 4096;

pub struct Chorus {
    lines: Box<[[f32; SIZE]; 2]>,
    pos: usize,
    phase: f32,
    depth: f32,
}

impl Chorus {
    pub fn new(phase: f32) -> Self {
        Self { lines: Box::new([[0.0; SIZE]; 2]), pos: 0, phase, depth: 0.0 }
    }

    pub fn clear(&mut self) {
        self.lines.iter_mut().for_each(|l| l.fill(0.0));
    }

    /// `delay` samples ago (4-point Hermite), from line `ch`.
    #[inline]
    fn read(&self, ch: usize, delay: f32) -> f32 {
        let delay = delay.clamp(1.0, (SIZE - 3) as f32);
        let whole = delay.floor();
        let f = delay - whole;
        let d = whole as usize;
        let newest = self.pos + SIZE - 1;
        let at = |back: usize| self.lines[ch][(newest - back) % SIZE];
        let (xm1, x0, x1, x2) = (at(d - 1), at(d), at(d + 1), at(d + 2));
        let c1 = 0.5 * (x1 - xm1);
        let c2 = xm1 - 2.5 * x0 + 2.0 * x1 - 0.5 * x2;
        let c3 = 0.5 * (x2 - xm1) + 1.5 * (x0 - x1);
        ((c3 * f + c2) * f + c1) * f + x0
    }

    /// Choruses `left` and `right` in place, with Depth moving to `depth` over the block.
    pub fn process(&mut self, left: &mut [f32], right: &mut [f32], depth: f32, rate: f32) {
        let n = left.len().min(right.len());
        let start = self.depth;
        self.depth = depth.clamp(0.0, 1.0);
        let base = DELAY_S * rate;
        let step = RATE_HZ / rate;
        for i in 0..n {
            self.lines[0][self.pos] = left[i];
            self.lines[1][self.pos] = right[i];
            self.pos = (self.pos + 1) % SIZE;
            self.phase = (self.phase + step).fract();
            let depth = start + (self.depth - start) * (i as f32 + 1.0) / n as f32;
            if depth <= 0.0 {
                continue;
            }
            let mix = MIX * (depth / FADE_IN).min(1.0);
            for (ch, x) in [&mut left[i], &mut right[i]].into_iter().enumerate() {
                let swing = super::geometry::turn_sin_cos(self.phase + SPREAD * ch as f32).0;
                let wet = self.read(ch, base * (1.0 + depth * swing));
                *x += (wet - *x) * mix;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn off_is_a_bypass() {
        let mut c = Chorus::new(0.3);
        let mut l: Vec<f32> = (0..256).map(|i| (i as f32 * 0.1).sin()).collect();
        let mut r = l.clone();
        let dry = l.clone();
        c.process(&mut l, &mut r, 0.0, 48_000.0);
        assert_eq!(l, dry);
        assert_eq!(r, dry);
    }

    #[test]
    fn on_it_mixes_in_a_moving_copy_and_stays_bounded() {
        let mut c = Chorus::new(0.0);
        let rate = 48_000.0;
        let n = 48_000;
        let mut l: Vec<f32> = (0..n).map(|i| (2.0 * std::f32::consts::PI * 220.0 * i as f32 / rate).sin()).collect();
        let mut r = l.clone();
        let dry = l.clone();
        for (a, b) in l.chunks_mut(32).zip(r.chunks_mut(32)) {
            c.process(a, b, 0.5, rate);
        }
        assert!(l.iter().chain(&r).all(|v| v.is_finite() && v.abs() <= 1.01));
        let differs = l.iter().zip(&dry).skip(2_000).map(|(a, b)| (a - b).abs()).fold(0.0, f32::max);
        assert!(differs > 0.1, "{differs}");
        let sides = l.iter().zip(&r).skip(2_000).map(|(a, b)| (a - b).abs()).fold(0.0, f32::max);
        assert!(sides > 0.01, "the channels should move apart: {sides}");
    }
}
