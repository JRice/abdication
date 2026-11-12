//! Vintage: one knob of analogue imperfection, per voice, as Abdication's (voice.lib).
//!
//! Each voice gets its own stable offsets, as if its parts were slightly out of tolerance
//! (tuning, cutoff, envelope times), and its pitch drifts slowly. The offsets are
//! correlated, as on GForce's OB-X: each voice has two identity values, and each trait mixes
//! them with its own weights plus a part of its own, so a voice that runs sharp also tends
//! to be brighter and quicker. Each voice also has two noises of its own, growing with the
//! knob: one on its pitch, one on its level. Near the top a tape-style wow and flutter come
//! in (they grow with the square of the knob, so the low end stays subtle).
//!
//! Worked out once a block (up to 32 host samples): everything here moves below 60 Hz.

use super::random::Rng;

/// At full Vintage: the spreads of the stable offsets.
const TUNING_CENTS: f32 = 8.0;
const DRIFT_CENTS: f32 = 10.0;
const CUTOFF_OCTAVES: f32 = 0.25;
const CUTOFF_JITTER_OCTAVES: f32 = 0.1;
const ENVELOPE_SPREAD: f32 = 0.25;
/// At full Vintage: the noises (RMS) and their lowpasses.
const NOISE_FM_CENTS: f32 = 3.0;
const NOISE_FM_HZ: f32 = 60.0;
const NOISE_AM_DB: f32 = 0.7;
const NOISE_AM_HZ: f32 = 25.0;
/// At full Vintage (they grow with its square): wow and flutter.
const WOW_CENTS: f32 = 18.0;
const FLUTTER_CENTS: f32 = 2.0;
const FLUTTER_HZ: f32 = 6.5;

/// The traits, and their weights on the voice's two identity values (Abdication's
/// traitW1/traitW2), with traitOwn its own part.
#[derive(Clone, Copy)]
enum Trait {
    Tuning,
    DriftRate,
    WowRate,
    Cutoff,
    CutoffJitterRate,
    Attack,
    Decay,
    Release,
}
const TRAITS: usize = 8;
const WEIGHTS: [(f32, f32); TRAITS] =
    [(0.7, 0.3), (0.0, 0.6), (0.0, 0.5), (0.6, -0.4), (0.0, 0.3), (-0.3, 0.6), (-0.4, 0.5), (-0.4, 0.5)];
const OWN: f32 = 0.5;

/// A slow random wander, -1..1-ish: a new random target every 1/rate s, followed by a
/// one-pole at that rate (core.lib's jitter).
#[derive(Clone)]
struct Jitter {
    rate: f32,
    clock: f32,
    target: f32,
    value: f32,
}

impl Jitter {
    fn new(rate: f32, rng: &mut Rng) -> Self {
        let start = rng.signed();
        Self { rate, clock: rng.unit(), target: start, value: start }
    }

    fn step(&mut self, dt: f32, rng: &mut Rng) -> f32 {
        self.clock += self.rate * dt;
        if self.clock >= 1.0 {
            self.clock -= self.clock.floor();
            self.target = rng.signed();
        }
        self.value += (self.target - self.value) * (1.0 - (-std::f32::consts::TAU * self.rate * dt).exp());
        self.value
    }
}

/// White noise through a one-pole lowpass, scaled to unit RMS.
#[derive(Clone)]
struct Noise {
    hz: f32,
    value: f32,
}

impl Noise {
    fn step(&mut self, dt: f32, rng: &mut Rng) -> f32 {
        let a = 1.0 - (-std::f32::consts::TAU * self.hz * dt).exp();
        self.value += (rng.gaussian() - self.value) * a;
        // A one-pole's output RMS is sqrt(a / (2 - a)) of its input's.
        self.value / (a / (2.0 - a)).sqrt()
    }
}

/// One voice's tolerances and wanderings.
#[derive(Clone)]
pub struct Vintage {
    traits: [f32; TRAITS],
    rng: Rng,
    drift: Jitter,
    cutoff: Jitter,
    wow: f32,
    flutter: f32,
    noise_fm: Noise,
    noise_am: Noise,
}

/// What Vintage does to a voice in one block.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Offsets {
    pub cents: f32,
    pub cutoff: f32,
    pub gain: f32,
    /// Factors on the envelopes' attack, decay and release times.
    pub attack: f32,
    pub decay: f32,
    pub release: f32,
}

impl Offsets {
    pub const NONE: Offsets = Offsets { cents: 0.0, cutoff: 1.0, gain: 1.0, attack: 1.0, decay: 1.0, release: 1.0 };
}

/// A stable value in -1..1 for voice `slot` and key `k` (voice.lib's hash).
fn hash(slot: usize, k: f32) -> f32 {
    let v = ((slot as f32 * 12.9898 + k * 78.233).sin() * 43_758.547).fract();
    2.0 * v.abs() - 1.0
}

