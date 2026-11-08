//! The Agitation generator: a free-running rise and fall, from a minute to 1 kHz.

use std::f32::consts::PI;

const SLOWEST_HZ: f32 = 1.0 / 60.0;
const FASTEST_HZ: f32 = 1000.0;

/// The rate at a Speed knob position, 0..1.
pub fn speed_hz(speed: f32) -> f32 {
    SLOWEST_HZ * (FASTEST_HZ / SLOWEST_HZ).powf(speed)
}

/// The Speed knob position for a rate: the inverse of `speed_hz`.
pub fn speed_knob(hz: f32) -> f32 {
    ((hz / SLOWEST_HZ).ln() / (FASTEST_HZ / SLOWEST_HZ).ln()).clamp(0.0, 1.0)
}

/// The rise and fall at a point in the cycle, 0..1. Returns 0..1. `angle` is the part
/// of the cycle spent rising.
#[inline(always)]
pub fn shape(phase: f32, angle: f32) -> f32 {
    let rise = angle.clamp(0.01, 0.99);
    let segment = if phase < rise { phase / rise } else { (1.0 - phase) / (1.0 - rise) };
    0.5 - 0.5 * (PI * segment).cos()
}

pub struct Agitation {
    phase: f32,
    dt: f32,
}

impl Agitation {
    pub fn new(sample_rate: f32) -> Self {
        Self { phase: 0.0, dt: 1.0 / sample_rate }
    }

    /// `angle` is the part of the cycle spent rising, 0..1. Returns 0..1.
    #[inline(always)]
    pub fn process(&mut self, speed: f32, angle: f32) -> f32 {
        self.phase = (self.phase + speed_hz(speed) * self.dt).fract();
        shape(self.phase, angle)
    }

    pub fn phase(&self) -> f32 {
        self.phase
    }
}
