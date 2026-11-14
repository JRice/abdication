//! Checks for the FDSP stage (voice.lib's `fdsp`): that Off leaves patches
//! exactly as they were, the tuning of its delays, its levels and its cost.
//! The measuring ones are ignored; run them with
//! `cargo test --release fdsp -- --ignored --nocapture`.

use crate::engine::{Engine, MAX_BLOCK};
use crate::params::SynthParams;

pub(crate) fn engine(sets: &[(&str, f32)]) -> (Engine, SynthParams) {
    let params = SynthParams::from_faust();
    let mut e = Engine::new(params.slots());
    e.init(48_000.0);
    e.load(&params);
    for (id, v) in sets {
        e.set_by_id(&params, id, *v);
    }
    (e, params)
}

pub(crate) fn render(e: &mut Engine, blocks: usize) -> Vec<f32> {
    let mut out = Vec::with_capacity(blocks * MAX_BLOCK);
    let mut buf = [0.0; MAX_BLOCK];
    for _ in 0..blocks {
        e.render(&mut buf);
        out.extend_from_slice(&buf);
    }
    out
}

/// A few patches covering the voice's paths, for comparing builds.
pub(crate) const PATCHES: [&[(&str, f32)]; 5] = [
    &[],
    &[("mix.osc_2", 0.7), ("mix.noise", 0.4), ("drive.amount", 0.6), ("filter.type", 2.0), ("filter.resonance", 4.0)],
    &[("dual_1.algorithm", 8.0), ("dual_1.a", 0.7), ("dual_1.b", 0.4), ("unison.mode", 2.0), ("vintage.amount", 0.5)],
    &[("transient.level", 0.8), ("transient.type", 3.0), ("vintage.amount", 0.6)],
    &[("osc_1.mode", 1.0), ("wt_1.position", 0.6), ("wt_1.oscmod", 9.0), ("noise.tilt", -6.0), ("mix.noise", 0.5)],
];

/// A chord held then released, through each of PATCHES, to ABD_DUMP/patch<k>.f32.
pub(crate) fn play(sets: &[(&str, f32)]) -> Vec<f32> {
    let (mut e, _) = engine(sets);
    for n in [36, 48, 55, 64] {
        e.note_on(0, n, 0.8);
    }
    let mut out = render(&mut e, 750 * 2);
    for n in [36, 48, 55, 64] {
        e.note_off(0, n);
    }
    out.extend(render(&mut e, 750));
    out
}

#[test]
#[ignore]
fn fdsp_dump_patches() {
    let Ok(dir) = std::env::var("ABD_DUMP") else { return };
    for (k, sets) in PATCHES.iter().enumerate() {
        let out = play(sets);
        std::fs::write(format!("{dir}/patch{k}.f32"), out.iter().flat_map(|x| x.to_le_bytes()).collect::<Vec<u8>>()).unwrap();
    }
}

/// With Type Off, or a type whose Mix is at 0, the voice is exactly as
/// without FDSP, whatever its other knobs say.
#[test]
fn fdsp_off_leaves_the_voice_alone() {
    let plain = play(PATCHES[1]);
    let moved: Vec<(&str, f32)> = [("water.ring", 0.9), ("tornado.drive", 0.8), ("fdsp.feedback", -0.5), ("fdsp.pitch", 5.0), ("phase.stages", 5.0)]
        .into_iter().chain(PATCHES[1].iter().copied()).collect();
    assert!(play(&moved) == plain, "Type Off with other knobs moved changed the sound");
    for kind in [1.0, 3.0] {
        let off: Vec<(&str, f32)> = [("fdsp.type", kind), ("fdsp.mix", 0.0)].into_iter().chain(PATCHES[1].iter().copied()).collect();
        assert!(play(&off) == plain, "Mix at 0 changed the sound");
    }
}

const TYPES: [(&str, f32); 4] = [("Water", 1.0), ("Tornado", 2.0), ("Flange", 3.0), ("Phase", 4.0)];

fn rms_db(x: &[f32]) -> f32 {
    10.0 * (x.iter().map(|s| s * s).sum::<f32>() / x.len() as f32).max(1e-20).log10()
}

/// Every type at its defaults, and at its knobs' extremes: finite, near the
/// voice's own level, and nothing running away.
#[test]
fn fdsp_types_sound_and_hold_their_level() {
    let base: &[(&str, f32)] = &[("filter.cutoff", 6000.0), ("mix.osc_2", 0.5), ("mix.noise", 0.2)];
    let level = |sets: &[(&str, f32)]| {
        let all: Vec<(&str, f32)> = base.iter().chain(sets).copied().collect();
        let out = play(&all);
        assert!(out.iter().all(|s| s.is_finite()), "{sets:?} made a non-finite sample");
        let held = &out[750 * MAX_BLOCK / 2..750 * 2 * MAX_BLOCK];
        (rms_db(held), held.iter().fold(0f32, |m, s| m.max(s.abs())))
    };
    let (plain, plain_peak) = level(&[]);
    println!("off: {plain:.1} dB rms, peak {plain_peak:.2}");
    for (name, kind) in TYPES {
        let (db, peak) = level(&[("fdsp.type", kind)]);
        println!("{name} at its defaults: {:+.1} dB, peak {peak:.2}", db - plain);
        assert!((db - plain).abs() < 4.0, "{name} at its defaults moved the level {:+.1} dB", db - plain);
        for corner in [0.0, 1.0] {
            let sets: Vec<(&str, f32)> = [("fdsp.type", kind)].into_iter().chain(extremes(name, corner)).collect();
            let (db, peak) = level(&sets);
            println!("{name} at its {} extremes: {:+.1} dB, peak {peak:.2}", if corner == 0.0 { "low" } else { "high" }, db - plain);
            assert!(peak < 4.0, "{name} ran away: peak {peak}");
        }
    }
}

