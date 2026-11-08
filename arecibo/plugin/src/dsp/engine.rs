//! The outside of the machine: the host's rate down to the network's, a dry path around
//! it and a limiter at the end.

use super::limiter::Limiter;
use super::network::Network;
use super::resample::Oversampler;
use super::station::Tuning;
use super::wear::CHANNELS;

/// Below this host rate, the network runs at twice it: a saturated loop aliases badly.
const OVERSAMPLE_BELOW: f32 = 60_000.0;
const SEED: u32 = 1963;
const OUTPUT_TRIM: f32 = 0.56;
/// How much of the inner stations reaches the near and far sides.
const NEAR: f32 = 0.9;
const FAR: f32 = 0.35;

#[derive(Clone, Copy)]
pub struct StripKnobs {
    pub shift_hz: f32,
    pub peak_hz: f32,
    pub gain: f32,
}

#[derive(Clone, Copy)]
pub struct Controls {
    pub strips: [StripKnobs; CHANNELS],
    pub feedback: f32,
    pub cross: f32,
    pub sky: f32,
    pub width_hz: f32,
    pub distance_s: f32,
    pub agc_release_s: f32,
    pub weather: f32,
    pub age: f32,
    pub input: f32,
    pub side_input: f32,
    pub dry: f32,
    pub level: f32,
}

struct Resamplers {
    up: [Oversampler; CHANNELS],
    down: [Oversampler; 2],
}

pub struct Engine {
    network: Network,
    /// Up for each input, down for each output.
    resamplers: Option<Resamplers>,
    limiter: Limiter,
    last_strips: [StripKnobs; CHANNELS],
}

impl Engine {
    pub fn new(sample_rate: f32) -> Self {
        let oversample = sample_rate < OVERSAMPLE_BELOW;
        let rate = if oversample { sample_rate * 2.0 } else { sample_rate };
        Self {
            network: Network::new(rate, SEED),
            resamplers: oversample.then(|| Resamplers { up: std::array::from_fn(|_| Oversampler::new()), down: std::array::from_fn(|_| Oversampler::new()) }),
            limiter: Limiter::new(sample_rate),
            last_strips: [StripKnobs { shift_hz: 0.0, peak_hz: 1.0, gain: 0.0 }; CHANNELS],
        }
    }

    pub fn latency(&self) -> u32 {
        if self.resamplers.is_some() { super::resample::ROUND_TRIP_LATENCY as u32 } else { 0 }
    }

    pub fn reduction_db(&self) -> f32 {
        self.limiter.reduction_db()
    }

    pub fn agc_gains(&self) -> [f32; CHANNELS] {
        self.network.agc_gains()
    }

    /// Once per block.
    pub fn set_controls(&mut self, c: &Controls) {
        let tunings = c.strips.map(|strip| Tuning {
            shift_hz: strip.shift_hz,
            peak_hz: strip.peak_hz,
            gain: strip.gain,
            sky: c.sky,
            width_hz: c.width_hz,
            agc_release_s: c.agc_release_s,
            weather: c.weather,
            distance_s: c.distance_s,
        });
        self.network.set(tunings, c.feedback, c.cross, c.age);
        // A worn pot crackles when it's moved.
        for i in 0..CHANNELS {
            let (was, now) = (self.last_strips[i], c.strips[i]);
            let moved = (now.peak_hz / was.peak_hz.max(1.0)).ln().abs() + (now.gain - was.gain).abs();
            if moved > 1e-4 && was.peak_hz > 1.0 {
                self.network.touched(i, moved);
            }
        }
        self.last_strips = c.strips;
    }

    /// A and C lean left, B and D right, with C and D nearer the middle.
    fn spread_out(outputs: [f32; CHANNELS]) -> [f32; 2] {
        let [a, b, c, d] = outputs;
        [OUTPUT_TRIM * (a + NEAR * c + FAR * d), OUTPUT_TRIM * (b + NEAR * d + FAR * c)]
    }

