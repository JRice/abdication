//! Transwave tables, built to the statistics measured from the Fizmo's (see
//! `fizmo-notes.md`): dense spectra reaching hundreds of harmonics, a slow skirt above
//! the brightest region, and a fixed per-harmonic roughness that widens toward the top.
//! No Ensoniq data is used.
//!
//! A SEED makes a bank of tables. Table 0 is always the Metal Comb shape Jeremy picked,
//! roughened and phased by the seed; the rest are drawn variations on it, growing
//! rougher along the bank, with jagged Grit tables at the far end.
//!
//! After the drawn tables come the recorded ones: the keyframe waves of 18 Phizmo sweeps
//! Jeremy recorded (`assets/recorded.bin`, made by `tools/recorded.py`), each table
//! gliding through its sweep's keys in order. They're the same for every SEED, so they
//! are built once and shared.

use std::sync::{Arc, OnceLock};

use realfft::num_complex::Complex;
use realfft::RealFftPlanner;

use super::random::Rng;

/// Samples in one cycle, plus one more repeating the first, so reads need no wrap.
pub const CYCLE: usize = 2048;
pub const STRIDE: usize = CYCLE + 1;
/// Harmonics 1..=HARMONICS.
pub const HARMONICS: usize = CYCLE / 2 - 1;
pub const FRAMES: usize = 64;
/// Tables drawn per SEED, then recorded ones: SCAN runs across all of them.
pub const DRAWN: usize = 16;
pub const RECORDED: usize = 18;
pub const TABLES: usize = DRAWN + RECORDED;
/// Where the Grit tables begin.
const FIRST_GRIT: usize = 13;
/// Every frame is brought to this RMS, then a table is lowered as a whole if any of
/// its frames would pass the peak ceiling.
const FRAME_RMS: f32 = 0.25;
const PEAK_CEILING: f32 = 0.95;
/// Recorded waves keep most of their energy low, where the dark body lets it through, so
/// at the same RMS they'd come out far louder than the drawn tables. Each recorded frame is
/// turned down (never up) until its level through a body-like filter (a C2 note through a
/// 4-pole lowpass at 520 Hz) is at most this: a little under the drawn frames' median
/// (0.156; quartiles 0.109 and 0.207, from `drawn_body_level`). Rendered chords then sit
/// within the drawn tables' range, about -20 to -24 dB RMS at default settings.
const BODY_NOTE_HZ: f32 = 65.4;
const BODY_CUTOFF_HZ: f32 = 520.0;
const DRAWN_BODY_LEVEL: f32 = 0.125;

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

/// Turns spectra into normalized cycles: each frame to FRAME_RMS, then each table
/// lowered as a whole if a frame would pass the peak ceiling.
struct CycleMaker {
    inverse: Arc<dyn realfft::ComplexToReal<f32>>,
    spectrum: Vec<Complex<f32>>,
    cycle: Vec<f32>,
    scratch: Vec<Complex<f32>>,
}

impl CycleMaker {
    fn new() -> Self {
        let inverse = RealFftPlanner::<f32>::new().plan_fft_inverse(CYCLE);
        let (spectrum, cycle, scratch) = (inverse.make_input_vec(), inverse.make_output_vec(), inverse.make_scratch_vec());
        Self { inverse, spectrum, cycle, scratch }
    }

