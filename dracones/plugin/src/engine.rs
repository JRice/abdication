//! The whole instrument, free of any plugin API. Free-running: no notes, it sounds
//! whenever it's loaded, its pitches set only by the knobs.
//!
//! ```text
//! osc I / II / III → lowpass (Wasp | SVF) → × Gain → insert delay (one each) → sum
//! sum → Fuzz → Spectral Morph → Apse's Plate → × Output → safety limiter → out
//! ```
//!
//! Everything slow (the modulators, the knobs they move, the filters' targets) is worked
//! out once per CONTROL samples; the audio runs per sample between.

use std::f32::consts::FRAC_PI_2;
use std::sync::Arc;

use crate::dsp::apse::engine::{Controls as ReverbControls, Engine as Reverb};
use crate::dsp::apse::limiter::Limiter;
use crate::dsp::filters::{self, Character, Lowpass};
use crate::dsp::fold::Folder;
use crate::dsp::fuzz::Fuzz;
use crate::dsp::harmonic::{HarmonicEq, HarmonicSettings};
use crate::dsp::inserts::{resolve, rows_of, Setting, Tank, DIVISIONS, EFFECTS, MIX_DEFAULT, ORDERS};
use crate::dsp::mods::{Modulators, Transport, Values, COPIES};
use crate::dsp::osc::{Osc, Reading};
use crate::dsp::tables::Bank;
use crate::dsp::{argon, pitch};
use crate::knobs::{self, ARMED_COUNT};
use crate::params::{osc_gain, output_gain, reverb_time, slant_db};

pub const CONTROL: usize = 32;
/// The oscillators' centre pitch: C2.
pub const CENTRE_HZ: f32 = 65.406;
/// Argent's default SEED: the drawn Fizmo tables are the ones it starts with.
const FIZMO_SEED: u32 = 417;
const GLIDE: Reading = Reading { stepped: false, coarse: false };
/// A swapped plaque's inputs crossfade over this long.
const ROUTE_S: f32 = 0.05;
/// The reverb's share at full (equal-power, as Apse's Mix).
const REVERB_MIX: f32 = 0.3;
/// The Fizmo tables come out hot; brought to about the others' level.
const FIZMO_LEVEL: f32 = 0.7;
/// Points in each seal's scope: one cycle of its oscillator, after the filter, each
/// point the mean of its slice of the cycle (a bright wave sampled at single points
/// would alias into noise).
pub const SCOPE: usize = 64;

/// Everything the engine reads from the controls.
#[derive(Clone, Copy, Debug)]
pub struct Controls {
    pub knobs: [f32; ARMED_COUNT],
    pub depths: [[f32; 4]; ARMED_COUNT],
    pub filters: [Character; 3],
    /// Oscillator I's Fizmo table (0..34), II's Argon table (0..6).
    pub tables: [usize; 2],
    /// Each effect's Division, as set (before they're kept apart).
    pub divisions: [usize; EFFECTS],
    pub order: usize,
    pub morph_shape: usize,
    pub morph_depth: f32,
    /// Each delay's dry/wet Mix (per effect).
    pub mixes: [f32; EFFECTS],
    /// Red's and slate's three copies.
    pub red_time: [f32; COPIES],
    pub red_slope: [f32; COPIES],
    pub slate_rate: [usize; COPIES],
    pub output: f32,
}

impl Default for Controls {
    fn default() -> Self {
        Self {
            knobs: knobs::ARMED.map(|k| k.default),
            depths: [[0.0; 4]; ARMED_COUNT],
            filters: [Character::Wasp, Character::Svf, Character::Wasp],
            tables: [3, 1],
            divisions: [1, 3, 5],
            order: 0,
            morph_shape: 0,
            morph_depth: 1.0,
            mixes: [MIX_DEFAULT; EFFECTS],
            red_time: RED_TIMES,
            red_slope: RED_SLOPES,
            slate_rate: SLATE_HOLDS,
            output: 0.8,
        }
    }
}

