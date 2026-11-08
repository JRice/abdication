//! Drawing helpers on femtovg: colours, fonts, arcs, text and outlines.

use std::cell::Cell;

use nih_plug_vizia::vizia::prelude::Canvas;
use nih_plug_vizia::vizia::vg::{self, Align, Baseline, Color, Paint, Path};

static DISPLAY: &[u8] = include_bytes!("../../assets/fonts/BarlowCondensed-SemiBold.ttf");
static MONO: &[u8] = include_bytes!("../../assets/fonts/IBMPlexMono-Medium.ttf");

pub fn color(hex: u32, alpha: f32) -> Color {
    let mut c = Color::rgb((hex >> 16) as u8, (hex >> 8) as u8, hex as u8);
    c.set_alphaf(alpha.clamp(0.0, 1.0));
    c
}

#[derive(Clone, Copy)]
pub struct Fonts {
    display: vg::FontId,
    mono: vg::FontId,
}

thread_local! {
    /// Fonts are registered per canvas; the canvas address tells us when a reopened editor
    /// brought a new one.
    static FONTS: Cell<Option<(usize, Fonts)>> = const { Cell::new(None) };
}

pub fn fonts(canvas: &mut Canvas) -> Option<Fonts> {
    let key = canvas as *const Canvas as usize;
    if let Some((k, fonts)) = FONTS.get() {
        let mut probe = Paint::color(Color::white());
        probe.set_font(&[fonts.mono]);
        if k == key && canvas.measure_text(0.0, 0.0, "A", &probe).is_ok() {
            return Some(fonts);
        }
    }
    let fonts = Fonts { display: canvas.add_font_mem(DISPLAY).ok()?, mono: canvas.add_font_mem(MONO).ok()? };
    FONTS.set(Some((key, fonts)));
    Some(fonts)
}

#[derive(Clone, Copy)]
pub enum Face {
    /// Barlow Condensed, upper case, spaced.
    Label,
    /// Barlow Condensed as written.
    Display,
    Mono,
}

pub struct Painter<'a> {
    pub canvas: &'a mut Canvas,
    pub fonts: Option<Fonts>,
}

impl Painter<'_> {
    pub fn fill_circle(&mut self, x: f32, y: f32, r: f32, fill: Color) {
        let mut path = Path::new();
        path.circle(x, y, r);
        self.canvas.fill_path(&path, &Paint::color(fill));
    }

    pub fn stroke(&mut self, path: &Path, stroke: Color, width: f32) {
        let mut paint = Paint::color(stroke);
        paint.set_line_width(width);
        paint.set_line_join(vg::LineJoin::Round);
        paint.set_line_cap(vg::LineCap::Round);
        self.canvas.stroke_path(path, &paint);
    }

    pub fn line(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, stroke: Color, width: f32) {
        let mut path = Path::new();
        path.move_to(x0, y0);
        path.line_to(x1, y1);
        self.stroke(&path, stroke, width);
    }

    pub fn rounded(&mut self, x: f32, y: f32, w: f32, h: f32, r: f32, fill: Option<Color>, edge: Option<(Color, f32)>) {
        let mut path = Path::new();
        path.rounded_rect(x, y, w, h, r);
        if let Some(fill) = fill {
            self.canvas.fill_path(&path, &Paint::color(fill));
        }
        if let Some((edge, width)) = edge {
            self.stroke(&path, edge, width);
        }
    }

    /// An arc from angle `a0` to `a1` (radians, clockwise from the right), as a polyline.
    pub fn arc(&mut self, x: f32, y: f32, r: f32, a0: f32, a1: f32, stroke: Color, width: f32) {
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
        self.stroke(&path, stroke, width);
    }

    pub fn polyline(&mut self, points: impl IntoIterator<Item = (f32, f32)>, stroke: Color, width: f32) {
        let mut path = Path::new();
        for (i, (x, y)) in points.into_iter().enumerate() {
            if i == 0 {
                path.move_to(x, y);
            } else {
                path.line_to(x, y);
            }
        }
        self.stroke(&path, stroke, width);
    }

    pub fn text(&mut self, x: f32, y: f32, text: &str, face: Face, size: f32, fill: Color, align: Align) {
        let mut paint = Paint::color(fill);
        if let Some(fonts) = &self.fonts {
            paint.set_font(&[match face {
                Face::Label | Face::Display => fonts.display,
                Face::Mono => fonts.mono,
            }]);
        }
        paint.set_font_size(size);
        paint.set_text_align(align);
        paint.set_text_baseline(Baseline::Middle);
        if let Face::Label = face {
            paint.set_letter_spacing(0.14 * size);
            let _ = self.canvas.fill_text(x, y, text.to_uppercase(), &paint);
        } else {
            let _ = self.canvas.fill_text(x, y, text, &paint);
        }
    }
}
