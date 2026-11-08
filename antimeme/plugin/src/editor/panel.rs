//! The panel: drawing, dragging, and listening to the audio thread.
//!
//! Sliders: the middle grabs the value (a press there jumps to it), the top edge the
//! Agitation send, the bottom edge the Interference send. Shift for fine, double-click
//! to reset, scroll to nudge. The big numerals drag up and down for Time.

use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use nih_plug::prelude::{Param, ParamPtr};
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::vizia::vg::{Align, BlendFactor, Color};
use nih_plug_vizia::widgets::RawParamEvent;

use super::glitch::{Corruption, Frame, Glitch, BANDS};
use super::layout::{self, Rect, Row, Zone, HEIGHT, TIME, WIDTH};
use super::paint::{color, fade, symbol, Face, Painter, GROUND, INK, RED, RED_LABEL, RULE, TRACK};
use crate::dsp::agitation::shape;
use crate::dsp::voice::time_seconds;
use crate::params::AntimemeParams;
use crate::telemetry::Telemetry;

const TICK: Duration = Duration::from_millis(33);
/// Shift-drag moves this much slower.
const FINE: f32 = 0.12;
/// Dragging the numerals this far covers all of Time.
const NUMERALS_TRAVEL: f32 = 400.0;
const SCROLL_STEP: f32 = 0.01;
/// About ten seconds across.
const HISTORY: usize = 306;

