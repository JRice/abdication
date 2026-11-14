//! Drawing for the Fresco panel: its colours and faces, and a painter with the few
//! marks the panel is made of (plaster, cracks, gilding, grooves, text).

use std::cell::Cell;
use std::f32::consts::PI;

use nih_plug_vizia::vizia::prelude::Canvas;
use nih_plug_vizia::vizia::vg::{self, Align, Baseline, Color, Paint, Path};

pub const PLASTER: u32 = 0x15100c;
/// The stains on the plaster: damp, soot, old patching.
pub const STAINS: [u32; 4] = [0x22190f, 0x1f1712, 0x281e15, 0x0e0a07];
pub const CRACK: u32 = 0x0a0705;
pub const CRACK_EDGE: u32 = 0x2e241b;
pub const GROOVE: u32 = 0x0c0907;
pub const GROOVE_EDGE: u32 = 0x3a2e24;
pub const RULE: u32 = 0x2a2019;
pub const INK: u32 = 0xe4d6bb;
pub const DIM: u32 = 0xb8a888;
pub const FAINT: u32 = 0xa89a7f;
/// Controls that do nothing in the current mode.
pub const DORMANT: u32 = 0x6f6450;
pub const VALUE: u32 = 0xd6c8ab;
pub const GOLD: u32 = 0xc4a25a;
pub const GOLD_BRIGHT: u32 = 0xdcbd78;
pub const GILT: u32 = 0xa98845;
pub const BRASS: u32 = 0x7d6640;
pub const CHAIN: u32 = 0x6e5a3c;
pub const LAMP_DARK: u32 = 0x2a1512;
pub const LAMP_LIT: u32 = 0x8e1f18;
pub const FLAME_DARK: u32 = 0x3a2a20;
pub const FLAME: u32 = 0xffb15e;
pub const EMBER: u32 = 0xc83024;
pub const LIMIT: u32 = 0xa8362c;

static CINZEL: &[u8] = include_bytes!("../../assets/fonts/Cinzel.ttf");
static CINZEL_SEMIBOLD: &[u8] = include_bytes!("../../assets/fonts/CinzelSemiBold.ttf");
static CORMORANT_ITALIC: &[u8] = include_bytes!("../../assets/fonts/CormorantItalic.ttf");
static MONO: &[u8] = include_bytes!("../../assets/fonts/Mono.ttf");

pub fn color(hex: u32) -> Color {
    Color::rgb((hex >> 16) as u8, (hex >> 8) as u8, hex as u8)
}

pub fn faded(hex: u32, alpha: f32) -> Color {
    let mut c = color(hex);
    c.set_alphaf(alpha);
    c
}

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && y >= self.y && x < self.x + self.w && y < self.y + self.h
    }

    pub fn middle(&self) -> f32 {
        self.y + self.h / 2.0
    }
}

#[derive(Clone, Copy)]
pub struct Fonts {
    cinzel: vg::FontId,
    cinzel_semibold: vg::FontId,
    italic: vg::FontId,
    mono: vg::FontId,
}

thread_local! {
    /// Fonts are registered per canvas; the canvas address tells us when a reopened
    /// editor brought a new one.
    static FONTS: Cell<Option<(usize, Fonts)>> = const { Cell::new(None) };
}

pub fn fonts(canvas: &mut Canvas) -> Option<Fonts> {
    let key = canvas as *const Canvas as usize;
    if let Some((k, fonts)) = FONTS.get() {
        let mut probe = Paint::color(Color::white());
        probe.set_font(&[fonts.cinzel]);
        if k == key && canvas.measure_text(0.0, 0.0, "A", &probe).is_ok() {
            return Some(fonts);
        }
    }
    let fonts = Fonts {
        cinzel: canvas.add_font_mem(CINZEL).ok()?,
        cinzel_semibold: canvas.add_font_mem(CINZEL_SEMIBOLD).ok()?,
        italic: canvas.add_font_mem(CORMORANT_ITALIC).ok()?,
        mono: canvas.add_font_mem(MONO).ok()?,
    };
    FONTS.set(Some((key, fonts)));
    Some(fonts)
}

#[derive(Clone, Copy)]
pub enum Face {
    Cinzel,
    CinzelSemibold,
    Italic,
    Mono,
}

/// A face, its size, its colour and its letter spacing.
#[derive(Clone, Copy)]
pub struct TextStyle {
    pub face: Face,
    pub size: f32,
    pub ink: u32,
    pub spacing: f32,
}

impl TextStyle {
    pub const fn new(face: Face, size: f32, ink: u32, spacing: f32) -> Self {
        Self { face, size, ink, spacing }
    }

    pub const fn ink(self, ink: u32) -> Self {
        Self { ink, ..self }
    }
}

/// An irregular blotch on the wall, and its colour.
pub struct Stain {
    pub outline: Vec<(f32, f32)>,
    pub tint: Color,
}

/// The same stains every time, scattered over a `width` by `height` wall. Overlapping
/// blotches rather than gradients, which band in colours this dark.
pub fn stains(width: f32, height: f32) -> Vec<Stain> {
    const COUNT: usize = 90;
    const CORNERS: usize = 15;
    let mut seed = 0x2545_f491_u32;
    let mut next = move || {
        seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        (seed >> 8) as f32 / (1 << 24) as f32
    };
    (0..COUNT)
        .map(|i| {
            let (cx, cy, r) = (next() * width, next() * height, 20.0 + 100.0 * next());
            let outline = (0..CORNERS)
                .map(|k| {
                    let a = 2.0 * PI * (k as f32 + 0.4 * next()) / CORNERS as f32;
                    let reach = r * (0.8 + 0.4 * next());
                    (cx + reach * a.cos(), cy + reach * a.sin())
                })
                .collect();
            Stain { outline, tint: faded(STAINS[i % STAINS.len()], 0.12 + 0.2 * next()) }
        })
        .collect()
}

