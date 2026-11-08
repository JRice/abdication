//! Amara: an instrument that talks to itself in an invented language, over a bad line,
//! as a CLAP and VST3 plugin. `dsp` is the sound; this file is the wrapper.

pub mod dsp;
mod editor;
mod params;
mod telemetry;

use std::num::NonZeroU32;
use std::sync::Arc;

use nih_plug::prelude::*;

use dsp::engine::Engine;
use params::AmaraParams;
use telemetry::Telemetry;

/// Notes are optional: with one held, it babbles around that pitch.
#[derive(Default)]
struct Notes {
    held: [u8; 16],
    count: usize,
}

impl Notes {
    fn press(&mut self, note: u8) {
        self.release(note);
        if self.count < self.held.len() {
            self.held[self.count] = note;
            self.count += 1;
        }
    }

    fn release(&mut self, note: u8) {
        if let Some(i) = self.held[..self.count].iter().position(|n| *n == note) {
            self.held.copy_within(i + 1..self.count, i);
            self.count -= 1;
        }
    }

    /// The pitch of the note pressed last.
    fn hz(&self) -> Option<f32> {
        self.count.checked_sub(1).map(|last| util::midi_note_to_freq(self.held[last]))
    }
}

pub struct Amara {
    params: Arc<AmaraParams>,
    telemetry: Arc<Telemetry>,
    engine: Option<Engine>,
    notes: Notes,
    /// In per-note mode the pitch stays with the last note, released or not.
    last_note_hz: Option<f32>,
}

impl Default for Amara {
    fn default() -> Self {
        Self { params: Arc::new(AmaraParams::default()), telemetry: Arc::default(), engine: None, notes: Notes::default(), last_note_hz: None }
    }
}

impl Plugin for Amara {
    const NAME: &'static str = "Amara";
    const VENDOR: &'static str = "Abdicant";
    const URL: &'static str = "";
    const EMAIL: &'static str = "";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: None,
        main_output_channels: NonZeroU32::new(2),
        ..AudioIOLayout::const_default()
    }];
    const MIDI_INPUT: MidiConfig = MidiConfig::Basic;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(self.params.clone(), self.telemetry.clone())
    }

    fn initialize(
        &mut self,
        _layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        self.engine = Some(Engine::new(buffer_config.sample_rate, (self.params.language.value() as u32, self.params.strangeness())));
        true
    }

    fn reset(&mut self) {
        self.notes = Notes::default();
        self.last_note_hz = None;
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let Some(engine) = self.engine.as_mut() else {
            return ProcessStatus::Normal;
        };
        let params = &self.params;
        let mut next_event = context.next_event();
        let mut reduction = 0.0f32;
        let per_note = params.per_note.value();
        for (sample, mut frame) in buffer.iter_samples().enumerate() {
            while let Some(event) = next_event {
                if event.timing() as usize > sample {
                    break;
                }
                match event {
                    NoteEvent::NoteOn { note, velocity, .. } => {
                        self.notes.press(note);
                        if per_note {
                            let hz = util::midi_note_to_freq(note);
                            self.last_note_hz = Some(hz);
                            engine.say(velocity, hz, &params.controls(Some(hz)).speaker);
                        }
                    }
                    NoteEvent::NoteOff { note, .. } => self.notes.release(note),
                    _ => {}
                }
                next_event = context.next_event();
            }
            let note_hz = if per_note { self.last_note_hz } else { self.notes.hz() };
            let controls = params.controls(note_hz);
            if sample == 0 {
                engine.set_controls(&controls);
            }
            let level = params.level.smoothed.next();
            let [l, r] = engine.process(&controls, level);
            let mut channels = frame.iter_mut();
            if let (Some(left), Some(right)) = (channels.next(), channels.next()) {
                *left = l;
                *right = r;
            }
            reduction = reduction.max(engine.reduction_db());
        }
        engine.drain_marks(|mark| self.telemetry.push(mark));
        self.telemetry.publish(reduction, engine.possessed());
        ProcessStatus::KeepAlive
    }
}

impl ClapPlugin for Amara {
    const CLAP_ID: &'static str = "com.abdicant.amara";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Talks to itself in an invented language, over a bad line");
    const CLAP_MANUAL_URL: Option<&'static str> = None;
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::Instrument, ClapFeature::Synthesizer, ClapFeature::Stereo];
}

impl Vst3Plugin for Amara {
    const VST3_CLASS_ID: [u8; 16] = *b"AmaraSpkAbdicant";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[Vst3SubCategory::Instrument, Vst3SubCategory::Synth];
}

nih_export_clap!(Amara);
nih_export_vst3!(Amara);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_last_note_pressed_wins_and_releases_fall_back() {
        let mut notes = Notes::default();
        assert_eq!(notes.hz(), None);
        notes.press(60);
        notes.press(64);
        assert_eq!(notes.hz(), Some(util::midi_note_to_freq(64)));
        notes.release(64);
        assert_eq!(notes.hz(), Some(util::midi_note_to_freq(60)));
        notes.release(60);
        assert_eq!(notes.hz(), None);
    }

    #[test]
    fn too_many_notes_are_ignored_rather_than_overflowing() {
        let mut notes = Notes::default();
        (0..40).for_each(|n| notes.press(n));
        assert_eq!(notes.count, 16);
        (0..40).for_each(|n| notes.release(n));
        assert_eq!(notes.count, 0);
    }
}
