//! What's wrong with it. Nothing here is an effect someone chose: it's old equipment on a
//! shared power supply, so everything slowly depends on everything else. The Age knob
//! scales all of it, and at zero the machine is new.

use super::filters::rate_for;
use super::random::Rng;

pub const CHANNELS: usize = 4;
const SAG_S: f32 = 0.06;
/// How long a channel takes to warm up, per channel: they aren't the same.
const WARM_S: [f32; CHANNELS] = [31.0, 53.0, 83.0, 67.0];
/// How a loaded rail and a warm box push each oscillator, in Hz at full age.
const PULL_HZ: [f32; CHANNELS] = [2.6, -1.7, 3.4, -2.2];
const DRIFT_HZ: [f32; CHANNELS] = [-5.0, 7.0, 3.0, -4.0];
const MAINS_HZ: f32 = 60.0;
/// Bad joints open this often per second, at full age, and more when it's shaking.
const JOINT_RATE: f32 = 0.5;
const JOINT_SHAKE_RATE: f32 = 14.0;
const JOINT_MEAN_S: f32 = 0.012;
const JOINT_OPEN_GAIN: f32 = 0.06;
const SCRATCH_DECAY_S: f32 = 0.03;

#[derive(Clone, Copy, Default)]
struct Joint {
    open_for: f32,
}

/// What wear does to one channel this sample.
#[derive(Clone, Copy)]
pub struct Effect {
    /// What's left of the channel's gain.
    pub gain: f32,
    /// How far its oscillator has wandered, in Hz.
    pub pull_hz: f32,
    /// How warm it is, 0..1, for parts that drift with temperature.
    pub warmth: f32,
    /// Rubbish to add to the signal.
    pub dirt: f32,
    /// How unbalanced its sideband filter is, so a mirror image leaks through.
    pub imbalance: f32,
}

pub struct Wear {
    age: f32,
    rail: f32,
    rail_rate: f32,
    warmth: [f32; CHANNELS],
    warm_rates: [f32; CHANNELS],
    joints: [Joint; CHANNELS],
    scratch: [f32; CHANNELS],
    scratch_decay: f32,
    mains: f32,
    mains_step: f32,
    per_sample: f32,
    rng: Rng,
}

