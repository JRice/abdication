//! The panel, after the mockup: a dark copper-plate map that drifts slowly left, with
//! ships and beasts, and over it the controls in engraved brass. No text but the title:
//! the host has every parameter's name.
//!
//! Knobs drag up and down (Shift is fine, double-click resets, scroll nudges). Each
//! armed knob's four triangles are its modulators' depths: drag one outward along its
//! own axis to deepen it, double-click to zero it. The seals (each a small scope of its
//! oscillator's last cycle) step their tables, the levers switch, the plaques drag from
//! row to row (the one there takes their old row), the Morph cartouche steps its shape
//! (left half back, right half on). The horizontal sliders (each delay's Mix, the
//! Morph's Depth) drag sideways.
//!
//! Red and slate come in three copies, I to III, one per row. Their corner displays
//! draw all three, the last-touched row's bright and the other two faded.

use std::f32::consts::{FRAC_PI_2, PI, TAU};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use std::cell::RefCell;
use std::collections::VecDeque;

use nih_plug::prelude::{Param, ParamPtr};
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::vizia::vg::{self, Paint, Path};
use nih_plug_vizia::widgets::RawParamEvent;

use super::paint::*;
use super::svg;
use super::world::{self, World, BEAST_LIFE, LANES, SPEED};
use super::{HEIGHT, WIDTH};
use crate::dsp::inserts::{order_of, rows_of, EFFECTS};
use crate::dsp::{argon, harmlib, pitch, tables};
use crate::knobs::{self, arm_angles, Mulberry, ARMED, ARMED_COUNT};
use crate::params::{osc_gain, DraconesParams};
use crate::telemetry::Telemetry;

const TICK: Duration = Duration::from_millis(40);
const MAP_INK: f32 = 0.8;
const ROWS: [f32; 3] = [236.0, 362.0, 488.0];
const OSC_X: [f32; 5] = [262.0, 350.0, 438.0, 526.0, 614.0];
const FX_X: [f32; 3] = [868.0, 952.0, 1036.0];
const POST_Y: f32 = 684.0;
const POST_X: [f32; 4] = [345.0, 430.0, 672.0, 840.0];
const SEAL_X: f32 = 172.0;
const SWITCH_X: f32 = 690.0;
const PLAQUE_X: f32 = 785.0;
const CARTOUCHE: (f32, f32) = (560.0, 684.0);
/// The slate levers, one per copy, and the pip legend left of them.
const LEVERS_X: [f32; 3] = [972.0, 1014.0, 1056.0];
const LEVER_Y: f32 = 712.0;
const PIP_LEGEND_X: f32 = 924.0;
/// Each delay's Mix: a slider under its Division, this far below and this wide each way.
const MIX_DY: f32 = 31.0;
const MIX_HALF: f32 = 22.0;
/// The Morph's Depth: a slider under the cartouche.
const DEPTH_Y: f32 = 724.0;
const DEPTH_HALF: f32 = 40.0;
/// The seals' scopes: half-width and half-height of the trace.
const SCOPE_W: f32 = 15.0;
const SCOPE_H: f32 = 10.0;
const OUT_X: f32 = 1176.0;
const OUT_TOP: f32 = 232.0;
const OUT_BOTTOM: f32 = 492.0;
/// Red's small knobs: a column per copy (Time above, Slope below).
const RED_X: [f32; 3] = [930.0, 985.0, 1040.0];
const RED_Y: [f32; 2] = [62.0, 108.0];
const NUMERALS: [&str; 3] = ["I", "II", "III"];
const CORNERS: [(f32, f32); 4] = [(88.0, 88.0), (1152.0, 88.0), (88.0, 712.0), (1152.0, 712.0)];
const KNOB_R: f32 = 17.0;
const SMALL_R: f32 = 10.0;
const KNOB_TRAVEL: f32 = 170.0;
const ARM_TRAVEL: f32 = 19.0;
const DIVISION_STEP: f32 = 22.0;
const OUT_TRAVEL: f32 = 260.0;
const FINE: f32 = 0.2;
const HIST: usize = 120;
/// Green's history keeps one point in this many ticks (about the last minute).
const GREEN_EVERY: u32 = 12;
const SPIRAL_S: f32 = 900.0;

const TITLE_TEXT: &str = "HC SVNT DRACONES";

const EMBLEMS: [&str; 3] = [
    "M-6 -5 Q1 -11 8 -5 Q1 1 -6 -5 Z M-6 -5 L-11 -9 L-11 -1 Z M5 -6 h.1 M6 5 Q-1 -1 -8 5 Q-1 11 6 5 Z M6 5 L11 1 L11 9 Z M-5 4 h.1",
    "M0 13 C-9 9 -10 -1 -3 -9 C-3 -4 0 -3 1 -6 C2 -10 0 -13 -1 -15 C7 -11 11 -3 9 4 C8 9 4 12 0 13 Z M0 10 C-4 8 -4 3 -1 -1 C0 2 2 3 3 1 C5 4 4 8 0 10 Z",
    "M3 -13 A13 13 0 1 0 3 13 A10 10 0 1 1 3 -13 Z M10 -7 h.1 M12 4 h.1 M8 10 h.1",
];

/// The column symbols: (path, x, y, scale).
const GLYPHS: [(&str, f32, f32, f32); 15] = [
    ("M-6 7 C-9 0 -7 -6 -4 -8 M6 7 C9 0 7 -6 4 -8 M-6 7 H6 M-6.5 -2 H6.5 M-2 -2 V7 M0 -2 V7 M2 -2 V7 M-4 -8 h-1.5 M4 -8 h1.5", 262.0, 182.0, 1.0),
    ("M-6 -8 H6 C6 -2 3 1 0 1 C-3 1 -6 -2 -6 -8 Z M0 1 V6 M-4 8 H4 M-2.5 6 H2.5", 350.0, 182.0, 1.0),
    ("M0 0 a1 1 0 0 1 2 0 a2 2 0 0 1 -4 0 a3 3 0 0 1 6 0 a4 4 0 0 1 -8 0 a5 5 0 0 1 10 0", 438.0, 182.0, 1.0),
    ("M0 -9 L1.6 -6 V4 H-1.6 V-6 Z M-5 4 H5 M0 4 V8 M-1.5 9 H1.5", 526.0, 182.0, 1.0),
    ("M-6 5 C-6 -1 -4 -7 0 -7 C4 -7 6 -1 6 5 Z M-8 5 H8 M0 -7 V-9 M0 8 h.1", 614.0, 182.0, 1.0),
    (HOURGLASS, 868.0, 182.0, 1.0),
    ("M5.4 -3.4 A6.4 6.4 0 1 1 2.6 -5.8 M2.6 -5.8 L7 -6.6 L5.4 -3.4 Z M4.6 -5.6 h.1", 952.0, 182.0, 1.0),
    ("M-1.5 -8 H1.5 M-1.2 -8 V-3.5 C-6 -1.5 -6 7 0 7 C6 7 6 -1.5 1.2 -3.5 V-8 M1.4 -6 L8 -3 M-3 3 H3", 1036.0, 182.0, 1.0),
    ("M-8 -1.5 H-1 L6 -6 V6 L-1 1.5 H-8 Z M-8 -3 V3", 1176.0, 182.0, 1.0),
    ("M3 -9 L-4 1 L1 1 L-3 10 L5 -2 L0 -2 Z", 345.0, 642.0, 1.0),
    ("M-6 8 C-4 0 0 -6 6 -9 C4 -2 0 4 -6 8 Z M-6 8 L3 -4 M-1 1 L-3 -1 M1 -2 L-1 -4", 430.0, 642.0, 1.0),
    ("M0 -8 V7 M-4 8 H4 M-7 -5 H7 M-7 -5 L-9.5 1 H-4.5 Z M7 -5 L4.5 1 H9.5 Z", 672.0, 642.0, 1.0),
    ("M-9 8 V-1 A9 9 0 0 1 9 -1 V8 M-5 8 V0 A5 5 0 0 1 5 0 V8 M-12 8 H12", 840.0, 642.0, 1.0),
    (HOURGLASS, 898.0, 62.0, 0.75),
    ("M-7 6 L-2 -6 L7 6 Z", 898.0, 108.0, 0.75),
];
const HOURGLASS: &str = "M-5 -8 H5 M-5 8 H5 M-4 -8 C-4 -2 4 2 4 8 M4 -8 C4 -2 -4 2 -4 8 M-1.5 6 H1.5";

