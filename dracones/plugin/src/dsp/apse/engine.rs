//! The whole effect: pre-delay with early-reflection taps, the input's bandwidth
//! filter and diffusers, the tank, then Width, Color, Mix, Level and the limiter.

use std::f32::consts::FRAC_PI_2;

use super::choir::{Choir, ChoirControls};
use super::delay::{Allpass, Delay};
use super::filters::{one_pole_coefficient, OnePole, SvfShape, Tone};
use super::limiter::Limiter;
use super::modes::{largest_size, longest_diffuser, Mode, EARLY, MAX_DIFFUSERS, MODES, REFERENCE_HZ};
use super::tank::{Setting, Tank};

pub const TIME_MIN_S: f32 = 0.2;
pub const TIME_MAX_S: f32 = 30.0;
pub const SIZE_MIN: f32 = 0.25;
pub const SIZE_MAX: f32 = 2.0;
pub const PREDELAY_MAX_S: f32 = 0.5;
/// Mod at 100 % is this many times the mode's own depth; 50 % is the mode as voiced.
const MOD_RANGE: f32 = 2.0;
/// Damp from 0 to 1 sweeps the tank's lowpass between these.
const DAMP_OPEN_HZ: f32 = 20_000.0;
const DAMP_SHUT_HZ: f32 = 600.0;
/// Color from 0 to -1 brings a lowpass down to here; from 0 to 1, a highpass up to here.
const COLOR_DARKEST_HZ: f32 = 1_500.0;
const COLOR_BRIGHTEST_HZ: f32 = 1_000.0;
const COLOR_OPEN_LOW_HZ: f32 = 20.0;
const COLOR_OPEN_HIGH_HZ: f32 = 20_000.0;
/// Derived values are worked out once per this many samples.
const CONTROL_PERIOD: u32 = 16;
/// Changing mode fades the wet signal out, clears the memory and fades back in.
const MODE_FADE_S: f32 = 0.012;
/// Freeze shuts the input this quickly.
const GATE_S: f32 = 0.03;
/// Decay never quite reaches 1 unless frozen.
const MAX_DECAY: f32 = 0.99995;
/// Time is how long the tail takes to fall this far: about where it drops out of
/// hearing at a normal level, so the number matches what you hear. (Rated the usual
/// way, to -60 dB, the tail lasts half as long again.)
const TIME_DB: f32 = 40.0;

/// What the host's knobs say, sample by sample.
#[derive(Clone, Copy)]
pub struct Controls {
    pub time_s: f32,
    pub size: f32,
    pub predelay_s: f32,
    pub diffusion: f32,
    pub modulation: f32,
    pub damp: f32,
    pub color: f32,
    pub width: f32,
    pub mix: f32,
    pub level: f32,
    pub freeze: bool,
    pub choir: ChoirControls,
}

impl Default for Controls {
    fn default() -> Self {
        Self { time_s: 2.5, size: 1.0, predelay_s: 0.02, diffusion: 1.0, modulation: 0.5, damp: 0.35, color: 0.0, width: 1.0, mix: 0.35, level: 1.0, freeze: false, choir: ChoirControls::default() }
    }
}

/// Logarithmic travel between two frequencies.
fn sweep(from: f32, to: f32, amount: f32) -> f32 {
    from * (to / from).powf(amount)
}

/// Everything worked out at control rate.
struct Derived {
    tank: Setting,
    low_cut: SvfShape,
    high_cut: SvfShape,
}

pub struct Engine {
    sample_rate: f32,
    mode: usize,
    wanted_mode: usize,
    /// The wet signal's fade, for changing mode.
    fade: f32,
    fade_step: f32,
    gate: f32,
    gate_step: f32,
    predelay: Delay,
    bandwidth: OnePole,
    bandwidth_coefficient: f32,
    diffusers: Vec<Allpass>,
    tank: Tank,
    choir: Choir,
    tone: [Tone; 2],
    derived: Derived,
    countdown: u32,
    limiter: Limiter,
    wet_peak: f32,
}

impl Engine {
    pub fn new(sample_rate: f32) -> Self {
        let max_scale = largest_size() * SIZE_MAX;
        let max_wobble = MODES.iter().map(|m| m.wobble_ms).fold(0.0, f32::max) * MOD_RANGE * 1e-3 * sample_rate;
        let early_reach = EARLY.iter().map(|e| e.0).fold(0.0, f32::max) * 1e-3 * SIZE_MAX;
        let diffuser = longest_diffuser() * max_scale * sample_rate / REFERENCE_HZ + 4.0;
        let mut engine = Self {
            sample_rate,
            mode: 0,
            wanted_mode: 0,
            fade: 1.0,
            fade_step: 1.0 / (MODE_FADE_S * sample_rate),
            gate: 1.0,
            gate_step: 1.0 / (GATE_S * sample_rate),
            predelay: Delay::new((PREDELAY_MAX_S + early_reach) * sample_rate + 4.0),
            bandwidth: OnePole::default(),
            bandwidth_coefficient: 0.0,
            diffusers: (0..MAX_DIFFUSERS).map(|_| Allpass::new(diffuser)).collect(),
            tank: Tank::new(sample_rate, max_scale, max_wobble),
            choir: Choir::new(sample_rate),
            tone: Default::default(),
            derived: Derived { tank: Setting::default(), low_cut: SvfShape::new(COLOR_OPEN_LOW_HZ, sample_rate), high_cut: SvfShape::new(COLOR_OPEN_HIGH_HZ, sample_rate) },
            countdown: 0,
            limiter: Limiter::new(sample_rate),
            wet_peak: 0.0,
        };
        engine.apply_mode();
        engine
    }

