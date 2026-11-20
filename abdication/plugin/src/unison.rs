//! Unison's layout and its Eco mode.
//!
//! The layout is the Virus TI's, from OsTIrus measurements
//! (abdicant-syn/research/synth-character/ostirus.md): the copies sit at
//! evenly spaced positions q from −1 to +1, each weighted 1 − |q|/4, so the
//! outer pair is only 2.5 dB under the centre. Detune (0–127) follows the
//! Virus's measured curve for the outer pair, which narrows up the keyboard
//! (about 0.85× per octave). Spread (0–127) pans with its clipped linear
//! law, −4 dB in the middle; the outer pair reaches the sides at 64.
//!
//! Eco plays only the centre copy, and the others from snapshots of it:
//! every `REFRESH` cycles one cycle of the centre's output is resampled into
//! a table, and each side copy reads that table at its own detuned pitch,
//! crossfading to each new snapshot over `FADE` cycles.

use crate::voice::Unison;

/// The outer pair's measured frequency offset (a fraction of the note's)
/// at A4 for each Detune setting (ostirus/paired_unison.hpp).
const OUTER: [f32; 128] = [
    0.0, 0.0, 1.39691793813e-05, 1.39691793813e-05, 4.17714580541e-05, 4.17714580541e-05, 5.56047877915e-05,
    8.35430012983e-05, 9.73758797582e-05, 9.73758797582e-05, 9.73758797582e-05, 0.000125178477093, 0.000125178477093,
    0.00013914747611, 0.000166949929299, 0.000180783206352, 0.000180783206352, 0.000222554545321, 0.000264190203927,
    0.000291992537075, 0.000347597298941, 0.000375399846465, 0.000417171081933, 0.000458942238481, 0.000472775604606,
    0.000514547528512, 0.000556182510505, 0.000597953991202, 0.000625756358933, 0.000681361002323, 0.00070916308076,
    0.000750934731011, 0.000792570262516, 0.000834341842061, 0.000875977603991, 0.000931582043495, 0.000973353386963,
    0.00104279179152, 0.00108456292968, 0.00112633386916, 0.00118193855538, 0.00122357411867, 0.0012931479289,
    0.00133478343089, 0.00137655537267, 0.00143215961119, 0.00147393413329, 0.00154336767638, 0.0015851409155,
    0.00162677613978, 0.00168238097373, 0.00172415245189, 0.00179372617537, 0.00183536180076, 0.0018771330694,
    0.00193273770799, 0.00197437362698, 0.00204394671344, 0.00208558269274, 0.0021273541884, 0.00218295881684,
    0.00222473018348, 0.00229416823511, 0.00233593968804, 0.00237757533983, 0.00243317992126, 0.00247495128329,
    0.00251672254202, 0.00258616065597, 0.00262793186211, 0.00268353661357, 0.00272517222209, 0.00276694364256,
    0.00283651726829, 0.00287815304365, 0.0029337576597, 0.00297552896219, 0.00301716499226, 0.00308673880017,
    0.0031283741542, 0.00318411414108, 0.00329532370638, 0.00343433558545, 0.00354554491157, 0.00368455658025,
    0.0037957659573, 0.00393491320846, 0.00404612258836, 0.0041851344999, 0.00429634369248, 0.00443535590848,
    0.00454670212823, 0.00468571221867, 0.00479692156087, 0.0049359332968, 0.00504714272727, 0.00518615462257,
    0.00535310403357, 0.00551991826556, 0.0056867323196, 0.00588134853934, 0.00604829840845, 0.00621511242504,
    0.0063958951919, 0.00656270929292, 0.0067573260432, 0.00692427551513, 0.00710492294112, 0.00727187292071,
    0.00743868694743, 0.00760550080429, 0.00780011714264, 0.00796706701187, 0.00827289265252, 0.00856488530015,
    0.00885687740809, 0.00917667236528, 0.00946866485219, 0.00977449052968, 0.0100526495281, 0.0103584755519,
    0.0106504684395, 0.0109702625285, 0.0112622551464, 0.0115542474418, 0.0118600732042, 0.0121520654219,
    0.0124440582932,
];

