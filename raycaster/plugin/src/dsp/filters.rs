//! The voice's tube drive and its two filters (a 2-pole state-variable and a 4-pole
//! ladder), plus the decimator that brings the oversampled voices back down.

use std::f32::consts::PI;

/// Cheap tanh, exact enough for saturation: |error| < 2% and it reaches ±1 at ±3.
#[inline(always)]
pub fn soft_clip(x: f32) -> f32 {
    if x > 3.0 {
        1.0
    } else if x < -3.0 {
        -1.0
    } else {
        x * (27.0 + x * x) / (27.0 + 9.0 * x * x)
    }
}

/// tanh to within about 1e-6 (a Pade approximant, clamped where it would pass 1).
#[inline(always)]
pub fn fast_tanh(x: f32) -> f32 {
    let x = x.clamp(-4.97, 4.97);
    let x2 = x * x;
    let a = x * (135_135.0 + x2 * (17_325.0 + x2 * (378.0 + x2)));
    let b = 135_135.0 + x2 * (62_370.0 + x2 * (3_150.0 + 28.0 * x2));
    (a / b).clamp(-1.0, 1.0)
}

/// A state-variable filter, stable under modulation (Zavalishin's TPT form).
#[derive(Clone, Copy, Default)]
pub struct Svf {
    ic1: f32,
    ic2: f32,
    g: f32,
    k: f32,
    a1: f32,
    a2: f32,
}

impl Svf {
    /// `resonance` 0..1: from a gentle bump to a ringing peak.
    pub fn set(&mut self, hz: f32, resonance: f32, rate: f32) {
        let q = 0.6 + 22.0 * resonance * resonance;
        self.g = (PI * hz.clamp(10.0, 0.45 * rate) / rate).tan();
        self.k = 1.0 / q;
        self.a1 = 1.0 / (1.0 + self.g * (self.g + self.k));
        self.a2 = self.g * self.a1;
    }

    /// One sample in; the band and low outputs.
    #[inline(always)]
    fn tick(&mut self, x: f32) -> (f32, f32) {
        let v3 = x - self.ic2;
        let v1 = self.a1 * self.ic1 + self.a2 * v3;
        let v2 = self.ic2 + self.g * v1;
        self.ic1 = 2.0 * v1 - self.ic1;
        self.ic2 = 2.0 * v2 - self.ic2;
        (v1, v2)
    }

    #[inline(always)]
    pub fn low(&mut self, x: f32) -> f32 {
        self.tick(x).1
    }

    /// `mode` 0..1 sweeps lowpass (0) through bandpass (½) to highpass (1). The band peaks as
    /// high as the lowpass's resonance does, so the sweep keeps its level.
    #[inline(always)]
    pub fn morph(&mut self, x: f32, mode: f32) -> f32 {
        let (band, low) = self.tick(x);
        let high = x - self.k * band - low;
        let m = 2.0 * mode.clamp(0.0, 1.0);
        if m <= 1.0 {
            low + (band - low) * m
        } else {
            band + (high - band) * (m - 1.0)
        }
    }

    pub fn clear(&mut self) {
        (self.ic1, self.ic2) = (0.0, 0.0);
    }

    /// Takes another filter's settings, keeping this one's state.
    pub fn follow(&mut self, other: &Svf) {
        (self.g, self.k, self.a1, self.a2) = (other.g, other.k, other.a1, other.a2);
    }
}

/// A 4-pole ladder lowpass with zero-delay feedback and a saturating input, so it
/// self-oscillates at full resonance and growls when pushed.
#[derive(Clone, Copy, Default)]
pub struct Ladder {
    s: [f32; 4],
    big_g: f32,
    k: f32,
}

impl Ladder {
    pub fn set(&mut self, hz: f32, resonance: f32, rate: f32) {
        let g = (PI * hz.clamp(10.0, 0.45 * rate) / rate).tan();
        self.big_g = g / (1.0 + g);
        self.k = 4.2 * resonance;
    }

