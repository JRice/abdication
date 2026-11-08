//! A chip alone, under a sine and after it stops: how much noise, by band?
use antimeme::dsp::chip::{Chip, BITS};
use std::f32::consts::TAU;
const SR: f32 = 96_000.0;

fn band_rms(y: &[f32], lo: f32, hi: f32) -> f32 {
    // A plain DFT over a coarse grid of bins is enough here.
    let n = 1 << 14;
    let seg = &y[y.len() - n..];
    let mut power = 0.0;
    let mut k = (lo / SR * n as f32) as usize;
    while (k as f32) < hi / SR * n as f32 {
        let w = TAU * k as f32 / n as f32;
        let (mut re, mut im) = (0.0f32, 0.0f32);
        for (i, v) in seg.iter().enumerate() {
            let win = 0.5 - 0.5 * (TAU * i as f32 / n as f32).cos();
            re += v * win * (w * i as f32).cos();
            im += v * win * (w * i as f32).sin();
        }
        power += re * re + im * im;
        k += 1;
    }
    20.0 * (2.0 * (power / (0.375 * n as f32)).sqrt() / (n as f32).sqrt() + 1e-9).log10()
}

fn main() {
    for delay in [0.036, 0.098, 0.197, 0.545, 1.2, 3.1] {
        let rate = BITS as f32 / delay;
        for (label, amp) in [("sine", 0.25f32), ("idle", 0.0)] {
            let mut chip = Chip::new(1);
            let total = (delay + 0.5) * SR;
            let y: Vec<f32> = (0..total as usize)
                .map(|n| chip.process(amp * (TAU * 1000.0 * n as f32 / SR).sin(), rate, 1.0 / SR, 0.0))
                .collect();
            println!("{delay:6.3}s {label}  3-6k {:6.1}  6-12k {:6.1}  12-20k {:6.1}", band_rms(&y, 3000.0, 6000.0), band_rms(&y, 6000.0, 12000.0), band_rms(&y, 12000.0, 20000.0));
        }
    }
}
