//! Painting the panel with vector paths (femtovg), in the mockup's earth
//! tones. All coordinates are logical panel pixels; the caller has already
//! scaled the canvas. (The font handling is Abdication's.)

use std::cell::Cell;
use std::f32::consts::PI;

use nih_plug_vizia::vizia::prelude::Canvas;
use nih_plug_vizia::vizia::vg::{self, Align, Baseline, Color, LineCap, LineJoin, Paint, Path, Solidity};

use super::layout::{Part, Rect};
use crate::scope::Trace;

// ---- Palette ---------------------------------------------------------------

pub fn rgb(hex: u32) -> Color {
    Color::rgb((hex >> 16) as u8, (hex >> 8) as u8, hex as u8)
}
pub fn rgba(hex: u32, alpha: f32) -> Color {
    let mut c = rgb(hex);
    c.set_alphaf(alpha);
    c
}

pub const GROUND: u32 = 0x1a1612;
pub const CARD: u32 = 0x251f19;
pub const EDGE: u32 = 0x3b3229;
pub const INK: u32 = 0xebe0cc;
pub const DIM: u32 = 0xa69884;
pub const STONE: u32 = 0xcbbfa8;
pub const CAP: u32 = 0x30281f;
pub const CAP_EDGE: u32 = 0x4a3d30;
pub const PHOSPHOR: u32 = 0xeab95e;
pub const PHOSPHOR_DIM: u32 = 0xa1814b;
pub const GLASS: u32 = 0x120f0b;
pub const GLASS_LINE: u32 = 0x2a2118;

/// The free envelope and its rings: a cool slate, apart from the parts' earth tones.
pub const MOD: u32 = 0x93a8c9;
pub const TONE: u32 = 0xd9a24c;
pub const BODY: u32 = 0xd27a52;
pub const NOISE: u32 = 0x9fb27e;
/// Sample layers take their set's tint: thump, knock, spark.
pub const SET_TINTS: [u32; 3] = [0xc4835c, 0xb89c62, 0xe3c97f];

pub fn part_accent(part: Part, sample_set: usize) -> u32 {
    match part {
        Part::Tone => TONE,
        Part::Body => BODY,
        Part::Noise => NOISE,
        Part::Sample1 | Part::Sample2 => SET_TINTS[sample_set.min(2)],
    }
}

// ---- Fonts -------------------------------------------------------------------

static LABEL_FONT: &[u8] = include_bytes!("../../assets/fonts/BarlowCondensed-SemiBold.ttf");
static MONO_FONT: &[u8] = include_bytes!("../../assets/fonts/IBMPlexMono-Medium.ttf");

#[derive(Clone, Copy)]
pub enum Font {
    /// Barlow Condensed: titles and labels, drawn in capitals.
    Label,
    /// IBM Plex Mono: values and screen text.
    Mono,
}

#[derive(Clone, Copy)]
struct Fonts {
    label: vg::FontId,
    mono: vg::FontId,
}

thread_local! {
    /// Fonts are registered per canvas; the canvas address tells us when a
    /// reopened editor brought a new one.
    static FONTS: Cell<Option<(usize, Fonts)>> = const { Cell::new(None) };
}

fn fonts(canvas: &mut Canvas) -> Option<Fonts> {
    let key = canvas as *const Canvas as usize;
    if let Some((k, fonts)) = FONTS.get() {
        let mut probe = Paint::color(Color::white());
        probe.set_font(&[fonts.label]);
        if k == key && canvas.measure_text(0.0, 0.0, "A", &probe).is_ok() {
            return Some(fonts);
        }
    }
    let fonts = Fonts { label: canvas.add_font_mem(LABEL_FONT).ok()?, mono: canvas.add_font_mem(MONO_FONT).ok()? };
    FONTS.set(Some((key, fonts)));
    Some(fonts)
}

