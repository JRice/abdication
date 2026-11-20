//! The three insert delays. Each oscillator feeds exactly one of them (the plaques on
//! the panel are always a permutation), and each delay has an effect inside its loop:
//!
//! 1. Juno chorus (Juno-60 mode I, BBD-style): a triangle at 0.513 Hz sweeping a short
//!    delay between 1.66 and 5.35 ms, the right side's sweep inverted, so the tank turns
//!    stereo here. Custom is how much chorus goes into the loop (0: a plain delay).
//! 2. Tube: an asymmetric, triode-like curve whose ceiling sits near -6 dBFS. Drive
//!    raises what goes into the tank (0 to +24 dB, the output brought back down) and
//!    the loop's gain toward 1.03, so at full drive it sustains on its own, held only
//!    by the tube: out of control, but only just.
//! 3. Small Stone phaser: four OTA allpass stages swept together, Color off, full depth.
//!    Custom is its Speed, 0.05 to 8 Hz.
//!
//! In every loop: read → effect → a 40 Hz high-pass and a gentle 9 kHz low-pass →
//! × Feedback → write. Each delay's Mix (the small slider under its Division) sets
//! dry against wet: from off, the wet rises to full at the middle with the dry at unity;
//! past the middle the dry fades out, so at 100 % only the echoes are heard. The default,
//! 35 %, is the first version's fixed balance (dry at unity, wet at -3 dB). Each tank holds 30 s; Division is in beats of the host's tempo, and a change
//! of division or tempo crossfades two read heads over 120 ms (no pitch glide).

use std::f32::consts::PI;

use super::filters::{one_pole, OnePole};

/// The Division dial: 1/8 dotted, 1/4, 1 bar, 1 bar dotted, 2, 3 and 4 bars, in beats.
pub const DIVISIONS: [f32; 7] = [0.75, 1.0, 4.0, 6.0, 8.0, 12.0, 16.0];
pub const DIVISION_NAMES: [&str; 7] = ["1/8.", "1/4", "1 bar", "1 bar.", "2 bars", "3 bars", "4 bars"];
pub const EFFECTS: usize = 3;
pub const EFFECT_NAMES: [&str; EFFECTS] = ["Chorus", "Tube", "Phaser"];
pub const CHORUS: usize = 0;
pub const TUBE: usize = 1;
pub const PHASER: usize = 2;

const TANK_S: f32 = 30.0;
const HEAD_FADE_S: f32 = 0.12;
/// The Mix slider's default: dry at unity, wet at -3 dB.
pub const MIX_DEFAULT: f32 = 0.35;
/// Mix glides this fast, so dragging it doesn't zip.
const MIX_HZ: f32 = 20.0;
const LOOP_HP_HZ: f32 = 40.0;
const LOOP_LP_HZ: f32 = 9_000.0;
pub const MAX_FEEDBACK: f32 = 0.98;
/// The tube's loop gain at full drive.
const TUBE_RUNAWAY: f32 = 1.03;
const TUBE_DRIVE_DB: f32 = 24.0;

const CHORUS_HZ: f32 = 0.513;
const CHORUS_MIN_MS: f32 = 1.66;
const CHORUS_MAX_MS: f32 = 5.35;
const PHASER_LOW_HZ: f32 = 120.0;
const PHASER_HIGH_HZ: f32 = 3_600.0;

/// The phaser's Speed knob (0..1): 0.05 to 8 Hz.
pub fn phaser_hz(knob: f32) -> f32 {
    0.05 * 160f32.powf(knob.clamp(0.0, 1.0))
}

/// Each effect's Division, with no two on the same value: the effects are settled in
/// row order, top first, and one that lands on a taken value moves to the next free
/// longer one (or, past 4 bars, the next free shorter). `rows[e]` is effect e's row.
pub fn resolve(wanted: [usize; EFFECTS], rows: [usize; EFFECTS]) -> [usize; EFFECTS] {
    let mut order = [0, 1, 2];
    order.sort_by_key(|&e| rows[e]);
    let mut taken = [false; DIVISIONS.len()];
    let mut out = [0; EFFECTS];
    for e in order {
        let w = wanted[e].min(DIVISIONS.len() - 1);
        let free = (w..DIVISIONS.len()).chain((0..w).rev()).find(|&d| !taken[d]).unwrap_or(w);
        taken[free] = true;
        out[e] = free;
    }
    out
}

