//! The panel: controls as stars on dial rings, joined into four constellations round a
//! starless void, where the frame being played is traced round the rim. Dials drag up
//! and down (Shift is fine), double-click resets, scroll nudges. SEED rerolls on a
//! click and steps on a drag. Dragging up or down inside the void sets the Dread.

use std::cell::Cell;
use std::f32::consts::{FRAC_PI_2, PI, TAU};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use nih_plug::prelude::{Param, ParamPtr};
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::vizia::vg::{self, Align, Baseline, Color, Paint, Path};
use nih_plug_vizia::widgets::RawParamEvent;

use super::sky::{self, Book, Star, Void};
use crate::params::{ArgentParams, SEEDS};
use crate::telemetry::{Snapshot, Telemetry, TRACE};

pub const WIDTH: f32 = 980.0;
pub const HEIGHT: f32 = 620.0;

const GROUND: u32 = 0x07080c;
const HOLLOW: u32 = 0x040508;
const INK: u32 = 0xc9cdd6;
const BRIGHT: u32 = 0xe4e7ee;
const WHITE: u32 = 0xf4f5f8;
const DIM: u32 = 0x8b91a0;
const SOFT: u32 = 0xa9afbd;
const LINE: u32 = 0x525868;
const TRACK: u32 = 0x2c313c;
const ORBIT: u32 = 0x8a90a0;
const DASH: u32 = 0x6b7180;
const INNER: u32 = 0x3b404c;
const STAR: u32 = 0xdfe3ec;
const PILL_OFF: u32 = 0x3a3f4b;
const BUTTON: u32 = 0x0b0d12;
const BLOOD: (u8, u8, u8) = (96, 20, 30);
const PUPIL_RED: u32 = 0x7a1c26;

const VOID: Void = Void { x: 300.0, y: 300.0, r: 110.0 };
const DASHED_RING: f32 = 117.0;
const INNER_RING: f32 = 36.0;
const TRACE_RADIUS: f32 = 64.0;
const TRACE_SWING: f32 = 34.0;
const GLOW_RADIUS: f32 = 174.0;

/// A dial sweeps 270 degrees, from bottom left to bottom right.
const SWEEP: f32 = 1.5 * PI;
const DIAL_START: f32 = -FRAC_PI_2 - 0.75 * PI;
/// How far beyond its ring a dial can be grabbed.
const GRAB: f32 = 12.0;
const DRAG_PIXELS: f32 = 220.0;
const SEED_PIXELS_PER_STEP: f32 = 5.0;
/// Under this much movement, a press on SEED is a click.
const CLICK_PIXELS: f32 = 3.0;
const FINE: f32 = 0.12;
const SCROLL_STEP: f32 = 0.01;
const TICK: Duration = Duration::from_millis(33);

static FELL_SC: &[u8] = include_bytes!("../../assets/fonts/FellSC.ttf");
static FELL_ITALIC: &[u8] = include_bytes!("../../assets/fonts/FellItalic.ttf");
static MONO: &[u8] = include_bytes!("../../assets/fonts/Mono.ttf");

fn color(hex: u32, alpha: f32) -> Color {
    let mut c = Color::rgb((hex >> 16) as u8, (hex >> 8) as u8, hex as u8);
    c.set_alphaf(alpha.clamp(0.0, 1.0));
    c
}

#[derive(Clone, Copy)]
struct Rect {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

impl Rect {
    fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && y >= self.y && x < self.x + self.w && y < self.y + self.h
    }
}

#[derive(Clone, Copy, PartialEq)]
enum DialKind {
    Value,
    Seed,
}

struct Dial {
    label: &'static str,
    param: ParamPtr,
    x: f32,
    y: f32,
    r: f32,
    kind: DialKind,
}

/// The four constellations: which dials are joined. Names index `dial_layout()`.
const JOINS: [(&str, &str); 13] = [
    ("morph", "scan"),
    ("scan", "seed"),
    ("seed", "span"),
    ("span", "morph"),
    ("tone", "fizz"),
    ("tide", "depth"),
    ("tide", "flare"),
    ("flare", "bloom"),
    ("depth", "bloom"),
    ("bloom", "tremor"),
    ("tremor", "wander"),
    ("fade", "curve"),
    ("curve", "release"),
];

const NAMES: [(&str, f32, f32); 4] = [("Materia", 600.0, 200.0), ("Lumen", 856.0, 336.0), ("Motus", 488.0, 496.0), ("Adventus", 40.0, 400.0)];

