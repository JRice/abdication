//! The whole unit: analog in, converters on the machine's clock, the machine, stairsteps
//! out, analog out. One DAC serves both channels, so right comes out half a tick after
//! left, an offset that grows as the clock slows.
//!
//! The analog side runs at a high fixed rate. The clock can be anywhere below it, and the
//! converter's edges land between analog samples.

use super::analog::{Cliff, Shelf};
use super::limiter::Limiter;
use super::machine::{Knobs, Machine};
use super::ram::Rng;

/// The output filter's cliff, measured from the recording.
const CLIFF_HZ: f32 = 13_300.0;
pub const STOCK_HZ: f32 = 31_250.0;
pub const MIN_HZ: f32 = 180.0;
pub const MAX_HZ: f32 = 44_000.0;
const ANALOG_MIN_RATE: f32 = 150_000.0;
const FULL_SCALE: f32 = 32_767.0;
/// Noise in the input stage, which a slow clock folds down into the audio.
const INPUT_NOISE: f32 = 3e-5;
/// How long the clock's spike on the analog ground takes to die away.
const SPIKE_S: f32 = 40e-6;
const WHINE_PEAK: f32 = 0.25;

#[derive(Clone, Copy)]
pub struct Controls {
    pub clock_hz: f32,
    pub drive: f32,
    /// Clean signal around the machine altogether.
    pub direct: f32,
    pub whine: f32,
    pub level: f32,
}

/// One channel's analog path. The converter steps are the machine's.
struct Channel {
    anti_alias: Cliff,
    emphasis: Shelf,
    de_emphasis: Shelf,
    output: Cliff,
    /// The analog input now and one analog sample ago, for edges between them.
    input: [f32; 2],
    dac: f32,
}

impl Channel {
    fn new(rate: f32) -> Self {
        Self {
            anti_alias: Cliff::new(CLIFF_HZ, rate),
            emphasis: Shelf::emphasis(rate),
            de_emphasis: Shelf::de_emphasis(rate),
            output: Cliff::new(CLIFF_HZ, rate),
            input: [0.0; 2],
            dac: 0.0,
        }
    }

    /// The converter's reading `early` of the way back through this analog sample.
    fn sample(&self, early: f32, drive: f32) -> i16 {
        let held = self.input[0] + (self.input[1] - self.input[0]) * early;
        ((held * drive).clamp(-1.0, 1.0) * FULL_SCALE) as i16
    }
}

pub struct Engine {
    machine: Machine,
    channels: [Channel; 2],
    limiter: Limiter,
    oversampling: usize,
    /// Where the clock is in its cycle: a tick at 0, right's output at one half.
    phase: f64,
    per_sample: f64,
    /// The right channel's next value: it's output half a cycle after left's.
    right_pending: Option<f32>,
    spike: f32,
    spike_decay: f32,
    previous: [f32; 2],
    rng: Rng,
}

impl Engine {
    pub fn new(sample_rate: f32) -> Self {
        let oversampling = (ANALOG_MIN_RATE / sample_rate).ceil().max(1.0) as usize;
        let rate = sample_rate * oversampling as f32;
        Self {
            machine: Machine::new(),
            channels: [Channel::new(rate), Channel::new(rate)],
            limiter: Limiter::new(sample_rate),
            oversampling,
            phase: 0.0,
            per_sample: 1.0 / rate as f64,
            right_pending: None,
            spike: 0.0,
            spike_decay: (-1.0 / (SPIKE_S * rate)).exp(),
            previous: [0.0; 2],
            rng: Rng::new(0xC10C),
        }
    }

    pub fn reduction_db(&self) -> f32 {
        self.limiter.reduction_db()
    }

    /// Once per block.
    pub fn set_knobs(&mut self, knobs: &Knobs) {
        self.machine.set_knobs(knobs);
    }

