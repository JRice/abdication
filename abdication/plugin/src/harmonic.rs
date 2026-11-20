//! The Harmonic EQ (after DS Thorn's Harmonic Filter, which shapes an
//! additive synth's partials): on each oscillator slot, before the mixer, a
//! chain of 16 peaking filters at harmonics of the slot's own pitch, so a
//! shape over the harmonics lands on the same partials at every note.
//!
//! The shape is one of Thorn's (harmlib), read as Thorn reads it, at entry
//! harmonic * k: Shift moves k by octaves (16 entries per harmonic at 0), so
//! the shape slides along the harmonics. Each band's target is the shape's
//! mean power over the harmonics it covers, relative to its loudest, floored
//! at FLOOR_DB and lifted or lowered (within LEVEL_DB) so a saw comes out as
//! loud as it went in; then Depth scales it and Tilt (dB per octave of
//! harmonics, about harmonic 8) is added. The bands' gains come from those
//! targets through the inverse of how the bands overlap (a peaking filter's
//! skirts reach its neighbours), so the chain lands on them at the bands'
//! centres. After the bands, Odd/Even: a comb half the slot's period long
//! (−1 keeps only the odd harmonics, +1 only the even, an octave up).

use std::f32::consts::PI;
use std::sync::OnceLock;

use crate::harmlib;

/// The bands' centres, in harmonics of the slot's pitch: one on each of the
/// first six partials, then about three to the octave.
pub const BANDS: [f32; 16] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 8.0, 10.0, 12.0, 16.0, 20.0, 24.0, 32.0, 40.0, 48.0, 64.0];
const N: usize = BANDS.len();
/// Shape entries per harmonic at Shift 0.
pub const BASE_SCALE: f32 = 16.0;
/// How far under its loudest part a shape is followed (deeper is the floor).
pub const FLOOR_DB: f32 = -30.0;
/// How far the level correction may lift or lower a shape.
const LEVEL_DB: f32 = 12.0;
/// No band goes past this, either way.
pub const GAIN_DB: f32 = 30.0;
/// Tilt's pivot, in harmonics.
const PIVOT: f32 = 8.0;

/// The bands' fixed arrangement: the harmonics each covers, its Q, how much
/// of a saw's power falls in it, and the inverse of their overlap.
struct Layout {
    lo: [f32; N],
    hi: [f32; N],
    q: [f32; N],
    saw: [f32; N],
    solve: [[f32; N]; N],
}

/// A peaking filter's gain (dB) at `f`, centred on `fc` (same units), with
/// Q `q` and `db` at its centre (the analogue prototype the SVF matches).
pub fn bell_db(f: f32, fc: f32, q: f32, db: f32) -> f32 {
    let a = 10f32.powf(db / 40.0);
    let w = f / fc;
    let re = 1.0 - w * w;
    let num = re * re + (w * a / q).powi(2);
    let den = re * re + (w / (a * q)).powi(2);
    10.0 * (num / den).log10()
}

fn layout() -> &'static Layout {
    static LAYOUT: OnceLock<Layout> = OnceLock::new();
    LAYOUT.get_or_init(|| {
        let lg = BANDS.map(f32::log2);
        let edge = |i: usize| match i {
            0 => lg[0] - 0.5,
            i if i == N => lg[N - 1] + 0.2,
            i => (lg[i - 1] + lg[i]) / 2.0,
        };
        let lo = std::array::from_fn(|b| edge(b).exp2());
        let hi = std::array::from_fn(|b| edge(b + 1).exp2());
        // A Q whose bandwidth is the band's share of the octaves.
        let q = std::array::from_fn(|b| {
            let octaves = edge(b + 1) - edge(b);
            octaves.exp2().sqrt() / (octaves.exp2() - 1.0)
        });
        let saw = std::array::from_fn(|b: usize| {
            (1..=96).map(|h| h as f32).filter(|h| *h >= lo[b] && *h < hi[b]).map(|h| 1.0 / (h * h)).sum()
        });
        // overlap[i][j]: band j at +1 dB, in dB at band i's centre.
        let overlap: [[f64; N]; N] =
            std::array::from_fn(|i| std::array::from_fn(|j| bell_db(BANDS[i], BANDS[j], q[j], 1.0) as f64));
        let solve = invert(overlap).map(|row| row.map(|v| v as f32));
        Layout { lo, hi, q, saw, solve }
    })
}

