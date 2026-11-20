//! The four global modulators, each with its colour on the panel:
//!
//! - Silver, Interference (after Antimeme's): there it's the delay loop's own audio
//!   through a 30 Hz one-pole, times three. Here it's the three insert delays' summed
//!   output (before the fuzz) through the same pole, tanh-clipped: the drone shaking
//!   its own knobs, which reads as a noise-like jitter. Bipolar.
//! - Red, Agitation (after Antimeme's): a raised-cosine rise and fall; Time is its rate
//!   (0.02–8 Hz), Slope how much of the cycle it spends rising. Unipolar: it only ever
//!   pushes a control upward. Three copies (I, II, III), each with its own Time and Slope.
//! - Green, a slow random walk: a new target every 6–16 s, reached through two one-poles
//!   of about 3.5 s. Bipolar.
//! - Slate, sample and hold: white noise held for a quarter note, a bar or four bars
//!   (the host's grid; with the transport stopped it keeps time at the host's tempo).
//!   Bipolar. Three copies (I, II, III), each with its own hold.
//!
//! A knob's red and slate arms follow the copy of its row: oscillator row r's knobs use
//! copy r, an insert effect the copy of the row it's docked in, the bottom row copy I.

use std::f32::consts::PI;

use super::filters::{one_pole, OnePole};

pub const COUNT: usize = 4;
/// The modulators in panel order (and arm order): silver, red, green, slate.
pub const NAMES: [&str; COUNT] = ["Silver", "Red", "Green", "Slate"];
pub const SILVER: usize = 0;
pub const RED: usize = 1;
pub const GREEN: usize = 2;
pub const SLATE: usize = 3;

const SILVER_HZ: f32 = 30.0;
const SILVER_GAIN: f32 = 3.0;
const GREEN_TAU_S: f32 = 3.5;
/// Slate's three holds, in beats (quarter notes).
pub const SLATE_BEATS: [f64; 3] = [1.0, 4.0, 16.0];
/// How many copies of red and slate there are: one per row.
pub const COPIES: usize = 3;

/// Every modulator's latest value.
#[derive(Clone, Copy, Debug, Default)]
pub struct Values {
    pub silver: f32,
    pub green: f32,
    pub red: [f32; COPIES],
    pub slate: [f32; COPIES],
}

impl Values {
    /// The four a knob on row `r` sees, in arm order (silver, red, green, slate).
    pub fn row(&self, r: usize) -> [f32; COUNT] {
        let r = r.min(COPIES - 1);
        [self.silver, self.red[r], self.green, self.slate[r]]
    }

    /// All eight, in a fixed order: silver, red I–III, green, slate I–III.
    pub fn flat(&self) -> [f32; 8] {
        [self.silver, self.red[0], self.red[1], self.red[2], self.green, self.slate[0], self.slate[1], self.slate[2]]
    }

    pub fn from_flat(v: [f32; 8]) -> Self {
        Self { silver: v[0], red: [v[1], v[2], v[3]], green: v[4], slate: [v[5], v[6], v[7]] }
    }
}

/// Red's Time knob (0..1) as a rate: 0.02 to 8 Hz, evenly in octaves.
pub fn red_hz(knob: f32) -> f32 {
    0.02 * 400f32.powf(knob.clamp(0.0, 1.0))
}

/// Agitation's rise and fall (Antimeme's `shape`): 0..1; `angle` is the share rising.
#[inline(always)]
pub fn agitation(phase: f32, angle: f32) -> f32 {
    let rise = angle.clamp(0.01, 0.99);
    let segment = if phase < rise { phase / rise } else { (1.0 - phase) / (1.0 - rise) };
    0.5 - 0.5 * (PI * segment).cos()
}

/// A small xorshift, for the green walk and the slate's samples.
#[derive(Clone, Copy)]
pub struct Rng(u32);

impl Rng {
    pub fn new(seed: u32) -> Self {
        Self(seed.max(1))
    }

    /// -1..1.
    pub fn bipolar(&mut self) -> f32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 17;
        self.0 ^= self.0 << 5;
        (self.0 as f32 / u32::MAX as f32) * 2.0 - 1.0
    }

    /// 0..1.
    pub fn unit(&mut self) -> f32 {
        0.5 + 0.5 * self.bipolar()
    }
}

/// Where the host's transport is, for the slate's grid.
#[derive(Clone, Copy, Debug)]
pub struct Transport {
    pub tempo: f64,
    /// The song position in beats while playing.
    pub beats: Option<f64>,
}

pub struct Modulators {
    rate: f32,
    silver_pole: OnePole,
    silver_a: f32,
    red_phase: [f32; COPIES],
    green_target: f32,
    green: [f32; 2],
    green_wait: f32,
    slate: [f32; COPIES],
    slate_slot: [i64; COPIES],
    free_beats: f64,
    rng: Rng,
    pub values: Values,
}

