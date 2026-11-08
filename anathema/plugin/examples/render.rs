//! Renders a scene to raw stereo f32 at 48 kHz, for measuring against the recording.
//! cargo run --release --example render -- <scene> <out.f32>
use anathema::dsp::engine::{Controls, Engine};
use anathema::dsp::machine::Knobs;

const SR: f32 = 48_000.0;

/// A plucky arpeggio like the one on the recording.
fn arp(n: usize) -> f32 {
    let t = n as f32 / SR;
    let step = (t * 6.0) as usize;
    let notes = [220.0, 330.0, 440.0, 494.0, 659.0, 440.0, 330.0, 247.0];
    let freq = notes[step % notes.len()];
    let local = t - step as f32 / 6.0;
    let saw = 2.0 * (local * freq).fract() - 1.0;
    0.4 * saw * (-local * 18.0).exp()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (scene, path) = (args[1].as_str(), &args[2]);
    let mut knobs = Knobs { mix: 1.0, decay: 0.75, damp: 0.3, predelay: 0.1, ..Default::default() };
    let mut clock_hz = 7_797.0;
    let mut seconds = 24.0;
    let mut input_on = true;
    let mut whine = 0.0;
    let mut contention = 0.0;
    let mut freeze_at = f32::INFINITY;
    let mut sweep: Option<(f32, f32)> = None;
    let close = |k: &mut Knobs, which: &[usize]| which.iter().for_each(|s| k.matrix[*s] = true);
    match scene {
        "fs1414" => clock_hz = 1_414.0,
        "fs7797" => {}
        "fs13609" => clock_hz = 13_609.0,
        "whine" => {
            clock_hz = 7_797.0;
            whine = 0.3;
        }
        "loop1414" => {
            clock_hz = 1_414.0;
            freeze_at = 12.0;
            seconds = 40.0;
        }
        "sweep" => sweep = Some((31_250.0, 3_000.0)),
        s if s.starts_with("sw:") => {
            // sw:<clock>:<contention>:<input 0/1>:a,b,c[:program[:decay]]
            let mut parts = s[3..].split(':');
            clock_hz = parts.next().unwrap().parse().unwrap();
            contention = parts.next().unwrap().parse().unwrap();
            input_on = parts.next().unwrap() == "1";
            let which: Vec<usize> = parts.next().unwrap().split(',').map(|x| x.parse().unwrap()).collect();
            close(&mut knobs, &which);
            if let Some(program) = parts.next() {
                knobs.program = program.parse().unwrap();
            }
            if let Some(decay) = parts.next() {
                knobs.decay = decay.parse().unwrap();
            }
            seconds = 12.0;
        }
        other => panic!("no such scene: {other}"),
    }
    knobs.contention = contention;
    let mut engine = Engine::new(SR);
    engine.set_knobs(&knobs);
    let mut out = Vec::new();
    for n in 0..(seconds * SR) as usize {
        let t = n as f32 / SR;
        if t >= freeze_at && !knobs.loop_ram {
            knobs.loop_ram = true;
            engine.set_knobs(&knobs);
        }
        let hz = match sweep {
            Some((from, to)) => from * (to / from).powf((t / seconds).min(1.0)),
            None => clock_hz,
        };
        let controls = Controls { clock_hz: hz, drive: 1.0, direct: 0.0, whine, level: 1.0 };
        let x = if input_on && t < freeze_at { arp(n) } else { 0.0 };
        let [l, r] = engine.process([x, x], &controls);
        out.extend_from_slice(&l.to_le_bytes());
        out.extend_from_slice(&r.to_le_bytes());
    }
    std::fs::write(path, out).unwrap();
}
