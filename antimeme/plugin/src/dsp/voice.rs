//! One channel's Time/Filter experiment, as measured on a real Strega:
//!
//! ```text
//!               +--> chip A --+
//! input ------> +--> chip B --+--> sum --> Filter/Absorb --> output
//!               +--> chip C --+
//! ```
//!
//! Three chips in parallel, each on its own clock, none in sync. A and B each
//! feed themselves a fixed amount. Decay cross-couples them (and, weakly, C).
//! Filter and Absorb are outside the loop: closing the Filter doesn't change how
//! the repeats decay. The measurements behind the numbers are in docs/measurements.md.

use super::chip::{Chip, BITS};
use super::filters::{DcBlock, Follower, OnePole, Svf, SvfCoefficients};
use super::noise::Noise;
use super::table::{inverse_log, lookup, lookup_log};

const CHIPS: usize = 3;
const A: usize = 0;
const B: usize = 1;
const C: usize = 2;

/// Each chip's delay in seconds at measured Time knob positions: 0 is full
/// counter-clockwise (7:00), 1 is full clockwise (5:00).
const CHIP_DELAYS: [&[(f32, f32)]; CHIPS] = [
    &[(0.0, 3.138), (0.2, 1.212), (0.3, 0.545), (0.5, 0.197), (0.6, 0.098), (0.7, 0.0731), (0.8, 0.0488), (1.0, 0.0358)],
    &[(0.0, 5.764), (0.2, 2.359), (0.3, 1.071), (0.5, 0.377), (0.6, 0.174), (0.7, 0.1233), (0.8, 0.0757), (1.0, 0.0484)],
    // C's first two points are extrapolated: its echoes were too faint to find.
    &[(0.0, 12.0), (0.2, 9.476), (0.3, 2.681), (0.5, 0.778), (0.6, 0.332), (0.7, 0.2246), (0.8, 0.1255), (1.0, 0.0835)],
];

/// Into the chips: full level on them is about -6 dBFS at the Strega's input.
const INPUT_GAIN: f32 = 2.1;

/// What each chip contributes to the sum.
const OUTPUT_GAINS: [f32; CHIPS] = [0.42, 0.36, 0.216];

/// Fixed self-feedback: repeats fall 2.65 dB and 8 dB per pass, even with Decay down.
const SELF_FEEDBACK: [f32; CHIPS] = [0.737, 0.398, 0.0];

/// The cross-feedback between A and B at each Decay knob position.
const CROSS_FEEDBACK: &[(f32, f32)] = &[(0.0, 0.0), (0.2, 0.27), (0.5, 0.60), (0.7, 0.62), (0.9, 0.9), (1.0, 1.2)];

/// The cross-feedback only passes signal above a threshold (in chip level, full scale 1).
/// So big sounds hang on while quiet ones fall away at chip A's own rate, down to silence.
const CROSS_GATE: (f32, f32) = (0.08, 0.8);
const CROSS_GATE_ATTACK_MS: f32 = 1.0;
const CROSS_GATE_RELEASE_MS: f32 = 30.0;

/// The cross-feedback also loses its lows: why it self-oscillates near 1 kHz.
const CROSS_HIGHPASS_HZ: f32 = 420.0;

/// C couples to the others this much more weakly.
const C_COUPLING: f32 = 0.18;

/// The AC-coupled input, on the way into the chips.
const INPUT_HIGHPASS_HZ: f32 = 45.0;

/// Around each chip, inside the loop.
const LOOP_FILTER_HZ: f32 = 20_000.0;
const LOOP_FILTER_Q: f32 = 0.7;

/// On the sum: all the top end the Strega's delay ever has.
const OUTPUT_FILTER_HZ: f32 = 16_000.0;
const OUTPUT_FILTER_Q: f32 = 0.6;

/// The Filter's cutoff, fitted from sweeps: a low-Q two-pole, out of the way at full clockwise.
const FILTER_CUTOFF: &[(f32, f32)] = &[(0.0, 98.0), (0.8, 8533.0), (1.0, 120_000.0)];
const FILTER_Q: f32 = 0.4;

