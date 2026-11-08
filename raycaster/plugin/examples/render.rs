//! Renders held notes to a 16-bit stereo WAV at 48 kHz, for measuring and listening, and
//! reports how much of a core the audio took.
//!
//! cargo run --release --example render -- <out.wav> <seconds> [key=value ...]
//! keys: notes=45,52  every=<s> (play the notes in turn, this far apart, instead of
//!       together)  hold=<s> (how long each is held when played in turn)  element=earth|water|air|fire|wood|metal|chaos  element2=...
//!       level2  squish skew tilt center ray orbit rate chorus  cutoff res drive env
//!       attack decay sustain release (amp envelope, seconds and 0..1)
//!       unison=<1..6> detune=<cents> free=1 free2=1 mode=<0..1> wheel=<0..1>
//!       filter=svf|ladder  width  mono=1  lfo=<target>:<shape>:<hz>:<amount>
//!       (mod shapes: sine saw ramp square noise brown sh)
use std::time::Instant;

use raycaster::dsp::engine::{BlockControls, Engine, FilterKind, MAX_BLOCK};
use raycaster::dsp::modulator::{LfoShape, ModSettings, SHAPES};
use raycaster::dsp::shapes::{chaos, Element, ALL};
use raycaster::dsp::targets::{global_target, osc_target, GlobalTarget, OscTarget};

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

fn element(name: &str) -> Element {
    *ALL.iter().find(|e| e.name().eq_ignore_ascii_case(name)).expect("no such element")
}

fn osc_key(key: &str) -> Option<OscTarget> {
    Some(match key {
        "fine" => OscTarget::Fine,
        "level" => OscTarget::Level,
        "squish" => OscTarget::Squish,
        "skew" => OscTarget::Skew,
        "tilt" => OscTarget::Tilt,
        "center" => OscTarget::Center,
        "ray" => OscTarget::Ray,
        "orbit" => OscTarget::Orbit,
        "rate" => OscTarget::OrbitRate,
        _ => return None,
    })
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (path, seconds) = (&args[1], args[2].parse::<f32>().unwrap());
    let mut c = BlockControls::default();
    let mut notes = vec![45u8];
    let mut chaos_seed = None;
    let (mut every, mut hold) = (0.0f32, 0.0f32);
    for pair in &args[3..] {
        let (key, value) = pair.split_once('=').unwrap();
        let v = || value.parse::<f32>().unwrap();
        let (base, osc) = match key.strip_suffix('2') {
            Some(k) => (k, 1),
            None => (key, 0),
        };
        if let Some(t) = osc_key(base) {
            c.set(osc_target(osc, t), v());
            continue;
        }
        match base {
            "attack" => c.amp_env.attack_s = v(),
            "decay" => c.amp_env.decay_s = v(),
            "sustain" => c.amp_env.sustain = v(),
            "release" => c.amp_env.release_s = v(),
            "unison" => c.unison = value.parse().unwrap(),
            "detune" => c.detune = v(),
            "free" => c.oscs[osc].free = v() > 0.5,
            "mode" => c.set(global_target(GlobalTarget::FilterMode), v()),
            "every" => every = v(),
            "hold" => hold = v(),
            "notes" => notes = value.split(',').map(|n| n.parse().unwrap()).collect(),
            "element" => c.oscs[osc].element = element(value),
            "seed" => chaos_seed = Some((osc, value.parse::<u32>().unwrap())),
            "octave" => c.oscs[osc].octave = value.parse().unwrap(),
            "semi" => c.oscs[osc].semi = value.parse().unwrap(),
            "chorus" => c.set(global_target(GlobalTarget::Chorus), v()),
            "cutoff" => c.set(global_target(GlobalTarget::Cutoff), v()),
            "res" => c.set(global_target(GlobalTarget::Resonance), v()),
            "drive" => c.set(global_target(GlobalTarget::Drive), v()),
            "env" => c.set(global_target(GlobalTarget::FilterEnv), v()),
            "width" => c.set(global_target(GlobalTarget::Width), v()),
            "filter" => c.filter = if value == "svf" { FilterKind::Svf } else { FilterKind::Ladder },
            "mono" => c.mono = v() > 0.5,
            "lfo" => {
                let parts: Vec<&str> = value.split(':').collect();
                let t = osc_key(parts[0]).map(|t| osc_target(0, t)).unwrap_or(global_target(GlobalTarget::Cutoff));
                let shape = SHAPES
                    .iter()
                    .copied()
                    .find(|s| s.label().replace("S&H", "sh").eq_ignore_ascii_case(parts[1]))
                    .unwrap_or(LfoShape::Sine);
                c.mods[t] = ModSettings { shape, hz: parts[2].parse().unwrap(), lfo: parts[3].parse().unwrap(), ..ModSettings::default() };
            }
            other => panic!("no such key: {other}"),
        }
    }
    let mut engine = Engine::new(SR);
    if let Some((osc, seed)) = chaos_seed {
        engine.take_chaos(osc, std::sync::Arc::new(chaos(seed).0));
    }
    // (sample, note, on)
    let mut events: Vec<(usize, u8, bool)> = Vec::new();
    for (i, key) in notes.iter().enumerate() {
        let at = (i as f32 * every * SR) as usize;
        events.push((at, *key, true));
        if every > 0.0 {
            events.push((at + (hold.max(0.001) * SR) as usize, *key, false));
        }
    }
    events.sort_by_key(|e| (e.0, e.2));
    let n = (seconds * SR) as usize;
    let mut samples = Vec::with_capacity(n);
    let (mut l, mut r) = ([0.0f32; MAX_BLOCK], [0.0f32; MAX_BLOCK]);
    let started = Instant::now();
    let mut next = 0;
    while samples.len() < n {
        while next < events.len() && events[next].0 <= samples.len() {
            let (_, key, on) = events[next];
            if on {
                engine.note_on(key, 0.9, &c);
            } else {
                engine.note_off(key);
            }
            next += 1;
        }
        engine.render(&c, &mut l, &mut r);
        samples.extend(l.iter().zip(&r).map(|(a, b)| [*a, *b]));
    }
    let took = started.elapsed();
    eprintln!(
        "{} notes: audio {:.2} s for {seconds} s ({:.1} % of a core); limiter {:.1} dB",
        notes.len(),
        took.as_secs_f32(),
        100.0 * took.as_secs_f32() / seconds,
        engine.reduction_db()
    );
    wav(path, &samples);
}