impl Wear {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            age: 0.0,
            rail: 0.0,
            rail_rate: rate_for(SAG_S, sample_rate),
            warmth: [0.0; CHANNELS],
            warm_rates: WARM_S.map(|s| rate_for(s, sample_rate)),
            joints: [Joint::default(); CHANNELS],
            scratch: [0.0; CHANNELS],
            scratch_decay: (-1.0 / (SCRATCH_DECAY_S * sample_rate)).exp(),
            mains: 0.0,
            mains_step: std::f32::consts::TAU * MAINS_HZ / sample_rate,
            per_sample: 1.0 / sample_rate,
            rng: Rng::new(0x1963),
        }
    }

    pub fn set_age(&mut self, age: f32) {
        self.age = age;
    }

    /// Someone touched a worn control on this channel: `amount` of its travel.
    pub fn touched(&mut self, channel: usize, amount: f32) {
        self.scratch[channel] = (self.scratch[channel] + amount * 6.0).min(1.0);
    }

    /// Tell it how hard the channels are being driven, once a sample.
    #[inline(always)]
    pub fn load(&mut self, levels: [f32; CHANNELS]) {
        let total = levels.iter().map(|l| l.abs()).sum::<f32>() / CHANNELS as f32;
        self.rail += (total - self.rail) * self.rail_rate;
        for ((warmth, rate), level) in self.warmth.iter_mut().zip(&self.warm_rates).zip(&levels) {
            *warmth += (level.abs().min(1.0) - *warmth) * rate;
        }
        self.mains += self.mains_step;
        if self.mains > std::f32::consts::TAU {
            self.mains -= std::f32::consts::TAU;
        }
    }

    #[inline(always)]
    pub fn effect(&mut self, channel: usize) -> Effect {
        let age = self.age;
        // The supply sags under load, and has a little mains on it when it's old.
        let hum = self.mains.sin();
        let supply = 1.0 - age * (0.55 * self.rail.min(1.0) + 0.02 * hum);

        let shaking = self.rail.min(1.0);
        let joint = &mut self.joints[channel];
        let mut joint_gain = 1.0;
        if joint.open_for > 0.0 {
            joint.open_for -= self.per_sample;
            joint_gain = JOINT_OPEN_GAIN;
        } else if age > 0.0 && self.rng.chance(age * age * (JOINT_RATE + JOINT_SHAKE_RATE * shaking * shaking) * self.per_sample) {
            joint.open_for = self.rng.exponential(JOINT_MEAN_S);
        }

        let scratch = self.scratch[channel];
        self.scratch[channel] *= self.scratch_decay;
        let dirt = if scratch > 1e-4 { age * scratch * self.rng.signed() * self.rng.unit().powi(4) } else { 0.0 };

        let warmth = self.warmth[channel];
        Effect {
            gain: supply * joint_gain,
            pull_hz: age * (PULL_HZ[channel] * self.rail.min(1.0) + DRIFT_HZ[channel] * warmth),
            warmth: warmth * age,
            dirt: dirt + 0.0015 * age * hum,
            imbalance: 0.3 * age,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RATE: f32 = 96_000.0;

    #[test]
    fn a_new_machine_has_nothing_wrong_with_it() {
        let mut wear = Wear::new(RATE);
        for _ in 0..RATE as usize {
            wear.load([0.9; CHANNELS]);
            let e = wear.effect(0);
            assert_eq!((e.gain, e.pull_hz, e.dirt, e.imbalance), (1.0, 0.0, 0.0, 0.0));
        }
    }

    #[test]
    fn load_sags_the_supply_and_pulls_the_oscillators() {
        let mut wear = Wear::new(RATE);
        wear.set_age(1.0);
        let idle = wear.effect(0);
        for _ in 0..(RATE as usize / 2) {
            wear.load([0.9; CHANNELS]);
        }
        let loaded = wear.effect(0);
        assert!(idle.pull_hz.abs() < 0.01);
        assert!(loaded.pull_hz.abs() > 1.0, "{}", loaded.pull_hz);
    }

    #[test]
    fn it_warms_up_slowly_and_each_channel_at_its_own_rate() {
        let mut wear = Wear::new(RATE);
        wear.set_age(1.0);
        for _ in 0..(RATE as usize * 10) {
            wear.load([0.8; CHANNELS]);
        }
        let warmth: Vec<f32> = (0..CHANNELS).map(|c| wear.effect(c).warmth).collect();
        assert!(warmth[0] > warmth[1] && warmth[1] > warmth[2] && warmth[3] > warmth[2]);
        assert!(warmth[0] < 0.5, "{}", warmth[0]);
    }

    #[test]
    fn joints_open_more_when_it_is_shaking_and_only_when_old() {
        let opens = |age: f32, level: f32| {
            let mut wear = Wear::new(RATE);
            wear.set_age(age);
            let (mut count, mut was) = (0, false);
            for _ in 0..(RATE as usize * 40) {
                wear.load([level; CHANNELS]);
                let open = wear.effect(0).gain < 0.2;
                count += (open && !was) as usize;
                was = open;
            }
            count
        };
        assert_eq!(opens(0.0, 0.9), 0);
        assert!(opens(1.0, 0.9) > opens(1.0, 0.01) * 3, "{} {}", opens(1.0, 0.9), opens(1.0, 0.01));
    }

    #[test]
    fn touching_a_worn_control_scratches() {
        let mut wear = Wear::new(RATE);
        wear.set_age(1.0);
        let quiet = (0..1_000).map(|_| wear.effect(1).dirt.abs()).fold(0.0, f32::max);
        wear.touched(1, 0.2);
        let loud = (0..1_000).map(|_| wear.effect(1).dirt.abs()).fold(0.0, f32::max);
        assert!(loud > quiet * 5.0, "{quiet} {loud}");
    }
}