const RELEASE: (f32, f32, f32) = (240.0, 536.0, 22.0);

/// The switches: frames 64 | 32, read step | glide.
const PILL_H: f32 = 26.0;
const FRAMES_64: Rect = Rect { x: 98.0, y: 28.0, w: 44.0, h: PILL_H };
const FRAMES_32: Rect = Rect { x: 150.0, y: 28.0, w: 44.0, h: PILL_H };
const READ_STEP: Rect = Rect { x: 98.0, y: 64.0, w: 58.0, h: PILL_H };
const READ_GLIDE: Rect = Rect { x: 164.0, y: 64.0, w: 58.0, h: PILL_H };

fn dial_layout(params: &ArgentParams) -> Vec<Dial> {
    let value = |label, param: ParamPtr, x, y, r| Dial { label, param, x, y, r, kind: DialKind::Value };
    vec![
        value("morph", params.morph.as_ptr(), 560.0, 110.0, 30.0),
        value("scan", params.scan.as_ptr(), 668.0, 70.0, 34.0),
        Dial { label: "seed", param: params.seed.as_ptr(), x: 782.0, y: 118.0, r: 22.0, kind: DialKind::Seed },
        value("span", params.span.as_ptr(), 722.0, 212.0, 26.0),
        value("volume", params.volume.as_ptr(), 900.0, 70.0, 16.0),
        value("tone", params.tone.as_ptr(), 880.0, 250.0, 28.0),
        value("fizz", params.fizz.as_ptr(), 796.0, 318.0, 20.0),
        value("tide", params.tide.as_ptr(), 520.0, 410.0, 30.0),
        value("depth", params.depth.as_ptr(), 690.0, 392.0, 18.0),
        value("flare", params.flare.as_ptr(), 620.0, 480.0, 26.0),
        value("bloom", params.bloom.as_ptr(), 748.0, 500.0, 24.0),
        value("tremor", params.tremor.as_ptr(), 850.0, 440.0, 20.0),
        value("wander", params.wander.as_ptr(), 920.0, 520.0, 18.0),
        value("fade", params.fade.as_ptr(), 80.0, 470.0, 26.0),
        value("curve", params.curve.as_ptr(), 164.0, 512.0, 14.0),
    ]
}

#[derive(Clone, Copy, PartialEq)]
enum Target {
    Dial(usize),
    Release,
    Frames(bool),
    Read(bool),
    Void,
}

enum PanelEvent {
    Tick,
}

struct Drag {
    target: Target,
    param: ParamPtr,
    anchor_y: f32,
    anchor_value: f32,
    fine: bool,
    moved: f32,
}

#[derive(Clone, Copy)]
struct Fonts {
    small_caps: vg::FontId,
    italic: vg::FontId,
    mono: vg::FontId,
}

thread_local! {
    /// Fonts are registered per canvas; the canvas address tells us when a reopened
    /// editor brought a new one.
    static FONTS: Cell<Option<(usize, Fonts)>> = const { Cell::new(None) };
}

fn fonts(canvas: &mut Canvas) -> Option<Fonts> {
    let key = canvas as *const Canvas as usize;
    if let Some((k, fonts)) = FONTS.get() {
        let mut probe = Paint::color(Color::white());
        probe.set_font(&[fonts.mono]);
        if k == key && canvas.measure_text(0.0, 0.0, "A", &probe).is_ok() {
            return Some(fonts);
        }
    }
    let fonts = Fonts {
        small_caps: canvas.add_font_mem(FELL_SC).ok()?,
        italic: canvas.add_font_mem(FELL_ITALIC).ok()?,
        mono: canvas.add_font_mem(MONO).ok()?,
    };
    FONTS.set(Some((key, fonts)));
    Some(fonts)
}

#[derive(Clone, Copy)]
enum Face {
    SmallCaps,
    Italic,
    Mono,
}

struct Painter<'a> {
    canvas: &'a mut Canvas,
    fonts: Option<Fonts>,
}

