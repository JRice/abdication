//! What the Rate knob actually produces, in syllables per second of speaking time.
use amara::dsp::phones::{phone, Manner};
use amara::dsp::speaker::{Mark, Speaker, SpeakerKnobs};

fn main() {
    for seed in [137u32, 13, 42] {
        for rate in [1.0f32, 2.5, 5.0, 7.5, 10.0, 16.0] {
            let knobs = SpeakerKnobs { rate, sparsity: 0.0, temper: 0.45, swing: 0.0, breath: 0.1, tract: 1.0, pitch_hz: 90.0, per_note: false };
            let mut speaker = Speaker::new(seed, 0.4);
            let (mut syllables, mut speaking, dt) = (0, 0.0, 0.005);
            for _ in 0..(300.0 / dt) as usize {
                speaker.step(dt, &knobs);
                for mark in speaker.take_marks() {
                    if matches!(mark, Mark::Phone(p) if phone(*p).manner == Manner::Vowel) {
                        syllables += 1;
                    }
                }
                if speaker.target(&knobs).gain > 1e-3 {
                    speaking += dt;
                }
            }
            println!("seed {seed} knob {rate:>4}: {:.1} syl/s overall, {:.2} of the time audible", syllables as f32 / 300.0, speaking / 300.0);
        }
    }
}
