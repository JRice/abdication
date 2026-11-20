//! The map under the panel: an endless engraved coastline drifting left, built strip by
//! strip (each strip from its index, so the same index is always the same land), with
//! ships on two lanes and beasts surfacing in open water. Ported from the mockup's
//! script, with its numbers.

use nih_plug_vizia::vizia::vg::Path;

use crate::knobs::Mulberry;

/// A strip's width, and how fast the world drifts (px a second).
pub const CW: f32 = 420.0;
pub const SPEED: f32 = 1.6;
/// The shipping lanes (y, direction).
pub const LANES: [(f32, f32); 2] = [(152.0, 1.0), (584.0, -1.0)];

pub struct Land {
    /// The coast itself, and three water lines outside it.
    pub coast: Path,
    pub rings: [Path; 3],
    /// For telling land from water.
    pub poly: Vec<(f32, f32)>,
}

pub struct Chunk {
    pub lands: Vec<Land>,
    pub mountains: Path,
}

type Blob = Vec<(f32, f32)>;

fn blob(r: &mut Mulberry, n: usize, amp: f32) -> Blob {
    let ks: Vec<(f32, f32, f32)> = (2..=9).map(|k| (k as f32, (amp / k as f32) * (0.5 + r.next()), r.next() * 6.283)).collect();
    (0..n)
        .map(|i| {
            let th = i as f32 / n as f32 * 6.283;
            let mut m = 1.0;
            for (k, a, ph) in &ks {
                m += a * (k * th + ph).sin();
            }
            m += (r.next() - 0.5) * 0.03;
            (th, m)
        })
        .collect()
}

struct Spec {
    pts: Blob,
    cx: f32,
    cy: f32,
    r: f32,
}

#[allow(clippy::too_many_arguments)]
fn fit(r: &mut Mulberry, cx: f32, cy: f32, radius: f32, amp: f32, n: usize, y_min: Option<f32>, y_max: Option<f32>) -> Spec {
    let pts = blob(r, n, amp);
    let (mut up, mut down) = (0.0f32, 0.0f32);
    for (th, m) in &pts {
        let s = th.sin() * m;
        up = up.max(-s);
        down = down.max(s);
    }
    let (mut radius, mut cy) = (radius, cy);
    up *= radius;
    down *= radius;
    if let (Some(lo), Some(hi)) = (y_min, y_max) {
        if up + down > (hi - lo) * 0.96 {
            let k = (hi - lo) * 0.96 / (up + down);
            radius *= k;
            up *= k;
            down *= k;
        }
    }
    if let Some(hi) = y_max {
        if cy + down > hi {
            cy = hi - down;
        }
    }
    if let Some(lo) = y_min {
        if cy - up < lo {
            cy = lo + up;
        }
    }
    Spec { pts, cx, cy, r: radius }
}

fn ring(pts: &Blob, cx: f32, cy: f32, radius: f32) -> (Path, Vec<(f32, f32)>) {
    let poly: Vec<(f32, f32)> = pts.iter().map(|(th, m)| (cx + th.cos() * radius * m, cy + th.sin() * radius * m)).collect();
    let mut p = Path::new();
    for (i, (x, y)) in poly.iter().enumerate() {
        if i == 0 { p.move_to(*x, *y) } else { p.line_to(*x, *y) }
    }
    p.close();
    (p, poly)
}