#[derive(Clone, Copy)]
pub struct Text {
    pub font: Font,
    pub size: f32,
    pub color: Color,
    pub align: Align,
    /// Letter spacing in em.
    pub spacing: f32,
}

impl Text {
    pub fn label(size: f32, color: u32) -> Self {
        Self { font: Font::Label, size, color: rgb(color), align: Align::Center, spacing: 0.08 }
    }
    pub fn mono(size: f32, color: u32) -> Self {
        Self { font: Font::Mono, size, color: rgb(color), align: Align::Center, spacing: 0.0 }
    }
    pub fn left(self) -> Self {
        Self { align: Align::Left, ..self }
    }
    pub fn right(self) -> Self {
        Self { align: Align::Right, ..self }
    }
    pub fn spaced(self, em: f32) -> Self {
        Self { spacing: em, ..self }
    }
}

fn rrect(r: Rect, radius: f32) -> Path {
    let mut p = Path::new();
    p.rounded_rect(r.x, r.y, r.w, r.h, radius);
    p
}

fn circle(x: f32, y: f32, r: f32) -> Path {
    let mut p = Path::new();
    p.circle(x, y, r);
    p
}

fn stroke(color: Color, width: f32) -> Paint {
    let mut p = Paint::color(color);
    p.set_line_width(width);
    p.set_line_cap(LineCap::Round);
    p.set_line_join(LineJoin::Round);
    p
}

pub struct Painter<'a> {
    pub canvas: &'a mut Canvas,
    fonts: Option<Fonts>,
}

impl<'a> Painter<'a> {
    pub fn new(canvas: &'a mut Canvas) -> Self {
        let fonts = fonts(canvas);
        Self { canvas, fonts }
    }

    pub fn fill_rect(&mut self, r: Rect, radius: f32, color: u32) {
        self.canvas.fill_path(&rrect(r, radius), &Paint::color(rgb(color)));
    }

    pub fn outline(&mut self, r: Rect, radius: f32, color: u32) {
        let r = r.inset(0.5, 0.5, 0.5, 0.5);
        self.canvas.stroke_path(&rrect(r, radius), &stroke(rgb(color), 1.0));
    }

    /// A filled box with a hairline edge.
    pub fn boxed(&mut self, r: Rect, radius: f32, fill: u32, edge: u32) {
        self.fill_rect(r, radius, fill);
        self.outline(r, radius, edge);
    }

    fn text_paint(&self, style: Text) -> Paint {
        let mut p = Paint::color(style.color);
        if let Some(f) = self.fonts {
            p.set_font(&[match style.font {
                Font::Label => f.label,
                Font::Mono => f.mono,
            }]);
        }
        p.set_font_size(style.size);
        p.set_letter_spacing(style.spacing * style.size);
        p.set_text_align(style.align);
        p.set_text_baseline(Baseline::Middle);
        p
    }

    fn cased(s: &str, style: Text) -> String {
        match style.font {
            Font::Label => s.to_uppercase(),
            Font::Mono => s.to_string(),
        }
    }

    /// Text vertically centred on `y`. Labels are drawn in capitals.
    pub fn text(&mut self, x: f32, y: f32, s: &str, style: Text) {
        let paint = self.text_paint(style);
        let _ = self.canvas.fill_text(x, y, Self::cased(s, style), &paint);
    }

    pub fn text_width(&self, s: &str, style: Text) -> f32 {
        let paint = self.text_paint(style);
        self.canvas.measure_text(0.0, 0.0, Self::cased(s, style), &paint).map(|m| m.width()).unwrap_or(0.0)
    }

    /// Text in `r`, shortened with an ellipsis if it doesn't fit.
    pub fn text_in(&mut self, r: Rect, s: &str, style: Text) {
        let mut s = s.to_string();
        if self.text_width(&s, style) > r.w {
            while !s.is_empty() && self.text_width(&format!("{s}…"), style) > r.w {
                s.pop();
            }
            s = format!("{}…", s.trim_end());
        }
        let x = match style.align {
            Align::Left => r.x,
            Align::Right => r.right(),
            Align::Center => r.x + r.w / 2.0,
        };
        self.text(x, r.y + r.h / 2.0, &s, style);
    }

