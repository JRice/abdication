//! The Windows scope's additive light (Jeremy, Nov 2026): each interrupt has
//! a softened cyan, magenta or yellow, after his CMY cube, lighter and less
//! saturated than pure. Where windows overlap their light adds up, as a
//! projector's does, so two make a pale mix and all three go toward white.
//! Everything is added in linear light and converted to sRGB only at the end.

/// The interrupts' pure hues: cyan, magenta, yellow.
const PURE: [[f32; 3]; 3] = [[0.0, 1.0, 1.0], [1.0, 0.0, 1.0], [1.0, 1.0, 0.0]];
/// The grey the hues are softened toward, and how saturated they stay.
const GREY: f32 = 0.667;
const SATURATION: f32 = 0.8;
/// How bright one window's fill is.
const FILL: f32 = 0.15;
/// The edge lines, and (dimmer) the row labels: the hue fully saturated.
const EDGE: f32 = 0.9;
const LABEL: f32 = 0.75;
/// Over this luminance (sRGB-weighted) a span draws its trace dark.
const INVERT: f32 = 0.5;

/// Interrupt `i`'s light at saturation `s`, linear.
fn light(i: usize, s: f32) -> [f32; 3] {
    PURE[i].map(|p| s * p + (1.0 - s) * GREY)
}

/// A shoulder so bright overlaps roll off instead of clipping (white at 1.5).
fn shoulder(x: f32) -> f32 {
    (x * (1.0 + x / 2.25) / (1.0 + x)).min(1.0)
}

fn to_srgb(x: f32) -> f32 {
    let x = x.clamp(0.0, 1.0);
    if x <= 0.003_130_8 { 12.92 * x } else { 1.055 * x.powf(1.0 / 2.4) - 0.055 }
}

fn from_srgb(x: f32) -> f32 {
    if x <= 0.040_45 { x / 12.92 } else { ((x + 0.055) / 1.055).powf(2.4) }
}

fn pack(rgb: [f32; 3]) -> u32 {
    let [r, g, b] = rgb.map(|c| (to_srgb(c) * 255.0).round() as u32);
    (r << 16) | (g << 8) | b
}

fn unpack(hex: u32) -> [f32; 3] {
    [hex >> 16, hex >> 8, hex].map(|c| from_srgb((c & 0xff) as f32 / 255.0))
}

/// Interrupt `i`'s edge lines (sRGB hex).
pub fn edge(i: usize) -> u32 {
    pack(light(i, (SATURATION + 0.3).min(1.0)).map(|c| c * EDGE))
}

/// Interrupt `i`'s row labels (sRGB hex).
pub fn label(i: usize) -> u32 {
    pack(light(i, (SATURATION + 0.3).min(1.0)).map(|c| c * LABEL))
}

/// A span's fill over `background` (sRGB hex) with the windows in `open`
/// (bit i for interrupt i) lit, and whether its trace draws dark.
pub fn fill(background: u32, open: u8) -> (u32, bool) {
    let mut sum = unpack(background);
    for i in (0..3).filter(|i| open & (1 << i) != 0) {
        let l = light(i, SATURATION);
        for c in 0..3 {
            sum[c] += FILL * l[c];
        }
    }
    let out = sum.map(shoulder);
    let [r, g, b] = out.map(to_srgb);
    (pack(out), 0.2126 * r + 0.7152 * g + 0.0722 * b > INVERT)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SCREEN: u32 = 0x1b1309;

    #[test]
    fn one_window_keeps_the_trace_light_three_invert_it() {
        for open in [0b001, 0b010, 0b100] {
            assert!(!fill(SCREEN, open).1, "{open:03b}: {:06x}", fill(SCREEN, open).0);
        }
        assert!(fill(SCREEN, 0b111).1, "{:06x}", fill(SCREEN, 0b111).0);
        // Pairs go by their brightness: green-heavy cyan + yellow is the brightest.
        let pairs = [0b011, 0b101, 0b110].map(|o| fill(SCREEN, o));
        println!("pairs {:06x?}", pairs);
    }

    #[test]
    fn overlaps_are_paler_and_brighter() {
        let luma = |hex: u32| unpack(hex).iter().sum::<f32>();
        let (one, _) = fill(SCREEN, 0b001);
        let (two, _) = fill(SCREEN, 0b011);
        let (three, _) = fill(SCREEN, 0b111);
        assert!(luma(one) < luma(two) && luma(two) < luma(three));
        // All three come out near grey-white: no channel far from the others.
        let [r, g, b] = unpack(three);
        assert!((r - g).abs() < 0.05 && (g - b).abs() < 0.05, "{three:06x}");
        // Cyan + magenta leans blue (a periwinkle).
        let [r, g, b] = unpack(two);
        assert!(b > r && r > g, "{two:06x}");
    }

    #[test]
    fn edges_and_labels_are_the_hues() {
        assert_eq!(edge(0) >> 16, 0);
        assert_eq!((edge(1) >> 8) & 0xff, 0);
        assert_eq!(label(2) & 0xff, 0);
        assert!(label(0) & 0xff < edge(0) & 0xff);
    }
}
