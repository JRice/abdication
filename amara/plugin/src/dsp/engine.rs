//! The whole chain: the speaker's tract frame by frame through the codec and the line,
//! out through the decoder's voices at the codec's own rate, and back up to the host's.

use super::codec::{encode, Frame, Line};
use super::filter::Cliff;
use super::limiter::Limiter;
use super::possession::Possession;
use super::speaker::{Mark, Speaker, SpeakerKnobs, Target};
use super::voice::{Settings, Voice};

const WORST_RATE: f32 = 6_000.0;
const BEST_RATE: f32 = 16_000.0;
const WORST_FRAME_S: f32 = 0.03;
const BEST_FRAME_S: f32 = 0.01;
/// Steps of the tract per frame: short sounds shouldn't fall between frames.
const SUBSTEPS: usize = 4;
const EXTRA_VOICE_LEVEL: f32 = 0.75;
/// The filter only gets rebuilt for a real change of rate.
const REBUILD_RATIO: f32 = 0.01;

#[derive(Clone, Copy)]
pub struct Controls {
    pub speaker: SpeakerKnobs,
    pub language: u32,
    pub strangeness: f32,
    pub possession: f32,
    pub extremity: f32,
    pub seized: bool,
    pub line: f32,
    /// 0 is the worst codec, 1 the best.
    pub fidelity: f32,
}

pub struct Engine {
    speaker: Speaker,
    line: Line,
    possession: Possession,
    voices: [Voice; 3],
    outputs: [Cliff; 2],
    limiter: Limiter,
    host_rate: f32,
    codec_rate: f32,
    cutoff_rate: f32,
    frame_samples: usize,
    until_frame: usize,
    until_filter_step: usize,
    previous: Frame,
    next: Frame,
    settings: Settings,
    frame_position: usize,
    /// Where we are between codec samples, and the two either side.
    phase: f32,
    held: [f32; 2],
    current: [f32; 2],
    language: (u32, f32),
    /// Waiting for the panel.
    marks: Vec<Mark>,
}

fn codec_rate(fidelity: f32) -> f32 {
    WORST_RATE * (BEST_RATE / WORST_RATE).powf(fidelity)
}

fn frame_seconds(fidelity: f32) -> f32 {
    WORST_FRAME_S * (BEST_FRAME_S / WORST_FRAME_S).powf(fidelity)
}

impl Engine {
    pub fn new(sample_rate: f32, language: (u32, f32)) -> Self {
        let rate = codec_rate(0.5);
        Self {
            speaker: Speaker::new(language.0, language.1),
            line: Line::new(),
            possession: Possession::new(),
            voices: [Voice::new(1), Voice::new(2), Voice::new(3)],
            outputs: [Self::band_edge(rate, sample_rate), Self::band_edge(rate, sample_rate)],
            limiter: Limiter::new(sample_rate),
            host_rate: sample_rate,
            codec_rate: rate,
            cutoff_rate: rate,
            frame_samples: 1,
            until_frame: 0,
            until_filter_step: 0,
            previous: Frame::SILENT,
            next: Frame::SILENT,
            settings: Settings::between(&Frame::SILENT, &Frame::SILENT, 0.0),
            frame_position: 0,
            phase: 0.0,
            held: [0.0; 2],
            current: [0.0; 2],
            language,
            marks: Vec::with_capacity(64),
        }
    }

    fn band_edge(rate: f32, host_rate: f32) -> Cliff {
        Cliff::new((0.46 * rate).min(0.45 * host_rate), host_rate)
    }

    pub fn reduction_db(&self) -> f32 {
        self.limiter.reduction_db()
    }

    pub fn possessed(&self) -> bool {
        self.possession.active().is_some()
    }

    /// The transcript since last asked. Keeps its storage.
    pub fn drain_marks(&mut self, mut each: impl FnMut(Mark)) {
        self.marks.drain(..).for_each(&mut each);
    }