    // ---- Panel furniture -----------------------------------------------------

    pub fn card(&mut self, r: Rect) {
        self.boxed(r, 6.0, CARD, EDGE);
    }

    /// A card's title: its on/off square (filled when on, hollow when off),
    /// the name, and an optional side label.
    pub fn card_title(&mut self, r: Rect, title: &str, accent: u32, side: &str, on: bool, hot: bool) {
        let cy = r.y + r.h / 2.0;
        let square = Rect::new(r.x, cy - 4.0, 8.0, 8.0);
        if hot {
            self.fill_rect(square.inset(-4.0, -4.0, -4.0, -4.0), 4.0, CAP);
        }
        if on {
            self.fill_rect(square, 2.0, accent);
        } else {
            let edge = square.inset(0.75, 0.75, 0.75, 0.75);
            self.canvas.stroke_path(&rrect(edge, 2.0), &stroke(rgb(accent), 1.5));
        }
        let style = Text::label(15.0, if on { accent } else { DIM }).left().spaced(0.14);
        self.text(r.x + 15.0, cy, title, style);
        if !side.is_empty() {
            let x = r.x + 15.0 + self.text_width(title, style) + 7.0;
            self.text(x, cy + 1.0, side, Text::label(11.0, DIM).left().spaced(0.12));
        }
    }

    /// The die: a rounded square showing `face` (1..6) pips.
    pub fn die(&mut self, r: Rect, face: u8, hot: bool) {
        let body = r.inset(1.0, 1.0, 1.0, 1.0);
        self.fill_rect(body, 4.0, if hot { CAP_EDGE } else { CAP });
        self.outline(body, 4.0, if hot { INK } else { DIM });
        let (cx, cy, d) = (body.x + body.w / 2.0, body.y + body.h / 2.0, body.w * 0.26);
        let pips: &[(f32, f32)] = match face {
            1 => &[(0.0, 0.0)],
            2 => &[(-1.0, -1.0), (1.0, 1.0)],
            3 => &[(-1.0, -1.0), (0.0, 0.0), (1.0, 1.0)],
            4 => &[(-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)],
            5 => &[(-1.0, -1.0), (1.0, -1.0), (0.0, 0.0), (-1.0, 1.0), (1.0, 1.0)],
            _ => &[(-1.0, -1.0), (1.0, -1.0), (-1.0, 0.0), (1.0, 0.0), (-1.0, 1.0), (1.0, 1.0)],
        };
        for (x, y) in pips {
            self.canvas.fill_path(&circle(cx + x * d, cy + y * d, 1.7), &Paint::color(rgb(INK)));
        }
    }

    /// The export button: an arrow out of a tray, and what it does.
    pub fn export_button(&mut self, r: Rect, label: &str, hot: bool) {
        self.boxed(r, 4.0, if hot { CAP } else { GROUND }, if hot { CAP_EDGE } else { EDGE });
        let (x, cy) = (r.x + 12.0, r.y + r.h / 2.0);
        let ink = stroke(rgb(if hot { PHOSPHOR } else { INK }), 1.5);
        let mut arrow = Path::new();
        arrow.move_to(x, cy - 6.0);
        arrow.line_to(x, cy + 2.0);
        arrow.move_to(x - 3.5, cy - 1.5);
        arrow.line_to(x, cy + 2.0);
        arrow.line_to(x + 3.5, cy - 1.5);
        arrow.move_to(x - 5.0, cy + 2.5);
        arrow.line_to(x - 5.0, cy + 5.5);
        arrow.line_to(x + 5.0, cy + 5.5);
        arrow.line_to(x + 5.0, cy + 2.5);
        self.canvas.stroke_path(&arrow, &ink);
        self.text(r.x + 24.0, cy + 0.5, label, Text::label(11.0, if hot { PHOSPHOR } else { DIM }).left().spaced(0.1));
    }

