//! The panel: what it's saying, written down, and what you can do about it. Sliders jump
//! to a press and drag from there; Shift is fine, double-click resets, scroll nudges.
//! Behind it all, very faintly, something moves.

use std::cell::Cell;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use nih_plug::prelude::{Param, ParamPtr};
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::vizia::vg::{self, Align, Baseline, Color, Paint, Path};
use nih_plug_vizia::widgets::RawParamEvent;

use super::tentacles::{Tentacle, Tentacles};
use super::transcript::Transcript;
use crate::dsp::language::Language;
use crate::params::AmaraParams;
use crate::telemetry::Telemetry;

pub const WIDTH: f32 = 860.0;
pub const HEIGHT: f32 = 556.0;

const GROUND: u32 = 0x0b0a09;
const INK: u32 = 0xece8df;
const DIM: u32 = 0x8a867d;
const RULE: u32 = 0x4d4a44;
const RED: u32 = 0xf0402c;

const MARGIN: f32 = 24.0;
const CONTENT_WIDTH: f32 = WIDTH - 2.0 * MARGIN;
const COLUMN_GAP: f32 = 24.0;
const COLUMN_WIDTH: f32 = (CONTENT_WIDTH - COLUMN_GAP) / 2.0;

const TRANSCRIPT_Y: f32 = 70.0;
const TRANSCRIPT_PITCH: f32 = 46.0;
const TRANSCRIPT_SIZE: f32 = 29.0;

const SLIDERS_Y: f32 = 198.0;
const SLIDER_ROW: f32 = 40.0;
const SLIDER_ROWS: usize = 6;
const LABEL_WIDTH: f32 = 100.0;
const VALUE_WIDTH: f32 = 76.0;

const BOTTOM_Y: f32 = 446.0;
const LANGUAGE_ROW: Rect = Rect { x: MARGIN, y: BOTTOM_Y, w: COLUMN_WIDTH, h: SLIDER_ROW };
const STRANGENESS_ROW: Rect = Rect { x: MARGIN + COLUMN_WIDTH + COLUMN_GAP, y: BOTTOM_Y, w: COLUMN_WIDTH, h: SLIDER_ROW };
const BUTTONS_Y: f32 = BOTTOM_Y + SLIDER_ROW + 12.0;
const BUTTON_WIDTH: f32 = 120.0;
const BUTTON_HEIGHT: f32 = 30.0;
const SEIZE: Rect = Rect { x: MARGIN, y: BUTTONS_Y, w: BUTTON_WIDTH, h: BUTTON_HEIGHT };
const GATE: Rect = Rect { x: SEIZE.x + BUTTON_WIDTH + 12.0, y: BUTTONS_Y, w: BUTTON_WIDTH, h: BUTTON_HEIGHT };
const LIMIT_METER: Rect = Rect { x: STRANGENESS_ROW.x, y: BUTTONS_Y + 10.0, w: COLUMN_WIDTH, h: 4.0 };
const LIMIT_FULL_DB: f32 = 12.0;
/// Rails and ribs of the tentacles at their most visible, which isn't very.
const TENTACLE_RAIL_ALPHA: f32 = 0.085;
const TENTACLE_RIB_ALPHA: f32 = 0.04;
/// They move so slowly that every other tick is plenty.
const TENTACLE_TICKS: u32 = 2;

const FREE_RUNNING_ONLY: [&str; 4] = ["Sparsity", "Temper", "Swing", "Pitch"];
/// One breath of the 666 pulse, in seconds, and how far it sinks into the ground.
const PULSE_S: f32 = 2.6;
const PULSE_DEPTH: f32 = 0.6;
const TICK: Duration = Duration::from_millis(33);
const FINE: f32 = 0.12;
const SCROLL_STEP: f32 = 0.01;
/// Limiting dies away on screen at this much per second, so short peaks can be seen.
const LIMIT_FALL_DB: f32 = 40.0;

static MONO: &[u8] = include_bytes!("../../assets/fonts/Mono.ttf");
static MEDIUM: &[u8] = include_bytes!("../../assets/fonts/MonoMedium.ttf");
static IPA: &[u8] = include_bytes!("../../assets/fonts/Ipa.ttf");

fn color(hex: u32) -> Color {
    Color::rgb((hex >> 16) as u8, (hex >> 8) as u8, hex as u8)
}

