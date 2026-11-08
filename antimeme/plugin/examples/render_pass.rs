//! Plays the capture's test signal through the model with the same knob moves as
//! the Strega got, so the same analysis can run on both.
//!
//!   render_pass <signal.f32> <moves.csv> <out.f32>
//!
//! moves.csv: seconds,blend,time,decay,filter,absorb,strength (knobs 0..1).
use antimeme::dsp::engine::{Controls, Engine};
use std::fs;

const SR: f32 = 48_000.0;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let bytes = fs::read(&args[1]).unwrap();
    let signal: Vec<f32> = bytes.chunks_exact(4).map(|b| f32::from_le_bytes(b.try_into().unwrap())).collect();
    let moves: Vec<Vec<f32>> = fs::read_to_string(&args[2]).unwrap().lines()
        .map(|l| l.split(',').map(|v| v.parse().unwrap()).collect()).collect();

    let mut engine = Engine::new(SR);
    let mut controls = Controls { level: 1.0, ..Default::default() };
    let mut smoothed = [0.0f32; 6];
    let mut next = 0;
    let mut target = moves[0][1..].to_vec();
    smoothed.copy_from_slice(&target);
    let smooth = 1.0 - (-1.0 / (0.05 * SR)).exp();

    let mut out = Vec::with_capacity(signal.len() * 4);
    for (n, &x) in signal.iter().enumerate() {
        let t = n as f32 / SR;
        while next < moves.len() && moves[next][0] <= t {
            target = moves[next][1..].to_vec();
            next += 1;
        }
        for (s, g) in smoothed.iter_mut().zip(&target) {
            *s += (g - *s) * smooth;
        }
        controls.blend = smoothed[0];
        controls.time = smoothed[1];
        controls.decay = smoothed[2];
        controls.filter = smoothed[3];
        controls.absorb = smoothed[4];
        controls.strength = smoothed[5];
        // The recording's unit: the Strega's output and our dry signal are 6 dB apart.
        let y = engine.process([x, x], &controls)[0] * 0.5;
        out.extend_from_slice(&y.to_le_bytes());
    }
    fs::write(&args[3], out).unwrap();
}