/// Gauss-Jordan, with partial pivoting.
fn invert(mut m: [[f64; N]; N]) -> [[f64; N]; N] {
    let mut inv: [[f64; N]; N] = std::array::from_fn(|i| std::array::from_fn(|j| (i == j) as u8 as f64));
    for c in 0..N {
        let p = (c..N).max_by(|&a, &b| m[a][c].abs().total_cmp(&m[b][c].abs())).unwrap();
        m.swap(c, p);
        inv.swap(c, p);
        let d = m[c][c];
        for j in 0..N {
            m[c][j] /= d;
            inv[c][j] /= d;
        }
        for r in (0..N).filter(|&r| r != c) {
            let f = m[r][c];
            for j in 0..N {
                m[r][j] -= f * m[c][j];
                inv[r][j] -= f * inv[c][j];
            }
        }
    }
    inv
}

/// What the Harmonic EQ reads (params::HARMONIC_FIELDS, in that order).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HarmonicSettings {
    /// 0 Off, then a shape (harmlib::NAMES[shape - 1]).
    pub shape: f32,
    pub shift: f32,
    pub tilt: f32,
    pub depth: f32,
    pub odd_even: f32,
}

impl HarmonicSettings {
    pub fn from_values(v: [f32; 5]) -> Self {
        let [shape, shift, tilt, depth, odd_even] = v;
        Self { shape, shift, tilt, depth, odd_even }
    }

    fn shape_index(&self) -> Option<usize> {
        let s = self.shape.round() as usize;
        (s >= 1 && s <= harmlib::NAMES.len()).then(|| s - 1)
    }

    /// Whether the bands do anything.
    fn bands_on(&self) -> bool {
        (self.shape_index().is_some() && self.depth != 0.0) || self.tilt != 0.0
    }

    /// Whether the EQ does anything at all.
    pub fn on(&self) -> bool {
        self.bands_on() || self.odd_even != 0.0
    }

    /// Each band's target (dB at its centre).
    pub fn targets(&self) -> [f32; N] {
        let l = layout();
        let tilt = BANDS.map(|r| self.tilt * (r / PIVOT).log2());
        let Some(shape) = self.shape_index() else { return tilt };
        let k = BASE_SCALE * (-self.shift).exp2();
        let power: [f32; N] = std::array::from_fn(|b| harmlib::mean_power(shape, l.lo[b] * k, l.hi[b] * k));
        let db = power.map(|p| 10.0 * (p + 1e-12).log10());
        let top = db.iter().copied().fold(f32::MIN, f32::max);
        if top < -100.0 {
            // Nothing of the shape reaches these harmonics: it lets nothing through.
            return std::array::from_fn(|b| self.depth * FLOOR_DB + tilt[b]);
        }
        let shaped = db.map(|d| (d - top).max(FLOOR_DB));
        // As loud as a saw went in: the saw's power through the shape.
        let through: f32 = shaped.iter().zip(&l.saw).map(|(d, w)| w * 10f32.powf(d / 10.0)).sum();
        let level = (-10.0 * (through / l.saw.iter().sum::<f32>()).log10()).clamp(-LEVEL_DB, LEVEL_DB);
        std::array::from_fn(|b| self.depth * (shaped[b] + level) + tilt[b])
    }

    /// The bands' gains (dB) that land the chain on the targets.
    pub fn gains(&self) -> [f32; N] {
        let t = self.targets();
        let solve = &layout().solve;
        std::array::from_fn(|i| solve[i].iter().zip(&t).map(|(s, t)| s * t).sum::<f32>().clamp(-GAIN_DB, GAIN_DB))
    }