/// Sections: (x0, y0, x1, y1, colour): oscillators, filter, inserts, output, post chain.
const SECTIONS: [(f32, f32, f32, f32, u32); 5] = [
    (134.0, 160.0, 480.0, 542.0, 0xc9a24a),
    (488.0, 160.0, 728.0, 542.0, 0x5f9c86),
    (744.0, 160.0, 1094.0, 542.0, 0xb0473c),
    (1148.0, 160.0, 1204.0, 542.0, 0xd9c9a0),
    (240.0, 618.0, 900.0, 736.0, 0x5072b0),
];

const BEASTS: [&str; 4] = [
    "M-30 0 q5 -12 10 0 M-14 0 q6 -15 12 0 M3 0 q5 -12 10 0 q3 -10 9 -11 q4 -1 6 2 l-3 1 M-36 3 h7 M-9 3 h9 M15 3 h9",
    "M0 2 C-2 -5 -2 -10 0 -14 C-4 -16 -10 -16 -15 -21 C-8 -21 -3 -19 0 -16 C3 -19 8 -21 15 -21 C10 -16 4 -16 0 -14 M-12 4 h24",
    "M0 2 C-3 -9 6 -14 4 -23 C3 -29 -4 -28 -3 -24 C-2 -21 2 -22 1 -25 M10 2 C9 -5 14 -8 13 -13 C12 -16 9 -15 10 -13 M-9 4 h26",
    "M-22 1 q4 -6 8 -6 q4 0 7 4 q2 -9 9 -10 q5 0 6 5 q-3 -2 -5 0 q-2 3 2 6 M-26 4 h10 M-4 4 h14",
];
const SHIP_HULL: &str = "M-16 -4 L16 -4 L11 2 L-12 2 Z M-4 -4 V-27 M7 -4 V-21 M-4 -27 l7 2 l-7 2 M-21 5 q4 -2 8 0 t8 0 M9 5 q4 -2 8 0";
const SHIP_SAILS: &str = "M-12 -23 Q-4 -19 4 -23 L4 -8 Q-4 -5 -12 -8 Z M1 -18 Q7 -15 13 -18 L13 -8 Q7 -6 1 -8 Z";
const WASP: &str = "M12 -10 a4 2.6 0 1 0 8 0 a4 2.6 0 1 0 -8 0 M15 -12.3 v4.6 M17.5 -12.1 v4.2 M15 -12 q-2 -6 2 -6.5 q1 3 -1 6.4 M17 -12 q3 -5 5 -3 q-1 3 -4 3 M21.2 -10 h.1";
const LOWPASS: &str = "M10 7 H17 q3 0 4 -2.5 q1.5 -2.5 2 2 q1 6 3 7";

fn knob_angle(v: f32) -> f32 {
    (-135.0 + 270.0 * v) * PI / 180.0
}

/// Where a point at `angle` (from straight up, clockwise) and radius `r` sits.
fn polar(angle: f32, r: f32) -> (f32, f32) {
    (angle.sin() * r, -angle.cos() * r)
}

/// One letter of the title.
struct Letter {
    c: char,
    x: f32,
    y: f32,
    k: f32,
    rot: f32,
}

/// Hand-lettered: initials larger, every letter a little off the line and tilted, all
/// following the ribbon's arch.
fn title_letters() -> Vec<Letter> {
    let mut r = Mulberry::new(10510);
    let chars: Vec<char> = TITLE_TEXT.chars().collect();
    let mut items = Vec::new();
    let mut total = 0.0;
    for (i, &ch) in chars.iter().enumerate() {
        if ch == ' ' {
            items.push((None, 0.0, 13.0));
            total += 13.0;
            continue;
        }
        let initial = i == 0 || chars[i - 1] == ' ';
        let s = if initial { 32.0 + r.next() * 2.0 } else { 23.0 + r.next() * 6.0 };
        let w = s * match ch {
            'I' => 0.4,
            'M' | 'W' => 0.95,
            _ => 0.72,
        } + 2.0;
        items.push((Some(ch), s, w));
        total += w;
    }
    let mut x = 620.0 - total / 2.0;
    let mut out = Vec::new();
    for (c, s, w) in items {
        if let Some(c) = c {
            let cx = x + w / 2.0;
            let u = (cx - 620.0) / 200.0;
            let y = 75.0 - 7.0 * (1.0 - u * u) + (r.next() - 0.5) * 4.4;
            let rot = (r.next() - 0.5) * 7.0;
            out.push(Letter { c, x: cx, y, k: s / 28.0, rot });
        }
        x += w;
    }
    out
}

/// The Morph cartouche's island for shape `idx` (none for Off).
fn isle(idx: usize) -> Option<Path> {
    if idx == 0 {
        return None;
    }
    let mut r = Mulberry::new((idx as u32).wrapping_mul(977).wrapping_add(13));
    let n = 1 + (r.next() * 3.0) as usize;
    let bumps: Vec<(f32, f32, f32)> = (0..n).map(|_| (-30.0 + r.next() * 60.0, 6.0 + r.next() * 16.0, 0.4 + r.next() * 0.6)).collect();
    let mut p = Path::new();
    p.move_to(-44.0, 10.0);
    let mut x = -44.0;
    while x <= 44.0 {
        let h: f32 = bumps.iter().map(|(c, w, a)| a * (-((x - c) / w).powi(2)).exp()).sum::<f32>().min(1.1);
        let jitter = if h > 0.05 { r.next() * 1.2 } else { 0.0 };
        p.line_to(x, 10.0 - h * 20.0 - jitter);
        x += 2.0;
    }
    p.close();
    Some(p)
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Target {
    /// An armed knob.
    Knob(usize),
    Arm(usize, usize),
    Division(usize),
    /// Red copy k/2's Time (k even) or Slope (k odd).
    Red(usize),
    Seal(usize),
    Switch(usize),
    Plaque(usize),
    Cartouche(bool),
    /// Slate copy, position.
    Lever(usize, usize),
    Output,
    /// Effect e's delay Mix.
    Mix(usize),
    Depth,
}

enum Drag {
    /// A knob or slider: the param, its value and the pointer's y (x for a horizontal
    /// slider) when grabbed.
    Value { param: ParamPtr, anchor: f32, value: f32, per_px: f32, fine: bool, horizontal: bool },
    Arm { knob: usize, m: usize, last: f32 },
    Division { e: usize, last: f32, acc: f32 },
    Plaque { e: usize, y0: f32, d0: f32 },
}

enum PanelEvent {
    Tick,
}

/// What changes between frames, kept apart so `draw(&self)` can read it.
struct Motion {
    t: f32,
    world: World,
    /// The modulators' recent values, in `Values::flat` order: silver, red I–III,
    /// green, slate I–III.
    hist: [VecDeque<f32>; 8],
    ticks: u32,
    dock: [f32; EFFECTS],
}

pub struct Panel {
    params: Arc<DraconesParams>,
    telemetry: Arc<Telemetry>,
    statics: world::Statics,
    letters: Vec<Letter>,
    arms: Vec<[f32; 4]>,
    motion: RefCell<Motion>,
    drag: Option<Drag>,
    /// The row last touched: its red and slate copies are drawn bright.
    focus: usize,
    started: Instant,
    ticking: Arc<AtomicBool>,
}

impl Drop for Panel {
    fn drop(&mut self) {
        self.ticking.store(false, Ordering::Relaxed);
    }
}

/// The world's drift: from the wall clock, so it keeps sailing while the panel is shut
/// and each opening shows another stretch of coast.
fn drift() -> f64 {
    let s = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs_f64()).unwrap_or(0.0);
    s * SPEED as f64
}

