//! Renders the machine with no input, to raw stereo f32 at 48 kHz, for measuring.
//! cargo run --release --example render -- <out.f32> <seconds> [key=value ...]
use arecibo::dsp::engine::{Controls, Engine, StripKnobs};

const SR: f32 = 48_000.0;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (path, seconds) = (&args[1], args[2].parse::<f32>().unwrap());
    let mut c = Controls {
        strips: [
            StripKnobs { shift_hz: 1.3, peak_hz: 420.0, gain: 1.2 },
            StripKnobs { shift_hz: -0.7, peak_hz: 1_300.0, gain: 1.2 },
            StripKnobs { shift_hz: 3.1, peak_hz: 2_900.0, gain: 1.2 },
            StripKnobs { shift_hz: -1.9, peak_hz: 680.0, gain: 1.2 },
        ],
        feedback: 0.9,
        cross: 0.35,
        sky: 0.7,
        width_hz: 2_800.0,
        distance_s: 0.028,
        agc_release_s: 0.7,
        weather: 0.35,
        age: 0.4,
        input: 1.0,
        side_input: 1.0,
        dry: 0.0,
        level: 1.0,
    };
    // A low thump every two seconds, on the main input, the sidechain or neither.
    let (mut kick, mut side_kick) = (0.0f32, 0.0f32);
    for pair in &args[3..] {
        let (key, value) = pair.split_once('=').unwrap();
        let v: f32 = value.parse().unwrap();
        match key {
            "kick" => kick = v,
            "side_kick" => side_kick = v,
            "feedback" => c.feedback = v,
            "cross" => c.cross = v,
            "sky" => c.sky = v,
            "width" => c.width_hz = v,
            "distance" => c.distance_s = v,
            "agc" => c.agc_release_s = v,
            "weather" => c.weather = v,
            "age" => c.age = v,
            "gain" => c.strips.iter_mut().for_each(|s| s.gain = v),
            "peak" => c.strips.iter_mut().enumerate().for_each(|(i, s)| s.peak_hz = v * [1.0, 3.1, 6.9, 1.6][i]),
            "shift" => c.strips.iter_mut().enumerate().for_each(|(i, s)| s.shift_hz = v * [1.0, -0.54, 2.4, -1.5][i]),
            other => panic!("no such key: {other}"),
        }
    }
    let mut engine = Engine::new(SR);
    engine.set_controls(&c);
    let mut out = Vec::with_capacity((seconds * SR) as usize * 8);
    for n in 0..(seconds * SR) as usize {
        let t = (n as f32 / SR) % 2.0;
        let thump = ((t * 90.0 * std::f32::consts::TAU).sin() + 0.5 * (t * 1_100.0 * std::f32::consts::TAU).sin()) * (-t * 25.0).exp();
        let [l, r] = engine.process([kick * thump; 2], [side_kick * thump; 2], &c);
        out.extend_from_slice(&l.to_le_bytes());
        out.extend_from_slice(&r.to_le_bytes());
    }
    std::fs::write(path, out).unwrap();
}
