//! How much of a core the worst settings take, at 48 kHz. cargo run --release --example cpu
use std::time::Instant;

use anathema::dsp::engine::{Controls, Engine, MAX_HZ};
use anathema::dsp::machine::Knobs;

fn main() {
    const SR: f32 = 48_000.0;
    let mut knobs = Knobs { mix: 1.0, decay: 1.0, contention: 0.5, rot: 1.0, ..Default::default() };
    knobs.matrix = [true; 16];
    let controls = Controls { clock_hz: MAX_HZ, drive: 1.0, direct: 0.0, whine: 1.0, level: 1.0 };
    let mut engine = Engine::new(SR);
    engine.set_knobs(&knobs);
    let seconds = 30.0;
    let start = Instant::now();
    let mut sum = 0.0;
    for n in 0..(seconds * SR) as usize {
        let x = (n as f32 * 0.05).sin() * 0.5;
        sum += engine.process([x, x], &controls)[0];
    }
    println!("{:.2} % of a core (checksum {sum})", 100.0 * start.elapsed().as_secs_f32() / seconds);
}