    /// One frame into `out` (STRIDE samples), harmonic h+1 given by `harmonic(h)`, at
    /// FRAME_RMS. Returns its level through the body (see DRAWN_BODY_LEVEL).
    fn frame(&mut self, out: &mut [f32], harmonic: impl Fn(usize) -> Complex<f32>) -> f32 {
        self.spectrum.iter_mut().for_each(|c| *c = Complex::new(0.0, 0.0));
        let (mut total, mut through) = (0.0f32, 0.0f32);
        for h in 0..HARMONICS {
            let c = harmonic(h);
            let power = c.norm_sqr();
            total += power;
            through += power * body_weight(h + 1);
            self.spectrum[h + 1] = c;
        }
        self.inverse.process_with_scratch(&mut self.spectrum, &mut self.cycle, &mut self.scratch).expect("sizes match the plan");
        let rms = (self.cycle.iter().map(|x| x * x).sum::<f32>() / CYCLE as f32).sqrt().max(1e-12);
        for (o, x) in out.iter_mut().zip(self.cycle.iter()) {
            *o = x * FRAME_RMS / rms;
        }
        out[CYCLE] = out[0];
        FRAME_RMS * (through / total.max(1e-30)).sqrt()
    }

    fn cap(table: &mut [f32]) {
        let peak = table.iter().fold(0.0f32, |m, x| m.max(x.abs()));
        if peak > PEAK_CEILING {
            let scale = PEAK_CEILING / peak;
            table.iter_mut().for_each(|x| *x *= scale);
        }
    }
}

/// How much of harmonic `h` the body passes, in power.
fn body_weight(h: usize) -> f32 {
    1.0 / (1.0 + (h as f32 * BODY_NOTE_HZ / BODY_CUTOFF_HZ).powi(8))
}

const TABLE_LEN: usize = FRAMES * STRIDE;

/// The recorded tables, built once from the keyframes in `assets/recorded.bin`.
pub struct Recorded {
    names: Vec<String>,
    samples: Vec<f32>,
}

static RECORDED_ASSET: &[u8] = include_bytes!("../../assets/recorded.bin");
static RECORDED_TABLES: OnceLock<Recorded> = OnceLock::new();

/// One recorded sweep: its name and its keys, each a magnitude and a phase per harmonic.
struct Sweep {
    name: String,
    keys: Vec<Vec<(f32, f32)>>,
}

/// Reads the asset. Its layout is described in `tools/recorded.py`.
fn sweeps(bytes: &[u8]) -> Vec<Sweep> {
    let mut at = 0usize;
    let mut take = |n: usize| {
        let s = &bytes[at..at + n];
        at += n;
        s
    };
    assert_eq!(take(4), b"ARGR", "recorded tables: not an Argent asset");
    let u16_of = |b: &[u8]| u16::from_le_bytes([b[0], b[1]]) as usize;
    let f32_of = |b: &[u8]| f32::from_le_bytes([b[0], b[1], b[2], b[3]]);
    assert_eq!(u16_of(take(2)), 1, "recorded tables: unknown version");
    let count = u16_of(take(2));
    (0..count)
        .map(|_| {
            let len = take(1)[0] as usize;
            let name = String::from_utf8_lossy(take(len)).into_owned();
            let harmonics = u16_of(take(2));
            let key_count = take(1)[0] as usize;
            let keys = (0..key_count)
                .map(|_| (0..harmonics).map(|_| (f32_of(take(4)), f32_of(take(4)))).collect())
                .collect();
            Sweep { name, keys }
        })
        .collect()
}

/// The way from one phase to another that turns least.
fn phase_toward(from: f32, to: f32, t: f32) -> f32 {
    let mut d = (to - from) % std::f32::consts::TAU;
    if d > std::f32::consts::PI {
        d -= std::f32::consts::TAU;
    } else if d < -std::f32::consts::PI {
        d += std::f32::consts::TAU;
    }
    from + d * t
}