/// Sent to the panel by its ticker thread.
enum PanelEvent {
    Tick,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Target {
    Slider(usize, Zone),
    Numerals,
}

struct Drag {
    param: ParamPtr,
    numerals: bool,
    /// How far a pixel moves the normalized value.
    per_pixel: f32,
    anchor: f32,
    anchor_value: f32,
    fine: bool,
}

pub struct Panel {
    params: Arc<AntimemeParams>,
    telemetry: Arc<Telemetry>,
    rows: Vec<Row>,
    hover: Option<Target>,
    drag: Option<Drag>,
    glitch: Glitch,
    history: VecDeque<f32>,
    started: Instant,
    last_tick: Instant,
    last_slip: f32,
    last_phase_x: i32,
    /// Keeps the ticker thread running; cleared when the panel goes away.
    ticking: Arc<AtomicBool>,
}

impl Drop for Panel {
    fn drop(&mut self) {
        self.ticking.store(false, Ordering::Relaxed);
    }
}

impl Panel {
    pub fn new(cx: &mut Context, params: Arc<AntimemeParams>, telemetry: Arc<Telemetry>) -> Handle<'_, Self> {
        let ticking = Arc::new(AtomicBool::new(true));
        let ticker = ticking.clone();
        let rows = layout::rows(&params);
        let now = Instant::now();
        Self {
            params,
            telemetry,
            rows,
            hover: None,
            drag: None,
            glitch: Glitch::new(),
            history: VecDeque::from(vec![0.0; HISTORY]),
            started: now,
            last_tick: now,
            last_slip: 0.0,
            last_phase_x: 0,
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

    // ---- Coordinates ---------------------------------------------------------

    fn transform(bounds: BoundingBox) -> (f32, f32, f32) {
        let scale = (bounds.w / WIDTH).min(bounds.h / HEIGHT);
        (scale, bounds.x + (bounds.w - WIDTH * scale) / 2.0, bounds.y + (bounds.h - HEIGHT * scale) / 2.0)
    }

    fn to_panel(cx: &EventContext, x: f32, y: f32) -> (f32, f32) {
        let (scale, ox, oy) = Self::transform(cx.bounds());
        ((x - ox) / scale, (y - oy) / scale)
    }

    fn hit(&self, x: f32, y: f32) -> Option<Target> {
        for (i, row) in self.rows.iter().enumerate() {
            if row.slider.contains(x, y) {
                return Some(Target::Slider(i, row.zone_at(y)));
            }
        }
        layout::NUMERALS.contains(x, y).then_some(Target::Numerals)
    }

    fn time_param(&self) -> ParamPtr {
        self.rows[TIME].value
    }

    // ---- Ticking ----------------------------------------------------------------

    /// Works around a vizia bug: when the pointer goes past the window's right or bottom
    /// edge (during a drag, say), vizia marks the window as "not under the pointer" and only
    /// undoes that when the platform reports the cursor re-entering, which baseview on
    /// Windows never does. Until then every click goes to the window instead of the
    /// panel. So, whenever the pointer is inside the window, say it entered.
    fn keep_hover_alive(cx: &mut EventContext) {
        let bounds = cx.bounds();
        let (x, y) = (cx.mouse().cursorx, cx.mouse().cursory);
        if x >= bounds.x && y >= bounds.y && x < bounds.x + bounds.w && y < bounds.y + bounds.h {
            cx.emit_custom(Event::new(WindowEvent::MouseEnter).origin(Entity::root()).direct(Entity::root()));
        }
    }

    fn tick(&mut self, cx: &mut EventContext) {
        Self::keep_hover_alive(cx);
        let now = Instant::now();
        let dt = (now - self.last_tick).as_secs_f32();
        self.last_tick = now;

        let reduction = self.telemetry.take_reduction_db().max(demo_reduction(self.seconds()));
        let glitch_changed = self.glitch.update(dt, reduction);

        let interference = self.telemetry.interference();
        let trace_changed = interference != *self.history.back().unwrap_or(&0.0) || self.history.iter().any(|v| *v != interference);
        self.history.pop_front();
        self.history.push_back(interference);

        let t = self.seconds();
        let slip: f32 = BANDS.iter().map(|b| b.slip.at(t)).sum();
        let phase_x = (self.telemetry.agitation_phase() * layout::AGITATION_TRACE.w / 2.0) as i32;
        if glitch_changed || trace_changed || slip != self.last_slip || phase_x != self.last_phase_x {
            cx.needs_redraw();
        }
        self.last_slip = slip;
        self.last_phase_x = phase_x;
    }

    fn seconds(&self) -> f32 {
        (self.last_tick - self.started).as_secs_f32()
    }

    // ---- Dragging ----------------------------------------------------------------

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

    fn mouse_down(&mut self, cx: &mut EventContext, x: f32, y: f32) {
        // A drag whose release never arrived: finish it before starting anything new.
        self.mouse_up(cx);
        let fine = cx.modifiers().contains(Modifiers::SHIFT);
        match self.hit(x, y) {
            Some(Target::Slider(i, zone)) => {
                let row = &self.rows[i];
                let p = row.param(zone);
                let width = row.slider.w;
                Self::begin(cx, p);
                let per_pixel = if zone == Zone::Value { 1.0 / width } else { 0.5 / width };
                let mut anchor_value = Self::value_of(p);
                if zone == Zone::Value && !fine {
                    anchor_value = ((x - row.slider.x) / width).clamp(0.0, 1.0);
                    Self::set(cx, p, anchor_value);
                }
                self.drag = Some(Drag { param: p, numerals: false, per_pixel, anchor: x, anchor_value, fine });
                cx.capture();
            }
            Some(Target::Numerals) => {
                let p = self.time_param();
                Self::begin(cx, p);
                self.drag = Some(Drag { param: p, numerals: true, per_pixel: 1.0 / NUMERALS_TRAVEL, anchor: y, anchor_value: Self::value_of(p), fine });
                cx.capture();
            }
            None => {}
        }
    }

    fn mouse_move(&mut self, cx: &mut EventContext, x: f32, y: f32) {
        if self.drag.is_some() && cx.mouse().left.state != MouseButtonState::Pressed {
            self.mouse_up(cx); // the button is up but we never heard the release
        }
        let fine = cx.modifiers().contains(Modifiers::SHIFT);
        if let Some(drag) = &mut self.drag {
            let at = if drag.numerals { y } else { x };
            if fine != drag.fine {
                // Re-anchor, so changing speed never makes the value jump.
                drag.anchor_value = Self::value_of(drag.param);
                drag.anchor = at;
                drag.fine = fine;
            }
            let moved = if drag.numerals { drag.anchor - at } else { at - drag.anchor };
            let v = drag.anchor_value + moved * drag.per_pixel * if fine { FINE } else { 1.0 };
            Self::set(cx, drag.param, v);
            cx.needs_redraw();
            return;
        }
        let hover = self.hit(x, y);
        if hover != self.hover {
            self.hover = hover;
            cx.needs_redraw();
        }
    }

    fn mouse_up(&mut self, cx: &mut EventContext) {
        if let Some(drag) = self.drag.take() {
            Self::end(cx, drag.param);
            cx.release();
        }
    }

    fn target_param(&self, target: Target) -> ParamPtr {
        match target {
            Target::Slider(i, zone) => self.rows[i].param(zone),
            Target::Numerals => self.time_param(),
        }
    }

    fn reset(&mut self, cx: &mut EventContext, x: f32, y: f32) {
        self.mouse_up(cx);
        if let Some(target) = self.hit(x, y) {
            let p = self.target_param(target);
            Self::begin(cx, p);
            Self::set(cx, p, unsafe { p.default_normalized_value() });
            Self::end(cx, p);
        }
    }

    fn scroll(&mut self, cx: &mut EventContext, x: f32, y: f32, notches: f32) {
        if let Some(target) = self.hit(x, y) {
            let p = self.target_param(target);
            let step = SCROLL_STEP * if cx.modifiers().contains(Modifiers::SHIFT) { FINE } else { 1.0 };
            Self::begin(cx, p);
            Self::set(cx, p, Self::value_of(p) + notches * step);
            Self::end(cx, p);
        }
    }

    // ---- Painting -------------------------------------------------------------------

    fn time_ms(&self) -> f32 {
        time_seconds(self.params.time.unmodulated_plain_value()) * 1000.0
    }

    fn numerals_text(&self, frame: &Frame) -> String {
        let mut digits: Vec<char> = layout::tibetan_digits(&format!("{:.0}", self.time_ms())).chars().collect();
        if let Some((at, digit)) = frame.scramble {
            if let Some(slot) = digits.get_mut(at) {
                *slot = char::from_u32(0x0F20 + digit).unwrap_or(*slot);
            }
        }
        digits.into_iter().collect()
    }

    fn paint_numerals(&self, p: &mut Painter, frame: &Frame, t: f32) {
        let text = self.numerals_text(frame);
        let mut size = layout::NUMERAL_SIZE;
        let width = p.text_width(&text, Face::Digits, size);
        if width > layout::NUMERAL_WIDTH {
            size *= layout::NUMERAL_WIDTH / width;
        }
        let area = layout::NUMERALS;
        if let Some((dx, dy, alpha)) = frame.ghost {
            p.canvas.save();
            p.canvas.intersect_scissor(area.x, area.y, area.w, area.h);
            p.text_on(layout::MARGIN + dx, layout::NUMERAL_BASELINE + dy, &text, Face::Digits, size, fade(RED, alpha));
            p.canvas.restore();
        }
        let mut top = area.y;
        for (band, extra) in BANDS.iter().zip(frame.band_shift) {
            let shift = band.offset + band.slip.at(t) + extra;
            p.canvas.save();
            p.canvas.intersect_scissor(area.x, top, area.w, band.height);
            let ink = color(if band.red { RED } else { INK });
            p.text_on(layout::MARGIN + shift, layout::NUMERAL_BASELINE, &text, Face::Digits, size, ink);
            p.canvas.restore();
            top += band.height;
        }
    }

    /// The hairline, the sends as short bars above and below it, and the handle.
    fn paint_slider(&self, p: &mut Painter, row: &Row, hovered: Option<Zone>) {
        let r = row.slider;
        let centre = r.centre_y();
        p.rect(r.x, centre, r.w, 1.0, color(TRACK));
        let position = row.normalized(Zone::Value) * r.w;
        let span = |zone: Zone| {
            let (from, to) = (position, position + row.send(zone) * r.w);
            let (left, right) = (from.min(to).max(0.0), from.max(to).min(r.w));
            (r.x + left, (right - left).max(0.0))
        };
        if row.agitation.is_some() {
            if hovered == Some(Zone::Agitation) {
                p.rect(r.x, centre - 8.0, r.w, 1.0, color(RULE));
            }
            let (x, w) = span(Zone::Agitation);
            if w >= 0.5 {
                p.rect(x, centre - 8.0, w, 2.0, color(RED));
            }
        }
        if row.interference.is_some() {
            if hovered == Some(Zone::Interference) {
                p.rect(r.x, centre + 7.0, r.w, 1.0, color(RULE));
            }
            let (x, w) = span(Zone::Interference);
            if w >= 0.5 {
                p.rect(x, centre + 7.0, w, 2.0, color(INK));
            }
        }
        let handle = if hovered == Some(Zone::Value) { 5.0 } else { 3.0 };
        p.rect(r.x + position - handle / 2.0, centre - 6.0, handle, 13.0, color(INK));
    }

    fn paint_row_label(&self, p: &mut Painter, row: &Row, corruption: Corruption) {
        let (x, y) = (row.label_x(), row.centre_y());
        match corruption {
            Corruption::Clean => p.text(x, y, row.name, Face::SemiBold, 15.0, color(INK), Align::Left),
            Corruption::Swapped(other) => p.text(x, y, self.rows[other].name, Face::SemiBold, 15.0, color(RED_LABEL), Align::Left),
            Corruption::Symbols(seed) => {
                for i in 0..4u32 {
                    let path = symbol(seed + i, x + i as f32 * 15.0, y - 9.0, 11.0, 18.0);
                    p.stroke(&path, color(RED_LABEL), 1.3);
                }
            }
        }
    }

    fn paint_grid(&self, p: &mut Painter, frame: &Frame) {
        for (i, row) in self.rows[..TIME].iter().enumerate() {
            p.rect(row.left, row.top, row.value_x() - row.left, 1.0, color(RULE));
            let path = symbol(row.seed, row.glyph_x() + 1.3, row.centre_y() - 7.5, 12.8, 15.0);
            p.stroke(&path, color(INK), 1.2);
            self.paint_row_label(p, row, frame.rows[i]);

            let hovered = self.zone_shown(i);
            self.paint_slider(p, row, hovered);
            match hovered {
                Some(zone @ (Zone::Agitation | Zone::Interference)) => {
                    let tint = if zone == Zone::Agitation { RED_LABEL } else { INK };
                    p.text(row.value_x(), row.centre_y(), &row.send_readout(zone), Face::Regular, 14.0, color(tint), Align::Right);
                }
                _ => {
                    let lines = row.readout();
                    let first = row.centre_y() - (lines.len() as f32 - 1.0) * 10.0;
                    for (n, line) in lines.iter().enumerate() {
                        p.text(row.value_x(), first + n as f32 * 20.0, line, Face::Regular, 14.0, color(INK), Align::Right);
                    }
                }
            }
        }
    }

    /// The zone to show for a row: what's being dragged, else what's under the pointer.
    fn zone_shown(&self, row: usize) -> Option<Zone> {
        if let Some(drag) = &self.drag {
            if drag.numerals {
                return None;
            }
            let r = &self.rows[row];
            return [Zone::Value, Zone::Agitation, Zone::Interference]
                .into_iter()
                .filter(|z| *z == Zone::Value || r.param(*z) != r.value)
                .find(|z| r.param(*z) == drag.param);
        }
        match self.hover {
            Some(Target::Slider(i, zone)) if i == row => Some(zone),
            _ => None,
        }
    }

    fn paint_time_row(&self, p: &mut Painter) {
        let row = &self.rows[TIME];
        p.text(layout::MARGIN, layout::TIME_ROW_Y, layout::TIME_LABEL, Face::SemiBold, 16.0, color(INK), Align::Left);
        let hovered = self.zone_shown(TIME);
        self.paint_slider(p, row, hovered);
        let right = WIDTH - layout::MARGIN;
        match hovered {
            Some(zone @ (Zone::Agitation | Zone::Interference)) => {
                let tint = if zone == Zone::Agitation { RED_LABEL } else { INK };
                p.text(right, layout::TIME_ROW_Y, &row.send_readout(zone), Face::Regular, 16.0, color(tint), Align::Right);
            }
            _ => p.text(right, layout::TIME_ROW_Y, layout::TIME_UNIT, Face::Regular, 16.0, color(INK), Align::Right),
        }
    }

    fn paint_traces(&self, p: &mut Painter) {
        let a = layout::AGITATION_TRACE;
        p.text(a.x, layout::AGITATION_LABEL_Y, layout::AGITATION_LABEL, Face::SemiBold, 16.0, color(RED_LABEL), Align::Left);
        let angle = self.params.agitation_angle.unmodulated_plain_value();
        let inset = Rect { x: a.x, y: a.y + 3.0, w: a.w, h: a.h - 6.0 };
        let points = (0..=a.w as usize / 2).map(|i| {
            let u = i as f32 * 2.0 / a.w;
            (u, 1.0 - shape((u * 2.0).fract(), angle))
        });
        p.line_through(inset, points, color(RED), 1.4);
        p.rect(a.x, a.y + a.h - 0.5, a.w, 1.0, color(RULE));
        let phase = self.telemetry.agitation_phase();
        let x = a.x + phase * a.w / 2.0;
        p.rect(x - 0.5, a.y, 1.0, a.h, fade(INK, 0.7));

        let b = layout::INTERFERENCE_TRACE;
        p.text(b.x, layout::INTERFERENCE_LABEL_Y, layout::INTERFERENCE_LABEL, Face::SemiBold, 16.0, color(INK), Align::Left);
        let count = self.history.len() as f32;
        let points = self.history.iter().enumerate().map(|(i, v)| (i as f32 / (count - 1.0), 0.5 - 0.48 * v.clamp(-1.0, 1.0)));
        p.line_through(b, points, color(INK), 1.1);
    }

    /// Everything but the background, so a tear can redraw all of it, displaced.
    fn paint_contents(&self, p: &mut Painter, frame: &Frame, t: f32) {
        p.text(layout::MARGIN, 23.0, layout::WORDMARK, Face::SemiBold, 17.0, color(INK), Align::Left);
        self.paint_numerals(p, frame, t);
        self.paint_time_row(p);
        self.paint_traces(p);
        self.paint_grid(p, frame);
    }

    fn paint_inverted_strip(&self, p: &mut Painter, y: f32, height: f32) {
        p.canvas.save();
        p.canvas.global_composite_blend_func(BlendFactor::OneMinusDstColor, BlendFactor::Zero);
        p.rect(0.0, y, WIDTH, height, Color::white());
        p.canvas.restore();
    }
}

/// A stand-in for real limiting, for checking the panel without a loud signal: the
/// standalone app with ANTIMEME_FAKE_LIMIT set to a depth in dB, or "sweep".
#[cfg(feature = "standalone")]
fn demo_reduction(t: f32) -> f32 {
    match std::env::var("ANTIMEME_FAKE_LIMIT").as_deref() {
        Ok("sweep") => {
            let phase = (t / 8.0).fract();
            14.0 * (1.0 - (2.0 * phase - 1.0).abs())
        }
        Ok(depth) => depth.parse().unwrap_or(0.0),
        Err(_) => 0.0,
    }
}

#[cfg(not(feature = "standalone"))]
fn demo_reduction(_t: f32) -> f32 {
    0.0
}

impl View for Panel {
    fn element(&self) -> Option<&'static str> {
        Some("antimeme-panel")
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
                WindowEvent::MouseLeave => self.hover = None,
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
        let t = self.seconds();
        let frame = &self.glitch.frame;
        let mut p = Painter::new(canvas);
        p.rect(0.0, 0.0, WIDTH, HEIGHT, color(GROUND));
        self.paint_contents(&mut p, frame, t);

        for tear in &frame.tears {
            p.canvas.save();
            p.canvas.intersect_scissor(0.0, tear.y, WIDTH, tear.height);
            p.rect(0.0, tear.y, WIDTH, tear.height, color(GROUND));
            p.canvas.translate(tear.shift, 0.0);
            self.paint_contents(&mut p, frame, t);
            p.canvas.restore();
        }
        if let Some((y, height)) = frame.inverted {
            self.paint_inverted_strip(&mut p, y, height);
        }
        canvas.restore();
    }
}
