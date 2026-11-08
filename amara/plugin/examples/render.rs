//! Renders a scene to raw stereo f32 at 48 kHz, for measuring.
//! cargo run --release --example render -- <out.f32> <seconds> [key=value ...]
use amara::dsp::engine::{Controls, Engine};
use amara::dsp::speaker::SpeakerKnobs;

const SR: f32 = 48_000.0;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (path, seconds) = (&args[1], args[2].parse::<f32>().unwrap());
    let mut c = Controls {
        speaker: SpeakerKnobs { rate: 5.0, sparsity: 0.2, temper: 0.45, swing: 0.5, breath: 0.1, tract: 0.85, pitch_hz: 90.0, per_note: false },
        language: 13,
        strangeness: 0.4,
        possession: 0.0,
        extremity: 0.6,
        seized: false,
        line: 0.0,
        fidelity: 0.55,
    };
    for pair in &args[3..] {
        let (key, value) = pair.split_once('=').unwrap();
        let v: f32 = value.parse().unwrap();
        match key {
            "rate" => c.speaker.rate = v,
            "sparsity" => c.speaker.sparsity = v,
            "temper" => c.speaker.temper = v,
            "swing" => c.speaker.swing = v,
            "breath" => c.speaker.breath = v,
            "tract" => c.speaker.tract = v,
            "pitch" => c.speaker.pitch_hz = v,
            "language" => c.language = v as u32,
            "strangeness" => c.strangeness = v,
            "possession" => c.possession = v,
            "extremity" => c.extremity = v,
            "seized" => c.seized = v > 0.5,
            "line" => c.line = v,
            "fidelity" => c.fidelity = v,
            other => panic!("no such key: {other}"),
        }
    }
    let mut engine = Engine::new(SR, (c.language, c.strangeness));
    engine.set_controls(&c);
    let mut out = Vec::new();
    for _ in 0..(seconds * SR) as usize {
        let [l, r] = engine.process(&c, 1.0);
        out.extend_from_slice(&l.to_le_bytes());
        out.extend_from_slice(&r.to_le_bytes());
    }
    std::fs::write(path, out).unwrap();
    println!("{}", engine.reduction_db());
}