/// Slowest bit rate the clocks reach, and the chip's fastest.
const MIN_BIT_RATE: f32 = 500.0;
const MAX_BIT_RATE: f32 = BITS as f32 / 0.031;

/// How far a full-depth modulation moves Time.
const TIME_MOD_OCTAVES: f32 = 4.0;

/// Slow random wander in the clock, as a fraction of its rate.
const CLOCK_JITTER: f32 = 0.002;

const INTERFERENCE_HZ: f32 = 30.0;
const INTERFERENCE_GAIN: f32 = 3.0;

/// The first echo's delay at a Time knob position.
pub fn time_seconds(time: f32) -> f32 {
    lookup_log(CHIP_DELAYS[A], time)
}

/// The Time knob position for a first echo's delay: the inverse of `time_seconds`.
pub fn time_knob(seconds: f32) -> f32 {
    inverse_log(CHIP_DELAYS[A], seconds)
}

/// Absorb resonates at the cutoff and lets the highs back in, fitted from sweeps
/// at two Filter settings, to within about half a dB:
/// output = low + high_gain * high + band_gain * band.
struct AbsorbShape {
    cutoff_scale: f32,
    q: f32,
    high_gain: f32,
    band_gain: f32,
}

#[inline(always)]
fn absorb_shape(absorb: f32) -> AbsorbShape {
    let a = absorb.clamp(0.0, 1.0);
    let a2 = a * a;
    AbsorbShape {
        cutoff_scale: 1.0 - 0.22 * a,
        q: FILTER_Q + 0.8 * a + 0.8 * a2 * a2,
        high_gain: 2.2 * a2 * a,
        band_gain: 5.1 * a2,
    }
}

/// Everything a voice needs for one internal sample.
#[derive(Clone, Copy, Default)]
pub struct Frame {
    pub time: f32,
    pub decay: f32,
    pub filter: f32,
    pub absorb: f32,
    pub whine: f32,
    /// Multiplies the clocks: how the channels are offset from each other.
    pub clock_ratio: f32,
    /// Agitation's current value, already scaled for each destination.
    pub agitation_time: f32,
    pub agitation_filter: f32,
    pub agitation_decay: f32,
    pub agitation_absorb: f32,
    pub agitation_whine: f32,
    /// Depths for this voice's own Interference.
    pub interference_time: f32,
    pub interference_filter: f32,
    pub interference_decay: f32,
    pub interference_absorb: f32,
    pub interference_whine: f32,
}

struct Line {
    chip: Chip,
    filter: Svf,
    output: f32,
    /// Its output as the cross-feedback sees it.
    cross_highpass: DcBlock,
    gate: Follower,
}

impl Line {
    /// This line's contribution to the cross-feedback: 0 at Decay down, and gated.
    #[inline(always)]
    fn cross(&mut self) -> f32 {
        let high = self.cross_highpass.process(self.output);
        let (open_below, open_above) = CROSS_GATE;
        let t = ((self.gate.process(high.abs()) - open_below) / (open_above - open_below)).clamp(0.0, 1.0);
        high * t * t * (3.0 - 2.0 * t)
    }
}

pub struct Voice {
    lines: [Line; CHIPS],
    loop_filter: SvfCoefficients,
    input_highpass: DcBlock,
    sum_dc: DcBlock,
    output_filter: Svf,
    output_coefficients: SvfCoefficients,
    filter: Svf,
    interference_filter: OnePole,
    jitter: Noise,
    wander: OnePole,
    interference: f32,
    sample_rate: f32,
    dt: f32,
}

