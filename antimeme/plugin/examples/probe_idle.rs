//! The default settings with little or no input: raw output for tools/idle_tones.py.
//!
//!   probe_idle <time> <whine> <out.f32> [filter] [absorb] [drone level]
use antimeme::dsp::engine::{Controls, Engine};
use std::f32::consts::TAU;
use std::fs;

const SR: f32 = 48_000.0;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let number = |i: usize, default: f32| args.get(i).map_or(default, |v| v.parse().unwrap());
    let c = Controls {
        time: number(1, 0.1),
        whine: number(2, 0.0),
        decay: 0.7,
        filter: number(4, 0.35),
        absorb: number(5, 0.35),
        blend: 1.0,
        level: 1.0,
        agitation_speed: 0.25,
        agitation_angle: 0.1,
        ..Default::default()
    };
    let drone = number(6, 0.0);
    let mut engine = Engine::new(SR);
    let mut out = Vec::new();
    for n in 0..(SR as usize * 30) {
        let t = n as f32 / SR;
        let x = drone * ((TAU * 110.0 * t).sin() + 0.5 * (TAU * 332.0 * t).sin());
        out.extend_from_slice(&engine.process([x, x], &c)[0].to_le_bytes());
    }
    fs::write(&args[3], out).unwrap();
}
