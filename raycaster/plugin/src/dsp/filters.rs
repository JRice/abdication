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

/// tanh(v) / v, the gain a tanh has at v (for linearizing one around its last input): the
/// cheap tanh's Padé form divided through by v, so one division and finite at 0 (within 2 %).
#[inline(always)]
fn tanh_ratio(v: f32) -> f32 {
    let v2 = v * v;
    if v2 < 9.0 {
        (27.0 + v2) / (27.0 + 9.0 * v2)
    } else {
        1.0 / v.abs()
    }
}

/// The SVF's character, as Abdication's (core.lib's svf, its Character at the middle).
const SVF_ASYM: f32 = 0.15; // offset into the loop's tanh: its limits 1.35 : 1 apart, near the Model 84's
const SVF_HEAT: f32 = 0.95; // loop gain into the tanh
const SVF_LOW_Q_HZ: f32 = 750.0; // below this, the resonance eases off ...
const SVF_LOW_Q_EASE: f32 = 1.2; // ... to Q / (1 + this) at 0 Hz

/// A state-variable filter, stable under modulation (Zavalishin's TPT form). As in the
/// Model 84 (and Abdication), the band integrator is read through a lopsided tanh,
/// re-centred so it passes 0 with a slope of 1: loud resonance rounds off with some even
/// harmonics instead of ringing ever higher. Low cutoffs ease the resonance, as the hardware
/// does, so the bass doesn't boom.
#[derive(Clone, Copy, Default)]
pub struct Svf {
    ic1: f32,
    ic2: f32,
    g: f32,
    k: f32,
    d: f32,
}

impl Svf {
    /// `resonance` 0..1: from a gentle bump to a ringing peak.
    pub fn set(&mut self, hz: f32, resonance: f32, rate: f32) {
        let hz = hz.clamp(10.0, 0.45 * rate);
        let low = (1.0 - hz / SVF_LOW_Q_HZ).max(0.0);
        let q = (0.6 + 22.0 * resonance * resonance) / (1.0 + SVF_LOW_Q_EASE * low * low);
        self.g = (PI * hz / rate).tan();
        self.k = 1.0 / q.max(0.5);
        self.d = 1.0 / (1.0 + self.g * (self.g + self.k));
    }

    #[inline(always)]
    fn sat(x: f32) -> f32 {
        let t0 = SVF_ASYM.tanh();
        (fast_tanh(x * SVF_HEAT + SVF_ASYM) - t0) / (SVF_HEAT * (1.0 - t0 * t0))
    }

    /// One sample in; the low, band and high outputs.
    #[inline(always)]
    fn tick(&mut self, x: f32) -> (f32, f32, f32) {
        let s1 = Self::sat(self.ic1);
        let high = (x - (self.k + self.g) * s1 - self.ic2) * self.d;
        let v1 = self.g * high;
        let band = v1 + s1;
        let v2 = self.g * band;
        let low = v2 + self.ic2;
        self.ic1 = band + v1;
        self.ic2 = low + v2;
        (low, band, high)
    }

    #[inline(always)]
    pub fn low(&mut self, x: f32) -> f32 {
        self.tick(x).0
    }

    /// `mode` 0..1 sweeps lowpass (0) through bandpass (½) to highpass (1). The band peaks as
    /// high as the lowpass's resonance does, so the sweep keeps its level.
    #[inline(always)]
    pub fn morph(&mut self, x: f32, mode: f32) -> f32 {
        let (low, band, high) = self.tick(x);
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
        (self.g, self.k, self.d) = (other.g, other.k, other.d);
    }
}

/// The four one-pole stages shared by the ladder and the SH-101 filter, solved with
/// zero-delay feedback: each sample, every stage's tanh is replaced by its gain at the
/// stage's last input (the linearization that keeps a nonlinear ladder cheap), so the loop
/// can be solved exactly and the tuning and resonance hold up to the top.
#[derive(Clone, Copy, Default)]
struct Stages {
    s: [f32; 4],
    /// Each stage's last tanh gains: on its input and on its output (the ladder), or on
    /// their difference in `input` (the OTA).
    gain_in: [f32; 4],
    gain_out: [f32; 4],
}

impl Stages {
    fn reset(&mut self) {
        self.s = [0.0; 4];
        self.gain_in = [1.0; 4];
        self.gain_out = [1.0; 4];
    }

