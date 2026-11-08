//! The panel. Sliders jump to a press and drag from there; Shift is fine, double-click
//! resets, scroll nudges. The big numerals drag up and down for Reclock. The matrix and
//! the loop are plain toggles.

use std::cell::Cell;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use nih_plug::prelude::{Param, ParamPtr};
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::vizia::vg::{self, Align, Baseline, Color, Paint, Path};
use nih_plug_vizia::widgets::RawParamEvent;

use crate::dsp::engine::STOCK_HZ;
use crate::dsp::program::PROGRAM_NAMES;
use crate::dsp::ram::SWITCHES;
use crate::params::{clock_hz, clock_knob, AnathemaParams};
use crate::telemetry::Telemetry;

pub const WIDTH: f32 = 860.0;
pub const HEIGHT: f32 = 520.0;

const GROUND: u32 = 0x0b0a09;
const INK: u32 = 0xece8df;
const DIM: u32 = 0x8a867d;
const RULE: u32 = 0x4d4a44;
const RED: u32 = 0xf0402c;

const MARGIN: f32 = 24.0;
const LEFT_WIDTH: f32 = 540.0;
const RIGHT_X: f32 = 596.0;
const RIGHT_WIDTH: f32 = 240.0;

const NUMERALS: Rect = Rect { x: 0.0, y: 40.0, w: 580.0, h: 172.0 };
const RECLOCK: Rect = Rect { x: MARGIN, y: 214.0, w: LEFT_WIDTH, h: 34.0 };
const PROGRAMS_Y: f32 = 262.0;
const PROGRAMS_HEIGHT: f32 = 26.0;
const SLIDERS_Y: f32 = 304.0;
const SLIDER_ROW: f32 = 41.0;
const SLIDER_ROWS: usize = 5;
const SLIDER_COLUMN_GAP: f32 = 24.0;
const MATRIX_Y: f32 = 50.0;
const MATRIX_SIDE: usize = 4;
const CELL_GAP: f32 = 8.0;
const ROW_PITCH: f32 = 60.0;
const CELL_HEIGHT: f32 = 40.0;
const ROW_CAPTIONS: [&str; MATRIX_SIDE] = ["Tones", "Tremolo", "Rhythm", "Crunch"];
const LOOP_BUTTON: Rect = Rect { x: RIGHT_X, y: 304.0, w: RIGHT_WIDTH, h: 56.0 };
const LIMIT_METER: Rect = Rect { x: RIGHT_X, y: 392.0, w: RIGHT_WIDTH, h: 4.0 };
const LIMIT_FULL_DB: f32 = 12.0;

const TICK: Duration = Duration::from_millis(33);
const FINE: f32 = 0.12;
const NUMERALS_TRAVEL: f32 = 400.0;
const SCROLL_STEP: f32 = 0.01;
/// Limiting dies away on screen at this much per second, so short peaks can be seen.
const LIMIT_FALL_DB: f32 = 40.0;

/// The numerals' cuts: where each band starts and how far it has slipped. A nod to its
/// sibling until this one gets a design of its own.
const BANDS: [(f32, f32, f32); 5] = [(0.0, 72.0, 0.0), (72.0, 9.0, 34.0), (81.0, 48.0, 6.0), (129.0, 5.0, -22.0), (134.0, 38.0, 0.0)];
const NUMERAL_SIZE: f32 = 196.0;
const NUMERAL_BASELINE: f32 = 208.0;

static MONO: &[u8] = include_bytes!("../../assets/fonts/Mono.ttf");
static MEDIUM: &[u8] = include_bytes!("../../assets/fonts/MonoMedium.ttf");
static NUMERAL: &[u8] = include_bytes!("../../assets/fonts/Numerals.ttf");

fn color(hex: u32) -> Color {
    Color::rgb((hex >> 16) as u8, (hex >> 8) as u8, hex as u8)
}

#[derive(Clone, Copy, Debug)]
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

fn program_cell(i: usize) -> Rect {
    let w = LEFT_WIDTH / PROGRAM_NAMES.len() as f32;
    Rect { x: MARGIN + i as f32 * w, y: PROGRAMS_Y, w, h: PROGRAMS_HEIGHT }
}