impl Painter<'_> {
    fn fill_circle(&mut self, x: f32, y: f32, r: f32, fill: Color) {
        let mut path = Path::new();
        path.circle(x, y, r);
        self.canvas.fill_path(&path, &Paint::color(fill));
    }

    fn stroke(&mut self, path: &Path, stroke: Color, width: f32) {
        let mut paint = Paint::color(stroke);
        paint.set_line_width(width);
        self.canvas.stroke_path(path, &paint);
    }

    fn circle(&mut self, x: f32, y: f32, r: f32, stroke: Color, width: f32) {
        let mut path = Path::new();
        path.circle(x, y, r);
        self.stroke(&path, stroke, width);
    }

    /// An arc from angle `a0` to `a1` (radians, clockwise from the right), as a polyline.
    fn arc(&mut self, x: f32, y: f32, r: f32, a0: f32, a1: f32, stroke: Color, width: f32) {
        let steps = ((a1 - a0).abs() * r / 3.0).ceil().max(2.0) as usize;
        let mut path = Path::new();
        for i in 0..=steps {
            let a = a0 + (a1 - a0) * i as f32 / steps as f32;
            let (px, py) = (x + r * a.cos(), y + r * a.sin());
            if i == 0 {
                path.move_to(px, py);
            } else {
                path.line_to(px, py);
            }
        }
        let mut paint = Paint::color(stroke);
        paint.set_line_width(width);
        paint.set_line_cap(vg::LineCap::Round);
        self.canvas.stroke_path(&path, &paint);
    }

    /// A ring of short dashes: `on` and `off` in pixels along it.
    fn dashed(&mut self, x: f32, y: f32, r: f32, on: f32, off: f32, stroke: Color, width: f32) {
        let count = (TAU * r / (on + off)).floor().max(1.0) as usize;
        let step = TAU / count as f32;
        let dash = step * on / (on + off);
        let mut path = Path::new();
        for i in 0..count {
            let a = i as f32 * step;
            path.move_to(x + r * a.cos(), y + r * a.sin());
            path.line_to(x + r * (a + dash).cos(), y + r * (a + dash).sin());
        }
        self.stroke(&path, stroke, width);
    }

    fn text(&mut self, x: f32, y: f32, text: &str, face: Face, size: f32, fill: Color, align: Align, baseline: Baseline) {
        let mut paint = Paint::color(fill);
        if let Some(fonts) = &self.fonts {
            paint.set_font(&[match face {
                Face::SmallCaps => fonts.small_caps,
                Face::Italic => fonts.italic,
                Face::Mono => fonts.mono,
            }]);
        }
        paint.set_font_size(size);
        paint.set_text_align(align);
        paint.set_text_baseline(baseline);
        if let Face::SmallCaps = face {
            paint.set_letter_spacing(0.12 * size);
        }
        let _ = self.canvas.fill_text(x, y, text, &paint);
    }

    fn star(&mut self, star: &Star, t: f32) {
        let (brightness, grow) = star.at(t);
        let r = 0.5 * star.size * grow;
        if star.size >= 1.8 || grow > 1.0 {
            self.fill_circle(star.x, star.y, r * 2.6, color(STAR, 0.18 * brightness));
        }
        self.fill_circle(star.x, star.y, r, color(STAR, brightness));
    }
}

pub struct Panel {
    params: Arc<ArgentParams>,
    telemetry: Arc<Telemetry>,
    dials: Vec<Dial>,
    stars: Vec<Star>,
    eye: Vec<Star>,
    eye_dread: f32,
    book: Book,
    snapshot: Snapshot,
    drag: Option<Drag>,
    hover: Option<Target>,
    opened: Instant,
    /// Keeps the ticker thread running; cleared when the panel goes away.
    ticking: Arc<AtomicBool>,
}

impl Drop for Panel {
    fn drop(&mut self) {
        self.ticking.store(false, Ordering::Relaxed);
    }
}

fn clock_seed() -> u32 {
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.subsec_nanos() ^ d.as_secs() as u32).unwrap_or(1);
    nanos.wrapping_mul(0x9E37_79B1) ^ std::process::id()
}

