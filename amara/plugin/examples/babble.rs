//! Prints what a few languages sound like on paper. cargo run --release --example babble
use amara::dsp::language::{Language, DEFAULT_STRANGENESS};
use amara::dsp::phones::phone;
use amara::dsp::speaker::{Mark, Speaker, SpeakerKnobs};

fn main() {
    let knobs = SpeakerKnobs { rate: 5.0, sparsity: 0.1, temper: 0.5, swing: 0.5, breath: 0.1, tract: 1.0, pitch_hz: 90.0, per_note: false };
    for seed in [3u32, 42, 137, 512, 777] {
        let language = Language::new(seed, DEFAULT_STRANGENESS);
        let mut speaker = Speaker::new(seed, DEFAULT_STRANGENESS);
        let mut text = String::new();
        for _ in 0..3_000 {
            speaker.step(0.005, &knobs);
            for mark in speaker.take_marks() {
                match mark {
                    Mark::Phone(p) => text.push_str(phone(*p).symbol),
                    Mark::Long => text.push('ː'),
                    Mark::Stress => text.push('ˈ'),
                    Mark::Word => text.push(' '),
                    Mark::Phrase => text.push_str(" ‖"),
                }
            }
        }
        println!("{seed} {}: stress {:?}, unstressed {:.2}\n  {}\n", language.name(), language.stress, language.unstressed_scale, text.trim());
    }
}