    #[inline(always)]
    pub fn low(&mut self, x: f32) -> f32 {
        let g = self.big_g;
        let h = 1.0 - g;
        let g2 = g * g;
        let g4 = g2 * g2;
        let sigma = g2 * g * h * self.s[0] + g2 * h * self.s[1] + g * h * self.s[2] + h * self.s[3];
        // The bass a ladder loses to its own resonance, partly given back.
        let input = x * (1.0 + 0.35 * self.k);
        let estimate = (g4 * input + sigma) / (1.0 + self.k * g4);
        let mut u = soft_clip(input - self.k * estimate);
        for s in &mut self.s {
            let v = (u - *s) * g;
            let y = v + *s;
            *s = y + v;
            u = y;
        }
        u
    }

    pub fn clear(&mut self) {
        self.s = [0.0; 4];
    }

    /// Takes another filter's settings, keeping this one's state.
    pub fn follow(&mut self, other: &Ladder) {
        (self.big_g, self.k) = (other.big_g, other.k);
    }
}

/// Removes DC: a highpass at about 5 Hz.
#[derive(Clone, Copy, Default)]
pub struct DcBlock {
    x1: f32,
    y1: f32,
    pole: f32,
}

impl DcBlock {
    pub fn new(rate: f32) -> Self {
        Self { x1: 0.0, y1: 0.0, pole: (-2.0 * PI * 5.0 / rate).exp() }
    }

    #[inline(always)]
    pub fn process(&mut self, x: f32) -> f32 {
        let y = x - self.x1 + self.pole * self.y1;
        self.x1 = x;
        self.y1 = y;
        y
    }

    pub fn clear(&mut self) {
        (self.x1, self.y1) = (0.0, 0.0);
    }
}

/// Tube drive before the filter: an asymmetric tanh, so it adds even harmonics, with up to
/// +40 dB into it. The driven sound is held at the clean sound's level (both followed over
/// about 30 ms), so Drive changes the colour, not the loudness. Amount 0 is a true bypass;
/// the first tenth fades the drive in.
#[derive(Clone, Copy, Default)]
pub struct Tube {
    dc: DcBlock,
    power_in: f32,
    power_out: f32,
    follow: f32,
}

impl Tube {
    pub fn new(rate: f32) -> Self {
        Self { dc: DcBlock::new(rate), power_in: 0.0, power_out: 0.0, follow: 1.0 - (-1.0 / (0.03 * rate)).exp() }
    }

    #[inline(always)]
    pub fn process(&mut self, x: f32, drive: &TubeDrive) -> f32 {
        if drive.mix <= 0.0 {
            return x;
        }
        let wet = self.dc.process((fast_tanh(drive.gain * x + drive.bias) - drive.offset) * drive.makeup);
        self.power_in += (x * x - self.power_in) * self.follow;
        self.power_out += (wet * wet - self.power_out) * self.follow;
        let matched = ((self.power_in + 1e-9) / (self.power_out + 1e-9)).sqrt().clamp(0.05, 4.0);
        x + (wet * matched - x) * drive.mix
    }

    pub fn clear(&mut self) {
        self.dc.clear();
        self.power_in = 0.0;
        self.power_out = 0.0;
    }
}

/// A drive amount worked out once a block.
#[derive(Clone, Copy, Debug)]
pub struct TubeDrive {
    gain: f32,
    bias: f32,
    offset: f32,
    makeup: f32,
    mix: f32,
}

impl TubeDrive {
    const REFERENCE: f32 = 0.35;

    pub fn new(amount: f32) -> Self {
        let amount = amount.clamp(0.0, 1.0);
        let gain = 10f32.powf(40.0 * amount / 20.0);
        let bias = 0.35 * amount;
        let offset = bias.tanh();
        let swing = (gain * Self::REFERENCE + bias).tanh() - offset;
        Self { gain, bias, offset, makeup: Self::REFERENCE / swing.max(1e-6), mix: (amount * 10.0).min(1.0) }
    }
}

/// Halves the sample rate: a 63-tap halfband lowpass (Kaiser window), then every second
/// sample. Only the odd taps and the centre are non-zero.
pub struct Decimator {
    history: [f32; 64],
    pos: usize,
}

