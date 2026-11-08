//! A linear-predictive voice codec and the line it travels over. A frame says only what
//! the filter is, what's exciting it and how loud to be, in a few dozen bits. Those bits
//! are real, so errors on the line do what they did: put the vowel somewhere impossible,
//! make the pitch yelp, or lose the frame so the last one has to do again.
//!
//! The filter travels as log area ratios, so no bit pattern can make it unstable. It can
//! get very close, though, and that's where the whistling comes from.

use super::random::Rng;
use super::speaker::{Target, FORMANTS};

pub const ORDER: usize = 2 + 2 * FORMANTS;

const COEFFICIENT_BITS: [u8; ORDER] = [6, 6, 5, 5, 5, 5, 4, 4, 4, 4, 3, 3, 3, 3];
/// The largest log area ratio each coefficient can carry.
const COEFFICIENT_RANGE: [f32; ORDER] = [6.0, 6.0, 4.5, 4.0, 3.5, 3.5, 3.0, 3.0, 2.5, 2.5, 2.0, 2.0, 2.0, 2.0];
const PITCH_BITS: u8 = 7;
const PITCH_MIN_HZ: f32 = 20.0;
const PITCH_MAX_HZ: f32 = 1_400.0;
const VOICING_BITS: u8 = 2;
const ROUGH_BITS: u8 = 2;
const GAIN_BITS: u8 = 6;
/// Full code at this level, with each step below it this many dB. The bottom code is silence.
const GAIN_TOP_DB: f32 = 6.0;
const GAIN_STEP_DB: f32 = 1.3;
const LARGEST_REFLECTION: f32 = 0.9995;
/// Each lost frame in a row repeats the last one this much quieter.
const CONCEALMENT_FADE: f32 = 0.8;

#[derive(Clone, Copy)]
pub struct Packet {
    coefficients: [u8; ORDER],
    pitch: u8,
    voicing: u8,
    rough: u8,
    gain: u8,
}

#[derive(Clone, Copy, Debug)]
pub struct Frame {
    pub ratios: [f32; ORDER],
    pub f0: f32,
    pub voicing: f32,
    pub rough: f32,
    pub gain: f32,
}

impl Frame {
    pub const SILENT: Frame = Frame { ratios: [0.0; ORDER], f0: 100.0, voicing: 0.0, rough: 0.0, gain: 0.0 };
}

fn levels(bits: u8) -> f32 {
    ((1u32 << bits) - 1) as f32
}

fn quantize(value: f32, low: f32, high: f32, bits: u8) -> u8 {
    (((value - low) / (high - low)).clamp(0.0, 1.0) * levels(bits)).round() as u8
}

fn unquantize(code: u8, low: f32, high: f32, bits: u8) -> f32 {
    low + (high - low) * code as f32 / levels(bits)
}

/// A coefficient's code. These have to be able to say exactly zero: a quantizer that can
/// only say "a little either way" adds resonances that were never asked for.
fn quantize_coefficient(value: f32, i: usize) -> u8 {
    let half = (1u32 << (COEFFICIENT_BITS[i] - 1)) as f32;
    let step = COEFFICIENT_RANGE[i] / half;
    ((value / step).round() + half).clamp(0.0, 2.0 * half - 1.0) as u8
}

fn unquantize_coefficient(code: u8, i: usize) -> f32 {
    let half = (1u32 << (COEFFICIENT_BITS[i] - 1)) as f32;
    (code as f32 - half) * COEFFICIENT_RANGE[i] / half
}

