//! Painting the panel's parts with vector paths (femtovg). Colours and
//! proportions follow the design mockup. All coordinates are logical panel
//! pixels; the caller has already scaled the canvas.

use std::cell::Cell;
use std::f32::consts::PI;

use nih_plug_vizia::vizia::prelude::Canvas;
use nih_plug_vizia::vizia::vg::{self, Align, Baseline, Color, LineCap, Paint, Path, Solidity};

use super::layout::Rect;

// ---- Palette ---------------------------------------------------------------

pub fn rgb(hex: u32) -> Color {
    Color::rgb((hex >> 16) as u8, (hex >> 8) as u8, hex as u8)
}
pub fn rgba(hex: u32, alpha: f32) -> Color {
    let mut c = rgb(hex);
    c.set_alphaf(alpha);
    c
}

pub const AMBER: u32 = 0xffb547;
pub const AMBER_DIM: u32 = 0x9b6e2c;
pub const ARC: u32 = 0xf0a43c;
pub const VALUE_TEXT: u32 = 0xe9a54a;
pub const LABEL: u32 = 0xbdb7aa;
pub const TITLE: u32 = 0xd8d2c4;
pub const MUTED: u32 = 0x8d877a;
pub const WELL: u32 = 0x17181b;
pub const LED_ON: u32 = 0xff5a36;
pub const LED_OFF: u32 = 0x3d1f18;
pub const SCREEN_LINE: u32 = 0x3b2a12;
/// The free modulators and their rings: cool colours, apart from the amber.
pub const ENV_COLOR: u32 = 0x8fb3e0;
pub const LFO_COLOR: u32 = 0xd98bc9;

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
    let fonts = Fonts {
        label: canvas.add_font_mem(LABEL_FONT).ok()?,
        mono: canvas.add_font_mem(MONO_FONT).ok()?,
    };
    FONTS.set(Some((key, fonts)));
    Some(fonts)
}

// ---- The painter -------------------------------------------------------------

pub struct Painter<'a> {
    pub canvas: &'a mut Canvas,
    fonts: Option<Fonts>,
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
        Self { font: Font::Label, size, color: rgb(color), align: Align::Center, spacing: 0.1 }
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

fn vgrad(r: Rect, top: Color, bottom: Color) -> Paint {
    Paint::linear_gradient(r.x, r.y, r.x, r.bottom(), top, bottom)
}

/// A path through `points` (0..1 each way, y up) inside `r`.
fn polyline(r: Rect, points: &[(f32, f32)]) -> Path {
    let mut path = Path::new();
    for (k, (u, v)) in points.iter().enumerate() {
        let (x, y) = (r.x + u * r.w, r.bottom() - v * r.h);
        if k == 0 { path.move_to(x, y) } else { path.line_to(x, y) }
    }
    path
}

fn stroke(color: Color, width: f32) -> Paint {
    let mut p = Paint::color(color);
    p.set_line_width(width);
    p
}

impl<'a> Painter<'a> {
    pub fn new(canvas: &'a mut Canvas) -> Self {
        let fonts = fonts(canvas);
        Self { canvas, fonts }
    }

