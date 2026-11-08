//! One oscillator reading a bank: linear interpolation within a cycle, no band-limiting,
//! so harmonics past Nyquist fold back as they did on 1990s sample playback. A cycle
//! plays from one frame; the next frame is chosen as the cycle starts over (stepped), or
//! neighbouring frames blend continuously (glide). Neighbouring tables always blend,
//! so SCAN glides through the bank.

use super::tables::{Bank, CYCLE, FRAMES, TABLES};

/// How a position inside a table becomes frames.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Reading {
    pub stepped: bool,
    /// Every other frame: 32 instead of 64.
    pub coarse: bool,
}

impl Reading {
    fn step(self) -> usize {
        if self.coarse {
            2
        } else {
            1
        }
    }

    fn slots(self) -> usize {
        (FRAMES - 1) / self.step()
    }

    /// The nearest frame on this reading's grid.
    pub fn nearest(self, wave: f32) -> usize {
        ((wave.clamp(0.0, 1.0) * self.slots() as f32).round() as usize * self.step()).min(FRAMES - 1)
    }

    /// The frames either side, and how far between them.
    fn between(self, wave: f32) -> (usize, usize, f32) {
        let p = wave.clamp(0.0, 1.0) * self.slots() as f32;
        let lo = (p as usize).min(self.slots());
        let hi = (lo + 1).min(self.slots());
        (lo * self.step(), hi * self.step(), p - lo as f32)
    }

    /// How many frames a table has, read this way.
    pub fn frame_count(self) -> usize {
        self.slots() + 1
    }
}

#[inline(always)]
fn sample(cycle: &[f32], j: usize, frac: f32) -> f32 {
    cycle[j] + (cycle[j + 1] - cycle[j]) * frac
}

#[derive(Clone, Copy, Default)]
pub struct Osc {
    /// 0..1 through the cycle.
    phase: f32,
    /// The frame this cycle is reading, when stepped.
    frame: usize,
}

impl Osc {
    pub fn restart(&mut self, phase: f32, wave: f32, reading: Reading) {
        self.phase = phase.rem_euclid(1.0);
        self.frame = reading.nearest(wave);
    }

    pub fn frame(&self) -> usize {
        self.frame
    }

    /// The current sample. `table` is 0..TABLES-1, fractional; `wave` 0..1.
    #[inline(always)]
    pub fn read(&self, bank: &Bank, table: f32, wave: f32, reading: Reading) -> f32 {
        let x = self.phase * CYCLE as f32;
        let j = (x as usize).min(CYCLE - 1);
        let frac = x - j as f32;
        let t = table.clamp(0.0, (TABLES - 1) as f32);
        let lo = t as usize;
        let mix = t - lo as f32;
        let one = |tbl: usize| {
            if reading.stepped {
                sample(bank.cycle(tbl, self.frame), j, frac)
            } else {
                let (a, b, m) = reading.between(wave);
                let va = sample(bank.cycle(tbl, a), j, frac);
                va + (sample(bank.cycle(tbl, b), j, frac) - va) * m
            }
        };
        let v = one(lo);
        if mix > 0.0 {
            v + (one((lo + 1).min(TABLES - 1)) - v) * mix
        } else {
            v
        }
    }

    /// Moves on by `increment` of a cycle, choosing the next frame if a cycle begins.
    #[inline(always)]
    pub fn advance(&mut self, increment: f32, wave: f32, reading: Reading) {
        self.phase += increment;
        if self.phase >= 1.0 {
            self.phase -= self.phase.floor();
            self.frame = reading.nearest(wave);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FINE: Reading = Reading { stepped: true, coarse: false };
    const COARSE: Reading = Reading { stepped: true, coarse: true };

    #[test]
    fn readings_land_on_their_grids() {
        assert_eq!(FINE.nearest(0.0), 0);
        assert_eq!(FINE.nearest(1.0), 63);
        assert_eq!(COARSE.nearest(1.0), 62);
        assert!((0..=100).all(|i| COARSE.nearest(i as f32 / 100.0) % 2 == 0));
        assert_eq!(FINE.frame_count(), 64);
        assert_eq!(COARSE.frame_count(), 32);
    }

    #[test]
    fn stepped_frames_change_only_when_a_cycle_begins() {
        let bank = Bank::build(1);
        let mut osc = Osc::default();
        osc.restart(0.0, 0.0, FINE);
        // Halfway through a cycle, the wave moves: the frame waits.
        osc.advance(0.5, 1.0, FINE);
        assert_eq!(osc.frame(), 0);
        osc.advance(0.6, 1.0, FINE);
        assert_eq!(osc.frame(), 63);
        assert!(osc.read(&bank, 0.0, 1.0, FINE).is_finite());
    }

    #[test]
    fn glide_blends_between_neighbouring_frames() {
        let bank = Bank::build(1);
        let glide = Reading { stepped: false, coarse: false };
        let mut osc = Osc::default();
        osc.restart(0.3, 0.0, glide);
        let (a, b) = (osc.read(&bank, 0.0, 10.0 / 63.0, glide), osc.read(&bank, 0.0, 11.0 / 63.0, glide));
        let mid = osc.read(&bank, 0.0, 10.5 / 63.0, glide);
        assert!((mid - 0.5 * (a + b)).abs() < 1e-5);
    }
}