impl Panel {
    pub fn new(cx: &mut Context, params: Arc<ArgentParams>, telemetry: Arc<Telemetry>) -> Handle<'_, Self> {
        let ticking = Arc::new(AtomicBool::new(true));
        let ticker = ticking.clone();
        let dread = params.dread.value();
        Self {
            dials: dial_layout(&params),
            stars: sky::field(WIDTH, HEIGHT, VOID),
            eye: sky::eye(dread, VOID),
            eye_dread: dread,
            book: Book::new(clock_seed()),
            snapshot: telemetry.snapshot(),
            params,
            telemetry,
            drag: None,
            hover: None,
            opened: Instant::now(),
            ticking,
        }
        .build(cx, move |cx| {
            // vizia's timers never fire under baseview, but events sent through a
            // context proxy are delivered every frame. The proxy targets this panel.
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

    fn transform(bounds: BoundingBox) -> (f32, f32, f32) {
        let scale = (bounds.w / WIDTH).min(bounds.h / HEIGHT);
        (scale, bounds.x + (bounds.w - WIDTH * scale) / 2.0, bounds.y + (bounds.h - HEIGHT * scale) / 2.0)
    }

    fn to_panel(cx: &EventContext, x: f32, y: f32) -> (f32, f32) {
        let (scale, ox, oy) = Self::transform(cx.bounds());
        ((x - ox) / scale, (y - oy) / scale)
    }

    fn hit(&self, x: f32, y: f32) -> Option<Target> {
        for (rect, target) in [
            (FRAMES_64, Target::Frames(false)),
            (FRAMES_32, Target::Frames(true)),
            (READ_STEP, Target::Read(false)),
            (READ_GLIDE, Target::Read(true)),
        ] {
            if rect.contains(x, y) {
                return Some(target);
            }
        }
        if (x - RELEASE.0).hypot(y - RELEASE.1) <= RELEASE.2 + 4.0 {
            return Some(Target::Release);
        }
        if let Some(i) = self.dials.iter().position(|d| (x - d.x).hypot(y - d.y) <= d.r + GRAB) {
            return Some(Target::Dial(i));
        }
        ((x - VOID.x).hypot(y - VOID.y) <= VOID.r).then_some(Target::Void)
    }

    /// Works around a vizia bug: when the pointer goes past the window's right or bottom
    /// edge (during a drag, say), vizia marks the window as "not under the pointer" and
    /// only undoes that when the platform reports the cursor re-entering, which baseview
    /// on Windows never does. So, whenever the pointer is inside the window, say it entered.
    fn keep_hover_alive(cx: &mut EventContext) {
        let bounds = cx.bounds();
        let (x, y) = (cx.mouse().cursorx, cx.mouse().cursory);
        if x >= bounds.x && y >= bounds.y && x < bounds.x + bounds.w && y < bounds.y + bounds.h {
            cx.emit_custom(Event::new(WindowEvent::MouseEnter).origin(Entity::root()).direct(Entity::root()));
        }
    }

    fn tick(&mut self, cx: &mut EventContext) {
        Self::keep_hover_alive(cx);
        self.snapshot = self.telemetry.snapshot();
        let dread = self.params.dread.value();
        if (dread - self.eye_dread).abs() > 0.005 {
            self.eye = sky::eye(dread, VOID);
            self.eye_dread = dread;
        }
        // The sky is always moving a little.
        cx.needs_redraw();
    }

    fn begin(cx: &mut EventContext, p: ParamPtr) {
        cx.emit(RawParamEvent::BeginSetParameter(p));
    }

    fn set(cx: &mut EventContext, p: ParamPtr, v: f32) {
        cx.emit(RawParamEvent::SetParameterNormalized(p, v.clamp(0.0, 1.0)));
    }

    fn end(cx: &mut EventContext, p: ParamPtr) {
        cx.emit(RawParamEvent::EndSetParameter(p));
    }

    fn value_of(p: ParamPtr) -> f32 {
        unsafe { p.unmodulated_normalized_value() }
    }

    fn set_once(cx: &mut EventContext, p: ParamPtr, v: f32) {
        Self::begin(cx, p);
        Self::set(cx, p, v);
        Self::end(cx, p);
    }

    fn param_of(&self, target: Target) -> Option<ParamPtr> {
        match target {
            Target::Dial(i) => Some(self.dials[i].param),
            Target::Void => Some(self.params.dread.as_ptr()),
            Target::Release => Some(self.params.release.as_ptr()),
            Target::Frames(_) => Some(self.params.coarse.as_ptr()),
            Target::Read(_) => Some(self.params.glide.as_ptr()),
        }
    }

    fn mouse_down(&mut self, cx: &mut EventContext, x: f32, y: f32) {
        // A drag whose release never arrived: finish it before starting anything new.
        self.mouse_up(cx);
        let Some(target) = self.hit(x, y) else { return };
        let Some(param) = self.param_of(target) else { return };
        match target {
            Target::Frames(on) | Target::Read(on) => Self::set_once(cx, param, if on { 1.0 } else { 0.0 }),
            Target::Release => {
                Self::begin(cx, param);
                Self::set(cx, param, 1.0);
                self.drag = Some(Drag { target, param, anchor_y: y, anchor_value: 1.0, fine: false, moved: 0.0 });
                cx.capture();
            }
            Target::Dial(_) | Target::Void => {
                Self::begin(cx, param);
                let fine = cx.modifiers().contains(Modifiers::SHIFT);
                self.drag = Some(Drag { target, param, anchor_y: y, anchor_value: Self::value_of(param), fine, moved: 0.0 });
                cx.capture();
            }
        }
    }

    fn is_seed(&self, target: Target) -> bool {
        matches!(target, Target::Dial(i) if self.dials[i].kind == DialKind::Seed)
    }

    fn mouse_move(&mut self, cx: &mut EventContext, x: f32, y: f32) {
        if self.drag.is_some() && cx.mouse().left.state != MouseButtonState::Pressed {
            self.mouse_up(cx); // the button is up but we never heard the release
        }
        let hover = self.hit(x, y);
        if hover != self.hover {
            self.hover = hover;
            cx.needs_redraw();
        }
        let fine = cx.modifiers().contains(Modifiers::SHIFT);
        let seed_target = self.drag.as_ref().is_some_and(|d| self.is_seed(d.target));
        let Some(drag) = &mut self.drag else { return };
        if drag.target == Target::Release {
            return;
        }
        drag.moved = drag.moved.max((y - drag.anchor_y).abs());
        if fine != drag.fine {
            // Re-anchor, so changing speed never makes the value jump.
            drag.anchor_value = Self::value_of(drag.param);
            drag.anchor_y = y;
            drag.fine = fine;
        }
        let rise = drag.anchor_y - y;
        let v = if seed_target {
            let steps = (rise / SEED_PIXELS_PER_STEP).round();
            drag.anchor_value + steps / (SEEDS - 1) as f32
        } else {
            drag.anchor_value + rise / DRAG_PIXELS * if fine { FINE } else { 1.0 }
        };
        Self::set(cx, drag.param, v);
        cx.needs_redraw();
    }

    fn mouse_up(&mut self, cx: &mut EventContext) {
        let Some(drag) = self.drag.take() else { return };
        if drag.target == Target::Release {
            Self::set(cx, drag.param, 0.0);
        } else if self.is_seed(drag.target) && drag.moved < CLICK_PIXELS {
            // A click on SEED: somewhere new.
            let seed = clock_seed() % SEEDS as u32;
            Self::set(cx, drag.param, seed as f32 / (SEEDS - 1) as f32);
        }
        Self::end(cx, drag.param);
        cx.release();
        cx.needs_redraw();
    }

    fn reset(&mut self, cx: &mut EventContext, x: f32, y: f32) {
        self.mouse_up(cx);
        if let Some(target @ (Target::Dial(_) | Target::Void)) = self.hit(x, y) {
            if self.is_seed(target) {
                return; // a double click on SEED is two rerolls, not a reset
            }
            if let Some(p) = self.param_of(target) {
                Self::set_once(cx, p, unsafe { p.default_normalized_value() });
            }
        }
    }

    fn scroll(&mut self, cx: &mut EventContext, x: f32, y: f32, notches: f32) {
        let Some(target @ (Target::Dial(_) | Target::Void)) = self.hit(x, y) else { return };
        let Some(p) = self.param_of(target) else { return };
        let step = match unsafe { p.step_count() } {
            Some(steps) => 1.0 / steps as f32,
            None => SCROLL_STEP * if cx.modifiers().contains(Modifiers::SHIFT) { FINE } else { 1.0 },
        };
        Self::set_once(cx, p, Self::value_of(p) + notches * step);
    }

    fn position(&self, name: &str) -> Option<(f32, f32, f32)> {
        if name == "release" {
            return Some(RELEASE);
        }
        self.dials.iter().find(|d| d.label == name).map(|d| (d.x, d.y, d.r))
    }

    fn paint_sky(&self, p: &mut Painter, t: f32) {
        for star in self.stars.iter().chain(&self.eye) {
            p.star(star, t);
        }
        let dread = self.params.dread.value();
        let (r, g, b) = BLOOD;
        let glow = Paint::radial_gradient(
            VOID.x,
            VOID.y,
            0.0,
            GLOW_RADIUS,
            Color::rgba(r, g, b, (0.6 * dread * 255.0) as u8),
            Color::rgba(r, g, b, 0),
        );
        let mut path = Path::new();
        path.circle(VOID.x, VOID.y, GLOW_RADIUS);
        p.canvas.fill_path(&path, &glow);
    }

    fn paint_joins(&self, p: &mut Painter) {
        let mut path = Path::new();
        for (a, b) in JOINS {
            let (Some((ax, ay, ar)), Some((bx, by, br))) = (self.position(a), self.position(b)) else { continue };
            let d = (bx - ax).hypot(by - ay);
            let (ux, uy) = ((bx - ax) / d, (by - ay) / d);
            path.move_to(ax + ux * (ar + 9.0), ay + uy * (ar + 9.0));
            path.line_to(bx - ux * (br + 9.0), by - uy * (br + 9.0));
        }
        p.stroke(&path, color(LINE, 0.75), 0.8);
    }

    fn paint_void(&self, p: &mut Painter) {
        let shown = self.snapshot.shown;
        p.fill_circle(VOID.x, VOID.y, VOID.r, color(HOLLOW, 0.55));
        p.circle(VOID.x, VOID.y, VOID.r, color(INK, 0.9), 0.9);
        p.dashed(VOID.x, VOID.y, DASHED_RING, 2.0, 6.0, color(DASH, 1.0), 0.5);
        p.circle(VOID.x, VOID.y, INNER_RING, color(INNER, 1.0), 0.6);

        // The frame being played, read once round the rim.
        let peak = self.snapshot.trace.iter().fold(1e-6f32, |m, v| m.max(v.abs()));
        let mut path = Path::new();
        for i in 0..=TRACE {
            let v = self.snapshot.trace[i % TRACE] / peak;
            let a = TAU * i as f32 / TRACE as f32 - FRAC_PI_2;
            let r = TRACE_RADIUS + TRACE_SWING * v;
            let (x, y) = (VOID.x + r * a.cos(), VOID.y + r * a.sin());
            if i == 0 {
                path.move_to(x, y);
            } else {
                path.line_to(x, y);
            }
        }
        p.stroke(&path, color(BRIGHT, if shown.sounding { 0.88 } else { 0.3 }), 1.1);

        let dread = self.params.dread.value();
        p.fill_circle(VOID.x, VOID.y, 2.2, color(if dread > 0.5 { PUPIL_RED } else { INNER }, 1.0));

        let step = if shown.frames > 0 { 64 / shown.frames.max(1) } else { 1 };
        let readout = format!(
            "SCAN {:.1} · MORPH {:.2} · FRAME {} / {}",
            shown.table + 1.0,
            shown.wave,
            shown.frame / step.max(1) + 1,
            shown.frames
        );
        p.text(VOID.x, 172.0, &readout, Face::Mono, 10.0, color(DIM, 1.0), Align::Center, Baseline::Middle);
    }

    fn paint_dial(&self, p: &mut Painter, dial: &Dial, hovered: bool) {
        let (x, y, r) = (dial.x, dial.y, dial.r);
        let v = Self::value_of(dial.param);
        p.circle(x, y, r + 5.0, color(ORBIT, if hovered { 0.55 } else { 0.3 }), 0.5);
        let (sx, sy) = match dial.kind {
            DialKind::Value => {
                p.arc(x, y, r, DIAL_START, DIAL_START + SWEEP, color(TRACK, 1.0), 1.0);
                let end = DIAL_START + SWEEP * v;
                if v > 0.002 {
                    p.arc(x, y, r, DIAL_START, end, color(BRIGHT, 1.0), 1.5);
                }
                (x + r * end.cos(), y + r * end.sin())
            }
            DialKind::Seed => {
                p.dashed(x, y, r, 1.5, 4.0, color(SOFT, 0.9), 0.8);
                // The seed's star sits where its number puts it round the ring.
                let a = TAU * v - FRAC_PI_2;
                (x + r * a.cos(), y + r * a.sin())
            }
        };
        p.fill_circle(sx, sy, 6.5, color(STAR, if hovered { 0.3 } else { 0.16 }));
        p.fill_circle(sx, sy, 2.4, color(WHITE, 1.0));
        p.fill_circle(x, y, 1.4, color(SOFT, 1.0));
        let label_y = y + r + 14.0;
        p.text(x, label_y, dial.label, Face::SmallCaps, 12.5, color(INK, 1.0), Align::Center, Baseline::Middle);
        let shown = unsafe { dial.param.normalized_value_to_string(v, true) };
        p.text(x, label_y + 15.0, &shown, Face::Mono, 10.5, color(DIM, 1.0), Align::Center, Baseline::Middle);
    }

    fn paint_pill(p: &mut Painter, r: Rect, label: &str, on: bool, hovered: bool) {
        let mut path = Path::new();
        path.rounded_rect(r.x + 0.5, r.y + 0.5, r.w - 1.0, r.h - 1.0, r.h / 2.0);
        let edge = if on { color(INK, 1.0) } else if hovered { color(DIM, 1.0) } else { color(PILL_OFF, 1.0) };
        p.stroke(&path, edge, 1.0);
        let ink = if on { color(0xeef0f4, 1.0) } else { color(DIM, 1.0) };
        p.text(r.x + r.w / 2.0, r.y + r.h / 2.0, label, Face::Mono, 11.0, ink, Align::Center, Baseline::Middle);
    }

    fn paint_switches(&self, p: &mut Painter) {
        let label = |p: &mut Painter, y: f32, text: &str| {
            p.text(32.0, y + PILL_H / 2.0, text, Face::SmallCaps, 12.5, color(SOFT, 1.0), Align::Left, Baseline::Middle);
        };
        let (coarse, glide) = (self.params.coarse.value(), self.params.glide.value());
        let hovered = |t: Target| self.hover == Some(t);
        label(p, FRAMES_64.y, "frames");
        Self::paint_pill(p, FRAMES_64, "64", !coarse, hovered(Target::Frames(false)));
        Self::paint_pill(p, FRAMES_32, "32", coarse, hovered(Target::Frames(true)));
        label(p, READ_STEP.y, "read");
        Self::paint_pill(p, READ_STEP, "STEP", !glide, hovered(Target::Read(false)));
        Self::paint_pill(p, READ_GLIDE, "GLIDE", glide, hovered(Target::Read(true)));
    }

    fn paint_release(&self, p: &mut Painter) {
        let (x, y, r) = RELEASE;
        let down = self.params.release.value();
        let hovered = self.hover == Some(Target::Release);
        p.fill_circle(x, y, r, color(if down { 0x1c2029 } else { BUTTON }, 1.0));
        p.circle(x, y, r, color(if hovered || down { BRIGHT } else { SOFT }, 1.0), 1.0);
        p.dashed(x, y, 8.0, 2.0, 3.0, color(INK, 1.0), 0.9);
        p.fill_circle(x, y, 2.0, color(0xeef0f4, 1.0));
        p.text(x, y + r + 14.0, "release", Face::SmallCaps, 12.5, color(INK, 1.0), Align::Center, Baseline::Middle);
    }

    fn paint_words(&self, p: &mut Painter, t: f32) {
        for (name, x, y) in NAMES {
            p.text(x, y, name, Face::Italic, 16.0, color(SOFT, 1.0), Align::Left, Baseline::Top);
        }
        if let Some((line, alpha)) = self.book.at(t) {
            p.text(WIDTH / 2.0, 602.0, line, Face::Italic, 13.5, color(DIM, alpha), Align::Center, Baseline::Middle);
        }
    }
}

impl View for Panel {
    fn element(&self) -> Option<&'static str> {
        Some("argent-panel")
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
                _ => {}
            }
        });
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        let (scale, ox, oy) = Self::transform(cx.bounds());
        let t = self.opened.elapsed().as_secs_f32();
        canvas.save();
        canvas.translate(ox, oy);
        canvas.scale(scale, scale);
        let fonts = fonts(canvas);
        let mut p = Painter { canvas, fonts };
        let mut ground = Path::new();
        ground.rect(0.0, 0.0, WIDTH, HEIGHT);
        p.canvas.fill_path(&ground, &Paint::color(color(GROUND, 1.0)));
        self.paint_sky(&mut p, t);
        self.paint_joins(&mut p);
        self.paint_void(&mut p);
        for (i, dial) in self.dials.iter().enumerate() {
            self.paint_dial(&mut p, dial, self.hover == Some(Target::Dial(i)));
        }
        self.paint_switches(&mut p);
        self.paint_release(&mut p);
        self.paint_words(&mut p, t);
        canvas.restore();
    }
}
