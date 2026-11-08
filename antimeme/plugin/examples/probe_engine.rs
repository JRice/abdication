//! Runs the whole effect headless: sanity and CPU checks at the extremes.
use antimeme::dsp::engine::{Controls, Engine};
use std::f32::consts::TAU;

const SR: f32 = 48_000.0;

fn base() -> Controls {
    Controls { time: 0.5, decay: 0.3, filter: 1.0, blend: 1.0, level: 1.0, agitation_speed: 0.3, agitation_angle: 0.5, ..Default::default() }
}

fn rms_peak(c: &Controls, seconds: f32, burst: bool) -> (f32, f32) {
    let mut engine = Engine::new(SR);
    let (mut energy, mut peak, mut count) = (0.0_f32, 0.0_f32, 0);
    for n in 0..(seconds * SR) as usize {
        let t = n as f32 / SR;
        let x = if burst && t < 0.2 { 0.5 * (TAU * 440.0 * t).sin() } else { 0.0 };
        let [l, r] = engine.process([x, x], c);
        assert!(l.is_finite() && r.is_finite());
        if t > seconds * 0.5 {
            energy += l * l;
            peak = peak.max(l.abs());
            count += 1;
        }
    }
    (10.0 * (energy / count as f32).max(1e-12).log10(), peak)
}

fn report(name: &str, c: Controls, seconds: f32) {
    let (rms, peak) = rms_peak(&c, seconds, true);
    println!("{name:34} tail rms {rms:7.1} dB  peak {peak:5.2}");
}

fn main() {
    report("fast, low decay", Controls { time: 1.0, ..base() }, 4.0);
    report("mid, low decay", base(), 6.0);
    report("long, low decay", Controls { time: 0.2, ..base() }, 20.0);
    report("mid, unity decay", Controls { decay: 0.8, ..base() }, 20.0);
    report("mid, full decay", Controls { decay: 1.0, ..base() }, 20.0);
    report("fast, full decay", Controls { time: 1.0, decay: 1.0, ..base() }, 10.0);
    report("full decay, dark", Controls { decay: 1.0, filter: 0.3, ..base() }, 20.0);
    report("everything moving", Controls {
        decay: 0.9, agitation_speed: 0.7, agitation_to_time: 0.6, agitation_to_filter: -0.4, agitation_to_decay: 0.1, agitation_to_whine: 0.5,
        interference_to_time: 0.5, interference_to_filter: 0.5, interference_to_decay: 0.2, interference_to_whine: 0.5, spread: 1.0, ..base()
    }, 20.0);

    println!("-- the limiter: a 440 Hz tone through at rising Level");
    for level_db in [0.0f32, 6.0, 12.0] {
        let c = Controls { level: 10f32.powf(level_db / 20.0), ..base() };
        let mut engine = Engine::new(SR);
        let (mut deepest, mut peak) = (0.0f32, 0.0f32);
        for n in 0..(SR as usize * 2) {
            let x = 0.5 * (TAU * 440.0 * n as f32 / SR).sin();
            let [l, _] = engine.process([x, x], &c);
            deepest = deepest.max(engine.reduction_db());
            peak = peak.max(l.abs());
        }
        println!("Level +{level_db:2.0} dB: deepest limiting {deepest:5.1} dB, output peak {peak:.3}");
    }

    let mut engine = Engine::new(SR);
    let c = Controls { time: 1.0, spread: 1.0, ..base() };
    let start = std::time::Instant::now();
    let mut sum = 0.0;
    for n in 0..SR as usize {
        let x = (n as f32 * 0.05).sin() * 0.3;
        sum += engine.process([x, x], &c)[0];
    }
    println!("worst case, stereo at the fastest clock: {:.1} % of a core ({sum:.2})", start.elapsed().as_secs_f32() * 100.0);
}
