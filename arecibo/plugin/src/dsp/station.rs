//! One channel of the machine: a mixer strip and, on the way back round the loop, a
//! transmitter, a stretch of sky and a receiver. The two run side by side.

use super::filters::{rate_for, DcBlock, Svf};
use super::hilbert::Hilbert;
use super::random::Rng;
use super::sky::{Atmosphere, Fading, Path};
use super::wear::Effect;

/// The farthest a signal can travel, once round the planet: about 138 ms.
pub const LONGEST_DISTANCE_S: f32 = 0.16;
const MAINS_HISS: f32 = 1.5e-4;
const STRIP_DRIVE_BIAS: f32 = 0.06;
const PEAK_Q: f32 = 4.0;
const PEAK_BOOST: f32 = 1.2;
const LOW_CUT_HZ: f32 = 35.0;
const TOP_HZ: f32 = 15_000.0;
const TRANSMIT_LOW_HZ: f32 = 120.0;
const FILTER_Q: f32 = 0.75;
/// What the AGC aims the signal at, and the most gain it has to do it with.
const AGC_TARGET: f32 = 0.4;
pub const AGC_MOST: f32 = 10.0;
const AGC_ATTACK_S: f32 = 0.004;
const AUDIO_STAGE: f32 = 0.8;
/// How strongly an outside signal grabs the AGC. A strong signal close by overloads a
/// receiver's front end, whatever it's tuned to, and the AGC drops to protect it.
const DESENSE: f32 = 2.5;
/// How far a second path arrives behind the first, in seconds at worst weather.
const MULTIPATH_S: f32 = 0.006;
const DOPPLER_CALM_HZ: f32 = 0.05;
const DOPPLER_STORM_HZ: f32 = 6.0;
/// How strong the second path is against the first.
const SECOND_PATH: f32 = 0.75;
/// The far-off carrier that's sometimes there.
const BEACON_LEVEL: f32 = 0.05;

#[derive(Clone, Copy)]
pub struct Tuning {
    /// How far the receiver's mistuned, in Hz.
    pub shift_hz: f32,
    pub peak_hz: f32,
    pub gain: f32,
    /// 0 all wire, 1 all sky.
    pub sky: f32,
    /// Receiver bandwidth in Hz.
    pub width_hz: f32,
    /// How long the AGC takes to let go, in seconds.
    pub agc_release_s: f32,
    pub weather: f32,
    pub distance_s: f32,
}

/// A slow keyed carrier from nobody in particular.
#[derive(Clone)]
struct Beacon {
    phase: f32,
    step: f32,
    until_change: f32,
    keyed: f32,
    level: f32,
    per_sample: f32,
    fade_rate: f32,
    rng: Rng,
}

impl Beacon {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = Rng::new(seed);
        let hz = rng.range(400.0, 1_800.0);
        Self {
            phase: 0.0,
            step: std::f32::consts::TAU * hz / sample_rate,
            until_change: rng.range(1.0, 20.0),
            keyed: 0.0,
            level: 0.0,
            per_sample: 1.0 / sample_rate,
            fade_rate: rate_for(0.004, sample_rate),
            rng,
        }
    }

    #[inline(always)]
    fn next(&mut self, weather: f32) -> (f32, f32) {
        self.until_change -= self.per_sample;
        if self.until_change <= 0.0 {
            // Long silences, then a run of slow dits and dahs.
            if self.keyed > 0.5 {
                self.keyed = 0.0;
                self.until_change = if self.rng.chance(0.15) { self.rng.range(4.0, 40.0) } else { self.rng.range(0.1, 0.3) };
            } else {
                self.keyed = 1.0;
                self.until_change = if self.rng.chance(0.3) { 0.45 } else { 0.15 };
            }
        }
        self.level += (self.keyed - self.level) * self.fade_rate;
        self.phase += self.step;
        if self.phase > std::f32::consts::TAU {
            self.phase -= std::f32::consts::TAU;
        }
        let amount = BEACON_LEVEL * weather * self.level;
        let (sin, cos) = self.phase.sin_cos();
        (cos * amount, sin * amount)
    }
}