    /// The ⟲ that turns a card over (lit while its back shows).
    pub fn flip_button(&mut self, r: Rect, flipped: bool, hot: bool) {
        if flipped || hot {
            self.fill_rect(r, 4.0, if flipped { EDGE } else { CAP });
        }
        let (cx, cy, rad) = (r.x + r.w / 2.0, r.y + r.h / 2.0, 5.5);
        let ink = stroke(rgb(INK), 1.5);
        for (start, tip) in [(-PI * 0.95, -PI * 0.15), (PI * 0.05, PI * 0.85)] {
            let mut arc = Path::new();
            arc.arc(cx, cy, rad, start, tip, Solidity::Hole);
            self.canvas.stroke_path(&arc, &ink);
            // An arrowhead at the arc's end: two barbs swept back from its
            // direction of travel (the tangent, as the angle grows).
            let (x, y) = (cx + rad * tip.cos(), cy + rad * tip.sin());
            let travel = tip + PI / 2.0;
            let mut head = Path::new();
            for barb in [travel + PI - 0.6, travel + PI + 0.6] {
                head.move_to(x, y);
                head.line_to(x + 3.0 * barb.cos(), y + 3.0 * barb.sin());
            }
            self.canvas.stroke_path(&head, &ink);
        }
    }

    /// A part's scope: the latest hit, frozen. Columns of (min, max).
    pub fn scope(&mut self, r: Rect, trace: &Trace, accent: u32) {
        self.fill_rect(r, 3.0, GROUND);
        let mid = r.y + r.h / 2.0;
        let mut line = Path::new();
        line.move_to(r.x, mid);
        line.line_to(r.right(), mid);
        self.canvas.stroke_path(&line, &stroke(rgb(0x2c251e), 1.0));
        if trace.columns.is_empty() {
            return;
        }
        let half = r.h / 2.0 - 2.5;
        let n = trace.columns.len() as f32;
        let mut p = Path::new();
        for (i, (lo, hi)) in trace.columns.iter().enumerate() {
            let x = r.x + (i as f32 + 0.5) * r.w / n;
            if i == 0 {
                p.move_to(x, mid - hi * half);
            } else {
                p.line_to(x, mid - hi * half);
            }
            if hi - lo > 0.02 {
                p.line_to(x, mid - lo * half);
            }
        }
        self.canvas.stroke_path(&p, &stroke(rgb(accent), 1.0));
    }

    /// The amber-on-brown screen's glass.
    pub fn glass(&mut self, r: Rect) {
        self.boxed(r, 6.0, GLASS, EDGE);
    }

    /// A curve on the screen: a faint fill under a bright line. `ys` are
    /// 0..1 from the bottom of `r`.
    pub fn screen_curve(&mut self, r: Rect, ys: &[f32]) {
        self.curve(r, ys, PHOSPHOR, GLASS_LINE);
    }

    /// A curve in `color` over a few grid lines in `grid`.
    pub fn curve(&mut self, r: Rect, ys: &[f32], color: u32, grid: u32) {
        for i in 1..4 {
            let y = r.y + r.h * i as f32 / 4.0;
            let mut line = Path::new();
            line.move_to(r.x, y);
            line.line_to(r.right(), y);
            self.canvas.stroke_path(&line, &stroke(rgb(grid), 1.0));
        }
        if ys.len() < 2 {
            return;
        }
        let point = |i: usize, y: f32| (r.x + r.w * i as f32 / (ys.len() - 1) as f32, r.bottom() - 2.0 - y.clamp(0.0, 1.0) * (r.h - 6.0));
        let mut line = Path::new();
        for (i, y) in ys.iter().enumerate() {
            let (x, y) = point(i, *y);
            if i == 0 { line.move_to(x, y) } else { line.line_to(x, y) }
        }
        let mut area = line.clone();
        area.line_to(r.right(), r.bottom());
        area.line_to(r.x, r.bottom());
        area.close();
        self.canvas.fill_path(&area, &Paint::color(rgba(color, 0.13)));
        self.canvas.stroke_path(&line, &stroke(rgb(color), 1.5));
    }