impl Panel {
    pub fn new(cx: &mut Context, params: Arc<DraconesParams>, telemetry: Arc<Telemetry>) -> Handle<'_, Self> {
        let ticking = Arc::new(AtomicBool::new(true));
        let ticker = ticking.clone();
        let rows = rows_of(params.order.value() as usize);
        let seed = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.subsec_nanos()).unwrap_or(1) | 1;
        let motion = Motion {
            t: 0.0,
            world: World::new(seed),
            hist: Default::default(),
            ticks: 0,
            dock: rows.map(|r| ROWS[r]),
        };
        Self {
            params,
            telemetry,
            statics: world::statics(),
            letters: title_letters(),
            arms: ARMED.iter().map(|k| arm_angles(k.id)).collect(),
            motion: RefCell::new(motion),
            drag: None,
            focus: 0,
            started: Instant::now(),
            ticking,
        }
        .build(cx, move |cx| {
            // vizia's timers never fire under baseview; events sent through a proxy do.
            let mut proxy = cx.get_proxy();
            std::thread::spawn(move || {
                while ticker.load(Ordering::Relaxed) {
                    std::thread::sleep(TICK);
                    let _ = proxy.emit(PanelEvent::Tick);
                }
            });
        })
        .width(Stretch(1.0))
        .height(Stretch(1.0))
    }

    // ---- Where things are ---------------------------------------------------------

    fn knob_at(&self, i: usize, dock: &[f32; EFFECTS]) -> (f32, f32) {
        match i {
            0..=14 => (OSC_X[i % 5], ROWS[i / 5]),
            15..=20 => {
                let e = (i - 15) / 2;
                (FX_X[1 + (i - 15) % 2], dock[e])
            }
            _ => (POST_X[i - 21], POST_Y),
        }
    }

    fn arm_triangle(&self, i: usize, m: usize, cx: f32, cy: f32, depth: f32) -> [(f32, f32); 3] {
        let a = self.arms[i][m];
        let (ux, uy) = (a.cos(), a.sin());
        let (px, py) = (-uy, ux);
        let (rb, hw) = (KNOB_R + 2.5, 5.5);
        let h = 9.5 * (1.0 + 2.0 * depth);
        [
            (cx + ux * rb + px * hw, cy + uy * rb + py * hw),
            (cx + ux * (rb + h), cy + uy * (rb + h)),
            (cx + ux * rb - px * hw, cy + uy * rb - py * hw),
        ]
    }

    fn depth(&self, i: usize, m: usize) -> f32 {
        self.params.depths[i][m].unmodulated_normalized_value()
    }

    fn hit(&self, x: f32, y: f32) -> Option<Target> {
        let dock = self.motion.borrow().dock;
        let near = |(cx, cy): (f32, f32), r: f32| (x - cx).powi(2) + (y - cy).powi(2) <= r * r;
        // Arms first: they reach past their knobs.
        for i in 0..ARMED_COUNT {
            let (cx, cy) = self.knob_at(i, &dock);
            if !near((cx, cy), KNOB_R + 50.0) {
                continue;
            }
            for m in 0..4 {
                let tri = self.arm_triangle(i, m, cx, cy, self.depth(i, m));
                if near_triangle(x, y, &tri, 4.5) {
                    return Some(Target::Arm(i, m));
                }
            }
        }
        for i in 0..ARMED_COUNT {
            if near(self.knob_at(i, &dock), KNOB_R + 1.0) {
                return Some(Target::Knob(i));
            }
        }
        for e in 0..EFFECTS {
            if near((FX_X[0], dock[e]), KNOB_R + 1.0) {
                return Some(Target::Division(e));
            }
            let my = dock[e] + MIX_DY;
            if (FX_X[0] - MIX_HALF - 6.0..FX_X[0] + MIX_HALF + 6.0).contains(&x) && (my - 8.0..my + 8.0).contains(&y) {
                return Some(Target::Mix(e));
            }
            if near((PLAQUE_X, dock[e]), 24.0) {
                return Some(Target::Plaque(e));
            }
        }
        for (copy, &rx) in RED_X.iter().enumerate() {
            for (which, &ry) in RED_Y.iter().enumerate() {
                if near((rx, ry), SMALL_R + 3.0) {
                    return Some(Target::Red(2 * copy + which));
                }
            }
        }
        for o in 0..3 {
            if near((SEAL_X, ROWS[o]), 25.0) {
                return Some(Target::Seal(o));
            }
            if (SWITCH_X - 8.0..SWITCH_X + 28.0).contains(&x) && (ROWS[o] - 22.0..ROWS[o] + 22.0).contains(&y) {
                return Some(Target::Switch(o));
            }
        }
        let (cx, cy) = CARTOUCHE;
        if (cx - 52.0..cx + 52.0).contains(&x) && (cy - 29.0..cy + 29.0).contains(&y) {
            return Some(Target::Cartouche(x >= cx));
        }
        let dx = CARTOUCHE.0;
        if (dx - DEPTH_HALF - 6.0..dx + DEPTH_HALF + 6.0).contains(&x) && (DEPTH_Y - 8.0..DEPTH_Y + 8.0).contains(&y) {
            return Some(Target::Depth);
        }
        for (copy, &lx) in LEVERS_X.iter().enumerate() {
            if (lx - 10.0..lx + 10.0).contains(&x) && (LEVER_Y - 24.0..LEVER_Y + 25.0).contains(&y) {
                let pos = if y < LEVER_Y - 8.0 { 0 } else if y < LEVER_Y + 8.0 { 1 } else { 2 };
                return Some(Target::Lever(copy, pos));
            }
        }
        if (OUT_X - 15.0..OUT_X + 15.0).contains(&x) && (OUT_TOP - 10.0..OUT_BOTTOM + 10.0).contains(&y) {
            return Some(Target::Output);
        }
        None
    }

    // ---- Changing parameters ------------------------------------------------------

    fn begin(cx: &mut EventContext, p: ParamPtr) {
        cx.emit(RawParamEvent::BeginSetParameter(p));
    }

    fn set(cx: &mut EventContext, p: ParamPtr, v: f32) {
        cx.emit(RawParamEvent::SetParameterNormalized(p, v.clamp(0.0, 1.0)));
    }

    fn end(cx: &mut EventContext, p: ParamPtr) {
        cx.emit(RawParamEvent::EndSetParameter(p));
    }

    fn set_once(cx: &mut EventContext, p: ParamPtr, v: f32) {
        Self::begin(cx, p);
        Self::set(cx, p, v);
        Self::end(cx, p);
    }

    /// Set an int parameter to `plain`.
    fn set_plain(cx: &mut EventContext, p: ParamPtr, plain: f32) {
        Self::set_once(cx, p, unsafe { p.preview_normalized(plain) });
    }

    fn plain(p: ParamPtr) -> f32 {
        unsafe { p.unmodulated_plain_value() }
    }

    fn param_of(&self, t: Target) -> Option<ParamPtr> {
        let p = &self.params;
        Some(match t {
            Target::Knob(i) => p.knobs[i].as_ptr(),
            Target::Arm(i, m) => p.depths[i][m].as_ptr(),
            Target::Division(e) => p.divisions[e].as_ptr(),
            Target::Red(k) if k % 2 == 0 => p.red_time[k / 2].as_ptr(),
            Target::Red(k) => p.red_slope[k / 2].as_ptr(),
            Target::Output => p.output.as_ptr(),
            Target::Mix(e) => p.mixes[e].as_ptr(),
            Target::Depth => p.morph_depth.as_ptr(),
            _ => return None,
        })
    }

    /// Division `e` one step `dir` along, past the values the other two hold.
    fn step_division(&self, cx: &mut EventContext, e: usize, dir: i32) {
        let taken: Vec<i32> = (0..EFFECTS).filter(|&o| o != e).map(|o| self.params.divisions[o].value()).collect();
        let mut d = self.params.divisions[e].value() + dir;
        while (0..7).contains(&d) && taken.contains(&d) {
            d += dir;
        }
        if (0..7).contains(&d) {
            let p = self.params.divisions[e].as_ptr();
            Self::set(cx, p, unsafe { p.preview_normalized(d as f32) });
        }
    }

    /// The row a control belongs to, for which copies are drawn bright.
    fn row_of(&self, t: Target) -> Option<usize> {
        let rows = rows_of(self.params.order.value() as usize);
        match t {
            Target::Knob(i) | Target::Arm(i, _) if i < 21 => Some(knobs::row(i, rows)),
            Target::Division(e) | Target::Mix(e) | Target::Plaque(e) => Some(rows[e]),
            Target::Red(k) => Some(k / 2),
            Target::Lever(copy, _) => Some(copy),
            Target::Seal(o) | Target::Switch(o) => Some(o),
            _ => None,
        }
    }

    fn grab(&mut self, cx: &mut EventContext, t: Target, anchor: f32, per_px: f32, fine: bool, horizontal: bool) {
        let p = self.param_of(t).unwrap();
        Self::begin(cx, p);
        let value = unsafe { p.unmodulated_normalized_value() };
        self.drag = Some(Drag::Value { param: p, anchor, value, per_px, fine, horizontal });
    }

    fn mouse_down(&mut self, cx: &mut EventContext, x: f32, y: f32) {
        self.mouse_up(cx);
        let fine = cx.modifiers().contains(Modifiers::SHIFT);
        let Some(target) = self.hit(x, y) else { return };
        if let Some(r) = self.row_of(target) {
            self.focus = r;
        }
        match target {
            Target::Knob(_) | Target::Red(_) => self.grab(cx, target, y, 1.0 / KNOB_TRAVEL, fine, false),
            Target::Output => self.grab(cx, target, y, 1.0 / OUT_TRAVEL, fine, false),
            Target::Mix(_) => self.grab(cx, target, x, 0.5 / MIX_HALF, fine, true),
            Target::Depth => self.grab(cx, target, x, 0.5 / DEPTH_HALF, fine, true),
            Target::Arm(i, m) => {
                Self::begin(cx, self.params.depths[i][m].as_ptr());
                self.drag = Some(Drag::Arm { knob: i, m, last: self.along_arm(i, m, x, y) });
            }
            Target::Division(e) => {
                Self::begin(cx, self.params.divisions[e].as_ptr());
                self.drag = Some(Drag::Division { e, last: y, acc: 0.0 });
            }
            Target::Plaque(e) => {
                let d0 = self.motion.borrow().dock[e];
                self.drag = Some(Drag::Plaque { e, y0: y, d0 });
            }
            Target::Seal(o) if o < 2 => {
                let (p, count) = if o == 0 { (self.params.fizmo_table.as_ptr(), tables::TABLES) } else { (self.params.argon_table.as_ptr(), argon::TABLES) };
                let dir = if fine { count - 1 } else { 1 };
                let next = (Self::plain(p) as usize + dir) % count;
                Self::set_plain(cx, p, next as f32);
            }
            Target::Seal(_) => {}
            Target::Switch(o) => {
                let p = self.params.filters[o].as_ptr();
                let v = unsafe { p.unmodulated_normalized_value() };
                Self::set_once(cx, p, if v > 0.5 { 0.0 } else { 1.0 });
            }
            Target::Cartouche(forward) => {
                let p = self.params.morph_shape.as_ptr();
                let count = harmlib::NAMES.len() + 1;
                let step = if forward { 1 } else { count - 1 };
                Self::set_plain(cx, p, ((Self::plain(p) as usize + step) % count) as f32);
            }
            Target::Lever(copy, pos) => Self::set_plain(cx, self.params.slate_rate[copy].as_ptr(), pos as f32),
        }
        if self.drag.is_some() {
            cx.capture();
        }
    }

    /// How far along arm `m` of knob `i` the point is.
    fn along_arm(&self, i: usize, m: usize, x: f32, y: f32) -> f32 {
        let (cx, cy) = self.knob_at(i, &self.motion.borrow().dock);
        let a = self.arms[i][m];
        (x - cx) * a.cos() + (y - cy) * a.sin()
    }

    fn mouse_move(&mut self, cx: &mut EventContext, x: f32, y: f32) {
        let fine = cx.modifiers().contains(Modifiers::SHIFT);
        let speed = if fine { FINE } else { 1.0 };
        match self.drag.as_mut() {
            Some(Drag::Value { param, anchor, value, per_px, fine: was, horizontal }) => {
                // Up for a knob, right for a horizontal slider.
                let (at, sign) = if *horizontal { (x, -1.0) } else { (y, 1.0) };
                if *was != fine {
                    *value = unsafe { param.unmodulated_normalized_value() };
                    *anchor = at;
                    *was = fine;
                }
                let v = *value + sign * (*anchor - at) * *per_px * speed;
                let p = *param;
                Self::set(cx, p, v);
            }
            Some(Drag::Arm { knob, m, .. }) => {
                let (i, m) = (*knob, *m);
                let along = self.along_arm(i, m, x, y);
                let Some(Drag::Arm { last, .. }) = self.drag.as_mut() else { return };
                let delta = (along - *last) / ARM_TRAVEL * speed;
                *last = along;
                let p = self.params.depths[i][m].as_ptr();
                Self::set(cx, p, unsafe { p.unmodulated_normalized_value() } + delta);
            }
            Some(Drag::Division { e, last, acc }) => {
                *acc += *last - y;
                *last = y;
                let e = *e;
                let mut steps: i32 = 0;
                while let Some(Drag::Division { acc, .. }) = self.drag.as_mut() {
                    if *acc > DIVISION_STEP {
                        *acc -= DIVISION_STEP;
                        steps += 1;
                    } else if *acc < -DIVISION_STEP {
                        *acc += DIVISION_STEP;
                        steps -= 1;
                    } else {
                        break;
                    }
                }
                for _ in 0..steps.abs() {
                    self.step_division(cx, e, steps.signum());
                }
            }
            Some(Drag::Plaque { e, y0, d0 }) => {
                let at = (*d0 + y - *y0).clamp(ROWS[0] - 24.0, ROWS[2] + 24.0);
                self.motion.borrow_mut().dock[*e] = at;
            }
            None => return,
        }
        cx.needs_redraw();
    }

    fn mouse_up(&mut self, cx: &mut EventContext) {
        let Some(drag) = self.drag.take() else { return };
        match drag {
            Drag::Value { param, .. } => Self::end(cx, param),
            Drag::Arm { knob, m, .. } => Self::end(cx, self.params.depths[knob][m].as_ptr()),
            Drag::Division { e, .. } => Self::end(cx, self.params.divisions[e].as_ptr()),
            Drag::Plaque { e, .. } => {
                // Dropped on the nearest row; whoever was there takes this one's old row.
                let y = self.motion.borrow().dock[e];
                let best = (0..3).min_by(|&a, &b| (ROWS[a] - y).abs().total_cmp(&(ROWS[b] - y).abs())).unwrap_or(0);
                let mut rows = rows_of(self.params.order.value() as usize);
                let from = rows[e];
                if let Some(other) = (0..EFFECTS).find(|&o| rows[o] == best) {
                    rows[other] = from;
                }
                rows[e] = best;
                Self::set_plain(cx, self.params.order.as_ptr(), order_of(rows) as f32);
            }
        }
        cx.release();
    }

    fn reset(&mut self, cx: &mut EventContext, x: f32, y: f32) {
        let Some(t) = self.hit(x, y) else { return };
        if let Target::Arm(i, m) = t {
            Self::set_once(cx, self.params.depths[i][m].as_ptr(), 0.0);
            return;
        }
        if matches!(t, Target::Knob(_) | Target::Red(_) | Target::Output | Target::Division(_) | Target::Mix(_) | Target::Depth) {
            if let Some(p) = self.param_of(t) {
                Self::set_once(cx, p, unsafe { p.default_normalized_value() });
            }
        } else {
            // Buttons: a quick second click is another press.
            self.mouse_down(cx, x, y);
            self.mouse_up(cx);
        }
    }

    fn scroll(&mut self, cx: &mut EventContext, x: f32, y: f32, notches: f32) {
        let Some(t) = self.hit(x, y) else { return };
        let notch = if cx.modifiers().contains(Modifiers::SHIFT) { 0.002 } else { 0.01 } * notches.signum();
        match t {
            Target::Division(e) => {
                let p = self.params.divisions[e].as_ptr();
                Self::begin(cx, p);
                self.step_division(cx, e, notches.signum() as i32);
                Self::end(cx, p);
            }
            Target::Knob(_) | Target::Red(_) | Target::Output | Target::Arm(..) | Target::Mix(_) | Target::Depth => {
                let p = self.param_of(t).unwrap();
                Self::set_once(cx, p, unsafe { p.unmodulated_normalized_value() } + notch);
            }
            _ => {}
        }
    }

    // ---- Time -------------------------------------------------------------------

    fn tick(&mut self, cx: &mut EventContext) {
        keep_hover_alive(cx);
        let dt = TICK.as_secs_f32();
        let mods = self.telemetry.mods();
        let order_rows = rows_of(self.params.order.value() as usize);
        let dragging = match self.drag {
            Some(Drag::Plaque { e, .. }) => Some(e),
            _ => None,
        };
        let mut m = self.motion.borrow_mut();
        m.t = self.started.elapsed().as_secs_f32();
        let t = m.t;
        let off = drift();
        m.world.sail(dt, t, off, BEASTS.len());
        let flat = mods.flat();
        let green_now = m.ticks % GREEN_EVERY == 0;
        for (k, h) in m.hist.iter_mut().enumerate() {
            // Green (4) keeps one point in GREEN_EVERY: its trace is about the last minute.
            if k == 4 && !green_now {
                continue;
            }
            h.push_back(flat[k]);
            if h.len() > HIST {
                h.pop_front();
            }
        }
        m.ticks = m.ticks.wrapping_add(1);
        for e in 0..EFFECTS {
            if dragging != Some(e) {
                let target = ROWS[order_rows[e]];
                m.dock[e] += (target - m.dock[e]) * 0.3;
            }
        }
        cx.needs_redraw();
    }

    // ---- Drawing ----------------------------------------------------------------

    fn transform(bounds: BoundingBox) -> (f32, f32, f32) {
        let scale = (bounds.w / WIDTH).min(bounds.h / HEIGHT);
        (scale, bounds.x + (bounds.w - WIDTH * scale) / 2.0, bounds.y + (bounds.h - HEIGHT * scale) / 2.0)
    }

    fn to_panel(cx: &EventContext, x: f32, y: f32) -> (f32, f32) {
        let (scale, ox, oy) = Self::transform(cx.bounds());
        ((x - ox) / scale, (y - oy) / scale)
    }

    fn draw_map(&self, c: &mut Canvas, kit: Kit, m: &mut Motion) {
        let off = drift();
        let mut all = Path::new();
        all.rect(0.0, 0.0, WIDTH, HEIGHT);
        // The plate itself, under the map's ink.
        c.fill_path(&all, &fill(GROUND, 1.0));
        c.save();
        c.set_global_alpha(MAP_INK);
        c.fill_path(&all, &fill(GROUND, 1.0));
        let sea_x = -(off % 26.0) as f32;
        c.fill_path(&all, &Paint::image(kit.waves, sea_x, 0.0, 26.0, 8.0, 0.0, 1.0));
        // The faint spiral, turning once per quarter hour.
        c.save();
        c.translate(620.0, 410.0);
        c.rotate(m.t / SPIRAL_S * TAU);
        c.translate(-620.0, -410.0);
        c.stroke_path(&self.statics.spiral, &pen(0x8a6a44, 0.12, 0.6));
        c.restore();
        c.stroke_path(&self.statics.rhumbs, &pen(INK, 0.09, 0.5));
        for i in m.world.visible(off) {
            let Some(chunk) = m.world.chunk(i) else { continue };
            let x = (i as f64 * world::CW as f64 - off) as f32;
            c.save();
            c.translate(x, 0.0);
            for land in &chunk.lands {
                for (ring, (w, a)) in land.rings.iter().zip([(0.7, 0.34), (0.6, 0.22), (0.5, 0.13)]).rev() {
                    c.stroke_path(ring, &pen(INK, a, w));
                }
                c.fill_path(&land.coast, &fill(0x1f1811, 1.0));
                c.fill_path(&land.coast, &Paint::image(kit.hatch, -x, 0.0, 5.0, 5.0, 35f32.to_radians(), 1.0));
                c.stroke_path(&land.coast, &pen(INK, 0.6, 1.1));
            }
            c.stroke_path(&chunk.mountains, &pen(INK, 0.5, 0.85));
            c.restore();
        }
        c.fill_path(&self.statics.rose, &fill(0x2a2016, 0.55));
        c.stroke_path(&self.statics.rose, &pen(INK, 0.55, 0.8));
        // Ships, fading in and out at the edges.
        let hull = svg::path(SHIP_HULL, 0.0, 0.0, 1.0);
        let sails = svg::path(SHIP_SAILS, 0.0, 0.0, 1.0);
        for (lane, ship) in m.world.ships.iter().enumerate() {
            let Some(sx) = *ship else { continue };
            let op = 0.6 * ((sx - 40.0) / 160.0).clamp(0.0, 1.0) * ((1200.0 - sx) / 160.0).clamp(0.0, 1.0);
            let (y, dir) = LANES[lane];
            c.save();
            c.translate(sx, y);
            c.scale(dir, 1.0);
            c.fill_path(&sails, &fill(SHIP, 0.14 * op));
            c.stroke_path(&sails, &pen(SHIP, op, 0.9));
            c.stroke_path(&hull, &pen(SHIP, op, 1.0));
            c.restore();
        }
        for b in &m.world.beasts {
            let age = m.t - b.born;
            let env = if age < 1.2 { age / 1.2 } else if age < 4.7 { 1.0 } else { (1.0 - (age - 4.7) / (BEAST_LIFE - 4.7 - 0.0)).clamp(0.0, 1.0) };
            c.save();
            c.translate((b.wx - off) as f32, b.y + 6.0 * (1.0 - env));
            c.scale(b.flip, 1.0);
            c.stroke_path(&svg::path(BEASTS[b.kind], 0.0, 0.0, 1.0), &pen(SHIP, 0.5 * env, 1.2));
            c.restore();
        }
        // The vignette, elliptical over the plate.
        c.save();
        c.translate(620.0, 400.0);
        c.scale(1.0, HEIGHT / WIDTH);
        let r = 0.72 * WIDTH;
        let vig = Paint::radial_gradient_stops(0.0, 0.0, 0.0, r, [(0.55, faded(0, 0.0)), (1.0, faded(0, 0.6))]);
        let mut big = Path::new();
        big.rect(-WIDTH, -WIDTH, 2.0 * WIDTH, 2.0 * WIDTH);
        c.fill_path(&big, &vig);
        c.restore();
        c.restore();
    }

    fn draw_frame(&self, c: &mut Canvas) {
        let mut b = Path::new();
        b.rect(10.0, 10.0, 1220.0, 780.0);
        b.rect(18.0, 18.0, 1204.0, 764.0);
        c.stroke_path(&b, &pen(INK, 0.75, 1.2));
        c.fill_path(&self.statics.border, &fill(INK, 0.5));
        for (x0, y0, x1, y1, col) in SECTIONS {
            let outer = chamfer(x0, y0, x1, y1, 9.0);
            c.fill_path(&outer, &fill(col, 0.075));
            c.stroke_path(&outer, &pen(col, 0.34, 1.0));
            c.stroke_path(&chamfer(x0 + 4.0, y0 + 4.0, x1 - 4.0, y1 - 4.0, 7.0), &pen(col, 0.16, 0.7));
        }
        for (d, x, y, s) in GLYPHS {
            c.stroke_path(&svg::path(d, x, y, s), &pen(0xd2b07a, 0.78, 1.0 * s));
        }
    }

    fn draw_title(&self, c: &mut Canvas, kit: Kit) {
        let tails = svg::path("M420 52 L378 50 L394 67 L378 86 L428 84 Z M820 52 L862 50 L846 67 L862 86 L812 84 Z", 0.0, 0.0, 1.0);
        c.fill_path(&tails, &fill(0x1a130d, 1.0));
        c.stroke_path(&tails, &pen(0xb8925a, 1.0, 1.0));
        let ribbon = svg::path("M420 46 Q620 32 820 46 L820 86 Q620 72 420 86 Z", 0.0, 0.0, 1.0);
        c.fill_path(&ribbon, &fill(0x231a11, 1.0));
        c.stroke_path(&ribbon, &pen(GROUND, 1.0, 1.2));
        for l in &self.letters {
            c.save();
            c.translate(l.x, l.y);
            c.rotate(l.rot.to_radians());
            c.scale(l.k, l.k);
            let mut p = Paint::color(color(TITLE));
            p.set_font(&[kit.fell]);
            p.set_font_size(28.0);
            p.set_text_align(vg::Align::Center);
            p.set_text_baseline(vg::Baseline::Alphabetic);
            let _ = c.fill_text(0.0, 0.0, l.c.to_string(), &p);
            c.restore();
        }
    }

    fn draw_corners(&self, c: &mut Canvas, m: &Motion) {
        let mut ticks = Path::new();
        for i in 0..32 {
            let a = i as f32 / 32.0 * TAU;
            let r0 = if i % 8 == 0 { 41.0 } else { 43.5 };
            ticks.move_to(a.cos() * r0, a.sin() * r0);
            ticks.line_to(a.cos() * 45.0, a.sin() * 45.0);
        }
        for (k, &(x, y)) in CORNERS.iter().enumerate() {
            halo(c, x, y, 64.0);
            c.save();
            c.translate(x, y);
            c.fill_path(&circle(0.0, 0.0, 50.0), &fill(0x1a140f, 1.0));
            c.stroke_path(&circle(0.0, 0.0, 50.0), &pen(GOLD, 1.0, 1.7));
            c.stroke_path(&circle(0.0, 0.0, 45.0), &pen(GOLD, 0.5, 0.6));
            c.stroke_path(&ticks, &pen(GOLD, 0.65, 0.8));
            let mut axis = Path::new();
            axis.move_to(-40.0, 0.0);
            axis.line_to(40.0, 0.0);
            axis.move_to(0.0, -40.0);
            axis.line_to(0.0, 40.0);
            c.stroke_path(&axis, &pen(GOLD, 0.28, 0.5));
            let col = MODS[k];
            // Red and slate have three copies: the focused row's last, so it's on top.
            let traces: Vec<(usize, bool)> = match k {
                0 => vec![(0, true)],
                2 => vec![(4, true)],
                _ => {
                    let first = if k == 1 { 1 } else { 5 };
                    let mut v: Vec<(usize, bool)> = (0..3).filter(|&r| r != self.focus).map(|r| (first + r, false)).collect();
                    v.push((first + self.focus, true));
                    v
                }
            };
            for (idx, bright) in traces {
                let h = &m.hist[idx];
                let mut trace = Path::new();
                let mut head = 0.0;
                for (j, &v) in h.iter().enumerate() {
                    let v = if k == 1 { 2.0 * v - 1.0 } else { v };
                    let px = 38.0 - 76.0 * (h.len() - 1 - j) as f32 / (HIST - 1) as f32;
                    let py = -v.clamp(-1.0, 1.0) * 28.0;
                    if j == 0 { trace.move_to(px, py) } else { trace.line_to(px, py) }
                    head = py;
                }
                let (alpha, width) = if bright { (1.0, 1.7) } else { (0.28, 1.2) };
                c.stroke_path(&trace, &pen(col, alpha, width));
                c.fill_path(&circle(38.0, head, if bright { 2.6 } else { 1.8 }), &fill(col, alpha));
            }
            c.fill_path(&svg::path("M0 -56 l4.5 6 l-4.5 6 l-4.5 -6 Z", 0.0, 0.0, 1.0), &fill(col, 1.0));
            c.restore();
        }
    }

    /// A numeral in the title's face.
    fn numeral(c: &mut Canvas, kit: Kit, x: f32, y: f32, size: f32, text: &str, alpha: f32) {
        let mut p = Paint::color(faded(TITLE, alpha));
        p.set_font(&[kit.fell]);
        p.set_font_size(size);
        p.set_text_align(vg::Align::Center);
        p.set_text_baseline(vg::Baseline::Alphabetic);
        let _ = c.fill_text(x, y, text, &p);
    }

    fn draw_seals(&self, c: &mut Canvas, kit: Kit) {
        let tables = [self.params.fizmo_table.value() as usize, self.params.argon_table.value() as usize];
        let counts = [tables::TABLES, argon::TABLES];
        let running = self.telemetry.running();
        for o in 0..3 {
            let (x, y) = (SEAL_X, ROWS[o]);
            halo(c, x, y, 40.0);
            c.fill_path(&circle(x, y, 24.0), &fill(0x1f1812, 1.0));
            c.stroke_path(&circle(x, y, 24.0), &pen(GOLD, 1.0, 1.6));
            c.stroke_path(&circle(x, y, 20.0), &pen(GOLD, 0.5, 0.6));
            // The scope: the oscillator's last cycle after its filter, scaled to fit,
            // brighter the more of it is heard.
            let mut axis = Path::new();
            axis.move_to(x - 17.0, y);
            axis.line_to(x + 17.0, y);
            c.stroke_path(&axis, &pen(GOLD, 0.3, 0.5));
            if running {
                let cycle = self.telemetry.scope(o);
                let peak = cycle.iter().fold(0.0f32, |a, v| a.max(v.abs()));
                if peak > 1e-5 {
                    let mut trace = Path::new();
                    for (i, v) in cycle.iter().enumerate() {
                        let px = x - SCOPE_W + 2.0 * SCOPE_W * i as f32 / (cycle.len() - 1) as f32;
                        let py = y - v / peak * SCOPE_H;
                        if i == 0 { trace.move_to(px, py) } else { trace.line_to(px, py) }
                    }
                    let heard = osc_gain(self.telemetry.moved(knobs::osc(o, knobs::GAIN))).sqrt();
                    c.stroke_path(&trace, &pen(CREAM, 0.3 + 0.7 * heard.min(1.0), 1.3));
                }
            }
            Self::numeral(c, kit, x, y + 46.0, 13.0, NUMERALS[o], 0.85);
            if o < 2 {
                let n = counts[o];
                for i in 0..n {
                    let a = if o == 0 { i as f32 / n as f32 * TAU - FRAC_PI_2 } else { -FRAC_PI_2 + (i as f32 - (n as f32 - 1.0) / 2.0) * 0.32 };
                    let r = if o == 0 { 1.1 } else { 2.0 };
                    let lit = i == tables[o];
                    c.fill_path(&circle(x + a.cos() * 30.0, y + a.sin() * 30.0, r), &fill(if lit { PALE } else { 0x6d5739 }, 1.0));
                }
            }
        }
    }

    fn draw_switches(&self, c: &mut Canvas) {
        let wasp = svg::path(WASP, 0.0, 0.0, 1.0);
        let lowpass = svg::path(LOWPASS, 0.0, 0.0, 1.0);
        for o in 0..3 {
            let svf = self.params.filters[o].unmodulated_normalized_value() > 0.5;
            c.save();
            c.translate(SWITCH_X, ROWS[o]);
            slot(c, -10.0, 10.0);
            let ball = if svf { 9.0 } else { -9.0 };
            c.fill_path(&circle(0.0, ball, 4.6), &fill(BALL, 1.0));
            c.stroke_path(&circle(0.0, ball, 4.6), &pen(0x4d3c29, 1.0, 1.0));
            let (lit, dim) = (pen(PALE, 1.0, 1.0), pen(GOLD, 0.38, 0.9));
            c.stroke_path(&wasp, if svf { &dim } else { &lit });
            c.stroke_path(&lowpass, if svf { &lit } else { &dim });
            c.restore();
        }
    }

    /// I, II, III over red's three columns of small knobs.
    fn draw_red_numerals(&self, c: &mut Canvas, kit: Kit) {
        for (copy, &x) in RED_X.iter().enumerate() {
            let bright = if copy == self.focus { 1.0 } else { 0.6 };
            Self::numeral(c, kit, x, 38.0, 13.0, NUMERALS[copy], bright);
        }
    }

    fn draw_ropes(&self, c: &mut Canvas, dock: &[f32; EFFECTS]) {
        for &y in &ROWS {
            let pts: Vec<(f32, f32)> = (0..=24).map(|k| {
                let t = k as f32 / 24.0;
                let u = 1.0 - t;
                let bx = u * u * u * 720.0 + 3.0 * u * u * t * 734.0 + 3.0 * u * t * t * 748.0 + t * t * t * 762.0;
                let by = u * u * u * y + 3.0 * u * u * t * (y - 5.0) + 3.0 * u * t * t * (y + 5.0) + t * t * t * y;
                (bx, by)
            }).collect();
            c.stroke_path(&dashed(&pts, 3.0, 2.0), &pen(ROPE, 0.7, 1.3));
        }
        c.stroke_path(&dashed(&[(300.0, POST_Y), (840.0, POST_Y)], 2.0, 5.0), &pen(ROPE, 0.55, 1.0));
        for e in 0..EFFECTS {
            let y = dock[e];
            halo(c, PLAQUE_X, y, 38.0);
            c.fill_path(&circle(PLAQUE_X, y, 23.0), &fill(0x211912, 1.0));
            c.stroke_path(&circle(PLAQUE_X, y, 23.0), &pen(GOLD, 1.0, 1.7));
            c.stroke_path(&circle(PLAQUE_X, y, 19.0), &pen(GOLD, 0.5, 0.6));
            c.stroke_path(&svg::path(EMBLEMS[e], PLAQUE_X, y, 1.0), &pen(CREAM, 1.0, 1.2));
        }
    }

    fn draw_cartouche(&self, c: &mut Canvas, kit: Kit) {
        let (x, y) = CARTOUCHE;
        halo(c, x, y, 58.0);
        c.save();
        c.translate(x, y);
        let mut outer = Path::new();
        outer.rounded_rect(-52.0, -29.0, 104.0, 58.0, 4.0);
        c.fill_path(&outer, &fill(0x1d1611, 1.0));
        c.stroke_path(&outer, &pen(GOLD, 1.0, 1.5));
        let mut inner = Path::new();
        inner.rounded_rect(-48.0, -25.0, 96.0, 50.0, 2.0);
        c.stroke_path(&inner, &pen(GOLD, 0.5, 0.6));
        if let Some(isle) = isle(self.params.morph_shape.value() as usize) {
            c.fill_path(&isle, &fill(0x2a2017, 1.0));
            c.fill_path(&isle, &Paint::image(kit.hatch, -x, -y, 5.0, 5.0, 35f32.to_radians(), 1.0));
            c.stroke_path(&isle, &pen(TITLE, 1.0, 1.0));
        }
        c.stroke_path(&svg::path("M-40 15 h10 M-22 15 h14 M2 15 h12 M22 15 h16 M-34 20 h12 M-10 20 h10 M10 20 h14", 0.0, 0.0, 1.0), &pen(ROPE, 0.7, 0.8));
        c.fill_path(&svg::path("M-47 0 l4 -4.5 v9 Z M47 0 l-4 -4.5 v9 Z", 0.0, 0.0, 1.0), &fill(GOLD, 0.75));
        c.restore();
    }

    /// The three slate levers (I, II, III), and left of them one legend of pips: 1, 4
    /// and 16 quarter notes, level with the levers' three positions.
    fn draw_levers(&self, c: &mut Canvas, kit: Kit) {
        for (copy, &x) in LEVERS_X.iter().enumerate() {
            let k = self.params.slate_rate[copy].value() as usize;
            c.save();
            c.translate(x, LEVER_Y);
            slot(c, -17.0, 17.0);
            let by = [-16.0, 0.0, 16.0][k.min(2)];
            c.fill_path(&circle(0.0, by, 4.8), &fill(BALL, 1.0));
            c.stroke_path(&circle(0.0, by, 4.8), &pen(0x4d3c29, 1.0, 1.0));
            c.restore();
            let bright = if copy == self.focus { 1.0 } else { 0.6 };
            Self::numeral(c, kit, x, LEVER_Y - 25.0, 13.0, NUMERALS[copy], bright);
        }
        c.save();
        c.translate(PIP_LEGEND_X, LEVER_Y);
        let pips: [&[(f32, f32)]; 3] = [
            &[(14.0, -16.0)],
            &[(13.0, -2.0), (17.0, -2.0), (13.0, 2.0), (17.0, 2.0)],
            &[(12.0, 11.0), (15.0, 11.0), (18.0, 11.0), (21.0, 11.0), (12.0, 14.0), (15.0, 14.0), (18.0, 14.0), (21.0, 14.0),
              (12.0, 17.0), (15.0, 17.0), (18.0, 17.0), (21.0, 17.0), (12.0, 20.0), (15.0, 20.0), (18.0, 20.0), (21.0, 20.0)],
        ];
        for group in pips.iter() {
            for &(px, py) in group.iter() {
                c.fill_path(&circle(px, py, 1.0), &fill(GOLD, 0.85));
            }
        }
        c.restore();
    }

    /// A horizontal slider: a groove from x0 to x1 with a mark at its middle, and a brass
    /// thumb at `v` of the way along.
    fn hslider(c: &mut Canvas, x0: f32, x1: f32, y: f32, v: f32) {
        let mut g = Path::new();
        g.move_to(x0, y);
        g.line_to(x1, y);
        c.stroke_path(&g, &pen(0x0a0705, 1.0, 5.0));
        let mut e = Path::new();
        e.move_to(x0, y - 3.0);
        e.line_to(x1, y - 3.0);
        e.move_to(x0, y + 3.0);
        e.line_to(x1, y + 3.0);
        let mid = 0.5 * (x0 + x1);
        e.move_to(mid, y + 4.5);
        e.line_to(mid, y + 7.5);
        c.stroke_path(&e, &pen(GOLD, 1.0, 0.8));
        let tx = x0 + v.clamp(0.0, 1.0) * (x1 - x0);
        let mut thumb = Path::new();
        thumb.rounded_rect(tx - 3.5, y - 6.0, 7.0, 12.0, 1.5);
        c.fill_path(&thumb, &fill(BALL, 1.0));
        c.stroke_path(&thumb, &pen(0x4d3c29, 1.0, 1.0));
        let mut line = Path::new();
        line.move_to(tx, y - 3.5);
        line.line_to(tx, y + 3.5);
        c.stroke_path(&line, &pen(0x4d3c29, 1.0, 1.1));
    }

    fn draw_sliders(&self, c: &mut Canvas, dock: &[f32; EFFECTS]) {
        for e in 0..EFFECTS {
            let x = FX_X[0];
            Self::hslider(c, x - MIX_HALF, x + MIX_HALF, dock[e] + MIX_DY, self.params.mixes[e].unmodulated_normalized_value());
        }
        let x = CARTOUCHE.0;
        Self::hslider(c, x - DEPTH_HALF, x + DEPTH_HALF, DEPTH_Y, self.params.morph_depth.unmodulated_normalized_value());
    }

    fn draw_output(&self, c: &mut Canvas) {
        c.save();
        c.translate(OUT_X, 0.0);
        slot(c, OUT_TOP, OUT_BOTTOM);
        let mut ticks = Path::new();
        for k in 0..=10 {
            let y = OUT_BOTTOM - 26.0 * k as f32;
            ticks.move_to(-10.0, y);
            ticks.line_to(-6.0, y);
        }
        // Unity, at 80 % of the travel.
        let unity = OUT_BOTTOM - 0.8 * (OUT_BOTTOM - OUT_TOP);
        ticks.move_to(-13.0, unity);
        ticks.line_to(-6.0, unity);
        ticks.move_to(6.0, unity);
        ticks.line_to(10.0, unity);
        c.stroke_path(&ticks, &pen(GOLD, 0.8, 1.0));
        let v = self.params.output.unmodulated_normalized_value();
        let ty = OUT_BOTTOM - v * (OUT_BOTTOM - OUT_TOP);
        let mut thumb = Path::new();
        thumb.rounded_rect(-11.0, ty - 4.5, 22.0, 9.0, 1.5);
        c.fill_path(&thumb, &fill(BALL, 1.0));
        c.stroke_path(&thumb, &pen(0x4d3c29, 1.0, 1.0));
        let mut line = Path::new();
        line.move_to(-7.0, ty);
        line.line_to(7.0, ty);
        c.stroke_path(&line, &pen(0x4d3c29, 1.0, 1.2));
        c.restore();
    }

    /// A knob face: halo, body, ticks, pointer.
    fn knob_face(c: &mut Canvas, x: f32, y: f32, r: f32, value: f32, ticks: &Path) {
        halo(c, x, y, r + 18.0);
        c.fill_path(&circle(x, y, r), &fill(FACE, 1.0));
        c.stroke_path(&circle(x, y, r), &pen(GOLD, 1.0, 1.6));
        c.stroke_path(&circle(x, y, r - 6.0), &pen(GOLD, 0.45, 0.6));
        c.save();
        c.translate(x, y);
        c.stroke_path(ticks, &pen(GOLD, 0.8, 1.0));
        let a = knob_angle(value);
        let (s, co) = (a.sin(), -a.cos());
        let tip = r - 4.0;
        let mut ptr = Path::new();
        ptr.move_to(s * tip, co * tip);
        ptr.line_to(s * 2.0 - co * 2.3, co * 2.0 + s * 2.3);
        ptr.line_to(-s * 4.5, -co * 4.5);
        ptr.line_to(s * 2.0 + co * 2.3, co * 2.0 - s * 2.3);
        ptr.close();
        c.fill_path(&ptr, &fill(CREAM, 1.0));
        c.restore();
    }

    fn ticks_for(r: f32, n: usize, pitch_zones: bool) -> Path {
        let mut d = Path::new();
        for i in 0..n {
            let a = knob_angle(i as f32 / (n - 1) as f32);
            let r0 = if pitch_zones && i == n / 2 { r - 8.0 } else { r - 5.0 };
            let (x0, y0) = polar(a, r0);
            let (x1, y1) = polar(a, r - 2.5);
            d.move_to(x0, y0);
            d.line_to(x1, y1);
        }
        if pitch_zones {
            // Where the Pitch law changes: cents → semitones → octaves, each way.
            for z in [pitch::CENTS_END, pitch::SEMIS_END, pitch::OCTAVE_END] {
                for side in [-1.0, 1.0] {
                    let a = knob_angle(0.5 + 0.5 * side * z);
                    let (x0, y0) = polar(a, r - 7.0);
                    let (x1, y1) = polar(a, r - 2.5);
                    d.move_to(x0, y0);
                    d.line_to(x1, y1);
                }
            }
        }
        d
    }

    fn draw_knobs(&self, c: &mut Canvas, dock: &[f32; EFFECTS]) {
        let plain_ticks = Self::ticks_for(KNOB_R, 11, false);
        let pitch_ticks = Self::ticks_for(KNOB_R, 11, true);
        let running = self.telemetry.running();
        for i in 0..ARMED_COUNT {
            let (x, y) = self.knob_at(i, dock);
            let value = self.params.knobs[i].unmodulated_normalized_value();
            let mut any = false;
            for m in 0..4 {
                let depth = self.depth(i, m);
                any |= depth > 0.0;
                let tri = self.arm_triangle(i, m, x, y, depth);
                let mut p = Path::new();
                p.move_to(tri[0].0, tri[0].1);
                p.line_to(tri[1].0, tri[1].1);
                p.line_to(tri[2].0, tri[2].1);
                p.close();
                c.fill_path(&p, &fill(MODS[m], if depth > 0.0 { 0.35 + 0.55 * depth } else { 0.12 }));
                c.stroke_path(&p, &pen(MODS[m], 1.0, 1.0));
            }
            Self::knob_face(c, x, y, KNOB_R, value, if knobs::is_pitch(i) { &pitch_ticks } else { &plain_ticks });
            if any && running {
                let (bx, by) = polar(knob_angle(self.telemetry.moved(i)), KNOB_R - 0.5);
                c.fill_path(&circle(x + bx, y + by, 2.3), &fill(0xfff4d8, 1.0));
                c.stroke_path(&circle(x + bx, y + by, 2.3), &pen(GROUND, 1.0, 0.8));
            }
        }
        // The Divisions: seven notches, this one's lit, the others' as rocks.
        for e in 0..EFFECTS {
            let (x, y) = (FX_X[0], dock[e]);
            let own = if running { self.telemetry.division(e) } else { self.params.divisions[e].value() as usize };
            let others: Vec<usize> = (0..EFFECTS).filter(|&o| o != e).map(|o| {
                if running { self.telemetry.division(o) } else { self.params.divisions[o].value() as usize }
            }).collect();
            for i in 0..7 {
                let (nx, ny) = polar(knob_angle(i as f32 / 6.0), KNOB_R + 7.0);
                let (fill_c, stroke_c) = if i == own { (PALE, PALE) } else if others.contains(&i) { (0x4d3c29, 0x7a6040) } else { (GROUND, GOLD) };
                c.fill_path(&circle(x + nx, y + ny, 2.4), &fill(fill_c, 1.0));
                c.stroke_path(&circle(x + nx, y + ny, 2.4), &pen(stroke_c, 1.0, 0.8));
            }
            Self::knob_face(c, x, y, KNOB_R, own as f32 / 6.0, &plain_ticks);
        }
        let small = Self::ticks_for(SMALL_R, 7, false);
        for (copy, &x) in RED_X.iter().enumerate() {
            Self::knob_face(c, x, RED_Y[0], SMALL_R, self.params.red_time[copy].unmodulated_normalized_value(), &small);
            Self::knob_face(c, x, RED_Y[1], SMALL_R, self.params.red_slope[copy].unmodulated_normalized_value(), &small);
        }
    }
}

