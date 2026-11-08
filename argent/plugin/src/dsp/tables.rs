//! Transwave tables, built to the statistics measured from the Fizmo's (see
//! `fizmo-notes.md`): dense spectra reaching hundreds of harmonics, a slow skirt above
//! the brightest region, and a fixed per-harmonic roughness that widens toward the top.
//! No Ensoniq data is used.
//!
//! A SEED makes a bank of tables. Table 0 is always the Metal Comb shape Jeremy picked,
//! roughened and phased by the seed; the rest are drawn variations on it, growing
//! rougher along the bank, with jagged Grit tables at the far end.

use std::sync::Arc;

use realfft::num_complex::Complex;
use realfft::RealFftPlanner;

use super::random::Rng;

/// Samples in one cycle, plus one more repeating the first, so reads need no wrap.
pub const CYCLE: usize = 2048;
pub const STRIDE: usize = CYCLE + 1;
/// Harmonics 1..=HARMONICS.
pub const HARMONICS: usize = CYCLE / 2 - 1;
pub const FRAMES: usize = 64;
pub const TABLES: usize = 16;
/// Where the Grit tables begin.
const FIRST_GRIT: usize = 13;
/// Every frame is brought to this RMS, then a table is lowered as a whole if any of
/// its frames would pass the peak ceiling.
const FRAME_RMS: f32 = 0.25;
const PEAK_CEILING: f32 = 0.95;

fn db(x: f32) -> f32 {
    10f32.powf(x / 20.0)
}

/// Exponential path from a to b.
fn glide(a: f32, b: f32, t: f32) -> f32 {
    a * (b / a).powf(t)
}

/// A resonance: centre harmonic, width in octaves, gain in dB.
#[derive(Clone, Copy)]
struct Peak(f32, f32, f32);

/// One frame's harmonic amplitudes, as `shaped()` in the prototype.
struct Shape {
    tilt: f32,
    peaks: [Peak; 2],
    top: f32,
    skirt_db: f32,
    /// Period in harmonics, depth.
    comb: (f32, f32),
    floor: f32,
}

impl Shape {
    fn amplitudes(&self, out: &mut [f32; HARMONICS]) {
        for (i, a) in out.iter_mut().enumerate() {
            let h = (i + 1) as f32;
            let log_h = h.log2();
            let mut v = h.powf(-self.tilt);
            for Peak(centre, width, gain) in self.peaks {
                let d = (log_h - centre.log2()) / width;
                v *= 1.0 + (db(gain) - 1.0) * (-d * d).exp();
            }
            let (period, depth) = self.comb;
            v *= (1.0 + depth * (std::f32::consts::TAU * h / period).cos()).max(0.0);
            if self.floor > 0.0 {
                v = v.max(self.floor * h.powf(-0.55));
            }
            let over = (h / self.top).max(1.0).log2();
            *a = v * db(self.skirt_db * over);
        }
    }
}

/// How a table's frames change from first to last.
enum Family {
    Metal { tilt: f32, top: f32, comb: (f32, f32), rise: (f32, f32), fall: (f32, f32) },
    Grit { keys: [Shape; 8] },
}

impl Family {
    /// The original Metal Comb.
    fn metal_comb() -> Self {
        Family::Metal { tilt: 1.0, top: 700.0, comb: (2.3, 9.0), rise: (8.0, 250.0), fall: (300.0, 12.0) }
    }

    fn metal(rng: &mut Rng) -> Self {
        let mut comb = (rng.range(2.0, 4.0), rng.range(6.0, 12.0));
        if rng.chance(0.5) {
            comb = (comb.1, comb.0);
        }
        Family::Metal {
            tilt: rng.range(0.9, 1.3),
            top: rng.range(500.0, 750.0),
            comb,
            rise: (rng.range(5.0, 12.0), rng.range(150.0, 350.0)),
            fall: (rng.range(200.0, 400.0), rng.range(8.0, 20.0)),
        }
    }

    fn grit(rng: &mut Rng) -> Self {
        let mut key = || Shape {
            tilt: rng.range(0.8, 1.6),
            peaks: [
                Peak(rng.range(2.0, 300.0), rng.range(0.5, 1.5), rng.range(6.0, 18.0)),
                Peak(rng.range(2.0, 300.0), rng.range(0.5, 1.5), rng.range(6.0, 18.0)),
            ],
            top: rng.range(500.0, 900.0),
            skirt_db: -7.0,
            comb: (rng.range(2.0, 9.0), rng.range(0.3, 0.8)),
            floor: 0.006,
        };
        Family::Grit { keys: std::array::from_fn(|_| key()) }
    }

    /// Frame `t` of the table, 0..1.
    fn amplitudes(&self, t: f32, out: &mut [f32; HARMONICS]) {
        match self {
            Family::Metal { tilt, top, comb, rise, fall } => Shape {
                tilt: *tilt,
                peaks: [Peak(glide(rise.0, rise.1, t), 0.8, 18.0), Peak(glide(fall.0, fall.1, t), 0.6, 14.0)],
                top: *top,
                skirt_db: -9.0,
                comb: (glide(comb.0, comb.1, t), 0.95),
                floor: 0.0,
            }
            .amplitudes(out),
            // Unrelated keyframes, a few frames each: grit jumps rather than glides.
            Family::Grit { keys } => keys[((t * keys.len() as f32) as usize).min(keys.len() - 1)].amplitudes(out),
        }
    }
}