    // ---- Controls ------------------------------------------------------------

    /// A knob with its label and value below, and its envelope ring if it
    /// has one. `focused` lights its label and cell.
    #[allow(clippy::too_many_arguments)]
    pub fn knob(&mut self, cell: Rect, size: f32, value: f32, bipolar: bool, label: &str, text: &str,
                accent: u32, focused: bool, hot: bool, ring: Option<Ring>) {
        if focused {
            self.fill_rect(cell, 4.0, CAP);
        }
        let KnobGeometry { cx, cy, r, ring_r } = KnobGeometry::of(cell, size);
        let width = if size < 32.0 { 3.0 } else { 3.5 };
        let angle = |t: f32| (135.0 + 270.0 * t) * PI / 180.0;
        let arc = |t0: f32, t1: f32| {
            let mut p = Path::new();
            p.arc(cx, cy, r, angle(t0), angle(t1), Solidity::Hole);
            p
        };
        self.canvas.stroke_path(&arc(0.0, 1.0), &stroke(rgb(EDGE), width));
        let (from, to) = if bipolar { (value.min(0.5), value.max(0.5)) } else { (0.0, value) };
        if to - from > 0.004 {
            let color = if hot { rgb(lighten(accent)) } else { rgb(accent) };
            self.canvas.stroke_path(&arc(from, to), &stroke(color, width));
        }
        if let Some(ring) = ring {
            self.ring(cx, cy, ring_r, value, ring, angle);
        }
        let cap = size / 2.0 * 0.5;
        self.canvas.fill_path(&circle(cx, cy, cap), &Paint::color(rgb(CAP)));
        self.canvas.stroke_path(&circle(cx, cy, cap), &stroke(rgb(CAP_EDGE), 1.0));
        let a = angle(value);
        let mut pointer = Path::new();
        pointer.move_to(cx + cap * 0.35 * a.cos(), cy + cap * 0.35 * a.sin());
        pointer.line_to(cx + (r - 1.0) * a.cos(), cy + (r - 1.0) * a.sin());
        self.canvas.stroke_path(&pointer, &stroke(rgb(INK), 2.0));

        let small = size < 32.0;
        let y = cell.y + size + if small { 6.0 } else { 7.0 };
        let label_color = if focused { accent } else { DIM };
        self.text(cx, y, label, Text::label(if small { 11.0 } else { 12.0 }, label_color).spaced(0.06));
        let value_style = Text::mono(if small { 9.5 } else { 10.0 }, INK);
        self.text_in(Rect::new(cell.x - 2.0, y + 6.0, cell.w + 4.0, 12.0), text, value_style);
    }

    /// An envelope ring outside a knob's arc: from the value to where the
    /// envelope's peak takes it. Hovered, its whole track shows (where to grab).
    fn ring(&mut self, cx: f32, cy: f32, radius: f32, value: f32, ring: Ring, angle: impl Fn(f32) -> f32) {
        let arc = |t0: f32, t1: f32| {
            let mut p = Path::new();
            p.arc(cx, cy, radius, angle(t0.min(t1)), angle(t0.max(t1)), Solidity::Hole);
            p
        };
        if ring.hot {
            self.canvas.stroke_path(&arc(0.0, 1.0), &stroke(rgba(MOD, 0.28), 2.0));
        }
        let to = (value + ring.depth).clamp(0.0, 1.0);
        if (to - value).abs() > 0.004 {
            self.canvas.stroke_path(&arc(value, to), &stroke(rgb(if ring.hot { lighten(MOD) } else { MOD }), 2.5));
            let a = angle(to);
            self.canvas.fill_path(&circle(cx + radius * a.cos(), cy + radius * a.sin(), 2.2), &Paint::color(rgb(MOD)));
        }
    }