fn slot(c: &mut Canvas, top: f32, bottom: f32) {
    let mut s = Path::new();
    s.move_to(0.0, top);
    s.line_to(0.0, bottom);
    c.stroke_path(&s, &pen(0x0a0705, 1.0, 6.0));
    let mut e = Path::new();
    e.move_to(-3.5, top);
    e.line_to(-3.5, bottom);
    e.move_to(3.5, top);
    e.line_to(3.5, bottom);
    c.stroke_path(&e, &pen(GOLD, 1.0, 0.8));
}

/// Whether (x, y) is inside the triangle or within `slack` of its edges.
fn near_triangle(x: f32, y: f32, t: &[(f32, f32); 3], slack: f32) -> bool {
    let sign = |a: (f32, f32), b: (f32, f32)| (x - b.0) * (a.1 - b.1) - (a.0 - b.0) * (y - b.1);
    let (d1, d2, d3) = (sign(t[0], t[1]), sign(t[1], t[2]), sign(t[2], t[0]));
    let neg = d1 < 0.0 || d2 < 0.0 || d3 < 0.0;
    let pos = d1 > 0.0 || d2 > 0.0 || d3 > 0.0;
    if !(neg && pos) {
        return true;
    }
    let seg = |a: (f32, f32), b: (f32, f32)| {
        let (dx, dy) = (b.0 - a.0, b.1 - a.1);
        let l = (dx * dx + dy * dy).max(1e-6);
        let u = (((x - a.0) * dx + (y - a.1) * dy) / l).clamp(0.0, 1.0);
        ((x - a.0 - u * dx).powi(2) + (y - a.1 - u * dy).powi(2)).sqrt()
    };
    seg(t[0], t[1]).min(seg(t[1], t[2])).min(seg(t[2], t[0])) <= slack
}