/// The measured maximum spread at MIDI 33, 45, 57, 69, 81 and 93.
const PITCH_ANCHORS: [f32; 6] = [0.02025451694, 0.01733746123, 0.01456756229, 0.01245771590, 0.01054398144,
                                 0.008971630423];

/// The Virus's centre pan gain: −4 dB.
const CENTRE_GAIN: f32 = 0.630_957_3;

/// The outer pair's offset (a fraction of the note's frequency) for a Detune
/// setting (0–127, between settings by straight lines).
pub fn outer_fraction(detune: f32) -> f32 {
    let d = detune.clamp(0.0, 127.0);
    let low = d as usize;
    let high = (low + 1).min(127);
    OUTER[low] + (d - low as f32) * (OUTER[high] - OUTER[low])
}

/// The Detune setting whose outer pair sits `cents` out at A4 (the old
/// knob's meaning), held at 127 past the widest.
pub fn detune_for_cents(cents: f32) -> f32 {
    let target = (cents.max(0.0) / 1200.0).exp2() - 1.0;
    match OUTER.iter().position(|&f| f >= target) {
        None => 127.0,
        Some(0) => 0.0,
        Some(i) => (i - 1) as f32 + (target - OUTER[i - 1]) / (OUTER[i] - OUTER[i - 1]),
    }
}

/// A Detune setting as the panel shows it: the setting, then the outer
/// pair's offset at A4 ("64 · 4.1 ct").
pub fn detune_name(detune: f32) -> String {
    format!("{:.0} · {:.1} ct", detune, 1200.0 * (1.0 + outer_fraction(detune)).log2())
}

/// How much wider (below A4) or narrower (above) the spread is at a pitch:
/// the measured anchors, interpolated in log, held past their ends.
pub fn pitch_scale(hz: f32) -> f32 {
    let note = 69.0 + 12.0 * (hz.max(1.0) / 440.0).log2();
    let x = ((note - 33.0) / 12.0).clamp(0.0, 5.0);
    let a = (x as usize).min(4);
    let t = x - a as f32;
    (PITCH_ANCHORS[a].ln() * (1.0 - t) + PITCH_ANCHORS[a + 1].ln() * t).exp() / PITCH_ANCHORS[3]
}

/// One playing copy: which of the seven, its pitch against the note's,
/// and its gains into the voice's left and right.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Place {
    pub copy: usize,
    pub ratio: f32,
    pub left: f32,
    pub right: f32,
}

/// Where each playing copy goes, and how many there are.
pub struct Layout {
    pub places: [Place; 7],
    pub count: usize,
}

impl Layout {
    pub fn places(&self) -> &[Place] {
        &self.places[..self.count]
    }

    /// Whether any copy sounds differently on the two sides.
    pub fn stereo(&self) -> bool {
        self.places().iter().any(|p| p.left != p.right)
    }
}