/// The permutations of three rows, in "Insert Order" order: entry p says which effect
/// sits on rows I, II and III.
pub const ORDERS: [[usize; 3]; 6] = [[0, 1, 2], [0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]];

/// Which row each effect is on, for order `p`.
pub fn rows_of(p: usize) -> [usize; EFFECTS] {
    let order = ORDERS[p.min(5)];
    let mut rows = [0; EFFECTS];
    for (row, &e) in order.iter().enumerate() {
        rows[e] = row;
    }
    rows
}

/// The order index for effects on `rows`.
pub fn order_of(rows: [usize; EFFECTS]) -> usize {
    ORDERS.iter().position(|o| (0..3).all(|r| rows[o[r]] == r)).unwrap_or(0)
}

/// The Mix slider (0..1) as (dry, wet) gains.
pub fn mix_gains(mix: f32) -> (f32, f32) {
    let m = mix.clamp(0.0, 1.0);
    ((2.0 * (1.0 - m)).min(1.0), (2.0 * m).min(1.0))
}

/// What one tank is told each block.
#[derive(Clone, Copy, Debug)]
pub struct Setting {
    /// The delay, in samples.
    pub delay: f32,
    /// 0..1, clamped to MAX_FEEDBACK after modulation.
    pub feedback: f32,
    /// The effect's own knob: chorus amount, tube drive, phaser speed (0..1).
    pub custom: f32,
    /// Dry against wet (0..1), see `mix_gains`.
    pub mix: f32,
}

/// A short modulated line, for the chorus.
struct Bbd {
    line: Vec<[f32; 2]>,
    write: usize,
}

impl Bbd {
    fn new(rate: f32) -> Self {
        Self { line: vec![[0.0; 2]; (CHORUS_MAX_MS * 1e-3 * rate) as usize + 8], write: 0 }
    }

    #[inline(always)]
    fn read(&self, delay: f32, ch: usize) -> f32 {
        let n = self.line.len();
        let at = self.write as f32 + n as f32 - delay;
        let i = at as usize;
        let f = at - i as f32;
        let (a, b) = (self.line[i % n][ch], self.line[(i + 1) % n][ch]);
        a + (b - a) * f
    }
}

/// One first-order allpass per stage and side.
#[derive(Clone, Copy, Default)]
struct Allpass {
    x1: f32,
    y1: f32,
}

impl Allpass {
    #[inline(always)]
    fn tick(&mut self, x: f32, a: f32) -> f32 {
        let y = a * x + self.x1 - a * self.y1;
        self.x1 = x;
        self.y1 = y;
        y
    }
}

/// The tube: a rounded-knee curve (p-norm, p = 8), unity at small signals, flattening
/// at 0.5 above and a little further below (as a triode's grid conducts first).
#[inline(always)]
pub fn tube(x: f32) -> f32 {
    let lim = if x >= 0.0 { 0.5 } else { 0.62 };
    let u = x / lim;
    let u2 = u * u;
    let u8 = (u2 * u2) * (u2 * u2);
    lim * u / (1.0 + u8).powf(0.125)
}

pub struct Tank {
    rate: f32,
    effect: usize,
    buf: Vec<[f32; 2]>,
    write: usize,
    delay: f32,
    old_delay: f32,
    /// 1 when the heads have finished crossfading.
    fade: f32,
    fade_step: f32,
    hp: [OnePole; 2],
    lp: [OnePole; 2],
    hp_a: f32,
    lp_a: f32,
    bbd: Bbd,
    lfo: f32,
    stages: [[Allpass; 4]; 2],
    coefficient: f32,
    countdown: u32,
    mix: f32,
    mix_a: f32,
}

impl Tank {
    pub fn new(effect: usize, rate: f32) -> Self {
        Self {
            rate,
            effect,
            buf: vec![[0.0; 2]; (TANK_S * rate) as usize + 4],
            write: 0,
            delay: 0.0,
            old_delay: 0.0,
            fade: 1.0,
            fade_step: 1.0 / (HEAD_FADE_S * rate),
            hp: Default::default(),
            lp: Default::default(),
            hp_a: one_pole(LOOP_HP_HZ, rate),
            lp_a: one_pole(LOOP_LP_HZ, rate),
            bbd: Bbd::new(rate),
            lfo: 0.0,
            stages: Default::default(),
            coefficient: 0.0,
            countdown: 0,
            mix: -1.0,
            mix_a: 1.0 - (-std::f32::consts::TAU * MIX_HZ / rate).exp(),
        }
    }