impl Recorded {
    fn build() -> Self {
        let sweeps = sweeps(RECORDED_ASSET);
        let mut maker = CycleMaker::new();
        let mut samples = vec![0.0f32; sweeps.len() * TABLE_LEN];
        for (sweep, table) in sweeps.iter().zip(samples.chunks_mut(TABLE_LEN)) {
            let last = sweep.keys.len() - 1;
            for (frame, out) in table.chunks_mut(STRIDE).enumerate() {
                // Keys spread evenly along the table; magnitudes and phases glide between
                // them, so a frame between two keys never cancels itself out.
                let u = frame as f32 / (FRAMES - 1) as f32 * last as f32;
                let lo = (u as usize).min(last);
                let hi = (lo + 1).min(last);
                let t = u - lo as f32;
                let (a, b) = (&sweep.keys[lo], &sweep.keys[hi]);
                let body = maker.frame(out, |h| match (a.get(h), b.get(h)) {
                    (Some(&(ma, pa)), Some(&(mb, pb))) => Complex::from_polar(ma + (mb - ma) * t, phase_toward(pa, pb, t)),
                    _ => Complex::new(0.0, 0.0),
                });
                if body > DRAWN_BODY_LEVEL {
                    let down = DRAWN_BODY_LEVEL / body;
                    out.iter_mut().for_each(|x| *x *= down);
                }
            }
            CycleMaker::cap(table);
        }
        Self { names: sweeps.into_iter().map(|s| s.name).collect(), samples }
    }

    /// Built the first time it's asked for; Bank::build asks, off the audio thread.
    pub fn get() -> &'static Self {
        RECORDED_TABLES.get_or_init(Self::build)
    }

    fn cycle(&self, table: usize, frame: usize) -> &[f32] {
        let at = table * TABLE_LEN + frame * STRIDE;
        &self.samples[at..at + STRIDE]
    }
}

/// The name of a recorded table, for the panel. None for the drawn ones.
pub fn table_name(table: usize) -> Option<&'static str> {
    table.checked_sub(DRAWN).and_then(|i| Recorded::get().names.get(i)).map(String::as_str)
}

/// One SEED's drawn tables (DRAWN x FRAMES cycles of STRIDE samples), and the
/// recorded ones after them.
pub struct Bank {
    pub seed: u32,
    samples: Vec<f32>,
    recorded: &'static Recorded,
}

impl Bank {
    pub fn build(seed: u32) -> Arc<Self> {
        let mut maker = CycleMaker::new();
        let mut samples = vec![0.0f32; DRAWN * TABLE_LEN];
        let mut amplitudes = [0.0f32; HARMONICS];
        let mut rough = [0.0f32; HARMONICS];
        let mut phases = [0.0f32; HARMONICS];

        for (table, out) in samples.chunks_mut(TABLE_LEN).enumerate() {
            let mut rng = Rng::new(seed.wrapping_mul(7919).wrapping_add(table as u32 * 104_729 + 1));
            let along = table as f32 / (DRAWN - 1) as f32;
            let (family, spread_db) = match table {
                0 => (Family::metal_comb(), 6.0),
                t if t >= FIRST_GRIT => (Family::grit(&mut rng), 9.0),
                _ => (Family::metal(&mut rng), 5.0 + 4.0 * along),
            };
            roughness(&mut rng, spread_db, &mut rough);
            phases.iter_mut().for_each(|p| *p = rng.range(-std::f32::consts::PI, std::f32::consts::PI));
            for (frame, cycle) in out.chunks_mut(STRIDE).enumerate() {
                family.amplitudes(frame as f32 / (FRAMES - 1) as f32, &mut amplitudes);
                maker.frame(cycle, |h| Complex::from_polar(amplitudes[h] * rough[h], phases[h]));
            }
            CycleMaker::cap(out);
        }
        Arc::new(Self { seed, samples, recorded: Recorded::get() })
    }