fn switch_cell(i: usize) -> Rect {
    let w = (RIGHT_WIDTH - CELL_GAP * (MATRIX_SIDE - 1) as f32) / MATRIX_SIDE as f32;
    let (row, column) = (i / MATRIX_SIDE, i % MATRIX_SIDE);
    Rect { x: RIGHT_X + column as f32 * (w + CELL_GAP), y: MATRIX_Y + row as f32 * ROW_PITCH + 14.0, w, h: CELL_HEIGHT }
}

fn slider_row(i: usize) -> Rect {
    let w = (LEFT_WIDTH - SLIDER_COLUMN_GAP) / 2.0;
    let (column, row) = (i / SLIDER_ROWS, i % SLIDER_ROWS);
    Rect { x: MARGIN + column as f32 * (w + SLIDER_COLUMN_GAP), y: SLIDERS_Y + row as f32 * SLIDER_ROW, w, h: SLIDER_ROW }
}

/// The part of a slider row you can grab: right of the label, left of the value.
fn slider_track(row: Rect) -> Rect {
    const LABEL: f32 = 92.0;
    const VALUE: f32 = 62.0;
    Rect { x: row.x + LABEL, y: row.y, w: row.w - LABEL - VALUE, h: row.h }
}

struct Slider {
    label: &'static str,
    param: ParamPtr,
    row: Rect,
}

#[derive(Clone, Copy, PartialEq)]
enum Target {
    Slider(usize),
    Reclock,
    Numerals,
    Program(usize),
    Switch(usize),
    Loop,
}

/// Sent to the panel by its ticker thread.
enum PanelEvent {
    Tick,
}

struct Drag {
    param: ParamPtr,
    vertical: bool,
    per_pixel: f32,
    anchor: f32,
    anchor_value: f32,
    fine: bool,
}

#[derive(Clone, Copy)]
struct Fonts {
    regular: vg::FontId,
    medium: vg::FontId,
    numeral: vg::FontId,
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
        probe.set_font(&[fonts.regular]);
        if k == key && canvas.measure_text(0.0, 0.0, "A", &probe).is_ok() {
            return Some(fonts);
        }
    }
    let fonts = Fonts {
        regular: canvas.add_font_mem(MONO).ok()?,
        medium: canvas.add_font_mem(MEDIUM).ok()?,
        numeral: canvas.add_font_mem(NUMERAL).ok()?,
    };
    FONTS.set(Some((key, fonts)));
    Some(fonts)
}

/// A face, its size and its colour.
type Style = (fn(&Fonts) -> vg::FontId, f32, u32);

struct Painter<'a> {
    canvas: &'a mut Canvas,
    fonts: Option<Fonts>,
}

impl Painter<'_> {
    fn rect(&mut self, r: Rect, fill: u32) {
        let mut path = Path::new();
        path.rect(r.x, r.y, r.w, r.h);
        self.canvas.fill_path(&path, &Paint::color(color(fill)));
    }

    fn outline(&mut self, r: Rect, stroke: u32) {
        let mut path = Path::new();
        path.rect(r.x + 0.5, r.y + 0.5, r.w - 1.0, r.h - 1.0);
        let mut paint = Paint::color(color(stroke));
        paint.set_line_width(1.0);
        self.canvas.stroke_path(&path, &paint);
    }

    fn text(&mut self, x: f32, y: f32, text: &str, style: Style, align: Align) {
        let (font, size, fill) = style;
        let mut paint = Paint::color(color(fill));
        if let Some(fonts) = &self.fonts {
            paint.set_font(&[font(fonts)]);
        }
        paint.set_font_size(size);
        paint.set_text_align(align);
        paint.set_text_baseline(Baseline::Middle);
        let _ = self.canvas.fill_text(x, y, text, &paint);
    }

    fn numerals(&mut self, text: &str) {
        let mut paint = Paint::color(color(INK));
        if let Some(fonts) = &self.fonts {
            paint.set_font(&[fonts.numeral]);
        }
        paint.set_font_size(NUMERAL_SIZE);
        paint.set_text_baseline(Baseline::Alphabetic);
        for (top, height, slip) in BANDS {
            self.canvas.save();
            self.canvas.intersect_scissor(NUMERALS.x, NUMERALS.y + top, NUMERALS.w, height);
            paint.set_color(color(if height < 10.0 { RED } else { INK }));
            let _ = self.canvas.fill_text(MARGIN + slip, NUMERAL_BASELINE, text, &paint);
            self.canvas.restore();
        }
    }
}