    /// The longest delay a tank holds, in samples.
    pub fn longest(&self) -> f32 {
        (self.buf.len() - 4) as f32
    }

    #[inline(always)]
    fn read(&self, delay: f32) -> [f32; 2] {
        let n = self.buf.len();
        let at = self.write as f32 + n as f32 - delay.clamp(1.0, self.longest());
        let i = at as usize;
        let f = at - i as f32;
        let (a, b) = (self.buf[i % n], self.buf[(i + 1) % n]);
        [a[0] + (b[0] - a[0]) * f, a[1] + (b[1] - a[1]) * f]
    }

    /// Point the read head at `delay` samples, crossfading from where it was.
    fn head_to(&mut self, delay: f32) {
        if self.delay == 0.0 {
            self.delay = delay;
        } else if (delay - self.delay).abs() > 0.5 {
            // A change mid-fade starts from the head that's loudest now.
            self.old_delay = if self.fade < 0.5 { self.old_delay } else { self.delay };
            self.delay = delay;
            self.fade = 0.0;
        }
    }

    /// One sample of the effect, inside the loop.
    #[inline]
    fn effect(&mut self, r: [f32; 2], s: &Setting) -> [f32; 2] {
        match self.effect {
            CHORUS => {
                let n = self.bbd.line.len();
                self.bbd.line[self.bbd.write] = r;
                self.bbd.write = (self.bbd.write + 1) % n;
                self.lfo = (self.lfo + CHORUS_HZ / self.rate).fract();
                let tri = 1.0 - 2.0 * (self.lfo - 0.5).abs();
                let ms = |t: f32| (CHORUS_MIN_MS + (CHORUS_MAX_MS - CHORUS_MIN_MS) * t) * 1e-3 * self.rate;
                let ch = [self.bbd.read(ms(tri), 0), self.bbd.read(ms(1.0 - tri), 1)];
                // L and R are swept opposite ways: the tank is stereo from here.
                let wet = ch;
                let c = s.custom.clamp(0.0, 1.0);
                let norm = 1.0 / (1.0 + 0.5 * c);
                [(r[0] + c * wet[0]) * norm, (r[1] + c * wet[1]) * norm]
            }
            TUBE => r.map(tube),
            _ => {
                if self.countdown == 0 {
                    // The sweep is exponential in frequency, a triangle in time.
                    let tri = 1.0 - 2.0 * (self.lfo - 0.5).abs();
                    let hz = PHASER_LOW_HZ * (PHASER_HIGH_HZ / PHASER_LOW_HZ).powf(tri);
                    let t = (PI * hz / self.rate).tan();
                    self.coefficient = (t - 1.0) / (t + 1.0);
                    self.countdown = 8;
                }
                self.countdown -= 1;
                self.lfo = (self.lfo + phaser_hz(s.custom) / self.rate).fract();
                let a = self.coefficient;
                let mut out = [0.0; 2];
                for (side, stages) in self.stages.iter_mut().enumerate() {
                    let mut y = r[side];
                    for st in stages.iter_mut() {
                        y = st.tick(y, a);
                    }
                    out[side] = 0.5 * (r[side] + y);
                }
                out
            }
        }
    }

    /// One sample in (mono, dry) → the tank's output (dry + wet), and the wet alone.
    #[inline]
    pub fn tick(&mut self, input: f32, s: &Setting) -> ([f32; 2], [f32; 2]) {
        let r = if self.fade < 1.0 {
            self.fade = (self.fade + self.fade_step).min(1.0);
            let (a, b) = (self.read(self.old_delay), self.read(self.delay));
            // Equal power: the two heads read unrelated stretches.
            let (g_old, g_new) = ((self.fade * PI * 0.5).cos(), (self.fade * PI * 0.5).sin());
            [a[0] * g_old + b[0] * g_new, a[1] * g_old + b[1] * g_new]
        } else {
            self.read(self.delay)
        };
        let y = self.effect(r, s);
        let y: [f32; 2] = std::array::from_fn(|c| {
            let hp = self.hp[c].high(y[c], self.hp_a);
            self.lp[c].low(hp, self.lp_a)
        });
        let (gain_in, loop_gain) = if self.effect == TUBE {
            let d = s.custom.clamp(0.0, 1.0);
            let fb = s.feedback.clamp(0.0, MAX_FEEDBACK);
            (10f32.powf(TUBE_DRIVE_DB * d / 20.0), fb + (TUBE_RUNAWAY - fb) * d * d * d)
        } else {
            (1.0, s.feedback.clamp(0.0, MAX_FEEDBACK))
        };
        let into = input * gain_in;
        self.buf[self.write] = [into + loop_gain * y[0], into + loop_gain * y[1]];
        self.write = (self.write + 1) % self.buf.len();
        if self.mix < 0.0 {
            self.mix = s.mix;
        }
        self.mix += (s.mix - self.mix) * self.mix_a;
        let (dry, wet_mix) = mix_gains(self.mix);
        // The drive's level brought back down (partly: a driven tube is meant to be louder).
        let wet_gain = wet_mix / gain_in.sqrt();
        let wet = [y[0] * wet_gain, y[1] * wet_gain];
        ([dry * input + wet[0], dry * input + wet[1]], wet)
    }