pub struct Station {
    rate: f32,
    // The strip.
    low_cut: Svf,
    peak: Svf,
    top: Svf,
    dc: DcBlock,
    // The transmitter.
    transmit_low: Svf,
    transmit_high: Svf,
    hilbert: Hilbert,
    // The sky.
    path: Path,
    fades: [Fading; 2],
    atmosphere: Atmosphere,
    beacon: Beacon,
    wander: f32,
    wander_rate: f32,
    // The receiver.
    envelope: f32,
    hang_left: f32,
    per_sample: f32,
    attack: f32,
    release: f32,
    phase: f32,
    receive_low: [Svf; 2],
    receive_dc: DcBlock,
    rng: Rng,
    /// The S-meter: how much gain the AGC is using.
    pub agc_gain: f32,
}

#[inline(always)]
fn times(a: (f32, f32), b: (f32, f32)) -> (f32, f32) {
    (a.0 * b.0 - a.1 * b.1, a.0 * b.1 + a.1 * b.0)
}

impl Station {
    pub fn new(rate: f32, seed: u32) -> Self {
        let path_samples = ((LONGEST_DISTANCE_S + MULTIPATH_S) * rate) as usize + 8;
        let mut low_cut = Svf::default();
        low_cut.set(LOW_CUT_HZ, 0.707, rate);
        let mut top = Svf::default();
        top.set(TOP_HZ, 0.707, rate);
        let mut transmit_low = Svf::default();
        transmit_low.set(TRANSMIT_LOW_HZ, FILTER_Q, rate);
        Self {
            rate,
            low_cut,
            peak: Svf::default(),
            top,
            dc: DcBlock::default(),
            transmit_low,
            transmit_high: Svf::default(),
            hilbert: Hilbert::new(),
            path: Path::new(path_samples),
            fades: [Fading::new(rate, seed.wrapping_add(11)), Fading::new(rate, seed.wrapping_add(23))],
            atmosphere: Atmosphere::new(rate, seed.wrapping_add(37)),
            beacon: Beacon::new(rate, seed.wrapping_add(41)),
            wander: 0.0,
            wander_rate: rate_for(3.0, rate),
            envelope: 0.0,
            hang_left: 0.0,
            per_sample: 1.0 / rate,
            attack: rate_for(AGC_ATTACK_S, rate),
            release: rate_for(1.0, rate),
            phase: 0.0,
            receive_low: [Svf::default(), Svf::default()],
            receive_dc: DcBlock::default(),
            rng: Rng::new(seed.wrapping_add(53)),
            agc_gain: 1.0,
        }
    }

    /// Filters don't need changing every sample. A few times a millisecond is smooth.
    pub fn retune(&mut self, tuning: &Tuning, warmth: f32) {
        let rate = self.rate;
        // Hot parts run sharp, a little.
        self.peak.set(tuning.peak_hz * (1.0 + 0.06 * warmth), PEAK_Q, rate);
        self.transmit_high.set(tuning.width_hz, FILTER_Q, rate);
        self.receive_low.iter_mut().for_each(|f| f.set(tuning.width_hz, FILTER_Q, rate));
        self.release = rate_for(0.25 * tuning.agc_release_s, rate);
        let spread = DOPPLER_CALM_HZ * (DOPPLER_STORM_HZ / DOPPLER_CALM_HZ).powf(tuning.weather);
        self.fades.iter_mut().for_each(|f| f.set_spread(spread));
    }

    /// The strip: trim, a transistor stage running out of headroom, and a resonant peak.
    #[inline(always)]
    fn strip(&mut self, x: f32, tuning: &Tuning, wear: &Effect) -> f32 {
        let trimmed = x * tuning.gain * wear.gain;
        // A touch of bias gives it even harmonics, as real stages have.
        let driven = (trimmed + STRIP_DRIVE_BIAS).tanh() - STRIP_DRIVE_BIAS.tanh();
        // Unity at the peak, so the Feedback knob means the same wherever the peak is.
        let shaped = (driven + PEAK_BOOST * self.peak.process(driven).band) / (1.0 + PEAK_BOOST);
        let low = self.low_cut.process(shaped).high;
        self.dc.process(self.top.process(low).low)
    }