const HALFBAND_TAPS: usize = 63;
const HALFBAND_CENTER: usize = HALFBAND_TAPS / 2;

fn bessel_i0(x: f32) -> f32 {
    let (mut sum, mut term) = (1.0f64, 1.0f64);
    let x = x as f64;
    for k in 1..40 {
        term *= (x / (2.0 * k as f64)).powi(2);
        sum += term;
    }
    sum as f32
}

pub fn halfband() -> [f32; HALFBAND_TAPS] {
    let beta = 8.6f32;
    let mut h = [0.0f32; HALFBAND_TAPS];
    for (n, tap) in h.iter_mut().enumerate() {
        let m = n as f32 - HALFBAND_CENTER as f32;
        let sinc = if m == 0.0 { 0.5 } else { (0.5 * PI * m).sin() / (PI * m) };
        let r = m / HALFBAND_CENTER as f32;
        let window = bessel_i0(beta * (1.0 - r * r).max(0.0).sqrt()) / bessel_i0(beta);
        *tap = sinc * window;
    }
    let sum: f32 = h.iter().sum();
    h.iter_mut().for_each(|t| *t /= sum);
    h
}

/// Doubles the sample rate with the same halfband: each input gives two outputs, one
/// filtered and one simply delayed (the halfband's only non-zero odd tap is its centre).
pub struct Interpolator {
    history: [f32; 32],
    pos: usize,
}

impl Default for Interpolator {
    fn default() -> Self {
        Self { history: [0.0; 32], pos: 0 }
    }
}

impl Interpolator {
    #[inline]
    pub fn process(&mut self, taps: &[f32; HALFBAND_TAPS], x: f32) -> [f32; 2] {
        self.history[self.pos] = x;
        self.pos = (self.pos + 1) & 31;
        let at = |age: usize| self.history[(self.pos + 31 - age) & 31];
        // Even taps 0, 2 .. 62 meet inputs 0 .. 31 samples old.
        let mut even = 0.0;
        for k in 0..16 {
            even += taps[2 * k] * (at(k) + at(31 - k));
        }
        [2.0 * even, 2.0 * taps[HALFBAND_CENTER] * at(15)]
    }

    pub fn clear(&mut self) {
        self.history = [0.0; 32];
    }
}

impl Default for Decimator {
    fn default() -> Self {
        Self { history: [0.0; 64], pos: 0 }
    }
}

impl Decimator {
    /// Takes two oversampled samples and gives one.
    #[inline]
    pub fn process(&mut self, taps: &[f32; HALFBAND_TAPS], a: f32, b: f32) -> f32 {
        self.history[self.pos] = a;
        self.pos = (self.pos + 1) & 63;
        self.history[self.pos] = b;
        self.pos = (self.pos + 1) & 63;
        // history[pos - 1] is the newest sample.
        let at = |age: usize| self.history[(self.pos + 63 - age) & 63];
        let mut y = taps[HALFBAND_CENTER] * at(HALFBAND_CENTER);
        let mut n = 0;
        while n < HALFBAND_TAPS {
            y += taps[n] * (at(n) + at(HALFBAND_TAPS - 1 - n));
            n += 2;
            if n >= HALFBAND_CENTER {
                break;
            }
        }
        y
    }