/// Strip `i`: northern coasts, southern coasts and mid-sea islands, kept clear of the
/// lanes; a slow sine over the index trades land between north and south.
pub fn chunk(i: i64) -> Chunk {
    let mut r = Mulberry::new(((i as u32).wrapping_mul(2_654_435_761)) ^ 0x1510);
    let fi = i as f32;
    let phase = 0.5 * (fi * 0.73).sin() + 0.5 * (fi * 0.29 + 1.3).sin();
    let mut specs = Vec::new();
    let n_top = if r.next() < 0.42 + 0.3 * phase { if r.next() < 0.3 { 2 } else { 1 } } else { 0 };
    for _ in 0..n_top {
        let (cx, cy, rad) = (r.next() * CW, -20.0 - r.next() * 80.0, 100.0 + r.next() * 120.0);
        specs.push(fit(&mut r, cx, cy, rad, 0.17, 72, None, Some(114.0)));
    }
    let n_bot = if r.next() < 0.55 - 0.3 * phase { if r.next() < 0.3 { 2 } else { 1 } } else { 0 };
    for _ in 0..n_bot {
        let (cx, cy, rad) = (r.next() * CW, 860.0 + r.next() * 80.0, 110.0 + r.next() * 120.0);
        specs.push(fit(&mut r, cx, cy, rad, 0.17, 72, Some(604.0), None));
    }
    let n_mid = if r.next() < 0.6 { if r.next() < 0.35 { 2 } else { 1 } } else { 0 };
    for _ in 0..n_mid {
        let big = r.next() < 0.2;
        let (cx, cy) = (r.next() * CW, 190.0 + r.next() * 330.0);
        let rad = if big { 50.0 + r.next() * 45.0 } else { 12.0 + r.next() * 26.0 };
        specs.push(fit(&mut r, cx, cy, rad, if big { 0.2 } else { 0.3 }, if big { 56 } else { 40 }, Some(170.0), Some(540.0)));
    }
    let mut lands = Vec::new();
    let mut mountains = Path::new();
    for s in &specs {
        let small = s.r < 40.0;
        let o = if small { [3.0, 6.0, 10.0] } else { [6.0, 12.0, 19.0] };
        let (coast, _) = ring(&s.pts, s.cx, s.cy, s.r);
        let (r1, poly) = ring(&s.pts, s.cx, s.cy, s.r + o[0]);
        let (r2, _) = ring(&s.pts, s.cx, s.cy, s.r + o[1]);
        let (r3, _) = ring(&s.pts, s.cx, s.cy, s.r + o[2]);
        lands.push(Land { coast, rings: [r1, r2, r3], poly });
        let count = (s.r / 5.0).round() as usize;
        for _ in 0..count {
            let (th, m) = s.pts[((r.next() * s.pts.len() as f32) as usize).min(s.pts.len() - 1)];
            let rad = (0.15 + 0.6 * r.next()) * s.r * m;
            let (x, y) = (s.cx + th.cos() * rad, s.cy + th.sin() * rad);
            if !(28.0..=772.0).contains(&y) {
                continue;
            }
            let z = 3.5 + r.next() * 3.5;
            mountains.move_to(x - z, y + z * 0.6);
            mountains.line_to(x, y - z * 0.8);
            mountains.line_to(x + z, y + z * 0.6);
            mountains.move_to(x + 0.6, y - z * 0.4);
            mountains.line_to(x + z * 0.55, y + z * 0.5);
        }
    }
    Chunk { lands, mountains }
}

pub fn in_poly(x: f32, y: f32, poly: &[(f32, f32)]) -> bool {
    let mut c = false;
    let mut j = poly.len() - 1;
    for i in 0..poly.len() {
        let ((xi, yi), (xj, yj)) = (poly[i], poly[j]);
        if (yi > y) != (yj > y) && x < (xj - xi) * (y - yi) / (yj - yi) + xi {
            c = !c;
        }
        j = i;
    }
    c
}

/// The fixed parts of the map: the spiral, the rhumb lines, the compass rose and the
/// graduated border.
pub struct Statics {
    pub spiral: Path,
    pub rhumbs: Path,
    pub rose: Path,
    pub border: Path,
}

pub fn statics() -> Statics {
    let mut spiral = Path::new();
    let mut th = 0.0f32;
    let mut first = true;
    loop {
        let rr = 14.0 + 13.0 * th / 6.283;
        if rr > 720.0 {
            break;
        }
        let (x, y) = (620.0 + th.cos() * rr * (1.0 + 0.04 * (th * 3.1).sin()), 410.0 + th.sin() * rr);
        if first { spiral.move_to(x, y) } else { spiral.line_to(x, y) }
        first = false;
        th += 0.17;
    }
    let mut rhumbs = Path::new();
    for i in 0..32 {
        let a = i as f32 / 32.0 * 6.283;
        rhumbs.move_to(725.0, 410.0);
        rhumbs.line_to(725.0 + a.cos() * 1100.0, 410.0 + a.sin() * 1100.0);
    }
    let mut rose = Path::new();
    for i in 0..16 {
        let a = i as f32 / 16.0 * 6.283 - std::f32::consts::FRAC_PI_2;
        let rl = if i % 4 == 0 { 19.0 } else if i % 2 == 0 { 11.0 } else { 3.5 };
        let (x, y) = (725.0 + a.cos() * rl, 410.0 + a.sin() * rl);
        if i == 0 { rose.move_to(x, y) } else { rose.line_to(x, y) }
    }
    rose.close();
    let mut border = Path::new();
    let mut x = 18.0;
    while x < 1222.0 {
        border.rect(x, 10.0, 20.0, 8.0);
        border.rect(x, 782.0, 20.0, 8.0);
        x += 40.0;
    }
    let mut y = 18.0;
    while y < 782.0 {
        border.rect(10.0, y, 8.0, 20.0);
        border.rect(1222.0, y, 8.0, 20.0);
        y += 40.0;
    }
    Statics { spiral, rhumbs, rose, border }
}

/// The world as it drifts: its strips (made as they come into view), the ships and the
/// beasts.
pub struct World {
    chunks: std::collections::BTreeMap<i64, Chunk>,
    pub ships: [Option<f32>; 2],
    ship_wait: [f32; 2],
    pub beasts: Vec<Beast>,
    next_beast: f32,
    rng: Mulberry,
}

