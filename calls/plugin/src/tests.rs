//! Rendering checks, through the plugin's own `Voice`.

use super::*;

const SR: f32 = 48_000.0;
const BLOCK: usize = 512;

fn voice(settings: &Settings, gates: &[(f32, bool); MAX_CALLERS]) -> Voice {
    let mut v = Voice::new(SR, BLOCK, 777);
    v.set(settings, 120.0, gates);
    v
}

/// Root's caller only, as with no MIDI.
fn root(note: f32) -> [(f32, bool); MAX_CALLERS] {
    std::array::from_fn(|k| (notes::hz(note), k == 0))
}

fn render(v: &mut Voice, input: &[f32]) -> [Vec<f32>; 2] {
    let mut out = [vec![0.0; input.len()], vec![0.0; input.len()]];
    for (k, chunk) in input.chunks(BLOCK).enumerate() {
        let at = k * BLOCK..k * BLOCK + chunk.len();
        let [l, r] = &mut out;
        v.compute([chunk, chunk], [&mut l[at.clone()], &mut r[at]]);
    }
    out
}

fn energy(x: &[f32]) -> f32 {
    x.iter().map(|s| s * s).sum()
}

/// Lively, free-running calls, heard dry (no echo, no reverb).
fn dry_and_busy() -> Settings {
    Settings { activity: 1.0, follow: 0.0, level: -6.0, echo: 0.0, reverb: 0.0, ..Settings::default() }
}

#[test]
fn the_parameters_defaults_are_the_settings() {
    assert_eq!(CallsParams::default().settings(), Settings::default());
}

/// Following the input (the default), silence in is silence out (bar the
/// reverb's own floor, around -400 dB).
#[test]
fn following_silence_is_silent() {
    let settings = Settings { activity: 1.0, ..Settings::default() };
    let [l, r] = render(&mut voice(&settings, &root(60.0)), &vec![0.0; 48_000 * 10]);
    assert!(l.iter().chain(&r).all(|s| s.abs() < 1e-12));
}

/// Free-running, calls come with nothing in; following, they come with input.
#[test]
fn calls_come() {
    let quiet = vec![0.0; 48_000 * 10];
    let [l, _] = render(&mut voice(&dry_and_busy(), &root(60.0)), &quiet);
    println!("free-running: {:.2}", energy(&l));
    assert!(energy(&l) > 1.0, "free-running calls should sound with no input");

    // A steady tone at about -12 dBFS, followed: calls on top of it.
    let tone: Vec<f32> = (0..48_000 * 10).map(|i| 0.25 * (i as f32 * 0.05).sin()).collect();
    let followed = Settings { follow: 1.0, ..dry_and_busy() };
    let [l, _] = render(&mut voice(&followed, &root(60.0)), &tone);
    let added: Vec<f32> = l.iter().zip(&tone).map(|(o, i)| o - i).collect();
    println!("followed over a tone: {:.2}", energy(&added));
    assert!(energy(&added) > 1.0, "followed calls should sound while the input does");
}

/// The input passes through underneath, unchanged where no call sounds.
#[test]
fn the_input_passes() {
    let tone: Vec<f32> = (0..48_000).map(|i| 0.25 * (i as f32 * 0.05).sin()).collect();
    let settings = Settings { level: -60.0, follow: 1.0, ..Settings::default() };
    let [l, _] = render(&mut voice(&settings, &root(60.0)), &tone);
    // The first call waits out a silent gap, so the first second is the input.
    assert!(l.iter().zip(&tone).all(|(o, i)| (o - i).abs() < 1e-6));
}

#[test]
fn width_zero_is_in_the_middle() {
    let settings = Settings { width: 0.0, ..dry_and_busy() };
    let [l, r] = render(&mut voice(&settings, &root(60.0)), &vec![0.0; 48_000 * 10]);
    let off: f32 = l.iter().zip(&r).map(|(a, b)| (a - b).abs()).fold(0.0, f32::max);
    assert!(energy(&l) > 1.0 && off < 1e-5, "left and right should match ({off})");
    let wide = Settings { width: 1.0, ..dry_and_busy() };
    let [l, r] = render(&mut voice(&wide, &root(60.0)), &vec![0.0; 48_000 * 10]);
    assert!(l.iter().zip(&r).any(|(a, b)| (a - b).abs() > 1e-3), "Width 1 should spread them");
}

/// Two notes, two callers; a note let go fades out and stays out.
#[test]
fn callers_follow_their_notes() {
    let mut gates: [(f32, bool); MAX_CALLERS] = std::array::from_fn(|_| (notes::hz(48.0), false));
    gates[0] = (notes::hz(60.0), true);
    gates[1] = (notes::hz(67.0), true);
    let mut v = voice(&dry_and_busy(), &gates);
    let quiet = vec![0.0; 48_000 * 6];
    let [two, _] = render(&mut v, &quiet);
    gates[0].1 = false;
    gates[1].1 = false;
    v.set(&dry_and_busy(), 120.0, &gates);
    let [after, _] = render(&mut v, &quiet);
    let last = &after[48_000 * 3..];
    println!("held: {:.2}, the last 3 s after letting go: {:.6}", energy(&two), energy(last));
    assert!(energy(&two) > 1.0);
    assert_eq!(energy(last), 0.0, "callers should go quiet once let go");
}

/// The echo follows the tempo when synced.
#[test]
fn sync_follows_the_tempo() {
    let s = Settings { sync: true, division: [Division::D4, Division::D8], ..Settings::default() };
    assert_eq!(s.echo_at(120.0).seconds, [0.5, 0.25]);
    assert_eq!(s.echo_at(60.0).seconds, [1.0, 0.5]);
    let free = Settings::default();
    assert_eq!(free.echo_at(60.0).seconds, [0.36, 0.48]);
}

#[test]
fn finite_at_the_extremes() {
    let gates: [(f32, bool); MAX_CALLERS] = std::array::from_fn(|k| (notes::hz(24.0 + 12.0 * k as f32), true));
    let settings = Settings { level: 0.0, activity: 1.0, rate: 3.0, follow: 0.0, echo: 1.0, feedback: 0.95,
                              echo_damp: 500.0, decay: 20.0, damp: 12_000.0, predelay: 150.0, sync: true,
                              division: [Division::D32, Division::D1], ..Settings::default() };
    let [l, r] = render(&mut voice(&settings, &gates), &vec![0.0; 48_000 * 10]);
    let peak = l.iter().chain(&r).fold(0.0f32, |p, s| p.max(s.abs()));
    println!("peak {peak:.2}");
    assert!(l.iter().chain(&r).all(|s| s.is_finite()) && peak < 30.0);
}

/// What it costs: `cargo test --release cpu -- --ignored --nocapture`.
#[test]
#[ignore]
fn cpu() {
    for callers in [1, 8] {
        let gates: [(f32, bool); MAX_CALLERS] = std::array::from_fn(|k| (notes::hz(48.0 + 3.0 * k as f32), k < callers));
        let mut v = voice(&Settings { follow: 0.0, ..Settings::default() }, &gates);
        let input = vec![0.0; 48_000 * 10];
        let t = std::time::Instant::now();
        render(&mut v, &input);
        println!("{callers} callers: {:.2} % of a core", t.elapsed().as_secs_f64() * 10.0);
    }
}

