//! How much of a core the worst settings take, at 48 kHz. cargo run --release --example cpu
use std::time::Instant;

use arecibo::dsp::engine::{Controls, Engine, StripKnobs};

fn main() {
    const SR: f32 = 48_000.0;
    let strip = StripKnobs { shift_hz: 20.0, peak_hz: 1_000.0, gain: 4.0 };
    let c = Controls {
        strips: [strip; 4],
        feedback: 1.6,
        cross: 0.5,
        sky: 0.5,
        width_hz: 12_000.0,
        distance_s: 0.16,
        agc_release_s: 0.2,
        weather: 1.0,
        age: 1.0,
        input: 1.0,
        side_input: 1.0,
        dry: 0.5,
        level: 1.0,
    };
    let mut engine = Engine::new(SR);
    let seconds = 30.0;
    let start = Instant::now();
    let mut sum = 0.0;
    for n in 0..(seconds * SR) as usize {
        if n % 512 == 0 {
            engine.set_controls(&c);
        }
        let x = (n as f32 * 0.05).sin() * 0.3;
        sum += engine.process([x, x], [-x, x], &c)[0];
    }
    println!("{:.2} % of a core (checksum {sum})", 100.0 * start.elapsed().as_secs_f32() / seconds);
}