/// The layout for a mode, Detune and Spread (0–127) and the note's pitch.
/// Unison off is the centre copy alone at full level. With it on, the
/// copies are scaled so that, at Spread 0, the sum is as loud as one copy
/// (the copies being uncorrelated), and widening keeps the Virus's law.
pub fn layout(mode: Unison, detune: f32, spread: f32, note_hz: f32) -> Layout {
    let copies = mode.copies();
    let count = copies.len();
    let mut places = [Place::default(); 7];
    if count == 1 {
        places[0] = Place { copy: copies[0], ratio: 1.0, left: 1.0, right: 1.0 };
        return Layout { places, count };
    }
    let half = (count / 2) as f32;
    let amount = outer_fraction(detune) * pitch_scale(note_hz);
    let width = spread.clamp(0.0, 127.0) / 127.0;
    let q = |i: usize| (i as f32 - half) / half;
    let weight = |i: usize| 1.0 - 0.25 * q(i).abs();
    let power: f32 = (0..count).map(|i| weight(i).powi(2)).sum();
    let norm = 1.0 / (CENTRE_GAIN * power.sqrt());
    for (i, &copy) in copies.iter().enumerate() {
        let pan = (2.0 * q(i) * width).clamp(-1.0, 1.0);
        let (left, right) = if pan <= 0.0 {
            (CENTRE_GAIN - (1.0 - CENTRE_GAIN) * pan, CENTRE_GAIN * (1.0 + pan))
        } else {
            (CENTRE_GAIN * (1.0 - pan), CENTRE_GAIN + (1.0 - CENTRE_GAIN) * pan)
        };
        let w = weight(i) * norm;
        places[i] = Place { copy, ratio: 1.0 + q(i) * amount, left: w * left, right: w * right };
    }
    Layout { places, count }
}

// ---- Eco ------------------------------------------------------------------

/// Points in a snapshot of one cycle (enough for every harmonic of a
/// 20 Hz note up to 20 kHz).
const TABLE: usize = 2048;
/// Cycles between snapshots.
pub const REFRESH: u32 = 12;
/// Cycles each new snapshot fades in over.
const FADE: f32 = 2.0;
/// The lowest pitch the ring holds a whole cycle of.
const LOWEST_HZ: f32 = 20.0;

/// One cycle, with three points of wrap-around for the cubic read.
type Table = Box<[f32]>;

/// A cubic (Hermite) read at `x` between y1 and y2.
fn hermite(y0: f32, y1: f32, y2: f32, y3: f32, x: f32) -> f32 {
    let c1 = 0.5 * (y2 - y0);
    let c2 = y0 - 2.5 * y1 + 2.0 * y2 - 0.5 * y3;
    let c3 = 0.5 * (y3 - y0) + 1.5 * (y1 - y2);
    ((c3 * x + c2) * x + c1) * x + y1
}

/// A table at phase `p` (0..1).
fn read(table: &[f32], p: f32) -> f32 {
    let x = p * TABLE as f32;
    let i = (x as usize).min(TABLE - 1);
    // The table's point 0 sits at index 1, so i - 1 is always there.
    hermite(table[i], table[i + 1], table[i + 2], table[i + 3], x - i as f32)
}

/// One oscillator slot's snapshots in one voice, and its side copies' phases.
pub struct Snapshots {
    /// The centre's output, the last `ring.len()` samples, by sample time.
    ring: Box<[f32]>,
    mask: usize,
    /// Samples since note-on.
    time: usize,
    /// The centre's cycles since note-on (counted at its pitch).
    cycles: f64,
    /// Cycles since the last snapshot.
    since: u32,
    /// A cycle ended (at that sample time, that long) and waits for the two
    /// samples after it that the cubic read needs.
    pending: Option<(f64, f64)>,
    /// The newest snapshot and the one before (or none: the centre itself).
    tables: [Table; 2],
    newest: usize,
    snapshots: u32,
    /// How far the newest has faded in (0..1), and by how much a sample.
    fade: f32,
    fade_step: f32,
    /// Each copy's phase in its reading of the table (f64: it runs for the
    /// whole note).
    phases: [f64; 7],
}

impl Snapshots {
    pub fn new(sample_rate: f32) -> Self {
        let len = ((sample_rate / LOWEST_HZ) as usize + 8).next_power_of_two();
        let table = || vec![0.0; TABLE + 3].into_boxed_slice();
        Self { ring: vec![0.0; len].into_boxed_slice(), mask: len - 1, time: 0, cycles: 0.0, since: 0,
               pending: None, tables: [table(), table()], newest: 0, snapshots: 0, fade: 1.0, fade_step: 0.0,
               phases: [0.0; 7] }
    }

