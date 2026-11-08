//! The sky behind the controls: a fixed field of stars, some breathing slowly and a few
//! flaring now and then; an eye only suggested by stars along the curve of two lids; and
//! a line from a book that was never written, once a minute.

use std::f32::consts::TAU;

use crate::dsp::random::Rng;

const STARS: usize = 160;
const PULSING: f32 = 0.22;
const TWINKLING: f32 = 0.05;
/// No stars inside the void, nor just round it.
const CLEAR_OF_VOID: f32 = 18.0;
/// The eye's stars all breathe together, on one slow cycle.
const EYE_PERIOD_S: f32 = 11.0;
const EYE_STARS_MIN: f32 = 10.0;
const EYE_STARS_MAX: f32 = 56.0;
/// Where a flaring star spends its moment, as a share of its cycle.
const TWINKLE_SHARE: f32 = 0.07;

static LINES: &str = include_str!("../../assets/lines.txt");
pub const LINE_PERIOD_S: f32 = 60.0;
const LINE_FADE_S: f32 = 3.0;
const LINE_HOLD_S: f32 = 9.0;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Kind {
    Still,
    Pulse,
    Twinkle,
}

#[derive(Clone, Copy, Debug)]
pub struct Star {
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub base: f32,
    pub kind: Kind,
    period: f32,
    phase: f32,
}

impl Star {
    /// Brightness (0..1) and size multiplier at time `t` seconds.
    pub fn at(&self, t: f32) -> (f32, f32) {
        match self.kind {
            Kind::Still => (self.base, 1.0),
            Kind::Pulse => {
                let breath = 0.5 - 0.5 * (TAU * (t / self.period + self.phase)).cos();
                (self.base * (1.0 - 0.7 * breath), 1.0)
            }
            Kind::Twinkle => {
                let u = (t / self.period + self.phase).fract();
                let into = (u - (1.0 - TWINKLE_SHARE)) / TWINKLE_SHARE;
                if into > 0.0 {
                    let bump = (std::f32::consts::PI * into).sin();
                    (self.base * (0.55 + 0.45 * bump), 1.0 + 1.4 * bump)
                } else {
                    (self.base * 0.55, 1.0)
                }
            }
        }
    }
}

/// The void the panel is drawn round: centre and radius.
#[derive(Clone, Copy)]
pub struct Void {
    pub x: f32,
    pub y: f32,
    pub r: f32,
}

impl Void {
    fn clear(&self, x: f32, y: f32) -> bool {
        (x - self.x).hypot(y - self.y) >= self.r + CLEAR_OF_VOID
    }
}

/// The field of stars: the same every time the panel opens.
pub fn field(width: f32, height: f32, void: Void) -> Vec<Star> {
    let mut rng = Rng::new(417);
    let mut stars = Vec::with_capacity(STARS);
    while stars.len() < STARS {
        let (x, y, m, roll) = (rng.unit() * width, rng.unit() * height, rng.unit(), rng.unit());
        if !void.clear(x, y) {
            continue;
        }
        let size = if m > 0.97 { 2.6 } else if m > 0.85 { 1.8 } else { 1.1 };
        let star = if roll < TWINKLING {
            Star { x, y, size: size + 0.4, base: 0.85, kind: Kind::Twinkle, period: rng.range(18.0, 52.0), phase: rng.unit() }
        } else if roll < TWINKLING + PULSING {
            Star { x, y, size, base: rng.range(0.3, 0.8), kind: Kind::Pulse, period: rng.range(7.0, 18.0), phase: rng.unit() }
        } else {
            Star { x, y, size, base: 0.16 + 0.6 * rng.unit() * rng.unit(), kind: Kind::Still, period: 1.0, phase: 0.0 }
        };
        stars.push(star);
    }
    stars
}

