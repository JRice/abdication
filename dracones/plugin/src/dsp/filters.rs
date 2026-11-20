//! Each oscillator's lowpass, one of two characters (a lever on the panel):
//!
//! - SVF: Abdication's state-variable lowpass (dsp/core.lib's `svf`): a TPT SVF whose
//!   band-pass state goes through a lopsided tanh before it feeds back, so resonance
//!   thickens rather than rings, eased off at low cutoffs so low sweeps stay round.
//! - Wasp: after EDP's Wasp, whose state-variable filter is built from CMOS inverters
//!   run in their linear region: both integrators clip early and hard-ish (a rounded
//!   knee rather than a tanh), the input is pushed into them, and resonance gets
//!   raspy and loud instead of polite.
//!
//! Plus the small pieces the rest of the instrument uses: one-poles and DC blocks.

use std::f32::consts::PI;

/// Abdication's SVF constants (core.lib).
const ASYM: f32 = 0.15;
const LOW_Q_HZ: f32 = 750.0;
const LOW_Q_EASE: f32 = 1.2;
/// The SVF's loop heat (core.lib's 0.2 + 1.5 × character, at character 0.6).
const HEAT: f32 = 1.1;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Character {
    #[default]
    Wasp,
    Svf,
}

/// Cutoff knob (0..1) to Hz: 20 Hz to 18 kHz, evenly in octaves.
pub fn cutoff_hz(knob: f32) -> f32 {
    20.0 * 900f32.powf(knob.clamp(0.0, 1.0))
}

/// Resonance knob (0..1) to Q: 0.5 to 20, evenly in octaves.
pub fn resonance_q(knob: f32) -> f32 {
    0.5 * 40f32.powf(knob.clamp(0.0, 1.0))
}

#[inline(always)]
fn tanh(x: f32) -> f32 {
    // A rational fit, accurate to ~1e-4 over the range it matters, saturating at ±1.
    let x = x.clamp(-4.97, 4.97);
    let x2 = x * x;
    x * (27.0 + x2) / (27.0 + 9.0 * x2)
}

/// The CMOS inverter's knee: linear, then flattening sharply (a p-norm clip, p = 4),
/// with its two halves a little apart, as a real inverter's supply rails are.
#[inline(always)]
fn inverter(x: f32) -> f32 {
    let lim = if x >= 0.0 { 1.0 } else { 1.2 };
    let u = x / lim;
    let u2 = u * u;
    lim * u / (1.0 + u2 * u2).sqrt().sqrt()
}

/// One oscillator's filter. The cutoff and Q are set per block and glide across it.
#[derive(Clone, Copy, Default)]
pub struct Lowpass {
    s1: f32,
    s2: f32,
    g: f32,
    g_step: f32,
    k: f32,
    k_step: f32,
}

impl Lowpass {
    pub fn clear(&mut self) {
        self.s1 = 0.0;
        self.s2 = 0.0;
    }

    /// Head for `hz` and `q` over the next `n` samples.
    pub fn target(&mut self, character: Character, hz: f32, q: f32, rate: f32, n: usize) {
        let fc = hz.clamp(20.0, 0.45 * rate);
        let g = (PI * fc / rate).tan();
        let k = match character {
            Character::Svf => {
                let low = (1.0 - fc / LOW_Q_HZ).max(0.0);
                1.0 / (q / (1.0 + LOW_Q_EASE * low * low)).max(0.5)
            }
            // The Wasp lets its resonance run all the way to oscillation.
            Character::Wasp => (2.0 / q.max(0.5) - 0.08).max(0.0),
        };
        if self.g == 0.0 {
            (self.g, self.k) = (g, k);
        }
        let n = n.max(1) as f32;
        self.g_step = (g - self.g) / n;
        self.k_step = (k - self.k) / n;
    }

    #[inline]
    pub fn tick(&mut self, character: Character, x: f32) -> f32 {
        self.g += self.g_step;
        self.k += self.k_step;
        let (g, k) = (self.g, self.k);
        match character {
            Character::Svf => {
                let t0 = tanh(ASYM);
                let s1 = (tanh(self.s1 * HEAT + ASYM) - t0) / (HEAT * (1.0 - t0 * t0));
                let hp = (x - (k + g) * s1 - self.s2) / (1.0 + k * g + g * g);
                let v1 = g * hp;
                let bp = v1 + s1;
                let v2 = g * bp;
                let lp = v2 + self.s2;
                self.s1 = bp + v1;
                self.s2 = lp + v2;
                lp
            }
            Character::Wasp => {
                // Pushed in, both states clipped by the inverters, made up after.
                let x = 1.6 * x;
                let s1 = inverter(self.s1);
                let s2 = inverter(self.s2);
                let hp = (x - (k + g) * s1 - s2) / (1.0 + k * g + g * g);
                let v1 = g * hp;
                let bp = inverter(v1 + s1);
                let v2 = g * bp;
                let lp = v2 + s2;
                self.s1 = bp + v1;
                self.s2 = lp + v2;
                0.75 * lp
            }
        }
    }
}

/// A one-pole lowpass with its coefficient given.
#[derive(Clone, Copy, Default)]
pub struct OnePole {
    pub y: f32,
}

impl OnePole {
    #[inline(always)]
    pub fn low(&mut self, x: f32, a: f32) -> f32 {
        self.y += a * (x - self.y);
        self.y
    }

    #[inline(always)]
    pub fn high(&mut self, x: f32, a: f32) -> f32 {
        x - self.low(x, a)
    }
}

/// The one-pole coefficient for a corner at `hz`.
pub fn one_pole(hz: f32, rate: f32) -> f32 {
    1.0 - (-2.0 * PI * hz / rate).exp()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gain_at(character: Character, cut: f32, q: f32, hz: f32) -> f32 {
        let rate = 48_000.0;
        let mut f = Lowpass::default();
        f.target(character, cut, q, rate, 1);
        let n = 48_000;
        let mut peak = 0.0f32;
        for i in 0..n {
            let x = 0.05 * (2.0 * PI * hz * i as f32 / rate).sin();
            let y = f.tick(character, x);
            if i > n / 2 {
                peak = peak.max(y.abs());
            }
        }
        peak / 0.05
    }

    #[test]
    fn both_are_lowpasses() {
        for c in [Character::Svf, Character::Wasp] {
            let pass = gain_at(c, 2_000.0, 0.7, 100.0);
            let stop = gain_at(c, 2_000.0, 0.7, 12_000.0);
            assert!(pass > 0.6 && pass < 1.6, "{c:?} {pass}");
            assert!(stop < 0.1 * pass, "{c:?} {stop}");
        }
    }

    #[test]
    fn resonance_peaks_and_stays_bounded() {
        for c in [Character::Svf, Character::Wasp] {
            let peak = gain_at(c, 1_000.0, 12.0, 1_000.0);
            assert!(peak > 2.0, "{c:?} {peak}");
        }
        // The Wasp screaming on its own stays finite and bounded.
        let rate = 48_000.0;
        let mut f = Lowpass::default();
        f.target(Character::Wasp, 800.0, resonance_q(1.0), rate, 1);
        let mut y = 0.0f32;
        for i in 0..96_000 {
            let x = if i == 0 { 1.0 } else { 0.0 };
            y = y.max(f.tick(Character::Wasp, x).abs());
        }
        assert!(y.is_finite() && y < 4.0, "{y}");
    }
}
