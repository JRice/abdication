//! Numeric checks of the chip model, printed as JSON lines.
use antimeme::dsp::chip::{Chip, BITS};
use std::f32::consts::TAU;

const SR: f32 = 96_000.0;

fn run(delay: f32, seconds: f32, input: impl Fn(f32) -> f32) -> Vec<f32> {
    let mut chip = Chip::new(1);
    let rate = BITS as f32 / delay;
    let dt = 1.0 / SR;
    (0..(seconds * SR) as usize).map(|n| chip.process(input(n as f32 * dt), rate, dt, 0.0)).collect()
}

fn rms(x: &[f32]) -> f32 {
    (x.iter().map(|v| v * v).sum::<f32>() / x.len() as f32).sqrt()
}

fn db(x: f32) -> f32 {
    20.0 * x.max(1e-9).log10()
}

/// One-pole cascade, to look at the audio band only.
fn band(x: &[f32], hz: f32) -> Vec<f32> {
    let a = (-TAU * hz / SR).exp();
    let (mut s1, mut s2) = (0.0, 0.0);
    x.iter().map(|&v| { s1 = v + (s1 - v) * a; s2 = s1 + (s2 - s1) * a; s2 }).collect()
}

fn main() {
    println!("-- delay accuracy: burst position");
    for delay in [0.031, 0.1, 0.34, 1.0, 5.0] {
        let total = delay * 1.5 + 0.2;
        let y = run(delay, total, |t| if t < 0.01 { 0.5 * (TAU * 800.0 * t).sin() } else { 0.0 });
        // The burst is 10 ms at 800 Hz: correlate, since long delays bury it in noise.
        let len = (0.01 * SR) as usize;
        let template: Vec<f32> = (0..len).map(|n| (TAU * 800.0 * n as f32 / SR).sin()).collect();
        let (mut best, mut best_at) = (0.0, 0);
        for at in 0..y.len() - len {
            let score: f32 = template.iter().zip(&y[at..]).map(|(a, b)| a * b).sum();
            if score > best { best = score; best_at = at; }
        }
        println!("delay {delay:6.3}s  burst found at {:.4}s", best_at as f32 / SR);
    }

    println!("-- idle noise, input silent (dB re full scale)");
    for delay in [0.031, 0.1, 0.34, 1.0, 3.0, 10.0, 30.0] {
        let y = run(delay, delay.max(1.0) + 1.0, |_| 0.0);
        let tail = &y[y.len() - SR as usize / 2..];
        println!("delay {delay:6.3}s  bit rate {:9.0}  wide {:6.1}  below 8 kHz {:6.1}", BITS as f32 / delay, db(rms(tail)), db(rms(&band(tail, 8000.0))));
    }

    println!("-- output/input by frequency and level, 100 ms delay");
    for hz in [100.0, 300.0, 1000.0, 3000.0, 8000.0] {
        let mut row = format!("{hz:6.0} Hz:");
        for level_db in [-30.0_f32, -18.0, -6.0, 0.0, 6.0] {
            let amp = 10f32.powf(level_db / 20.0);
            let y = run(0.1, 0.6, |t| amp * (TAU * hz * t).sin());
            let tail = &y[y.len() - SR as usize / 4..];
            row += &format!("  {:6.1}", db(rms(tail) / (amp / 2f32.sqrt())));
        }
        println!("{row}");
    }

    println!("-- repitching: a sine in RAM, then the clock halves");
    let mut chip = Chip::new(1);
    let dt = 1.0 / SR;
    let rate = BITS as f32 / 0.5;
    let mut crossings_after = 0;
    let mut prev = 0.0;
    let (mut s1, mut s2) = (0.0_f32, 0.0_f32);
    let a = (-TAU * 1000.0 / SR).exp();
    for n in 0..(SR * 0.9) as usize {
        let t = n as f32 * dt;
        let (input, r) = if t < 0.5 { ((TAU * 500.0 * t).sin() * 0.3, rate) } else { (0.0, rate / 2.0) };
        let raw = chip.process(input, r, dt, 0.0);
        s1 = raw + (s1 - raw) * a;
        s2 = s1 + (s2 - s1) * a;
        let y = s2;
        if t > 0.55 && t < 0.85 && prev <= 0.0 && y > 0.0 {
            crossings_after += 1;
        }
        prev = y;
    }
    println!("500 Hz written, clock halved, reads as about {:.0} Hz", crossings_after as f32 / 0.3);

    println!("-- speed");
    let mut chip = Chip::new(1);
    let start = std::time::Instant::now();
    let dt = 1.0 / SR;
    let mut sum = 0.0;
    for n in 0..SR as usize {
        sum += chip.process((n as f32 * 0.01).sin() * 0.3, BITS as f32 / 0.031, dt, 0.0);
    }
    println!("1 s at the fastest clock took {:?} (sum {sum:.1})", start.elapsed());
}