impl Modulators {
    pub fn new(rate: f32) -> Self {
        let mut rng = Rng::new(0x5eed_d4a6);
        let green_target = rng.bipolar();
        Self {
            rate,
            silver_pole: OnePole::default(),
            silver_a: one_pole(SILVER_HZ, rate),
            // The copies start a third of a cycle apart, so equal settings still differ.
            red_phase: [0.0, 1.0 / 3.0, 2.0 / 3.0],
            green_target,
            green: [0.0; 2],
            green_wait: 6.0,
            slate: [rng.bipolar(), rng.bipolar(), rng.bipolar()],
            slate_slot: [i64::MIN; COPIES],
            free_beats: 0.0,
            rng,
            values: Values::default(),
        }
    }

    /// The silver source, fed one sample at a time (the delays' summed output).
    #[inline(always)]
    pub fn feed_silver(&mut self, x: f32) {
        self.silver_pole.low(x, self.silver_a);
    }

    /// Move the slow ones on by `n` samples, and read them all.
    pub fn block(&mut self, n: usize, red_time: [f32; COPIES], red_slope: [f32; COPIES], slate_rate: [usize; COPIES],
                 transport: Transport) -> Values {
        let dt = n as f32 / self.rate;
        let silver = (SILVER_GAIN * self.silver_pole.y).tanh();

        let mut red = [0.0; COPIES];
        for k in 0..COPIES {
            self.red_phase[k] = (self.red_phase[k] + red_hz(red_time[k]) * dt).fract();
            red[k] = agitation(self.red_phase[k], red_slope[k]);
        }

        self.green_wait -= dt;
        if self.green_wait <= 0.0 {
            self.green_target = self.rng.bipolar();
            self.green_wait = 6.0 + 10.0 * self.rng.unit();
        }
        let a = (dt / GREEN_TAU_S).min(1.0);
        self.green[0] += a * (self.green_target - self.green[0]);
        self.green[1] += a * (self.green[0] - self.green[1]);

        let beats = match transport.beats {
            Some(b) => {
                self.free_beats = b;
                b
            }
            None => {
                self.free_beats += dt as f64 * transport.tempo / 60.0;
                self.free_beats
            }
        };
        for k in 0..COPIES {
            let slot = (beats / SLATE_BEATS[slate_rate[k].min(2)]).floor() as i64;
            if slot != self.slate_slot[k] {
                self.slate_slot[k] = slot;
                self.slate[k] = self.rng.bipolar();
            }
        }

        self.values = Values { silver, green: self.green[1], red, slate: self.slate };
        self.values
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ranges() {
        let mut m = Modulators::new(48_000.0);
        let stopped = Transport { tempo: 120.0, beats: None };
        let (mut lo, mut hi) = ([1.0f32; COUNT], [-1.0f32; COUNT]);
        for i in 0..48_000 * 60 / 32 {
            m.feed_silver(((i * 7919) % 101) as f32 / 50.0 - 1.0);
            let v = m.block(32, [0.5; 3], [0.3; 3], [0; 3], stopped).row(1);
            for k in 0..COUNT {
                lo[k] = lo[k].min(v[k]);
                hi[k] = hi[k].max(v[k]);
            }
        }
        assert!(lo[RED] >= 0.0 && hi[RED] <= 1.0 && hi[RED] > 0.99);
        assert!(lo[SLATE] < -0.3 && hi[SLATE] > 0.3);
        assert!(hi[GREEN] - lo[GREEN] > 0.05 && hi[GREEN] <= 1.0 && lo[GREEN] >= -1.0);
        assert!(hi[SILVER] <= 1.0 && lo[SILVER] >= -1.0);
    }

    #[test]
    fn slate_holds_for_its_grid() {
        let mut m = Modulators::new(48_000.0);
        let mut changes = 0;
        let mut last = f32::NAN;
        // 8 beats at 120 BPM, playing: quarter-note holds change 8 times (plus the first).
        for i in 0..(4 * 48_000 / 32) {
            let beats = i as f64 * 32.0 / 48_000.0 * 2.0;
            let v = m.block(32, [0.5; 3], [0.5; 3], [0, 1, 2], Transport { tempo: 120.0, beats: Some(beats) }).slate[0];
            if v != last {
                changes += 1;
                last = v;
            }
        }
        assert_eq!(changes, 8);
    }
}

#[cfg(test)]
mod copies {
    use super::*;

    #[test]
    fn copies_differ() {
        let mut m = Modulators::new(48_000.0);
        let stopped = Transport { tempo: 120.0, beats: None };
        let mut apart = [0.0f32; 2];
        for _ in 0..48_000 * 8 / 32 {
            let v = m.block(32, [0.5; 3], [0.5; 3], [0; 3], stopped);
            apart[0] = apart[0].max((v.red[0] - v.red[1]).abs());
            apart[1] = apart[1].max((v.slate[0] - v.slate[2]).abs());
        }
        // Same settings, still not in step.
        assert!(apart[0] > 0.5 && apart[1] > 0.1, "{apart:?}");
        let v = m.values;
        assert_eq!(Values::from_flat(v.flat()).row(2), v.row(2));
        assert_eq!(v.row(9), v.row(2));
    }
}