    pub fn clear(&mut self) {
        self.history = [0.0; 64];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gain_at(mut f: impl FnMut(f32) -> f32, hz: f32, rate: f32) -> f32 {
        let n = (rate * 0.5) as usize;
        let mut peak = 0.0f32;
        for i in 0..n {
            let y = f((2.0 * PI * hz * i as f32 / rate).sin());
            if i > n / 2 {
                peak = peak.max(y.abs());
            }
        }
        peak
    }

    #[test]
    fn fast_tanh_is_close() {
        for i in -800..800 {
            let x = i as f32 / 100.0;
            let tolerance = if x.abs() < 3.0 { 2e-6 } else { 2e-4 };
            assert!((fast_tanh(x) - x.tanh()).abs() < tolerance, "{x}");
        }
    }

    #[test]
    fn the_svf_mode_sweeps_low_band_high() {
        let rate = 96_000.0;
        let mut svf = Svf::default();
        svf.set(1_000.0, 0.0, rate);
        let mut at = |mode: f32, hz: f32| {
            svf.clear();
            gain_at(|x| svf.morph(x, mode), hz, rate)
        };
        assert!(at(0.0, 100.0) > 0.9 && at(0.0, 8_000.0) < 0.05, "lowpass");
        assert!(at(0.5, 100.0) < 0.2 && at(0.5, 8_000.0) < 0.2 && at(0.5, 1_000.0) > 0.5, "bandpass");
        assert!(at(1.0, 100.0) < 0.05 && at(1.0, 8_000.0) > 0.9, "highpass");
    }

    #[test]
    fn both_filters_pass_lows_and_cut_highs() {
        let rate = 96_000.0;
        let mut svf = Svf::default();
        svf.set(1_000.0, 0.0, rate);
        assert!(gain_at(|x| svf.low(x), 100.0, rate) > 0.9);
        svf.clear();
        assert!(gain_at(|x| svf.low(x), 8_000.0, rate) < 0.05);
        let mut ladder = Ladder::default();
        ladder.set(1_000.0, 0.0, rate);
        assert!(gain_at(|x| 2.0 * ladder.low(0.5 * x), 100.0, rate) > 0.8);
        ladder.clear();
        assert!(gain_at(|x| 2.0 * ladder.low(0.5 * x), 8_000.0, rate) < 0.01);
    }

    #[test]
    fn the_ladder_rings_at_full_resonance_and_stays_finite() {
        let rate = 96_000.0;
        let mut ladder = Ladder::default();
        ladder.set(800.0, 1.0, rate);
        let mut out = 0.0f32;
        for i in 0..96_000 {
            let kick = if i == 0 { 0.5 } else { 0.0 };
            out = ladder.low(kick);
            assert!(out.is_finite());
        }
        assert!(out.abs() > 1e-3, "should still be ringing: {out}");
    }

    #[test]
    fn drive_off_is_a_bypass_and_on_holds_the_level() {
        let mut tube = Tube::new(96_000.0);
        let off = TubeDrive::new(0.0);
        assert_eq!(tube.process(0.3, &off), 0.3);
        let on = TubeDrive::new(1.0);
        for input in [0.05, 0.35] {
            let mut tube = Tube::new(96_000.0);
            let level = gain_at(|x| tube.process(input * x, &on), 200.0, 96_000.0);
            assert!(level > 0.5 * input && level < 2.5 * input, "{input} -> {level}");
        }
    }

    #[test]
    fn up_then_down_gives_back_the_audio() {
        let taps = halfband();
        let (mut up, mut down) = (Interpolator::default(), Decimator::default());
        let mut peak = 0.0f32;
        for i in 0..48_000 {
            let x = (2.0 * std::f64::consts::PI * 5_000.0 * i as f64 / 48_000.0).sin() as f32;
            let [a, b] = up.process(&taps, x);
            let y = down.process(&taps, a, b);
            if i > 1_000 {
                peak = peak.max(y.abs());
            }
        }
        assert!((peak - 1.0).abs() < 0.01, "{peak}");
    }

    #[test]
    fn the_decimator_passes_audio_and_stops_what_would_fold() {
        let taps = halfband();
        let rate = 96_000.0;
        let through = |hz: f32| {
            let mut d = Decimator::default();
            let mut peak = 0.0f32;
            for i in 0..48_000 {
                let s = |k: usize| (2.0 * std::f64::consts::PI * hz as f64 * k as f64 / rate as f64).sin() as f32;
                let y = d.process(&taps, s(2 * i), s(2 * i + 1));
                if i > 1_000 {
                    peak = peak.max(y.abs());
                }
            }
            peak
        };
        assert!((through(1_000.0) - 1.0).abs() < 0.01);
        assert!(through(18_000.0) > 0.95);
        assert!(through(32_000.0) < 0.001, "{}", through(32_000.0));
    }
}