    /// One sample through four stages with feedback `k·fb_gain` from the last to the first,
    /// where stage i is y = s + g·(a·u − b·y) (a, b the tanh gains, their tanh scaled by
    /// `scale`: where it bends, in the voice's signal units); returns the output.
    /// `ota` saturates on each stage's input-output difference (an OTA, the SH-101's
    /// IR3109); otherwise on input and output separately (a transistor ladder).
    #[inline(always)]
    fn tick(&mut self, x: f32, g: f32, k: f32, ota: bool, scale: f32) -> f32 {
        let mut big = [0.0f32; 4];
        let mut small = [0.0f32; 4];
        for i in 0..4 {
            let inv = 1.0 / (1.0 + g * self.gain_out[i]);
            big[i] = g * self.gain_in[i] * inv;
            small[i] = self.s[i] * inv;
        }
        let gamma = big[0] * big[1] * big[2] * big[3];
        let sigma = ((small[0] * big[1] + small[1]) * big[2] + small[2]) * big[3] + small[3];
        let y4 = (gamma * x + sigma) / (1.0 + k * gamma);
        let mut u = x - k * y4;
        for i in 0..4 {
            let y = big[i] * u + small[i];
            let v = g * (self.gain_in[i] * u - self.gain_out[i] * y);
            self.s[i] = y + v;
            if ota {
                let r = tanh_ratio(scale * (u - y));
                (self.gain_in[i], self.gain_out[i]) = (r, r);
            } else {
                // A stage's input is the last one's output: only the first input is new.
                self.gain_in[i] = if i == 0 { tanh_ratio(scale * u) } else { self.gain_out[i - 1] };
                self.gain_out[i] = tanh_ratio(scale * y);
            }
            u = y;
        }
        y4
    }
}

/// Where the ladder's stages bend (tanh(1.6 x)): a voice peaks around 0.3–0.7, so loud
/// or resonant notes drive them. At 0.8 the ladder measured almost as before (only its
/// input stage saturated); at 1.6 a 110 Hz saw at 0.3 through 800 Hz, resonance 0.6, has a
/// nonlinear part 7 dB below the output, against 17 before, at the same level.
const LADDER_THERMAL: f32 = 1.6;
/// The SH-101's OTAs bend later (as ABDrum's): its diodes hold the level first, so
/// its self-oscillation stays near the cutoff.
const SH101_OTA: f32 = 0.5;

/// A 4-pole ladder lowpass with zero-delay feedback. Every stage saturates, on its input and
/// on its output, as a transistor ladder's do (the input stage alone used to): driven or
/// resonant, it thickens and compresses through the whole ladder, and it self-oscillates at
/// full resonance.
#[derive(Clone, Copy)]
pub struct Ladder {
    stages: Stages,
    g: f32,
    k: f32,
}

impl Default for Ladder {
    fn default() -> Self {
        let mut stages = Stages::default();
        stages.reset();
        Self { stages, g: 0.0, k: 0.0 }
    }
}

impl Ladder {
    pub fn set(&mut self, hz: f32, resonance: f32, rate: f32) {
        self.g = (PI * hz.clamp(10.0, 0.45 * rate) / rate).tan();
        self.k = 4.2 * resonance;
    }

    #[inline(always)]
    pub fn low(&mut self, x: f32) -> f32 {
        // The bass a ladder loses to its own resonance, partly given back.
        self.stages.tick(x * (1.0 + 0.35 * self.k), self.g, self.k, false, LADDER_THERMAL)
    }

    pub fn clear(&mut self) {
        self.stages.reset();
    }

    /// Takes another filter's settings, keeping this one's state.
    pub fn follow(&mut self, other: &Ladder) {
        (self.g, self.k) = (other.g, other.k);
    }
}

/// After the SH-101's VCF (as ABDrum's pad filter): four OTA stages (the IR3109), 24 dB/oct,
/// each saturating on its input-output difference; resonance fed back, inverted, through a
/// soft limiter (the 101's clipping diodes, which start conducting early), and half of it
/// added back at the output (the 101 feeds its phase splitter's other side to the VCA), so
/// resonance costs less bass than in a plain ladder. It self-oscillates near the top, as
/// the 101 does, the diodes holding the sine's level.
#[derive(Clone, Copy)]
pub struct Sh101 {
    stages: Stages,
    g: f32,
    k: f32,
    /// The diodes' gain at the last output.
    diode: f32,
}

/// The 101's diodes' knee (in the voice's units: its self-oscillation then sits near a
/// loud voice's level, about 0.37 peak), and how much of the feedback comes back at the
/// output.
const SH101_DIODE: f32 = 0.2;
const SH101_OUT_MIX: f32 = 0.5;

