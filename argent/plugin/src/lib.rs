//! Argent: a four-voice drone instrument on Fizmo-style transwaves, played the Vhikk X
//! way, as a CLAP and VST3 plugin. `dsp` is the sound; this file is the wrapper.
//!
//! Notes latch: a key starts a note fading in, and letting go doesn't end it. Release
//! (a parameter, so any controller can be mapped to it) fades out every note whose key
//! isn't down. A new SEED's tables are built on a background thread and fade in.

pub mod dsp;
mod editor;
mod params;
mod telemetry;

use std::num::NonZeroU32;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};

use nih_plug::prelude::*;

use dsp::engine::Engine;
use dsp::tables::Bank;
use params::ArgentParams;
use telemetry::{Telemetry, TRACE};

/// Where finished banks wait for the audio thread, and where it leaves the old ones to
/// be dropped elsewhere.
#[derive(Default)]
pub struct BankSlot {
    wanted: AtomicU32,
    ready: Mutex<Option<Arc<Bank>>>,
    graveyard: Mutex<Vec<Arc<Bank>>>,
}

impl BankSlot {
    fn new() -> Self {
        Self { wanted: AtomicU32::new(u32::MAX), ready: Mutex::new(None), graveyard: Mutex::new(Vec::with_capacity(8)) }
    }

    /// On the background thread. Skips a seed that's already been moved past.
    fn build(&self, seed: u32) {
        if let Ok(mut graveyard) = self.graveyard.lock() {
            graveyard.clear();
        }
        if self.wanted.load(Ordering::Acquire) != seed {
            return;
        }
        let bank = Bank::build(seed);
        if self.wanted.load(Ordering::Acquire) == seed {
            if let Ok(mut ready) = self.ready.lock() {
                *ready = Some(bank);
            }
        }
    }
}

pub enum Task {
    Build(u32),
}

pub struct Argent {
    params: Arc<ArgentParams>,
    telemetry: Arc<Telemetry>,
    slot: Arc<BankSlot>,
    engine: Option<Engine>,
    requested_seed: u32,
    release_was_down: bool,
    trace: [f32; TRACE],
}

impl Default for Argent {
    fn default() -> Self {
        Self {
            params: Arc::new(ArgentParams::default()),
            telemetry: Arc::default(),
            slot: Arc::new(BankSlot::new()),
            engine: None,
            requested_seed: u32::MAX,
            release_was_down: false,
            trace: [0.0; TRACE],
        }
    }
}

impl Argent {
    /// Asks for the knob's seed if it's new, and takes a finished bank if one is waiting.
    fn follow_seed(&mut self, context: &mut impl ProcessContext<Self>) {
        let Some(engine) = self.engine.as_mut() else { return };
        let seed = self.params.seed.value() as u32;
        if seed != self.requested_seed {
            self.requested_seed = seed;
            self.slot.wanted.store(seed, Ordering::Release);
            if seed != engine.seed() {
                context.execute_background(Task::Build(seed));
            }
        }
        if let Some(old) = engine.take_retired() {
            match self.slot.graveyard.try_lock() {
                Ok(mut graveyard) if graveyard.len() < graveyard.capacity() => graveyard.push(old),
                _ => engine.keep_retired(old),
            }
        }
        if engine.ready_for_bank() && seed != engine.seed() {
            if let Ok(mut ready) = self.slot.ready.try_lock() {
                if ready.as_ref().is_some_and(|bank| bank.seed == seed) {
                    if let Some(bank) = ready.take() {
                        engine.take_bank(bank);
                    }
                }
            }
        }
    }
}

impl Plugin for Argent {
    const NAME: &'static str = "Argent";
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
    type BackgroundTask = Task;

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn task_executor(&mut self) -> TaskExecutor<Self> {
        let slot = self.slot.clone();
        Box::new(move |task| match task {
            Task::Build(seed) => slot.build(seed),
        })
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
        let seed = self.params.seed.value() as u32;
        self.requested_seed = seed;
        self.slot.wanted.store(seed, Ordering::Release);
        self.engine = Some(Engine::new(buffer_config.sample_rate, Bank::build(seed)));
        true
    }

    fn reset(&mut self) {
        self.release_was_down = false;
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        self.follow_seed(context);
        let Some(engine) = self.engine.as_mut() else {
            return ProcessStatus::Normal;
        };
        let release = self.params.release.value();
        if release && !self.release_was_down {
            engine.release();
        }
        self.release_was_down = release;

        let mut next_event = context.next_event();
        for (sample, mut frame) in buffer.iter_samples().enumerate() {
            while let Some(event) = next_event {
                if event.timing() as usize > sample {
                    break;
                }
                match event {
                    NoteEvent::NoteOn { note, .. } => engine.press(note),
                    NoteEvent::NoteOff { note, .. } => engine.lift(note),
                    _ => {}
                }
                next_event = context.next_event();
            }
            let controls = self.params.next_controls();
            let [l, r] = engine.process(&controls);
            let mut channels = frame.iter_mut();
            if let (Some(left), Some(right)) = (channels.next(), channels.next()) {
                *left = l;
                *right = r;
            }
        }
        engine.trace(&mut self.trace);
        self.telemetry.publish(&self.trace, engine.shown());
        // Latched notes and slow fades: it keeps sounding with no input at all.
        ProcessStatus::KeepAlive
    }
}

impl ClapPlugin for Argent {
    const CLAP_ID: &'static str = "com.abdicant.argent";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("A four-voice drone instrument on transwaves");
    const CLAP_MANUAL_URL: Option<&'static str> = None;
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::Instrument, ClapFeature::Synthesizer, ClapFeature::Stereo];
}

impl Vst3Plugin for Argent {
    const VST3_CLASS_ID: [u8; 16] = *b"ArgentDrnAbdican";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[Vst3SubCategory::Instrument, Vst3SubCategory::Synth];
}

nih_export_clap!(Argent);
nih_export_vst3!(Argent);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_seed_that_was_moved_past_is_never_built() {
        let slot = BankSlot::new();
        slot.wanted.store(5, Ordering::Release);
        slot.build(4);
        assert!(slot.ready.lock().unwrap().is_none());
        slot.build(5);
        assert_eq!(slot.ready.lock().unwrap().as_ref().map(|b| b.seed), Some(5));
    }
}