/// The copies' defaults: set a little apart, so they don't start out the same.
pub const RED_TIMES: [f32; COPIES] = [0.55, 0.47, 0.63];
pub const RED_SLOPES: [f32; COPIES] = [0.3, 0.55, 0.75];
pub const SLATE_HOLDS: [usize; COPIES] = [0, 1, 2];

/// What the panel is told after each block.
#[derive(Clone, Copy, Debug)]
pub struct Report {
    pub mods: Values,
    pub moved: [f32; ARMED_COUNT],
    pub divisions: [usize; EFFECTS],
    pub peak: f32,
    /// The last whole cycle of each oscillator, after its filter (before its Gain).
    pub scopes: [[f32; SCOPE]; 3],
}

impl Default for Report {
    fn default() -> Self {
        Self { mods: Values::default(), moved: [0.0; ARMED_COUNT], divisions: [0; EFFECTS], peak: 0.0, scopes: [[0.0; SCOPE]; 3] }
    }
}

pub struct Engine {
    rate: f32,
    bank: Arc<Bank>,
    fizmo: Osc,
    argon_phase: f32,
    folder: Folder,
    filters: [Lowpass; 3],
    gains: [f32; 3],
    tanks: [Tank; EFFECTS],
    /// How much of row r goes into effect e, gliding when the plaques are swapped.
    route: [[f32; EFFECTS]; 3],
    mods: Modulators,
    scope_phase: [f32; 3],
    /// Each slice's running sum and count, for the cycle being filled.
    scope_fill: [[(f32, u32); SCOPE]; 3],
    scopes: [[f32; SCOPE]; 3],
    fuzz: Fuzz,
    morph: [HarmonicEq; 2],
    reverb: Reverb,
    limiter: Limiter,
    out_gain: f32,
    pub report: Report,
}

impl Engine {
    pub fn new(rate: f32) -> Self {
        crate::dsp::harmlib::prepare();
        let mut reverb = Reverb::new(rate);
        reverb.set_mode(0);
        let route = std::array::from_fn(|r| std::array::from_fn(|e| (ORDERS[0][r] == e) as i32 as f32));
        Self {
            rate,
            bank: Bank::build(FIZMO_SEED),
            fizmo: Osc::default(),
            argon_phase: 0.0,
            folder: Folder::new(rate),
            filters: Default::default(),
            gains: [0.0; 3],
            tanks: std::array::from_fn(|e| Tank::new(e, rate)),
            route,
            mods: Modulators::new(rate),
            scope_phase: [0.0; 3],
            scope_fill: [[(0.0, 0); SCOPE]; 3],
            scopes: [[0.0; SCOPE]; 3],
            fuzz: Fuzz::new(rate),
            morph: Default::default(),
            reverb,
            limiter: Limiter::new(rate),
            out_gain: 0.0,
            report: Report::default(),
        }
    }

    /// Oscillator `o`'s pitch for its Pitch knob at `travel` (0..1).
    pub fn osc_hz(travel: f32) -> f32 {
        CENTRE_HZ * (pitch::semitones(2.0 * travel - 1.0) / 12.0).exp2()
    }

    /// Fill `left` and `right` (any length).
    pub fn process(&mut self, left: &mut [f32], right: &mut [f32], c: &Controls, transport: Transport) {
        let mut start = 0;
        let mut transport = transport;
        while start < left.len() {
            let n = (left.len() - start).min(CONTROL);
            self.block(&mut left[start..start + n], &mut right[start..start + n], c, transport);
            if let Some(b) = transport.beats.as_mut() {
                *b += n as f64 / self.rate as f64 * transport.tempo / 60.0;
            }
            start += n;
        }
    }

