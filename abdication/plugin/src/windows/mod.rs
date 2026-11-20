//! Windows (Jeremy's, Nov 2026): a base wave, and three interrupts, each a
//! window in the base's cycle through which a hidden wave shows. The hidden
//! wave is aligned to the base cycle's start and shifted by the interrupt's
//! Phase, so moving a window (X) scans across it. The fifth engine for an
//! oscillator slot, played in Rust per unison copy as Ray is. See README
//! ("Windows").
//!
//! Each cycle's settings are latched when the base's phase wraps, so a window
//! never moves while it's open. The wave is the base plus every open window's
//! hidden wave, all at 1 / (1 + the interrupts switched on), less the cycle's
//! mean (exact, from the shapes' integrals). Every jump and corner in the
//! cycle (a window opening or closing, a wave's own edges, a change of
//! settings at the wrap) is corrected at its time within the sample:
//! polyBLEP for jumps, polyBLAMP for changes of slope, one sample late.
//!
//! An interrupt's Pitch multiplies its hidden wave's frequency inside the
//! window: shape(r·t + phase), t the base's phase. It still counts from the
//! base cycle's start, so it restarts every base cycle (hard sync), and any r
//! stays harmonic with the note. r is held so r × the slot's pitch stays
//! under MAX_STEP (cycles per sample), as the FM transient reins in high notes.

pub mod player;

use std::f64::consts::TAU;

/// The interrupts per slot.
pub const INTERRUPTS: usize = 3;
/// Pitch's range, as a ratio (the parameter is in octaves of it, -2..6).
pub const MIN_RATIO: f64 = 0.25;
pub const MAX_RATIO: f64 = 64.0;
/// The most jumps and corners a cycle can have: the base's two, and each
/// window's opening and closing, and its hidden wave's: two edges per period
/// over up to half a cycle at MAX_RATIO, plus one more period's worth where
/// the window wraps (it starts over at the base's start).
const MAX_EVENTS: usize = 2 + INTERRUPTS * (2 + 2 * (MAX_RATIO as usize / 2 + 2));
/// How close (in cycles) two phases count as one edge.
const EDGE_TOLERANCE: f64 = 1e-11;
/// A window shorter than this (a share of the cycle) is closed.
const MIN_LEN: f64 = 1e-6;
/// The fastest a phase may run (cycles per sample), well under Nyquist.
const MAX_STEP: f64 = 0.45;

/// The waves. Each starts at 0 and rises where it can, as Abdication's basic
/// shapes do (engines.lib's `naive`): the saw jumps half way, the ramp is the
/// saw turned over, the square is high for the first half.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Shape {
    Sine,
    Cosine,
    Triangle,
    #[default]
    Saw,
    Ramp,
    Square,
}

/// Which way a value is approached at a point: from before it, or after.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Side {
    Before,
    After,
}

/// Phase `q` in one cycle: [0, 1) approached from after, (0, 1] from before
/// (so a point where something happens belongs to its own side).
fn wrap(q: f64, side: Side) -> f64 {
    let r = q - q.floor();
    if side == Side::Before && r == 0.0 { 1.0 } else { r }
}

impl Shape {
    /// The Base menu (voice.lib's windowsControls), in order.
    pub const BASE: [Shape; 6] = [Shape::Sine, Shape::Cosine, Shape::Triangle, Shape::Saw, Shape::Ramp, Shape::Square];
    /// An interrupt's Shape menu: no Cosine (its Phase covers it).
    pub const INTERRUPT: [Shape; 5] = [Shape::Sine, Shape::Triangle, Shape::Saw, Shape::Ramp, Shape::Square];

    fn pick(menu: &[Shape], value: f32) -> Shape {
        menu[(value.max(0.0).round() as usize).min(menu.len() - 1)]
    }

