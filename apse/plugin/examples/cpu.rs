//! How much of a core the heaviest settings take, at 48 kHz. cargo run --release --example cpu
use std::time::Instant;

use apse::dsp::engine::{Controls, Engine, SIZE_MAX};

fn main() {
    const SR: f32 = 48_000.0;
    let c = Controls { mix: 1.0, size: SIZE_MAX, modulation: 1.0, time_s: 10.0, ..Default::default() };
    let mut engine = Engine::new(SR);
    engine.set_mode(5); // Choir: the tank and six singers
    let seconds = 30.0;
    let start = Instant::now();
    let mut sum = 0.0;
    for n in 0..(seconds * SR) as usize {
        let x = (n as f32 * 0.05).sin() * 0.5;
        sum += engine.process([x, x], &c)[0];
    }
    println!("{:.2} % of a core (checksum {sum})", 100.0 * start.elapsed().as_secs_f32() / seconds);
}
