//! Outlines, the three bends (squish, skew, tilt) and the ray.
//!
//! The shape turns clockwise about a center, scaled so its farthest point is 2 away. A ray
//! starts at (offset, 2) and falls straight down. The wave is the height it hits, halved, so
//! hits read -1..1. A ray that reaches the floor at y = -2 without a hit reads 0, the
//! center's height. Overlapping outlines need no special handling: the first hit from above
//! always lands on the outside edge of the shapes combined.

use wide::{f32x8, CmpGe, CmpLe, CmpLt};

pub type Point = [f32; 2];

/// Enough for three of the most detailed elements overlapped.
pub const MAX_POINTS: usize = 640;
pub const MAX_LOOPS: usize = 16;

/// The probe sits this far above the center, in the shape's scaled units.
pub const PROBE_HEIGHT: f32 = 2.0;
/// From the probe to the floor.
pub const REACH: f32 = 4.0;

/// An outline stored flat: every loop's points one after another, and where each loop ends.
#[derive(Clone, Debug, Default)]
pub struct Outline {
    pub xs: Vec<f32>,
    pub ys: Vec<f32>,
    /// Exclusive end index of each loop.
    pub ends: Vec<usize>,
}

impl Outline {
    pub fn from_loops<'a>(loops: impl IntoIterator<Item = &'a [Point]>) -> Self {
        let mut outline = Self::default();
        for points in loops {
            for &[x, y] in points.iter().take(MAX_POINTS.saturating_sub(outline.xs.len())) {
                outline.xs.push(x);
                outline.ys.push(y);
            }
            if outline.ends.len() < MAX_LOOPS && outline.ends.last() != Some(&outline.xs.len()) {
                outline.ends.push(outline.xs.len());
            }
        }
        outline
    }

    pub fn len(&self) -> usize {
        self.xs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.xs.is_empty()
    }

    /// Each loop as a list of points, for drawing.
    pub fn loops(&self) -> Vec<Vec<Point>> {
        let mut start = 0;
        self.ends
            .iter()
            .map(|&end| {
                let points = (start..end).map(|i| [self.xs[i], self.ys[i]]).collect();
                start = end;
                points
            })
            .collect()
    }
}

/// How a shape is bent before it turns. Each runs -1..1; 0 leaves the shape alone.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Bend {
    /// Up to 4x wide (positive) or 4x tall (negative).
    pub squish: f32,
    /// A shear, up to 45 degrees.
    pub skew: f32,
    /// Perspective: positive leans the top away, so it shrinks more than the bottom.
    pub tilt: f32,
}

/// The bounding box's middle and half its larger side: what fits points inside -1..1.
#[derive(Clone, Copy)]
struct Fit {
    mid_x: f32,
    mid_y: f32,
    inverse_half: f32,
}

impl Fit {
    fn of(xs: &[f32], ys: &[f32]) -> Self {
        let (mut x0, mut x1, mut y0, mut y1) = (f32::MAX, f32::MIN, f32::MAX, f32::MIN);
        for (&x, &y) in xs.iter().zip(ys) {
            x0 = x0.min(x);
            x1 = x1.max(x);
            y0 = y0.min(y);
            y1 = y1.max(y);
        }
        let half = 0.5 * (x1 - x0).max(y1 - y0).max(1e-6);
        Self { mid_x: 0.5 * (x0 + x1), mid_y: 0.5 * (y0 + y1), inverse_half: 1.0 / half }
    }

    #[inline(always)]
    fn apply(&self, x: f32, y: f32) -> (f32, f32) {
        ((x - self.mid_x) * self.inverse_half, (y - self.mid_y) * self.inverse_half)
    }
}

/// Bends `xs, ys` into `out_x, out_y` (same length): squish and skew, fit, tilt, fit again.
/// Straight edges stay straight through all three, so a polygon stays a polygon.
pub fn bend_points(xs: &[f32], ys: &[f32], bend: Bend, out_x: &mut [f32], out_y: &mut [f32]) {
    let n = xs.len().min(out_x.len()).min(out_y.len());
    let (sx, sy) = (bend.squish.exp2(), (-bend.squish).exp2());
    for i in 0..n {
        out_x[i] = sx * xs[i] + bend.skew * sy * ys[i];
        out_y[i] = sy * ys[i];
    }
    let first = Fit::of(&out_x[..n], &out_y[..n]);
    let lean = 1.5 * bend.tilt.abs();
    let p = 1.0 + 0.5 * lean;
    let q = 0.5 * lean * bend.tilt.signum();
    for i in 0..n {
        let (x, y) = first.apply(out_x[i], out_y[i]);
        let w = (p + q * y).max(0.2);
        out_x[i] = x / w;
        out_y[i] = y / w;
    }
    let second = Fit::of(&out_x[..n], &out_y[..n]);
    for i in 0..n {
        (out_x[i], out_y[i]) = second.apply(out_x[i], out_y[i]);
    }
}

