//! The panel: the band, three strips and the rest. Sliders jump to a press and drag from
//! there; Shift is fine, double-click resets, scroll nudges.

use std::cell::Cell;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use nih_plug::prelude::{Param, ParamPtr};
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::vizia::vg::{self, Align, Baseline, Color, Paint, Path};
use nih_plug_vizia::widgets::RawParamEvent;

use super::waterfall::{Waterfall, COLUMNS, ROWS, SHADES, WINDOW};
use crate::dsp::station::AGC_MOST;
use crate::dsp::wear::CHANNELS;
use crate::params::AreciboParams;
use crate::telemetry::Telemetry;

pub const WIDTH: f32 = 860.0;
pub const HEIGHT: f32 = 580.0;

const GROUND: u32 = 0x0b0a09;
const INK: u32 = 0xece8df;
const DIM: u32 = 0x8a867d;
const RULE: u32 = 0x4d4a44;
const RED: u32 = 0xf0402c;
/// From just above the ground up through red to paper.
const WATER_SHADES: [u32; SHADES] = [0x241312, 0x4a1a14, 0x8c2a1c, 0xd93c28, 0xf08a70, 0xece8df];

const MARGIN: f32 = 24.0;
const CONTENT_WIDTH: f32 = WIDTH - 2.0 * MARGIN;
const WATER: Rect = Rect { x: MARGIN, y: 44.0, w: CONTENT_WIDTH, h: 136.0 };
const STRIP_GAP: f32 = 18.0;
const STRIP_WIDTH: f32 = (CONTENT_WIDTH - (CHANNELS - 1) as f32 * STRIP_GAP) / CHANNELS as f32;
const STRIPS_Y: f32 = 192.0;
const STRIP_HEADER: f32 = 18.0;
const STRIP_ROW: f32 = 32.0;
const GLOBALS_Y: f32 = 322.0;
const GLOBAL_ROW: f32 = 37.0;
const GLOBAL_ROWS: usize = 6;
const COLUMN_GAP: f32 = 24.0;
const COLUMN_WIDTH: f32 = (CONTENT_WIDTH - COLUMN_GAP) / 2.0;
const STRIP_LABEL: f32 = 42.0;
const STRIP_VALUE: f32 = 58.0;
const GLOBAL_LABEL: f32 = 80.0;
const GLOBAL_VALUE: f32 = 76.0;
const STRIP_NAMES: [&str; CHANNELS] = ["A", "B", "C", "D"];
/// What arrives at each station from outside.
const STRIP_SOURCES: [&str; CHANNELS] = ["in L", "in R", "side L", "side R"];
const STRIP_SOURCE_WIDTH: f32 = 62.0;

const TICK: Duration = Duration::from_millis(33);
const WATER_TICKS: u32 = 2;
const WATER_RATE_HZ: f32 = 24_000.0;
const FINE: f32 = 0.12;
const SCROLL_STEP: f32 = 0.01;
const LIMIT_FULL_DB: f32 = 12.0;
const LIMIT_WIDTH: f32 = 160.0;
const LIMIT_FALL_DB: f32 = 40.0;

static MONO: &[u8] = include_bytes!("../../assets/fonts/Mono.ttf");
static MEDIUM: &[u8] = include_bytes!("../../assets/fonts/MonoMedium.ttf");

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

struct Slider {
    label: &'static str,
    param: ParamPtr,
    row: Rect,
    track: Rect,
}

impl Slider {
    fn new(label: &'static str, param: ParamPtr, row: Rect, label_width: f32, value_width: f32) -> Self {
        Self { label, param, row, track: Rect { x: row.x + label_width, y: row.y, w: row.w - label_width - value_width, h: row.h } }
    }
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
    let fonts = Fonts { regular: canvas.add_font_mem(MONO).ok()?, medium: canvas.add_font_mem(MEDIUM).ok()? };
    FONTS.set(Some((key, fonts)));
    Some(fonts)
}

/// A face, its size and its colour.
type Style = (fn(&Fonts) -> vg::FontId, f32, u32);

const REGULAR: fn(&Fonts) -> vg::FontId = |f| f.regular;
const MEDIUM_WEIGHT: fn(&Fonts) -> vg::FontId = |f| f.medium;

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
}