    /// Once per block.
    pub fn set_controls(&mut self, controls: &Controls) {
        let wanted = (controls.language, controls.strangeness);
        if wanted != self.language {
            self.language = wanted;
            self.speaker.set_language(wanted.0, wanted.1);
        }
        self.possession.set_knobs(controls.possession, controls.extremity, controls.seized);
    }

    /// A note: says one syllable, starting with a new frame, so it's on time.
    pub fn say(&mut self, temper: f32, hz: f32, knobs: &SpeakerKnobs) {
        self.speaker.say(temper, hz, knobs);
        self.until_frame = 0;
        self.until_filter_step = 0;
    }

    /// The tract's next frame, what it comes out of the codec as, and what happens to it.
    fn next_frame(&mut self, controls: &Controls) {
        self.codec_rate = codec_rate(controls.fidelity);
        if (self.codec_rate / self.cutoff_rate - 1.0).abs() > REBUILD_RATIO {
            self.cutoff_rate = self.codec_rate;
            self.outputs = [Self::band_edge(self.codec_rate, self.host_rate), Self::band_edge(self.codec_rate, self.host_rate)];
        }
        let seconds = frame_seconds(controls.fidelity);
        self.frame_samples = ((seconds * self.codec_rate) as usize).max(1);

        self.possession.advance(seconds);
        let mut target = self.tract_over(seconds, &controls.speaker);
        self.possession.shape(&mut target);
        let packet = encode(&target, self.codec_rate);
        let damage = controls.line.max(self.possession.line_damage());
        let decoded = self.line.carry(packet, damage);
        self.previous = self.next;
        self.next = self.possession.after_decoding(decoded);
        self.frame_position = 0;
    }

    /// The tract through this frame: its loudest moment, so bursts aren't lost, and the
    /// rest as it stands at the end.
    fn tract_over(&mut self, seconds: f32, knobs: &SpeakerKnobs) -> Target {
        let dt = seconds / SUBSTEPS as f32;
        let mut loudest = 0.0f32;
        for _ in 0..SUBSTEPS {
            self.speaker.step(dt, knobs);
            for mark in self.speaker.take_marks() {
                if self.marks.len() < self.marks.capacity() {
                    self.marks.push(*mark);
                }
            }
            loudest = loudest.max(self.speaker.target(knobs).gain);
        }
        Target { gain: loudest, ..self.speaker.target(knobs) }
    }

    /// One sample at the codec's rate.
    #[inline(always)]
    fn codec_sample(&mut self, controls: &Controls) -> [f32; 2] {
        if self.until_frame == 0 {
            self.next_frame(controls);
            self.until_frame = self.frame_samples;
        }
        self.until_frame -= 1;
        if self.until_filter_step == 0 {
            let along = self.frame_position as f32 / self.frame_samples as f32;
            self.settings = Settings::between(&self.previous, &self.next, along);
            self.until_filter_step = Settings::step();
        }
        self.until_filter_step -= 1;
        self.frame_position += 1;

        let rate = self.codec_rate;
        let [main, left, right] = &mut self.voices;
        let voice = main.process(&self.settings, 1.0, rate);
        match self.possession.legion() {
            Some([to_left, to_right]) => [
                voice + EXTRA_VOICE_LEVEL * left.process(&self.settings, to_left, rate),
                voice + EXTRA_VOICE_LEVEL * right.process(&self.settings, to_right, rate),
            ],
            None => [voice; 2],
        }
    }

