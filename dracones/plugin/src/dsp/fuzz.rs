//! The fuzz after the delays: a high-pass that decides what gets fuzzed (20 Hz to
//! 1.2 kHz), then two clipping stages and a fixed tone. Gain 0 is clean, and the fuzz
//! fades in over the first tenth of the knob.

use super::filters::{one_pole, OnePole};

const TONE_HZ: f32 = 3_800.0;
const FADE_IN: f32 = 0.1;

/// HP knob (0..1) to Hz.
pub fn hp_hz(knob: f32) -> f32 {
    20.0 * 60f32.powf(knob.clamp(0.0, 1.0))
}

pub struct Fuzz {
    rate: f32,
    hp: [[OnePole; 2]; 2],
    tone: [OnePole; 2],
    tone_a: f32,
}

impl Fuzz {
    pub fn new(rate: f32) -> Self {
        Self { rate, hp: Default::default(), tone: Default::default(), tone_a: one_pole(TONE_HZ, rate) }
    }

    /// The coefficient for this block's HP corner.
    pub fn hp_coefficient(&self, knob: f32) -> f32 {
        one_pole(hp_hz(knob), self.rate)
    }

    #[inline]
    pub fn tick(&mut self, x: [f32; 2], gain: f32, hp_a: f32) -> [f32; 2] {
        let g = gain.clamp(0.0, 1.0);
        if g <= 0.0 {
            return x;
        }
        let fuzz = (g / FADE_IN).min(1.0);
        let drive = 3.0 + 70.0 * g * g;
        std::array::from_fn(|c| {
            let [a, b] = &mut self.hp[c];
            let h = b.high(a.high(x[c], hp_a), hp_a);
            // Stage one: a transistor's soft clip, pushed hard. Stage two: a lopsided
            // diode pair, which squares the top off and leaves a little even ring.
            let s1 = (drive * h).tanh();
            let s2 = (1.6 * s1 + 0.25).clamp(-1.0, 1.0) - 0.25 * 0.6;
            let y = 0.55 * self.tone[c].low(s2, self.tone_a);
            x[c] + fuzz * (y - x[c])
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gain_zero_is_clean_and_full_is_loud_and_bounded() {
        let mut f = Fuzz::new(48_000.0);
        let a = f.hp_coefficient(0.0);
        assert_eq!(f.tick([0.3, -0.2], 0.0, a), [0.3, -0.2]);
        let mut peak = 0.0f32;
        for i in 0..48_000 {
            let x = 0.1 * (i as f32 * 0.02).sin();
            let y = f.tick([x, x], 1.0, a);
            peak = peak.max(y[0].abs());
        }
        assert!(peak > 0.2 && peak < 1.0, "{peak}");
    }
}