/// The filter a tract's resonances amount to, as reflection coefficients in log-area form.
fn analyse(target: &Target, sample_rate: f32) -> [f32; ORDER] {
    use std::f32::consts::PI;
    let mut a = [0.0f32; ORDER + 1];
    a[0] = 1.0;
    let mut used = 0;
    let mut multiply = |a1: f32, a2: f32| {
        for i in (1..=(used + 2).min(ORDER)).rev() {
            a[i] += a1 * a[i - 1] + if i >= 2 { a2 * a[i - 2] } else { 0.0 };
        }
        used += 2;
    };
    // The source's slope: two real poles.
    multiply(-1.5 * target.tilt, 0.5 * target.tilt * target.tilt);
    for (formant, bandwidth) in target.formants.iter().zip(&target.bandwidths) {
        // Above the band it can't exist, and the codec simply has one resonance fewer.
        if *formant < 0.47 * sample_rate {
            let radius = (-PI * bandwidth / sample_rate).exp();
            multiply(-2.0 * radius * (2.0 * PI * formant / sample_rate).cos(), radius * radius);
        } else {
            multiply(0.0, 0.0);
        }
    }

    // Step down from the polynomial to the lattice.
    let mut ratios = [0.0; ORDER];
    for m in (1..=ORDER).rev() {
        let k = a[m].clamp(-LARGEST_REFLECTION, LARGEST_REFLECTION);
        ratios[m - 1] = ((1.0 + k) / (1.0 - k)).ln();
        let denominator = 1.0 - k * k;
        let before = a;
        for i in 1..m {
            a[i] = (before[i] - k * before[m - i]) / denominator;
        }
    }
    ratios
}

pub fn encode(target: &Target, sample_rate: f32) -> Packet {
    let ratios = analyse(target, sample_rate);
    let gain_db = 20.0 * target.gain.max(1e-6).log10();
    let gain_codes = levels(GAIN_BITS);
    let gain = (gain_codes - (GAIN_TOP_DB - gain_db) / GAIN_STEP_DB).round().clamp(0.0, gain_codes) as u8;
    Packet {
        coefficients: std::array::from_fn(|i| quantize_coefficient(ratios[i], i)),
        pitch: quantize(target.f0.clamp(PITCH_MIN_HZ, PITCH_MAX_HZ).ln(), PITCH_MIN_HZ.ln(), PITCH_MAX_HZ.ln(), PITCH_BITS),
        voicing: quantize(target.voicing, 0.0, 1.0, VOICING_BITS),
        rough: quantize(target.rough, 0.0, 1.0, ROUGH_BITS),
        gain,
    }
}

pub fn decode(packet: &Packet) -> Frame {
    let gain_codes = levels(GAIN_BITS);
    Frame {
        ratios: std::array::from_fn(|i| unquantize_coefficient(packet.coefficients[i], i)),
        f0: unquantize(packet.pitch, PITCH_MIN_HZ.ln(), PITCH_MAX_HZ.ln(), PITCH_BITS).exp(),
        voicing: unquantize(packet.voicing, 0.0, 1.0, VOICING_BITS),
        rough: unquantize(packet.rough, 0.0, 1.0, ROUGH_BITS),
        gain: if packet.gain == 0 { 0.0 } else { 10f32.powf((GAIN_TOP_DB - (gain_codes - packet.gain as f32) * GAIN_STEP_DB) / 20.0) },
    }
}

/// A channel that's fine for a while and then isn't: errors come in bursts.
pub struct Line {
    bad: bool,
    last: Frame,
    rng: Rng,
}

impl Line {
    pub fn new() -> Self {
        Self { bad: false, last: Frame::SILENT, rng: Rng::new(0x2400) }
    }

    fn flip(&mut self, value: u8, bits: u8, rate: f32) -> u8 {
        (0..bits).fold(value, |value, bit| if self.rng.chance(rate) { value ^ (1 << bit) } else { value })
    }