    /// A level bar: label, bar, value; Volume's also shows the output meter
    /// (`meter`, on the bar's own scale) behind its setting.
    #[allow(clippy::too_many_arguments)]
    pub fn level(&mut self, bar: Rect, label: &str, value: f32, text: &str, accent: u32, hot: bool, meter: Option<f32>) {
        let cy = bar.y + bar.h / 2.0;
        self.text(bar.x - 8.0, cy, label, Text::label(11.0, DIM).right().spaced(0.1));
        let track = Rect::new(bar.x, cy - 3.0, bar.w, 6.0);
        self.fill_rect(track, 3.0, GROUND);
        let color = if hot { lighten(accent) } else { accent };
        match meter {
            None if value > 0.002 => {
                self.fill_rect(Rect::new(track.x, track.y, track.w * value.clamp(0.0, 1.0), track.h), 3.0, color);
            }
            None => {}
            Some(meter) => {
                // The setting as a faint fill and a handle; the meter bright in front.
                let x = track.x + track.w * value.clamp(0.0, 1.0);
                self.fill_rect(Rect::new(track.x, track.y, x - track.x, track.h), 3.0, CAP_EDGE);
                if meter > 0.002 {
                    self.fill_rect(Rect::new(track.x, track.y + 1.0, track.w * meter.clamp(0.0, 1.0), track.h - 2.0), 2.0, PHOSPHOR);
                }
                self.fill_rect(Rect::new(x - 1.5, bar.y + 2.0, 3.0, bar.h - 4.0), 1.5, color);
            }
        }
        self.text_in(Rect::new(bar.right() + 4.0, bar.y, 50.0, bar.h), text, Text::mono(10.0, INK).right());
    }

    /// A small value box (Generator, Drive type, Comp timing).
    pub fn chip(&mut self, r: Rect, text: &str, color: u32, hot: bool) {
        self.boxed(r, 4.0, GROUND, if hot { CAP_EDGE } else { EDGE });
        self.text_in(r.inset(4.0, 0.0, 4.0, 0.0), text, Text::mono(10.5, color));
    }

    /// One of a menu's values as a button; filled with `tint` when chosen.
    pub fn choice(&mut self, r: Rect, label: &str, on: bool, tint: u32, hot: bool) {
        if on {
            self.fill_rect(r, 4.0, tint);
        } else {
            self.boxed(r, 4.0, GROUND, if hot { CAP_EDGE } else { EDGE });
        }
        self.text(r.x + r.w / 2.0, r.y + r.h / 2.0, label, Text::label(11.0, if on { GROUND } else { DIM }).spaced(0.1));
    }

    /// ‹ name  n/m ›
    pub fn stepper(&mut self, r: Rect, name: &str, count: &str, hot: bool) {
        self.boxed(r, 4.0, GROUND, if hot { CAP_EDGE } else { EDGE });
        let cy = r.y + r.h / 2.0;
        let arrows = Text::mono(13.0, DIM);
        self.text(r.x + 8.0, cy, "‹", arrows);
        self.text(r.right() - 8.0, cy, "›", arrows);
        let count_w = if count.is_empty() { 0.0 } else { self.text_width(count, Text::mono(9.5, DIM)) + 4.0 };
        if !count.is_empty() {
            self.text(r.right() - 17.0, cy, count, Text::mono(9.5, DIM).right());
        }
        self.text_in(Rect::new(r.x + 17.0, r.y, r.w - 34.0 - count_w, r.h), name, Text::mono(10.0, INK).left());
    }