    /// The Odd/Even comb's gain at harmonic `h` (a whole number).
    pub fn comb_gain(&self, h: u32) -> f32 {
        let a = self.odd_even.clamp(-1.0, 1.0);
        let sign = if h % 2 == 0 { 1.0 } else { -1.0 };
        (1.0 + a * sign).abs() / (1.0 + a.abs())
    }
}

/// The chain's gain (dB) at harmonic `h` with band gains `gains` (for the panel).
pub fn response(gains: &[f32; N], h: f32) -> f32 {
    let q = &layout().q;
    (0..N).map(|b| bell_db(h, BANDS[b], q[b], gains[b])).sum()
}

/// One peaking filter: Andrew Simper's trapezoidal SVF bell.
#[derive(Clone, Copy, Default)]
struct Bell {
    ic1: f32,
    ic2: f32,
    a1: f32,
    a2: f32,
    a3: f32,
    m1: f32,
    on: bool,
}

impl Bell {
    fn set(&mut self, hz: f32, rate: f32, q: f32, db: f32) {
        let on = db.abs() > 0.01 && hz < 0.45 * rate;
        if !on {
            if self.on {
                self.ic1 = 0.0;
                self.ic2 = 0.0;
            }
            self.on = false;
            return;
        }
        let a = 10f32.powf(db / 40.0);
        let g = (PI * hz / rate).tan();
        let k = 1.0 / (q * a);
        self.a1 = 1.0 / (1.0 + g * (g + k));
        self.a2 = g * self.a1;
        self.a3 = g * self.a2;
        self.m1 = k * (a * a - 1.0);
        self.on = true;
    }

    #[inline(always)]
    fn tick(&mut self, v0: f32) -> f32 {
        let v3 = v0 - self.ic2;
        let v1 = self.a1 * self.ic1 + self.a2 * v3;
        let v2 = self.ic2 + self.a2 * self.ic1 + self.a3 * v3;
        self.ic1 = 2.0 * v1 - self.ic1;
        self.ic2 = 2.0 * v2 - self.ic2;
        v0 + self.m1 * v1
    }
}

/// Longest comb delay (samples): half a period of 20 Hz at 192 kHz.
const COMB: usize = 8192;

/// One slot's Harmonic EQ in one voice.
pub struct HarmonicEq {
    bands: [Bell; N],
    /// The settings the bands' gains were worked out for, and those gains.
    made_for: Option<HarmonicSettings>,
    gains: [f32; N],
    comb: Box<[f32]>,
    write: usize,
}

impl Default for HarmonicEq {
    fn default() -> Self {
        Self { bands: [Bell::default(); N], made_for: None, gains: [0.0; N], comb: vec![0.0; COMB].into_boxed_slice(),
               write: 0 }
    }
}

impl HarmonicEq {
    /// Take over another's state (without allocating: the comb is copied in place).
    pub fn copy_from(&mut self, other: &HarmonicEq) {
        self.bands = other.bands;
        self.made_for = other.made_for;
        self.gains = other.gains;
        self.comb.copy_from_slice(&other.comb);
        self.write = other.write;
    }

    pub fn clear(&mut self) {
        for b in &mut self.bands {
            b.ic1 = 0.0;
            b.ic2 = 0.0;
        }
        self.comb.fill(0.0);
    }