const REGULAR: fn(&Fonts) -> vg::FontId = |f| f.regular;
const MEDIUM_WEIGHT: fn(&Fonts) -> vg::FontId = |f| f.medium;

pub struct Panel {
    params: Arc<AnathemaParams>,
    telemetry: Arc<Telemetry>,
    sliders: Vec<Slider>,
    drag: Option<Drag>,
    limit_db: f32,
    /// Keeps the ticker thread running; cleared when the panel goes away.
    ticking: Arc<AtomicBool>,
}

impl Drop for Panel {
    fn drop(&mut self) {
        self.ticking.store(false, Ordering::Relaxed);
    }
}

impl Panel {
    pub fn new(cx: &mut Context, params: Arc<AnathemaParams>, telemetry: Arc<Telemetry>) -> Handle<'_, Self> {
        let ticking = Arc::new(AtomicBool::new(true));
        let ticker = ticking.clone();
        let list: [(&'static str, ParamPtr); 10] = [
            ("Decay", params.decay.as_ptr()),
            ("Damp", params.damp.as_ptr()),
            ("Pre Delay", params.predelay.as_ptr()),
            ("Mix", params.mix.as_ptr()),
            ("Direct", params.direct.as_ptr()),
            ("Drive", params.drive.as_ptr()),
            ("Whine", params.whine.as_ptr()),
            ("Rot", params.rot.as_ptr()),
            ("Contention", params.contention.as_ptr()),
            ("Level", params.level.as_ptr()),
        ];
        let sliders = list.iter().enumerate().map(|(i, (label, param))| Slider { label, param: *param, row: slider_row(i) }).collect();
        Self { params, telemetry, sliders, drag: None, limit_db: 0.0, ticking }
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
        if RECLOCK.contains(x, y) {
            return Some(Target::Reclock);
        }
        if LOOP_BUTTON.contains(x, y) {
            return Some(Target::Loop);
        }
        if let Some(i) = (0..SWITCHES).find(|i| switch_cell(*i).contains(x, y)) {
            return Some(Target::Switch(i));
        }
        if let Some(i) = (0..PROGRAM_NAMES.len()).find(|i| program_cell(*i).contains(x, y)) {
            return Some(Target::Program(i));
        }
        if let Some(i) = self.sliders.iter().position(|s| slider_track(s.row).contains(x, y)) {
            return Some(Target::Slider(i));
        }
        NUMERALS.contains(x, y).then_some(Target::Numerals)
    }

