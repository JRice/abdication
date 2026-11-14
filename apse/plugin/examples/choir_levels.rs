//! Choir's level across Resonance and Vowel, for keeping it even.
//! cargo run --release --example choir_levels
use apse::dsp::choir::ChoirControls;
use apse::dsp::engine::{Controls, Engine};
use apse::dsp::modes::MODES;

const SR: f32 = 48_000.0;

fn main() {
    let choir = MODES.iter().position(|m| m.choir).unwrap();
    for resonance in [0.0, 0.45, 1.0] {
        for vowel in [0.0, 0.5, 1.0] {
            let c = Controls { mix: 1.0, time_s: 3.0, predelay_s: 0.0, choir: ChoirControls { resonance, vowel, ..Default::default() }, ..Default::default() };
            let mut engine = Engine::new(SR);
            engine.set_mode(choir);
            let mut seed = 7u32;
            let (mut input, mut output) = (0.0f64, 0.0f64);
            for n in 0..(SR * 4.0) as usize {
                seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                let x = if n < 2_400 { 0.25 * ((seed >> 9) as f32 / (1 << 22) as f32 - 1.0) } else { 0.0 };
                input += (x * x) as f64;
                let [l, r] = engine.process([x, x], &c);
                output += 0.5 * (l * l + r * r) as f64;
            }
            println!("resonance {resonance:.2} vowel {vowel:.1}: {:.1} dB", 10.0 * (output / input).log10());
        }
    }
}