    /// One frame's trip. `damage` is 0 for a clean line, 1 for a terrible one.
    pub fn carry(&mut self, packet: Packet, damage: f32) -> Frame {
        if damage <= 0.0 {
            self.bad = false;
            self.last = decode(&packet);
            return self.last;
        }
        // Into a bad patch rarely on a decent line; out again at a steady rate.
        let enter = damage * damage * 0.3;
        self.bad = if self.bad { !self.rng.chance(0.25) } else { self.rng.chance(enter) };
        if !self.bad {
            self.last = decode(&packet);
            return self.last;
        }
        if self.rng.chance(0.25 + 0.3 * damage) {
            // The frame's gone: say the last one again, fading.
            self.last.gain *= CONCEALMENT_FADE;
            return self.last;
        }
        let rate = 0.03 + 0.2 * damage;
        let mut hit = packet;
        for (i, code) in hit.coefficients.iter_mut().enumerate() {
            *code = self.flip(*code, COEFFICIENT_BITS[i], rate);
        }
        hit.pitch = self.flip(hit.pitch, PITCH_BITS, rate);
        hit.voicing = self.flip(hit.voicing, VOICING_BITS, rate);
        hit.rough = self.flip(hit.rough, ROUGH_BITS, rate);
        hit.gain = self.flip(hit.gain, GAIN_BITS, rate * 0.3);
        self.last = decode(&hit);
        self.last
    }
}

impl Default for Line {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vowel() -> Target {
        Target {
            formants: [700.0, 1200.0, 2500.0, 3500.0, 4500.0, 5500.0],
            bandwidths: [60.0, 80.0, 120.0, 175.0, 280.0, 400.0],
            tilt: 0.9,
            f0: 120.0,
            voicing: 1.0,
            gain: 0.3,
            rough: 0.0,
        }
    }

    #[test]
    fn a_frame_survives_a_clean_line_about_intact() {
        let frame = decode(&encode(&vowel(), 16_000.0));
        assert!((frame.f0 - 120.0).abs() < 4.0, "{}", frame.f0);
        assert!((frame.gain - 0.3).abs() < 0.03, "{}", frame.gain);
        assert_eq!(frame.voicing, 1.0);
        let exact = analyse(&vowel(), 16_000.0);
        for i in 0..ORDER {
            let step = COEFFICIENT_RANGE[i] / (1u32 << (COEFFICIENT_BITS[i] - 1)) as f32;
            assert!((frame.ratios[i] - exact[i]).abs() <= step * 0.51 || exact[i].abs() >= COEFFICIENT_RANGE[i] * 0.95, "{i}");
        }
    }

    #[test]
    fn every_bit_pattern_decodes_to_a_stable_filter() {
        let mut line = Line::new();
        for _ in 0..20_000 {
            let frame = line.carry(encode(&vowel(), 11_000.0), 1.0);
            assert!(frame.ratios.iter().all(|r| r.is_finite()));
            assert!(frame.ratios.iter().all(|r| (r * 0.5).tanh().abs() < 1.0));
            assert!(frame.f0 >= PITCH_MIN_HZ * 0.99 && frame.f0 <= PITCH_MAX_HZ * 1.01);
        }
    }

    #[test]
    fn a_clean_line_changes_nothing_and_a_bad_one_does() {
        let packet = encode(&vowel(), 11_000.0);
        let clean = decode(&packet);
        let mut line = Line::new();
        assert_eq!(line.carry(packet, 0.0).ratios, clean.ratios);
        let different = (0..500).filter(|_| line.carry(packet, 1.0).ratios != clean.ratios).count();
        assert!(different > 150, "{different}");
    }

    #[test]
    fn a_silent_filter_decodes_to_exactly_nothing() {
        for i in 0..ORDER {
            assert_eq!(unquantize_coefficient(quantize_coefficient(0.0, i), i), 0.0);
        }
    }

    #[test]
    fn a_formant_above_the_band_just_drops_out() {
        let ratios = analyse(&Target { formants: [700.0, 1200.0, 2500.0, 3500.0, 4500.0, 9000.0], ..vowel() }, 8_000.0);
        assert!(ratios.iter().all(|r| r.is_finite()));
    }
}