    /// The trip through the air and the receiver at the far end.
    #[inline(always)]
    fn radio(&mut self, x: f32, outside: f32, tuning: &Tuning, wear: &Effect) -> f32 {
        let high_passed = self.transmit_low.process(x).high;
        let z = self.hilbert.process(self.transmit_high.process(high_passed).low);
        self.path.write(z);

        // Two routes through a sky that's always moving, however slightly.
        self.wander += (self.rng.signed() - self.wander) * self.wander_rate;
        let first_delay = (tuning.distance_s * self.rate + self.wander * 2.0).max(1.0);
        let second_delay = first_delay + tuning.weather * MULTIPATH_S * self.rate;
        let first = times(self.path.read(first_delay), self.fades[0].gain());
        let second = times(self.path.read(second_delay), self.fades[1].gain());
        // On a calm day the fading matters much less, as there's a steady path as well.
        let steady = 1.0 - tuning.weather;
        let direct = self.path.read(first_delay);
        let air = (
            steady * direct.0 + tuning.weather * first.0 + SECOND_PATH * tuning.weather * second.0,
            steady * direct.1 + tuning.weather * first.1 + SECOND_PATH * tuning.weather * second.1,
        );

        let noise = self.atmosphere.next(tuning.weather);
        let carrier = self.beacon.next(tuning.weather);
        let heard = (air.0 + noise.0 + carrier.0, air.1 + noise.1 + carrier.1);

        // The AGC turns up until it finds something, including nothing.
        let strength = (heard.0 * heard.0 + heard.1 * heard.1).sqrt().max(outside.abs() * DESENSE);
        // Hang AGC: after a peak it holds its gain for a while, then lets go. In a loop, that
        // starves the signal and then floods it, which is the breathing.
        if strength > self.envelope {
            self.envelope += (strength - self.envelope) * self.attack;
            self.hang_left = tuning.agc_release_s;
        } else if self.hang_left > 0.0 {
            self.hang_left -= self.per_sample;
        } else {
            self.envelope += (strength - self.envelope) * self.release;
        }
        self.agc_gain = (AGC_TARGET / self.envelope.max(AGC_TARGET / AGC_MOST)).min(AGC_MOST);

        // Mistuned by a little: everything comes out moved by that much.
        let step = std::f32::consts::TAU * (tuning.shift_hz + wear.pull_hz) / self.rate;
        self.phase += step;
        if self.phase.abs() > std::f32::consts::TAU {
            self.phase -= std::f32::consts::TAU * self.phase.signum();
        }
        let (sin, cos) = self.phase.sin_cos();
        // A worn phasing network: not quite a quarter turn, not quite balanced.
        let detected = (heard.0 * cos - (1.0 - wear.imbalance) * heard.1 * sin) * self.agc_gain;
        let filtered = self.receive_low.iter_mut().fold(detected, |y, f| f.process(y).low);
        // The audio stage overloads before the AGC gets hold of a crash.
        AUDIO_STAGE * (self.receive_dc.process(filtered) / AUDIO_STAGE).tanh()
    }

    /// One sample: what comes round the loop, what's arriving from outside, and what comes
    /// out of this channel.
    #[inline(always)]
    pub fn process(&mut self, looped: f32, outside: f32, tuning: &Tuning, wear: &Effect) -> f32 {
        let hiss = self.rng.signed() * MAINS_HISS;
        let strip = self.strip(looped + outside + hiss + wear.dirt, tuning, wear);
        let radio = self.radio(strip, outside, tuning, wear);
        strip * (1.0 - tuning.sky) + radio * tuning.sky
    }
}