    /// One cycle (STRIDE samples, the last repeating the first).
    #[inline(always)]
    pub fn cycle(&self, table: usize, frame: usize) -> &[f32] {
        if table < DRAWN {
            let at = table * TABLE_LEN + frame * STRIDE;
            &self.samples[at..at + STRIDE]
        } else {
            self.recorded.cycle(table - DRAWN, frame)
        }
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
    fn the_asset_holds_the_recorded_tables_the_bank_expects() {
        let sweeps = sweeps(RECORDED_ASSET);
        assert_eq!(sweeps.len(), RECORDED);
        assert!(sweeps.iter().all(|s| (2..=5).contains(&s.keys.len()) && s.keys.iter().all(|k| k.len() >= 200)));
        assert_eq!(table_name(DRAWN), Some("BELL-X-WAVE 3"));
        assert_eq!(table_name(TABLES - 1), Some("TRI SWEEP"));
        assert_eq!(table_name(DRAWN - 1), None);
    }

    #[test]
    fn recorded_tables_hit_their_keys_and_glide_between_them() {
        // The first and last frames are the first and last keys; frames between them
        // change a little at a time, never jumping.
        let bank = Bank::build(1);
        for table in DRAWN..TABLES {
            let step = |f: usize| {
                let (a, b) = (bank.cycle(table, f), bank.cycle(table, f + 1));
                a.iter().zip(b).map(|(x, y)| (x - y).powi(2)).sum::<f32>().sqrt()
            };
            let steps: Vec<f32> = (0..FRAMES - 1).map(step).collect();
            let biggest = steps.iter().cloned().fold(0.0, f32::max);
            assert!(biggest < 8.0, "{}: a step of {biggest}", table_name(table).unwrap());
        }
    }

    #[test]
    fn a_recorded_table_begins_and_ends_on_the_measured_waves() {
        let sweeps = sweeps(RECORDED_ASSET);
        let bank = Bank::build(1);
        let mut planner = RealFftPlanner::<f32>::new();
        let forward = planner.plan_fft_forward(CYCLE);
        for (i, sweep) in sweeps.iter().enumerate() {
            for (frame, key) in [(0, &sweep.keys[0]), (FRAMES - 1, sweep.keys.last().unwrap())] {
                let mut input = bank.cycle(DRAWN + i, frame)[..CYCLE].to_vec();
                let mut spectrum = forward.make_output_vec();
                forward.process(&mut input, &mut spectrum).unwrap();
                // Same shape: magnitudes proportional, harmonic by harmonic.
                let (mut dot, mut aa, mut bb) = (0.0f64, 0.0f64, 0.0f64);
                for (h, &(m, _)) in key.iter().enumerate() {
                    let got = spectrum[h + 1].norm() as f64;
                    dot += got * m as f64;
                    aa += got * got;
                    bb += (m as f64) * (m as f64);
                }
                let similarity = dot / (aa.sqrt() * bb.sqrt());
                assert!(similarity > 0.999, "{} frame {frame}: {similarity}", sweep.name);
            }
        }
    }

    /// cargo test --release --lib drawn_body_level -- --ignored --nocapture
    #[test]
    #[ignore]
    fn drawn_body_level() {
        let mut maker = CycleMaker::new();
        let mut levels = Vec::new();
        for seed in [1, 417, 2024, 9_999] {
            let bank = Bank::build(seed);
            for table in 0..DRAWN {
                for frame in (0..FRAMES).step_by(4) {
                    let mut planner = RealFftPlanner::<f32>::new();
                    let forward = planner.plan_fft_forward(CYCLE);
                    let mut input = bank.cycle(table, frame)[..CYCLE].to_vec();
                    let mut spectrum = forward.make_output_vec();
                    forward.process(&mut input, &mut spectrum).unwrap();
                    let mut out = vec![0.0; STRIDE];
                    levels.push(maker.frame(&mut out, |h| spectrum[h + 1]));
                }
            }
        }
        levels.sort_by(f32::total_cmp);
        println!("drawn body level: median {:.4}, quartiles {:.4} {:.4}", levels[levels.len() / 2], levels[levels.len() / 4], levels[3 * levels.len() / 4]);
    }

    #[test]
    fn phases_take_the_short_way_round() {
        use std::f32::consts::PI;
        assert!((phase_toward(3.0, -3.0, 1.0) - (2.0 * PI - 3.0)).abs() < 1e-5);
        assert!((phase_toward(0.5, 1.5, 0.5) - 1.0).abs() < 1e-6);
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