/// `hex` sunk this far toward `into`.
fn sink(hex: u32, into: u32, amount: f32) -> u32 {
    let channel = |shift: u32| {
        let (a, b) = (((hex >> shift) & 0xff) as f32, ((into >> shift) & 0xff) as f32);
        ((a + (b - a) * amount) as u32) << shift
    };
    channel(16) | channel(8) | channel(0)
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

fn slider_row(i: usize) -> Rect {
    let (column, row) = (i / SLIDER_ROWS, i % SLIDER_ROWS);
    Rect { x: MARGIN + column as f32 * (COLUMN_WIDTH + COLUMN_GAP), y: SLIDERS_Y + row as f32 * SLIDER_ROW, w: COLUMN_WIDTH, h: SLIDER_ROW }
}

/// The part of a slider row you can grab: right of the label, left of the value.
fn slider_track(row: Rect) -> Rect {
    Rect { x: row.x + LABEL_WIDTH, y: row.y, w: row.w - LABEL_WIDTH - VALUE_WIDTH, h: row.h }
}

struct Slider {
    label: &'static str,
    param: ParamPtr,
    row: Rect,
    /// Means nothing while one note says one syllable.
    free_running_only: bool,
}

#[derive(Clone, Copy, PartialEq)]
enum Target {
    Slider(usize),
    Seize,
    Gate,
}

/// Sent to the panel by its ticker thread.
enum PanelEvent {
    Tick,
}

struct Drag {
    param: ParamPtr,
    per_pixel: f32,
    anchor: f32,
    anchor_value: f32,
    fine: bool,
}

#[derive(Clone, Copy)]
struct Fonts {
    regular: vg::FontId,
    medium: vg::FontId,
    ipa: vg::FontId,
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
        ipa: canvas.add_font_mem(IPA).ok()?,
    };
    FONTS.set(Some((key, fonts)));
    Some(fonts)
}

/// A face, its size and its colour.
type Style = (fn(&Fonts) -> vg::FontId, f32, u32);

const REGULAR: fn(&Fonts) -> vg::FontId = |f| f.regular;
const MEDIUM_WEIGHT: fn(&Fonts) -> vg::FontId = |f| f.medium;
const PHONETIC: fn(&Fonts) -> vg::FontId = |f| f.ipa;

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

    fn line(&mut self, path: &Path, alpha: f32) {
        let mut ink = color(INK);
        ink.set_alphaf(alpha);
        let mut paint = Paint::color(ink);
        paint.set_line_width(1.0);
        self.canvas.stroke_path(path, &paint);
    }

    /// One tentacle: its outline up one side, round the tip and down the other, then the
    /// ribs across it.
    fn tentacle(&mut self, tentacle: &Tentacle) {
        let visibility = tentacle.visibility();
        if visibility <= 0.0 {
            return;
        }
        let (mut rails, mut ribs) = (Path::new(), Path::new());
        let mut first = true;
        tentacle.for_each_edge(true, |x, y| {
            if first {
                rails.move_to(x, y);
                first = false;
            } else {
                rails.line_to(x, y);
            }
        });
        tentacle.for_each_edge(false, |x, y| rails.line_to(x, y));
        tentacle.for_each_rib(|(x1, y1), (x2, y2)| {
            ribs.move_to(x1, y1);
            ribs.line_to(x2, y2);
        });
        self.line(&rails, TENTACLE_RAIL_ALPHA * visibility);
        self.line(&ribs, TENTACLE_RIB_ALPHA * visibility);
    }

    /// Returns how wide it came out.
    fn text(&mut self, x: f32, y: f32, text: &str, style: Style, align: Align) -> f32 {
        let (font, size, fill) = style;
        let mut paint = Paint::color(color(fill));
        if let Some(fonts) = &self.fonts {
            paint.set_font(&[font(fonts)]);
        }
        paint.set_font_size(size);
        paint.set_text_align(align);
        paint.set_text_baseline(Baseline::Middle);
        self.canvas.fill_text(x, y, text, &paint).map(|m| m.width()).unwrap_or(0.0)
    }
}

pub struct Panel {
    params: Arc<AmaraParams>,
    telemetry: Arc<Telemetry>,
    sliders: Vec<Slider>,
    drag: Option<Drag>,
    transcript: Transcript,
    marks_seen: u32,
    language_shown: (i32, f32, String),
    limit_db: f32,
    tentacles: Tentacles,
    started: Instant,
    last_tentacle_step: Instant,
    ticks: u32,
    /// Keeps the ticker thread running; cleared when the panel goes away.
    ticking: Arc<AtomicBool>,
}

impl Drop for Panel {
    fn drop(&mut self) {
        self.ticking.store(false, Ordering::Relaxed);
    }
}