/// Area centroid over every loop. Holes run the other way round, so they subtract.
pub fn centroid(xs: &[f32], ys: &[f32], ends: &[usize]) -> Point {
    let (mut area, mut cx, mut cy) = (0.0f64, 0.0f64, 0.0f64);
    let mut start = 0;
    for &end in ends {
        for i in start..end {
            let j = if i + 1 == end { start } else { i + 1 };
            let (x0, y0, x1, y1) = (xs[i] as f64, ys[i] as f64, xs[j] as f64, ys[j] as f64);
            let cross = x0 * y1 - x1 * y0;
            area += cross;
            cx += (x0 + x1) * cross;
            cy += (y0 + y1) * cross;
        }
        start = end;
    }
    if area.abs() < 1e-9 {
        return [0.0, 0.0];
    }
    [(cx / (3.0 * area)) as f32, (cy / (3.0 * area)) as f32]
}

/// sin and cos of a fraction of a turn, to within about 1e-6, far cheaper than the library's.
#[inline(always)]
pub fn turn_sin_cos(turn: f32) -> (f32, f32) {
    // Reduce to the nearest quarter turn, then a polynomial on +-1/8 of a turn.
    let x = turn - turn.floor();
    let quarter = (x * 4.0).round();
    let r = (x - 0.25 * quarter) * std::f32::consts::TAU;
    let r2 = r * r;
    let s = r * (1.0 + r2 * (-1.0 / 6.0 + r2 * (1.0 / 120.0 + r2 * (-1.0 / 5_040.0 + r2 * (1.0 / 362_880.0)))));
    let c = 1.0 + r2 * (-0.5 + r2 * (1.0 / 24.0 + r2 * (-1.0 / 720.0 + r2 * (1.0 / 40_320.0))));
    match quarter as i32 & 3 {
        0 => (s, c),
        1 => (c, -s),
        2 => (-s, -c),
        _ => (-c, s),
    }
}

/// Edges are tested eight at a time, and each group of eight has a bounding circle, so a
/// ray only looks closely at the groups it passes near.
const LANES: usize = 8;
const MAX_GROUPS: usize = MAX_POINTS / LANES;

/// An outline after bending, ready for the ray: fixed size, so the audio thread can bend a
/// shape every block without allocating.
pub struct Bent {
    pub xs: [f32; MAX_POINTS],
    pub ys: [f32; MAX_POINTS],
    pub n: usize,
    /// Edges as start point and direction, in groups of eight lanes.
    ax: [[f32; LANES]; MAX_GROUPS],
    ay: [[f32; LANES]; MAX_GROUPS],
    ex: [[f32; LANES]; MAX_GROUPS],
    ey: [[f32; LANES]; MAX_GROUPS],
    /// The same, as SIMD vectors for the ray.
    vax: [f32x8; MAX_GROUPS],
    vay: [f32x8; MAX_GROUPS],
    vex: [f32x8; MAX_GROUPS],
    vey: [f32x8; MAX_GROUPS],
    /// Each group's bounding circle.
    gx: [f32; MAX_GROUPS],
    gy: [f32; MAX_GROUPS],
    gr: [f32; MAX_GROUPS],
    groups: usize,
    pub centroid: Point,
}

impl Default for Bent {
    fn default() -> Self {
        Self {
            xs: [0.0; MAX_POINTS],
            ys: [0.0; MAX_POINTS],
            n: 0,
            ax: [[0.0; LANES]; MAX_GROUPS],
            ay: [[0.0; LANES]; MAX_GROUPS],
            ex: [[0.0; LANES]; MAX_GROUPS],
            ey: [[0.0; LANES]; MAX_GROUPS],
            vax: [f32x8::ZERO; MAX_GROUPS],
            vay: [f32x8::ZERO; MAX_GROUPS],
            vex: [f32x8::ZERO; MAX_GROUPS],
            vey: [f32x8::ZERO; MAX_GROUPS],
            gx: [0.0; MAX_GROUPS],
            gy: [0.0; MAX_GROUPS],
            gr: [0.0; MAX_GROUPS],
            groups: 0,
            centroid: [0.0; 2],
        }
    }
}

