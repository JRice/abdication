//! The engraved map's inks, its two faces, the tiles its sea and land are hatched with,
//! and a few marks drawn everywhere (halos, dashed ropes, chamfered panels).

use std::cell::Cell;

use imgref::Img;
use nih_plug_vizia::vizia::prelude::Canvas;
use nih_plug_vizia::vizia::vg::{self, Color, ImageFlags, ImageId, LineCap, LineJoin, Paint, Path};
use rgb::RGBA8;

pub const GROUND: u32 = 0x16110c;
pub const INK: u32 = 0xb08a55;
pub const GOLD: u32 = 0xbf9a60;
pub const CREAM: u32 = 0xecdfc2;
pub const PALE: u32 = 0xf2e4c2;
pub const TITLE: u32 = 0xe2d2aa;
pub const FACE: u32 = 0x1c1611;
pub const BALL: u32 = 0xdcc79c;
pub const ROPE: u32 = 0x977850;
pub const SHIP: u32 = 0xcdb084;
/// The modulators' colours: silver, red, green, slate.
pub const MODS: [u32; 4] = [0xc9cdd3, 0xc95c3d, 0x76aa84, 0x8597b0];

// Argent's copy of IM Fell English SC (OFL; its licence is beside it there).
static FELL: &[u8] = include_bytes!("../../../../argent/plugin/assets/fonts/FellSC.ttf");

pub fn color(hex: u32) -> Color {
    Color::rgb((hex >> 16) as u8, (hex >> 8) as u8, hex as u8)
}

pub fn faded(hex: u32, alpha: f32) -> Color {
    let mut c = color(hex);
    c.set_alphaf(alpha.clamp(0.0, 1.0));
    c
}

/// A stroke paint: colour, opacity, width, rounded.
pub fn pen(hex: u32, alpha: f32, width: f32) -> Paint {
    let mut p = Paint::color(faded(hex, alpha));
    p.set_line_width(width);
    p.set_line_cap(LineCap::Round);
    p.set_line_join(LineJoin::Round);
    p
}

pub fn fill(hex: u32, alpha: f32) -> Paint {
    Paint::color(faded(hex, alpha))
}

pub fn circle(x: f32, y: f32, r: f32) -> Path {
    let mut p = Path::new();
    p.circle(x, y, r);
    p
}

/// What each canvas needs made once: the faces and the tiles.
#[derive(Clone, Copy)]
pub struct Kit {
    pub fell: vg::FontId,
    pub waves: ImageId,
    pub hatch: ImageId,
}

thread_local! {
    static KIT: Cell<Option<(usize, Kit)>> = const { Cell::new(None) };
}

/// How finely the tiles are drawn, per logical pixel.
const TILE_SCALE: usize = 4;

/// A tile `w` x `h` (logical px) of `ink`, opaque where `distance` (logical px from the
/// mark) is under `half` wide.
fn tile(w: f32, h: f32, ink: u32, alpha: f32, half: f32, distance: impl Fn(f32, f32) -> f32) -> Vec<RGBA8> {
    let (pw, ph) = ((w * TILE_SCALE as f32) as usize, (h * TILE_SCALE as f32) as usize);
    let s = TILE_SCALE as f32;
    let mut px = Vec::with_capacity(pw * ph);
    for j in 0..ph {
        for i in 0..pw {
            let (x, y) = ((i as f32 + 0.5) / s, (j as f32 + 0.5) / s);
            let d = distance(x, y);
            let a = ((half + 0.5 / s - d) * s).clamp(0.0, 1.0) * alpha;
            px.push(RGBA8::new((ink >> 16) as u8, (ink >> 8) as u8, ink as u8, (a * 255.0).round() as u8));
        }
    }
    px
}

