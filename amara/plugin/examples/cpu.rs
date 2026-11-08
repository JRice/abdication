//! How much of a core the worst settings take, at 48 kHz. cargo run --release --example cpu
use std::time::Instant;

use amara::dsp::engine::{Controls, Engine};
use amara::dsp::speaker::SpeakerKnobs;

fn main() {
    const SR: f32 = 48_000.0;
    let c = Controls {
        speaker: SpeakerKnobs { rate: 16.0, sparsity: 0.0, temper: 1.0, swing: 1.0, breath: 0.5, tract: 1.0, pitch_hz: 200.0, per_note: false },
        language: 13,
        strangeness: 0.4,
        possession: 1.0,
        extremity: 1.0,
        seized: true,
        line: 1.0,
        fidelity: 1.0,
    };
    let mut engine = Engine::new(SR, (c.language, c.strangeness));
    engine.set_controls(&c);
    let seconds = 30.0;
    let start = Instant::now();
    let mut sum = 0.0;
    for _ in 0..(seconds * SR) as usize {
        sum += engine.process(&c, 1.0)[0];
    }
    println!("{:.2} % of a core (checksum {sum})", 100.0 * start.elapsed().as_secs_f32() / seconds);
}