    #[inline]
    pub fn process(&mut self, input: [f32; 2], controls: &Controls) -> [f32; 2] {
        let hz = controls.clock_hz.clamp(MIN_HZ, MAX_HZ);
        self.machine.set_clock(hz);
        let step = hz as f64 * self.per_sample;
        let mut analog = [0.0; 2];
        for i in 0..self.oversampling {
            let along = (i + 1) as f32 / self.oversampling as f32;
            let incoming = [0, 1].map(|c| self.previous[c] + (input[c] - self.previous[c]) * along);
            analog = self.analog_step(incoming, step, controls);
        }
        self.previous = input;
        let output = [0, 1].map(|c| (analog[c] + input[c] * controls.direct) * controls.level);
        self.limiter.process(output)
    }

    /// One sample of the analog world, in which the clock may or may not go off.
    #[inline(always)]
    fn analog_step(&mut self, incoming: [f32; 2], step: f64, controls: &Controls) -> [f32; 2] {
        for (channel, x) in self.channels.iter_mut().zip(incoming) {
            let noise = (self.rng.bits() as i32 as f32) / i32::MAX as f32 * INPUT_NOISE;
            let filtered = channel.anti_alias.process(x + noise);
            channel.input = [channel.emphasis.process(filtered), channel.input[0]];
        }

        let before = self.phase;
        self.phase += step;
        // Fractions of this analog sample spent on the old value of each channel.
        let mut staying = [1.0f32; 2];
        let mut steps = [None; 2];
        if before < 0.5 && self.phase >= 0.5 {
            if let Some(value) = self.right_pending.take() {
                staying[1] = 1.0 - ((self.phase - 0.5) / step) as f32;
                steps[1] = Some(value);
            }
        }
        if self.phase >= 1.0 {
            self.phase -= 1.0;
            let after = (self.phase / step) as f32;
            let converted = [0, 1].map(|c| self.channels[c].sample(1.0 - after, controls.drive));
            let out = self.machine.tick(converted);
            staying[0] = 1.0 - after;
            steps[0] = Some(out[0] as f32 / FULL_SCALE);
            self.right_pending = Some(out[1] as f32 / FULL_SCALE);
            self.spike += WHINE_PEAK * controls.whine * controls.whine;
        }
        let spike = self.spike;
        self.spike *= self.spike_decay;

        [0, 1].map(|c| {
            let channel = &mut self.channels[c];
            let stair = match steps[c] {
                Some(next) => {
                    let blended = channel.dac * staying[c] + next * (1.0 - staying[c]);
                    channel.dac = next;
                    blended
                }
                None => channel.dac,
            };
            channel.output.process(channel.de_emphasis.process(stair) + spike)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SR: f32 = 48_000.0;

    fn run(engine: &mut Engine, clock_hz: f32, knobs: &Knobs, seconds: f32, mut input: impl FnMut(usize) -> [f32; 2]) -> Vec<[f32; 2]> {
        engine.set_knobs(knobs);
        let controls = Controls { clock_hz, drive: 1.0, direct: 0.0, whine: 0.0, level: 1.0 };
        (0..(seconds * SR) as usize).map(|n| engine.process(input(n), &controls)).collect()
    }

    fn rms(signal: &[f32]) -> f32 {
        (signal.iter().map(|x| x * x).sum::<f32>() / signal.len().max(1) as f32).sqrt()
    }

    fn burst(n: usize) -> [f32; 2] {
        let x = if n < 2_400 { 0.5 * ((n * n % 257) as f32 / 128.0 - 1.0) } else { 0.0 };
        [x, x]
    }

    #[test]
    fn the_stock_clock_passes_dry_signal_nearly_untouched() {
        let knobs = Knobs { mix: 0.0, ..Default::default() };
        let mut engine = Engine::new(SR);
        let tone = |n: usize| {
            let x = 0.25 * (2.0 * std::f32::consts::PI * 440.0 * n as f32 / SR).sin();
            [x, x]
        };
        let out = run(&mut engine, STOCK_HZ, &knobs, 0.5, tone);
        let left: Vec<f32> = out[4_800..].iter().map(|f| f[0]).collect();
        let gain = rms(&left) / (0.25 / 2f32.sqrt());
        assert!((gain - 1.0).abs() < 0.08, "{gain}");
    }

    #[test]
    fn right_lags_left_by_half_a_clock_tick() {
        // Measured on the recording too: the offset grows as the clock slows. At 1 kHz
        // that's half a millisecond.
        let knobs = Knobs { mix: 0.0, ..Default::default() };
        let mut engine = Engine::new(SR);
        let out = run(&mut engine, 1_000.0, &knobs, 0.1, |n| if n > 1_000 { [0.5; 2] } else { [0.0; 2] });
        let crossing = |c: usize| out.iter().position(|f| f[c] > 0.25).unwrap() as f32;
        let lag = (crossing(1) - crossing(0)).abs();
        assert!((lag - 24.0).abs() <= 2.0, "{lag}");
    }

    #[test]
    fn a_frozen_memory_repeats_exactly_once_around_the_ring() {
        // Measured from the recording: the loop lasts 16,384 clock ticks.
        const HZ: f32 = 16_384.0; // one second
        let knobs = Knobs { mix: 1.0, decay: 0.7, ..Default::default() };
        let mut engine = Engine::new(SR);
        run(&mut engine, HZ, &knobs, 0.9, |n| burst(n * 3));
        let frozen = Knobs { loop_ram: true, ..knobs };
        let out = run(&mut engine, HZ, &frozen, 2.5, |_| [0.0; 2]);
        let left: Vec<f32> = out.iter().map(|f| f[0]).collect();
        let (period, start, n) = (SR as usize, 2_400, 24_000);
        let (a, b) = (&left[start..start + n], &left[start + period..start + period + n]);
        let difference: Vec<f32> = a.iter().zip(b).map(|(x, y)| x - y).collect();
        assert!(rms(a) > 1e-3, "nothing in the loop: {}", rms(a));
        assert!(rms(&difference) < rms(a) * 0.1, "{} vs {}", rms(&difference), rms(a));
    }

    #[test]
    fn reverb_tails_die_away_and_nothing_blows_up_on_any_program() {
        for program in 0..super::super::program::PROGRAM_COUNT {
            let knobs = Knobs { program, mix: 1.0, decay: 0.5, ..Default::default() };
            let mut engine = Engine::new(SR);
            let out = run(&mut engine, STOCK_HZ, &knobs, 6.0, burst);
            let left: Vec<f32> = out.iter().map(|f| f[0].abs().max(f[1].abs())).collect();
            let (early, late) = (rms(&left[..24_000]), rms(&left[out.len() - 24_000..]));
            assert!(early > 1e-3, "program {program} silent");
            assert!(late < early * 0.1, "program {program}: {late} vs {early}");
            assert!(left.iter().all(|x| x.is_finite() && *x <= 1.0));
        }
    }

    #[test]
    fn every_pair_of_switches_stays_bounded_at_any_clock() {
        for a in 0..16 {
            for b in (a + 1)..16 {
                let mut knobs = Knobs { mix: 1.0, decay: 0.9, contention: 0.3, ..Default::default() };
                knobs.matrix[a] = true;
                knobs.matrix[b] = true;
                let mut engine = Engine::new(SR);
                for out in run(&mut engine, 4_000.0 + 2_000.0 * a as f32, &knobs, 0.2, burst) {
                    assert!(out[0].is_finite() && out[0].abs() <= 1.0 && out[1].is_finite() && out[1].abs() <= 1.0);
                }
            }
        }
    }

    #[test]
    fn slowing_the_clock_lengthens_the_delay_by_the_same_ratio() {
        let echo = |hz: f32| {
            let knobs = Knobs { program: 4, mix: 1.0, decay: 0.0, damp: 0.0, predelay: 0.0, ..Default::default() };
            let mut engine = Engine::new(SR);
            let out = run(&mut engine, hz, &knobs, 2.0, |n| if n < 48 { [0.8; 2] } else { [0.0; 2] });
            out.iter().position(|f| f[0].abs() > 0.05).unwrap_or(0) as f32
        };
        let (fast, slow) = (echo(31_250.0), echo(15_625.0));
        assert!(((slow / fast) - 2.0).abs() < 0.1, "{fast} {slow}");
    }
}