impl Default for Sh101 {
    fn default() -> Self {
        let mut stages = Stages::default();
        stages.reset();
        Self { stages, g: 0.0, k: 0.0, diode: 1.0 }
    }
}

impl Sh101 {
    pub fn set(&mut self, hz: f32, resonance: f32, rate: f32) {
        self.g = (PI * hz.clamp(10.0, 0.45 * rate) / rate).tan();
        self.k = 4.4 * resonance;
    }

    #[inline(always)]
    pub fn low(&mut self, x: f32) -> f32 {
        let fb = self.k * self.diode;
        let y = self.stages.tick(x, self.g, fb, true, SH101_OTA);
        self.diode = tanh_ratio(y / SH101_DIODE);
        y * (1.0 + SH101_OUT_MIX * fb)
    }

    pub fn clear(&mut self) {
        self.stages.reset();
        self.diode = 1.0;
    }

    /// Takes another filter's settings, keeping this one's state.
    pub fn follow(&mut self, other: &Sh101) {
        (self.g, self.k) = (other.g, other.k);
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
        // At a voice's level (its oscillators peak around 0.3), where the loop's tanh is
        // still gentle.
        let mut at = |mode: f32, hz: f32| {
            svf.clear();
            gain_at(|x| 4.0 * svf.morph(0.25 * x, mode), hz, rate)
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
    fn the_sh101_filters_24_db_keeps_its_bass_and_sings_at_the_cutoff() {
        let rate = 96_000.0;
        let mut f = Sh101::default();
        f.set(1_000.0, 0.0, rate);
        assert!(gain_at(|x| 4.0 * f.low(0.25 * x), 100.0, rate) > 0.9);
        f.clear();
        let (a, b) = (gain_at(|x| 4.0 * f.low(0.25 * x), 2_000.0, rate), { f.clear(); gain_at(|x| 4.0 * f.low(0.25 * x), 4_000.0, rate) });
        let slope = 20.0 * (a / b).log10();
        assert!(slope > 19.0 && slope < 25.0, "{slope} dB/oct");
        // Full resonance: less bass lost than the ladder's, then it oscillates on its own.
        f.set(1_000.0, 1.0, rate);
        f.clear();
        let mut ladder = Ladder::default();
        ladder.set(1_000.0, 1.0, rate);
        let bass = gain_at(|x| 10.0 * f.low(0.1 * x), 50.0, rate);
        let ladder_bass = gain_at(|x| 10.0 * ladder.low(0.1 * x), 50.0, rate);
        assert!(bass > 1.2 * ladder_bass, "{bass} vs {ladder_bass}");
        f.clear();
        let y: Vec<f32> = (0..96_000).map(|i| f.low(if i == 0 { 0.3 } else { 0.0 })).collect();
        let tail = &y[48_000..];
        let peak = tail.iter().fold(0f32, |m, v| m.max(v.abs()));
        let hz = tail.windows(2).filter(|w| w[0] <= 0.0 && w[1] > 0.0).count() as f32 * 2.0;
        assert!(peak > 0.2 && peak < 0.6, "self-oscillation level {peak}");
        assert!((hz / 1_000.0 - 1.0).abs() < 0.02, "sings at {hz} Hz");
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

/// Measurements of the filters against the versions before Nov 2026 (printed):
/// cargo test --release -- --ignored --nocapture filter_report
#[cfg(test)]
mod report {
    use super::*;
    const RATE: f32 = 96_000.0;

/// A state-variable filter, stable under modulation (Zavalishin's TPT form).
#[derive(Clone, Copy, Default)]
pub struct OldSvf {
    ic1: f32,
    ic2: f32,
    g: f32,
    k: f32,
    a1: f32,
    a2: f32,
}

impl OldSvf {
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
    pub fn follow(&mut self, other: &OldSvf) {
        (self.g, self.k, self.a1, self.a2) = (other.g, other.k, other.a1, other.a2);
    }
}

/// A 4-pole ladder lowpass with zero-delay feedback and a saturating input, so it
/// self-oscillates at full resonance and growls when pushed.
#[derive(Clone, Copy, Default)]
pub struct OldLadder {
    s: [f32; 4],
    big_g: f32,
    k: f32,
}

impl OldLadder {
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
    pub fn follow(&mut self, other: &OldLadder) {
        (self.big_g, self.k) = (other.big_g, other.k);
    }
}


    /// Runs `f` on a sine of `amp` at `hz` for 0.5 s; the levels (dB re the input) of
    /// harmonics 1..=3 over the last half, by correlation.
    fn harmonics(mut f: impl FnMut(f32) -> f32, hz: f32, amp: f32) -> [f32; 3] {
        let n = (RATE * 0.5) as usize;
        let y: Vec<f32> = (0..n).map(|i| f(amp * (2.0 * PI * hz * i as f32 / RATE).sin())).collect();
        let tail = &y[n / 2..];
        std::array::from_fn(|h| {
            let w = 2.0 * PI * hz * (h + 1) as f32 / RATE;
            let (mut c, mut s) = (0.0f64, 0.0f64);
            for (i, v) in tail.iter().enumerate() {
                let t = (i + n / 2) as f32 * w;
                c += (*v * t.cos()) as f64;
                s += (*v * t.sin()) as f64;
            }
            let a = 2.0 * (c * c + s * s).sqrt() / tail.len() as f64;
            20.0 * (a as f32 / amp).max(1e-9).log10()
        })
    }

    /// The impulse response's ring frequency (zero crossings over 50 ms from 20 ms in)
    /// and its level 1 s on (dB re the kick), at resonance r.
    fn ring(mut f: impl FnMut(f32) -> f32) -> (f32, f32) {
        let y: Vec<f32> = (0..96_000 * 2).map(|i| f(if i == 0 { 0.3 } else { 0.0 })).collect();
        let w = &y[1920..1920 + 4800];
        let crossings = w.windows(2).filter(|p| p[0] <= 0.0 && p[1] > 0.0).count() as f32 / 0.05;
        let late = y[96_000..105_600].iter().fold(0f32, |m, v| m.max(v.abs()));
        (crossings, 20.0 * (late / 0.3).max(1e-9).log10())
    }

    #[test]
    #[ignore]
    fn filter_report() {
        macro_rules! each {
            ($hz:expr, $r:expr, |$f:ident| $body:expr) => {{
                let mut a = OldSvf::default(); a.set($hz, $r, RATE);
                let mut b = Svf::default(); b.set($hz, $r, RATE);
                let mut c = OldLadder::default(); c.set($hz, $r, RATE);
                let mut d = Ladder::default(); d.set($hz, $r, RATE);
                let mut e = Sh101::default(); e.set($hz, $r, RATE);
                [("old SVF", { let $f = |x| a.low(x); $body }), ("SVF", { let $f = |x| b.low(x); $body }),
                 ("old ladder", { let $f = |x| c.low(x); $body }), ("ladder", { let $f = |x| d.low(x); $body }),
                 ("SH-101", { let $f = |x| e.low(x); $body })]
            }};
        }
        println!("Passband (50 Hz, cutoff 1 kHz) at resonance 0 / 0.5 / 0.9, dB:");
        let rows: Vec<_> = [0.0, 0.5, 0.9].iter().map(|r| each!(1000.0, *r, |f| harmonics(f, 50.0, 0.1)[0])).collect();
        for k in 0..5 {
            println!("  {:10} {}", rows[0][k].0, rows.iter().map(|r| format!("{:6.1}", r[k].1)).collect::<Vec<_>>().join(" "));
        }
        println!("Resonant peak at the cutoff (quiet sine), resonance 0.7, cutoff 100 Hz / 1 kHz, dB:");
        let rows: Vec<_> = [100.0, 1000.0].iter().map(|hz| each!(*hz, 0.7, |f| harmonics(f, *hz, 0.01)[0])).collect();
        for k in 0..5 {
            println!("  {:10} {}", rows[0][k].0, rows.iter().map(|r| format!("{:6.1}", r[k].1)).collect::<Vec<_>>().join(" "));
        }
        println!("220 Hz sine, cutoff 2 kHz, resonance 0.5: H1 / H2 / H3 (dB) at amplitude 0.1, 0.3, 1.0:");
        for amp in [0.1, 0.3, 1.0] {
            for (name, h) in each!(2000.0, 0.5, |f| harmonics(f, 220.0, amp)) {
                println!("  {amp:.1} {name:10} {:6.1} {:6.1} {:6.1}", h[0], h[1], h[2]);
            }
        }
        // Nonlinearity on a voice-like signal: a 110 Hz saw at `amp`, cutoff 800 Hz, resonance
        // 0.6; the difference from the same filter fed 1/100 as much (scaled up), dB re output.
        println!("110 Hz saw, cutoff 800 Hz, res 0.6: nonlinear part (dB re output) at amplitude 0.3 / 0.6:");
        let saw = |i: usize| 2.0 * (110.0 * i as f32 / RATE).fract() - 1.0;
        let nl = |amp: f32, mut f: Box<dyn FnMut(f32) -> f32>, mut g: Box<dyn FnMut(f32) -> f32>| {
            let n = 48_000;
            let (mut d, mut o) = (0.0f64, 0.0f64);
            for i in 0..n {
                let y = f(amp * saw(i));
                let lin = g(0.01 * amp * saw(i)) * 100.0;
                if i > n / 2 { d += ((y - lin) * (y - lin)) as f64; o += (y * y) as f64; }
            }
            10.0 * (d / o).log10()
        };
        for amp in [0.3f32, 0.6] {
            let mk = |r: f32| { let mut a = OldLadder::default(); a.set(800.0, r, RATE); a };
            let (mut a, mut a2) = (mk(0.6), mk(0.6));
            let o_l = nl(amp, Box::new(move |x| a.low(x)), Box::new(move |x| a2.low(x)));
            let mk = |r: f32| { let mut a = Ladder::default(); a.set(800.0, r, RATE); a };
            let (mut a, mut a2) = (mk(0.6), mk(0.6));
            let n_l = nl(amp, Box::new(move |x| a.low(x)), Box::new(move |x| a2.low(x)));
            let mk = |r: f32| { let mut a = Sh101::default(); a.set(800.0, r, RATE); a };
            let (mut a, mut a2) = (mk(0.6), mk(0.6));
            let n_s = nl(amp, Box::new(move |x| a.low(x)), Box::new(move |x| a2.low(x)));
            let mk = |r: f32| { let mut a = OldSvf::default(); a.set(800.0, r, RATE); a };
            let (mut a, mut a2) = (mk(0.6), mk(0.6));
            let o_s = nl(amp, Box::new(move |x| a.low(x)), Box::new(move |x| a2.low(x)));
            let mk = |r: f32| { let mut a = Svf::default(); a.set(800.0, r, RATE); a };
            let (mut a, mut a2) = (mk(0.6), mk(0.6));
            let s_s = nl(amp, Box::new(move |x| a.low(x)), Box::new(move |x| a2.low(x)));
            println!("  {amp:.1}: old SVF {o_s:6.1}  SVF {s_s:6.1}  old ladder {o_l:6.1}  ladder {n_l:6.1}  SH-101 {n_s:6.1}");
        }
        println!("110 Hz saw at 0.3, cutoff 800 Hz: output RMS (dB) at res 0.3 / 0.6 / 0.9:");
        let saw = |i: usize| 2.0 * (110.0 * i as f32 / RATE).fract() - 1.0;
        let level = |mut f: Box<dyn FnMut(f32) -> f32>| {
            let n = 48_000;
            let mut o = 0.0f64;
            for i in 0..n { let y = f(0.3 * saw(i)); if i > n / 2 { o += (y * y) as f64; } }
            10.0 * (o / (n / 2) as f64).log10()
        };
        let rows: Vec<[f64; 5]> = [0.3f32, 0.6, 0.9].iter().map(|&r| {
            let mut a = OldSvf::default(); a.set(800.0, r, RATE);
            let mut b = Svf::default(); b.set(800.0, r, RATE);
            let mut c = OldLadder::default(); c.set(800.0, r, RATE);
            let mut d = Ladder::default(); d.set(800.0, r, RATE);
            let mut e = Sh101::default(); e.set(800.0, r, RATE);
            [level(Box::new(move |x| a.low(x))), level(Box::new(move |x| b.low(x))), level(Box::new(move |x| c.low(x))),
             level(Box::new(move |x| d.low(x))), level(Box::new(move |x| e.low(x)))]
        }).collect();
        for (k, name) in ["old SVF", "SVF", "old ladder", "ladder", "SH-101"].iter().enumerate() {
            println!("  {name:10} {}", rows.iter().map(|r| format!("{:6.1}", r[k])).collect::<Vec<_>>().join(" "));
        }
        println!("Kicked at resonance 0.95 / 1.0, cutoff 1 kHz: rings at (Hz), level 1 s on (dB):");
        for r in [0.95, 1.0] {
            for (name, (hz, late)) in each!(1000.0, r, |f| ring(f)) {
                println!("  {r:.2} {name:10} {hz:6.0} {late:7.1}");
            }
        }
    }
}