    /// `input` and `side` each go to their own two stations, nothing shared.
    #[inline]
    pub fn process(&mut self, input: [f32; 2], side: [f32; 2], c: &Controls) -> [f32; 2] {
        let scaled = [input[0] * c.input, input[1] * c.input, side[0] * c.side_input, side[1] * c.side_input];
        let wet = if let Some(resamplers) = self.resamplers.as_mut() {
            let mut raised = [[0.0; 2]; CHANNELS];
            for (slot, (up, x)) in raised.iter_mut().zip(resamplers.up.iter_mut().zip(scaled)) {
                *slot = up.up(x);
            }
            let mut outs = [[0.0; 2]; 2];
            for (n, out) in outs.iter_mut().enumerate() {
                *out = Self::spread_out(self.network.process(std::array::from_fn(|ch| raised[ch][n])));
            }
            [resamplers.down[0].down([outs[0][0], outs[1][0]]), resamplers.down[1].down([outs[0][1], outs[1][1]])]
        } else {
            Self::spread_out(self.network.process(scaled))
        };
        let out = [0, 1].map(|ch| (wet[ch] + input[ch] * c.dry) * c.level);
        self.limiter.process(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SR: f32 = 48_000.0;

    fn controls() -> Controls {
        Controls {
            strips: [
                StripKnobs { shift_hz: 1.3, peak_hz: 420.0, gain: 1.2 },
                StripKnobs { shift_hz: -0.7, peak_hz: 1_300.0, gain: 1.2 },
                StripKnobs { shift_hz: 3.1, peak_hz: 2_900.0, gain: 1.2 },
                StripKnobs { shift_hz: -1.9, peak_hz: 680.0, gain: 1.2 },
            ],
            feedback: 0.9,
            cross: 0.35,
            sky: 0.7,
            width_hz: 2_800.0,
            distance_s: 0.028,
            agc_release_s: 0.7,
            weather: 0.35,
            age: 0.4,
            input: 1.0,
            side_input: 1.0,
            dry: 0.0,
            level: 1.0,
        }
    }

    fn run(c: &Controls, seconds: f32, mut input: impl FnMut(usize) -> [f32; 2]) -> Vec<[f32; 2]> {
        let mut engine = Engine::new(SR);
        engine.set_controls(c);
        (0..(seconds * SR) as usize).map(|n| engine.process(input(n), [0.0; 2], c)).collect()
    }

    fn rms(signal: &[[f32; 2]]) -> f32 {
        (signal.iter().map(|f| f[0] * f[0] + f[1] * f[1]).sum::<f32>() / (2.0 * signal.len().max(1) as f32)).sqrt()
    }

    fn silence(_: usize) -> [f32; 2] {
        [0.0; 2]
    }

    /// How much the level moves from one half second to the next, in dB.
    fn movement(signal: &[[f32; 2]]) -> f32 {
        let levels: Vec<f32> = signal.chunks(24_000).map(|c| 20.0 * rms(c).max(1e-6).log10()).collect();
        levels.windows(2).map(|w| (w[1] - w[0]).abs()).sum::<f32>() / (levels.len() - 1) as f32
    }

    #[test]
    fn it_starts_by_itself_with_nothing_going_in() {
        let out = run(&controls(), 20.0, silence);
        let level = rms(&out[240_000..]);
        assert!(level > 0.01, "{level}");
    }

    #[test]
    fn it_does_not_settle_into_a_drone() {
        // The hang AGC, the fading and the static should keep it breathing indefinitely.
        let out = run(&controls(), 120.0, silence);
        let (early, late) = (movement(&out[..2_880_000]), movement(&out[2_880_000..]));
        assert!(early > 2.0 && late > 2.0, "{early} {late}");
    }

    #[test]
    fn feedback_brings_it_alive() {
        let level = |feedback: f32, sky: f32| rms(&run(&Controls { feedback, sky, ..controls() }, 15.0, silence)[240_000..]);
        assert!(level(0.0, 0.0) < 1e-3, "{}", level(0.0, 0.0));
        assert!(level(1.4, 0.0) > 0.03, "{}", level(1.4, 0.0));
        assert!(level(1.2, 0.7) > level(0.1, 0.7) * 1.5, "{} {}", level(1.2, 0.7), level(0.1, 0.7));
    }

    #[test]
    fn with_no_feedback_the_agc_hauls_up_the_static_only_on_the_sky_path() {
        let c = Controls { feedback: 0.0, weather: 0.5, ..controls() };
        let (sky, wire) = (rms(&run(&Controls { sky: 1.0, ..c }, 10.0, silence)), rms(&run(&Controls { sky: 0.0, ..c }, 10.0, silence)));
        assert!(sky > wire * 10.0, "{sky} {wire}");
    }

    #[test]
    fn the_agc_evens_out_strong_and_weak_signals_coming_through() {
        let c = Controls { feedback: 0.0, sky: 1.0, weather: 0.0, age: 0.0, ..controls() };
        let tone = |amplitude: f32| move |n: usize| [amplitude * (n as f32 * 0.1).sin(); 2];
        let quiet = rms(&run(&c, 4.0, tone(0.02))[96_000..]);
        let loud = rms(&run(&c, 4.0, tone(0.5))[96_000..]);
        // 28 dB apart going in. Coming out, much less: the strip only limits the top.
        assert!(20.0 * (loud / quiet).log10() < 12.0, "{quiet} {loud}");
    }

    #[test]
    fn input_arrives_a_distance_later() {
        let c = Controls { feedback: 0.0, sky: 1.0, weather: 0.0, age: 0.0, distance_s: 0.05, ..controls() };
        let out = run(&c, 0.4, |n| if n >= 4_800 { [0.5 * (n as f32 * 0.15).sin(); 2] } else { [0.0; 2] });
        // Static-free and hiss aside, nothing should come out until it's had time to travel.
        let first = out.iter().position(|f| f[0].abs() > 0.05).unwrap();
        assert!((first as f32 - (4_800.0 + 0.05 * SR)).abs() < 400.0, "{first}");
    }

    #[test]
    fn nothing_escapes_at_any_extreme() {
        let mut rng = super::super::random::Rng::new(77);
        for _ in 0..40 {
            let strip = |rng: &mut super::super::random::Rng| StripKnobs {
                shift_hz: rng.range(-60.0, 60.0),
                peak_hz: rng.range(60.0, 9_000.0),
                gain: rng.range(0.25, 4.0),
            };
            let c = Controls {
                strips: std::array::from_fn(|_| strip(&mut rng)),
                feedback: rng.range(0.0, 1.6),
                cross: rng.unit(),
                sky: rng.unit(),
                width_hz: rng.range(300.0, 12_000.0),
                distance_s: rng.range(0.001, 0.16),
                agc_release_s: rng.range(0.05, 3.0),
                weather: rng.unit(),
                age: rng.unit(),
                input: 4.0,
                side_input: 4.0,
                dry: rng.unit(),
                level: 4.0,
            };
            let mut engine = Engine::new(SR);
            engine.set_controls(&c);
            for n in 0..(1.5 * SR) as usize {
                let x = (n as f32 * 0.31).sin();
                let frame = engine.process([x; 2], [-x; 2], &c);
                assert!(frame[0].is_finite() && frame[1].is_finite());
                assert!(frame[0].abs() <= 1.0 && frame[1].abs() <= 1.0);
            }
        }
    }

    #[test]
    fn dry_goes_straight_through() {
        let c = Controls { feedback: 0.0, sky: 0.0, dry: 1.0, input: 0.0, level: 1.0, ..controls() };
        for (n, frame) in run(&c, 0.05, |n| [0.25 * (n as f32 * 0.1).sin(); 2]).iter().enumerate() {
            assert!((frame[0] - 0.25 * (n as f32 * 0.1).sin()).abs() < 0.02, "{n}");
        }
    }

    /// The wire only, nothing coming back, so each input can be followed through.
    fn straight_through() -> Controls {
        Controls { feedback: 0.0, sky: 0.0, age: 0.0, weather: 0.0, ..controls() }
    }

    fn levels(c: &Controls, input: [f32; 2], side: [f32; 2]) -> [f32; 2] {
        let mut engine = Engine::new(SR);
        engine.set_controls(c);
        let out: Vec<[f32; 2]> = (0..24_000)
            .map(|n| {
                let tone = (n as f32 * 0.1).sin();
                engine.process(input.map(|a| a * tone), side.map(|a| a * tone), c)
            })
            .collect();
        let rms_of = |ch: usize| (out[4_800..].iter().map(|f| f[ch] * f[ch]).sum::<f32>() / 19_200.0).sqrt();
        [rms_of(0), rms_of(1)]
    }

    #[test]
    fn the_main_input_feeds_only_its_own_stations() {
        let [left, right] = levels(&straight_through(), [0.3, 0.0], [0.0; 2]);
        assert!(left > 0.02 && right < left * 0.01, "{left} {right}");
    }

    #[test]
    fn the_sidechain_feeds_its_own_stations_nearer_the_middle() {
        let [left, right] = levels(&straight_through(), [0.0; 2], [0.3, 0.0]);
        assert!(left > 0.01, "{left}");
        // Station C reaches both sides, unlike A.
        assert!(right > left * 0.3 && right < left, "{left} {right}");
    }

    #[test]
    fn side_input_is_its_own_knob() {
        let shut = Controls { side_input: 0.0, ..straight_through() };
        let [left, _] = levels(&shut, [0.0; 2], [0.5, 0.5]);
        assert!(left < 1e-3, "{left}");
        let [main, _] = levels(&shut, [0.3, 0.0], [0.0; 2]);
        assert!(main > 0.02, "{main}");
    }

    #[test]
    fn something_on_the_sidechain_changes_what_the_running_loop_does() {
        let c = controls();
        let render = |side: f32| {
            let mut engine = Engine::new(SR);
            engine.set_controls(&c);
            (0..(20.0 * SR) as usize).map(|n| engine.process([0.0; 2], [side * (n as f32 * 0.08).sin(); 2], &c)).collect::<Vec<_>>()
        };
        let (without, with) = (render(0.0), render(0.8));
        let difference: Vec<[f32; 2]> = without.iter().zip(&with).map(|(a, b)| [a[0] - b[0], a[1] - b[1]]).collect();
        assert!(rms(&difference[240_000..]) > rms(&without[240_000..]) * 0.5);
    }

    /// Sum of squares across both outputs over a stretch of a repeating two-second cycle.
    fn level_through_cycle(c: &Controls, hit: impl Fn(usize) -> ([f32; 2], [f32; 2]), from_s: f32, to_s: f32) -> f32 {
        const CYCLE: usize = 96_000;
        let mut engine = Engine::new(SR);
        engine.set_controls(c);
        let (mut sum, mut count) = (0.0, 0);
        for n in 0..(CYCLE * 25) {
            let (input, side) = hit(n % CYCLE);
            let frame = engine.process(input, side, c);
            let at = (n % CYCLE) as f32 / SR;
            if n > CYCLE * 5 && at >= from_s && at < to_s {
                sum += frame[0] * frame[0] + frame[1] * frame[1];
                count += 1;
            }
        }
        10.0 * (sum / count as f32).max(1e-12).log10()
    }

    /// A short, hard burst at the start of each cycle.
    fn burst(n: usize) -> [f32; 2] {
        if n < 1_200 { [0.9 * (n as f32 * 0.13).sin(); 2] } else { [0.0; 2] }
    }

    #[test]
    fn a_hit_on_one_input_pair_drops_the_gain_of_its_own_two_receivers() {
        let gains_after = |main: bool| {
            let c = controls();
            let mut engine = Engine::new(SR);
            engine.set_controls(&c);
            for _ in 0..240_000 {
                engine.process([0.0; 2], [0.0; 2], &c);
            }
            for n in 0..1_200 {
                let hit = burst(n);
                let (input, side) = if main { (hit, [0.0; 2]) } else { ([0.0; 2], hit) };
                engine.process(input, side, &c);
            }
            engine.agc_gains()
        };
        let main = gains_after(true);
        assert!(main[0] < main[2] * 0.5 && main[1] < main[3] * 0.5, "{main:?}");
        let side = gains_after(false);
        assert!(side[2] < side[0] * 0.5 && side[3] < side[1] * 0.5, "{side:?}");
    }

    #[test]
    fn hits_on_everything_at_once_silence_it_until_it_recovers() {
        let c = controls();
        let hit = |n: usize| (burst(n), burst(n));
        let after = level_through_cycle(&c, hit, 0.2, 0.6);
        let settled = level_through_cycle(&c, hit, 1.4, 1.9);
        assert!(settled - after > 15.0, "{after} {settled}");
    }

    #[test]
    fn something_quiet_only_gets_pulled_in() {
        let c = controls();
        let gentle = |n: usize| -> ([f32; 2], [f32; 2]) { (burst(n).map(|x| x * 0.02), [0.0; 2]) };
        let after = level_through_cycle(&c, gentle, 0.1, 0.5);
        let settled = level_through_cycle(&c, gentle, 1.4, 1.9);
        assert!((settled - after).abs() < 3.0, "{after} {settled}");
    }
}
