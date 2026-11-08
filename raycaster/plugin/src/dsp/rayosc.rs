//! One ray-cast oscillator: the bent shape turns once per cycle about its center and the
//! ray reads it once a sample. Every leap is smoothed with a polyBLEP placed where the leap
//! really happens, found by bisecting the turn between two samples. The voice upsamples
//! afterwards for its drive and filter.
//!
//! Width puts the right channel a little further round the turn. Rather than cast a second
//! ray, it reads the left channel's own past: a periodic wave a fraction w of a turn ahead
//! is the same wave (1 - w) of a period behind.
//!
//! The raw wave sits off centre (inside a shape every hit is above zero), so its mean over
//! the last period is taken away before it goes anywhere: otherwise the drive saturates on
//! the offset alone and the notes thump.
//!
//! That mean and the width both look a period back, so a note can't start on an empty (or
//! another shape's) past: `prime` fills one period of history from a quick sketch of the turn.

use super::geometry::{turn_sin_cos, Bend, Bent, Outline, Point};

/// Everything one block of one oscillator needs, already modulated.
#[derive(Clone, Copy, Debug)]
pub struct OscBlock {
    pub bend: Bend,
    /// How far right of the bent shape's centroid it turns.
    pub center_x: f32,
    pub probe: f32,
    pub orbit_depth: f32,
    /// Orbit turns per sample.
    pub orbit_step: f32,
    /// Shape turns per sample at the start and end of the block.
    pub step: [f32; 2],
    /// The right channel sits this far round the turn (0..0.25).
    pub width_turn: f32,
    pub level: [f32; 2],
}

/// Long enough for a period at 11 Hz at 48 kHz.
const HISTORY: usize = 4096;
const BISECTIONS: usize = 4;
const REACH_EVERY: usize = 4;
/// Rays in the sketch of one turn that primes the history.
const SKETCH: usize = 128;

pub struct RayOsc {
    bent: Box<Bent>,
    pub turn: f32,
    pub orbit: f32,
    previous: f32,
    pending: f32,
    level: f32,
    history: Box<[f32; HISTORY]>,
    /// Running sum of the raw wave, so the mean over any recent span is one subtraction.
    sums: Box<[f64; HISTORY]>,
    total: f64,
    written: usize,
    /// Nothing has sounded since the last `clear`.
    cleared: bool,
}

impl Default for RayOsc {
    fn default() -> Self {
        Self {
            bent: Box::default(),
            turn: 0.0,
            orbit: 0.0,
            previous: 0.0,
            pending: 0.0,
            level: 0.0,
            history: Box::new([0.0; HISTORY]),
            sums: Box::new([0.0; HISTORY]),
            total: 0.0,
            written: 0,
            cleared: true,
        }
    }
}

impl RayOsc {
    pub fn restart(&mut self) {
        self.set_phase(0.0, 0.0);
    }

    /// Where the turn and the orbit start (0..1 each).
    pub fn set_phase(&mut self, turn: f32, orbit: f32) {
        self.turn = turn.rem_euclid(1.0);
        self.orbit = orbit.rem_euclid(1.0);
    }

    pub fn clear(&mut self) {
        self.previous = 0.0;
        self.pending = 0.0;
        self.history.fill(0.0);
        self.sums.fill(0.0);
        self.total = 0.0;
        self.cleared = true;
    }

    /// Writes the last period as this block's shape would have played it, so the centring
    /// and the width are right from the first sample. A fresh oscillator also starts its
    /// leap smoothing there; one still sounding keeps its own, so its output doesn't jump.
    pub fn prime(&mut self, outline: &Outline, b: &OscBlock) {
        self.bent.bend(outline, b.bend);
        let center = self.center(b);
        let scale = 2.0 / self.bent.reach(center);
        let sketch: [f32; SKETCH + 1] = std::array::from_fn(|j| {
            self.bent.read((j % SKETCH) as f32 / SKETCH as f32, center, scale, b.probe)
        });
        let wave = |turn: f32| {
            let at = (turn - turn.floor()) * SKETCH as f32;
            let j = (at as usize).min(SKETCH - 1);
            sketch[j] + (sketch[j + 1] - sketch[j]) * (at - j as f32)
        };
        let step = b.step[1].max(1e-6);
        let newest = self.written + HISTORY - 1;
        // Back 0 is the sample just before this turn; the running sum ends at 0 there. The
        // whole ring is rewritten, not just a period: a pitch falling across the next block
        // reads further back, and old running sums there are on another scale entirely.
        self.total = 0.0;
        let mut sum = 0.0f64;
        for back in 0..HISTORY - 1 {
            let i = (newest - back) % HISTORY;
            let x = wave(self.turn - (back + 1) as f32 * step);
            self.history[i] = x;
            self.sums[i] = -sum;
            sum += x as f64;
        }
        if self.cleared {
            self.previous = self.history[newest % HISTORY];
            self.pending = self.previous;
        }
    }

    pub fn silent(&self) -> bool {
        self.level < 1e-5
    }