/// The sea's engraved swell: "M0 4 q6.5 -3.4 13 0 t13 0", 26 x 8, repeating.
fn waves() -> (Vec<RGBA8>, usize, usize) {
    let curve: Vec<(f32, f32)> = (0..=200)
        .map(|k| {
            let t = k as f32 / 100.0;
            let (u, sign) = if t <= 1.0 { (t, 1.0) } else { (t - 1.0, -1.0) };
            let x0 = if t <= 1.0 { 0.0 } else { 13.0 };
            let y = 4.0 - sign * 2.0 * u * (1.0 - u) * 3.4;
            (x0 + 13.0 * u, y)
        })
        .collect();
    let px = tile(26.0, 8.0, 0x86683f, 0.42, 0.375, |x, y| {
        curve.iter().map(|&(cx, cy)| {
            let dx = (x - cx).abs().min(26.0 - (x - cx).abs());
            (dx * dx + (y - cy) * (y - cy)).sqrt()
        }).fold(f32::MAX, f32::min)
    });
    (px, 26 * TILE_SCALE, 8 * TILE_SCALE)
}

/// The land's hatching: a line every 5 px (the pattern is turned 35° when drawn).
fn hatch() -> (Vec<RGBA8>, usize, usize) {
    let px = tile(5.0, 5.0, 0x7a6040, 0.45, 0.3, |x, _| x.min(5.0 - x));
    (px, 5 * TILE_SCALE, 5 * TILE_SCALE)
}

pub fn kit(canvas: &mut Canvas) -> Option<Kit> {
    let key = canvas as *const Canvas as usize;
    if let Some((k, kit)) = KIT.get() {
        if k == key && canvas.image_info(kit.waves).is_ok() {
            return Some(kit);
        }
    }
    let repeat = ImageFlags::REPEAT_X | ImageFlags::REPEAT_Y;
    let make = |canvas: &mut Canvas, (px, w, h): (Vec<RGBA8>, usize, usize)| canvas.create_image(Img::new(px.as_slice(), w, h), repeat).ok();
    let kit = Kit {
        fell: canvas.add_font_mem(FELL).ok()?,
        waves: make(canvas, waves())?,
        hatch: make(canvas, hatch())?,
    };
    KIT.set(Some((key, kit)));
    Some(kit)
}

/// A soft dark pool under something (the mockup's halos).
pub fn halo(canvas: &mut Canvas, x: f32, y: f32, r: f32) {
    let paint = Paint::radial_gradient_stops(x, y, 0.0, r, [(0.45, faded(0, 0.62)), (1.0, faded(0, 0.0))]);
    canvas.fill_path(&circle(x, y, r), &paint);
}

/// A dashed stroke along points: `on` drawn, `off` skipped.
pub fn dashed(points: &[(f32, f32)], on: f32, off: f32) -> Path {
    let mut p = Path::new();
    let mut drawing = true;
    let mut left = on;
    for w in points.windows(2) {
        let ((mut x0, mut y0), (x1, y1)) = (w[0], w[1]);
        let mut seg = ((x1 - x0).powi(2) + (y1 - y0).powi(2)).sqrt();
        while seg > 0.0 {
            let step = left.min(seg);
            let (nx, ny) = (x0 + (x1 - x0) * step / seg, y0 + (y1 - y0) * step / seg);
            if drawing {
                p.move_to(x0, y0);
                p.line_to(nx, ny);
            }
            (x0, y0) = (nx, ny);
            seg -= step;
            left -= step;
            if left <= 0.0 {
                drawing = !drawing;
                left = if drawing { on } else { off };
            }
        }
    }
    p
}

/// The mockup's chamfered panel.
pub fn chamfer(x0: f32, y0: f32, x1: f32, y1: f32, c: f32) -> Path {
    let mut p = Path::new();
    p.move_to(x0 + c, y0);
    p.line_to(x1 - c, y0);
    p.line_to(x1, y0 + c);
    p.line_to(x1, y1 - c);
    p.line_to(x1 - c, y1);
    p.line_to(x0 + c, y1);
    p.line_to(x0, y1 - c);
    p.line_to(x0, y0 + c);
    p.close();
    p
}