    /// The value at phase `q`, approached from `side`, and the slope there
    /// (per cycle).
    fn limit(self, q: f64, side: Side) -> (f64, f64) {
        let r = wrap(q, side);
        // Whether r is before point c (at c itself, a value from before is).
        let before = |c: f64| if side == Side::Before { r <= c } else { r < c };
        let saw = || if before(0.5) { (2.0 * r, 2.0) } else { (2.0 * r - 2.0, 2.0) };
        match self {
            Shape::Sine => ((TAU * r).sin(), TAU * (TAU * r).cos()),
            Shape::Cosine => ((TAU * r).cos(), -TAU * (TAU * r).sin()),
            Shape::Triangle if before(0.25) => (4.0 * r, 4.0),
            Shape::Triangle if before(0.75) => (2.0 - 4.0 * r, -4.0),
            Shape::Triangle => (4.0 * r - 4.0, 4.0),
            Shape::Saw => saw(),
            Shape::Ramp => {
                let (v, s) = saw();
                (-v, -s)
            }
            Shape::Square => (if before(0.5) { 1.0 } else { -1.0 }, 0.0),
        }
    }

    /// The value at phase `q` (the one a sample plays).
    fn value(self, q: f64) -> f64 {
        self.limit(q, Side::After).0
    }

    /// An integral of the shape (per cycle) up to phase `q`: continuous and
    /// periodic, as every shape here has no DC.
    fn integral(self, q: f64) -> f64 {
        let r = wrap(q, Side::After);
        let saw = if r < 0.5 { r * r } else { (1.0 - r) * (1.0 - r) };
        match self {
            Shape::Sine => -(TAU * r).cos() / TAU,
            Shape::Cosine => (TAU * r).sin() / TAU,
            Shape::Triangle if r < 0.25 => 2.0 * r * r,
            Shape::Triangle if r < 0.75 => 2.0 * r - 2.0 * r * r - 0.25,
            Shape::Triangle => 2.0 * r * r - 4.0 * r + 2.0,
            Shape::Saw => saw,
            Shape::Ramp => -saw,
            Shape::Square => if r < 0.5 { r } else { 1.0 - r },
        }
    }

    /// Where in its cycle the shape jumps or turns.
    fn edges(self) -> &'static [f64] {
        match self {
            Shape::Sine | Shape::Cosine => &[],
            Shape::Triangle => &[0.25, 0.75],
            Shape::Saw | Shape::Ramp => &[0.5],
            Shape::Square => &[0.0, 0.5],
        }
    }
}

/// One interrupt's controls, as the slot has them.
#[derive(Clone, Copy, Debug, Default)]
pub struct Interrupt {
    pub on: bool,
    pub shape: Shape,
    /// Where the window opens, a share of the cycle (0..0.99).
    pub x: f32,
    /// The hidden wave's shift against the base cycle (degrees, 0..360).
    pub phase: f32,
    /// How long the window stays open, a share of the cycle (0..0.5).
    pub len: f32,
    /// Octaves the hidden wave runs above the base inside the window (-2..6:
    /// ×0.25..×64).
    pub pitch: f32,
}

/// What a slot's Windows reads (params::WINDOWS_FIELDS, in that order).
#[derive(Clone, Copy, Debug, Default)]
pub struct WindowsSettings {
    pub mode: f32,
    pub fader: f32,
    pub base: Shape,
    pub interrupts: [Interrupt; INTERRUPTS],
}

/// Each interrupt's controls in params::WINDOWS_FIELDS.
pub const FIELDS: usize = 6;

/// The slot modes (voice.lib's Mode menu): Windows is the plugin's.
pub const MODE_WINDOWS: f32 = 4.0;

impl WindowsSettings {
    /// From the values of params::WINDOWS_FIELDS, in order: the mode, the
    /// fader, the base, then each interrupt's On, Shape, X, Phase, Len and Pitch.
    pub fn from_values(v: [f32; 3 + FIELDS * INTERRUPTS]) -> Self {
        let interrupts = std::array::from_fn(|i| {
            let [on, shape, x, phase, len, pitch] = std::array::from_fn(|f| v[3 + FIELDS * i + f]);
            Interrupt { on: on > 0.5, shape: Shape::pick(&Shape::INTERRUPT, shape), x, phase, len, pitch }
        });
        Self { mode: v[0], fader: v[1], base: Shape::pick(&Shape::BASE, v[2]), interrupts }
    }

    /// The slot plays Windows, and sounds.
    pub fn playing(&self) -> bool {
        self.mode == MODE_WINDOWS && self.fader != 0.0
    }
}

/// An open window, in shares of the cycle.
#[derive(Clone, Copy, Debug, Default)]
struct Window {
    shape: Shape,
    x: f64,
    phase: f64,
    len: f64,
    /// The hidden wave's cycles per base cycle (Pitch, held under the ceiling).
    ratio: f64,
}