    /// A new note: no snapshot yet; each copy starts at its phase.
    pub fn start(&mut self, phases: [f32; 7]) {
        self.time = 0;
        self.cycles = 0.0;
        self.since = 0;
        self.pending = None;
        self.snapshots = 0;
        self.fade = 1.0;
        self.phases = phases.map(f64::from);
    }

    /// The ring at fractional sample time `t` (cubic).
    fn at(&self, t: f64) -> f32 {
        let i = t.floor();
        let x = (t - i) as f32;
        let i = i as usize;
        let r = |k: usize| self.ring[k & self.mask];
        hermite(r(i.wrapping_sub(1)), r(i), r(i + 1), r(i + 2), x)
    }

    /// Resample the cycle that ended at `end`, `period` samples long, into
    /// the older table, which becomes the newest.
    fn take(&mut self, end: f64, period: f64) {
        let start = end - period;
        let older = 1 - self.newest;
        for j in 0..TABLE {
            let v = self.at(start + period * j as f64 / TABLE as f64);
            self.tables[older][j + 1] = v;
        }
        let t = &mut self.tables[older];
        t[0] = t[TABLE];
        t[TABLE + 1] = t[1];
        t[TABLE + 2] = t[2];
        self.newest = older;
        self.snapshots += 1;
        self.fade = 0.0;
        self.fade_step = 1.0 / (FADE * period as f32).max(64.0);
        self.since = 0;
    }

