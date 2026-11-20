//! The Choir mode's singers, after the BigSky's Chorale in spirit: the tank's tail is
//! sung by six voices. Each takes the tail through a short delay that wanders, so its
//! pitch drifts a few cents from the others', then through three vowel resonances
//! (formants), scaled by its own vocal-tract size, which wanders too. Breath is noise
//! that follows the tail's level through the same formants. See docs/design.md.

use std::f32::consts::PI;

use super::delay::Delay;
use super::filters::{Svf, SvfShape};

pub const VOICES: usize = 6;
const FORMANTS: usize = 3;

/// The vowels the Vowel control runs through, as F1–F3 in hertz (averages for an
/// adult male voice, after Peterson and Barney): AH, OH, OO.
const VOWELS: [[f32; FORMANTS]; 3] = [[730.0, 1_090.0, 2_440.0], [570.0, 840.0, 2_410.0], [300.0, 870.0, 2_240.0]];
pub const VOWEL_NAMES: [&str; 3] = ["AH", "OH", "OO"];
/// The formants' levels against the first.
const FORMANT_GAINS: [f32; FORMANTS] = [1.0, 0.63, 0.35];

/// Each voice's vocal tract against the table (bigger number, smaller singer, higher
/// formants), place in the stereo field (-1 left to 1 right) and delay.
const VOICE_SIZE: [f32; VOICES] = [0.94, 1.0, 1.07, 0.97, 1.03, 1.13];
const VOICE_PAN: [f32; VOICES] = [-0.9, -0.5, -0.15, 0.15, 0.5, 0.9];
const VOICE_DELAY_MS: [f32; VOICES] = [11.0, 17.0, 23.0, 13.0, 19.0, 29.0];

/// Ensemble at full: how far each voice's delay wanders (about ±20 cents at its
/// fastest) and how far its vocal tract stretches and shrinks.
const PITCH_WANDER_MS: f32 = 4.0;
const SIZE_WANDER: f32 = 0.07;
/// Drift at full: how far each voice's vowel strays, as a share of AH to OO.
const VOWEL_WANDER: f32 = 0.5;
/// Wandering heads for a new random target every this many seconds, give or take half.
const WANDER_S: f32 = 1.0;

/// Resonance sweeps the formants' Q between these.
const Q_LOW: f32 = 2.0;
const Q_HIGH: f32 = 16.0;
/// Narrower formants pass less of a broad tail, but as much of a note that lands in
/// them; this splits the difference, so neither gets much louder as Q rises.
fn q_makeup(q: f32) -> f32 {
    (q / Q_LOW).powf(0.35)
}

/// How much of the unsung tail stays under the voices, for body.
const BLEED: f32 = 0.15;
/// The voices together, brought to about the other modes' level.
const VOICES_GAIN: f32 = 5.0;
const BREATH_GAIN: f32 = 1.4;
const ENVELOPE_ATTACK_S: f32 = 0.01;
const ENVELOPE_RELEASE_S: f32 = 0.3;

#[derive(Clone, Copy)]
pub struct ChoirControls {
    /// AH at 0, OH at 0.5, OO at 1.
    pub vowel: f32,
    pub drift: f32,
    pub resonance: f32,
    pub ensemble: f32,
    pub breath: f32,
}

impl Default for ChoirControls {
    fn default() -> Self {
        Self { vowel: 0.0, drift: 0.3, resonance: 0.45, ensemble: 0.5, breath: 0.2 }
    }
}

/// The formants of a point on the AH–OH–OO line, moving smoothly (in log frequency)
/// from one vowel to the next.
pub fn vowel_formants(position: f32) -> [f32; FORMANTS] {
    let along = position.clamp(0.0, 1.0) * (VOWELS.len() - 1) as f32;
    let i = (along as usize).min(VOWELS.len() - 2);
    let t = along - i as f32;
    std::array::from_fn(|f| VOWELS[i][f] * (VOWELS[i + 1][f] / VOWELS[i][f]).powf(t))
}

/// The vowel nearest a position, or the pair it's between.
pub fn vowel_name(position: f32) -> String {
    let along = position.clamp(0.0, 1.0) * (VOWELS.len() - 1) as f32;
    let nearest = along.round();
    if (along - nearest).abs() < 0.1 {
        VOWEL_NAMES[nearest as usize].to_string()
    } else {
        let i = (along as usize).min(VOWELS.len() - 2);
        format!("{}·{}", VOWEL_NAMES[i], VOWEL_NAMES[i + 1])
    }
}