pub struct Beast {
    /// Its place on the world (x drifts with the map), and when it surfaced.
    pub wx: f64,
    pub y: f32,
    pub kind: usize,
    pub born: f32,
    pub flip: f32,
}

pub const BEAST_LIFE: f32 = 6.2;

impl World {
    pub fn new(seed: u32) -> Self {
        Self { chunks: Default::default(), ships: [None, None], ship_wait: [6.0, 45.0], beasts: Vec::new(), next_beast: 10.0,
               rng: Mulberry::new(seed) }
    }

    /// The strips in view at `off`, made if new; old ones are dropped.
    pub fn visible(&mut self, off: f64) -> Vec<i64> {
        let cw = CW as f64;
        let lo = ((off - cw) / cw).floor() as i64;
        let hi = ((off + 1240.0 + cw) / cw).floor() as i64;
        for i in lo..=hi {
            self.chunks.entry(i).or_insert_with(|| chunk(i));
        }
        self.chunks.retain(|&k, _| k >= lo - 2 && k <= hi + 2);
        (lo..=hi).collect()
    }

    pub fn chunk(&self, i: i64) -> Option<&Chunk> {
        self.chunks.get(&i)
    }

    /// Whether screen point (x, y) is open water at `off` (with a margin).
    pub fn is_water(&mut self, x: f32, y: f32, off: f64) -> bool {
        let idx = self.visible(off);
        for dx in [-32.0, 0.0, 32.0] {
            for &i in &idx {
                let lx = (x as f64 + dx + off - i as f64 * CW as f64) as f32;
                if self.chunks[&i].lands.iter().any(|l| in_poly(lx, y, &l.poly)) {
                    return false;
                }
            }
        }
        true
    }

    /// Move the ships and beasts on by `dt` seconds (`t` is the panel's clock).
    pub fn sail(&mut self, dt: f32, t: f32, off: f64, kinds: usize) {
        for i in 0..2 {
            let dir = LANES[i].1;
            match self.ships[i] {
                None => {
                    self.ship_wait[i] -= dt;
                    if self.ship_wait[i] <= 0.0 {
                        self.ships[i] = Some(if dir > 0.0 { -60.0 } else { 1300.0 });
                    }
                }
                Some(x) => {
                    let x = x + (dir * 8.0 - SPEED) * dt;
                    if !(-80.0..=1320.0).contains(&x) {
                        self.ships[i] = None;
                        self.ship_wait[i] = 40.0 + self.rng.next() * 120.0;
                    } else {
                        self.ships[i] = Some(x);
                    }
                }
            }
        }
        if t >= self.next_beast && self.beasts.len() < 2 {
            for _ in 0..24 {
                let band = self.rng.next();
                let (x, y) = if band < 0.4 {
                    (170.0 + self.rng.next() * 900.0, 146.0 + self.rng.next() * 10.0)
                } else if band < 0.8 {
                    (60.0 + self.rng.next() * 1120.0, 572.0 + self.rng.next() * 30.0)
                } else {
                    (160.0 + self.rng.next() * 900.0, 200.0 + self.rng.next() * 330.0)
                };
                if self.is_water(x, y, off) {
                    let kind = ((self.rng.next() * kinds as f32) as usize).min(kinds - 1);
                    let flip = if self.rng.next() < 0.5 { -1.0 } else { 1.0 };
                    self.beasts.push(Beast { wx: x as f64 + off, y, kind, born: t, flip });
                    break;
                }
            }
            self.next_beast = t + 14.0 + self.rng.next() * 30.0;
        }
        self.beasts.retain(|b| t - b.born < BEAST_LIFE);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_are_stable_and_keep_the_lanes_clear() {
        for i in -20..60 {
            let a = chunk(i);
            let b = chunk(i);
            assert_eq!(a.lands.len(), b.lands.len());
            for land in &a.lands {
                for &(_, y) in &land.poly {
                    // The first water line, just outside the coast, stays off the lanes.
                    assert!(!(124.0..160.0).contains(&y) && !(550.0..594.0).contains(&y), "strip {i}: coast at {y}");
                }
            }
        }
    }

    #[test]
    fn land_and_water_balance() {
        // Over a long stretch, roughly as many strips have northern land as southern.
        let (mut north, mut south) = (0, 0);
        for i in 0..400 {
            for l in chunk(i).lands {
                let y = l.poly.iter().map(|p| p.1).sum::<f32>() / l.poly.len() as f32;
                if y < 100.0 { north += 1 } else if y > 700.0 { south += 1 }
            }
        }
        assert!(north > 100 && south > 100, "{north} {south}");
    }
}