    fn current(&self) -> &'static Mode {
        &MODES[self.mode]
    }

    /// Asks for a mode; it arrives after a short fade.
    pub fn set_mode(&mut self, mode: usize) {
        self.wanted_mode = mode.min(MODES.len() - 1);
    }

    fn apply_mode(&mut self) {
        self.mode = self.wanted_mode;
        let mode = self.current();
        self.tank.configure(mode);
        self.tank.clear();
        self.choir.clear();
        self.diffusers.iter_mut().for_each(Allpass::clear);
        self.predelay.clear();
        self.bandwidth.clear();
        self.tone.iter_mut().for_each(Tone::clear);
        self.bandwidth_coefficient = one_pole_coefficient(mode.bandwidth_hz, self.sample_rate);
        self.countdown = 0;
    }

    fn derive(&mut self, c: &Controls) {
        let mode = self.current();
        let decay = if c.freeze {
            1.0
        } else {
            // Four multiplications by Decay per trip round the loop.
            let trips = c.time_s / self.tank.loop_seconds(c.size);
            10f32.powf(-TIME_DB / 20.0 / (4.0 * trips)).min(MAX_DECAY)
        };
        let damp = if c.freeze { 0.0 } else { one_pole_coefficient(sweep(DAMP_OPEN_HZ, DAMP_SHUT_HZ, c.damp), self.sample_rate) };
        let wobble = mode.wobble_ms * 1e-3 * self.sample_rate * c.modulation * MOD_RANGE;
        let low_cut_hz = mode.low_cut_hz.max(sweep(COLOR_OPEN_LOW_HZ, COLOR_BRIGHTEST_HZ, c.color.max(0.0)));
        let high_cut_hz = sweep(COLOR_OPEN_HIGH_HZ, COLOR_DARKEST_HZ, (-c.color).max(0.0));
        if mode.choir {
            self.choir.derive(&c.choir, CONTROL_PERIOD);
        }
        self.derived = Derived {
            tank: Setting { decay, damp, wobble },
            low_cut: SvfShape::new(low_cut_hz, self.sample_rate),
            high_cut: SvfShape::new(high_cut_hz, self.sample_rate),
        };
    }

    /// Moves `value` one step toward `target`.
    #[inline]
    fn approach(value: &mut f32, target: f32, step: f32) {
        *value = if *value < target { (*value + step).min(target) } else { (*value - step).max(target) };
    }

    #[inline]
    fn early(&self, predelay: f32, scale: f32, spread: f32) -> [f32; 2] {
        let mut out = [0.0; 2];
        for (ms, gain, pan) in EARLY {
            let x = gain * self.predelay.tap(predelay + ms * 1e-3 * spread * scale * self.sample_rate);
            out[0] += x * 0.5 * (1.0 - pan);
            out[1] += x * 0.5 * (1.0 + pan);
        }
        out
    }

    #[inline]
    pub fn process(&mut self, input: [f32; 2], c: &Controls) -> [f32; 2] {
        // A mode change waits for the wet signal to fade out.
        if self.wanted_mode != self.mode {
            Self::approach(&mut self.fade, 0.0, self.fade_step);
            if self.fade == 0.0 {
                self.apply_mode();
            }
        } else {
            Self::approach(&mut self.fade, 1.0, self.fade_step);
        }
        if self.countdown == 0 {
            self.derive(c);
            self.countdown = CONTROL_PERIOD;
        }
        self.countdown -= 1;
        Self::approach(&mut self.gate, if c.freeze { 0.0 } else { 1.0 }, self.gate_step);

        let mode = self.current();
        let scale = c.size;
        let predelay = c.predelay_s * self.sample_rate;
        let x = 0.5 * (input[0] + input[1]) * self.gate;
        let delayed = self.predelay.tap(predelay);
        let early = if mode.early > 0.0 { self.early(predelay, scale, mode.early_spread) } else { [0.0; 2] };
        self.predelay.write(x);

        let mut x = self.bandwidth.process(delayed, self.bandwidth_coefficient);
        let k = mode.size * scale * self.sample_rate / REFERENCE_HZ;
        for (diffuser, (length, gain)) in self.diffusers.iter_mut().zip(mode.diffusers) {
            x = diffuser.process(x, length * k, gain * c.diffusion);
        }
        let late = self.tank.process(x, scale, &self.derived.tank);
        let late = if mode.choir { self.choir.process(late) } else { late };

        let wet: [f32; 2] = std::array::from_fn(|i| (mode.late * late[i] + mode.early * early[i]) * self.fade);
        let mid = 0.5 * (wet[0] + wet[1]);
        let side = 0.5 * (wet[0] - wet[1]) * c.width;
        let wet = [mid + side, mid - side];
        let wet: [f32; 2] = std::array::from_fn(|i| self.tone[i].process(wet[i], &self.derived.low_cut, &self.derived.high_cut));
        self.wet_peak = self.wet_peak.max(wet[0].abs()).max(wet[1].abs());

        let (dry_gain, wet_gain) = ((c.mix * FRAC_PI_2).cos(), (c.mix * FRAC_PI_2).sin());
        let out = std::array::from_fn(|i| (dry_gain * input[i] + wet_gain * wet[i]) * c.level);
        self.limiter.process(out)
    }

    pub fn reduction_db(&self) -> f32 {
        self.limiter.reduction_db()
    }

    /// The loudest wet sample since the last call.
    pub fn take_wet_peak(&mut self) -> f32 {
        std::mem::take(&mut self.wet_peak)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::modes::MODE_COUNT;

    const SR: f32 = 48_000.0;

    fn wet(time_s: f32) -> Controls {
        Controls { mix: 1.0, time_s, predelay_s: 0.0, ..Default::default() }
    }

    fn settle(engine: &mut Engine, mode: usize, c: &Controls) {
        engine.set_mode(mode);
        for _ in 0..(SR * 0.05) as usize {
            engine.process([0.0; 2], c);
        }
    }

    /// RMS of each 50 ms window after a short noise burst.
    fn envelope(engine: &mut Engine, c: &Controls, seconds: f32, burst: bool) -> Vec<f32> {
        let mut seed = 1u32;
        let window = (SR * 0.05) as usize;
        let mut sum = 0.0;
        let mut out = Vec::new();
        for n in 0..(seconds * SR) as usize {
            seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            let x = if burst && n < window { (seed >> 9) as f32 / (1 << 22) as f32 - 1.0 } else { 0.0 };
            let [l, r] = engine.process([x * 0.25, x * 0.25], c);
            sum += l * l + r * r;
            if (n + 1) % window == 0 {
                out.push((sum / (2 * window) as f32).sqrt());
                sum = 0.0;
            }
        }
        out
    }

    fn db(x: f32) -> f32 {
        20.0 * x.max(1e-12).log10()
    }

    #[test]
    fn every_mode_decays_at_about_the_time_asked() {
        for mode in 0..MODE_COUNT {
            let c = Controls { damp: 0.0, ..wet(2.0) };
            let mut engine = Engine::new(SR);
            settle(&mut engine, mode, &c);
            let env = envelope(&mut engine, &c, 3.0, true);
            // The slope from 0.5 s to 1.5 s, in dB per second, should be -20 for 2 s.
            let slope = (db(env[30]) - db(env[10])) / 1.0;
            assert!((-25.0..-15.0).contains(&slope), "{}: {slope:.1} dB/s", MODES[mode].name);
        }
    }

    #[test]
    fn freeze_holds_without_growing() {
        for mode in 0..MODE_COUNT {
            let mut c = wet(3.0);
            let mut engine = Engine::new(SR);
            settle(&mut engine, mode, &c);
            envelope(&mut engine, &c, 0.3, true);
            c.freeze = true;
            let env = envelope(&mut engine, &c, 12.0, false);
            let (early, late) = (db(env[20]), db(env[env.len() - 1]));
            assert!(late > early - 6.0 && late < early + 1.0, "{}: {early:.1} → {late:.1} dB", MODES[mode].name);
        }
    }

    #[test]
    fn changing_mode_clears_the_tail() {
        let c = wet(20.0);
        let mut engine = Engine::new(SR);
        envelope(&mut engine, &c, 0.5, true);
        settle(&mut engine, 2, &c);
        let after = envelope(&mut engine, &c, 0.2, false);
        assert!(after.iter().skip(2).all(|x| *x == 0.0));
    }

    #[test]
    fn dry_passes_untouched_at_zero_mix() {
        let c = Controls { mix: 0.0, ..Default::default() };
        let mut engine = Engine::new(SR);
        for n in 0..4_800 {
            let x = 0.3 * (n as f32 * 0.01).sin();
            let [l, r] = engine.process([x, -x], &c);
            assert!((l - x).abs() < 1e-6 && (r + x).abs() < 1e-6);
        }
    }
}
