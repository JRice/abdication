//! Rendering checks on the DSP, through the plugin's own `Voice`.

use super::*;

const SR: f32 = 48_000.0;
const BLOCK: usize = 512;

/// White noise, the same every time.
fn noise(n: usize) -> Vec<f32> {
    let mut state = 0x1234_5678u32;
    (0..n).map(|_| {
        state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        (state as f32 / u32::MAX as f32 - 0.5) * 0.5
    }).collect()
}

fn voice(params: &Settings, notes: &[f32]) -> Voice {
    let mut v = Voice::new(SR, 4242);
    v.set(params, notes);
    v
}

/// The same signal into both channels, block by block; both channels out.
fn render(v: &mut Voice, input: &[f32]) -> [Vec<f32>; 2] {
    let mut out = [vec![0.0; input.len()], vec![0.0; input.len()]];
    for (k, chunk) in input.chunks(BLOCK).enumerate() {
        let at = k * BLOCK..k * BLOCK + chunk.len();
        let [l, r] = &mut out;
        v.compute([chunk, chunk], [&mut l[at.clone()], &mut r[at]]);
    }
    out
}

/// A frame's level at `hz`, in dB (Goertzel).
fn level_at(frame: &[f32], hz: f32) -> f32 {
    let w = 2.0 * std::f32::consts::PI * hz / SR;
    let c = 2.0 * w.cos();
    let (mut s1, mut s2) = (0.0f32, 0.0f32);
    for x in frame {
        let s = x + c * s1 - s2;
        s2 = s1;
        s1 = s;
    }
    10.0 * (s1 * s1 + s2 * s2 - c * s1 * s2).max(1e-20).log10()
}

/// A frame's power in a third of an octave around `hz` (17 Goertzel bins), in dB.
fn band_at(frame: &[f32], hz: f32) -> f32 {
    let power: f32 = (-8..=8).map(|k| 10f32.powf(level_at(frame, hz * 2f32.powf(k as f32 / 48.0)) / 10.0)).sum();
    10.0 * power.log10()
}

/// How far the output strays from the input around `hz` over 100 ms frames
/// (after the first second): the lowest and highest difference in dB.
fn range_at(input: &[f32], output: &[f32], hz: f32) -> (f32, f32) {
    let frame = 4800;
    input.chunks(frame).zip(output.chunks(frame)).skip(10).filter(|(i, _)| i.len() == frame)
        .map(|(i, o)| band_at(o, hz) - band_at(i, hz))
        .fold((f32::MAX, f32::MIN), |(lo, hi), d| (lo.min(d), hi.max(d)))
}

fn rms(x: &[f32]) -> f32 {
    (x.iter().map(|s| s * s).sum::<f32>() / x.len() as f32).sqrt()
}

#[test]
fn depth_zero_is_the_input_exactly() {
    let mut params = Settings::default();
    params.depth = 0.0;
    let input = noise(48_000);
    let [l, r] = render(&mut voice(&params, &[220.0]), &input);
    assert!(l == input && r == input, "Depth 0 should pass the input untouched");
}

/// With Reach 1, every bump is on the note itself: the spectrum moves there
/// and moves far less two octaves up (with Hold Level off, which would otherwise
/// move everything to make up for it), both channels the same.
#[test]
fn bumps_land_on_the_note() {
    let mut params = Settings::default();
    params.depth = 12.0;
    params.reach = 1;
    params.hold = false;
    let input = noise(48_000 * 12);
    let [l, r] = render(&mut voice(&params, &[1000.0]), &input);
    assert!(l == r, "both channels should get the same bumps");
    let (on_lo, on_hi) = range_at(&input, &l, 1000.0);
    let (off_lo, off_hi) = range_at(&input, &l, 4000.0);
    println!("at the note {on_lo:.1}..{on_hi:.1} dB, two octaves up {off_lo:.1}..{off_hi:.1} dB");
    assert!(on_hi - on_lo > 8.0, "the note's band should move");
    // (Six slots stacked on one note boost it by up to 40 dB or so; their
    // skirts reach a few dB two octaves away.)
    assert!(off_hi - off_lo < (on_hi - on_lo) / 5.0, "two octaves up should move far less");
}