impl Window {
    /// Whether phase `p`, approached from `side`, is inside the window
    /// ([x, x + len), carried on from the cycle's start past its end).
    fn holds(&self, p: f64, side: Side) -> bool {
        // Edges found as x + len (or merged with another edge nearby) can land
        // a rounding error off: within EDGE_TOLERANCE of the opening counts
        // as the opening itself.
        let into = match (p - self.x).rem_euclid(1.0) {
            r if r < EDGE_TOLERANCE || r > 1.0 - EDGE_TOLERANCE => if side == Side::Before { 1.0 } else { 0.0 },
            r => r,
        };
        if side == Side::Before { into <= self.len + EDGE_TOLERANCE } else { into < self.len - EDGE_TOLERANCE }
    }

    /// The stretches of the base cycle it covers (one, or two if it wraps).
    fn segments(&self) -> impl Iterator<Item = (f64, f64)> {
        let end = self.x + self.len;
        let first = (self.x, end.min(1.0));
        let second = (end > 1.0).then_some((0.0, end - 1.0));
        std::iter::once(first).chain(second)
    }

    /// The hidden wave's own phase at the base's phase `t`. An edge found
    /// from it comes back a rounding error off, so phases that close to a
    /// shape's edges (all quarters of its cycle) land on them exactly.
    fn at(&self, t: f64) -> f64 {
        let q = self.ratio * t + self.phase;
        let edge = (q * 4.0).round() / 4.0;
        if (q - edge).abs() < EDGE_TOLERANCE { edge } else { q }
    }

    /// The hidden wave's value and slope (per base cycle) at base phase `p`.
    fn limit(&self, p: f64, side: Side) -> (f64, f64) {
        let (v, s) = self.shape.limit(self.at(p), side);
        (v, s * self.ratio)
    }

    /// Its integral over the window (in base cycles).
    fn area(&self) -> f64 {
        self.segments()
            .map(|(a, b)| (self.shape.integral(self.at(b)) - self.shape.integral(self.at(a))) / self.ratio)
            .sum()
    }
}

/// A slot's Pitch for the hidden wave, in octaves, as the ratio it plays at
/// a base pitch of `step` cycles per sample: ×0.25..×64, held so the hidden
/// wave stays under MAX_STEP.
pub fn ratio(octaves: f32, step: f64) -> f64 {
    let r = (octaves as f64).exp2().clamp(MIN_RATIO, MAX_RATIO);
    if step > 0.0 { r.min((MAX_STEP / step).max(MIN_RATIO)) } else { r }
}

/// A jump of `h` and a change of slope of `d` (per cycle) at phase `at`.
#[derive(Clone, Copy, Debug, Default)]
struct Event {
    at: f64,
    h: f64,
    d: f64,
}

/// One cycle's wave, from the settings latched at its start.
#[derive(Clone, Copy, Debug)]
struct Cycle {
    base: Shape,
    windows: [Window; INTERRUPTS],
    open: usize,
    gain: f64,
    /// The cycle's mean, taken off.
    mean: f64,
    /// Its jumps and corners inside the cycle (0 < at < 1), in order; the
    /// one at the wrap is worked out against the next cycle.
    events: [Event; MAX_EVENTS],
    count: usize,
}

impl Default for Cycle {
    fn default() -> Self {
        Cycle { base: Shape::default(), windows: [Window::default(); INTERRUPTS], open: 0, gain: 1.0, mean: 0.0,
                events: [Event::default(); MAX_EVENTS], count: 0 }
    }
}

impl Cycle {
    /// A cycle on settings `s`, the base running `step` cycles per sample
    /// (which holds each hidden wave's Pitch under the ceiling).
    fn latch(s: &WindowsSettings, step: f64) -> Self {
        let on = s.interrupts.iter().filter(|i| i.on).count();
        let mut c = Cycle { base: s.base, gain: 1.0 / (1 + on) as f64, ..Default::default() };
        for i in s.interrupts.iter().filter(|i| i.on && i.len as f64 >= MIN_LEN) {
            c.windows[c.open] = Window {
                shape: i.shape,
                x: (i.x as f64).clamp(0.0, 0.99),
                phase: (i.phase as f64 / 360.0).rem_euclid(1.0),
                len: (i.len as f64).min(0.5),
                ratio: ratio(i.pitch, step),
            };
            c.open += 1;
        }
        c.mean = c.gain * c.windows().iter().map(Window::area).sum::<f64>();
        c.find_events();
        c
    }

