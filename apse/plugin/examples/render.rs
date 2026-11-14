//! Renders a short-attack phrase through each mode to 16-bit WAVs, for listening.
//! cargo run --release --example render -- <out_dir> [time_s] [length_s] [mode name, or all] [setting=value...]
//! Settings: mix, damp, vowel, drift, resonance, ensemble, breath (all 0–1).
use std::f32::consts::TAU;
use std::path::Path;

use apse::dsp::engine::{Controls, Engine};
use apse::dsp::modes::MODES;

const SR: f32 = 48_000.0;
/// The phrase stops here, leaving the rest for the tail.
const PHRASE_S: f32 = 2.6;

fn noise(seed: &mut u32) -> f32 {
    *seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
    (*seed >> 9) as f32 / (1 << 22) as f32 - 1.0
}

/// Plucked saws, a snare-ish hit at the end: everything with a sharp front.
fn phrase(n: usize, seed: &mut u32) -> f32 {
    let t = n as f32 / SR;
    let hit = noise(seed);
    if t >= PHRASE_S {
        return 0.0;
    }
    let notes = [(0.0, 220.0), (0.3, 330.0), (0.6, 440.0), (0.9, 554.4), (1.2, 659.3), (1.5, 440.0)];
    let mut x = 0.0;
    for (start, freq) in notes {
        let local = t - start;
        if local >= 0.0 {
            let saw = 2.0 * (local * freq).fract() - 1.0;
            x += 0.35 * saw * (-local * 22.0).exp() * (local * 2_000.0).min(1.0);
        }
    }
    let snare = t - 2.0;
    if snare >= 0.0 {
        let body = (TAU * 190.0 * snare).sin() * (-snare * 30.0).exp();
        x += 0.5 * (0.6 * hit * (-snare * 18.0).exp() + 0.5 * body);
    }
    x
}

fn write_wav(path: &Path, frames: &[[f32; 2]]) {
    let data_len = (frames.len() * 4) as u32;
    let mut out = Vec::with_capacity(44 + data_len as usize);
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&(36 + data_len).to_le_bytes());
    out.extend_from_slice(b"WAVEfmt ");
    out.extend_from_slice(&16u32.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes()); // PCM
    out.extend_from_slice(&2u16.to_le_bytes());
    out.extend_from_slice(&(SR as u32).to_le_bytes());
    out.extend_from_slice(&(SR as u32 * 4).to_le_bytes());
    out.extend_from_slice(&4u16.to_le_bytes());
    out.extend_from_slice(&16u16.to_le_bytes());
    out.extend_from_slice(b"data");
    out.extend_from_slice(&data_len.to_le_bytes());
    for frame in frames {
        for x in frame {
            out.extend_from_slice(&((x.clamp(-1.0, 1.0) * 32_767.0).round() as i16).to_le_bytes());
        }
    }
    std::fs::write(path, out).unwrap();
}

fn render(mode: Option<usize>, c: &Controls, seconds: f32) -> Vec<[f32; 2]> {
    let mut engine = Engine::new(SR);
    engine.set_mode(mode.unwrap_or(0));
    for _ in 0..2_400 {
        engine.process([0.0; 2], c);
    }
    let mut seed = 3u32;
    (0..(seconds * SR) as usize)
        .map(|n| {
            let x = phrase(n, &mut seed);
            if mode.is_some() {
                engine.process([x, x], c)
            } else {
                [x, x]
            }
        })
        .collect()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let dir = Path::new(&args[1]);
    let number = |i: usize, default: f32| args.get(i).map_or(default, |s| s.parse().unwrap());
    let (time_s, seconds) = (number(2, 4.0), number(3, 9.0));
    let only = args.get(4).filter(|m| *m != "all");
    std::fs::create_dir_all(dir).unwrap();
    let mut c = Controls { mix: 0.45, time_s, ..Default::default() };
    let settings = &args[args.len().min(5)..];
    for setting in settings {
        let (key, value) = setting.split_once('=').expect("settings look like vowel=0.5");
        let value: f32 = value.parse().unwrap();
        let slot = match key {
            "mix" => &mut c.mix,
            "damp" => &mut c.damp,
            "vowel" => &mut c.choir.vowel,
            "drift" => &mut c.choir.drift,
            "resonance" => &mut c.choir.resonance,
            "ensemble" => &mut c.choir.ensemble,
            "breath" => &mut c.choir.breath,
            other => panic!("no such setting: {other}"),
        };
        *slot = value;
    }
    let slug = |name: &str| name.to_lowercase().replace(' ', "-");
    let suffix = std::iter::once(format!("{time_s}s")).chain(settings.iter().map(|s| s.replace('=', ""))).collect::<Vec<_>>().join("-");
    if only.is_none() {
        write_wav(&dir.join(format!("apse-0-dry-{suffix}.wav")), &render(None, &c, seconds));
    }
    for (i, mode) in MODES.iter().enumerate().filter(|(_, m)| only.is_none_or(|o| slug(o) == slug(m.name))) {
        let name = format!("apse-{}-{}-{suffix}.wav", i + 1, slug(mode.name));
        write_wav(&dir.join(name), &render(Some(i), &c, seconds));
    }
}