/// Every knob of a type (its own, and the shared Pitch and Feedback) at the
/// bottom (0) or top (1) of its range.
fn extremes(name: &str, corner: f32) -> Vec<(&'static str, f32)> {
    let params = SynthParams::from_faust();
    let group = name.to_lowercase();
    params.ids().filter(|id| id.starts_with(&format!("{group}.")) || ["fdsp.pitch", "fdsp.feedback"].contains(id))
        .map(|id| {
            let p = params.ptr(id).unwrap();
            let v = unsafe { p.preview_plain(corner) };
            (Box::leak(id.to_string().into_boxed_str()) as &'static str, v)
        })
        .collect()
}

/// A note's spectrum peak near `hz`, by a long FFT-free scan: the frequency
/// whose single-bin DFT is strongest in +-`cents`.
fn peak_near(x: &[f32], hz: f32, cents: f32) -> f32 {
    let power = |f: f32| {
        let w = 2.0 * std::f64::consts::PI * f as f64 / 48_000.0;
        let (mut re, mut im) = (0.0f64, 0.0f64);
        let n = x.len() as f64;
        for (k, s) in x.iter().enumerate() {
            let hann = 0.5 - 0.5 * (2.0 * std::f64::consts::PI * k as f64 / n).cos();
            re += *s as f64 * hann * (w * k as f64).cos();
            im += *s as f64 * hann * (w * k as f64).sin();
        }
        re * re + im * im
    };
    let (mut best, mut best_p) = (hz, 0.0);
    let steps = 120;
    for i in 0..=steps {
        let f = hz * 2f32.powf((-cents + 2.0 * cents * i as f32 / steps as f32) / 1200.0);
        let p = power(f);
        if p > best_p {
            best = f;
            best_p = p;
        }
    }
    best
}

/// Water rings at the note it's played on, across the keyboard: struck once
/// by a click (the transient), ringing long and bright.
#[test]
#[ignore]
fn fdsp_water_is_in_tune() {
    for note in [36u8, 48, 60, 72, 84, 96] {
        let (mut e, _) = engine(&[("mix.osc_1", 0.0), ("mix.noise", 0.0), ("transient.level", 1.0), ("transient.type", 0.0),
                                  ("filter.cutoff", 18000.0),
                                  ("mod.filter_cutoff.env", 0.0), ("mod.filter_cutoff.key", 0.0), ("drive.amount", 0.0),
                                  ("amp.sustain", 1.0), ("fdsp.type", 1.0), ("water.ring", 0.9), ("water.damp", 0.0),
                                  ("water.wobble", 0.0), ("fdsp.pitch", 0.0), ("fdsp.mix", 1.0), ("settings.heat", 0.0)]);
        e.note_on(0, note, 0.8);
        render(&mut e, 75);
        let x = render(&mut e, 750 * 4);
        let hz = crate::engine::note_freq(note);
        let found = peak_near(&x, hz, 50.0);
        println!("note {note}: {hz:.1} Hz, rings at {found:.2} ({:+.1} cents)", 1200.0 * (found / hz).log2());
        assert!((1200.0 * (found / hz).log2()).abs() < 3.0);
    }
}

/// A voice reused from silence doesn't start with its last note's ringing:
/// its start against its middle is as without FDSP (the filter's envelope
/// makes every start a little louder).
#[test]
fn fdsp_reused_voice_starts_clean() {
    let ratio = |kind: f32| {
        let (mut e, _) = engine(&[("fdsp.type", kind), ("water.ring", 1.0), ("fdsp.feedback", 0.95),
                                  ("amp.release", 0.01)]);
        // Fill every voice with a loud low note, let them all go silent...
        let low = [24u8, 25, 26, 27, 28, 29, 30, 31];
        low.iter().for_each(|n| e.note_on(0, *n, 1.0));
        render(&mut e, 750);
        low.iter().for_each(|n| e.note_off(0, *n));
        render(&mut e, 750);
        assert_eq!(e.active_voices(), 0);
        // ...then a quiet high one.
        e.note_on(0, 84, 0.3);
        let x = render(&mut e, 750);
        let start = x[..2400].iter().fold(0f32, |m, s| m.max(s.abs()));
        let middle = x[24_000..].iter().fold(0f32, |m, s| m.max(s.abs()));
        start / middle
    };
    let plain = ratio(0.0);
    println!("off: first 50 ms peak / later {plain:.2}");
    for (name, kind) in TYPES {
        let r = ratio(kind);
        println!("{name}: first 50 ms peak / later {r:.2}");
        assert!(r < 1.25 * plain, "{name} started with a burst");
    }
}

/// Demo renders: a chord, a line and a low note, dry and through each type,
/// to ABD_DUMP/<name>.wav (16-bit mono, 48 kHz).
#[test]
#[ignore]
fn fdsp_demos() {
    let Ok(dir) = std::env::var("ABD_DUMP") else { return };
    let base: &[(&str, f32)] = &[("filter.cutoff", 2600.0), ("amp.release", 0.6), ("amp.sustain", 0.8), ("mix.osc_2", 0.0)];
    let demos: [(&str, &[(&str, f32)]); 10] = [
        ("0 dry", &[]),
        ("1 water", &[("fdsp.type", 1.0)]),
        ("2 water struck", &[("fdsp.type", 1.0), ("transient.level", 0.9), ("transient.type", 0.0), ("mix.osc_1", 0.35),
                             ("mix.noise", 0.25), ("noise.type", 0.0), ("water.ring", 0.85), ("water.damp", 0.25),
                             ("water.wobble", 0.5), ("amp.sustain", 0.5), ("amp.decay", 1.5)]),
        ("3 water an octave and a fifth up", &[("fdsp.type", 1.0), ("fdsp.pitch", 19.0), ("water.ring", 0.8), ("water.wobble", 0.6)]),
        ("4 tornado", &[("fdsp.type", 2.0)]),
        ("5 tornado pushed", &[("fdsp.type", 2.0), ("tornado.drive", 0.7), ("tornado.overtone", 0.7), ("tornado.ceiling", 0.6),
                               ("tornado.edge", 0.5)]),
        ("6 flange", &[("fdsp.type", 3.0)]),
        ("7 flange swept", &[("fdsp.type", 3.0), ("fdsp.feedback", 0.8), ("mod.fdsp_pitch.lfo", 0.12),
                             ("mod.fdsp_pitch.rate", 0.25)]),
        ("8 phase", &[("fdsp.type", 4.0)]),
        ("9 phase swept", &[("fdsp.type", 4.0), ("fdsp.feedback", 0.7), ("phase.stages", 4.0), ("mod.fdsp_pitch.lfo", 0.15),
                            ("mod.fdsp_pitch.rate", 0.3)]),
    ];
    let secs = |s: f32| (s * 48_000.0 / MAX_BLOCK as f32) as usize;
    for (name, sets) in demos {
        let all: Vec<(&str, f32)> = base.iter().chain(sets).copied().collect();
        let (mut e, _) = engine(&all);
        let mut out = Vec::new();
        // A chord...
        let chord = [48u8, 52, 55, 59];
        chord.iter().for_each(|n| e.note_on(0, *n, 0.8));
        out.extend(render(&mut e, secs(2.5)));
        chord.iter().for_each(|n| e.note_off(0, *n));
        out.extend(render(&mut e, secs(0.8)));
        // ...a line...
        for n in [60u8, 63, 67, 70, 72, 70, 67, 63, 65, 68, 72, 75] {
            e.note_on(0, n, 0.7);
            out.extend(render(&mut e, secs(0.2)));
            e.note_off(0, n);
            out.extend(render(&mut e, secs(0.05)));
        }
        out.extend(render(&mut e, secs(0.6)));
        // ...and a low note.
        e.note_on(0, 36, 0.9);
        out.extend(render(&mut e, secs(2.5)));
        e.note_off(0, 36);
        out.extend(render(&mut e, secs(1.5)));
        let peak = out.iter().fold(0f32, |m, s| m.max(s.abs()));
        println!("{name}: peak {peak:.2}, rms {:.1} dB", rms_db(&out));
        write_wav(&format!("{dir}/{name}.wav"), &out);
    }
}

fn write_wav(path: &str, x: &[f32]) {
    // At the plugin's default Volume (-6 dB), as you'd hear it.
    let data: Vec<u8> = x.iter().flat_map(|s| (((0.5 * s).clamp(-1.0, 1.0) * 32767.0).round() as i16).to_le_bytes()).collect();
    let mut w = Vec::new();
    w.extend(b"RIFF");
    w.extend((36 + data.len() as u32).to_le_bytes());
    w.extend(b"WAVEfmt ");
    w.extend(16u32.to_le_bytes());
    w.extend(1u16.to_le_bytes());
    w.extend(1u16.to_le_bytes());
    w.extend(48_000u32.to_le_bytes());
    w.extend((48_000u32 * 2).to_le_bytes());
    w.extend(2u16.to_le_bytes());
    w.extend(16u16.to_le_bytes());
    w.extend(b"data");
    w.extend((data.len() as u32).to_le_bytes());
    w.extend(data);
    std::fs::write(path, w).unwrap();
}