/// Hold Level keeps the average level, at the default settings.
#[test]
fn hold_level_holds_it() {
    let level = |hold: bool| {
        let mut params = Settings::default();
        params.hold = hold;
        let input = noise(48_000 * 12);
        let [l, _] = render(&mut voice(&params, &[110.0, 165.0]), &input);
        20.0 * (rms(&l[48_000 * 4..]) / rms(&input[48_000 * 4..])).log10()
    };
    let (held, free) = (level(true), level(false));
    println!("level: held {held:+.2} dB, free {free:+.2} dB");
    assert!(held.abs() < 0.5, "Hold Level should keep the level ({held:+.2} dB)");
}

/// With two notes and Reach 1, bumps land on both, and nowhere between.
#[test]
fn bumps_share_out_the_notes() {
    let mut params = Settings::default();
    params.depth = 12.0;
    params.reach = 1;
    params.sharp = 3.0;
    let input = noise(48_000 * 16);
    let [l, _] = render(&mut voice(&params, &[600.0, 2400.0]), &input);
    let moves = |hz| { let (lo, hi) = range_at(&input, &l, hz); hi - lo };
    let (a, between, b) = (moves(600.0), moves(1200.0), moves(2400.0));
    println!("moves: 600 Hz {a:.1} dB, 1200 Hz {between:.1} dB, 2400 Hz {b:.1} dB");
    assert!(a > 6.0 && b > 6.0 && between < a.min(b) / 2.0);
}

#[test]
fn fewer_slots_do_less() {
    let spread = |slots: i32| {
        let mut params = Settings::default();
        params.depth = 12.0;
        params.slots = slots;
        params.hold = false;
        let input = noise(48_000 * 8);
        let [l, _] = render(&mut voice(&params, &[110.0]), &input);
        // How much the output differs from the input.
        let diff: Vec<f32> = l.iter().zip(&input).map(|(o, i)| o - i).collect();
        rms(&diff) / rms(&input)
    };
    let (one, twelve) = (spread(1), spread(12));
    println!("difference from the input: 1 slot {one:.3}, 12 slots {twelve:.3}");
    assert!(twelve > 2.0 * one);
}

#[test]
fn mix_and_output() {
    let mut params = Settings::default();
    params.mix = 0.0;
    params.output = -6.0;
    let input = noise(48_000);
    let [l, _] = render(&mut voice(&params, &[220.0]), &input);
    // Mix 0 is the input, at Output (once its smoothing settles).
    let g = 10f32.powf(-6.0 / 20.0);
    let worst = l[24_000..].iter().zip(&input[24_000..]).map(|(o, i)| (o - g * i).abs()).fold(0.0, f32::max);
    assert!(worst < 1e-5, "{worst}");
}

#[test]
fn finite_at_the_extremes() {
    let mut params = Settings::default();
    params.depth = 18.0;
    params.density = 1.0;
    params.sharp = 4.0;
    params.slots = 12;
    params.reach = 128;
    params.walk = 1.0;
    params.drift = 0.5;
    params.cut = 6.0;
    params.life = 0.25;
    let input = noise(48_000 * 4);
    for notes in [vec![20.0], vec![8000.0], vec![55.0, 110.0, 220.0, 440.0, 880.0, 1760.0, 3520.0, 7040.0]] {
        let [l, _] = render(&mut voice(&params, &notes), &input);
        assert!(l.iter().all(|s| s.is_finite() && s.abs() < 20.0), "{notes:?}");
    }
}

#[test]
fn the_parameters_defaults_are_the_settings() {
    assert_eq!(AlternativesParams::default().settings(), Settings::default());
}

/// What it costs: `cargo test --release cpu -- --ignored --nocapture`.
#[test]
#[ignore]
fn cpu() {
    for slots in [6, 12] {
        let mut params = Settings::default();
        params.slots = slots;
        let mut v = voice(&params, &[110.0, 220.0, 330.0]);
        let input = noise(48_000 * 10);
        let t = std::time::Instant::now();
        render(&mut v, &input);
        let share = t.elapsed().as_secs_f64() / 10.0;
        println!("{slots} slots: {:.2} % of a core", share * 100.0);
    }
}