    /// A latching switch: name on the left, state on the right.
    pub fn toggle(&mut self, r: Rect, label: &str, state: &str, on: bool, accent: u32, hot: bool) {
        self.boxed(r, 4.0, GROUND, if hot { CAP_EDGE } else { EDGE });
        let cy = r.y + r.h / 2.0;
        self.text(r.x + 8.0, cy, label, Text::label(11.0, DIM).left().spaced(0.1));
        self.text_in(Rect::new(r.x + 70.0, r.y, r.w - 78.0, r.h), state, Text::mono(10.0, if on { accent } else { INK }).right());
    }

    /// A popup list laid out on `grid`.
    pub fn menu(&mut self, r: Rect, items: &[String], grid: MenuGrid, selected: Option<usize>, hover: Option<usize>, accent: u32) {
        let shadow = Paint::box_gradient(r.x, r.y + 6.0, r.w, r.h, 6.0, 14.0, rgba(0, 0.7), rgba(0, 0.0));
        self.canvas.fill_path(&rrect(r.inset(-14.0, -8.0, -14.0, -20.0), 18.0), &shadow);
        self.boxed(r, 6.0, 0x2a231c, CAP_EDGE);
        for (i, item) in items.iter().enumerate() {
            let cell = grid.cell(r, i);
            if Some(i) == hover || Some(i) == selected {
                let alpha = if Some(i) == hover { 0.22 } else { 0.12 };
                self.canvas.fill_path(&rrect(cell.inset(2.0, 1.0, 2.0, 1.0), 3.0), &Paint::color(rgba(accent, alpha)));
            }
            let color = if Some(i) == selected { accent } else { INK };
            self.text_in(cell.inset(8.0, 0.0, 4.0, 0.0), item, Text::mono(11.0, color).left());
        }
    }
}

/// A knob's envelope ring: its depth (-1..1 of the knob's travel, as shown)
/// and whether the mouse is on it.
#[derive(Clone, Copy)]
pub struct Ring {
    pub depth: f32,
    pub hot: bool,
}

/// Where a knob in `cell` sits: its centre, its value arc's radius and its
/// ring's (drawing and the mouse both go by this).
pub struct KnobGeometry {
    pub cx: f32,
    pub cy: f32,
    pub r: f32,
    pub ring_r: f32,
}

impl KnobGeometry {
    pub fn of(cell: Rect, size: f32) -> Self {
        let r = size / 2.0 * 0.75;
        Self { cx: cell.x + cell.w / 2.0, cy: cell.y + size / 2.0 + 1.0, r, ring_r: r + size * 0.1 + 0.5 }
    }

    /// Whether (x, y) is on the ring: outside the value arc, within reach of the ring.
    pub fn on_ring(&self, x: f32, y: f32) -> bool {
        let d = ((x - self.cx).powi(2) + (y - self.cy).powi(2)).sqrt();
        d >= self.r + 2.0 && d <= self.ring_r + 4.0
    }
}

/// A brighter version of a colour, for hovered controls.
fn lighten(hex: u32) -> u32 {
    let ch = |shift: u32| {
        let c = (hex >> shift) & 0xff;
        (c + (255 - c) * 3 / 10) << shift
    };
    ch(16) | ch(8) | ch(0)
}

/// How a popup menu's items are arranged: columns of `rows` items.
#[derive(Clone, Copy)]
pub struct MenuGrid {
    pub rows: usize,
    pub col_w: f32,
    pub row_h: f32,
}

impl MenuGrid {
    /// Item `i`'s cell inside the menu rectangle `r`.
    pub fn cell(&self, r: Rect, i: usize) -> Rect {
        let (col, row) = (i / self.rows, i % self.rows);
        Rect::new(r.x + 6.0 + col as f32 * self.col_w, r.y + 6.0 + row as f32 * self.row_h, self.col_w, self.row_h)
    }
}
