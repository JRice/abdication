//! Something in the background. Each one is a chain of fixed-length links rooted below the
//! window, bent by slow waves running along it, so it can't stretch or come loose however
//! long it runs, and it costs a few sines per link. Nothing here touches the sound.

use std::f32::consts::{FRAC_PI_2, PI};

use super::panel::{HEIGHT, WIDTH};
use crate::dsp::random::Rng;

const MOST_AT_ONCE: usize = 3;
const POINTS: usize = 26;
const LIFE_S: (f32, f32) = (26.0, 36.0);
const GAP_S: (f32, f32) = (5.0, 16.0);
const FIRST_S: f32 = 1.5;
/// How far below the window a root sits.
const ROOT_DEPTH: (f32, f32) = (20.0, 80.0);
const LENGTH: (f32, f32) = (360.0, 640.0);
const ROOT_WIDTH: (f32, f32) = (22.0, 44.0);
const TIP_WIDTH: f32 = 0.8;
/// How hard it leans back toward the middle, from the far edge of the window.
const LEAN: f32 = 0.9;
const RIB_EVERY: usize = 2;
/// A wave's period, and how far it reaches along the body at once.
const WAVE_PERIOD_S: (f32, f32) = (14.0, 42.0);
const WAVE_SPAN: (f32, f32) = (1.5, 5.0);
/// Radians of bend each wave can put into a whole tentacle.
const BEND: (f32, f32) = (1.2, 2.4);
/// How much more it bends at the tip than at the root.
const TIP_BEND: f32 = 2.5;
const WAVES: usize = 2;
/// How far the root's own heading sways.
const SWAY: f32 = 0.3;

#[derive(Clone, Copy)]
struct Wave {
    rate: f32,
    span: f32,
    bend: f32,
    phase: f32,
}

pub struct Tentacle {
    root: (f32, f32),
    link: f32,
    root_width: f32,
    lean: f32,
    sway: Wave,
    waves: [Wave; WAVES],
    age: f32,
    life: f32,
    points: [(f32, f32); POINTS],
    /// The way each point faces, for the sides of the body.
    headings: [f32; POINTS],
}

impl Tentacle {
    fn new(rng: &mut Rng) -> Self {
        let root = (rng.unit() * WIDTH, HEIGHT + rng.range(ROOT_DEPTH.0, ROOT_DEPTH.1));
        let wave = |rng: &mut Rng| Wave {
            rate: PI * 2.0 / rng.range(WAVE_PERIOD_S.0, WAVE_PERIOD_S.1),
            span: rng.range(WAVE_SPAN.0, WAVE_SPAN.1) * if rng.chance(0.5) { 1.0 } else { -1.0 },
            bend: rng.range(BEND.0, BEND.1),
            phase: rng.unit() * PI * 2.0,
        };
        let waves = [wave(rng), wave(rng)];
        let sway = wave(rng);
        let mut tentacle = Self {
            root,
            link: rng.range(LENGTH.0, LENGTH.1) / (POINTS - 1) as f32,
            root_width: rng.range(ROOT_WIDTH.0, ROOT_WIDTH.1),
            lean: LEAN * (WIDTH / 2.0 - root.0) / (WIDTH / 2.0),
            sway,
            waves,
            age: 0.0,
            life: rng.range(LIFE_S.0, LIFE_S.1),
            points: [root; POINTS],
            headings: [0.0; POINTS],
        };
        tentacle.solve();
        tentacle
    }

    /// Lays out the chain from the root, each link turning a little from the last.
    fn solve(&mut self) {
        let t = self.age;
        let sway = SWAY * (self.sway.rate * t + self.sway.phase).sin();
        let mut heading = -FRAC_PI_2 + self.lean + sway;
        let mut at = self.root;
        let per_link = 1.0 / (POINTS - 1) as f32;
        for i in 0..POINTS {
            self.points[i] = at;
            self.headings[i] = heading;
            let along = i as f32 * per_link;
            let curl: f32 = self.waves.iter().map(|w| w.bend * (w.rate * t + w.span * along + w.phase).sin()).sum();
            heading += curl * per_link * (1.0 + TIP_BEND * along * along);
            at = (at.0 + self.link * heading.cos(), at.1 + self.link * heading.sin());
        }
    }

    fn step(&mut self, dt: f32) {
        self.age += dt;
        self.solve();
    }

    fn finished(&self) -> bool {
        self.age >= self.life
    }

    /// 0 at either end of its life, 1 in the middle, and smooth all the way.
    pub fn visibility(&self) -> f32 {
        let progress = (self.age / self.life).clamp(0.0, 1.0);
        (PI * progress).sin().powi(2)
    }

    /// Half its width at a point: it narrows all the way to the tip.
    fn half_width(&self, i: usize) -> f32 {
        let along = i as f32 / (POINTS - 1) as f32;
        0.5 * (TIP_WIDTH + (self.root_width - TIP_WIDTH) * (1.0 - along).powf(0.9))
    }