impl Bent {
    pub fn bend(&mut self, outline: &Outline, bend: Bend) {
        let n = outline.len().min(MAX_POINTS);
        self.n = n;
        bend_points(&outline.xs[..n], &outline.ys[..n], bend, &mut self.xs[..n], &mut self.ys[..n]);
        let mut k = 0;
        let mut start = 0;
        for &end in &outline.ends {
            let end = end.min(n);
            for i in start..end {
                let j = if i + 1 == end { start } else { i + 1 };
                let (g, lane) = (k / LANES, k % LANES);
                self.ax[g][lane] = self.xs[i];
                self.ay[g][lane] = self.ys[i];
                self.ex[g][lane] = self.xs[j] - self.xs[i];
                self.ey[g][lane] = self.ys[j] - self.ys[i];
                k += 1;
            }
            start = end;
        }
        self.groups = k.div_ceil(LANES);
        // Unused lanes of the last group: edges of no length, which nothing can hit.
        for lane in k % LANES..if k % LANES == 0 { 0 } else { LANES } {
            let g = self.groups - 1;
            self.ax[g][lane] = self.ax[g][0];
            self.ay[g][lane] = self.ay[g][0];
            self.ex[g][lane] = 0.0;
            self.ey[g][lane] = 0.0;
        }
        for g in 0..self.groups {
            let used = if g + 1 == self.groups && k % LANES != 0 { k % LANES } else { LANES };
            let (mut x0, mut x1, mut y0, mut y1) = (f32::MAX, f32::MIN, f32::MAX, f32::MIN);
            for lane in 0..used {
                for (x, y) in [
                    (self.ax[g][lane], self.ay[g][lane]),
                    (self.ax[g][lane] + self.ex[g][lane], self.ay[g][lane] + self.ey[g][lane]),
                ] {
                    x0 = x0.min(x);
                    x1 = x1.max(x);
                    y0 = y0.min(y);
                    y1 = y1.max(y);
                }
            }
            self.gx[g] = 0.5 * (x0 + x1);
            self.gy[g] = 0.5 * (y0 + y1);
            self.gr[g] = 0.5 * (x1 - x0).hypot(y1 - y0) + 1e-5;
            self.vax[g] = f32x8::from(self.ax[g]);
            self.vay[g] = f32x8::from(self.ay[g]);
            self.vex[g] = f32x8::from(self.ex[g]);
            self.vey[g] = f32x8::from(self.ey[g]);
        }
        self.centroid = centroid(&self.xs[..n], &self.ys[..n], &outline.ends);
    }

    /// The farthest point from `center`.
    #[inline]
    pub fn reach(&self, center: Point) -> f32 {
        let mut far = [0.0f32; LANES];
        let chunks = self.n / LANES;
        for c in 0..chunks {
            for lane in 0..LANES {
                let i = c * LANES + lane;
                let (dx, dy) = (self.xs[i] - center[0], self.ys[i] - center[1]);
                far[lane] = far[lane].max(dx * dx + dy * dy);
            }
        }
        for i in chunks * LANES..self.n {
            let (dx, dy) = (self.xs[i] - center[0], self.ys[i] - center[1]);
            far[0] = far[0].max(dx * dx + dy * dy);
        }
        far.iter().fold(0.0f32, |a, b| a.max(*b)).sqrt().max(1e-6)
    }

    /// The first hit along a ray from (ox, oy) heading (dx, dy) (a unit vector), within `limit`.
    #[inline]
    fn first_hit(&self, ox: f32, oy: f32, dx: f32, dy: f32, limit: f32) -> f32 {
        let mut best = limit;
        for g in 0..self.groups {
            let (cx, cy) = (self.gx[g] - ox, self.gy[g] - oy);
            let r = self.gr[g];
            if (cx * dx + cy * dy - r > best) | ((cx * dy - cy * dx).abs() > r) {
                continue;
            }
            let (ex, ey) = (self.vex[g], self.vey[g]);
            let wx = self.vax[g] - f32x8::splat(ox);
            let wy = self.vay[g] - f32x8::splat(oy);
            let (vdx, vdy) = (f32x8::splat(dx), f32x8::splat(dy));
            let inverse = f32x8::ONE / (vdx * ey - vdy * ex);
            let t = (wx * ey - wy * ex) * inverse;
            let u = (wx * vdy - wy * vdx) * inverse;
            // A parallel or empty edge divides by zero: NaN and infinity fail every test.
            let hit = t.cmp_ge(f32x8::ZERO) & u.cmp_ge(f32x8::ZERO) & u.cmp_le(f32x8::ONE) & t.cmp_lt(f32x8::splat(best));
            if hit.any() {
                let lanes = hit.blend(t, f32x8::splat(best)).to_array();
                for v in lanes {
                    best = best.min(v);
                }
            }
        }
        best
    }

