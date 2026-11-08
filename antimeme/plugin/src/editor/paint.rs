//! Drawing primitives and the panel's palette and type, over femtovg.

use std::cell::Cell;

use nih_plug_vizia::vizia::prelude::Canvas;
use nih_plug_vizia::vizia::vg::{self, Align, Baseline, Color, Paint, Path};

use super::layout::Rect;

pub const GROUND: u32 = 0x0b0a09;
pub const INK: u32 = 0xece8df;
pub const RED: u32 = 0xf0402c;
pub const RED_LABEL: u32 = 0xff6a55;
pub const RULE: u32 = 0x4d4a44;
pub const TRACK: u32 = 0x8a867d;

pub fn color(hex: u32) -> Color {
    Color::rgb((hex >> 16) as u8, (hex >> 8) as u8, hex as u8)
}

pub fn fade(hex: u32, alpha: f32) -> Color {
    let mut c = color(hex);
    c.set_alphaf(alpha);
    c
}

static REGULAR: &[u8] = include_bytes!("../../assets/fonts/NotoSerifTibetan-Regular.ttf");
static SEMIBOLD: &[u8] = include_bytes!("../../assets/fonts/NotoSerifTibetan-SemiBold.ttf");
static DIGITS: &[u8] = include_bytes!("../../assets/fonts/NotoSerifTibetan-Digits.ttf");

#[derive(Clone, Copy)]
pub enum Face {
    Regular,
    SemiBold,
    Digits,
}

#[derive(Clone, Copy)]
struct Fonts {
    regular: vg::FontId,
    semibold: vg::FontId,
    digits: vg::FontId,
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
        probe.set_font(&[fonts.regular]);
        if k == key && canvas.measure_text(0.0, 0.0, "ཀ", &probe).is_ok() {
            return Some(fonts);
        }
    }
    let fonts = Fonts {
        regular: canvas.add_font_mem(REGULAR).ok()?,
        semibold: canvas.add_font_mem(SEMIBOLD).ok()?,
        digits: canvas.add_font_mem(DIGITS).ok()?,
    };
    FONTS.set(Some((key, fonts)));
    Some(fonts)
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

    pub fn rect(&mut self, x: f32, y: f32, w: f32, h: f32, fill: Color) {
        let mut path = Path::new();
        path.rect(x, y, w, h);
        self.canvas.fill_path(&path, &Paint::color(fill));
    }

    pub fn stroke(&mut self, path: &Path, stroke: Color, width: f32) {
        let mut paint = Paint::color(stroke);
        paint.set_line_width(width);
        self.canvas.stroke_path(path, &paint);
    }

    pub fn line_through(&mut self, r: Rect, points: impl Iterator<Item = (f32, f32)>, stroke: Color, width: f32) {
        let mut path = Path::new();
        for (i, (u, v)) in points.enumerate() {
            let (x, y) = (r.x + u * r.w, r.y + v * r.h);
            if i == 0 {
                path.move_to(x, y);
            } else {
                path.line_to(x, y);
            }
        }
        self.stroke(&path, stroke, width);
    }

    fn text_paint(&self, face: Face, size: f32, fill: Color, align: Align, baseline: Baseline) -> Paint {
        let mut paint = Paint::color(fill);
        if let Some(f) = self.fonts {
            let main = match face {
                Face::Regular => f.regular,
                Face::SemiBold => f.semibold,
                Face::Digits => f.digits,
            };
            paint.set_font(&[main]);
        }
        paint.set_font_size(size);
        paint.set_text_align(align);
        paint.set_text_baseline(baseline);
        paint
    }

    /// Text centred vertically on `y`.
    pub fn text(&mut self, x: f32, y: f32, text: &str, face: Face, size: f32, fill: Color, align: Align) {
        let paint = self.text_paint(face, size, fill, align, Baseline::Middle);
        let _ = self.canvas.fill_text(x, y, text, &paint);
    }

    /// Text sitting on the baseline `y`.
    pub fn text_on(&mut self, x: f32, y: f32, text: &str, face: Face, size: f32, fill: Color) {
        let paint = self.text_paint(face, size, fill, Align::Left, Baseline::Alphabetic);
        let _ = self.canvas.fill_text(x, y, text, &paint);
    }

    pub fn text_width(&self, text: &str, face: Face, size: f32) -> f32 {
        let paint = self.text_paint(face, size, Color::white(), Align::Left, Baseline::Alphabetic);
        self.canvas.measure_text(0.0, 0.0, text, &paint).map(|m| m.width()).unwrap_or(0.0)
    }
}

/// An unknown symbol: stems, bars and one small block, all from a seed. Same grammar
/// and sequence as in the design.
pub fn symbol(seed: u32, x: f32, y: f32, w: f32, h: f32) -> Path {
    const MODULUS: u64 = 1 << 32;
    let mut s = (seed as u64 * 2_654_435_761) % MODULUS;
    let mut next = move || {
        s = (s * 1_664_525 + 1_013_904_223) % MODULUS;
        s as f64 / MODULUS as f64
    };
    let across = |u: f32| x + u * w;
    let down = |v: f32| y + v * h;

    let mut path = Path::new();
    let stem = if next() < 0.5 { 0.0 } else { 1.0 };
    path.move_to(across(stem), down(0.0));
    path.line_to(across(stem), down(1.0));
    for v in [0.0, 0.5, 1.0] {
        if next() < 0.72 {
            let from = if next() < 0.5 { 0.0 } else { 0.35 };
            let to = if next() < 0.5 { 0.65 } else { 1.0 };
            path.move_to(across(from), down(v));
            path.line_to(across(to), down(v));
        }
    }
    if next() < 0.7 {
        let v = if next() < 0.5 { 0.25 } else { 0.75 };
        let u = if next() < 0.5 { 0.3 } else { 0.7 };
        path.rect(across(u) - 1.6, down(v) - 1.6, 3.2, 3.2);
    }
    path
}