/// A tiny random source, the same every time.
struct Dice(u32);

impl Dice {
    fn roll(&mut self) -> f32 {
        self.0 = self.0.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        (self.0 >> 8) as f32 / (1 << 24) as f32
    }

    /// From -1 to 1.
    fn bipolar(&mut self) -> f32 {
        2.0 * self.roll() - 1.0
    }
}

/// A value from -1 to 1 that eases from one random target to the next.
struct Wander {
    from: f32,
    to: f32,
    phase: f32,
    rate: f32,
    dice: Dice,
    seconds: f32,
}

impl Wander {
    fn new(seed: u32, seconds: f32) -> Self {
        let mut dice = Dice(seed);
        let from = dice.bipolar();
        let to = dice.bipolar();
        let rate = 1.0 / (seconds * (0.5 + dice.roll()));
        Self { from, to, phase: dice.roll(), rate, dice, seconds }
    }

    /// Moves on by `dt` seconds.
    fn advance(&mut self, dt: f32) -> f32 {
        self.phase += dt * self.rate;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
            self.from = self.to;
            self.to = self.dice.bipolar();
            self.rate = 1.0 / (self.seconds * (0.5 + self.dice.roll()));
        }
        let eased = 0.5 - 0.5 * (PI * self.phase).cos();
        self.from + (self.to - self.from) * eased
    }
}

struct Voice {
    line: Delay,
    delay: f32,
    formants: [Svf; FORMANTS],
    shapes: [SvfShape; FORMANTS],
    pitch: Wander,
    size: Wander,
    vowel: Wander,
    /// This sample's wander in delay, in samples, and the step to the next control tick.
    pitch_offset: f32,
    pitch_step: f32,
}

impl Voice {
    fn new(i: usize, sample_rate: f32) -> Self {
        let seed = 0x9e37_79b9_u32.wrapping_mul(i as u32 + 1);
        Self {
            line: Delay::new((VOICE_DELAY_MS[i] + PITCH_WANDER_MS) * 1e-3 * sample_rate + 4.0),
            delay: VOICE_DELAY_MS[i] * 1e-3 * sample_rate,
            formants: Default::default(),
            shapes: [SvfShape::new(1_000.0, sample_rate); FORMANTS],
            pitch: Wander::new(seed, WANDER_S),
            size: Wander::new(seed ^ 0x5bd1_e995, 2.0 * WANDER_S),
            vowel: Wander::new(seed ^ 0x68e3_1da4, 3.0 * WANDER_S),
            pitch_offset: 0.0,
            pitch_step: 0.0,
        }
    }

    fn clear(&mut self) {
        self.line.clear();
        self.formants.iter_mut().for_each(Svf::clear);
    }

    #[inline]
    fn sing(&mut self, x: f32, breath: f32) -> f32 {
        let delayed = self.line.read(self.delay + self.pitch_offset);
        self.line.write(x);
        self.pitch_offset += self.pitch_step;
        let excitation = delayed + breath;
        (0..FORMANTS).map(|f| FORMANT_GAINS[f] * self.formants[f].band(excitation, &self.shapes[f])).sum()
    }
}

pub struct Choir {
    voices: Vec<Voice>,
    /// Each voice's share of the left and right inputs and outputs.
    pans: [[f32; 2]; VOICES],
    makeup: f32,
    envelope: f32,
    attack: f32,
    release: f32,
    breath: f32,
    noise: Dice,
    sample_rate: f32,
}

