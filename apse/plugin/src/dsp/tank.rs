//! The tank: two branches in a figure of eight, each feeding the other. Energy goes
//! round and round, smeared by the allpasses, darkened by the damping, shrunk by
//! Decay twice per branch, and read out by signed taps spread across both branches.

use std::f32::consts::TAU;

use super::delay::{Allpass, Delay};
use super::filters::{DcBlock, OnePole};
use super::modes::{Mode, Section, Side, REFERENCE_HZ, TANK, TAPS};

/// Below this the tank lets go, so nothing sub-audible can build up forever.
const TANK_LOW_CUT_HZ: f32 = 6.0;
/// Each of the four allpasses has its own LFO, at these multiples of the mode's rate,
/// so their movements never line up.
const LFO_RATIOS: [f32; 4] = [1.0, 1.137, 0.719, 0.853];
/// The second allpasses move this much of the first ones' excursion.
const SECOND_WOBBLE: f32 = 0.5;
/// An excursion never takes more than this share of a line's length.
const MAX_WOBBLE_SHARE: f32 = 0.4;
/// What the reference scales its output by.
const OUTPUT_GAIN: f32 = 0.6;

/// What the tank needs, worked out at control rate.
#[derive(Clone, Copy, Default)]
pub struct Setting {
    /// Applied twice per branch.
    pub decay: f32,
    /// The damping lowpass's coefficient.
    pub damp: f32,
    /// Peak excursion of the first allpasses, in samples.
    pub wobble: f32,
}

struct Branch {
    ap1: Allpass,
    d1: Delay,
    damp: OnePole,
    dc: DcBlock,
    ap2: Allpass,
    d2: Delay,
    /// The branch's last output, which feeds the other branch.
    out: f32,
}

impl Branch {
    fn new(longest: [f32; 4], sample_rate: f32) -> Self {
        Self {
            ap1: Allpass::new(longest[0]),
            d1: Delay::new(longest[1]),
            damp: OnePole::default(),
            dc: DcBlock::new(TANK_LOW_CUT_HZ, sample_rate),
            ap2: Allpass::new(longest[2]),
            d2: Delay::new(longest[3]),
            out: 0.0,
        }
    }

    fn clear(&mut self) {
        self.ap1.clear();
        self.d1.clear();
        self.damp.clear();
        self.dc.clear();
        self.ap2.clear();
        self.d2.clear();
        self.out = 0.0;
    }

    #[inline]
    fn run(&mut self, x: f32, lengths: [f32; 4], gains: [f32; 2], s: &Setting) {
        // The first allpass's sign is flipped, as in the reference.
        let a = self.ap1.process(x, lengths[0], -gains[0]);
        let b = self.d1.read(lengths[1]);
        self.d1.write(a);
        let b = self.dc.process(self.damp.process(b, s.damp)) * s.decay;
        let c = self.ap2.process(b, lengths[2], gains[1]);
        self.out = self.d2.read(lengths[3]) * s.decay;
        self.d2.write(c);
    }

    fn line(&self, section: Section) -> &Delay {
        match section {
            Section::Delay1 => &self.d1,
            Section::Allpass2 => self.ap2.line(),
            Section::Delay2 => &self.d2,
        }
    }
}

pub struct Tank {
    branches: [Branch; 2],
    /// Per branch, the four lengths in samples at Size 1, for the current mode.
    lengths: [[f32; 4]; 2],
    /// Per output, the seven taps' distances at Size 1 and their signed weights.
    taps: [[(f32, f32); 7]; 2],
    gains: [f32; 2],
    phases: [f32; 4],
    lfo_step: f32,
    sample_rate: f32,
}

impl Tank {
    /// A tank with room for the largest mode at `max_scale` times its size.
    pub fn new(sample_rate: f32, max_scale: f32, max_wobble: f32) -> Self {
        let k = max_scale * sample_rate / REFERENCE_HZ;
        let branch = |b: usize| Branch::new(TANK[b].map(|len| len * k + max_wobble + 4.0), sample_rate);
        Self {
            branches: [branch(0), branch(1)],
            lengths: [[0.0; 4]; 2],
            taps: [[(0.0, 0.0); 7]; 2],
            gains: [0.0; 2],
            phases: [0.0, 0.25, 0.5, 0.75],
            lfo_step: 0.0,
            sample_rate,
        }
    }

    /// Takes on a mode's dimensions. Doesn't clear: the engine decides that.
    pub fn configure(&mut self, mode: &Mode) {
        let k = mode.size * self.sample_rate / REFERENCE_HZ;
        self.lengths = TANK.map(|branch| branch.map(|len| len * k));
        for (output, taps) in TAPS.iter().enumerate() {
            for (i, t) in taps.iter().enumerate() {
                self.taps[output][i] = (t.at * k, t.sign * mode.taps[i]);
            }
        }
        self.gains = mode.tank_diffusion;
        self.lfo_step = mode.wobble_hz / self.sample_rate;
    }

    pub fn clear(&mut self) {
        self.branches.iter_mut().for_each(Branch::clear);
    }

    /// Seconds for one trip round the whole figure of eight at Size `scale`, during
    /// which the signal is multiplied by Decay four times.
    pub fn loop_seconds(&self, scale: f32) -> f32 {
        self.lengths.iter().flatten().sum::<f32>() * scale / self.sample_rate
    }

    /// Each allpass's excursion this sample, in samples, at full depth.
    #[inline]
    fn wobbles(&mut self) -> [f32; 4] {
        let step = self.lfo_step;
        std::array::from_fn(|i| {
            let p = &mut self.phases[i];
            *p = (*p + step * LFO_RATIOS[i]).fract();
            (TAU * *p).sin()
        })
    }

    #[inline]
    pub fn process(&mut self, x: f32, scale: f32, s: &Setting) -> [f32; 2] {
        let w = self.wobbles();
        let into = [x + self.branches[1].out, x + self.branches[0].out];
        for b in 0..2 {
            let [ap1, d1, ap2, d2] = self.lengths[b].map(|len| len * scale);
            let lengths = [
                ap1 + w[b] * s.wobble.min(MAX_WOBBLE_SHARE * ap1),
                d1,
                ap2 + w[b + 2] * (SECOND_WOBBLE * s.wobble).min(MAX_WOBBLE_SHARE * ap2),
                d2,
            ];
            self.branches[b].run(into[b], lengths, self.gains, s);
        }
        std::array::from_fn(|output| {
            let sum: f32 = TAPS[output]
                .iter()
                .zip(&self.taps[output])
                .filter(|(_, (_, weight))| *weight != 0.0)
                .map(|(t, (at, weight))| {
                    let branch = if t.side == Side::Near { output } else { 1 - output };
                    weight * self.branches[branch].line(t.section).tap(at * scale)
                })
                .sum();
            OUTPUT_GAIN * sum
        })
    }
}