    fn param_of(&self, target: Target) -> ParamPtr {
        match target {
            Target::Slider(i) => self.sliders[i].param,
            Target::Reclock | Target::Numerals => self.params.reclock.as_ptr(),
            Target::Program(_) => self.params.program.as_ptr(),
            Target::Switch(i) => self.params.matrix[i].on.as_ptr(),
            Target::Loop => self.params.loop_ram.as_ptr(),
        }
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
        let fallen = (self.limit_db - LIMIT_FALL_DB * TICK.as_secs_f32()).max(0.0);
        let limit = fallen.max(self.telemetry.take_reduction_db());
        if limit != self.limit_db {
            self.limit_db = limit;
            cx.needs_redraw();
        }
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

    fn start_slider(&mut self, cx: &mut EventContext, p: ParamPtr, track: Rect, x: f32, fine: bool) {
        Self::begin(cx, p);
        let mut anchor_value = Self::value_of(p);
        if !fine {
            anchor_value = ((x - track.x) / track.w).clamp(0.0, 1.0);
            Self::set(cx, p, anchor_value);
        }
        self.drag = Some(Drag { param: p, vertical: false, per_pixel: 1.0 / track.w, anchor: x, anchor_value, fine });
        cx.capture();
    }

    fn mouse_down(&mut self, cx: &mut EventContext, x: f32, y: f32) {
        // A drag whose release never arrived: finish it before starting anything new.
        self.mouse_up(cx);
        let fine = cx.modifiers().contains(Modifiers::SHIFT);
        let Some(target) = self.hit(x, y) else { return };
        let p = self.param_of(target);
        match target {
            Target::Slider(i) => self.start_slider(cx, p, slider_track(self.sliders[i].row), x, fine),
            Target::Reclock => self.start_slider(cx, p, RECLOCK, x, fine),
            Target::Numerals => {
                Self::begin(cx, p);
                self.drag = Some(Drag { param: p, vertical: true, per_pixel: 1.0 / NUMERALS_TRAVEL, anchor: y, anchor_value: Self::value_of(p), fine });
                cx.capture();
            }
            Target::Program(i) => Self::set_once(cx, p, i as f32 / (PROGRAM_NAMES.len() - 1) as f32),
            Target::Switch(_) | Target::Loop => Self::set_once(cx, p, if Self::value_of(p) > 0.5 { 0.0 } else { 1.0 }),
        }
    }

    fn mouse_move(&mut self, cx: &mut EventContext, x: f32, y: f32) {
        if self.drag.is_some() && cx.mouse().left.state != MouseButtonState::Pressed {
            self.mouse_up(cx); // the button is up but we never heard the release
        }
        let fine = cx.modifiers().contains(Modifiers::SHIFT);
        if let Some(drag) = &mut self.drag {
            let at = if drag.vertical { y } else { x };
            if fine != drag.fine {
                // Re-anchor, so changing speed never makes the value jump.
                drag.anchor_value = Self::value_of(drag.param);
                drag.anchor = at;
                drag.fine = fine;
            }
            let moved = if drag.vertical { drag.anchor - at } else { at - drag.anchor };
            let v = drag.anchor_value + moved * drag.per_pixel * if fine { FINE } else { 1.0 };
            Self::set(cx, drag.param, v);
            cx.needs_redraw();
        }
    }

    fn mouse_up(&mut self, cx: &mut EventContext) {
        if let Some(drag) = self.drag.take() {
            Self::end(cx, drag.param);
            cx.release();
        }
    }

    fn reset(&mut self, cx: &mut EventContext, x: f32, y: f32) {
        self.mouse_up(cx);
        match self.hit(x, y) {
            // The toggles already changed on the first press of the double-click.
            None | Some(Target::Switch(_) | Target::Loop | Target::Program(_)) => {}
            Some(target) => {
                let p = self.param_of(target);
                Self::set_once(cx, p, unsafe { p.default_normalized_value() });
            }
        }
    }

    fn scroll(&mut self, cx: &mut EventContext, x: f32, y: f32, notches: f32) {
        if let Some(target @ (Target::Slider(_) | Target::Reclock | Target::Numerals)) = self.hit(x, y) {
            let p = self.param_of(target);
            let step = SCROLL_STEP * if cx.modifiers().contains(Modifiers::SHIFT) { FINE } else { 1.0 };
            Self::set_once(cx, p, Self::value_of(p) + notches * step);
        }
    }

    fn paint_clock(&self, p: &mut Painter) {
        let knob = self.params.reclock.unmodulated_plain_value();
        p.numerals(&format!("{:.0}", clock_hz(knob)));

        let (x, y, w) = (RECLOCK.x, RECLOCK.y + RECLOCK.h / 2.0, RECLOCK.w);
        p.rect(Rect { x, y: y - 0.5, w, h: 1.0 }, DIM);
        let stock = x + clock_knob(STOCK_HZ) * w;
        p.rect(Rect { x: stock - 1.5, y: y - 8.0, w: 3.0, h: 3.0 }, INK);
        p.text(stock, y + 11.0, "stock", (REGULAR, 11.0, DIM), Align::Center);
        p.text(x, y + 11.0, "Reclock", (MEDIUM_WEIGHT, 13.0, INK), Align::Left);
        p.text(x + w, y - 11.0, "Hz", (REGULAR, 13.0, INK), Align::Right);
        p.rect(Rect { x: x + knob * w - 1.5, y: y - 8.0, w: 3.0, h: 16.0 }, INK);
    }

    fn paint_programs(&self, p: &mut Painter) {
        let current = self.params.program.unmodulated_plain_value() as usize;
        for (i, name) in PROGRAM_NAMES.iter().enumerate() {
            let cell = program_cell(i);
            let (font, ink) = if i == current { (MEDIUM_WEIGHT, INK) } else { (REGULAR, DIM) };
            p.text(cell.x + cell.w / 2.0, cell.y + cell.h / 2.0, name, (font, 13.0, ink), Align::Center);
            if i == current {
                p.rect(Rect { x: cell.x + 6.0, y: cell.y + cell.h - 2.0, w: cell.w - 12.0, h: 2.0 }, RED);
            }
        }
    }

    fn paint_sliders(&self, p: &mut Painter) {
        for slider in &self.sliders {
            let (row, track) = (slider.row, slider_track(slider.row));
            let y = row.y + row.h / 2.0;
            p.rect(Rect { x: row.x, y: row.y, w: row.w, h: 1.0 }, RULE);
            p.text(row.x, y, slider.label, (MEDIUM_WEIGHT, 13.0, INK), Align::Left);
            p.rect(Rect { x: track.x, y: y - 0.5, w: track.w, h: 1.0 }, DIM);
            let value = Self::value_of(slider.param);
            p.rect(Rect { x: track.x + value * track.w - 1.5, y: y - 7.0, w: 3.0, h: 14.0 }, INK);
            let shown = unsafe { slider.param.normalized_value_to_string(value, true) };
            p.text(row.x + row.w, y, &shown, (REGULAR, 12.0, INK), Align::Right);
        }
    }

    fn paint_matrix(&self, p: &mut Painter) {
        p.text(RIGHT_X, 30.0, "Matrix", (MEDIUM_WEIGHT, 13.0, INK), Align::Left);
        p.text(RIGHT_X + RIGHT_WIDTH, 30.0, "close two or more", (REGULAR, 11.0, DIM), Align::Right);
        for (row, caption) in ROW_CAPTIONS.iter().enumerate() {
            p.text(RIGHT_X, MATRIX_Y + row as f32 * ROW_PITCH + 6.0, caption, (REGULAR, 11.0, DIM), Align::Left);
        }
        for i in 0..SWITCHES {
            let cell = switch_cell(i);
            if self.params.matrix[i].on.value() {
                p.rect(cell, RED);
            } else {
                p.outline(cell, DIM);
            }
        }
    }

    fn paint_loop(&self, p: &mut Painter) {
        let on = self.params.loop_ram.value();
        if on {
            p.rect(LOOP_BUTTON, INK);
        } else {
            p.outline(LOOP_BUTTON, DIM);
        }
        p.text(LOOP_BUTTON.x + LOOP_BUTTON.w / 2.0, LOOP_BUTTON.y + LOOP_BUTTON.h / 2.0, "RAM Loop", (MEDIUM_WEIGHT, 14.0, if on { GROUND } else { INK }), Align::Center);
        // How long one trip round the memory takes at this clock.
        let seconds = crate::dsp::ram::WORDS as f32 / clock_hz(self.params.reclock.unmodulated_plain_value());
        p.text(LOOP_BUTTON.x, LOOP_BUTTON.y + LOOP_BUTTON.h + 12.0, &format!("{seconds:.2} s around"), (REGULAR, 11.0, DIM), Align::Left);
    }

    fn paint_limit(&self, p: &mut Painter) {
        let meter = LIMIT_METER;
        p.text(meter.x, meter.y + 14.0, "Limit", (REGULAR, 11.0, DIM), Align::Left);
        p.rect(Rect { h: 1.0, y: meter.y + 1.5, ..meter }, RULE);
        let depth = (self.limit_db / LIMIT_FULL_DB).min(1.0);
        if depth > 0.0 {
            p.rect(Rect { w: meter.w * depth, ..meter }, RED);
        }
    }
}

impl View for Panel {
    fn element(&self) -> Option<&'static str> {
        Some("anathema-panel")
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
        canvas.save();
        canvas.translate(ox, oy);
        canvas.scale(scale, scale);
        let fonts = fonts(canvas);
        let mut p = Painter { canvas, fonts };
        p.rect(Rect { x: 0.0, y: 0.0, w: WIDTH, h: HEIGHT }, GROUND);
        p.text(MARGIN, 24.0, "ANATHEMA", (MEDIUM_WEIGHT, 13.0, INK), Align::Left);
        self.paint_clock(&mut p);
        self.paint_programs(&mut p);
        self.paint_sliders(&mut p);
        self.paint_matrix(&mut p);
        self.paint_loop(&mut p);
        self.paint_limit(&mut p);
        canvas.restore();
    }
}