    /// One block: the centre's output (`centre`, at its pitch `hz`), and the
    /// side copies (`places` but the centre) added into `left` and `right`
    /// at their gains (ramped from `from`, the gains the last block ended
    /// on). The centre itself isn't added.
    pub fn play(&mut self, centre: &[f32], hz: &[f32], rate: f32, places: &[Place], from: &[Place],
                centre_copy: usize, left: &mut [f32], right: &mut [f32]) {
        let n = centre.len();
        let ramp = 1.0 / n as f32;
        for i in 0..n {
            let x = centre[i];
            self.ring[self.time & self.mask] = x;
            let step = (hz[i].max(1.0) / rate) as f64;
            let before = self.cycles;
            self.cycles += step;
            if self.cycles.floor() > before.floor() {
                self.since += 1;
                if self.snapshots == 0 || self.since >= REFRESH {
                    // The cycle ended between this sample and the next.
                    let ago = self.cycles.fract() / step;
                    self.pending = Some(((self.time + 1) as f64 - ago, 1.0 / step));
                }
            }
            if let Some((end, period)) = self.pending {
                if self.time as f64 >= end.floor() + 2.0 {
                    self.pending = None;
                    self.take(end, period);
                }
            }
            self.time += 1;

            let fade = self.fade;
            self.fade = (self.fade + self.fade_step).min(1.0);
            let newest = &self.tables[self.newest];
            let older = &self.tables[1 - self.newest];
            let mix = (i as f32 + 1.0) * ramp;
            for (k, place) in places.iter().enumerate().filter(|(_, p)| p.copy != centre_copy) {
                let p = &mut self.phases[place.copy];
                let at = *p as f32;
                let wave = match self.snapshots {
                    0 => x,
                    1 => fade * read(newest, at) + (1.0 - fade) * x,
                    _ => fade * read(newest, at) + (1.0 - fade) * read(older, at),
                };
                *p += step * place.ratio as f64;
                *p -= p.floor();
                let was = from[k];
                left[i] += wave * (was.left + (place.left - was.left) * mix);
                right[i] += wave * (was.right + (place.right - was.right) * mix);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layout_matches_the_virus() {
        // Five copies: weights 0.75, 0.875, 1, as measured; linear positions.
        let five = layout(Unison::U5, 127.0, 0.0, 440.0);
        let w: Vec<f32> = five.places().iter().map(|p| p.left).collect();
        assert!((w[0] / w[2] - 0.75).abs() < 1e-5 && (w[1] / w[2] - 0.875).abs() < 1e-5);
        // Full Detune at A4: the outer pair 21.5 cents out, the inner half that.
        let cents = |r: f32| 1200.0 * r.log2();
        assert!((cents(five.places[4].ratio) - 21.5).abs() < 0.2, "{}", cents(five.places[4].ratio));
        assert!((cents(five.places[3].ratio) - 10.8).abs() < 0.2);
        // Narrower an octave up, wider an octave down (the measured anchors).
        let up = layout(Unison::U7, 127.0, 0.0, 880.0).places[6].ratio - 1.0;
        let down = layout(Unison::U7, 127.0, 0.0, 220.0).places[6].ratio - 1.0;
        let here = five.places[4].ratio - 1.0;
        assert!((up / here - 0.846).abs() < 0.01 && (down / here - 1.169).abs() < 0.01);
        // At Spread 0 the copies sum (uncorrelated) to one copy's level, and
        // nothing is panned.
        for mode in [Unison::U5, Unison::U7] {
            let l = layout(mode, 64.0, 0.0, 440.0);
            assert!(!l.stereo());
            let power: f32 = l.places().iter().map(|p| p.left * p.left).sum();
            assert!((power - 1.0).abs() < 1e-4);
        }
        // Spread: the outer pair hard left and right from 64; the -4 dB centre.
        let wide = layout(Unison::U7, 0.0, 64.0, 440.0);
        assert_eq!(wide.places[0].right, 0.0);
        assert_eq!(wide.places[6].left, 0.0);
        assert!(wide.places[5].left > 0.0);
        let c = wide.places[3];
        assert!((c.left / wide.places[6].right - CENTRE_GAIN / 0.75).abs() < 1e-5);
        // Off: the centre copy at full level.
        let off = layout(Unison::Off, 127.0, 127.0, 440.0);
        assert_eq!(off.places(), &[Place { copy: 3, ratio: 1.0, left: 1.0, right: 1.0 }]);
    }

    #[test]
    fn old_detune_converts() {
        for cents in [0.5f32, 2.0, 6.0, 12.0, 21.0] {
            let ratio = 1.0 + outer_fraction(detune_for_cents(cents));
            assert!((1200.0 * ratio.log2() - cents).abs() < 0.01, "{cents}");
        }
        assert_eq!(detune_for_cents(0.0), 0.0);
        assert_eq!(detune_for_cents(50.0), 127.0);
    }

    /// A periodic wave comes back out of the snapshots as it went in: a copy
    /// at the centre's pitch and phase matches the centre once faded in.
    #[test]
    fn snapshots_replay_the_cycle() {
        let rate = 48_000.0;
        let hz = 187.3f32;
        let mut s = Snapshots::new(rate);
        s.start([0.0; 7]);
        let place = Place { copy: 0, ratio: 1.0, left: 1.0, right: 0.0 };
        let wave = |t: f32| {
            let p = (t * hz / rate).fract();
            (std::f32::consts::TAU * p).sin() + 0.3 * (3.0 * std::f32::consts::TAU * p).sin()
        };
        let mut worst = 0.0f32;
        for b in 0..200 {
            let centre: Vec<f32> = (0..64).map(|i| wave((b * 64 + i) as f32)).collect();
            let (mut l, mut r) = ([0.0; 64], [0.0; 64]);
            s.play(&centre, &[hz; 64], rate, &[place], &[place], 3, &mut l, &mut r);
            // From the second snapshot on (the first one's first point reaches before the note).
            if b > 64 {
                for i in 0..64 {
                    worst = worst.max((l[i] - centre[i]).abs());
                }
            }
        }
        assert!(s.snapshots >= 4, "{} snapshots", s.snapshots);
        assert!(worst < 2e-3, "the replayed cycle is off by {worst}");
    }
}
