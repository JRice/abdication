//! Each element's backdrop, kept faint: crystal facets and fracture lines for Earth, a slow
//! swaying swell for Water, quick fluttering currents for Air that now and then curl into
//! an eddy, a low glow and rising embers for Fire, growth rings for Wood, brushed grain with
//! a passing sheen for Metal, and for Chaos its own overlapped outline, turning.
//!
//! Motifs are drawn in the area's own units, so the same backdrop fills the oscillator page
//! or, in miniature and clipped to a circle, the face of the selected knob.

use std::f32::consts::{FRAC_PI_2, PI, TAU};

use nih_plug_vizia::vizia::vg::{Color, Paint, Path};

use super::painter::{color, Painter};
use crate::dsp::random::Rng;
use crate::dsp::shapes::Element;

pub fn accent(element: Element) -> u32 {
    match element {
        Element::Earth => 0xc8955a,
        Element::Water => 0x5aaac4,
        Element::Air => 0xb6c6d8,
        Element::Fire => 0xe2643d,
        Element::Wood => 0x94b46a,
        Element::Metal => 0xcdd0d6,
        Element::Chaos => 0xb47fe2,
    }
}

/// Where a backdrop goes: a rectangle, or (for a knob) the circle inside it.
pub struct Area {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub round: bool,
}

struct Sketch<'p, 'c> {
    p: &'p mut Painter<'c>,
    a: Area,
    hex: u32,
    /// Alpha multiplier: a miniature needs more to read at all.
    boost: f32,
}

impl Sketch<'_, '_> {
    fn at(&self, u: f32, v: f32) -> (f32, f32) {
        (self.a.x + u * self.a.w, self.a.y + v * self.a.h)
    }

    fn inside(&self, x: f32, y: f32) -> bool {
        if !self.a.round {
            return true;
        }
        let r = 0.5 * self.a.w;
        let (dx, dy) = (x - self.a.x - r, y - self.a.y - r);
        dx * dx + dy * dy <= r * r
    }

    fn ink(&self, alpha: f32) -> Color {
        color(self.hex, (alpha * self.boost).min(0.7))
    }

    /// A polyline in pixels, broken wherever it leaves the area.
    fn line(&mut self, points: impl IntoIterator<Item = (f32, f32)>, alpha: f32, width: f32) {
        let ink = self.ink(alpha);
        let mut run: Vec<(f32, f32)> = Vec::new();
        for (x, y) in points {
            if self.inside(x, y) {
                run.push((x, y));
            } else if run.len() > 1 {
                self.p.polyline(run.drain(..), ink, width);
            } else {
                run.clear();
            }
        }
        if run.len() > 1 {
            self.p.polyline(run, ink, width);
        }
    }

    /// A horizontal wavy line across the area at height `v`, `wave` giving pixels of offset.
    fn wavy(&mut self, v: f32, wave: impl Fn(f32) -> f32, alpha: f32, width: f32) {
        let steps = (self.a.w / 4.0).max(12.0) as usize;
        let points: Vec<(f32, f32)> = (0..=steps)
            .map(|i| {
                let (x, y) = self.at(i as f32 / steps as f32, v);
                (x, y + wave(x - self.a.x))
            })
            .collect();
        self.line(points, alpha, width);
    }

    fn dot(&mut self, x: f32, y: f32, r: f32, alpha: f32) {
        if self.inside(x, y) {
            let ink = self.ink(alpha);
            self.p.fill_circle(x, y, r, ink);
        }
    }

    fn fill(&mut self, paint: &Paint) {
        let mut path = Path::new();
        if self.a.round {
            path.circle(self.a.x + 0.5 * self.a.w, self.a.y + 0.5 * self.a.h, 0.5 * self.a.w);
        } else {
            path.rect(self.a.x, self.a.y, self.a.w, self.a.h);
        }
        self.p.canvas.fill_path(&path, paint);
    }
}