    pub fn bent(&self) -> &Bent {
        &self.bent
    }

    fn center(&self, b: &OscBlock) -> Point {
        let base = [self.bent.centroid[0] + b.center_x, self.bent.centroid[1]];
        if b.orbit_depth > 0.0 {
            let (s, c) = turn_sin_cos(self.orbit);
            [base[0] + b.orbit_depth * c, base[1] + b.orbit_depth * s]
        } else {
            base
        }
    }

    /// The sample at `turn`, with the leap since the last one (if any) smoothed. Output is
    /// one sample late, so the step before a leap can be corrected too.
    #[inline]
    fn sample(&mut self, turn: f32, step: f32, center: Point, scale: f32, probe: f32) -> f32 {
        let value = self.bent.read(turn, center, scale, probe);
        let mut now = value;
        // A smooth wave moves at most about 2 pi x step per sample.
        let threshold = 0.02 + 7.0 * step;
        if (value - self.previous).abs() > threshold {
            let (mut lo, mut hi, mut v_lo, mut v_hi) = (0.0f32, 1.0f32, self.previous, value);
            for _ in 0..BISECTIONS {
                let mid = 0.5 * (lo + hi);
                let v_mid = self.bent.read(turn - step * (1.0 - mid), center, scale, probe);
                if (v_mid - v_lo).abs() < (v_hi - v_mid).abs() {
                    lo = mid;
                    v_lo = v_mid;
                } else {
                    hi = mid;
                    v_hi = v_mid;
                }
            }
            let d = 0.5 * (lo + hi);
            let h = v_hi - v_lo;
            self.pending += 0.5 * h * (1.0 - d) * (1.0 - d);
            now -= 0.5 * h * d * d;
        }
        let out = self.pending;
        self.pending = now;
        self.previous = value;
        out
    }

    /// The left channel's value `delay` samples ago (4-point Hermite).
    #[inline]
    fn past(&self, delay: f32) -> f32 {
        let delay = delay.clamp(1.0, (HISTORY - 3) as f32);
        let whole = delay.floor();
        let f = delay - whole;
        let newest = self.written + HISTORY - 1;
        let at = |back: usize| self.history[(newest - back) % HISTORY];
        let d = whole as usize;
        let (xm1, x0, x1, x2) = (at(d - 1), at(d), at(d + 1), at(d + 2));
        // Interpolating from x0 (d back) toward x1 (d + 1 back).
        let c1 = 0.5 * (x1 - xm1);
        let c2 = xm1 - 2.5 * x0 + 2.0 * x1 - 0.5 * x2;
        let c3 = 0.5 * (x2 - xm1) + 1.5 * (x0 - x1);
        ((c3 * f + c2) * f + c1) * f + x0
    }

    /// The raw wave's mean over the last `period` samples.
    #[inline]
    fn mean(&self, period: f32) -> f32 {
        let p = period.clamp(2.0, (HISTORY - 3) as f32);
        let whole = p.floor();
        let f = (p - whole) as f64;
        let newest = self.written + HISTORY - 1;
        let at = |back: usize| self.sums[(newest - back) % HISTORY];
        let d = whole as usize;
        let before = at(d) + (at(d + 1) - at(d)) * f;
        ((self.total - before) / p as f64) as f32
    }