    fn windows(&self) -> &[Window] {
        &self.windows[..self.open]
    }

    /// The wave and its slope at phase `p`, approached from `side`.
    fn limit(&self, p: f64, side: Side) -> (f64, f64) {
        let (mut v, mut s) = self.base.limit(p, side);
        for w in self.windows().iter().filter(|w| w.holds(p, side)) {
            let (wv, ws) = w.limit(p, side);
            v += wv;
            s += ws;
        }
        (self.gain * v - self.mean, self.gain * s)
    }

    /// The wave at phase `p` (the one a sample plays).
    fn value(&self, p: f64) -> f64 {
        let windows: f64 = self.windows().iter()
            .filter(|w| w.holds(p, Side::After))
            .map(|w| w.shape.value(w.at(p)))
            .sum();
        self.gain * (self.base.value(p) + windows) - self.mean
    }

    /// Every place in the cycle where something might jump or turn, then what
    /// the whole wave does there (so things that land on the same spot add up).
    fn find_events(&mut self) {
        let mut at = [0.0; MAX_EVENTS];
        let mut n = 0;
        let mut add = |p: f64| {
            if n < MAX_EVENTS {
                at[n] = wrap(p, Side::After);
                n += 1;
            }
        };
        self.base.edges().iter().for_each(|&e| add(e));
        for w in self.windows() {
            add(w.x);
            add(w.x + w.len);
            // The hidden wave's own edges inside the window: where
            // ratio·t + phase reaches an edge, once per period.
            for (a, b) in w.segments() {
                for &e in w.shape.edges() {
                    let first = (w.at(a) - e).ceil() as i64;
                    let last = (w.at(b) - e).floor() as i64;
                    (first..=last).for_each(|k| add((e + k as f64 - w.phase) / w.ratio));
                }
            }
        }
        let at = &mut at[..n];
        at.sort_by(f64::total_cmp);
        self.count = 0;
        let mut last = f64::NAN;
        for &p in at.iter() {
            // At 0, it's the wrap's (worked out with the next cycle).
            if p <= 0.0 || p - last < 1e-12 {
                continue;
            }
            last = p;
            let (before, after) = (self.limit(p, Side::Before), self.limit(p, Side::After));
            let (h, d) = (after.0 - before.0, after.1 - before.1);
            if h.abs() > 1e-9 || d.abs() > 1e-9 {
                self.events[self.count] = Event { at: p, h, d };
                self.count += 1;
            }
        }
    }

    fn events(&self) -> &[Event] {
        &self.events[..self.count]
    }
}

/// The corrections for a jump `h` and a change of slope `d` (per sample)
/// `tau` samples ago (0..1): two samples wide, the one before the event
/// (`pre`) and the one after (`post`). engines.lib's `event`.
fn correct(pre: &mut f64, post: &mut f64, tau: f64, h: f64, d: f64) {
    let t = tau.clamp(0.0, 1.0);
    let u = 1.0 - t;
    *pre += 0.5 * h * t * t + d * t * t * t / 6.0;
    *post += d * u * u * u / 6.0 - 0.5 * h * u * u;
}

/// One Windows oscillator (a slot of one unison copy).
#[derive(Clone, Copy, Debug, Default)]
pub struct WindowsOsc {
    /// The base's phase, 0..1.
    phase: f64,
    cycle: Cycle,
    /// The cycle's next event.
    next: usize,
    /// The last sample, waiting for the corrections of events just after it.
    held: f64,
    started: bool,
}

impl WindowsOsc {
    pub fn started(&self) -> bool {
        self.started
    }

    /// Start a note at `phase` (0..1), on the settings as they are, the base
    /// running `step` cycles per sample.
    pub fn start(&mut self, phase: f64, settings: &WindowsSettings, step: f64) {
        self.phase = phase.rem_euclid(1.0);
        self.cycle = Cycle::latch(settings, step);
        self.next = self.cycle.events().iter().take_while(|e| e.at <= self.phase).count();
        self.held = self.cycle.value(self.phase);
        self.started = true;
    }

    pub fn clear(&mut self) {
        *self = Self::default();
    }

