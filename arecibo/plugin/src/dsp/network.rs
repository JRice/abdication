//! The patch: four stations whose outputs feed each other's inputs, as on a mixer with
//! its outputs patched back into its inputs. Nothing has to go in for something to come out.

use super::filters::rate_for;
use super::random::Rng;
use super::station::{Station, Tuning};
use super::wear::{Wear, CHANNELS};

const RETUNE_EVERY: usize = 32;
/// Who feeds whom with what polarity. If everything were in phase, it would all just agree.
const SIGNS: [[f32; CHANNELS]; CHANNELS] = [[1.0, -1.0, 1.0, 1.0], [1.0, 1.0, -1.0, 1.0], [-1.0, 1.0, 1.0, -1.0], [1.0, -1.0, -1.0, 1.0]];
/// What each other channel gets of Cross, so the total stays about what it would be with
/// fewer of them.
const CROSS_SHARE: f32 = 0.47;
const CROSSTALK: f32 = 0.08;
const OUTPUT_SMOOTH_S: f32 = 0.002;
/// Long delayed echoes: a real and still-unexplained thing, where a station hears its own
/// signal seconds later. A rough, low-rate memory of what went out.
const ECHO_DECIMATION: usize = 4;
const ECHO_MEMORY_S: f32 = 30.0;
const ECHO_DELAY_S: (f32, f32) = (2.0, 28.0);
const ECHO_LENGTH_S: (f32, f32) = (0.4, 2.5);
const ECHOES_PER_S: f32 = 0.07;
const ECHO_LEVEL: f32 = 0.4;

/// Rarely, and only in bad weather, what the machine said a while ago comes back.
struct Echoes {
    memory: Vec<f32>,
    head: usize,
    until_write: usize,
    sum: f32,
    active: Option<Echo>,
    per_sample: f32,
    rate: f32,
    rng: Rng,
}

#[derive(Clone, Copy)]
struct Echo {
    behind: f32,
    elapsed: f32,
    length: f32,
    channel: usize,
}

impl Echoes {
    fn new(rate: f32) -> Self {
        let size = (ECHO_MEMORY_S * rate / ECHO_DECIMATION as f32) as usize + 2;
        Self { memory: vec![0.0; size], head: 0, until_write: 0, sum: 0.0, active: None, per_sample: 1.0 / rate, rate, rng: Rng::new(0x1927) }
    }

    /// Remembers this sample, and says what's coming back and to which channel.
    #[inline(always)]
    fn next(&mut self, level: f32, weather: f32) -> Option<(usize, f32)> {
        self.sum += level;
        if self.until_write == 0 {
            self.until_write = ECHO_DECIMATION;
            self.head = (self.head + 1) % self.memory.len();
            self.memory[self.head] = self.sum / ECHO_DECIMATION as f32;
            self.sum = 0.0;
        }
        self.until_write -= 1;
        if self.active.is_none() && self.rng.chance(ECHOES_PER_S * weather * weather * self.per_sample) {
            self.active = Some(Echo {
                behind: self.rng.range(ECHO_DELAY_S.0, ECHO_DELAY_S.1) * self.rate / ECHO_DECIMATION as f32,
                elapsed: 0.0,
                length: self.rng.range(ECHO_LENGTH_S.0, ECHO_LENGTH_S.1),
                channel: self.rng.below(CHANNELS),
            });
        }
        let echo = self.active.as_mut()?;
        echo.elapsed += self.per_sample;
        let progress = echo.elapsed / echo.length;
        let channel = echo.channel;
        let at = (self.head + self.memory.len() - echo.behind as usize) % self.memory.len();
        let out = self.memory[at] * ECHO_LEVEL * (std::f32::consts::PI * progress.min(1.0)).sin();
        if progress >= 1.0 {
            self.active = None;
        }
        Some((channel, out))
    }
}

pub struct Network {
    stations: [Station; CHANNELS],
    wear: Wear,
    outputs: [f32; CHANNELS],
    feedback: f32,
    cross: f32,
    age: f32,
    tunings: [Tuning; CHANNELS],
    until_retune: usize,
    smooth: f32,
    smoothed_levels: [f32; CHANNELS],
    echoes: Echoes,
}

impl Network {
    pub fn new(rate: f32, seed: u32) -> Self {
        let tuning = Tuning { shift_hz: 0.0, peak_hz: 1_000.0, gain: 1.0, sky: 0.5, width_hz: 2_800.0, agc_release_s: 0.5, weather: 0.3, distance_s: 0.02 };
        Self {
            stations: std::array::from_fn(|i| Station::new(rate, seed.wrapping_add(100 * i as u32))),
            wear: Wear::new(rate),
            outputs: [0.0; CHANNELS],
            feedback: 0.0,
            cross: 0.0,
            age: 0.0,
            tunings: [tuning; CHANNELS],
            until_retune: 0,
            smooth: rate_for(OUTPUT_SMOOTH_S, rate),
            smoothed_levels: [0.0; CHANNELS],
            echoes: Echoes::new(rate),
        }
    }

    pub fn set(&mut self, tunings: [Tuning; CHANNELS], feedback: f32, cross: f32, age: f32) {
        self.tunings = tunings;
        self.feedback = feedback;
        self.cross = cross;
        self.age = age;
        self.wear.set_age(age);
    }

    pub fn touched(&mut self, channel: usize, amount: f32) {
        self.wear.touched(channel, amount);
    }

    /// How much gain each receiver's using: high means it's hearing little.
    pub fn agc_gains(&self) -> [f32; CHANNELS] {
        std::array::from_fn(|i| self.stations[i].agc_gain)
    }

    #[inline(always)]
    fn return_to(&self, channel: usize) -> f32 {
        let (self_share, other_share) = (1.0 - self.cross, self.cross * CROSS_SHARE + CROSSTALK * self.age);
        (0..CHANNELS)
            .map(|from| {
                let share = if from == channel { self_share } else { other_share };
                SIGNS[channel][from] * share * self.outputs[from]
            })
            .sum::<f32>()
            * self.feedback
    }

    /// One sample, with this much arriving from outside on each channel.
    #[inline(always)]
    pub fn process(&mut self, input: [f32; CHANNELS]) -> [f32; CHANNELS] {
        if self.until_retune == 0 {
            self.until_retune = RETUNE_EVERY;
            for i in 0..CHANNELS {
                let warmth = self.wear.effect(i).warmth;
                self.stations[i].retune(&self.tunings[i], warmth);
            }
        }
        self.until_retune -= 1;

        let said = self.outputs.iter().sum::<f32>();
        let echo = self.echoes.next(said, self.tunings[0].weather);
        let mut next = [0.0; CHANNELS];
        for i in 0..CHANNELS {
            let coming_back = echo.filter(|(channel, _)| *channel == i).map_or(0.0, |(_, level)| level);
            let looped = self.return_to(i) + coming_back;
            let effect = self.wear.effect(i);
            let tuning = self.tunings[i];
            next[i] = self.stations[i].process(looped, input[i], &tuning, &effect);
            if !next[i].is_finite() {
                next[i] = 0.0;
            }
            self.smoothed_levels[i] += (next[i].abs() - self.smoothed_levels[i]) * self.smooth;
        }
        self.outputs = next;
        self.wear.load(self.smoothed_levels);
        next
    }
}
