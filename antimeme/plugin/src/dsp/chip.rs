//! The delay chip: a PT2399-style one-bit delta modulator and its RAM.
//!
//! One clock sets the delay. Every tick the comparator asks whether the input
//! is above the encoder's integrator, the answer is stored as a bit, and the
//! oldest bit in the RAM drives a matching decoder integrator. The integrators
//! leak (a one-pole lowpass towards +/-swing) and clip at +/-1, which gives the
//! chip's documented behaviour: clipping at low frequencies, slew limiting above.
//!
//! Slowing the clock makes each step bigger, so the noise, the whine at half the
//! bit rate and the crunch all grow with the delay, and sweeping the clock bends
//! whatever is in the RAM, without any of that being added separately.

use std::f32::consts::TAU;

use super::noise::Noise;

/// Size of the RAM, per the datasheet.
pub const BITS: usize = 44_000;

/// 1 / (2 pi R C) for the integrators: 5.6 k and about 80 nF, judging by how much top
/// end the Strega keeps at level. (The chip's own datasheet circuit uses 100 nF.)
const LEAK_HZ: f32 = 360.0;

/// Slewing towards +/-3.5 sets the slew limit: full level up to about 1.5 kHz.
const SWING: f32 = 3.5;

/// The comparator doesn't look at the error alone: it also accumulates it (leakily), so
/// small errors eventually flip a bit. That shapes the noise upwards, out of the audio
/// while the clock is fast but not once it's slow, and lets quiet signals through
/// instead of vanishing in a dead zone. The Strega's repeats fade smoothly into silence
/// and its noise falls about 15 dB per octave of clock, which a plain delta modulator
/// can't do. This is the corner of that accumulator, in hertz.
const ACCUMULATOR_HZ: f32 = 20_000.0;
const ACCUMULATOR_LEAK_HZ: f32 = 300.0;

/// Errors no bigger than this many steps don't accumulate, which lets signal fade cleanly.
const ACCUMULATOR_DEADBAND_STEPS: f32 = 0.6;

/// With next to no input (under this many steps), nothing accumulates at all. Silence is
/// then always the clean toggle at half the bit rate, which the decoder cancels; otherwise
/// it can lock into a toggle at a quarter of the bit rate, an idle tone in the audio.
const SILENCE_STEPS: f32 = 0.05;
const SILENCE_DEADBAND_STEPS: f32 = 1.1;

/// The burble under long delays: every so many bits the chip kicks its own output, heard
/// on the Strega as low tones whose pitch follows the clock (35, 69, 104... Hz at Time 9:00).
const ROW_BITS: u32 = 133;

/// Each kick's height: full scale once the delay is this long, falling steeply below it.
const KICK_FULL_DELAY: f32 = 9.5;
const KICK_LAW: f32 = 3.3;
const KICK_DECAY_RATE: f32 = 1.0 / 0.003;

/// Below this bit rate the clock itself gets unsteady, and the comparator's decisions turn
/// noisy, reaching this many steps of noise at `SHAKY_BIT_RATE_FULL` and below. It's what
/// makes the Strega hiss at long delays with no input: broad noise, not a clean tone.
const SHAKY_BIT_RATE: f32 = 40_000.0;
const SHAKY_BIT_RATE_FULL: f32 = 5_000.0;
const SHAKY_STEPS: f32 = 0.35;

/// Thermal noise at the comparator. Tiny: at a fast clock with no input the chip sits in a
/// perfect, ultrasonic limit cycle, and the Strega is measurably silent there.
const COMPARATOR_NOISE: f32 = 1e-5;

const LEAK_RATE: f32 = TAU * LEAK_HZ;

/// The op-amps' own lowpass after the decoder, run between the ticks. It stops a fast
/// clock's limit cycle from folding into the audio when sampled.
const SMOOTHING_RATE: f32 = TAU * 40_000.0;

/// e^-x for x >= 0, cheap and always positive: a [2/2] Pade approximant.
#[inline(always)]
fn decay(x: f32) -> f32 {
    let x = x.min(6.0);
    let half = x * 0.5;
    let square = x * x / 12.0;
    (1.0 - half + square) / (1.0 + half + square)
}

/// A stored bit as a drive level, -1 or 1.
#[inline(always)]
fn level(bit: u8) -> f32 {
    if bit != 0 {
        1.0
    } else {
        -1.0
    }
}

#[inline(always)]
fn integrate(state: f32, target: f32, keep: f32) -> f32 {
    (target + (state - target) * keep).clamp(-1.0, 1.0)
}

/// What one clock period implies for each tick.
struct TickCoefficients {
    keep: f32,
    deadband: f32,
    silence: f32,
    silence_deadband: f32,
    accumulate: f32,
    accumulator_keep: f32,
    kick_height: f32,
    comparator_noise: f32,
}