    fn block(&mut self, left: &mut [f32], right: &mut [f32], c: &Controls, transport: Transport) {
        let n = left.len();
        let rate = self.rate;
        let mods = self.mods.block(n, c.red_time, c.red_slope, c.slate_rate, transport);
        // Each knob's red and slate arms follow its row's copy.
        let rows = rows_of(c.order);
        let moved: [f32; ARMED_COUNT] =
            std::array::from_fn(|i| knobs::modulated(i, c.knobs[i], &c.depths[i], &mods.row(knobs::row(i, rows))));
        let at = |o: usize, p: usize| moved[knobs::osc(o, p)];

        // The oscillators, each through its filter, at its gain.
        let mut osc = [[0.0f32; CONTROL]; 3];
        let hz: [f32; 3] = std::array::from_fn(|o| Self::osc_hz(at(o, knobs::PITCH)));
        let table = c.tables[0].min(crate::dsp::tables::TABLES - 1) as f32;
        let position = at(0, knobs::CUSTOM);
        for x in osc[0][..n].iter_mut() {
            *x = FIZMO_LEVEL * self.fizmo.read(&self.bank, table, position, GLIDE);
            self.fizmo.advance(hz[0] / rate, position, GLIDE);
        }
        let (argon_table, argon_pos) = (c.tables[1], at(1, knobs::CUSTOM));
        for x in osc[1][..n].iter_mut() {
            *x = argon::read(argon_table, argon_pos, hz[1], rate, self.argon_phase);
            self.argon_phase = (self.argon_phase + hz[1] / rate).fract();
        }
        let fold = at(2, knobs::CUSTOM);
        for x in osc[2][..n].iter_mut() {
            *x = self.folder.tick(hz[2] / rate, fold);
        }
        for o in 0..3 {
            let f = &mut self.filters[o];
            let ch = c.filters[o];
            f.target(ch, filters::cutoff_hz(at(o, knobs::CUTOFF)), filters::resonance_q(at(o, knobs::RESONANCE)), rate, n);
            let to = osc_gain(at(o, knobs::GAIN));
            let step = (to - self.gains[o]) / n as f32;
            let cycle = hz[o] / rate;
            for x in osc[o][..n].iter_mut() {
                self.gains[o] += step;
                let y = f.tick(ch, *x);
                // The scope: one cycle, a point per 1/SCOPE of it, kept when it closes.
                let ph = &mut self.scope_phase[o];
                let slice = &mut self.scope_fill[o][((*ph * SCOPE as f32) as usize).min(SCOPE - 1)];
                slice.0 += y;
                slice.1 += 1;
                *ph += cycle;
                if *ph >= 1.0 {
                    *ph -= 1.0;
                    for (out, slice) in self.scopes[o].iter_mut().zip(self.scope_fill[o].iter_mut()) {
                        if slice.1 > 0 {
                            *out = slice.0 / slice.1 as f32;
                        }
                        *slice = (0.0, 0);
                    }
                }
                *x = y * self.gains[o];
            }
            self.gains[o] = to;
        }

        // The inserts: who feeds whom, each effect's Division kept apart from the others'.
        let order = ORDERS[c.order.min(5)];
        let divisions = resolve(c.divisions, rows);
        let beat = 60.0 / transport.tempo.max(1.0) as f32 * rate;
        let settings: [Setting; EFFECTS] = std::array::from_fn(|e| Setting {
            delay: (DIVISIONS[divisions[e]] * beat).min(self.tanks[e].longest()),
            feedback: moved[knobs::feedback(e)],
            custom: moved[knobs::custom(e)],
            mix: c.mixes[e],
        });
        for (tank, s) in self.tanks.iter_mut().zip(&settings) {
            tank.prepare(s);
        }
        let route_step = n as f32 / (ROUTE_S * rate);
        let mut weights = self.route;
        for r in 0..3 {
            for e in 0..EFFECTS {
                let target = (order[r] == e) as i32 as f32;
                let w = &mut self.route[r][e];
                *w += (target - *w).clamp(-route_step, route_step);
            }
        }
        let step: [[f32; EFFECTS]; 3] = std::array::from_fn(|r| std::array::from_fn(|e| (self.route[r][e] - weights[r][e]) / n as f32));

        // Fuzz and Morph settings.
        let hp_a = self.fuzz.hp_coefficient(moved[knobs::FUZZ_HP]);
        let fuzz_gain = moved[knobs::FUZZ_GAIN];
        // Depth scales the Slant too, so at off the Morph is off altogether.
        let depth = c.morph_depth.clamp(0.0, 1.0);
        let morph = HarmonicSettings { shape: c.morph_shape as f32, shift: 0.0, tilt: depth * slant_db(moved[knobs::SLANT]), depth, odd_even: 0.0 };
        let (time_s, fade) = reverb_time(moved[knobs::REVERB_TIME]);
        let reverb = ReverbControls { time_s, mix: 1.0, level: 1.0, ..ReverbControls::default() };
        let (dry, wet) = ((REVERB_MIX * fade * FRAC_PI_2).cos(), (REVERB_MIX * fade * FRAC_PI_2).sin());

        let mut sum = [[0.0f32; CONTROL]; 2];
        for i in 0..n {
            let mut l = 0.0;
            let mut r = 0.0;
            for e in 0..EFFECTS {
                let mut input = 0.0;
                for row in 0..3 {
                    weights[row][e] += step[row][e];
                    input += weights[row][e] * osc[row][i];
                }
                let (out, _) = self.tanks[e].tick(input, &settings[e]);
                l += out[0];
                r += out[1];
            }
            self.mods.feed_silver(0.5 * (l + r));
            let [fl, fr] = self.fuzz.tick([l, r], fuzz_gain, hp_a);
            sum[0][i] = fl;
            sum[1][i] = fr;
        }
        // Spectral Morph needs a fundamental: it tracks oscillator I.
        if morph.on() {
            for (eq, side) in self.morph.iter_mut().zip(sum.iter_mut()) {
                eq.process(&morph, hz[0], rate, &mut side[..n]);
            }
        }
        let to = output_gain(c.output);
        let out_step = (to - self.out_gain) / n as f32;
        let mut peak = 0.0f32;
        for i in 0..n {
            let x = [sum[0][i], sum[1][i]];
            let w = self.reverb.process(x, &reverb);
            self.out_gain += out_step;
            let mixed = [(dry * x[0] + wet * w[0]) * self.out_gain, (dry * x[1] + wet * w[1]) * self.out_gain];
            let [a, b] = self.limiter.process(mixed);
            left[i] = a;
            right[i] = b;
            peak = peak.max(a.abs()).max(b.abs());
        }
        self.out_gain = to;
        self.report = Report { mods, moved, divisions, peak: self.report.peak.max(peak), scopes: self.scopes };
    }