    /// Play `out.len()` samples, the phase moving from `steps.0` to `steps.1`
    /// cycles per sample over them; a new cycle latches `settings`.
    pub fn render(&mut self, settings: &WindowsSettings, steps: (f64, f64), out: &mut [f32]) {
        let n = out.len() as f64;
        for (i, o) in out.iter_mut().enumerate() {
            let dt = (steps.0 + (steps.1 - steps.0) * (i + 1) as f64 / n).clamp(1e-9, MAX_STEP);
            let mut t = self.phase + dt;
            let (mut pre, mut post) = (0.0, 0.0);
            // This cycle's events up to here (or to its end).
            let end = t.min(1.0);
            while let Some(e) = self.cycle.events().get(self.next).filter(|e| e.at <= end) {
                correct(&mut pre, &mut post, (t - e.at) / dt, e.h, e.d * dt);
                self.next += 1;
            }
            if t >= 1.0 {
                // The wrap: a new cycle on the settings as they are now, and
                // the step from the old one's end to its start.
                t -= 1.0;
                let next = Cycle::latch(settings, dt);
                let (v1, s1) = self.cycle.limit(1.0, Side::Before);
                let (v0, s0) = next.limit(0.0, Side::After);
                correct(&mut pre, &mut post, t / dt, v0 - v1, (s0 - s1) * dt);
                self.cycle = next;
                self.next = 0;
                while let Some(e) = self.cycle.events().get(self.next).filter(|e| e.at <= t) {
                    correct(&mut pre, &mut post, (t - e.at) / dt, e.h, e.d * dt);
                    self.next += 1;
                }
            }
            self.phase = t;
            *o = (self.held + pre) as f32;
            self.held = self.cycle.value(t) + post;
        }
    }
}

/// The scope's picture of a cycle: where each open window sits (base phase,
/// 0..1). The cycle split at every window edge into spans, each with the
/// windows lit over it (bit i for interrupt i); a wrapped window lights two.
pub fn spans(settings: &WindowsSettings) -> Vec<(f64, f64, u8)> {
    let open: Vec<(usize, Vec<(f64, f64)>)> = settings.interrupts.iter().enumerate()
        .filter(|(_, i)| i.on && i.len as f64 >= MIN_LEN)
        .map(|(k, i)| {
            let w = Window { x: (i.x as f64).clamp(0.0, 0.99), len: (i.len as f64).min(0.5), ..Default::default() };
            (k, w.segments().collect())
        })
        .collect();
    let mut cuts = vec![0.0, 1.0];
    open.iter().flat_map(|(_, segs)| segs).for_each(|&(a, b)| cuts.extend([a, b]));
    cuts.sort_by(f64::total_cmp);
    cuts.dedup_by(|a, b| (*a - *b).abs() < 1e-9);
    cuts.windows(2)
        .map(|c| {
            let mid = 0.5 * (c[0] + c[1]);
            let lit = open.iter()
                .filter(|(_, segs)| segs.iter().any(|&(a, b)| a <= mid && mid < b))
                .fold(0u8, |m, (k, _)| m | 1 << k);
            (c[0], c[1], lit)
        })
        .collect()
}

/// Each open window's edges (opening, closing) and its interrupt, for the
/// scope's lines.
pub fn edges(settings: &WindowsSettings) -> Vec<(f64, usize)> {
    settings.interrupts.iter().enumerate()
        .filter(|(_, i)| i.on && i.len as f64 >= MIN_LEN)
        .flat_map(|(k, i)| {
            let x = (i.x as f64).clamp(0.0, 0.99);
            [(x, k), ((x + (i.len as f64).min(0.5)).rem_euclid(1.0), k)]
        })
        .collect()
}