pub struct Panel {
    telemetry: Arc<Telemetry>,
    sliders: Vec<Slider>,
    drag: Option<Drag>,
    waterfall: Waterfall,
    samples: [f32; WINDOW],
    limit_db: f32,
    ticks: u32,
    /// Keeps the ticker thread running; cleared when the panel goes away.
    ticking: Arc<AtomicBool>,
}

impl Drop for Panel {
    fn drop(&mut self) {
        self.ticking.store(false, Ordering::Relaxed);
    }
}

fn strip_x(channel: usize) -> f32 {
    MARGIN + channel as f32 * (STRIP_WIDTH + STRIP_GAP)
}

fn sliders(params: &AreciboParams) -> Vec<Slider> {
    let mut sliders = Vec::new();
    for (channel, strip) in params.strips.iter().enumerate() {
        let rows: [(&'static str, ParamPtr); 3] = [("Tune", strip.tune.as_ptr()), ("Peak", strip.peak.as_ptr()), ("Gain", strip.gain.as_ptr())];
        for (i, (label, param)) in rows.into_iter().enumerate() {
            let row = Rect { x: strip_x(channel), y: STRIPS_Y + STRIP_HEADER + i as f32 * STRIP_ROW, w: STRIP_WIDTH, h: STRIP_ROW };
            sliders.push(Slider::new(label, param, row, STRIP_LABEL, STRIP_VALUE));
        }
    }
    let globals: [(&'static str, ParamPtr); 12] = [
        ("Feedback", params.feedback.as_ptr()),
        ("Cross", params.cross.as_ptr()),
        ("Sky", params.sky.as_ptr()),
        ("Width", params.width.as_ptr()),
        ("Distance", params.distance.as_ptr()),
        ("AGC", params.agc.as_ptr()),
        ("Weather", params.weather.as_ptr()),
        ("Age", params.age.as_ptr()),
        ("Input", params.input.as_ptr()),
        ("Side In", params.side_input.as_ptr()),
        ("Dry", params.dry.as_ptr()),
        ("Level", params.level.as_ptr()),
    ];
    for (i, (label, param)) in globals.into_iter().enumerate() {
        let (column, row) = (i / GLOBAL_ROWS, i % GLOBAL_ROWS);
        let rect = Rect { x: MARGIN + column as f32 * (COLUMN_WIDTH + COLUMN_GAP), y: GLOBALS_Y + row as f32 * GLOBAL_ROW, w: COLUMN_WIDTH, h: GLOBAL_ROW };
        sliders.push(Slider::new(label, param, rect, GLOBAL_LABEL, GLOBAL_VALUE));
    }
    sliders
}

impl Panel {
    pub fn new(cx: &mut Context, params: Arc<AreciboParams>, telemetry: Arc<Telemetry>) -> Handle<'_, Self> {
        let ticking = Arc::new(AtomicBool::new(true));
        let ticker = ticking.clone();
        Self {
            telemetry,
            sliders: sliders(&params),
            drag: None,
            waterfall: Waterfall::new(WATER_RATE_HZ),
            samples: [0.0; WINDOW],
            limit_db: 0.0,
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

    fn hit(&self, x: f32, y: f32) -> Option<usize> {
        self.sliders.iter().position(|s| s.track.contains(x, y))
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
        self.ticks = self.ticks.wrapping_add(1);
        let fallen = (self.limit_db - LIMIT_FALL_DB * TICK.as_secs_f32()).max(0.0);
        self.limit_db = fallen.max(self.telemetry.take_reduction_db());
        if self.ticks.is_multiple_of(WATER_TICKS) {
            self.telemetry.latest(&mut self.samples);
            let samples = self.samples;
            if self.waterfall.add(&samples) {
                cx.needs_redraw();
            }
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

    fn mouse_down(&mut self, cx: &mut EventContext, x: f32, y: f32) {
        // A drag whose release never arrived: finish it before starting anything new.
        self.mouse_up(cx);
        let fine = cx.modifiers().contains(Modifiers::SHIFT);
        if let Some(i) = self.hit(x, y) {
            let (p, track) = (self.sliders[i].param, self.sliders[i].track);
            Self::begin(cx, p);
            let mut anchor_value = Self::value_of(p);
            if !fine {
                anchor_value = ((x - track.x) / track.w).clamp(0.0, 1.0);
                Self::set(cx, p, anchor_value);
            }
            self.drag = Some(Drag { param: p, per_pixel: 1.0 / track.w, anchor: x, anchor_value, fine });
            cx.capture();
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
        if let Some(i) = self.hit(x, y) {
            let p = self.sliders[i].param;
            Self::set_once(cx, p, unsafe { p.default_normalized_value() });
        }
    }

    fn scroll(&mut self, cx: &mut EventContext, x: f32, y: f32, notches: f32) {
        if let Some(i) = self.hit(x, y) {
            let p = self.sliders[i].param;
            let step = SCROLL_STEP * if cx.modifiers().contains(Modifiers::SHIFT) { FINE } else { 1.0 };
            Self::set_once(cx, p, Self::value_of(p) + notches * step);
        }
    }

    /// One path per shade, merging runs within a column, keeps this cheap.
    fn paint_water(&self, p: &mut Painter) {
        p.rect(WATER, 0x080707);
        let (cell_w, cell_h) = (WATER.w / COLUMNS as f32, WATER.h / ROWS as f32);
        for (shade, ink) in WATER_SHADES.iter().enumerate() {
            let wanted = (shade + 1) as u8;
            let mut path = Path::new();
            for (c, column) in self.waterfall.columns().enumerate() {
                let x = WATER.x + c as f32 * cell_w;
                let mut r = 0;
                while r < ROWS {
                    if column[r].min(SHADES as u8) == wanted {
                        let start = r;
                        while r < ROWS && column[r].min(SHADES as u8) == wanted {
                            r += 1;
                        }
                        let y = WATER.y + WATER.h - r as f32 * cell_h;
                        path.rect(x, y, cell_w + 0.5, (r - start) as f32 * cell_h);
                    } else {
                        r += 1;
                    }
                }
            }
            p.canvas.fill_path(&path, &Paint::color(color(*ink)));
        }
    }

    fn paint_strip_headers(&self, p: &mut Painter) {
        for (channel, name) in STRIP_NAMES.iter().enumerate() {
            let x = strip_x(channel);
            let y = STRIPS_Y + STRIP_HEADER / 2.0;
            p.text(x, y, name, (MEDIUM_WEIGHT, 13.0, INK), Align::Left);
            p.text(x + 16.0, y, STRIP_SOURCES[channel], (REGULAR, 11.0, DIM), Align::Left);
            // The S-meter: full when the receiver needs no gain, empty when it's straining.
            let gain = self.telemetry.agc_gain(channel).clamp(1.0, AGC_MOST);
            let strength = 1.0 - gain.ln() / AGC_MOST.ln();
            let meter = Rect { x: x + STRIP_SOURCE_WIDTH, y: y - 1.5, w: STRIP_WIDTH - STRIP_SOURCE_WIDTH, h: 3.0 };
            p.rect(Rect { y: y - 0.5, h: 1.0, ..meter }, RULE);
            p.rect(Rect { w: meter.w * strength, ..meter }, DIM);
        }
    }

    fn paint_sliders(&self, p: &mut Painter) {
        for slider in &self.sliders {
            let (row, track) = (slider.row, slider.track);
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

    fn paint_limit(&self, p: &mut Painter) {
        // In the top corner: the patch has no row to spare for it.
        let meter = Rect { x: WIDTH - MARGIN - LIMIT_WIDTH, y: 22.0, w: LIMIT_WIDTH, h: 4.0 };
        p.text(meter.x - 8.0, meter.y + 2.0, "Limit", (REGULAR, 11.0, DIM), Align::Right);
        p.rect(Rect { h: 1.0, y: meter.y + 1.5, ..meter }, RULE);
        let depth = (self.limit_db / LIMIT_FULL_DB).min(1.0);
        if depth > 0.0 {
            p.rect(Rect { w: meter.w * depth, ..meter }, RED);
        }
    }
}

impl View for Panel {
    fn element(&self) -> Option<&'static str> {
        Some("arecibo-panel")
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
        p.text(MARGIN, 24.0, "ARECIBO", (MEDIUM_WEIGHT, 13.0, INK), Align::Left);
        self.paint_water(&mut p);
        self.paint_strip_headers(&mut p);
        self.paint_sliders(&mut p);
        self.paint_limit(&mut p);
        canvas.restore();
    }
}