/// `chaos` is the Chaos outline's loops (in -1..1), used only for Chaos.
pub fn paint(p: &mut Painter, element: Element, area: Area, t: f32, chaos: &[Vec<(f32, f32)>]) {
    let mini = area.round;
    let (x, y, w, h) = (area.x, area.y, area.w, area.h);
    // Feature size: a page is about 400 px tall; a knob face about 40.
    let s = if mini { (h / 400.0).max(0.12) * 2.2 } else { h / 400.0 };
    let width = if mini { 0.9 } else { 1.2 };
    let mut k = Sketch { p, a: area, hex: accent(element), boost: if mini { 3.6 } else { 1.0 } };
    k.p.canvas.save();
    k.p.canvas.intersect_scissor(x, y, w, h);
    match element {
        Element::Earth => {
            let mut rng = Rng::new(11);
            // Fracture lines on a crystal lattice: every turn a multiple of 30°.
            let paths = if mini { 3 } else { 7 };
            for i in 0..paths {
                let mut px = x - 20.0 * s;
                let mut py = y + (i as f32 + 0.3 + 0.4 * rng.unit()) / paths as f32 * h;
                let mut points = vec![(px, py)];
                while px < x + w + 20.0 * s {
                    let steer = ((y + 0.5 * h - py) / h * 2.0).clamp(-1.0, 1.0);
                    let turn = [-2.0, -1.0, 0.0, 0.0, 1.0, 2.0][rng.below(6)] + steer;
                    let angle = (turn.round().clamp(-2.0, 2.0)) * PI / 6.0;
                    let length = (28.0 + 36.0 * rng.unit()) * s;
                    px += length * angle.cos();
                    py += length * angle.sin();
                    points.push((px, py));
                }
                k.line(points, 0.07 + 0.04 * rng.unit(), width);
            }
            // Clusters of prisms, each catching the light now and then.
            let clusters = if mini { 2 } else { 7 };
            for c in 0..clusters {
                let (bx, by) = (x + (0.08 + 0.84 * rng.unit()) * w, y + (0.25 + 0.7 * rng.unit()) * h);
                let lean = rng.signed() * 0.5;
                for j in 0..2 + rng.below(3) {
                    let angle = -FRAC_PI_2 + lean + (j as f32 - 1.0) * 0.38 + 0.15 * rng.signed();
                    let (length, girth) = ((40.0 + 70.0 * rng.unit()) * s, (9.0 + 12.0 * rng.unit()) * s);
                    let glint = ((0.3 * t + 2.3 * c as f32 + 1.1 * j as f32).sin().max(0.0)).powi(12);
                    let alpha = 0.09 + 0.16 * glint;
                    let (ax, ay) = (angle.cos(), angle.sin());
                    let at = |along: f32, across: f32| (bx + along * ax - across * ay, by + along * ay + across * ax);
                    let shoulder = length - 0.6 * girth;
                    let half = 0.5 * girth;
                    k.line([at(0.0, -half), at(shoulder, -half), at(length, 0.0), at(shoulder, half), at(0.0, half), at(0.0, -half)], alpha, width);
                    // The facets meeting at the tip.
                    k.line([at(shoulder, -half), at(shoulder - 0.35 * girth, 0.0), at(shoulder, half)], 0.7 * alpha, width);
                    k.line([at(shoulder - 0.35 * girth, 0.0), at(length, 0.0)], 0.7 * alpha, width);
                }
            }
        }
        Element::Water => {
            // A slow sway back and forth, each line rising and falling on its own time.
            let gap = 20.0 * s;
            let lines = (h / gap).ceil() as usize + 1;
            for i in 0..lines {
                let f = i as f32;
                let sway = 1.6 * (0.09 * t + 0.6 * f).sin();
                let swell = 1.0 + 0.35 * (0.07 * t + 1.3 * f).sin();
                let bob = 4.0 * s * (0.06 * t + 0.9 * f).sin();
                let alpha = 0.05 + 0.04 * (f * 0.9 + 0.1 * t).sin().abs();
                k.wavy(((f + 0.5) * gap + bob) / h, |px| s * swell * (4.0 * (px / s * 0.021 + sway).sin() + 2.0 * (px / s * 0.047 - 0.7 * sway).sin()), alpha, width);
            }
        }
        Element::Air => {
            // Quick, fluttering currents. Every so often an eddy spins up somewhere, curls
            // the lines near it round, and lets them go.
            const EVERY: f32 = 9.0;
            const LASTS: f32 = 4.0;
            let cycle = (t / EVERY).floor();
            let into = t - cycle * EVERY;
            let mut place = Rng::new(97 + cycle as u32);
            let (ex, ey) = (x + (0.2 + 0.6 * place.unit()) * w, y + (0.25 + 0.5 * place.unit()) * h);
            let spin = if place.unit() < 0.5 { 1.0 } else { -1.0 } * 3.4 * if into < LASTS { (PI * into / LASTS).sin().powi(2) } else { 0.0 };
            let reach = 75.0 * s;
            let swirl = |(px, py): (f32, f32)| {
                let (dx, dy) = (px - ex, py - ey);
                let angle = spin * (-(dx * dx + dy * dy) / (reach * reach)).exp();
                let (sn, cs) = angle.sin_cos();
                (ex + dx * cs - dy * sn, ey + dx * sn + dy * cs)
            };
            let gap = 33.0 * s;
            let lines = (h / gap).ceil() as usize + 1;
            let steps = (w / 3.0).max(16.0) as usize;
            for i in 0..lines {
                let f = i as f32;
                let drift = 1.3 * t + f;
                let row = y + (f + 0.5) * gap;
                let points: Vec<(f32, f32)> = (0..=steps)
                    .map(|j| {
                        let px = x + w * j as f32 / steps as f32;
                        let u = px - x;
                        let lift = s * (20.0 * (u / s * 0.006 + drift).sin() + 7.0 * (u / s * 0.017 + 2.0 * f + 0.9 * t).sin() + 2.5 * (u / s * 0.05 - 4.0 * t + f).sin());
                        swirl((px, row + lift))
                    })
                    .collect();
                k.line(points, 0.07, 0.9);
            }
        }
        Element::Fire => {
            let glow = Paint::radial_gradient(x + 0.3 * w, y + h + 40.0 * s, 0.0, 0.95 * h, k.ink(0.20), k.ink(0.0));
            k.fill(&glow);
            let mut rng = Rng::new(23);
            for _ in 0..(if mini { 10 } else { 36 }) {
                let (x0, speed, phase, size) = (rng.unit(), 0.03 + 0.05 * rng.unit(), rng.unit(), 0.8 + 1.5 * rng.unit());
                let rise = (phase + speed * t).fract();
                let px = x + x0 * w + 8.0 * s * (1.3 * t + 6.0 * x0).sin();
                let py = y + h - rise * h;
                k.dot(px, py, size * s.max(0.5), 0.22 * (1.0 - rise));
            }
        }
        Element::Wood => {
            let (cx, cy) = (x + 0.22 * w, y + 0.78 * h);
            let stretch = if mini { 1.6 } else { 2.3 };
            for i in 0..16 {
                let f = i as f32;
                let r = s * (22.0 + f * 24.0 + 4.0 * (f * 2.3).sin());
                let wobble = f * 0.7;
                let steps = 120;
                let points: Vec<(f32, f32)> = (0..=steps)
                    .map(|j| {
                        let q = TAU * j as f32 / steps as f32;
                        let rr = r * (1.0 + 0.04 * (3.0 * q + wobble).sin() + 0.02 * (7.0 * q).sin());
                        (cx + stretch * rr * q.cos(), cy + rr * q.sin())
                    })
                    .collect();
                k.line(points, 0.07 + 0.03 * (i % 3 == 0) as u8 as f32, width);
            }
        }
        Element::Metal => {
            let mut rng = Rng::new(31);
            for _ in 0..(if mini { 16 } else { 150 }) {
                let py = y + rng.unit() * h;
                let x0 = x + rng.unit() * w * 0.6;
                let len = 0.2 * w + rng.unit() * 0.6 * w;
                let alpha = 0.025 + 0.04 * rng.unit();
                k.line([(x0, py), ((x0 + 0.5 * len).min(x + w), py), ((x0 + len).min(x + w), py)], alpha, 0.7);
            }
            let sx = x + ((0.05 * t).fract() * 2.0 - 0.5) * w;
            let band = 160.0 * s.max(0.15);
            let sheen = Paint::linear_gradient_stops(sx - band, y, sx + band, y + h, [(0.0, k.ink(0.0)), (0.5, k.ink(0.07)), (1.0, k.ink(0.0))]);
            k.fill(&sheen);
        }
        Element::Chaos => {
            let (cx, cy) = (x + 0.5 * w, y + 0.5 * h);
            let size = if mini { 0.75 * h } else { 1.1 * h };
            let (sn, cs) = (0.03 * t).sin_cos();
            for l in chaos {
                let mut points: Vec<(f32, f32)> =
                    l.iter().map(|(u, v)| (cx + size * (u * cs + v * sn), cy - size * (-u * sn + v * cs))).collect();
                if let Some(first) = points.first().copied() {
                    points.push(first);
                }
                k.line(points, 0.12, width);
            }
            for i in 0..6 {
                let f = i as f32;
                let r = s * (40.0 + 46.0 * f);
                let ox = cx + 30.0 * s * (0.2 * t + f).sin();
                let points: Vec<(f32, f32)> = (0..=60).map(|j| {
                    let q = TAU * j as f32 / 60.0;
                    (ox + r * q.cos(), cy + r * q.sin())
                }).collect();
                k.line(points, 0.03, 0.8);
            }
        }
    }
    k.p.canvas.restore();
}