    /// One block of the slot (`buf`, in place), at its pitch `hz`.
    pub fn process(&mut self, s: &HarmonicSettings, hz: f32, rate: f32, buf: &mut [f32]) {
        if s.bands_on() {
            if self.made_for != Some(*s) {
                self.gains = s.gains();
                self.made_for = Some(*s);
            }
            let q = &layout().q;
            for (b, bell) in self.bands.iter_mut().enumerate() {
                bell.set(hz * BANDS[b], rate, q[b], self.gains[b]);
            }
            for bell in self.bands.iter_mut().filter(|b| b.on) {
                for x in buf.iter_mut() {
                    *x = bell.tick(*x);
                }
            }
        } else if self.bands.iter().any(|b| b.on) {
            self.bands.iter_mut().for_each(|b| b.set(0.0, rate, 1.0, 0.0));
        }
        // Odd/Even: the slot and itself half a period later.
        let a = s.odd_even.clamp(-1.0, 1.0);
        let delay = (rate / (2.0 * hz.max(20.0))).clamp(2.0, (COMB - 4) as f32);
        let scale = 1.0 / (1.0 + a.abs());
        for x in buf.iter_mut() {
            self.comb[self.write] = *x;
            if a != 0.0 {
                *x = (*x + a * read(&self.comb, self.write, delay)) * scale;
            }
            self.write = (self.write + 1) % COMB;
        }
    }
}

/// The line `delay` samples before `write` (4-point Hermite).
#[inline(always)]
fn read(line: &[f32], write: usize, delay: f32) -> f32 {
    let d = delay.floor();
    let t = delay - d;
    let at = |back: isize| line[(write as isize - d as isize - back).rem_euclid(COMB as isize) as usize];
    // y0 is the newer neighbour, y3 the older; t runs from y1 toward y2 (older).
    let (y0, y1, y2, y3) = (at(-1), at(0), at(1), at(2));
    let c1 = 0.5 * (y2 - y0);
    let c2 = y0 - 2.5 * y1 + 2.0 * y2 - 0.5 * y3;
    let c3 = 0.5 * (y3 - y0) + 1.5 * (y1 - y2);
    ((c3 * t + c2) * t + c1) * t + y1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shape(name: &str) -> f32 {
        harmlib::NAMES.iter().position(|n| *n == name).unwrap() as f32 + 1.0
    }

    /// A saw at `hz` through the EQ; each harmonic's gain (dB) against the dry saw.
    pub(super) fn harmonic_gains(s: &HarmonicSettings, hz: f32, count: usize) -> Vec<f32> {
        let rate = 48_000.0;
        let n = 48_000;
        let saw = |i: usize| 2.0 * ((i as f32 * hz / rate).fract()) - 1.0;
        let mut eq = HarmonicEq::default();
        let mut out: Vec<f32> = (0..n).map(saw).collect();
        for block in out.chunks_mut(64) {
            eq.process(s, hz, rate, block);
        }
        let dry: Vec<f32> = (0..n).map(saw).collect();
        let amp = |x: &[f32], f: f32| {
            let (mut re, mut im) = (0.0f64, 0.0f64);
            for (i, v) in x[n / 2..].iter().enumerate() {
                let ph = 2.0 * std::f64::consts::PI * f as f64 * i as f64 / rate as f64;
                re += *v as f64 * ph.cos();
                im += *v as f64 * ph.sin();
            }
            (re * re + im * im).sqrt()
        };
        (1..=count).map(|h| 20.0 * (amp(&out, hz * h as f32) / amp(&dry, hz * h as f32)).log10() as f32).collect()
    }

    /// Off, it's the slot as it is.
    #[test]
    fn off_is_transparent() {
        let s = HarmonicSettings { depth: 1.0, ..Default::default() };
        assert!(!s.on());
        let g = harmonic_gains(&s, 220.0, 20);
        assert!(g.iter().all(|d| d.abs() < 0.01), "{g:?}");
    }

    /// The chain lands on its targets at the band centres, on a saw's partials.
    #[test]
    fn lands_on_the_shape() {
        for name in ["Amp 01", "EQ Lows 02", "Pulse 02", "Voice 01"] {
            for shift in [-1.0, 0.0, 1.5] {
                let s = HarmonicSettings { shape: shape(name), shift, depth: 1.0, ..Default::default() };
                let t = s.targets();
                let g = harmonic_gains(&s, 110.0, 64);
                for (b, r) in BANDS.iter().enumerate() {
                    let got = g[*r as usize - 1];
                    // Deep cuts are hard to land exactly (and inaudible); the rest within 3 dB.
                    if t[b] > -18.0 {
                        assert!((got - t[b]).abs() < 3.0, "{name} shift {shift}: band {r} got {got:.1}, want {:.1}", t[b]);
                    }
                }
            }
        }
    }

    /// Odd/Even at -1 keeps only the odd harmonics, at +1 only the even.
    #[test]
    fn odd_even() {
        let odd = harmonic_gains(&HarmonicSettings { odd_even: -1.0, depth: 1.0, ..Default::default() }, 220.0, 8);
        assert!(odd[1] < -30.0 && odd[3] < -30.0 && odd[0].abs() < 0.5 && odd[2].abs() < 0.5, "{odd:?}");
        let even = harmonic_gains(&HarmonicSettings { odd_even: 1.0, depth: 1.0, ..Default::default() }, 220.0, 8);
        assert!(even[0] < -30.0 && even[2] < -30.0 && even[1].abs() < 0.5, "{even:?}");
    }

    /// Shapes keep a saw about as loud as it was.
    #[test]
    fn level_holds() {
        let mut off = Vec::new();
        for (k, _) in harmlib::NAMES.iter().enumerate() {
            let s = HarmonicSettings { shape: k as f32 + 1.0, depth: 1.0, ..Default::default() };
            let g = harmonic_gains(&s, 110.0, 64);
            let power: f32 = g.iter().enumerate().map(|(h, d)| 10f32.powf(d / 10.0) / ((h + 1) * (h + 1)) as f32).sum();
            let total: f32 = (1..=64).map(|h| 1.0 / (h * h) as f32).sum();
            off.push(10.0 * (power / total).log10());
        }
        off.sort_by(f32::total_cmp);
        let median = off[off.len() / 2];
        assert!(median.abs() < 2.0 && off[off.len() / 10] > -9.0 && off[off.len() * 9 / 10] < 6.0,
                "level change: median {median:.1}, 10% {:.1}, 90% {:.1}", off[off.len() / 10], off[off.len() * 9 / 10]);
    }
}