/// Fixed per-harmonic scatter, widening toward the top, in dB.
fn roughness(rng: &mut Rng, spread_db: f32, out: &mut [f32; HARMONICS]) {
    for (i, r) in out.iter_mut().enumerate() {
        let widen = 1.0 + ((i + 1) as f32).log2() / 5.0;
        *r = db((rng.gaussian() * spread_db * widen).clamp(-24.0, 8.0));
    }
}

/// One SEED's tables: TABLES x FRAMES cycles of STRIDE samples.
pub struct Bank {
    pub seed: u32,
    samples: Vec<f32>,
}

impl Bank {
    pub fn build(seed: u32) -> Arc<Self> {
        let mut planner = RealFftPlanner::<f32>::new();
        let inverse = planner.plan_fft_inverse(CYCLE);
        let mut spectrum = inverse.make_input_vec();
        let mut cycle = inverse.make_output_vec();
        let mut scratch = inverse.make_scratch_vec();
        let mut samples = vec![0.0f32; TABLES * FRAMES * STRIDE];
        let mut amplitudes = [0.0f32; HARMONICS];
        let mut rough = [0.0f32; HARMONICS];
        let mut phases = [0.0f32; HARMONICS];

        for table in 0..TABLES {
            let mut rng = Rng::new(seed.wrapping_mul(7919).wrapping_add(table as u32 * 104_729 + 1));
            let along = table as f32 / (TABLES - 1) as f32;
            let (family, spread_db) = match table {
                0 => (Family::metal_comb(), 6.0),
                t if t >= FIRST_GRIT => (Family::grit(&mut rng), 9.0),
                _ => (Family::metal(&mut rng), 5.0 + 4.0 * along),
            };
            roughness(&mut rng, spread_db, &mut rough);
            phases.iter_mut().for_each(|p| *p = rng.range(-std::f32::consts::PI, std::f32::consts::PI));

            let start = table * FRAMES * STRIDE;
            for frame in 0..FRAMES {
                family.amplitudes(frame as f32 / (FRAMES - 1) as f32, &mut amplitudes);
                spectrum.iter_mut().for_each(|c| *c = Complex::new(0.0, 0.0));
                for h in 0..HARMONICS {
                    let a = amplitudes[h] * rough[h];
                    spectrum[h + 1] = Complex::from_polar(a, phases[h]);
                }
                inverse.process_with_scratch(&mut spectrum, &mut cycle, &mut scratch).expect("sizes match the plan");
                let rms = (cycle.iter().map(|x| x * x).sum::<f32>() / CYCLE as f32).sqrt().max(1e-12);
                let out = &mut samples[start + frame * STRIDE..start + (frame + 1) * STRIDE];
                for (o, x) in out.iter_mut().zip(cycle.iter()) {
                    *o = x * FRAME_RMS / rms;
                }
                out[CYCLE] = out[0];
            }
            let table_samples = &mut samples[start..start + FRAMES * STRIDE];
            let peak = table_samples.iter().fold(0.0f32, |m, x| m.max(x.abs()));
            if peak > PEAK_CEILING {
                let scale = PEAK_CEILING / peak;
                table_samples.iter_mut().for_each(|x| *x *= scale);
            }
        }
        Arc::new(Self { seed, samples })
    }

    /// One cycle (STRIDE samples, the last repeating the first).
    #[inline(always)]
    pub fn cycle(&self, table: usize, frame: usize) -> &[f32] {
        let at = (table * FRAMES + frame) * STRIDE;
        &self.samples[at..at + STRIDE]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_seed_always_makes_the_same_bank() {
        let (a, b) = (Bank::build(417), Bank::build(417));
        assert!(a.samples == b.samples);
        let c = Bank::build(418);
        assert!(a.samples != c.samples);
    }

    #[test]
    fn every_cycle_is_finite_loops_and_stays_under_the_ceiling() {
        let bank = Bank::build(3);
        for table in 0..TABLES {
            for frame in 0..FRAMES {
                let cycle = bank.cycle(table, frame);
                assert!(cycle.iter().all(|x| x.is_finite() && x.abs() <= PEAK_CEILING + 1e-4));
                assert_eq!(cycle[CYCLE], cycle[0]);
                let rms = (cycle[..CYCLE].iter().map(|x| x * x).sum::<f32>() / CYCLE as f32).sqrt();
                assert!(rms > 0.05 && rms <= FRAME_RMS + 1e-4, "table {table} frame {frame}: rms {rms}");
            }
        }
    }

    #[test]
    fn tables_reach_hundreds_of_harmonics() {
        // Real energy above harmonic 200 (here, better than -40 dB of the whole),
        // as the measured tables have.
        let bank = Bank::build(9);
        let mut planner = RealFftPlanner::<f32>::new();
        let forward = planner.plan_fft_forward(CYCLE);
        let mut input = bank.cycle(0, FRAMES / 2)[..CYCLE].to_vec();
        let mut spectrum = forward.make_output_vec();
        forward.process(&mut input, &mut spectrum).unwrap();
        let power = |range: std::ops::Range<usize>| range.map(|h| spectrum[h].norm_sqr()).sum::<f32>();
        let above = power(200..1024) / power(1..1024);
        assert!(above > 1e-4, "{above}");
    }
}