    /// Bends the shape for this block, then adds its sound into `left` and `right`.
    pub fn render(&mut self, outline: &Outline, b: &OscBlock, left: &mut [f32], right: &mut [f32]) {
        let n = left.len().min(right.len());
        self.bent.bend(outline, b.bend);
        let orbiting = b.orbit_depth > 0.0;
        // While orbiting, the farthest point is found every few samples and the scale
        // interpolated between: it moves smoothly, and finding it is most of the cost.
        let mut scale = 2.0 / self.bent.reach(self.center(b));
        let mut scale_step = 0.0;
        let stereo = b.width_turn > 1e-4;
        for i in 0..n {
            let f = (i as f32 + 0.5) / n as f32;
            let step = b.step[0] + (b.step[1] - b.step[0]) * f;
            let level = b.level[0] + (b.level[1] - b.level[0]) * f;
            let center = self.center(b);
            if orbiting {
                if i % REACH_EVERY == 0 {
                    let ahead = self.orbit + b.orbit_step * REACH_EVERY as f32;
                    let (s, c) = turn_sin_cos(ahead);
                    let base = [self.bent.centroid[0] + b.center_x, self.bent.centroid[1]];
                    let later = [base[0] + b.orbit_depth * c, base[1] + b.orbit_depth * s];
                    scale = 2.0 / self.bent.reach(center);
                    scale_step = (2.0 / self.bent.reach(later) - scale) / REACH_EVERY as f32;
                } else {
                    scale += scale_step;
                }
            }
            let l = self.sample(self.turn, step, center, scale, b.probe);
            self.history[self.written % HISTORY] = l;
            self.total += l as f64;
            self.sums[self.written % HISTORY] = self.total;
            self.written = (self.written + 1) % HISTORY;
            let period = 1.0 / step.max(1e-6);
            let r = if stereo { self.past((1.0 - b.width_turn) * period) } else { l };
            let centre = self.mean(period);
            let (l, r) = (l - centre, r - centre);
            left[i] += l * level;
            right[i] += r * level;
            self.turn += step;
            self.turn -= self.turn.floor();
            self.orbit += b.orbit_step;
            self.orbit -= self.orbit.floor();
        }
        self.level = b.level[1];
        self.cleared = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dsp::shapes::Element;

    fn block(step: f32) -> OscBlock {
        OscBlock {
            bend: Bend::default(),
            center_x: 0.0,
            probe: 0.0,
            orbit_depth: 0.0,
            orbit_step: 0.0,
            step: [step, step],
            width_turn: 0.0,
            level: [1.0, 1.0],
        }
    }

    #[test]
    fn priming_leaves_no_stale_past() {
        // Primed for a short period, then read a long period back (a pitch falling across
        // the block): nothing from before the priming may be reached.
        let outline = Element::Fire.outline();
        let mut osc = RayOsc::default();
        let (mut l, mut r) = (vec![0.0; 48_000], vec![0.0; 48_000]);
        let low = OscBlock { probe: 0.5, ..block(110.0 / 48_000.0) };
        osc.render(&outline, &low, &mut l, &mut r);
        let high = OscBlock { probe: 0.5, ..block(440.0 / 48_000.0) };
        osc.prime(&outline, &high);
        let falling = OscBlock { step: [110.0 / 48_000.0, 440.0 / 48_000.0], ..high };
        let (mut l, mut r) = (vec![0.0; 32], vec![0.0; 32]);
        osc.render(&outline, &falling, &mut l, &mut r);
        let worst = l.iter().chain(&r).fold(0.0f32, |a, x| a.max(x.abs()));
        assert!(worst < 4.0, "{worst}");
    }

    #[test]
    fn a_centred_square_repeats_four_times_a_turn() {
        let outline = Element::Air.outline();
        let mut osc = RayOsc::default();
        let n = 4096;
        let (mut l, mut r) = (vec![0.0; 2 * n], vec![0.0; 2 * n]);
        osc.render(&outline, &block(1.0 / n as f32), &mut l, &mut r);
        // The second turn, once the centring has a whole period behind it.
        let err: f32 = (n..n + n / 2).map(|i| (l[i] - l[i + n / 4]).abs()).fold(0.0, f32::max);
        assert!(err < 1e-3, "{err}");
    }

    #[test]
    fn leaps_are_smoothed_not_removed() {
        // The ring (Earth) turned about a point in its wall leaps as the ray crosses the hole.
        let outline = Element::Earth.outline();
        let mut osc = RayOsc::default();
        let mut b = block(1.0 / 400.0);
        b.center_x = 0.75;
        let n = 4000;
        let (mut l, mut r) = (vec![0.0; n], vec![0.0; n]);
        osc.render(&outline, &b, &mut l, &mut r);
        assert!(l.iter().all(|v| v.is_finite() && v.abs() <= 1.2));
        let biggest = l.windows(2).map(|w| (w[1] - w[0]).abs()).fold(0.0, f32::max);
        let span = l.iter().cloned().fold(f32::MIN, f32::max) - l.iter().cloned().fold(f32::MAX, f32::min);
        assert!(span > 0.3, "{span}");
        assert!(biggest < 0.8 * span, "a leap of {biggest} out of {span} went through untouched");
    }

    #[test]
    fn the_wave_is_centred() {
        let outline = Element::Earth.outline();
        let mut osc = RayOsc::default();
        let mut b = block(1.0 / 300.0);
        b.center_x = 0.3;
        let n = 6000;
        let (mut l, mut r) = (vec![0.0; n], vec![0.0; n]);
        osc.render(&outline, &b, &mut l, &mut r);
        let tail = &l[3000..];
        let mean = tail.iter().sum::<f32>() / tail.len() as f32;
        let span = tail.iter().cloned().fold(f32::MIN, f32::max) - tail.iter().cloned().fold(f32::MAX, f32::min);
        assert!(mean.abs() < 0.01 * span, "mean {mean} of {span}");
    }

    #[test]
    fn width_is_the_same_wave_further_round() {
        let outline = Element::Fire.outline();
        let mut osc = RayOsc::default();
        let period = 200.0;
        let mut b = block(1.0 / period);
        b.center_x = 0.2;
        b.width_turn = 0.25;
        let n = 2000;
        let (mut l, mut r) = (vec![0.0; n], vec![0.0; n]);
        osc.render(&outline, &b, &mut l, &mut r);
        // The right channel now equals the left a quarter period later.
        let err = (1000..1700).map(|i| (r[i] - l[i + 50]).abs()).fold(0.0, f32::max);
        let span = l.iter().cloned().fold(f32::MIN, f32::max) - l.iter().cloned().fold(f32::MAX, f32::min);
        assert!(err < 0.02 * span, "{err} of {span}");
    }
}
