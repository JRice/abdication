//! Renders a held chord to a 16-bit stereo WAV at 48 kHz, for measuring and listening,
//! and reports how long the bank and the audio took.
//!
//! cargo run --release --example render -- <out.wav> <seconds> [key=value ...]
//! keys: notes=33,38,45,52  seed  morph  scan  span  tone  fizz  tide  flare  depth
//!       bloom  tremor  wander  fade  curve  coarse=1  glide=1  release_at=<seconds>
use std::time::Instant;

use argent::dsp::engine::{Controls, Engine};
use argent::dsp::tables::Bank;

const SR: f32 = 48_000.0;

fn wav(path: &str, samples: &[[f32; 2]]) {
    let data_len = (samples.len() * 4) as u32;
    let mut out = Vec::with_capacity(44 + data_len as usize);
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&(36 + data_len).to_le_bytes());
    out.extend_from_slice(b"WAVEfmt ");
    out.extend_from_slice(&16u32.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&2u16.to_le_bytes());
    out.extend_from_slice(&(SR as u32).to_le_bytes());
    out.extend_from_slice(&(SR as u32 * 4).to_le_bytes());
    out.extend_from_slice(&4u16.to_le_bytes());
    out.extend_from_slice(&16u16.to_le_bytes());
    out.extend_from_slice(b"data");
    out.extend_from_slice(&data_len.to_le_bytes());
    for [l, r] in samples {
        for x in [l, r] {
            out.extend_from_slice(&((x.clamp(-1.0, 1.0) * 32_767.0) as i16).to_le_bytes());
        }
    }
    std::fs::write(path, out).unwrap();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (path, seconds) = (&args[1], args[2].parse::<f32>().unwrap());
    let mut c = Controls::default();
    let (mut notes, mut seed, mut release_at) = (vec![33u8, 38, 45, 52], 417u32, f32::INFINITY);
    for pair in &args[3..] {
        let (key, value) = pair.split_once('=').unwrap();
        let v = || value.parse::<f32>().unwrap();
        match key {
            "notes" => notes = value.split(',').map(|n| n.parse().unwrap()).collect(),
            "seed" => seed = value.parse().unwrap(),
            "morph" => c.morph = v(),
            "scan" => c.scan = v() - 1.0,
            "span" => c.span = v(),
            "tone" => c.tone_hz = v(),
            "fizz" => c.fizz = 10f32.powf(v() / 20.0),
            "tide" => c.tide = v(),
            "flare" => c.flare_s = v(),
            "depth" => c.depth_oct = v(),
            "bloom" => c.bloom = v(),
            "tremor" => c.tremor = v(),
            "wander" => c.wander_cents = v(),
            "fade" => c.fade_s = v(),
            "curve" => c.curve = v(),
            "coarse" => c.reading.coarse = v() > 0.5,
            "glide" => c.reading.stepped = v() < 0.5,
            "release_at" => release_at = v(),
            other => panic!("no such key: {other}"),
        }
    }
    let started = Instant::now();
    let bank = Bank::build(seed);
    let built = started.elapsed();
    let mut engine = Engine::new(SR, bank);
    for key in &notes {
        engine.press(*key);
        engine.lift(*key);
    }
    let started = Instant::now();
    let n = (seconds * SR) as usize;
    let mut samples = Vec::with_capacity(n);
    for i in 0..n {
        if i == (release_at * SR) as usize {
            engine.release();
        }
        samples.push(engine.process(&c));
    }
    let took = started.elapsed();
    eprintln!(
        "bank {:.0} ms; audio {:.2} s for {seconds} s ({:.1} % of a core); reduction now {:.1} dB",
        built.as_secs_f32() * 1000.0,
        took.as_secs_f32(),
        100.0 * took.as_secs_f32() / seconds,
        engine.reduction_db()
    );
    wav(path, &samples);
}