    /// The wave's value with the shape turned clockwise by `turn` (0..1 of a revolution),
    /// about `center`, at `scale` (2 / reach), with the ray `probe` right of the center.
    #[inline]
    pub fn read(&self, turn: f32, center: Point, scale: f32, probe: f32) -> f32 {
        let fallen = self.fall(turn, center, scale, probe);
        if fallen >= REACH * 0.99999 {
            0.0
        } else {
            0.5 * (PROBE_HEIGHT - fallen)
        }
    }

    /// How far the ray falls before it hits, in scaled units below the probe (REACH if it
    /// reaches the floor).
    #[inline]
    pub fn fall(&self, turn: f32, center: Point, scale: f32, probe: f32) -> f32 {
        let (s, c) = turn_sin_cos(turn);
        let inverse = 1.0 / scale;
        let ox = center[0] + (probe * c - PROBE_HEIGHT * s) * inverse;
        let oy = center[1] + (probe * s + PROBE_HEIGHT * c) * inverse;
        self.first_hit(ox, oy, s, -c, REACH * inverse) * scale
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn square() -> Outline {
        Outline::from_loops([&[[-1.0, -1.0], [1.0, -1.0], [1.0, 1.0], [-1.0, 1.0]][..]])
    }

    fn bent(outline: &Outline, bend: Bend) -> Box<Bent> {
        let mut b = Box::<Bent>::default();
        b.bend(outline, bend);
        b
    }

    #[test]
    fn turn_sin_cos_matches_the_library() {
        for i in -2000..2000 {
            let t = i as f32 / 997.0;
            let (s, c) = turn_sin_cos(t);
            let (es, ec) = (std::f32::consts::TAU * t).sin_cos();
            assert!((s - es).abs() < 2e-6 && (c - ec).abs() < 2e-6, "{t}");
        }
    }

    #[test]
    fn a_square_turning_about_its_middle_reads_its_flat_top() {
        let b = bent(&square(), Bend::default());
        let scale = 2.0 / b.reach([0.0, 0.0]);
        // Corner distance is sqrt 2, so the top edge sits at height 2 / sqrt 2 after scaling.
        let v = b.read(0.0001, [0.0, 0.0], scale, 0.0);
        assert!((v - 0.5 * 2.0f32.sqrt()).abs() < 1e-3, "{v}");
        // A quarter turn later it reads the same (four-fold symmetry).
        assert!((b.read(0.2501, [0.0, 0.0], scale, 0.0) - v).abs() < 1e-3);
    }

    #[test]
    fn a_miss_reads_zero() {
        let b = bent(&square(), Bend::default());
        // Center far outside: the ray passes beside the shape for part of the turn.
        let center = [3.0, 0.0];
        let scale = 2.0 / b.reach(center);
        let misses = (0..256).filter(|i| b.read(*i as f32 / 256.0, center, scale, 0.0) == 0.0).count();
        assert!(misses > 20, "{misses}");
    }

    #[test]
    fn bends_keep_the_shape_inside_the_unit_box() {
        let outline = square();
        for bend in [Bend { squish: 1.0, skew: -1.0, tilt: 1.0 }, Bend { squish: -1.0, skew: 1.0, tilt: -1.0 }] {
            let b = bent(&outline, bend);
            for i in 0..b.n {
                assert!(b.xs[i].abs() <= 1.0001 && b.ys[i].abs() <= 1.0001);
            }
        }
    }

    #[test]
    fn the_centroid_of_a_ring_is_its_middle() {
        let ring = |r: f32, flip: bool| -> Vec<Point> {
            let mut pts: Vec<Point> = (0..64).map(|k| {
                let a = std::f32::consts::TAU * k as f32 / 64.0;
                [r * a.cos() + 0.3, r * a.sin() - 0.2]
            }).collect();
            if flip {
                pts.reverse();
            }
            pts
        };
        let (outer, inner) = (ring(1.0, false), ring(0.5, true));
        let o = Outline::from_loops([&outer[..], &inner[..]]);
        let c = centroid(&o.xs, &o.ys, &o.ends);
        assert!((c[0] - 0.3).abs() < 1e-3 && (c[1] + 0.2).abs() < 1e-3, "{c:?}");
    }
}