impl TickCoefficients {
    #[inline(always)]
    fn new(period: f32) -> Self {
        let keep = decay(LEAK_RATE * period);
        let step = SWING * (1.0 - keep);
        let shaky = ((SHAKY_BIT_RATE - 1.0 / period) / (SHAKY_BIT_RATE - SHAKY_BIT_RATE_FULL)).clamp(0.0, 1.0);
        Self {
            keep,
            comparator_noise: COMPARATOR_NOISE + shaky * SHAKY_STEPS * step,
            deadband: ACCUMULATOR_DEADBAND_STEPS * step,
            silence: SILENCE_STEPS * step,
            silence_deadband: SILENCE_DEADBAND_STEPS * step,
            accumulate: (TAU * ACCUMULATOR_HZ * period).min(1.0),
            accumulator_keep: decay(TAU * ACCUMULATOR_LEAK_HZ * period),
            kick_height: (BITS as f32 * period / KICK_FULL_DELAY).powf(KICK_LAW).min(1.0),
        }
    }
}

pub struct Chip {
    ram: Box<[u8]>,
    head: usize,
    /// How far through the current tick we are, 0..1.
    phase: f64,
    encoder: f32,
    accumulator: f32,
    decoder: f32,
    smoothed: [f32; 2],
    drive_current: f32,
    drive_previous: f32,
    row_position: u32,
    kick: f32,
    /// The bit now driving the decoder, and the one before it.
    output_bit: u8,
    previous_output_bit: u8,
    noise: Noise,
}

impl Chip {
    pub fn new(seed: u32) -> Self {
        // Alternating bits decode to silence.
        let ram = (0..BITS).map(|i| (i & 1) as u8).collect::<Vec<_>>().into_boxed_slice();
        Self { ram, head: 0, phase: 0.0, encoder: 0.0, accumulator: 0.0, decoder: 0.0, smoothed: [0.0; 2], drive_current: SWING, drive_previous: 0.0, row_position: 0, kick: 0.0, output_bit: 0, previous_output_bit: 1, noise: Noise::new(seed) }
    }

    /// Clock one bit in and one bit out.
    #[inline(always)]
    fn tick(&mut self, input: f32, c: &TickCoefficients) {
        self.row_position += 1;
        if self.row_position == ROW_BITS {
            self.row_position = 0;
            self.kick = c.kick_height;
        }

        let error = input - self.encoder;
        let deadband = if input.abs() < c.silence { c.silence_deadband } else { c.deadband };
        let beyond = error - error.clamp(-deadband, deadband);
        self.accumulator = (self.accumulator * c.accumulator_keep + beyond * c.accumulate).clamp(-1.0, 1.0);
        let bit = (error + self.accumulator + self.noise.next() * c.comparator_noise > 0.0) as u8;
        self.encoder = integrate(self.encoder, level(bit) * SWING, c.keep);
        self.previous_output_bit = self.output_bit;
        self.output_bit = std::mem::replace(&mut self.ram[self.head], bit);
        self.head = (self.head + 1) % BITS;
    }

    /// Advance by `dt` seconds at `bit_rate` bits per second. Returns the
    /// decoder's average over that time: its true output between the ticks, so
    /// a slow clock's steps don't alias more than they must.
    ///
    /// `whine` is how much of the idle tone at half the bit rate to keep, 0..1. With no
    /// input the bits simply alternate, which is that tone, in the audio once the clock is
    /// slow. Driving the decoder from the average of the last two bits cancels it exactly.
    /// It also rolls off the signal gently near that frequency, which is out of the
    /// way until the clock is very slow.
    pub fn process(&mut self, input: f32, bit_rate: f32, dt: f32, whine: f32) -> f32 {
        self.drive_current = SWING * 0.5 * (1.0 + whine);
        self.drive_previous = SWING * 0.5 * (1.0 - whine);

        let period = 1.0 / bit_rate;
        let coefficients = TickCoefficients::new(period);
        let mut remaining = dt;
        let mut area = 0.0;
        let mut until_tick = (1.0 - self.phase) as f32 * period;

        while remaining >= until_tick {
            area += self.advance_decoder(until_tick);
            remaining -= until_tick;
            self.tick(input, &coefficients);
            until_tick = period;
        }
        area += self.advance_decoder(remaining);
        self.phase = 1.0 - (until_tick - remaining) as f64 / period as f64;
        area / dt
    }

    /// Let the decoder and its lowpass run for `span` seconds; returns the area under them.
    #[inline(always)]
    fn advance_decoder(&mut self, span: f32) -> f32 {
        let target = self.drive_current * level(self.output_bit) + self.drive_previous * level(self.previous_output_bit);
        self.decoder = integrate(self.decoder, target, decay(LEAK_RATE * span));
        self.kick *= decay(KICK_DECAY_RATE * span);
        let heard = self.decoder + self.kick;
        let keep = decay(SMOOTHING_RATE * span);
        let before = self.smoothed[1];
        self.smoothed[0] = heard + (self.smoothed[0] - heard) * keep;
        self.smoothed[1] = self.smoothed[0] + (self.smoothed[1] - self.smoothed[0]) * keep;
        0.5 * (before + self.smoothed[1]) * span
    }
}
