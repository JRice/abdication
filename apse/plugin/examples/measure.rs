//! Measures every mode on a short noise burst: decay time (to -40 dB, as Time is set), loudness against the
//! input, and how alike the two sides are. cargo run --release --example measure
use apse::dsp::engine::{Controls, Engine};
use apse::dsp::modes::MODES;

const SR: f32 = 48_000.0;

fn noise(seed: &mut u32) -> f32 {
    *seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
    (*seed >> 9) as f32 / (1 << 22) as f32 - 1.0
}

fn db(x: f32) -> f32 {
    10.0 * x.max(1e-24).log10()
}

fn main() {
    let time_s: f32 = std::env::args().nth(1).map_or(2.0, |s| s.parse().unwrap());
    let c = Controls { mix: 1.0, time_s, predelay_s: 0.0, ..Default::default() };
    println!("Time {time_s} s, Damp {:.0} %; burst of 50 ms noise at -12 dBFS RMS-ish", c.damp * 100.0);
    println!("{:<10} {:>8} {:>8} {:>9} {:>8} {:>7}", "mode", "T40 s", "T20 s", "level dB", "peak dB", "corr");
    for (m, mode) in MODES.iter().enumerate() {
        let mut engine = Engine::new(SR);
        engine.set_mode(m);
        for _ in 0..2_400 {
            engine.process([0.0; 2], &c);
        }
        let n_total = (SR * (time_s * 1.5 + 0.5)) as usize;
        let burst = (SR * 0.05) as usize;
        let (mut seed, mut input_energy) = (7u32, 0.0f64);
        let mut out = Vec::with_capacity(n_total);
        for n in 0..n_total {
            let x = if n < burst { 0.25 * noise(&mut seed) } else { 0.0 };
            input_energy += (x * x) as f64;
            out.push(engine.process([x, x], &c));
        }
        // Schroeder backward integration of the energy.
        let energy: Vec<f64> = out.iter().map(|[l, r]| (l * l + r * r) as f64 * 0.5).collect();
        let mut edc = vec![0.0f64; energy.len()];
        let mut acc = 0.0;
        for i in (0..energy.len()).rev() {
            acc += energy[i];
            edc[i] = acc;
        }
        let total = edc[0];
        let first = |target: f32| edc.iter().position(|e| db((*e / total) as f32) < target).map(|i| i as f32 / SR);
        let t20 = match (first(-5.0), first(-25.0)) {
            (Some(a), Some(b)) => 2.0 * (b - a),
            _ => f32::NAN,
        };
        let t40 = first(-40.0).unwrap_or(f32::NAN);
        let peak = out.iter().flat_map(|s| s.iter()).fold(0.0f32, |a, x| a.max(x.abs()));
        let (mut lr, mut ll, mut rr) = (0.0f64, 0.0f64, 0.0f64);
        for [l, r] in &out[(SR * 0.2) as usize..(SR * 1.0) as usize] {
            lr += (l * r) as f64;
            ll += (l * l) as f64;
            rr += (r * r) as f64;
        }
        let level = db((total / input_energy) as f32);
        println!(
            "{:<10} {:>8.2} {:>8.2} {:>9.1} {:>8.1} {:>7.2}",
            mode.name,
            t40,
            t20,
            level,
            20.0 * peak.log10(),
            lr / (ll * rr).sqrt()
        );
    }
}
