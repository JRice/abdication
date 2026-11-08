//! A small, allocation-free noise source (xorshift32).

pub struct Noise(u32);

impl Noise {
    pub fn new(seed: u32) -> Self {
        Self(seed.max(1))
    }

    /// Uniform in -1..1.
    #[inline(always)]
    pub fn next(&mut self) -> f32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 17;
        self.0 ^= self.0 << 5;
        (self.0 as i32) as f32 * (1.0 / 2_147_483_648.0)
    }
}