#[cfg(test)]
mod compare {
    use super::*;

    /// Each harmonic's gain (dB) of a saw at C3 through the EQ, for ABD_HARM
    /// ("shape,shift;shape,shift", shape 1-based), one line per setting:
    /// for comparing with Thorn. `cargo test --release dump_gains -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn dump_gains() {
        let Ok(list) = std::env::var("ABD_HARM") else { return };
        for item in list.split(';') {
            let (shape, shift) = item.split_once(',').unwrap();
            let s = HarmonicSettings { shape: shape.parse().unwrap(), shift: shift.parse().unwrap(), depth: 1.0,
                                       ..Default::default() };
            let g = super::tests::harmonic_gains(&s, 130.81, 80);
            println!("{}", g.iter().map(|v| format!("{v:.2}")).collect::<Vec<_>>().join(" "));
        }
    }
}

#[cfg(test)]
mod bench {
    use super::*;

    /// µs per 64-sample block of one slot's EQ (all bands and the comb on;
    /// the shape's Shift moving, so the gains are worked out every block):
    /// `cargo test --release harmonic_cost -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn harmonic_cost() {
        let mut eq = HarmonicEq::default();
        let mut buf = [0.1f32; 64];
        let blocks = 20_000;
        let t = std::time::Instant::now();
        for i in 0..blocks {
            let s = HarmonicSettings { shape: 1.0, shift: (i as f32 * 0.001).sin(), tilt: 1.0, depth: 1.0, odd_even: 0.3 };
            eq.process(&s, 110.0, 48_000.0, &mut buf);
        }
        let us = t.elapsed().as_secs_f64() * 1e6 / blocks as f64;
        println!("{us:.2} us per block per slot (a block lasts 1333 us)");
        assert!(buf.iter().all(|v| v.is_finite()));
    }
}
