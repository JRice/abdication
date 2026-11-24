//! Painting the panel's parts with vector paths (femtovg). Colours and
//! proportions follow the design mockup. All coordinates are logical panel
//! pixels; the caller has already scaled the canvas.

use std::cell::Cell;
use std::f32::consts::PI;

use nih_plug_vizia::vizia::prelude::Canvas;
use nih_plug_vizia::vizia::vg::{self, Align, Baseline, Color, LineCap, Paint, Path, Solidity};

use super::layout::{LabelAt, Rect};

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
pub const WELL: u32 = 0x0f0d0b;
pub const LED_ON: u32 = 0xff7a3c;
pub const LED_OFF: u32 = 0x3d1f18;
pub const SCREEN_LINE: u32 = 0x3b2a12;
/// The screens' glass, about its middle (what the Windows' light adds onto).
pub const SCREEN_GLASS: u32 = 0x1b1309;
/// Where the newest voice's modulators have a knob: a pale rose, apart
/// from the amber and the sources' shades.
pub const LFO_COLOR: u32 = 0xe3a6c6;

/// Each modulation source's own earth shade (modulation::SOURCES order):
/// envelopes ochre, terracotta, sage; LFOs slate, heather, verdigris; the
/// played sources sand, clay, lichen, lavender. They differ in lightness
/// as well as hue.
pub const SOURCE_COLORS: [u32; 10] = [0xd2a24e, 0xc56a48, 0x909d5e, 0x7d97b2, 0xa77ea0, 0x5f9e94,
                                      0xd8c296, 0xc08a72, 0x94aaa6, 0xa596bd];

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
    /// A colour for the next controls' labels (a Windows interrupt's row).
    pub tint: Option<u32>,
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
        Self { canvas, fonts, tint: None }
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
        self.fill(&rrect(Rect::new(0.0, 0.0, w, h), 0.0), &Paint::color(rgb(0x100e0b)));
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

    /// The faceplate between the cheeks: dark umber, faintly brushed.
    pub fn faceplate(&mut self, r: Rect) {
        let (cx, cy) = (r.x + r.w / 2.0, r.y + r.h * 0.3);
        let glow = Paint::radial_gradient(cx, cy, 0.0, r.w * 0.7, rgb(0x211d18), rgb(0x15130f));
        self.fill(&rrect(r, 0.0), &glow);
        let brush = stroke(rgba(0xfff0d8, 0.012), 1.0);
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

    /// A raised module plate with its title and a saturated strip along its
    /// top: one colour, or one segment per colour.
    pub fn plate(&mut self, r: Rect, title: &str, strip: &[u32]) {
        self.raised(r, 9.0, 0x34302a, 0x24211c);
        self.stroke(&rrect(r.inset(0.5, 0.5, 0.5, 0.5), 9.0), &stroke(rgba(0x0f0d0a, 0.9), 1.0));
        let span = r.w - 20.0;
        let mut x = r.x + 10.0;
        for &colour in strip {
            let w = span / strip.len() as f32;
            let seg = Rect::new(x, r.y + 1.0, w - 2.0, 3.0);
            self.fill(&rrect(seg.inset(-2.0, -1.0, -2.0, -3.0), 3.0), &Paint::color(rgba(colour, 0.18)));
            self.fill(&rrect(seg, 1.5), &Paint::color(rgb(colour)));
            x += w;
        }
        if !title.is_empty() {
            let style = Text { spacing: 0.16, ..Text::label(13.0, TITLE) }.left();
            self.text(r.x + 14.0, r.y + 23.0, title, style);
        }
    }

    /// A source's group on its plate: a faint wash of its shade.
    pub fn wash(&mut self, r: Rect, colour: u32) {
        let paint = Paint::linear_gradient(r.x, r.y, r.x, r.bottom(), rgba(colour, 0.11), rgba(colour, 0.05));
        self.fill(&rrect(r, 6.0), &paint);
        self.stroke(&rrect(r.inset(0.5, 0.5, 0.5, 0.5), 6.0), &stroke(rgba(colour, 0.18), 1.0));
    }

    /// A groove between parts of a plate.
    pub fn divider(&mut self, x: f32, y: f32, h: f32) {
        self.fill(&rrect(Rect::new(x, y, 1.0, h), 0.0), &Paint::color(rgba(0, 0.45)));
        self.fill(&rrect(Rect::new(x + 1.0, y, 1.0, h), 0.0), &Paint::color(rgba(0xffebc8, 0.06)));
    }

    /// The panel's name, engraved.
    pub fn wordmark(&mut self, x: f32, y: f32) {
        let style = Text { spacing: 0.42, ..Text::label(20.0, 0x0b0a08) };
        self.text(x, y + 1.0, "Abdication", style);
        self.text(x, y, "Abdication", Text { color: rgb(0xd8ccb4), ..style });
    }

    /// The veil over the panel while a source is being assigned.
    pub fn veil(&mut self, w: f32, h: f32) {
        self.fill(&rrect(Rect::new(0.0, 0.0, w, h), 0.0), &Paint::color(rgba(0x0c0a08, 0.55)));
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
        let radius = if r.w.min(r.h) < 60.0 { 4.0 } else { 6.0 };
        self.fill(&rrect(r.inset(-1.0, -1.0, -1.0, -1.0), radius + 1.0), &Paint::color(rgb(0x050403)));
        self.fill(&rrect(r.inset(-1.0, r.h, -1.0, -2.0), radius), &Paint::color(rgba(0xffebc8, 0.07)));
        let (cx, cy) = (r.x + r.w / 2.0, r.y + r.h * 0.4);
        let glass = Paint::radial_gradient(cx, cy, 0.0, r.w.max(r.h) * 0.6, rgb(0x22180b), rgb(0x110c06));
        self.fill(&rrect(r, radius), &glass);
        let inner = Paint::box_gradient(r.x, r.y + 2.0, r.w, r.h, radius, 6.0, rgba(0, 0.0), rgba(0, 0.9));
        self.fill(&rrect(r, radius), &inner);
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

    /// A line in its own colour (a source's outline on a screen).
    pub fn stroke_colour(&mut self, path: &Path, colour: u32, width: f32, alpha: f32) {
        let mut line = stroke(rgba(colour, alpha), width);
        line.set_line_cap(LineCap::Round);
        self.stroke(path, &line);
    }

    /// A bright dot with a glow (where a voice is).
    pub fn dot(&mut self, x: f32, y: f32, r: f32, colour: u32) {
        self.fill(&circle(x, y, r * 2.2), &Paint::color(rgba(colour, 0.25)));
        self.fill(&circle(x, y, r), &Paint::color(rgb(colour)));
    }

    /// A flat fill (a Windows span's light).
    pub fn flat(&mut self, r: Rect, color: u32) {
        self.fill(&rrect(r, 0.0), &Paint::color(rgb(color)));
    }

    /// A line on a screen in its own colour.
    pub fn screen_line_coloured(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, color: u32, width: f32) {
        let mut p = Path::new();
        p.move_to(x0, y0);
        p.line_to(x1, y1);
        self.stroke(&p, &stroke(rgb(color), width));
    }

    /// The trace again in dark, only inside `clip` (over a bright fill).
    pub fn trace_dark(&mut self, path: &Path, clip: Rect) {
        self.canvas.save();
        self.canvas.intersect_scissor(clip.x, clip.y, clip.w, clip.h);
        let mut line = stroke(rgb(0x1b1c20), 2.2);
        line.set_line_cap(LineCap::Round);
        self.stroke(path, &line);
        self.canvas.restore();
    }

    /// A filled shape on a screen (a sample's picture): `color` at `alpha`,
    /// edged with a faint trace.
    pub fn fill_shape(&mut self, path: &Path, color: u32, alpha: f32) {
        self.fill(path, &Paint::color(rgba(color, alpha)));
        self.stroke(path, &stroke(rgba(AMBER, 0.35 * alpha), 1.0));
    }

    /// A bright marker bar on a screen (where a player is), with a glow.
    pub fn bar(&mut self, r: Rect, alpha: f32) {
        self.fill(&rrect(r.inset(-2.0, 0.0, -2.0, 0.0), 2.0), &Paint::color(rgba(AMBER, 0.2 * alpha)));
        self.fill(&rrect(r, 1.0), &Paint::color(rgba(0xffcf73, alpha)));
    }

    /// A thin dim line from `top` down to `bottom` at `x` (a harmonic's level).
    pub fn screen_bar(&mut self, x: f32, top: f32, bottom: f32) {
        let mut p = Path::new();
        p.move_to(x, bottom);
        p.line_to(x, top);
        self.stroke(&p, &stroke(rgba(AMBER_DIM, 0.8), 1.5));
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
    pub fn knob(&mut self, cell: Rect, size: f32, value: f32, bipolar: bool, at: LabelAt,
                label: &str, text: &str, hot: bool, marks: &Marks) {
        let KnobGeometry { cx, cy, r } = KnobGeometry::of(cell, size, at);
        let angle = |t: f32| (135.0 + 270.0 * t) * PI / 180.0;
        let arc = |radius: f32, t0: f32, t1: f32| {
            let mut p = Path::new();
            p.arc(cx, cy, radius, angle(t0), angle(t1), Solidity::Hole);
            p
        };
        // Assigning: the source's ring, and how far it moves the knob from here.
        if let Some((colour, depth)) = marks.assign {
            self.fill(&circle(cx, cy, r + 11.0), &Paint::color(rgba(colour, 0.08)));
            self.stroke(&circle(cx, cy, r + 6.0), &stroke(rgba(colour, 0.35), 1.0));
            let to = (value + depth).clamp(0.0, 1.0);
            if (to - value).abs() > 0.002 {
                let mut line = stroke(rgb(colour), 3.0);
                line.set_line_cap(LineCap::Round);
                self.stroke(&arc(r + 6.0, value.min(to), value.max(to)), &line);
            }
        } else if let Some(moved) = marks.moved.filter(|m| (m - value).abs() > 0.004) {
            // Where the newest voice's modulators have it: an arc from the
            // knob's own value, ending in a dot.
            let radius = r + 6.0;
            let mut line = stroke(rgb(LFO_COLOR), 2.0);
            line.set_line_cap(LineCap::Round);
            self.stroke(&arc(radius, value.min(moved), value.max(moved)), &line);
            let a = angle(moved);
            self.fill(&circle(cx + radius * a.cos(), cy + radius * a.sin(), 2.4), &Paint::color(rgb(LFO_COLOR)));
        }
        let mut track = stroke(rgb(0x0b0a08), 3.6);
        track.set_line_cap(LineCap::Round);
        self.stroke(&arc(r, 0.0, 1.0), &track);
        let (from, to) = if bipolar { (value.min(0.5), value.max(0.5)) } else { (0.0, value) };
        if to - from > 0.004 {
            let mut lit = stroke(rgb(if hot { 0xffcf73 } else { ARC }), 2.4);
            lit.set_line_cap(LineCap::Round);
            self.stroke(&arc(r, from, to), &lit);
        }

        // The dial: shadow, a knurled skirt, the cap, a cream pointer.
        let body_r = r - 4.5;
        let cap_r = body_r * 0.70;
        self.fill(&circle(cx, cy + 2.2, body_r), &Paint::color(rgba(0, 0.6)));
        let body = Paint::radial_gradient_stops(
            cx - body_r * 0.3, cy - body_r * 0.44, 0.0, body_r * 1.6,
            [(0.0, rgb(0x5d5750)), (0.55, rgb(0x2c2925)), (1.0, rgb(0x151311))],
        );
        self.fill(&circle(cx, cy, body_r), &body);
        self.stroke(&circle(cx, cy, body_r), &stroke(rgb(0x070605), 1.0));
        let teeth = (size * 0.9).max(18.0) as usize;
        let mut knurl = Path::new();
        for k in 0..teeth {
            let a = 2.0 * PI * k as f32 / teeth as f32;
            knurl.move_to(cx + (body_r - 3.2) * a.cos(), cy + (body_r - 3.2) * a.sin());
            knurl.line_to(cx + (body_r - 0.6) * a.cos(), cy + (body_r - 0.6) * a.sin());
        }
        self.stroke(&knurl, &stroke(rgba(0, 0.45), 0.9));
        let cap = Paint::radial_gradient(cx - cap_r * 0.24, cy - cap_r * 0.4, 0.0, cap_r * 1.5, rgb(0x4e4943), rgb(0x1d1b18));
        self.fill(&circle(cx, cy, cap_r), &cap);
        self.stroke(&circle(cx, cy, cap_r), &stroke(rgba(0xffffff, 0.10), 1.0));
        let a = angle(value);
        let mut pointer = Path::new();
        pointer.move_to(cx + cap_r * 0.25 * a.cos(), cy + cap_r * 0.25 * a.sin());
        pointer.line_to(cx + (body_r - 2.5) * a.cos(), cy + (body_r - 2.5) * a.sin());
        let mut line = stroke(rgb(0xf1eadb), (size / 26.0).max(1.8));
        line.set_line_cap(LineCap::Round);
        self.stroke(&pointer, &line);

        // The sources acting on it: a dot each in its shade, above the dial.
        if marks.assign.is_none() && !marks.pips.is_empty() {
            let n = marks.pips.len() as f32;
            for (k, &colour) in marks.pips.iter().enumerate() {
                let x = cx - (n - 1.0) * 3.5 + 7.0 * k as f32;
                self.fill(&circle(x, cy - r - 9.0, 2.4), &Paint::color(rgb(colour)));
                self.stroke(&circle(x, cy - r - 9.0, 2.4), &stroke(rgba(0, 0.5), 0.6));
            }
        }

        let tint = self.tint.unwrap_or(LABEL);
        match at {
            LabelAt::Left => {
                let x = cx - size / 2.0 - 10.0;
                self.text(x, cy - 7.0, label, Text::label(11.0, tint).right());
                self.text(x, cy + 8.0, text, Text::mono(10.0, VALUE_TEXT).right());
            }
            LabelAt::Right => {
                let x = cx + size / 2.0 + 10.0;
                self.text(x, cy - 7.0, label, Text::label(11.0, tint).left());
                self.text_in(Rect::new(x, cy + 1.0, cell.right() - x, 14.0), text, Text::mono(10.0, VALUE_TEXT).left());
            }
            LabelAt::Below => {
                let y = cell.y + size + 9.0;
                self.text(cx, y, label, Text::label(10.0, tint));
                self.text_in(Rect::new(cell.x - 6.0, y + 7.0, cell.w + 12.0, 14.0), text, Text::mono(10.0, VALUE_TEXT));
            }
            LabelAt::BelowRight(dx) => {
                let y = cell.y + size + 9.0;
                self.text(cx + dx, y, label, Text::label(11.0, tint));
                self.text(cx + dx, y + 13.0, text, Text::mono(10.0, VALUE_TEXT));
            }
        }
    }

    /// The HPF: an arc round the cutoff's lower left quarter (6 o'clock off,
    /// 9 o'clock its top), a teal handle at its setting.
    pub fn hpf_arc(&mut self, cx: f32, cy: f32, r: f32, value: f32, hot: bool, marks: &Marks) {
        let angle = |t: f32| (90.0 + 90.0 * t) * PI / 180.0;
        let arc = |radius: f32, t0: f32, t1: f32| {
            let mut p = Path::new();
            p.arc(cx, cy, radius, angle(t0), angle(t1), Solidity::Hole);
            p
        };
        let mut track = stroke(rgb(0x0b0a08), 7.0);
        track.set_line_cap(LineCap::Round);
        self.stroke(&arc(r, 0.0, 1.0), &track);
        if value > 0.004 {
            let mut lit = stroke(rgb(if hot { 0xffcf73 } else { ARC }), 2.6);
            lit.set_line_cap(LineCap::Round);
            self.stroke(&arc(r, 0.0, value), &lit);
        }
        let mut ticks = Path::new();
        for k in 0..7 {
            let a = angle(k as f32 / 6.0);
            ticks.move_to(cx + (r - 7.0) * a.cos(), cy + (r - 7.0) * a.sin());
            ticks.line_to(cx + (r - 4.5) * a.cos(), cy + (r - 4.5) * a.sin());
        }
        self.stroke(&ticks, &stroke(rgb(0x6a6256), 0.8));
        if let Some((colour, depth)) = marks.assign {
            let to = (value + depth).clamp(0.0, 1.0);
            let mut line = stroke(rgb(colour), 3.0);
            line.set_line_cap(LineCap::Round);
            if (to - value).abs() > 0.002 {
                self.stroke(&arc(r + 7.0, value.min(to), value.max(to)), &line);
            }
        } else if let Some(moved) = marks.moved.filter(|m| (m - value).abs() > 0.004) {
            let mut line = stroke(rgb(LFO_COLOR), 2.0);
            line.set_line_cap(LineCap::Round);
            self.stroke(&arc(r + 7.0, value.min(moved), value.max(moved)), &line);
        }
        let a = angle(value);
        let (hx, hy) = (cx + r * a.cos(), cy + r * a.sin());
        self.fill(&circle(hx, hy + 0.6, 5.0), &Paint::color(rgba(0, 0.6)));
        self.fill(&circle(hx, hy, 4.4), &Paint::color(rgb(if hot { 0x5fc9bd } else { 0x2b8f86 })));
        self.stroke(&circle(hx, hy, 4.4), &stroke(rgba(0xd9f2ee, 0.6), 0.8));
        if !marks.pips.is_empty() && marks.assign.is_none() {
            let m = angle(0.5);
            for (k, &colour) in marks.pips.iter().enumerate() {
                let rr = r + 10.0 + 6.0 * k as f32;
                self.fill(&circle(cx + rr * m.cos(), cy + rr * m.sin(), 2.4), &Paint::color(rgb(colour)));
            }
        }
        let style = Text::label(9.0, MUTED);
        self.text(cx + 4.0, cy + r + 9.0, "off", style);
        self.text(cx - r - 12.0, cy - 4.0, "8k", style);
        let m = angle(0.5);
        self.text(cx + (r + 16.0) * m.cos(), cy + (r + 16.0) * m.sin() - 4.0, "HPF", Text::label(10.5, LABEL));
    }

    /// A thin fader: a 3 px groove with ticks, its value lit in `tint`
    /// (from the middle when `bipolar`), a slim cap `cap` wide, its label
    /// under it.
    #[allow(clippy::too_many_arguments)]
    pub fn fader(&mut self, cell: Rect, value: f32, bipolar: bool, tint: u32, cap: f32, label: &str, hot: bool,
                 marks: &Marks) {
        let cx = cell.x + cell.w / 2.0;
        let (top, bottom) = (cell.y, cell.bottom());
        let y = |v: f32| bottom - (bottom - top) * v.clamp(0.0, 1.0);
        let mut ticks = Path::new();
        for i in 0..=10 {
            let ty = top + (bottom - top) * i as f32 / 10.0;
            let w = if i % 5 == 0 { 5.5 } else { 3.5 };
            ticks.move_to(cx - 4.0 - w, ty);
            ticks.line_to(cx - 4.0, ty);
        }
        self.stroke(&ticks, &stroke(rgb(0x5c554b), 0.8));
        self.well(Rect::new(cx - 1.5, top, 3.0, bottom - top), 1.5, rgb(0x080706));
        let from = if bipolar { y(0.5) } else { bottom };
        let (a, b) = (from.min(y(value)), from.max(y(value)));
        if b - a > 0.5 {
            self.fill(&rrect(Rect::new(cx - 0.8, a, 1.6, b - a), 0.8), &Paint::color(rgb(if hot { 0xffcf73 } else { tint })));
        }
        if let Some((colour, depth)) = marks.assign {
            let to = y(value + depth);
            if (to - y(value)).abs() > 0.5 {
                self.fill(&rrect(Rect::new(cx + 3.0, to.min(y(value)), 3.0, (to - y(value)).abs()), 1.5), &Paint::color(rgb(colour)));
            }
        } else if let Some(moved) = marks.moved.filter(|m| (m - value).abs() > 0.004) {
            let mut p = Path::new();
            p.move_to(cx + 4.5, y(value));
            p.line_to(cx + 4.5, y(moved));
            let mut line = stroke(rgb(LFO_COLOR), 2.0);
            line.set_line_cap(LineCap::Round);
            self.stroke(&p, &line);
            self.fill(&circle(cx + 4.5, y(moved), 2.2), &Paint::color(rgb(LFO_COLOR)));
        }
        let cy = y(value);
        let capr = Rect::new(cx - cap / 2.0, cy - 3.5, cap, 7.0);
        self.fill(&rrect(Rect::new(capr.x, capr.y + 1.2, capr.w, capr.h), 1.5), &Paint::color(rgba(0, 0.55)));
        let paint = Paint::linear_gradient_stops(
            capr.x, capr.y, capr.x, capr.bottom(),
            [(0.0, rgb(if hot { 0x9a9286 } else { 0x8a8378 })), (0.5, rgb(0x4a453e)), (1.0, rgb(0x2a2722))],
        );
        self.fill(&rrect(capr, 1.5), &paint);
        self.stroke(&rrect(capr, 1.5), &stroke(rgb(0x0b0a08), 0.7));
        self.fill(&rrect(Rect::new(capr.x + 2.5, cy - 0.55, cap - 5.0, 1.1), 0.0), &Paint::color(rgb(0xf1eadb)));
        if marks.assign.is_none() {
            for (k, &colour) in marks.pips.iter().enumerate() {
                self.fill(&circle(cx + 7.0, top - 2.0 + 7.0 * k as f32, 2.2), &Paint::color(rgb(colour)));
            }
        }
        if !label.is_empty() {
            self.text(cx, bottom + 11.0, label, Text::label(10.5, self.tint.unwrap_or(LABEL)));
        }
    }

    /// An assignment button (after Elektron's): a square rubber key with
    /// an LED slot in its source's shade, pressed in and lit while assigning.
    pub fn assign_button(&mut self, r: Rect, label: &str, colour: u32, pressed: bool, hot: bool) {
        self.fill(&rrect(r.inset(-2.0, -2.0, -2.0, -3.0), 5.0), &Paint::color(rgb(0x0a0907)));
        let shift = if pressed { 1.5 } else { 0.0 };
        if !pressed {
            self.fill(&rrect(Rect::new(r.x, r.y + 2.5, r.w, r.h), 3.5), &Paint::color(rgba(0, 0.7)));
        }
        let key = Rect::new(r.x, r.y + shift, r.w, r.h);
        let (top, bottom) = match (pressed, hot) {
            (true, _) => (0x3d3933, 0x2e2b27),
            (false, true) => (0x625c53, 0x403b35),
            (false, false) => (0x58534b, 0x3a3631),
        };
        self.fill(&rrect(key, 3.5), &vgrad(key, rgb(top), rgb(bottom)));
        self.stroke(&rrect(key, 3.5), &stroke(rgb(0x0b0a08), 0.8));
        if pressed {
            self.fill(&rrect(key, 3.5), &Paint::color(rgba(colour, 0.18)));
        } else {
            self.fill(&rrect(Rect::new(key.x + 1.5, key.y + 1.0, key.w - 3.0, 1.2), 0.6), &Paint::color(rgba(0xffffff, 0.13)));
        }
        let led = Rect::new(key.x + key.w / 2.0 - 6.0, key.y + 4.0, 12.0, 2.6);
        if pressed {
            self.fill(&rrect(led.inset(-3.0, -1.5, -3.0, -1.5), 2.8), &Paint::color(rgba(colour, 0.35)));
        }
        self.fill(&rrect(led, 1.3), &Paint::color(rgb(if pressed { colour } else { 0x1a1612 })));
        let size = if label.len() == 1 { 13.0 } else { 10.5 };
        let ink = if pressed { colour } else { 0xe6dccb };
        self.text(key.x + key.w / 2.0, key.y + key.h / 2.0 + 3.0, label, Text::label(size, ink));
    }

    /// A row of segments, one per setting, the chosen one raised.
    pub fn segments(&mut self, r: Rect, names: &[&str], chosen: usize, hot: bool) {
        self.well(r, 4.0, rgb(0x0f0d0b));
        let w = r.w / names.len() as f32;
        for (k, name) in names.iter().enumerate() {
            let seg = Rect::new(r.x + k as f32 * w + 2.0, r.y + 2.0, w - 4.0, r.h - 4.0);
            if k == chosen {
                self.shadow(seg, 3.0, 2.0, 1.0, 0.6);
                self.fill(&rrect(seg, 3.0), &vgrad(seg, rgb(if hot { 0x666057 } else { 0x5a544b }), rgb(0x3a3630)));
            }
            let ink = if k == chosen { TITLE } else { MUTED };
            self.text(seg.x + seg.w / 2.0, seg.y + seg.h / 2.0, name, Text::label(10.0, ink));
        }
    }

    /// Where along a slider's track `x` falls (0..1).
    pub fn slider_value(cell: Rect, x: f32) -> f32 {
        let t = Self::slider_track(cell);
        ((x - t.x) / t.w).clamp(0.0, 1.0)
    }

    /// How far a slider's thumb travels.
    pub fn slider_travel(cell: Rect) -> f32 {
        Self::slider_track(cell).w
    }

    /// The part of a slider cell the thumb's centre runs along (its value
    /// readout takes the right end).
    fn slider_track(cell: Rect) -> Rect {
        Rect::new(cell.x + 4.0, cell.y, cell.w - 46.0, cell.h)
    }

    /// A thin horizontal fader (a Windows interrupt's Pitch): a slot, a small
    /// thumb, the value at its right, tinted like its row.
    pub fn slider(&mut self, cell: Rect, value: f32, text: &str, hot: bool, marks: &Marks) {
        let t = Self::slider_track(cell);
        let cy = t.y + t.h / 2.0;
        let x = t.x + value * t.w;
        let tint = self.tint.unwrap_or(ARC);
        self.well(Rect::new(t.x - 2.0, cy - 1.5, t.w + 4.0, 3.0), 1.5, rgb(0x0d0e10));
        if let Some((colour, depth)) = marks.assign {
            let to = t.x + (value + depth).clamp(0.0, 1.0) * t.w;
            if (to - x).abs() > 0.5 {
                self.fill(&rrect(Rect::new(x.min(to), cy + 3.0, (to - x).abs(), 3.0), 1.5), &Paint::color(rgb(colour)));
            }
        }
        // Lit from ×1's place (a quarter along) to the thumb.
        let one = t.x + 0.25 * t.w;
        let (a, b) = if x < one { (x, one) } else { (one, x) };
        if b - a > 0.5 {
            self.fill(&rrect(Rect::new(a, cy - 0.75, b - a, 1.5), 0.75), &Paint::color(rgba(tint, 0.85)));
        }
        let mut tick = Path::new();
        tick.move_to(one, cy - 5.0);
        tick.line_to(one, cy - 3.0);
        self.stroke(&tick, &stroke(rgb(0x57595e), 1.0));
        if let Some(moved) = marks.moved.filter(|m| (m - value).abs() > 0.004 && marks.assign.is_none()) {
            let mx = t.x + moved * t.w;
            let mut p = Path::new();
            p.move_to(x, cy + 4.5);
            p.line_to(mx, cy + 4.5);
            let mut line = stroke(rgb(LFO_COLOR), 1.6);
            line.set_line_cap(LineCap::Round);
            self.stroke(&p, &line);
            self.fill(&circle(mx, cy + 4.5, 2.0), &Paint::color(rgb(LFO_COLOR)));
        }
        let thumb = Rect::new(x - 4.0, cy - 5.0, 8.0, 10.0);
        self.shadow(thumb, 2.0, 2.0, 1.0, 0.7);
        let paint = Paint::linear_gradient_stops(
            thumb.x, thumb.y, thumb.x, thumb.bottom(),
            [(0.0, rgb(if hot { 0x7a7d84 } else { 0x62656b })), (0.5, rgb(0x3a3c41)), (1.0, rgb(0x24262a))],
        );
        self.fill(&rrect(thumb, 2.0), &paint);
        self.fill(&rrect(Rect::new(x - 0.75, thumb.y + 2.0, 1.5, 6.0), 0.5), &Paint::color(rgb(tint)));
        self.text_in(Rect::new(t.right() + 8.0, cell.y - 1.0, cell.right() - t.right() - 8.0, cell.h + 2.0), text,
                     Text::mono(10.0, VALUE_TEXT).left());
    }

    /// A latching button with an LED.
    pub fn toggle(&mut self, r: Rect, on: bool, label: &str, hot: bool) {
        self.button_face(r, hot);

        let style = Text::label(11.0, self.tint.unwrap_or(if on { TITLE } else { 0xa9a397 }));
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

    /// The init button: an empty-set sign (a ring struck through), drawn
    /// rather than set in type so it doesn't depend on the font having it.
    pub fn empty_set(&mut self, r: Rect, hot: bool) {
        self.button_face(r, hot);
        let colour = rgb(if hot { TITLE } else { 0xa9a397 });
        let (cx, cy) = r.center();
        let radius = 6.5;
        self.stroke(&circle(cx, cy, radius), &stroke(colour, 1.6));
        let mut slash = Path::new();
        let reach = radius + 3.5;
        slash.move_to(cx - 0.6 * reach, cy + 0.8 * reach);
        slash.line_to(cx + 0.6 * reach, cy - 0.8 * reach);
        self.stroke(&slash, &stroke(colour, 1.6));
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
        let (top, bottom) = if hot { (0x55504a, 0x34302b) } else { (0x4a453e, 0x2f2c27) };
        self.fill(&rrect(face, 4.0), &vgrad(face, rgb(top), rgb(bottom)));
        self.fill(&rrect(Rect::new(face.x + 2.0, face.y, face.w - 4.0, 1.0), 0.0), &Paint::color(rgba(0xffffff, 0.07)));
    }

    /// A floating panel's face (menus, the preset-name entry).
    pub fn popup(&mut self, r: Rect) {
        self.shadow(r, 6.0, 14.0, 6.0, 0.8);
        self.fill(&rrect(r, 6.0), &Paint::color(rgb(0x0b0a08)));
        let face = r.inset(1.0, 1.0, 1.0, 1.0);
        self.fill(&rrect(face, 5.0), &vgrad(face, rgb(0x2e2a25), rgb(0x1f1c18)));
    }

    /// One wave in the wave picker: its picture on a little screen
    /// (`f(t)`, -1..1 across one cycle), its name under it.
    pub fn wave_cell(&mut self, cell: Rect, name: &str, f: &dyn Fn(f32) -> f32, selected: bool, hover: bool) {
        if selected || hover {
            let alpha = if hover { 0.22 } else { 0.14 };
            self.fill(&rrect(cell.inset(1.0, 0.0, 1.0, 0.0), 4.0), &Paint::color(rgba(ARC, alpha)));
        }
        let pic = Rect::new(cell.x + 4.0, cell.y + 3.0, cell.w - 8.0, cell.h - 19.0);
        self.fill(&rrect(pic, 3.0), &Paint::color(rgb(0x120d06)));
        let inner = pic.inset(3.0, 3.0, 3.0, 3.0);
        let mut path = Path::new();
        for k in 0..=40 {
            let t = k as f32 / 40.0;
            let (x, y) = (inner.x + t * inner.w, inner.y + inner.h / 2.0 - f(t).clamp(-1.0, 1.0) * inner.h / 2.0);
            if k == 0 { path.move_to(x, y) } else { path.line_to(x, y) }
        }
        self.trace(&path, if hover || selected { 1.0 } else { 0.75 });
        let colour = if selected { 0xffcf73 } else { 0xd8d2c4 };
        self.text_in(Rect::new(cell.x, cell.bottom() - 15.0, cell.w, 14.0), name, Text::mono(9.0, colour));
    }

    /// A preset's save button: a disk on a button's face.
    pub fn save_button(&mut self, r: Rect, hot: bool) {
        self.button_face(r, hot);
        let ink = rgb(if hot { TITLE } else { 0xe6dccb });
        let d = Rect::new(r.x + r.w / 2.0 - 6.0, r.y + r.h / 2.0 - 6.5, 12.0, 13.0);
        let mut body = Path::new();
        body.move_to(d.x, d.y);
        body.line_to(d.right() - 2.5, d.y);
        body.line_to(d.right(), d.y + 2.5);
        body.line_to(d.right(), d.bottom());
        body.line_to(d.x, d.bottom());
        body.close();
        self.stroke(&body, &stroke(ink, 1.2));
        self.stroke(&rrect(Rect::new(d.x + 2.5, d.y, 5.5, 4.0), 0.0), &stroke(ink, 1.1));
        self.fill(&rrect(Rect::new(d.x + 2.0, d.y + 7.5, d.w - 4.0, 5.0), 0.6), &Paint::color(ink));
    }

    /// A text field (the preset's name): its text and a caret.
    pub fn text_field(&mut self, r: Rect, text: &str) {
        self.well(r, 4.0, rgb(WELL));
        let style = Text::mono(12.0, 0xffcf73).left();
        let w = self.text_width(text, style);
        self.text_in(r.inset(8.0, 0.0, 8.0, 0.0), text, style);
        let x = (r.x + 8.0 + w + 1.0).min(r.right() - 6.0);
        self.fill(&rrect(Rect::new(x, r.y + 5.0, 1.5, r.h - 10.0), 0.0), &Paint::color(rgb(AMBER)));
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

    /// A popup list, item k in `cells[k]`, with `headers` (text, x, y) over
    /// groups of them. With `thumb`, each item starts with a tiny picture of
    /// it: `thumb(item, t)` is its value (-1..1) at t in 0..1.
    #[allow(clippy::too_many_arguments)]
    pub fn menu(&mut self, r: Rect, items: &[String], cells: &[Rect], headers: &[(&str, f32, f32)], selected: usize,
                hover: Option<usize>, thumb: Option<&dyn Fn(usize, f32) -> f32>) {
        self.popup(r);
        for &(text, x, y) in headers {
            self.text(x, y, text, Text::label(11.0, MUTED).left());
        }
        for (i, (item, &cell)) in items.iter().zip(cells).enumerate() {
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

/// What a modulatable control shows besides its value: the shades of the
/// sources moving it (`pips`); while a source is being assigned, its shade
/// and how far it moves the control (`assign`: depth in travel, -1..1);
/// and where the newest voice's modulators have it now (travel 0..1, while
/// a note plays).
#[derive(Clone, Debug, Default)]
pub struct Marks {
    pub pips: Vec<u32>,
    pub assign: Option<(u32, f32)>,
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
    pub fn of(cell: Rect, size: f32, at: LabelAt) -> Self {
        let cx = match at {
            LabelAt::Left => cell.right() - size / 2.0,
            LabelAt::Right => cell.x + size / 2.0,
            _ => cell.x + cell.w / 2.0,
        };
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