impl Choir {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            voices: (0..VOICES).map(|i| Voice::new(i, sample_rate)).collect(),
            pans: VOICE_PAN.map(|pan| [0.5 * (1.0 - pan), 0.5 * (1.0 + pan)]),
            makeup: 1.0,
            envelope: 0.0,
            attack: (-1.0 / (ENVELOPE_ATTACK_S * sample_rate)).exp(),
            release: (-1.0 / (ENVELOPE_RELEASE_S * sample_rate)).exp(),
            breath: 0.0,
            noise: Dice(0x1234_5678),
            sample_rate,
        }
    }

    pub fn clear(&mut self) {
        self.voices.iter_mut().for_each(Voice::clear);
        self.envelope = 0.0;
    }

    /// Moves every voice's wandering on by `samples` and works out its formants and
    /// the path of its delay until the next call.
    pub fn derive(&mut self, c: &ChoirControls, samples: u32) {
        let dt = samples as f32 / self.sample_rate;
        let q = Q_LOW * (Q_HIGH / Q_LOW).powf(c.resonance);
        self.makeup = VOICES_GAIN * q_makeup(q) / VOICES as f32 * 2.0;
        self.breath = c.breath * BREATH_GAIN;
        let pitch_reach = PITCH_WANDER_MS * 1e-3 * self.sample_rate * c.ensemble;
        for (i, voice) in self.voices.iter_mut().enumerate() {
            let target = voice.pitch.advance(dt) * pitch_reach;
            voice.pitch_step = (target - voice.pitch_offset) / samples as f32;
            let size = VOICE_SIZE[i] * (1.0 + SIZE_WANDER * c.ensemble * voice.size.advance(dt));
            let vowel = c.vowel + VOWEL_WANDER * c.drift * voice.vowel.advance(dt);
            let formants = vowel_formants(vowel);
            voice.shapes = std::array::from_fn(|f| SvfShape::resonant(formants[f] * size, q, self.sample_rate));
        }
    }

    #[inline]
    pub fn process(&mut self, tail: [f32; 2]) -> [f32; 2] {
        let level = 0.5 * (tail[0].abs() + tail[1].abs());
        let coefficient = if level > self.envelope { self.attack } else { self.release };
        self.envelope = level + coefficient * (self.envelope - level);
        let mut out = [0.0; 2];
        for (voice, [left, right]) in self.voices.iter_mut().zip(self.pans) {
            let breath = self.breath * self.envelope * self.noise.bipolar();
            let sung = voice.sing(left * tail[0] + right * tail[1], breath);
            out[0] += left * sung;
            out[1] += right * sung;
        }
        std::array::from_fn(|i| self.makeup * out[i] + BLEED * tail[i])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SR: f32 = 48_000.0;
    const PERIOD: u32 = 16;

    /// Sings two seconds of noise; returns the energy around 300 Hz (OO's first
    /// formant) and around 800 Hz (AH's first two), and the correlation of the sides.
    fn sing(c: &ChoirControls) -> (f32, f32, f32) {
        let mut choir = Choir::new(SR);
        let mut dice = Dice(99);
        let probes = [SvfShape::resonant(300.0, 2.0, SR), SvfShape::resonant(800.0, 2.0, SR)];
        let mut filters = [Svf::default(), Svf::default()];
        let mut energy = [0.0f32; 2];
        let (mut lr, mut ll, mut rr) = (0.0f32, 0.0f32, 0.0f32);
        for n in 0..(2.0 * SR) as usize {
            if n as u32 % PERIOD == 0 {
                choir.derive(c, PERIOD);
            }
            let x = 0.3 * dice.bipolar();
            let [l, r] = choir.process([x, x]);
            assert!(l.is_finite() && r.is_finite());
            if n > SR as usize / 2 {
                let m = 0.5 * (l + r);
                for i in 0..2 {
                    let y = filters[i].band(m, &probes[i]);
                    energy[i] += y * y;
                }
                (lr, ll, rr) = (lr + l * r, ll + l * l, rr + r * r);
            }
        }
        (energy[0], energy[1], lr / (ll * rr).sqrt())
    }

    #[test]
    fn ah_and_oo_sit_where_they_should() {
        let quiet = ChoirControls { drift: 0.0, breath: 0.0, ..Default::default() };
        let (ah_low, ah_high, _) = sing(&ChoirControls { vowel: 0.0, ..quiet });
        let (oo_low, oo_high, _) = sing(&ChoirControls { vowel: 1.0, ..quiet });
        let (ah, oo) = (ah_high / ah_low, oo_high / oo_low);
        assert!(ah > 3.0 * oo, "800 Hz against 300 Hz: AH {ah}, OO {oo}");
    }

    #[test]
    fn the_voices_spread_across_the_stereo_field() {
        let (_, _, correlation) = sing(&ChoirControls::default());
        assert!(correlation < 0.9, "{correlation}");
    }

    #[test]
    fn vowels_have_names_and_formants() {
        assert_eq!(vowel_name(0.0), "AH");
        assert_eq!(vowel_name(0.5), "OH");
        assert_eq!(vowel_name(0.75), "OH·OO");
        assert_eq!(vowel_formants(1.0), VOWELS[2]);
        let between = vowel_formants(0.25);
        assert!(between[0] < VOWELS[0][0] && between[0] > VOWELS[1][0]);
    }
}