/// Stars loosely along the two lids of an eye round the void, breathing together. How
/// many depends on Dread.
pub fn eye(dread: f32, void: Void) -> Vec<Star> {
    let mut rng = Rng::new(666);
    let count = (EYE_STARS_MIN + (EYE_STARS_MAX - EYE_STARS_MIN) * dread.clamp(0.0, 1.0)).round() as usize;
    let (left, right, reach) = (void.x - 240.0, void.x + 240.0, 250.0);
    let mut stars = Vec::with_capacity(count);
    for i in 0..count {
        let t = rng.range(0.04, 0.96);
        let side = if i % 2 == 0 { -1.0 } else { 1.0 };
        let x = left + (right - left) * t + rng.gaussian() * 6.0;
        let y = void.y + side * 2.0 * t * (1.0 - t) * reach + rng.gaussian() * 6.0;
        let size = if rng.unit() > 0.8 { 1.8 } else { 1.2 };
        let base = rng.range(0.35, 0.7);
        if void.clear(x, y) {
            stars.push(Star { x, y, size, base, kind: Kind::Pulse, period: EYE_PERIOD_S, phase: 0.0 });
        }
    }
    stars
}

/// The lines, in an order shuffled once per panel, so none repeats until all have shown.
pub struct Book {
    lines: Vec<&'static str>,
    order: Vec<usize>,
}

impl Book {
    pub fn new(seed: u32) -> Self {
        let lines: Vec<&'static str> = LINES.lines().map(str::trim).filter(|l| !l.is_empty()).collect();
        let mut order: Vec<usize> = (0..lines.len()).collect();
        let mut rng = Rng::new(seed);
        for i in (1..order.len()).rev() {
            order.swap(i, rng.below(i + 1));
        }
        Self { lines, order }
    }

    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.lines.len()
    }

    /// The line showing `t` seconds after the panel opened, and how visible it is.
    pub fn at(&self, t: f32) -> Option<(&'static str, f32)> {
        if self.lines.is_empty() {
            return None;
        }
        let n = (t / LINE_PERIOD_S) as usize;
        let into = t - n as f32 * LINE_PERIOD_S;
        let alpha = if into < LINE_FADE_S {
            into / LINE_FADE_S
        } else if into < LINE_FADE_S + LINE_HOLD_S {
            1.0
        } else {
            (2.0 * LINE_FADE_S + LINE_HOLD_S - into) / LINE_FADE_S
        };
        (alpha > 0.0).then(|| (self.lines[self.order[n % self.order.len()]], alpha.min(1.0)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VOID: Void = Void { x: 300.0, y: 300.0, r: 110.0 };

    #[test]
    fn the_void_is_starless() {
        let stars: Vec<Star> = field(980.0, 620.0, VOID).into_iter().chain(eye(1.0, VOID)).collect();
        assert!(stars.iter().all(|s| (s.x - VOID.x).hypot(s.y - VOID.y) >= VOID.r));
    }

    #[test]
    fn dread_decides_how_much_of_the_eye_there_is() {
        assert!(eye(1.0, VOID).len() > 3 * eye(0.0, VOID).len());
    }

    #[test]
    fn some_stars_move_and_most_do_not() {
        let stars = field(980.0, 620.0, VOID);
        let moving = stars.iter().filter(|s| s.kind != Kind::Still).count();
        assert!(moving > 20 && moving < 80, "{moving}");
        assert!(stars.iter().all(|s| (0..600).all(|i| {
            let (a, k) = s.at(i as f32 * 0.1);
            (0.0..=1.0).contains(&a) && (1.0..=2.4).contains(&k)
        })));
    }

    #[test]
    fn there_are_250_lines_and_each_comes_once_before_any_repeats() {
        let book = Book::new(3);
        assert_eq!(book.len(), 250);
        let mut seen = std::collections::HashSet::new();
        for n in 0..250 {
            let (line, alpha) = book.at(n as f32 * LINE_PERIOD_S + 5.0).unwrap();
            assert_eq!(alpha, 1.0);
            assert!(seen.insert(line));
        }
    }

    #[test]
    fn a_line_fades_in_lingers_fades_out_and_leaves_silence() {
        let book = Book::new(1);
        assert!(book.at(1.5).unwrap().1 > 0.4 && book.at(1.5).unwrap().1 < 0.6);
        assert_eq!(book.at(8.0).unwrap().1, 1.0);
        assert!(book.at(13.5).unwrap().1 < 0.6);
        assert!(book.at(30.0).is_none());
    }
}
