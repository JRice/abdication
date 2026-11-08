//! The small linear filters the loop is built from.

use std::f32::consts::PI;

/// A state-variable filter (Simper's TPT form): stable under fast modulation.
#[derive(Default, Clone)]
pub struct Svf {
    ic1: f32,
    ic2: f32,
}

/// All of a filter's outputs. `band` has unity gain at the cutoff.
#[derive(Clone, Copy)]
pub struct SvfOutputs {
    pub low: f32,
    pub band: f32,
    pub high: f32,
}

impl Svf {
    #[inline(always)]
    pub fn process(&mut self, x: f32, c: &SvfCoefficients) -> SvfOutputs {
        let v3 = x - self.ic2;
        let v1 = c.a1 * self.ic1 + c.a2 * v3;
        let v2 = self.ic2 + c.a2 * self.ic1 + c.a3 * v3;
        self.ic1 = 2.0 * v1 - self.ic1;
        self.ic2 = 2.0 * v2 - self.ic2;
        let band = c.k * v1;
        SvfOutputs { low: v2, band, high: x - band - v2 }
    }

    #[inline(always)]
    pub fn lowpass(&mut self, x: f32, c: &SvfCoefficients) -> f32 {
        self.process(x, c).low
    }
}

#[derive(Clone, Copy)]
pub struct SvfCoefficients {
    a1: f32,
    a2: f32,
    a3: f32,
    k: f32,
}

impl SvfCoefficients {
    #[inline(always)]
    pub fn new(hz: f32, q: f32, sample_rate: f32) -> Self {
        let g = (PI * (hz / sample_rate).min(0.49)).tan();
        let k = 1.0 / q;
        let a1 = 1.0 / (1.0 + g * (g + k));
        let a2 = g * a1;
        Self { a1, a2, a3: g * a2, k }
    }
}

/// A one-pole lowpass, for control signals.
#[derive(Clone, Copy)]
pub struct OnePole {
    state: f32,
    keep: f32,
}

impl OnePole {
    pub fn new(hz: f32, sample_rate: f32) -> Self {
        Self { state: 0.0, keep: (-2.0 * PI * hz / sample_rate).exp() }
    }

    #[inline(always)]
    pub fn process(&mut self, x: f32) -> f32 {
        self.state = x + (self.state - x) * self.keep;
        self.state
    }
}

/// Follows a level: quick up, slow down.
#[derive(Clone, Copy)]
pub struct Follower {
    state: f32,
    attack: f32,
    release: f32,
}

impl Follower {
    pub fn new(attack_ms: f32, release_ms: f32, sample_rate: f32) -> Self {
        let keep = |ms: f32| (-1000.0 / (ms * sample_rate)).exp();
        Self { state: 0.0, attack: keep(attack_ms), release: keep(release_ms) }
    }

    #[inline(always)]
    pub fn process(&mut self, level: f32) -> f32 {
        let keep = if level > self.state { self.attack } else { self.release };
        self.state = level + (self.state - level) * keep;
        self.state
    }
}

/// A one-pole highpass. At low cutoffs: removes DC, which a loop can't accumulate.
#[derive(Clone, Copy)]
pub struct DcBlock(OnePole);

impl DcBlock {
    pub fn new(hz: f32, sample_rate: f32) -> Self {
        Self(OnePole::new(hz, sample_rate))
    }

    #[inline(always)]
    pub fn process(&mut self, x: f32) -> f32 {
        x - self.0.process(x)
    }
}

/// A fixed delay, used to keep the dry signal in line with the resampled wet one.
pub struct Delay {
    buffer: Vec<f32>,
    head: usize,
}

impl Delay {
    pub fn new(samples: usize) -> Self {
        Self { buffer: vec![0.0; samples.max(1)], head: 0 }
    }

    #[inline(always)]
    pub fn process(&mut self, x: f32) -> f32 {
        let out = std::mem::replace(&mut self.buffer[self.head], x);
        self.head = (self.head + 1) % self.buffer.len();
        out
    }
}
