//! A small, fast generator of unimportant decisions, seeded so a language is repeatable.

#[derive(Clone)]
pub struct Rng(u32);

impl Rng {
    pub fn new(seed: u32) -> Self {
        // A multiply first, so nearby seeds start far apart.
        Self(seed.wrapping_mul(0x9E37_79B1) | 1)
    }

    #[inline(always)]
    pub fn bits(&mut self) -> u32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 17;
        self.0 ^= self.0 << 5;
        self.0
    }

    /// 0..1
    #[inline(always)]
    pub fn unit(&mut self) -> f32 {
        self.bits() as f32 / (u32::MAX as f32 + 1.0)
    }

    /// -1..1
    #[inline(always)]
    pub fn signed(&mut self) -> f32 {
        self.unit() * 2.0 - 1.0
    }

    #[inline(always)]
    pub fn chance(&mut self, probability: f32) -> bool {
        self.unit() < probability
    }

    pub fn range(&mut self, low: f32, high: f32) -> f32 {
        low + (high - low) * self.unit()
    }

    pub fn below(&mut self, count: usize) -> usize {
        (self.unit() * count as f32) as usize % count.max(1)
    }

    /// An index into `weights`, in proportion to them.
    pub fn weighted(&mut self, weights: &[f32]) -> usize {
        let mut left = self.unit() * weights.iter().sum::<f32>();
        for (i, weight) in weights.iter().enumerate() {
            left -= weight;
            if left < 0.0 {
                return i;
            }
        }
        weights.len() - 1
    }

    /// Roughly normal, unit variance: enough for noise and fading.
    #[inline(always)]
    pub fn gaussian(&mut self) -> f32 {
        // Four uniforms have variance 1/3 between them, once centred.
        (self.unit() + self.unit() + self.unit() + self.unit() - 2.0) * 1.732
    }

    /// Exponentially distributed, with this mean.
    pub fn exponential(&mut self, mean: f32) -> f32 {
        -mean * (1.0 - self.unit()).max(1e-6).ln()
    }
}