impl Panel {
    pub fn new(cx: &mut Context, params: Arc<AmaraParams>, telemetry: Arc<Telemetry>) -> Handle<'_, Self> {
        let ticking = Arc::new(AtomicBool::new(true));
        let ticker = ticking.clone();
        let list: [(&'static str, ParamPtr); 12] = [
            ("Rate", params.rate.as_ptr()),
            ("Sparsity", params.sparsity.as_ptr()),
            ("Temper", params.temper.as_ptr()),
            ("Swing", params.swing.as_ptr()),
            ("Breath", params.breath.as_ptr()),
            ("Throat", params.throat.as_ptr()),
            ("Possession", params.possession.as_ptr()),
            ("Extremity", params.extremity.as_ptr()),
            ("Line", params.line.as_ptr()),
            ("Fidelity", params.fidelity.as_ptr()),
            ("Pitch", params.pitch.as_ptr()),
            ("Level", params.level.as_ptr()),
        ];
        let mut sliders: Vec<Slider> =
            list.iter().enumerate().map(|(i, (label, param))| Slider { label, param: *param, row: slider_row(i), free_running_only: FREE_RUNNING_ONLY.contains(label) }).collect();
        sliders.push(Slider { label: "Language", param: params.language.as_ptr(), row: LANGUAGE_ROW, free_running_only: false });
        sliders.push(Slider { label: "Strangeness", param: params.strangeness.as_ptr(), row: STRANGENESS_ROW, free_running_only: false });
        Self {
            params,
            telemetry,
            sliders,
            drag: None,
            transcript: Transcript::new(),
            marks_seen: 0,
            language_shown: (-1, 0.0, String::new()),
            limit_db: 0.0,
            tentacles: Tentacles::new(std::process::id()),
            started: Instant::now(),
            last_tentacle_step: Instant::now(),
            ticks: 0,
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
        if SEIZE.contains(x, y) {
            return Some(Target::Seize);
        }
        if GATE.contains(x, y) {
            return Some(Target::Gate);
        }
        self.sliders.iter().position(|s| slider_track(s.row).contains(x, y)).map(Target::Slider)
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
        let mut changed = self.refresh_language_name();
        let possessed = self.telemetry.possessed();
        let transcript = &mut self.transcript;
        self.telemetry.read_since(&mut self.marks_seen, |mark| {
            transcript.add(mark, possessed);
            changed = true;
        });
        let fallen = (self.limit_db - LIMIT_FALL_DB * TICK.as_secs_f32()).max(0.0);
        let limit = fallen.max(self.telemetry.take_reduction_db());
        // The pulse never stops moving, on the language that has one.
        changed |= self.params.devilish();
        self.ticks = self.ticks.wrapping_add(1);
        if self.ticks.is_multiple_of(TENTACLE_TICKS) {
            let now = Instant::now();
            self.tentacles.step((now - self.last_tentacle_step).as_secs_f32());
            self.last_tentacle_step = now;
            changed |= self.tentacles.any_visible();
        }
        if limit != self.limit_db || changed {
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

    fn toggle(cx: &mut EventContext, p: ParamPtr) {
        Self::set_once(cx, p, if Self::value_of(p) > 0.5 { 0.0 } else { 1.0 });
    }

    fn mouse_down(&mut self, cx: &mut EventContext, x: f32, y: f32) {
        // A drag whose release never arrived: finish it before starting anything new.
        self.mouse_up(cx);
        let fine = cx.modifiers().contains(Modifiers::SHIFT);
        match self.hit(x, y) {
            Some(Target::Slider(i)) => {
                let (p, track) = (self.sliders[i].param, slider_track(self.sliders[i].row));
                Self::begin(cx, p);
                let mut anchor_value = Self::value_of(p);
                if !fine {
                    anchor_value = ((x - track.x) / track.w).clamp(0.0, 1.0);
                    Self::set(cx, p, anchor_value);
                }
                self.drag = Some(Drag { param: p, per_pixel: 1.0 / track.w, anchor: x, anchor_value, fine });
                cx.capture();
            }
            Some(Target::Seize) => Self::toggle(cx, self.params.seize.as_ptr()),
            Some(Target::Gate) => Self::toggle(cx, self.params.per_note.as_ptr()),
            None => {}
        }
    }

    fn mouse_move(&mut self, cx: &mut EventContext, x: f32) {
        if self.drag.is_some() && cx.mouse().left.state != MouseButtonState::Pressed {
            self.mouse_up(cx); // the button is up but we never heard the release
        }
        let fine = cx.modifiers().contains(Modifiers::SHIFT);
        if let Some(drag) = &mut self.drag {
            if fine != drag.fine {
                // Re-anchor, so changing speed never makes the value jump.
                drag.anchor_value = Self::value_of(drag.param);
                drag.anchor = x;
                drag.fine = fine;
            }
            let v = drag.anchor_value + (x - drag.anchor) * drag.per_pixel * if fine { FINE } else { 1.0 };
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
        if let Some(Target::Slider(i)) = self.hit(x, y) {
            let p = self.sliders[i].param;
            Self::set_once(cx, p, unsafe { p.default_normalized_value() });
        }
    }

    fn scroll(&mut self, cx: &mut EventContext, x: f32, y: f32, notches: f32) {
        if let Some(Target::Slider(i)) = self.hit(x, y) {
            let p = self.sliders[i].param;
            // One language to a notch, so any of them can be reached exactly.
            let step = match unsafe { p.step_count() } {
                Some(steps) => 1.0 / steps as f32,
                None => SCROLL_STEP * if cx.modifiers().contains(Modifiers::SHIFT) { FINE } else { 1.0 },
            };
            Self::set_once(cx, p, Self::value_of(p) + notches * step);
        }
    }

    /// The name changes rarely and costs a whole language to work out. Says if it changed.
    fn refresh_language_name(&mut self) -> bool {
        let (seed, strangeness) = (self.params.language.unmodulated_plain_value(), self.params.strangeness());
        let changed = (self.language_shown.0, self.language_shown.1) != (seed, strangeness);
        if changed {
            self.language_shown = (seed, strangeness, Language::new(seed as u32, strangeness).name());
        }
        changed
    }

    /// The red of language 666, wherever the pulse has got to.
    fn pulsing_red(&self) -> u32 {
        let breath = (std::f32::consts::TAU * self.started.elapsed().as_secs_f32() / PULSE_S).sin();
        sink(RED, GROUND, PULSE_DEPTH * (0.5 - 0.5 * breath))
    }

    fn paint_header(&self, p: &mut Painter) {
        let (title, name) = if self.params.devilish() { (self.pulsing_red(), self.pulsing_red()) } else { (INK, DIM) };
        p.text(MARGIN, 24.0, "AMARA", (MEDIUM_WEIGHT, 13.0, title), Align::Left);
        p.text(WIDTH - MARGIN, 24.0, &self.language_shown.2, (PHONETIC, 15.0, name), Align::Right);
    }

    fn paint_transcript(&self, p: &mut Painter) {
        // Red already means possessed, so on 666 possession shows up the other way round.
        let (ordinary, possessed) = if self.params.devilish() { (self.pulsing_red(), INK) } else { (INK, RED) };
        for (i, runs) in self.transcript.lines().enumerate() {
            let y = TRANSCRIPT_Y + i as f32 * TRANSCRIPT_PITCH;
            let mut x = MARGIN;
            for run in runs {
                let ink = if run.possessed { possessed } else { ordinary };
                x += p.text(x, y, &run.text, (PHONETIC, TRANSCRIPT_SIZE, ink), Align::Left);
            }
        }
    }

    /// A slider's value as it stands, which isn't always what the knob says.
    fn shown_value(&self, slider: &Slider) -> f32 {
        if slider.param == self.params.possession.as_ptr() && self.params.devilish() {
            // A linear 0-1 knob, so its plain value is its position.
            self.params.possession()
        } else {
            Self::value_of(slider.param)
        }
    }

    fn paint_sliders(&self, p: &mut Painter) {
        let per_note = self.params.per_note.value();
        for slider in &self.sliders {
            let (row, track) = (slider.row, slider_track(slider.row));
            let y = row.y + row.h / 2.0;
            let (ink, line) = if per_note && slider.free_running_only { (RULE, RULE) } else { (INK, DIM) };
            p.rect(Rect { x: row.x, y: row.y, w: row.w, h: 1.0 }, RULE);
            p.text(row.x, y, slider.label, (MEDIUM_WEIGHT, 13.0, ink), Align::Left);
            p.rect(Rect { x: track.x, y: y - 0.5, w: track.w, h: 1.0 }, line);
            let value = self.shown_value(slider);
            p.rect(Rect { x: track.x + value * track.w - 1.5, y: y - 7.0, w: 3.0, h: 14.0 }, ink);
            let shown = unsafe { slider.param.normalized_value_to_string(value, true) };
            p.text(row.x + row.w, y, &shown, (REGULAR, 12.0, ink), Align::Right);
        }
    }

    fn paint_button(p: &mut Painter, r: Rect, label: &str, on: bool, lit: u32) {
        if on {
            p.rect(r, lit);
        } else {
            p.outline(r, DIM);
        }
        p.text(r.x + r.w / 2.0, r.y + r.h / 2.0, label, (MEDIUM_WEIGHT, 12.0, if on { GROUND } else { INK }), Align::Center);
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
        Some("amara-panel")
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
                    let (x, _) = Self::to_panel(cx, px, py);
                    self.mouse_move(cx, x);
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
        self.tentacles.alive().for_each(|tentacle| p.tentacle(tentacle));
        self.paint_header(&mut p);
        self.paint_transcript(&mut p);
        self.paint_sliders(&mut p);
        Self::paint_button(&mut p, SEIZE, "Seize", self.params.seize.value(), RED);
        Self::paint_button(&mut p, GATE, "Per note", self.params.per_note.value(), INK);
        self.paint_limit(&mut p);
        canvas.restore();
    }
}