    /// The loudest output since the last call.
    pub fn take_peak(&mut self) -> f32 {
        std::mem::take(&mut self.report.peak)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render(c: &Controls, seconds: f32) -> (Vec<f32>, Vec<f32>) {
        let rate = 48_000.0;
        let mut e = Engine::new(rate);
        let n = (seconds * rate) as usize;
        let (mut l, mut r) = (vec![0.0; n], vec![0.0; n]);
        for (a, b) in l.chunks_mut(512).zip(r.chunks_mut(512)) {
            e.process(a, b, c, Transport { tempo: 120.0, beats: None });
        }
        (l, r)
    }

    fn rms(x: &[f32]) -> f32 {
        (x.iter().map(|v| v * v).sum::<f32>() / x.len() as f32).sqrt()
    }

    #[test]
    fn it_drones_out_of_the_box() {
        let (l, r) = render(&Controls::default(), 4.0);
        assert!(l.iter().chain(&r).all(|x| x.is_finite() && x.abs() <= 0.95));
        let level = 20.0 * rms(&l[96_000..]).log10();
        println!("default level {level:.1} dB RMS");
        assert!(level > -30.0 && level < -3.0, "{level}");
        // Stereo (the chorus, the reverb).
        assert!(l.iter().zip(&r).any(|(a, b)| (a - b).abs() > 1e-3));
    }

    #[test]
    fn every_setting_stays_bounded() {
        let mut c = Controls::default();
        c.knobs = [1.0; ARMED_COUNT];
        c.depths = [[1.0; 4]; ARMED_COUNT];
        c.morph_shape = 40;
        c.filters = [Character::Wasp; 3];
        let (l, _) = render(&c, 3.0);
        assert!(l.iter().all(|x| x.is_finite() && x.abs() <= 0.95));
        c.knobs = [0.0; ARMED_COUNT];
        c.output = 0.0;
        let (l, _) = render(&c, 1.0);
        assert!(l.iter().all(|x| x.abs() < 1e-6));
    }

    #[test]
    fn scopes_hold_a_cycle() {
        let mut e = Engine::new(48_000.0);
        let (mut l, mut r) = (vec![0.0; 48_000], vec![0.0; 48_000]);
        e.process(&mut l, &mut r, &Controls::default(), Transport { tempo: 120.0, beats: None });
        for o in 0..3 {
            let s = e.report.scopes[o];
            let (lo, hi) = s.iter().fold((f32::MAX, f32::MIN), |(a, b), &v| (a.min(v), b.max(v)));
            assert!(hi - lo > 0.05, "oscillator {o}: {lo}..{hi}");
        }
    }

    #[test]
    fn a_plain_sine_scopes_as_a_sine() {
        let mut c = Controls::default();
        c.knobs[knobs::osc(2, knobs::CUSTOM)] = 0.0;
        c.knobs[knobs::osc(2, knobs::CUTOFF)] = 1.0;
        c.knobs[knobs::osc(2, knobs::RESONANCE)] = 0.0;
        c.filters[2] = Character::Svf;
        let mut e = Engine::new(48_000.0);
        let (mut l, mut r) = (vec![0.0; 48_000], vec![0.0; 48_000]);
        e.process(&mut l, &mut r, &c, Transport { tempo: 120.0, beats: None });
        let s = e.report.scopes[2];
        // Nearly all its energy in the first harmonic.
        let (mut re, mut im, mut all) = (0.0, 0.0, 0.0);
        for (i, v) in s.iter().enumerate() {
            let a = std::f32::consts::TAU * i as f32 / SCOPE as f32;
            re += v * a.cos();
            im += v * a.sin();
            all += v * v;
        }
        let first = 2.0 * (re * re + im * im) / SCOPE as f32;
        assert!(first / all > 0.97, "{}", first / all);
    }

    #[test]
    fn mix_off_and_depth_off() {
        let mut c = Controls::default();
        let (base, _) = render(&c, 3.0);
        c.mixes = [0.0; EFFECTS];
        c.morph_depth = 0.0;
        let (dry, _) = render(&c, 3.0);
        assert!(dry.iter().all(|x| x.is_finite()));
        assert!(base.iter().zip(&dry).any(|(a, b)| (a - b).abs() > 1e-3));
    }

    #[test]
    fn pitch_is_centred_on_c2() {
        assert!((Engine::osc_hz(0.5) - 65.406).abs() < 1e-3);
        assert!((Engine::osc_hz(1.0) - 4.0 * 65.406).abs() < 0.01);
    }

    /// CPU: `cargo test --release cpu -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn cpu() {
        let mut c = Controls::default();
        c.morph_shape = 12;
        c.depths[3] = [0.4, 0.3, 0.2, 0.1];
        let t = std::time::Instant::now();
        let (l, _) = render(&c, 10.0);
        let s = t.elapsed().as_secs_f64();
        println!("10 s rendered in {:.2} s ({:.1} % of a core)", s, 10.0 * s);
        assert!(l.len() > 0);
    }
}