    #[inline]
    pub fn process(&mut self, controls: &Controls, level: f32) -> [f32; 2] {
        self.phase += self.codec_rate / self.host_rate;
        while self.phase >= 1.0 {
            self.phase -= 1.0;
            self.held = self.current;
            self.current = self.codec_sample(controls);
        }
        let phase = self.phase;
        let output = [0, 1].map(|c| {
            let raised = self.held[c] + (self.current[c] - self.held[c]) * phase;
            self.outputs[c].process(raised) * level
        });
        self.limiter.process(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SR: f32 = 48_000.0;

    fn controls() -> Controls {
        Controls {
            speaker: SpeakerKnobs { rate: 5.0, sparsity: 0.2, temper: 0.5, swing: 0.5, breath: 0.1, tract: 0.85, pitch_hz: 90.0, per_note: false },
            language: 13,
            strangeness: 0.4,
            possession: 0.0,
            extremity: 0.6,
            seized: false,
            line: 0.0,
            fidelity: 0.5,
        }
    }

    fn run(c: &Controls, seconds: f32) -> Vec<[f32; 2]> {
        let mut engine = Engine::new(SR, (c.language, c.strangeness));
        engine.set_controls(c);
        (0..(seconds * SR) as usize).map(|_| engine.process(c, 1.0)).collect()
    }

    fn rms(signal: &[[f32; 2]]) -> f32 {
        (signal.iter().map(|f| f[0] * f[0]).sum::<f32>() / signal.len().max(1) as f32).sqrt()
    }

    #[test]
    fn it_talks_with_no_input_at_a_sensible_level() {
        let out = run(&controls(), 20.0);
        let level = rms(&out);
        assert!(level > 0.02 && level < 0.4, "{level}");
    }

    fn per_note_controls() -> Controls {
        let c = controls();
        Controls { speaker: SpeakerKnobs { per_note: true, ..c.speaker }, ..c }
    }

    #[test]
    fn per_note_is_silent_without_notes() {
        let out = run(&per_note_controls(), 3.0);
        assert!(rms(&out[24_000..]) < 1e-4);
    }

    #[test]
    fn a_note_sounds_soon_and_then_stops_by_itself() {
        let c = per_note_controls();
        let mut engine = Engine::new(SR, (c.language, c.strangeness));
        engine.set_controls(&c);
        for _ in 0..2_400 {
            engine.process(&c, 1.0);
        }
        engine.say(0.8, 110.0, &c.speaker);
        let out: Vec<[f32; 2]> = (0..96_000).map(|_| engine.process(&c, 1.0)).collect();
        // It might open with a closure, so give it a stop's length to arrive.
        let first = out.iter().position(|f| f[0].abs() > 0.01).expect("it spoke");
        assert!(first < 12_000, "{first}");
        assert!(rms(&out[first..first + 12_000]) > 0.01);
        assert!(rms(&out[72_000..]) < 1e-3, "{}", rms(&out[72_000..]));
    }

    #[test]
    fn nothing_gets_past_the_limiter_or_goes_bad_at_any_extreme() {
        for language in [0, 13, 666] {
            for (rate, sparsity, temper, line, fidelity, tract, pitch) in
                [(14.0, 0.0, 1.0, 1.0, 0.0, 0.55, 400.0), (1.5, 1.0, 0.0, 0.5, 1.0, 1.45, 40.0), (5.0, 0.0, 1.0, 1.0, 1.0, 1.45, 400.0)]
            {
                let c = Controls {
                    speaker: SpeakerKnobs { rate, sparsity, temper, swing: 1.0, breath: 1.0, tract, pitch_hz: pitch, per_note: false },
                    language,
                    strangeness: 1.0,
                    possession: 1.0,
                    extremity: 1.0,
                    seized: true,
                    line,
                    fidelity,
                };
                for frame in run(&c, 8.0) {
                    assert!(frame[0].is_finite() && frame[1].is_finite());
                    assert!(frame[0].abs() <= 1.0 && frame[1].abs() <= 1.0);
                }
            }
        }
    }

    #[test]
    fn legion_is_the_only_thing_that_opens_up_the_stereo() {
        let plain = run(&controls(), 5.0);
        assert!(plain.iter().all(|f| f[0] == f[1]));
    }

    #[test]
    fn changing_language_on_the_fly_is_safe() {
        let c = controls();
        let mut engine = Engine::new(SR, (1, 0.4));
        engine.set_controls(&c);
        for block in 0..200 {
            engine.set_controls(&Controls { language: block, ..c });
            for _ in 0..512 {
                assert!(engine.process(&c, 1.0)[0].is_finite());
            }
        }
    }
}