impl Voice {
    pub fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            lines: std::array::from_fn(|i| Line {
                chip: Chip::new(seed.wrapping_mul(31).wrapping_add(i as u32 + 1)),
                filter: Svf::default(),
                output: 0.0,
                cross_highpass: DcBlock::new(CROSS_HIGHPASS_HZ, sample_rate),
                gate: Follower::new(CROSS_GATE_ATTACK_MS, CROSS_GATE_RELEASE_MS, sample_rate),
            }),
            loop_filter: SvfCoefficients::new(LOOP_FILTER_HZ, LOOP_FILTER_Q, sample_rate),
            input_highpass: DcBlock::new(INPUT_HIGHPASS_HZ, sample_rate),
            sum_dc: DcBlock::new(10.0, sample_rate),
            output_filter: Svf::default(),
            output_coefficients: SvfCoefficients::new(OUTPUT_FILTER_HZ, OUTPUT_FILTER_Q, sample_rate),
            filter: Svf::default(),
            interference_filter: OnePole::new(INTERFERENCE_HZ, sample_rate),
            jitter: Noise::new(seed.wrapping_add(7919)),
            wander: OnePole::new(20.0, sample_rate),
            interference: 0.0,
            sample_rate,
            dt: 1.0 / sample_rate,
        }
    }

    /// The signal the loop modulates itself with, -1..1.
    pub fn interference(&self) -> f32 {
        self.interference
    }

    /// What multiplies every clock this sample: modulation, the channel offset and wander.
    fn clock_scale(&mut self, frame: &Frame) -> f32 {
        let octaves = TIME_MOD_OCTAVES * (frame.agitation_time + self.interference * frame.interference_time);
        let wander = self.wander.process(self.jitter.next()) * CLOCK_JITTER * 40.0;
        2f32.powf(octaves) * frame.clock_ratio * (1.0 + wander)
    }

    #[inline]
    pub fn process(&mut self, input: f32, frame: &Frame) -> f32 {
        let clock_scale = self.clock_scale(frame);

        let decay = (frame.decay + frame.agitation_decay + self.interference * frame.interference_decay).clamp(0.0, 1.0);
        let cross = lookup(CROSS_FEEDBACK, decay);
        let weak = cross * C_COUPLING;

        let feed = self.input_highpass.process(input) * INPUT_GAIN;
        let [cross_a, cross_b, cross_c] = [self.lines[A].cross(), self.lines[B].cross(), self.lines[C].cross()];
        let feeds = [
            feed + SELF_FEEDBACK[A] * self.lines[A].output + cross * cross_b + weak * cross_c,
            feed + SELF_FEEDBACK[B] * self.lines[B].output + cross * cross_a + weak * cross_c,
            feed + weak * (cross_a + cross_b),
        ];

        let whine = (frame.whine + frame.agitation_whine + self.interference * frame.interference_whine).clamp(0.0, 1.0);

        let mut sum = 0.0;
        for (i, line) in self.lines.iter_mut().enumerate() {
            let rate = BITS as f32 / lookup_log(CHIP_DELAYS[i], frame.time) * clock_scale;
            let raw = line.chip.process(feeds[i], rate.clamp(MIN_BIT_RATE, MAX_BIT_RATE), self.dt, whine);
            line.output = line.filter.lowpass(raw, &self.loop_filter);
            sum += line.output * OUTPUT_GAINS[i];
        }
        let raw = self.sum_dc.process(sum);
        self.interference = (self.interference_filter.process(raw) * INTERFERENCE_GAIN).clamp(-1.0, 1.0);

        let toned = self.output_filter.lowpass(raw, &self.output_coefficients);
        self.apply_filter(toned, frame)
    }

    /// The Filter and Absorb.
    #[inline(always)]
    fn apply_filter(&mut self, x: f32, frame: &Frame) -> f32 {
        let position = (frame.filter + frame.agitation_filter + self.interference * frame.interference_filter).clamp(0.0, 1.0);
        let shape = absorb_shape(frame.absorb + frame.agitation_absorb + self.interference * frame.interference_absorb);
        let cutoff = lookup_log(FILTER_CUTOFF, position) * shape.cutoff_scale;
        let outputs = self.filter.process(x, &SvfCoefficients::new(cutoff, shape.q, self.sample_rate));
        outputs.low + shape.high_gain * outputs.high + shape.band_gain * outputs.band
    }
}
