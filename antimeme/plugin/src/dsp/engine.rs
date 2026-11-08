//! The whole effect: Strength, two voices, Agitation, Blend, Level, and the
//! resampling around the loop.

use super::agitation::Agitation;
use super::filters::Delay;
use super::limiter::Limiter;
use super::resample::{Oversampler, ROUND_TRIP_LATENCY};
use super::voice::{Frame, Voice};

/// Below this host rate the loop runs at twice the rate.
const OVERSAMPLE_BELOW_HZ: f32 = 80_000.0;

/// Strength at 12:00 measured +7 dB; its law is assumed square from there.
const STRENGTH_MAX_DB: f32 = 30.0;

/// Spread at full: the right channel's clock runs this much faster.
const SPREAD_CLOCK: f32 = 0.03;

/// One host sample's control values, each 0..1 except where noted.
#[derive(Clone, Copy, Default)]
pub struct Controls {
    pub time: f32,
    pub decay: f32,
    pub filter: f32,
    pub absorb: f32,
    pub blend: f32,
    pub strength: f32,
    /// Linear gain.
    pub level: f32,
    pub spread: f32,
    pub whine: f32,
    pub agitation_speed: f32,
    pub agitation_angle: f32,
    /// Sends, -1..1.
    pub agitation_to_time: f32,
    pub agitation_to_filter: f32,
    pub agitation_to_decay: f32,
    pub agitation_to_absorb: f32,
    pub agitation_to_whine: f32,
    pub interference_to_time: f32,
    pub interference_to_filter: f32,
    pub interference_to_decay: f32,
    pub interference_to_absorb: f32,
    pub interference_to_whine: f32,
}

struct Channel {
    voice: Voice,
    resampler: Oversampler,
    dry: Delay,
}

impl Channel {
    fn new(sample_rate: f32, oversampled: bool, seed: u32) -> Self {
        let factor = if oversampled { 2.0 } else { 1.0 };
        let latency = if oversampled { ROUND_TRIP_LATENCY } else { 0 };
        Self { voice: Voice::new(sample_rate * factor, seed), resampler: Oversampler::new(), dry: Delay::new(latency) }
    }
}

pub struct Engine {
    channels: [Channel; 2],
    agitation: Agitation,
    limiter: Limiter,
    oversampled: bool,
}

/// Strength: gain into a soft ceiling, transparent at unity for normal levels.
#[inline(always)]
fn strength(x: f32, amount: f32) -> f32 {
    if amount <= 0.0 {
        return x;
    }
    let gain = 10f32.powf(amount * amount * STRENGTH_MAX_DB / 20.0);
    let ceiling = 2.0;
    let driven = x * gain / ceiling;
    ceiling * driven / (1.0 + driven * driven).sqrt()
}

impl Engine {
    pub fn new(sample_rate: f32) -> Self {
        let oversampled = sample_rate < OVERSAMPLE_BELOW_HZ;
        let internal = sample_rate * if oversampled { 2.0 } else { 1.0 };
        Self {
            channels: [Channel::new(sample_rate, oversampled, 1), Channel::new(sample_rate, oversampled, 2)],
            agitation: Agitation::new(internal),
            limiter: Limiter::new(sample_rate),
            oversampled,
        }
    }

    /// How hard the limiter is working right now, in dB.
    pub fn reduction_db(&self) -> f32 {
        self.limiter.reduction_db()
    }

    /// Where Agitation is in its cycle, 0..1.
    pub fn agitation_phase(&self) -> f32 {
        self.agitation.phase()
    }

    /// The first channel's Interference, -1..1.
    pub fn interference(&self) -> f32 {
        self.channels[0].voice.interference()
    }

    pub fn latency(&self) -> u32 {
        if self.oversampled {
            ROUND_TRIP_LATENCY as u32
        } else {
            0
        }
    }

    /// One internal sample's values: this also advances Agitation.
    fn frame(&mut self, c: &Controls) -> Frame {
        let agitation = self.agitation.process(c.agitation_speed, c.agitation_angle);
        Frame {
            time: c.time,
            decay: c.decay,
            filter: c.filter,
            absorb: c.absorb,
            whine: c.whine,
            clock_ratio: 1.0,
            agitation_time: agitation * c.agitation_to_time,
            agitation_filter: agitation * c.agitation_to_filter,
            agitation_decay: agitation * c.agitation_to_decay,
            agitation_absorb: agitation * c.agitation_to_absorb,
            agitation_whine: agitation * c.agitation_to_whine,
            interference_time: c.interference_to_time,
            interference_filter: c.interference_to_filter,
            interference_decay: c.interference_to_decay,
            interference_absorb: c.interference_to_absorb,
            interference_whine: c.interference_to_whine,
        }
    }

    /// Run the voices for one internal sample. With `stereo` off, only the first
    /// runs: that is the Strega's one mono loop, and half the work.
    #[inline(always)]
    fn step(&mut self, feeds: [f32; 2], c: &Controls, stereo: bool) -> [f32; 2] {
        let frame = self.frame(c);
        let left = self.channels[0].voice.process(feeds[0], &frame);
        if !stereo {
            return [left, left];
        }
        let right_frame = Frame { clock_ratio: 1.0 + SPREAD_CLOCK * c.spread, ..frame };
        [left, self.channels[1].voice.process(feeds[1], &right_frame)]
    }

    pub fn process(&mut self, input: [f32; 2], c: &Controls) -> [f32; 2] {
        let driven = input.map(|x| strength(x, c.strength));
        let mid = 0.5 * (driven[0] + driven[1]);
        let side = 0.5 * (driven[0] - driven[1]) * c.spread;
        let feeds = [mid + side, mid - side];

        let stereo = c.spread > 0.0;

        let wet = if self.oversampled {
            let [up_left, up_right] = [self.channels[0].resampler.up(feeds[0]), self.channels[1].resampler.up(feeds[1])];
            let first = self.step([up_left[0], up_right[0]], c, stereo);
            let second = self.step([up_left[1], up_right[1]], c, stereo);
            [self.channels[0].resampler.down([first[0], second[0]]), self.channels[1].resampler.down([first[1], second[1]])]
        } else {
            self.step(feeds, c, stereo)
        };

        let out = std::array::from_fn(|i| {
            let dry = self.channels[i].dry.process(driven[i]);
            (dry + (wet[i] - dry) * c.blend) * c.level
        });
        self.limiter.process(out)
    }
}