    fn side(&self, i: usize, left: bool) -> (f32, f32) {
        let (x, y) = self.points[i];
        let across = self.headings[i] + if left { FRAC_PI_2 } else { -FRAC_PI_2 };
        let half = self.half_width(i);
        (x + half * across.cos(), y + half * across.sin())
    }

    /// One edge of the body: root to tip on the way up, tip to root on the way back.
    pub fn for_each_edge(&self, up: bool, mut each: impl FnMut(f32, f32)) {
        if up {
            (0..POINTS).for_each(|i| {
                let (x, y) = self.side(i, true);
                each(x, y);
            });
        } else {
            (0..POINTS).rev().for_each(|i| {
                let (x, y) = self.side(i, false);
                each(x, y);
            });
        }
    }

    pub fn for_each_rib(&self, mut each: impl FnMut((f32, f32), (f32, f32))) {
        (0..POINTS).step_by(RIB_EVERY).for_each(|i| each(self.side(i, true), self.side(i, false)));
    }
}

pub struct Tentacles {
    alive: Vec<Tentacle>,
    until_next: f32,
    rng: Rng,
}

impl Tentacles {
    pub fn new(seed: u32) -> Self {
        Self { alive: Vec::with_capacity(MOST_AT_ONCE), until_next: FIRST_S, rng: Rng::new(seed) }
    }

    pub fn step(&mut self, dt: f32) {
        // A window that was hidden for a while shouldn't lurch.
        let dt = dt.min(0.25);
        self.alive.iter_mut().for_each(|t| t.step(dt));
        self.alive.retain(|t| !t.finished());
        self.until_next -= dt;
        if self.until_next <= 0.0 && self.alive.len() < MOST_AT_ONCE {
            self.alive.push(Tentacle::new(&mut self.rng));
            self.until_next = self.rng.range(GAP_S.0, GAP_S.1);
        }
    }

    pub fn alive(&self) -> impl Iterator<Item = &Tentacle> {
        self.alive.iter()
    }

    pub fn any_visible(&self) -> bool {
        !self.alive.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DT: f32 = 0.066;

    fn distance(a: (f32, f32), b: (f32, f32)) -> f32 {
        ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
    }

    /// Runs a long time, showing each state to `each`.
    fn run(minutes: f32, mut each: impl FnMut(&Tentacles)) {
        let mut tentacles = Tentacles::new(42);
        for _ in 0..(minutes * 60.0 / DT) as usize {
            tentacles.step(DT);
            each(&tentacles);
        }
    }

    #[test]
    fn there_are_never_more_than_three_and_there_are_often_some() {
        let (mut most, mut some, mut steps) = (0, 0, 0);
        run(30.0, |t| {
            most = most.max(t.alive().count());
            some += t.any_visible() as u32;
            steps += 1;
        });
        assert_eq!(most, MOST_AT_ONCE);
        assert!(some * 10 > steps * 9, "{some}/{steps}");
    }

    #[test]
    fn every_root_is_below_the_window_and_stays_put() {
        run(10.0, |t| {
            for tentacle in t.alive() {
                assert!(tentacle.points[0].1 > HEIGHT);
                assert_eq!(tentacle.points[0], tentacle.root);
            }
        });
    }

    #[test]
    fn links_never_stretch_and_widths_only_narrow() {
        run(10.0, |t| {
            for tentacle in t.alive() {
                for i in 1..POINTS {
                    assert!((distance(tentacle.points[i], tentacle.points[i - 1]) - tentacle.link).abs() < 1e-2);
                    assert!(tentacle.half_width(i) < tentacle.half_width(i - 1));
                }
            }
        });
    }

    #[test]
    fn they_fade_in_and_out_over_about_thirty_seconds() {
        let mut tentacle = Tentacle::new(&mut Rng::new(7));
        assert_eq!(tentacle.visibility(), 0.0);
        assert!(tentacle.life >= LIFE_S.0 && tentacle.life <= LIFE_S.1);
        tentacle.step(tentacle.life / 2.0);
        assert!(tentacle.visibility() > 0.999);
        tentacle.step(tentacle.life / 2.0);
        assert!(tentacle.visibility() < 1e-3 && tentacle.finished());
    }

    #[test]
    fn they_move_slowly() {
        let mut tentacle = Tentacle::new(&mut Rng::new(7));
        for _ in 0..500 {
            let before = tentacle.points;
            tentacle.step(DT);
            let fastest = before.iter().zip(&tentacle.points).map(|(a, b)| distance(*a, *b)).fold(0.0, f32::max);
            // Pixels a second, at the tip, which moves the most by far.
            assert!(fastest / DT < 110.0, "{}", fastest / DT);
        }
    }

    #[test]
    fn each_frame_costs_next_to_nothing() {
        let mut tentacles = Tentacles::new(1);
        for _ in 0..2_000 {
            tentacles.step(DT);
        }
        let start = std::time::Instant::now();
        for _ in 0..1_000 {
            tentacles.step(DT);
        }
        // Well under a millisecond a step even on a slow, loaded machine.
        assert!(start.elapsed().as_secs_f32() < 1.0);
    }
}