    fn fill(&mut self, path: &Path, paint: &Paint) {
        self.canvas.fill_path(path, paint);
    }
    fn stroke(&mut self, path: &Path, paint: &Paint) {
        self.canvas.stroke_path(path, paint);
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

    /// Text vertically centred on `y`. Labels are drawn in capitals.
    pub fn text(&mut self, x: f32, y: f32, s: &str, style: Text) {
        let paint = self.text_paint(style);
        let s = match style.font {
            Font::Label => s.to_uppercase(),
            Font::Mono => s.to_string(),
        };
        let _ = self.canvas.fill_text(x, y, s, &paint);
    }

    pub fn text_width(&self, s: &str, style: Text) -> f32 {
        let paint = self.text_paint(style);
        let s = match style.font {
            Font::Label => s.to_uppercase(),
            Font::Mono => s.to_string(),
        };
        self.canvas.measure_text(0.0, 0.0, s, &paint).map(|m| m.width()).unwrap_or(0.0)
    }

    /// Text clipped to `r` (for values that might not fit).
    pub fn text_in(&mut self, r: Rect, s: &str, style: Text) {
        self.canvas.save();
        self.canvas.intersect_scissor(r.x, r.y, r.w, r.h);
        let x = match style.align {
            Align::Left => r.x,
            Align::Right => r.right(),
            Align::Center => r.x + r.w / 2.0,
        };
        self.text(x, r.y + r.h / 2.0, s, style);
        self.canvas.restore();
    }

    /// A soft drop shadow under a rounded box.
    fn shadow(&mut self, r: Rect, radius: f32, blur: f32, dy: f32, alpha: f32) {
        let s = Rect::new(r.x, r.y + dy, r.w, r.h);
        let paint = Paint::box_gradient(s.x, s.y, s.w, s.h, radius, blur, rgba(0, alpha), rgba(0, 0.0));
        self.fill(&rrect(s.inset(-blur, -blur, -blur, -blur), radius + blur), &paint);
    }

    /// A recessed well: dark fill with an inner shadow and a faint lip.
    fn well(&mut self, r: Rect, radius: f32, fill: Color) {
        self.fill(&rrect(r.inset(0.0, 1.0, 0.0, -1.0), radius), &Paint::color(rgba(0xffffff, 0.06)));
        self.fill(&rrect(r, radius), &Paint::color(fill));
        let inner = Paint::box_gradient(r.x, r.y + 2.0, r.w, r.h, radius, 4.0, rgba(0, 0.0), rgba(0, 0.85));
        self.fill(&rrect(r, radius), &inner);
    }

    // ---- Panel furniture -----------------------------------------------------

    pub fn background(&mut self, w: f32, h: f32) {
        self.fill(&rrect(Rect::new(0.0, 0.0, w, h), 0.0), &Paint::color(rgb(0x111111)));
    }

    /// A walnut end cheek.
    pub fn cheek(&mut self, r: Rect) {
        let wood = Paint::linear_gradient_stops(
            r.x, 0.0, r.right(), 0.0,
            [(0.0, rgb(0x2e1a0e)), (0.45, rgb(0x6a4126)), (1.0, rgb(0x4c2d19))],
        );
        self.fill(&rrect(r, 0.0), &wood);
        // Grain: slightly slanted dark streaks, every few pixels.
        let grain = stroke(rgba(0, 0.18), 1.6);
        let mut y = r.y - 20.0;
        let mut i = 0;
        while y < r.bottom() + 20.0 {
            let mut p = Path::new();
            p.move_to(r.x, y);
            p.line_to(r.right(), y + r.w * 0.07);
            self.stroke(&p, &grain);
            y += 7.0 + (i % 3) as f32 * 2.0;
            i += 1;
        }
        let edge = Paint::linear_gradient(r.right() - 5.0, 0.0, r.right(), 0.0, rgba(0, 0.0), rgba(0, 0.5));
        self.fill(&rrect(Rect::new(r.right() - 5.0, r.y, 5.0, r.h), 0.0), &edge);
    }

    /// The brushed-metal faceplate between the cheeks.
    pub fn faceplate(&mut self, r: Rect) {
        self.fill(&rrect(r, 0.0), &vgrad(r, rgb(0x2a2b2f), rgb(0x1d1e21)));
        let brush = stroke(rgba(0xffffff, 0.014), 1.0);
        let mut p = Path::new();
        let mut x = r.x + 0.5;
        while x < r.right() {
            p.move_to(x, r.y);
            p.line_to(x, r.bottom());
            x += 3.0;
        }
        self.stroke(&p, &brush);
        for (x, y) in [(r.x + 10.5, r.y + 10.5), (r.right() - 10.5, r.y + 10.5),
                       (r.x + 10.5, r.bottom() - 10.5), (r.right() - 10.5, r.bottom() - 10.5)] {
            self.screw(x, y);
        }
    }

    fn screw(&mut self, x: f32, y: f32) {
        self.fill(&circle(x, y + 1.0, 4.5), &Paint::color(rgba(0, 0.9)));
        let head = Paint::radial_gradient(x - 1.2, y - 1.4, 0.5, 4.5, rgb(0x9a9ca1), rgb(0x3b3d41));
        self.fill(&circle(x, y, 4.5), &head);
        let mut slot = Path::new();
        slot.move_to(x - 3.0, y + 1.5);
        slot.line_to(x + 3.0, y - 1.5);
        self.stroke(&slot, &stroke(rgba(0x1a1a1c, 0.8), 1.0));
    }

    /// A raised module plate with its title.
    pub fn plate(&mut self, r: Rect, title: &str) {
        self.raised(r, 10.0, 0x3a3c41, 0x2b2d31);
        if !title.is_empty() {
            let style = Text { spacing: 0.14, ..Text::label(13.0, TITLE) }.left();
            self.text(r.x + 14.0, r.y + 23.0, title, style);
        }
    }

    /// A raised rounded box: shadow, gradient, top highlight, bottom lip.
    pub fn raised(&mut self, r: Rect, radius: f32, top: u32, bottom: u32) {
        self.shadow(r, radius, 8.0, 3.0, 0.55);
        self.fill(&rrect(r, radius), &Paint::color(rgba(0, 0.45)));
        self.fill(&rrect(r.inset(0.0, 0.0, 0.0, 2.0), radius), &vgrad(r, rgb(top), rgb(bottom)));
        let highlight = Paint::linear_gradient(r.x, r.y, r.x, r.y + 3.0, rgba(0xffffff, 0.09), rgba(0xffffff, 0.0));
        self.fill(&rrect(r.inset(0.0, 0.0, 0.0, 2.0), radius), &highlight);
    }

    /// An amber display: recessed glass with a glow in the middle.
    pub fn screen(&mut self, r: Rect) {
        self.fill(&rrect(r.inset(-1.0, -1.0, -1.0, -1.0), 7.0), &Paint::color(rgb(0x000000)));
        let (cx, cy) = (r.x + r.w / 2.0, r.y + r.h * 0.4);
        let glass = Paint::radial_gradient(cx, cy, 0.0, r.w.max(r.h) * 0.6, rgb(0x241a0c), rgb(0x120d06));
        self.fill(&rrect(r, 6.0), &glass);
        let inner = Paint::box_gradient(r.x, r.y + 2.0, r.w, r.h, 6.0, 6.0, rgba(0, 0.0), rgba(0, 0.9));
        self.fill(&rrect(r, 6.0), &inner);
    }

    /// A trace on a screen: a soft glow under a thin bright line.
    pub fn trace(&mut self, path: &Path, alpha: f32) {
        let mut glow = stroke(rgba(AMBER, 0.25 * alpha), 5.0);
        glow.set_line_cap(LineCap::Round);
        self.stroke(path, &glow);
        let mut line = stroke(rgba(AMBER, alpha), 1.6);
        line.set_line_cap(LineCap::Round);
        self.stroke(path, &line);
    }

    pub fn screen_line(&mut self, x0: f32, y0: f32, x1: f32, y1: f32) {
        let mut p = Path::new();
        p.move_to(x0, y0);
        p.line_to(x1, y1);
        self.stroke(&p, &stroke(rgb(SCREEN_LINE), 1.0));
    }

    // ---- Controls ------------------------------------------------------------

    /// A knob in its cell. `value` is normalized; `hot` brightens the arc
    /// while it's hovered or dragged.
    #[allow(clippy::too_many_arguments)]
    pub fn knob(&mut self, cell: Rect, size: f32, value: f32, bipolar: bool, inline: bool,
                label: &str, text: &str, hot: bool, marks: Marks) {
        let KnobGeometry { cx, cy, r } = KnobGeometry::of(cell, size, inline);
        let angle = |t: f32| (135.0 + 270.0 * t) * PI / 180.0;
        if marks.selected {
            self.dotted_circle(cx, cy, r + 6.5);
        }
        if let Some(moved) = marks.moved.filter(|m| (m - value).abs() > 0.004) {
            // Where the newest voice's modulators have it: an arc from the
            // knob's own value, ending in a dot.
            let radius = r + 6.5;
            let mut p = Path::new();
            p.arc(cx, cy, radius, angle(value.min(moved)), angle(value.max(moved)), Solidity::Hole);
            let mut line = stroke(rgb(LFO_COLOR), 2.0);
            line.set_line_cap(LineCap::Round);
            self.stroke(&p, &line);
            let a = angle(moved);
            self.fill(&circle(cx + radius * a.cos(), cy + radius * a.sin(), 2.6), &Paint::color(rgb(LFO_COLOR)));
        }
        let arc = |t0: f32, t1: f32| {
            let mut p = Path::new();
            p.arc(cx, cy, r, angle(t0), angle(t1), Solidity::Hole);
            p
        };
        let mut track = stroke(rgb(0x0e0f11), 3.5);
        track.set_line_cap(LineCap::Round);
        self.stroke(&arc(0.0, 1.0), &track);
        let (from, to) = if bipolar { (value.min(0.5), value.max(0.5)) } else { (0.0, value) };
        if to - from > 0.004 {
            let mut lit = stroke(rgb(if hot { 0xffcf73 } else { ARC }), 3.5);
            lit.set_line_cap(LineCap::Round);
            self.stroke(&arc(from, to), &lit);
        }

        let body_r = size / 2.0 - 8.0;
        let cap_r = body_r * 0.72;
        self.fill(&circle(cx, cy + 2.0, body_r + 1.0), &Paint::color(rgba(0, 0.6)));
        let body = Paint::radial_gradient_stops(
            cx - body_r * 0.24, cy - body_r * 0.4, 0.0, body_r * 1.6,
            [(0.0, rgb(0x6e7177)), (0.5, rgb(0x2e3034)), (1.0, rgb(0x141517))],
        );
        self.fill(&circle(cx, cy, body_r), &body);
        self.stroke(&circle(cx, cy, body_r), &stroke(rgb(0x08080a), 1.0));
        let cap = Paint::radial_gradient(cx - cap_r * 0.2, cy - cap_r * 0.3, 0.0, cap_r * 1.4, rgb(0x4a4d53), rgb(0x1c1d20));
        self.fill(&circle(cx, cy, cap_r), &cap);
        self.stroke(&circle(cx, cy, cap_r), &stroke(rgba(0xffffff, 0.10), 1.0));
        let a = angle(value);
        let mut pointer = Path::new();
        pointer.move_to(cx + cap_r * 0.25 * a.cos(), cy + cap_r * 0.25 * a.sin());
        pointer.line_to(cx + (body_r - 2.0) * a.cos(), cy + (body_r - 2.0) * a.sin());
        let mut line = stroke(rgb(0xf3eee2), 2.2);
        line.set_line_cap(LineCap::Round);
        self.stroke(&pointer, &line);

        if inline {
            let x = cx - size / 2.0 - 10.0;
            self.text(x, cy - 7.0, label, Text::label(11.0, LABEL).right());
            self.text(x, cy + 8.0, text, Text::mono(11.0, VALUE_TEXT).right());
        } else {
            let y = cell.y + size + 9.0;
            self.text(cx, y, label, Text::label(10.0, LABEL));
            self.text_in(Rect::new(cell.x - 6.0, y + 7.0, cell.w + 12.0, 14.0), text, Text::mono(10.0, VALUE_TEXT));
        }
    }

    /// The Modulations panel's mark: a ring of dots.
    fn dotted_circle(&mut self, cx: f32, cy: f32, radius: f32) {
        let dots = (radius * 1.1) as usize;
        let mut p = Path::new();
        for k in 0..dots {
            let a = 2.0 * PI * k as f32 / dots as f32;
            p.circle(cx + radius * a.cos(), cy + radius * a.sin(), 0.9);
        }
        self.fill(&p, &Paint::color(rgb(AMBER)));
    }

    /// The same round a fader: dots along a rounded box.
    fn dotted_box(&mut self, r: Rect) {
        let mut p = Path::new();
        let step = 5.0;
        let mut dot = |x: f32, y: f32| p.circle(x, y, 0.9);
        let (nx, ny) = ((r.w / step) as usize, (r.h / step) as usize);
        for i in 0..=nx {
            let x = r.x + r.w * i as f32 / nx as f32;
            dot(x, r.y);
            dot(x, r.bottom());
        }
        for j in 1..ny {
            let y = r.y + r.h * j as f32 / ny as f32;
            dot(r.x, y);
            dot(r.right(), y);
        }
        self.fill(&p, &Paint::color(rgb(AMBER)));
    }

    /// The part of a fader cell the cap travels along.
    pub fn fader_track(cell: Rect) -> Rect {
        Rect::new(cell.x + 7.0, cell.y + 4.0, 30.0, cell.h - 36.0)
    }

    pub fn fader(&mut self, cell: Rect, value: f32, label: &str, text: &str, hot: bool, marks: Marks) {
        let t = Self::fader_track(cell);
        let travel = t.h - 16.0;
        let cap_y = t.y + (1.0 - value) * travel;
        if marks.selected {
            self.dotted_box(t.inset(-4.0, -4.0, -4.0, -4.0));
        }
        if let Some(moved) = marks.moved.filter(|m| (m - value).abs() > 0.004) {
            // Beside the slot: from the fader's value to where the newest
            // voice's modulators have it.
            let x = t.right() + 1.0;
            let y = |v: f32| t.y + 8.0 + (1.0 - v) * travel;
            let mut p = Path::new();
            p.move_to(x, y(value));
            p.line_to(x, y(moved));
            let mut line = stroke(rgb(LFO_COLOR), 2.0);
            line.set_line_cap(LineCap::Round);
            self.stroke(&p, &line);
            self.fill(&circle(x, y(moved), 2.6), &Paint::color(rgb(LFO_COLOR)));
        }
        // Tick marks, the slot, the lit part of the slot.
        let mut ticks = Path::new();
        for i in 0..=10 {
            let y = t.y + 8.0 + travel * i as f32 / 10.0;
            ticks.move_to(t.x + 6.0, y);
            ticks.line_to(t.x + 9.0, y);
        }
        self.stroke(&ticks, &stroke(rgb(0x57595e), 1.0));
        self.well(Rect::new(t.x + 13.0, t.y, 4.0, t.h), 2.0, rgb(0x0d0e10));
        let lit = Rect::new(t.x + 14.0, cap_y + 8.0, 2.0, t.bottom() - cap_y - 8.0);
        if lit.h > 0.5 {
            self.fill(&rrect(lit, 1.0), &Paint::color(rgb(if hot { 0xffcf73 } else { ARC })));
        }
        // The cap.
        let cap = Rect::new(t.x, cap_y, 30.0, 16.0);
        self.shadow(cap, 3.0, 3.0, 2.0, 0.75);
        let paint = Paint::linear_gradient_stops(
            cap.x, cap.y, cap.x, cap.bottom(),
            [(0.0, rgb(0x62656b)), (0.45, rgb(0x3a3c41)), (1.0, rgb(0x24262a))],
        );
        self.fill(&rrect(cap, 3.0), &paint);
        self.fill(&rrect(Rect::new(cap.x + 4.0, cap.y + 7.0, 22.0, 2.0), 0.0), &Paint::color(rgb(0xf3eee2)));

        let y = t.bottom() + 11.0;
        self.text(cell.x + cell.w / 2.0, y, label, Text::label(10.0, LABEL));
        self.text_in(Rect::new(cell.x - 8.0, y + 7.0, cell.w + 16.0, 14.0), text, Text::mono(10.0, VALUE_TEXT));
    }

    /// A latching button with an LED.
    pub fn toggle(&mut self, r: Rect, on: bool, label: &str, hot: bool) {
        self.button_face(r, hot);

        let style = Text::label(11.0, if on { TITLE } else { 0xa9a397 });
        let w = self.text_width(label, style) + 13.0;
        let x = r.x + (r.w - w) / 2.0;
        let (lx, ly) = (x + 3.5, r.y + r.h / 2.0);
        if on {
            let glow = Paint::radial_gradient(lx, ly, 2.0, 8.0, rgba(LED_ON, 0.6), rgba(LED_ON, 0.0));
            self.fill(&circle(lx, ly, 8.0), &glow);
        }
        self.fill(&circle(lx, ly, 3.5), &Paint::color(rgb(if on { LED_ON } else { LED_OFF })));
        self.text(x + 13.0, ly, label, style.left());
    }

    /// ‹ value ›
    pub fn stepper(&mut self, r: Rect, text: &str, hot: bool) {
        self.well(r, r.h / 2.0, rgb(WELL));
        let arrows = Text::mono(14.0, MUTED);
        self.text(r.x + 11.0, r.y + r.h / 2.0, "‹", arrows);
        self.text(r.right() - 11.0, r.y + r.h / 2.0, "›", arrows);
        let value = Text::mono(12.0, if hot { 0xffcf73 } else { ARC });
        self.text_in(r.inset(22.0, 0.0, 22.0, 0.0), text, value);
    }

    /// The dice: a pale die on a button's face, showing five.
    pub fn dice(&mut self, r: Rect, hot: bool) {
        self.button_face(r, hot);
        let side = r.h - 12.0;
        let die = r.centered(side, side);
        self.fill(&rrect(die, 4.0), &Paint::color(rgb(if hot { 0xefe9da } else { TITLE })));
        let (cx, cy) = die.center();
        let far = side * 0.27;
        for (dx, dy) in [(-far, -far), (far, -far), (0.0, 0.0), (-far, far), (far, far)] {
            self.fill(&circle(cx + dx, cy + dy, side * 0.09), &Paint::color(rgb(0x1f2023)));
        }
    }

    /// A segment's curve button: its bow drawn small in a dark well
    /// (`points` 0..1 each way, y up), lit while hovered or dragged.
    pub fn curve_button(&mut self, r: Rect, points: &[(f32, f32)], hot: bool) {
        self.fill(&rrect(r, 3.0), &Paint::color(rgb(0x0d0e10)));
        let edge = if hot { 0xffcf73 } else { 0x4a4c51 };
        self.stroke(&rrect(r.inset(0.5, 0.5, 0.5, 0.5), 2.5), &stroke(rgb(edge), 1.0));
        let inner = r.inset(3.0, 3.0, 3.0, 3.0);
        self.stroke(&polyline(inner, points), &stroke(rgb(if hot { 0xffcf73 } else { ARC }), 1.3));
    }

    /// The dragged curve, large, beside its button: its name and value, the
    /// straight line for reference, and the curve.
    pub fn curve_popup(&mut self, button: Rect, name: &str, value: &str, points: &[(f32, f32)], rising: bool) {
        const W: f32 = 168.0;
        const H: f32 = 120.0;
        let x = (button.right() - W).max(8.0);
        let y = if button.y - H - 8.0 > 8.0 { button.y - H - 8.0 } else { button.bottom() + 8.0 };
        let r = Rect::new(x, y, W, H);
        self.shadow(r, 6.0, 14.0, 6.0, 0.8);
        self.fill(&rrect(r, 6.0), &Paint::color(rgb(0x0b0b0c)));
        let face = r.inset(1.0, 1.0, 1.0, 1.0);
        self.fill(&rrect(face, 5.0), &vgrad(face, rgb(0x2a2c30), rgb(0x1d1e21)));
        self.text(r.x + 12.0, r.y + 15.0, name, Text::label(10.0, LABEL).left());
        self.text(r.right() - 12.0, r.y + 15.0, value, Text::mono(10.0, ARC).right());
        let graph = r.inset(14.0, 28.0, 14.0, 12.0);
        self.screen(graph.inset(-4.0, -4.0, -4.0, -4.0));
        let straight = if rising { [(0.0, 0.0), (1.0, 1.0)] } else { [(0.0, 1.0), (1.0, 0.0)] };
        self.stroke(&polyline(graph, &straight), &stroke(rgba(0xffffff, 0.15), 1.0));
        self.trace(&polyline(graph, points), 1.0);
    }

    /// ☰: the settings button (lit while they're open).
    pub fn hamburger(&mut self, r: Rect, hot: bool, open: bool) {
        self.button_face(r, hot || open);
        let colour = if open { 0xffcf73 } else if hot { TITLE } else { 0xa9a397 };
        let (cx, cy) = r.center();
        for dy in [-6.0, 0.0, 6.0] {
            self.fill(&rrect(Rect::new(cx - 7.0, cy + dy - 1.0, 14.0, 2.0), 1.0), &Paint::color(rgb(colour)));
        }
    }

    /// A momentary button: a label on the toggles' face, no LED.
    pub fn button(&mut self, r: Rect, label: &str, hot: bool) {
        self.button_face(r, hot);
        let style = Text::label(11.0, if hot { TITLE } else { 0xa9a397 });
        self.text(r.x + r.w / 2.0, r.y + r.h / 2.0, label, style);
    }

    fn button_face(&mut self, r: Rect, hot: bool) {
        self.fill(&rrect(r, 5.0), &Paint::color(rgb(0x0b0b0c)));
        let face = r.inset(1.0, 1.0, 1.0, 1.0);
        let (top, bottom) = if hot { (0x3c3e43, 0x24252a) } else { (0x34363a, 0x1f2023) };
        self.fill(&rrect(face, 4.0), &vgrad(face, rgb(top), rgb(bottom)));
        self.fill(&rrect(Rect::new(face.x + 2.0, face.y, face.w - 4.0, 1.0), 0.0), &Paint::color(rgba(0xffffff, 0.07)));
    }

    /// value ▾
    pub fn dropdown(&mut self, r: Rect, text: &str, hot: bool) {
        self.well(r, 5.0, rgb(WELL));
        let value = Text::mono(12.0, if hot { 0xffcf73 } else { ARC }).left();
        self.text_in(r.inset(10.0, 0.0, 22.0, 0.0), text, value);
        let (x, y) = (r.right() - 12.0, r.y + r.h / 2.0);
        let mut arrow = Path::new();
        arrow.move_to(x - 4.0, y - 2.0);
        arrow.line_to(x + 4.0, y - 2.0);
        arrow.line_to(x, y + 3.0);
        arrow.close();
        self.fill(&arrow, &Paint::color(rgb(MUTED)));
    }

    /// The top bar's plate and its readout screen.
    pub fn readout(&mut self, r: Rect, name: &str, value: &str) {
        self.well(r.inset(-4.0, -4.0, -4.0, -4.0), 8.0, rgb(WELL));
        self.screen(r);
        let glow = Text::mono(14.0, AMBER);
        self.text_in(Rect::new(r.x + 12.0, r.y, r.w * 0.62, r.h), name, glow.left());
        self.text_in(Rect::new(r.x + r.w * 0.62, r.y, r.w * 0.38 - 12.0, r.h), value, glow.right());
    }

    /// A popup list laid out on `grid`.
    /// A popup list. With `thumb`, each item starts with a tiny picture of
    /// it: `thumb(item, t)` is its value (-1..1) at t in 0..1.
    pub fn menu(&mut self, r: Rect, items: &[String], grid: MenuGrid, selected: usize, hover: Option<usize>,
                thumb: Option<&dyn Fn(usize, f32) -> f32>) {
        self.shadow(r, 6.0, 14.0, 6.0, 0.8);
        self.fill(&rrect(r, 6.0), &Paint::color(rgb(0x0b0b0c)));
        let face = r.inset(1.0, 1.0, 1.0, 1.0);
        self.fill(&rrect(face, 5.0), &vgrad(face, rgb(0x2a2c30), rgb(0x1d1e21)));
        for (i, item) in items.iter().enumerate() {
            let cell = grid.cell(r, i);
            if Some(i) == hover || i == selected {
                let alpha = if Some(i) == hover { 0.22 } else { 0.12 };
                self.fill(&rrect(cell.inset(2.0, 1.0, 2.0, 1.0), 3.0), &Paint::color(rgba(ARC, alpha)));
            }
            let color = if i == selected { 0xffcf73 } else { 0xd8d2c4 };
            let mut text = cell.inset(8.0, 0.0, 4.0, 0.0);
            if let Some(f) = thumb {
                let pic = Rect::new(text.x, cell.y + 4.0, THUMB_W, cell.h - 8.0);
                let mut path = Path::new();
                for k in 0..=THUMB_POINTS {
                    let t = k as f32 / THUMB_POINTS as f32;
                    let (x, y) = (pic.x + t * pic.w, pic.y + pic.h / 2.0 - f(i, t).clamp(-1.0, 1.0) * pic.h / 2.0);
                    if k == 0 { path.move_to(x, y) } else { path.line_to(x, y) }
                }
                self.stroke(&path, &stroke(rgba(color, 0.85), 1.0));
                text = text.inset(THUMB_W + 6.0, 0.0, 0.0, 0.0);
            }
            self.text_in(text, item, Text::mono(12.0, color).left());
        }
    }
}

/// A menu thumbnail's width, and the points drawn across it.
pub const THUMB_W: f32 = 30.0;
const THUMB_POINTS: usize = 30;

/// What a modulatable control shows besides its value: that the
/// Modulations panel is on it (a ring of dots), and where the newest
/// voice's modulators have it now (travel 0..1, while a note plays).
#[derive(Clone, Copy, Debug, Default)]
pub struct Marks {
    pub selected: bool,
    pub moved: Option<f32>,
}

/// Where a knob's dial sits in its cell: the value arc at `r`.
#[derive(Clone, Copy, Debug)]
pub struct KnobGeometry {
    pub cx: f32,
    pub cy: f32,
    pub r: f32,
}

impl KnobGeometry {
    pub fn of(cell: Rect, size: f32, inline: bool) -> Self {
        let cx = if inline { cell.right() - size / 2.0 } else { cell.x + cell.w / 2.0 };
        Self { cx, cy: cell.y + size / 2.0, r: size / 2.0 - 3.0 }
    }
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