/// One cycle of the wave on `settings` at `n` points from the base cycle's
/// start (plain, not band-limited): what the scope lines its trace up with.
pub fn ideal(settings: &WindowsSettings, step: f64, n: usize) -> Vec<f32> {
    let c = Cycle::latch(settings, step);
    (0..n).map(|k| c.value(k as f64 / n as f64) as f32).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A base step for the tests (about 48 Hz at 48 kHz).
    const STEP: f64 = 1e-3;

    impl Cycle {
        fn latch_at(s: &WindowsSettings) -> Self {
            Cycle::latch(s, STEP)
        }
    }

    fn settings(base: Shape, interrupts: &[(Shape, f32, f32, f32)]) -> WindowsSettings {
        let mut s = WindowsSettings { mode: MODE_WINDOWS, fader: 1.0, base, ..Default::default() };
        for (slot, &(shape, x, phase, len)) in s.interrupts.iter_mut().zip(interrupts) {
            *slot = Interrupt { on: true, shape, x, phase, len, pitch: 0.0 };
        }
        s
    }

    /// The sum of a cycle, sampled finely (so its mean).
    fn mean(c: &Cycle) -> f64 {
        let n = 200_000;
        (0..n).map(|k| c.value((k as f64 + 0.5) / n as f64)).sum::<f64>() / n as f64
    }

    #[test]
    fn spans_light_the_windows() {
        let s = settings(Shape::Saw, &[(Shape::Sine, 0.1, 0.0, 0.3), (Shape::Sine, 0.3, 0.0, 0.2),
                                       (Shape::Sine, 0.9, 0.0, 0.2)]);
        let sp = spans(&s);
        let lit = |p: f64| sp.iter().find(|(a, b, _)| *a <= p && p < *b).unwrap().2;
        assert_eq!(lit(0.05), 0b100); // the third, wrapped
        assert_eq!(lit(0.2), 0b001);
        assert_eq!(lit(0.35), 0b011);
        assert_eq!(lit(0.45), 0b010);
        assert_eq!(lit(0.6), 0);
        assert_eq!(lit(0.95), 0b100);
        assert!((sp.first().unwrap().0, sp.last().unwrap().1) == (0.0, 1.0));
        assert_eq!(edges(&s).len(), 6);
    }

    #[test]
    fn integrals_are_the_shapes_areas() {
        for shape in Shape::BASE {
            for (a, b) in [(0.0, 0.3), (0.2, 0.7), (0.6, 1.1), (0.45, 0.55)] {
                let n = 100_000;
                let area = (0..n).map(|k| shape.value(a + (b - a) * (k as f64 + 0.5) / n as f64)).sum::<f64>() * (b - a) / n as f64;
                let exact = shape.integral(b) - shape.integral(a);
                assert!((area - exact).abs() < 1e-5, "{shape:?} {a}..{b}: {area} vs {exact}");
            }
        }
    }

    #[test]
    fn a_cycle_has_no_dc() {
        let s = settings(Shape::Saw, &[(Shape::Sine, 0.1, 0.0, 0.5), (Shape::Square, 0.8, 90.0, 0.4),
                                       (Shape::Triangle, 0.4, 200.0, 0.2)]);
        let c = Cycle::latch(&s, STEP);
        assert!(mean(&c).abs() < 1e-4, "{}", mean(&c));
        assert!(c.mean.abs() > 0.05, "the test should have DC to take off");
    }

    #[test]
    fn no_interrupts_is_the_base() {
        let c = Cycle::latch_at(&settings(Shape::Saw, &[]));
        assert_eq!(c.gain, 1.0);
        for k in 0..100 {
            let p = k as f64 / 100.0;
            assert!((c.value(p) - Shape::Saw.value(p)).abs() < 1e-12);
        }
        // The saw's jump half way is its only event, -2.
        assert_eq!(c.events().len(), 1);
        assert!((c.events()[0].h + 2.0).abs() < 1e-12);
    }

    #[test]
    fn interrupts_switched_on_share_the_gain() {
        let mut s = settings(Shape::Sine, &[(Shape::Sine, 0.0, 0.0, 0.0), (Shape::Sine, 0.5, 0.0, 0.0)]);
        assert_eq!(Cycle::latch(&s, STEP).gain, 1.0 / 3.0);
        s.interrupts[1].on = false;
        assert_eq!(Cycle::latch(&s, STEP).gain, 0.5);
    }

    #[test]
    fn a_window_peeks_at_the_hidden_wave() {
        // A sine window over a sine, phase 0: the base doubled inside it.
        let c = Cycle::latch_at(&settings(Shape::Sine, &[(Shape::Sine, 0.1, 0.0, 0.2)]));
        let raw = |p: f64| c.value(p) + c.mean;
        assert!((raw(0.2) - (TAU * 0.2).sin()).abs() < 1e-12);
        assert!((raw(0.5) - 0.5 * (TAU * 0.5).sin()).abs() < 1e-12);
        // Phase 180: it cancels the base inside the window.
        let c = Cycle::latch_at(&settings(Shape::Sine, &[(Shape::Sine, 0.1, 180.0, 0.2)]));
        assert!((c.value(0.2) + c.mean).abs() < 1e-12);
    }

    #[test]
    fn windows_past_the_end_carry_on_at_the_start() {
        let c = Cycle::latch_at(&settings(Shape::Sine, &[(Shape::Square, 0.9, 0.0, 0.3)]));
        let w = c.windows()[0];
        assert!(w.holds(0.95, Side::After) && w.holds(0.1, Side::After));
        assert!(!w.holds(0.25, Side::After) && !w.holds(0.5, Side::After));
    }

    /// Events' jumps are what the wave does there.
    #[test]
    fn events_match_the_wave() {
        let c = Cycle::latch_at(&settings(Shape::Square, &[(Shape::Saw, 0.2, 45.0, 0.3), (Shape::Triangle, 0.35, 0.0, 0.1),
                                                        (Shape::Ramp, 0.7, 300.0, 0.5)]));
        for e in c.events() {
            let jump = c.value(e.at + 1e-9) - c.value(e.at - 1e-9);
            let (b, a) = (c.limit(e.at, Side::Before), c.limit(e.at, Side::After));
            assert!((jump - e.h).abs() < 1e-6, "{e:?}: {jump} limits {b:?} {a:?} values {} {}", c.value(e.at - 1e-7), c.value(e.at + 1e-7));
        }
        // Nothing jumps where there's no event.
        let mut at: Vec<f64> = c.events().iter().map(|e| e.at).collect();
        at.push(0.0);
        at.push(1.0);
        for k in 1..10_000 {
            let p = k as f64 / 10_000.0;
            if at.iter().all(|a| (a - p).abs() > 2e-4) {
                assert!((c.value(p + 1e-4) - c.value(p - 1e-4)).abs() < 0.01, "a jump at {p}");
            }
        }
    }

    /// Pitch hard-syncs the hidden wave: ×2 shows it at twice the rate inside
    /// the window, restarted at the base cycle's start, and ×1 is unchanged.
    #[test]
    fn pitch_multiplies_the_hidden_wave() {
        let mut s = settings(Shape::Sine, &[(Shape::Saw, 0.1, 30.0, 0.4)]);
        let plain = Cycle::latch_at(&s);
        s.interrupts[0].pitch = 0.0;
        assert_eq!(Cycle::latch_at(&s).windows()[0].ratio, 1.0);
        s.interrupts[0].pitch = 1.0;
        let c = Cycle::latch_at(&s);
        let mut s4 = s;
        s4.interrupts[0].pitch = 2.0;
        let hidden = |p: f64| (c.value(p) + c.mean) / c.gain - Shape::Sine.value(p);
        for p in [0.15, 0.3, 0.42] {
            assert!((hidden(p) - Shape::Saw.value(2.0 * p + 30.0 / 360.0)).abs() < 1e-9);
        }
        assert!((hidden(0.7)).abs() < 1e-12 && plain.windows()[0].ratio == 1.0);
        // Its edges: a saw's jump every half period of the window now.
        assert!(Cycle::latch_at(&s4).events().len() > plain.events().len());
    }

    /// The ratio's range and the ceiling: ×0.25..×64, held so the hidden wave
    /// stays under 45 % of the sample rate.
    #[test]
    fn pitch_range_and_ceiling() {
        assert_eq!(ratio(-2.0, 1e-3), 0.25);
        assert_eq!(ratio(6.0, 1e-3), 64.0);
        assert_eq!(ratio(9.0, 1e-3), 64.0);
        let step = 1_000.0 / 48_000.0;
        assert!((ratio(6.0, step) - MAX_STEP / step).abs() < 1e-12);
        assert!(ratio(6.0, step) * step <= MAX_STEP + 1e-12);
    }

    /// With Pitch up and windows wrapping, the events still match the wave,
    /// and the mean is still exact.
    #[test]
    fn pitched_events_and_mean() {
        let mut s = settings(Shape::Triangle, &[(Shape::Square, 0.85, 45.0, 0.4), (Shape::Saw, 0.3, 0.0, 0.25),
                                               (Shape::Ramp, 0.5, 300.0, 0.33)]);
        for (i, oct) in [2.3f32, -1.5, 5.0].into_iter().enumerate() {
            s.interrupts[i].pitch = oct;
        }
        let c = Cycle::latch_at(&s);
        assert!(mean(&c).abs() < 1e-4, "{}", mean(&c));
        for e in c.events() {
            let jump = c.value(e.at + 1e-9) - c.value(e.at - 1e-9);
            assert!((jump - e.h).abs() < 1e-5, "{e:?}: {jump}");
        }
        let mut at: Vec<f64> = c.events().iter().map(|e| e.at).collect();
        at.extend([0.0, 1.0]);
        for k in 1..20_000 {
            let p = k as f64 / 20_000.0;
            if at.iter().all(|a| (a - p).abs() > 1e-4) {
                assert!((c.value(p + 2e-5) - c.value(p - 2e-5)).abs() < 0.02, "a jump at {p}: {:?}", c.events());
            }
        }
        // The most events a cycle can hold: three half-cycle squares at ×64.
        let mut s = settings(Shape::Square, &[(Shape::Square, 0.0, 0.0, 0.5), (Shape::Square, 0.6, 0.0, 0.5),
                                              (Shape::Square, 0.33, 0.0, 0.5)]);
        s.interrupts.iter_mut().for_each(|i| i.pitch = 6.0);
        let c = Cycle::latch_at(&s);
        assert!(c.count < MAX_EVENTS && c.count > 100, "{}", c.count);
    }

    /// Played, it's finite, within a sensible range, and band-limited: far
    /// less aliasing than the plain wave sampled.
    #[test]
    fn plays_band_limited() {
        let s = settings(Shape::Saw, &[(Shape::Square, 0.13, 37.0, 0.21), (Shape::Sine, 0.52, 90.0, 0.33),
                                       (Shape::Ramp, 0.81, 250.0, 0.4)]);
        let rate = 48_000.0;
        let hz = 1_234.5;
        let step = hz / rate;
        let n = 1 << 15;
        let mut osc = WindowsOsc::default();
        osc.start(0.0, &s, step);
        let mut out = vec![0.0f32; n];
        for block in out.chunks_mut(64) {
            osc.render(&s, (step, step), block);
        }
        assert!(out.iter().all(|x| x.is_finite() && x.abs() < 2.0));
        // The plain wave, sampled.
        let cycle = Cycle::latch(&s, STEP);
        let plain: Vec<f32> = (0..n).map(|k| cycle.value((k as f64 * step).fract()) as f32).collect();
        let alias = |x: &[f32]| crate::windows::player::tests::alias_db(x, hz, rate);
        let (a, b) = (alias(&out[1..]), alias(&plain));
        assert!(a < b - 15.0, "aliasing {a:.1} dB band-limited vs {b:.1} dB plain");
        println!("aliasing at ×1: {a:.1} dB band-limited, {b:.1} dB plain");
    }

    /// The same with Pitch up (×3, ×0.7, ×11): still band-limited.
    #[test]
    fn plays_band_limited_pitched() {
        let mut s = settings(Shape::Saw, &[(Shape::Square, 0.13, 37.0, 0.21), (Shape::Sine, 0.52, 90.0, 0.33),
                                           (Shape::Ramp, 0.81, 250.0, 0.4)]);
        for (i, r) in [3.0f32, 0.7, 11.0].into_iter().enumerate() {
            s.interrupts[i].pitch = r.log2();
        }
        let rate = 48_000.0;
        let hz = 523.25;
        let step = hz / rate;
        let n = 1 << 15;
        let mut osc = WindowsOsc::default();
        osc.start(0.0, &s, step);
        let mut out = vec![0.0f32; n];
        for block in out.chunks_mut(64) {
            osc.render(&s, (step, step), block);
        }
        assert!(out.iter().all(|x| x.is_finite() && x.abs() < 2.0));
        let cycle = Cycle::latch(&s, step);
        let plain: Vec<f32> = (0..n).map(|k| cycle.value((k as f64 * step).fract()) as f32).collect();
        let alias = |x: &[f32]| crate::windows::player::tests::alias_db(x, hz, rate);
        let (a, b) = (alias(&out[1..]), alias(&plain));
        println!("aliasing pitched: {a:.1} dB band-limited, {b:.1} dB plain");
        assert!(a < b - 7.0, "aliasing {a:.1} dB band-limited vs {b:.1} dB plain");
    }
}