impl Vintage {
    pub fn new(slot: usize) -> Self {
        let (id1, id2) = (hash(slot, 101.0), hash(slot, 102.0));
        let traits = std::array::from_fn(|k| {
            let (w1, w2) = WEIGHTS[k];
            ((w1 * id1 + w2 * id2 + OWN * hash(slot, k as f32 + 1.0)) / (w1 * w1 + w2 * w2 + OWN * OWN).sqrt()).clamp(-1.0, 1.0)
        });
        let mut rng = Rng::new(0xA11A_0000 + slot as u32 * 409);
        let t = |k: Trait| traits[k as usize];
        let drift = Jitter::new(0.25 + 0.15 * t(Trait::DriftRate), &mut rng);
        let cutoff = Jitter::new(0.2 + 0.1 * t(Trait::CutoffJitterRate), &mut rng);
        let (wow, flutter) = (rng.unit(), rng.unit());
        Self {
            traits,
            rng,
            drift,
            cutoff,
            wow,
            flutter,
            noise_fm: Noise { hz: NOISE_FM_HZ, value: 0.0 },
            noise_am: Noise { hz: NOISE_AM_HZ, value: 0.0 },
        }
    }

    fn trait_of(&self, k: Trait) -> f32 {
        self.traits[k as usize]
    }

    /// The next `dt` seconds at Vintage `amount` (0..1). At 0 nothing moves.
    pub fn step(&mut self, amount: f32, dt: f32) -> Offsets {
        if amount < 1e-4 {
            return Offsets::NONE;
        }
        let rng = &mut self.rng;
        let drift = 2.0 * self.drift.step(dt, rng);
        let jitter = self.cutoff.step(dt, rng);
        let (fm, am) = (self.noise_fm.step(dt, rng), self.noise_am.step(dt, rng));
        self.wow = (self.wow + (0.45 + 0.1 * self.trait_of(Trait::WowRate)) * dt).fract();
        self.flutter = (self.flutter + FLUTTER_HZ * dt).fract();
        let tau = std::f32::consts::TAU;
        let cents = amount * (TUNING_CENTS * self.trait_of(Trait::Tuning) + DRIFT_CENTS * drift + NOISE_FM_CENTS * fm)
            + amount * amount * (WOW_CENTS * (tau * self.wow).sin() + FLUTTER_CENTS * (tau * self.flutter).sin());
        let vary = |k: Trait| 1.0 + amount * ENVELOPE_SPREAD * self.trait_of(k);
        Offsets {
            cents,
            cutoff: (amount * (CUTOFF_OCTAVES * self.trait_of(Trait::Cutoff) + CUTOFF_JITTER_OCTAVES * jitter)).exp2(),
            gain: (amount * NOISE_AM_DB * am / 6.0206).exp2(),
            attack: vary(Trait::Attack),
            decay: vary(Trait::Decay),
            release: vary(Trait::Release),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn correlation(a: &[f32], b: &[f32]) -> f32 {
        let n = a.len() as f32;
        let (ma, mb) = (a.iter().sum::<f32>() / n, b.iter().sum::<f32>() / n);
        let cov: f32 = a.iter().zip(b).map(|(x, y)| (x - ma) * (y - mb)).sum();
        let (va, vb): (f32, f32) = (a.iter().map(|x| (x - ma).powi(2)).sum(), b.iter().map(|y| (y - mb).powi(2)).sum());
        cov / (va * vb).sqrt()
    }

    #[test]
    fn off_does_nothing() {
        let mut v = Vintage::new(3);
        assert_eq!(v.step(0.0, 0.001), Offsets::NONE);
    }

    /// Over 64 voices the traits spread as on Abdication, correlated as it is: a sharp voice
    /// tends to be brighter, and a slow attack goes with a slow decay.
    #[test]
    fn voices_differ_and_their_traits_correlate() {
        let voices: Vec<Vintage> = (0..64).map(Vintage::new).collect();
        let t = |k: Trait| voices.iter().map(|v| v.trait_of(k)).collect::<Vec<_>>();
        let (tuning, cutoff, attack, decay) = (t(Trait::Tuning), t(Trait::Cutoff), t(Trait::Attack), t(Trait::Decay));
        let spread = (tuning.iter().map(|x| x * x).sum::<f32>() / 64.0).sqrt();
        assert!(spread > 0.35 && spread < 0.8, "{spread}");
        assert!(correlation(&tuning, &cutoff) > 0.2, "{}", correlation(&tuning, &cutoff));
        assert!(correlation(&attack, &decay) > 0.4, "{}", correlation(&attack, &decay));
    }

    /// At full Vintage: tuning within the spreads, the pitch noise near its RMS, a level
    /// noise near 0.7 dB.
    #[test]
    fn full_vintage_stays_within_its_ranges() {
        let mut v = Vintage::new(5);
        let dt = 32.0 / 48_000.0;
        let steps: Vec<Offsets> = (0..48_000 * 10 / 32).map(|_| v.step(1.0, dt)).collect();
        let max_cents = steps.iter().fold(0f32, |m, o| m.max(o.cents.abs()));
        assert!(max_cents < TUNING_CENTS + 2.0 * DRIFT_CENTS + 5.0 * NOISE_FM_CENTS + WOW_CENTS + FLUTTER_CENTS, "{max_cents}");
        let db: Vec<f32> = steps.iter().map(|o| 20.0 * o.gain.log10()).collect();
        let rms = (db.iter().map(|d| d * d).sum::<f32>() / db.len() as f32).sqrt();
        assert!(rms > 0.4 && rms < 1.0, "level noise {rms} dB RMS");
        assert!(steps.iter().all(|o| o.cutoff > 0.75 && o.cutoff < 1.35));
    }
}