/// Works around a vizia bug (as Apse's panel does): when the pointer leaves past the
/// window's right or bottom edge, vizia stops believing it's inside until the platform
/// reports it re-entering, which baseview on Windows never does.
fn keep_hover_alive(cx: &mut EventContext) {
    let bounds = cx.bounds();
    let (x, y) = (cx.mouse().cursorx, cx.mouse().cursory);
    if x >= bounds.x && y >= bounds.y && x < bounds.x + bounds.w && y < bounds.y + bounds.h {
        cx.emit_custom(Event::new(WindowEvent::MouseEnter).origin(Entity::root()).direct(Entity::root()));
    }
}

impl View for Panel {
    fn element(&self) -> Option<&'static str> {
        Some("dracones-panel")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|e: &PanelEvent, _| match e {
            PanelEvent::Tick => self.tick(cx),
        });
        event.map(|e: &RawParamEvent, _| {
            if let RawParamEvent::ParametersChanged = e {
                cx.needs_redraw();
            }
        });
        event.map(|e: &WindowEvent, meta| {
            let (x, y) = Self::to_panel(cx, cx.mouse().cursorx, cx.mouse().cursory);
            match *e {
                WindowEvent::MouseMove(px, py) => {
                    let (x, y) = Self::to_panel(cx, px, py);
                    self.mouse_move(cx, x, y);
                    return;
                }
                WindowEvent::MouseDown(MouseButton::Left) => {
                    self.mouse_down(cx, x, y);
                    meta.consume();
                }
                WindowEvent::MouseDoubleClick(MouseButton::Left) => {
                    self.reset(cx, x, y);
                    meta.consume();
                }
                WindowEvent::MouseUp(MouseButton::Left) => self.mouse_up(cx),
                WindowEvent::MouseScroll(_, notches) => {
                    self.scroll(cx, x, y, notches);
                    meta.consume();
                }
                _ => return,
            }
            cx.needs_redraw();
        });
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        let (scale, ox, oy) = Self::transform(cx.bounds());
        let Some(kit) = kit(canvas) else { return };
        canvas.save();
        canvas.translate(ox, oy);
        canvas.scale(scale, scale);
        let mut m = self.motion.borrow_mut();
        self.draw_map(canvas, kit, &mut m);
        self.draw_frame(canvas);
        self.draw_title(canvas, kit);
        self.draw_corners(canvas, &m);
        self.draw_seals(canvas, kit);
        let dock = m.dock;
        drop(m);
        self.draw_ropes(canvas, &dock);
        self.draw_switches(canvas);
        self.draw_cartouche(canvas, kit);
        self.draw_levers(canvas, kit);
        self.draw_output(canvas);
        self.draw_sliders(canvas, &dock);
        self.draw_knobs(canvas, &dock);
        self.draw_red_numerals(canvas, kit);
        canvas.restore();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn title_fits_the_ribbon() {
        let l = title_letters();
        assert_eq!(l.len(), TITLE_TEXT.chars().filter(|c| *c != ' ').count());
        assert!(l.first().unwrap().x > 420.0 && l.last().unwrap().x < 820.0);
    }

    #[test]
    fn every_symbol_parses() {
        for (d, x, y, s) in GLYPHS {
            let _ = svg::path(d, x, y, s);
        }
        for d in EMBLEMS.iter().chain(BEASTS.iter()) {
            let _ = svg::path(d, 0.0, 0.0, 1.0);
        }
        assert!(isle(0).is_none() && isle(5).is_some());
    }

    #[test]
    fn triangles() {
        let t = [(0.0, 0.0), (10.0, 0.0), (0.0, 10.0)];
        assert!(near_triangle(2.0, 2.0, &t, 0.0));
        assert!(near_triangle(-3.0, 2.0, &t, 4.5));
        assert!(!near_triangle(-6.0, 2.0, &t, 4.5));
    }
}