    /// Set the read head for this block.
    pub fn prepare(&mut self, s: &Setting) {
        self.head_to(s.delay);
    }

    pub fn effect_index(&self) -> usize {
        self.effect
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::TAU;

    #[test]
    fn divisions_never_share() {
        assert_eq!(resolve([1, 3, 5], [0, 1, 2]), [1, 3, 5]);
        // Two on 1 bar: the lower row moves to the next longer free one.
        assert_eq!(resolve([2, 2, 5], [0, 1, 2]), [2, 3, 5]);
        // All on 4 bars: the lower rows step back down.
        assert_eq!(resolve([6, 6, 6], [2, 0, 1]), [4, 6, 5]);
        for p in 0..6 {
            assert_eq!(order_of(rows_of(p)), p);
        }
    }

    fn run(effect: usize, s: Setting, secs: f32) -> f32 {
        let rate = 48_000.0;
        let mut t = Tank::new(effect, rate);
        t.prepare(&s);
        let mut peak = 0.0f32;
        for i in 0..(secs * rate) as usize {
            let x = if i < 4800 { 0.5 * (TAU * 220.0 * i as f32 / rate).sin() } else { 0.0 };
            let (_, wet) = t.tick(x, &s);
            if i as f32 > (secs - 1.0) * rate {
                peak = peak.max(wet[0].abs()).max(wet[1].abs());
            }
        }
        peak
    }

    #[test]
    fn loops_decay_or_hold() {
        let s = |fb, custom| Setting { delay: 12_000.0, feedback: fb, custom, mix: MIX_DEFAULT };
        // Low feedback dies away; the chorus and phaser stay bounded at the top.
        assert!(run(CHORUS, s(0.3, 0.5), 8.0) < 1e-3);
        assert!(run(PHASER, s(0.98, 0.5), 8.0) < 1.0);
        // A fully driven tube sustains on its own, held near -6 dBFS (and below it after
        // the output's level correction).
        let tube = run(TUBE, s(0.2, 1.0), 12.0);
        assert!(tube > 0.02 && tube < 0.5, "{tube}");
        assert!(run(TUBE, s(0.2, 0.0), 8.0) < 1e-3);
    }

    #[test]
    fn mix_runs_from_dry_to_wet() {
        assert_eq!(mix_gains(0.0), (1.0, 0.0));
        assert_eq!(mix_gains(0.5), (1.0, 1.0));
        assert_eq!(mix_gains(1.0), (0.0, 1.0));
        // The default is the first version's balance: dry at unity, wet at -3 dB.
        let (dry, wet) = mix_gains(MIX_DEFAULT);
        assert!(dry == 1.0 && (20.0 * wet.log10() + 3.1).abs() < 0.1);
        let mut t = Tank::new(CHORUS, 48_000.0);
        let off = Setting { delay: 4_800.0, feedback: 0.5, custom: 0.5, mix: 0.0 };
        t.prepare(&off);
        for i in 0..48_000 {
            let (out, wet) = t.tick(if i < 100 { 1.0 } else { 0.0 }, &off);
            assert_eq!(wet, [0.0, 0.0]);
            if i >= 100 {
                assert_eq!(out, [0.0, 0.0]);
            }
        }
    }

    #[test]
    fn the_tube_is_unity_small_and_ceilinged_large() {
        assert!((tube(0.01) - 0.01).abs() < 1e-4);
        assert!(tube(10.0) < 0.51 && tube(-10.0) > -0.63);
    }
}