pub struct Painter<'a> {
    pub canvas: &'a mut Canvas,
    pub fonts: Option<Fonts>,
}

impl Painter<'_> {
    pub fn rect(&mut self, r: Rect, fill: u32) {
        let mut path = Path::new();
        path.rect(r.x, r.y, r.w, r.h);
        self.canvas.fill_path(&path, &Paint::color(color(fill)));
    }

    pub fn fill(&mut self, path: &Path, fill: Color) {
        self.canvas.fill_path(path, &Paint::color(fill));
    }

    pub fn stroke(&mut self, path: &Path, ink: Color, width: f32) {
        let mut paint = Paint::color(ink);
        paint.set_line_width(width);
        self.canvas.stroke_path(path, &paint);
    }

    /// A polyline through `points`.
    pub fn line(&mut self, points: &[(f32, f32)], ink: Color, width: f32) {
        let mut path = Path::new();
        for (i, (x, y)) in points.iter().enumerate() {
            if i == 0 {
                path.move_to(*x, *y);
            } else {
                path.line_to(*x, *y);
            }
        }
        self.stroke(&path, ink, width);
    }

    pub fn stain(&mut self, stain: &Stain) {
        let mut path = Path::new();
        for (i, (x, y)) in stain.outline.iter().enumerate() {
            if i == 0 {
                path.move_to(*x, *y);
            } else {
                path.line_to(*x, *y);
            }
        }
        path.close();
        self.fill(&path, stain.tint);
    }

    /// A soft patch of colour fading out from its centre, squashed to an ellipse.
    pub fn glow(&mut self, cx: f32, cy: f32, rx: f32, ry: f32, inner: Color) {
        let mut outer = inner;
        outer.set_alphaf(0.0);
        self.canvas.save();
        self.canvas.translate(cx, cy);
        self.canvas.scale(1.0, ry / rx);
        let mut path = Path::new();
        path.circle(0.0, 0.0, rx);
        self.canvas.fill_path(&path, &Paint::radial_gradient(0.0, 0.0, 0.0, rx, inner, outer));
        self.canvas.restore();
    }

    /// The upper half of a circle, as worn gilding: drawn where `pattern` says
    /// (alternating lengths along the arc, drawn first), left to right.
    pub fn worn_arc(&mut self, cx: f32, cy: f32, r: f32, pattern: &[f32], ink: Color, width: f32) {
        const STEP: f32 = 2.0;
        let length = PI * r;
        let (mut along, mut drawing, mut i) = (0.0, true, 0);
        while along < length {
            let end = (along + pattern[i % pattern.len()]).min(length);
            if drawing {
                let mut path = Path::new();
                let mut s = along;
                loop {
                    let a = PI + s / r;
                    let (x, y) = (cx + r * a.cos(), cy + r * a.sin());
                    if s == along {
                        path.move_to(x, y);
                    } else {
                        path.line_to(x, y);
                    }
                    if s >= end {
                        break;
                    }
                    s = (s + STEP).min(end);
                }
                self.stroke(&path, ink, width);
            }
            along = end;
            drawing = !drawing;
            i += 1;
        }
    }

    /// A slot cut into the plaster, with a gold tick at `value` (0–1), tarnished
    /// unless `lit`.
    pub fn groove(&mut self, track: Rect, value: f32, lit: bool) {
        let y = track.middle();
        self.rect(Rect { x: track.x, y: y - 1.5, w: track.w, h: 3.0 }, GROOVE);
        self.rect(Rect { x: track.x, y: y + 1.5, w: track.w, h: 1.0 }, GROOVE_EDGE);
        let tick = Rect { x: track.x + value * (track.w - 4.0), y: y - 7.0, w: 4.0, h: 14.0 };
        if lit {
            self.rect(tick, GOLD);
        } else {
            let mut path = Path::new();
            path.rect(tick.x, tick.y, tick.w, tick.h);
            self.fill(&path, faded(GOLD, 0.35));
        }
    }

    pub fn text(&mut self, x: f32, y: f32, text: &str, style: TextStyle, align: Align) {
        let mut paint = Paint::color(color(style.ink));
        if let Some(fonts) = &self.fonts {
            paint.set_font(&[match style.face {
                Face::Cinzel => fonts.cinzel,
                Face::CinzelSemibold => fonts.cinzel_semibold,
                Face::Italic => fonts.italic,
                Face::Mono => fonts.mono,
            }]);
        }
        paint.set_font_size(style.size);
        paint.set_letter_spacing(style.spacing);
        paint.set_text_align(align);
        paint.set_text_baseline(Baseline::Middle);
        let _ = self.canvas.fill_text(x, y, text, &paint);
    }

    /// Text sitting on a baseline, for the big numerals.
    pub fn text_on_baseline(&mut self, x: f32, y: f32, text: &str, style: TextStyle, align: Align) {
        let mut paint = Paint::color(color(style.ink));
        if let Some(fonts) = &self.fonts {
            paint.set_font(&[fonts.cinzel]);
        }
        paint.set_font_size(style.size);
        paint.set_text_align(align);
        paint.set_text_baseline(Baseline::Alphabetic);
        let _ = self.canvas.fill_text(x, y, text, &paint);
    }
}
