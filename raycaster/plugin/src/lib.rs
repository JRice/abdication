//! Raycaster: a six-voice synthesizer whose oscillators are shapes turning once per cycle,
//! read by a ray dropped from above, as a CLAP and VST3 plugin. `dsp` is the sound; this
//! file is the wrapper.
//!
//! Each oscillator plays one of six elements (Earth, Water, Air, Fire, Wood, Metal) or a
//! Chaos: three of them overlapped at random from a seed. Chaos outlines are built on
//! a background thread and handed to the audio thread when ready.

pub mod dsp;
mod editor;
pub mod params;
mod telemetry;

use std::num::NonZeroU32;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};

use nih_plug::prelude::*;

use dsp::engine::{BlockControls, Engine, MAX_BLOCK};
use dsp::geometry::Outline;
use dsp::modulator::Clock;
use dsp::shapes::chaos;
use dsp::targets::OSCS;
use params::RaycasterParams;
use telemetry::Telemetry;

/// Where finished Chaos outlines wait for the audio thread, and where it leaves the old
/// ones to be dropped elsewhere.
pub struct ChaosSlots {
    wanted: [AtomicU32; OSCS],
    ready: [Mutex<Option<(u32, Arc<Outline>)>>; OSCS],
    graveyard: Mutex<Vec<Arc<Outline>>>,
}

impl ChaosSlots {
    fn new() -> Self {
        Self {
            wanted: std::array::from_fn(|_| AtomicU32::new(u32::MAX)),
            ready: std::array::from_fn(|_| Mutex::new(None)),
            graveyard: Mutex::new(Vec::with_capacity(8)),
        }
    }

    /// On the background thread. Skips a seed that's already been moved past.
    fn build(&self, osc: usize, seed: u32) {
        if let Ok(mut graveyard) = self.graveyard.lock() {
            graveyard.clear();
        }
        if self.wanted[osc].load(Ordering::Acquire) != seed {
            return;
        }
        let outline = Arc::new(chaos(seed).0);
        if self.wanted[osc].load(Ordering::Acquire) == seed {
            if let Ok(mut ready) = self.ready[osc].lock() {
                *ready = Some((seed, outline));
            }
        }
    }
}

pub enum Task {
    Build(usize, u32),
}

pub struct Raycaster {
    params: Arc<RaycasterParams>,
    telemetry: Arc<Telemetry>,
    slots: Arc<ChaosSlots>,
    engine: Option<Engine>,
    controls: Option<BlockControls>,
    requested: [u32; OSCS],
    retired: Option<Arc<Outline>>,
    sample_rate: f32,
}

impl Default for Raycaster {
    fn default() -> Self {
        Self {
            params: Arc::new(RaycasterParams::default()),
            telemetry: Arc::default(),
            slots: Arc::new(ChaosSlots::new()),
            engine: None,
            controls: None,
            requested: [u32::MAX; OSCS],
            retired: None,
            sample_rate: 48_000.0,
        }
    }
}

impl Raycaster {
    /// Asks for each oscillator's Chaos seed if it's new, and takes a finished outline
    /// if one is waiting.
    fn follow_seeds(&mut self, context: &mut impl ProcessContext<Self>) {
        let Some(engine) = self.engine.as_mut() else { return };
        if let Some(old) = self.retired.take() {
            match self.slots.graveyard.try_lock() {
                Ok(mut graveyard) if graveyard.len() < graveyard.capacity() => graveyard.push(old),
                _ => self.retired = Some(old),
            }
        }
        for osc in 0..OSCS {
            let seed = self.params.osc[osc].seed.value() as u32;
            if seed != self.requested[osc] {
                self.requested[osc] = seed;
                self.slots.wanted[osc].store(seed, Ordering::Release);
                context.execute_background(Task::Build(osc, seed));
            }
            if self.retired.is_some() {
                continue;
            }
            if let Ok(mut ready) = self.slots.ready[osc].try_lock() {
                if ready.as_ref().is_some_and(|(s, _)| *s == seed) {
                    if let Some((_, outline)) = ready.take() {
                        self.retired = Some(engine.take_chaos(osc, outline));
                    }
                }
            }
        }
    }

    fn handle(&mut self, event: NoteEvent<()>) {
        let (Some(engine), Some(controls)) = (self.engine.as_mut(), self.controls.as_ref()) else { return };
        match event {
            NoteEvent::NoteOn { note, velocity, .. } => engine.note_on(note, velocity, controls),
            NoteEvent::NoteOff { note, .. } | NoteEvent::Choke { note, .. } => engine.note_off(note),
            NoteEvent::MidiPitchBend { value, .. } => engine.pitch_bend(value),
            NoteEvent::MidiCC { cc: 1, value, .. } => engine.mod_wheel(value),
            NoteEvent::MidiCC { cc: 123, .. } => engine.all_notes_off(),
            NoteEvent::MidiChannelPressure { pressure, .. } => engine.channel_pressure(pressure),
            NoteEvent::PolyPressure { note, pressure, .. } => engine.poly_pressure(note, pressure),
            _ => {}
        }
    }
}

impl Plugin for Raycaster {
    const NAME: &'static str = "Raycaster";
    const VENDOR: &'static str = "Abdicant";
    const URL: &'static str = "";
    const EMAIL: &'static str = "";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: None,
        main_output_channels: NonZeroU32::new(2),
        ..AudioIOLayout::const_default()
    }];
    const MIDI_INPUT: MidiConfig = MidiConfig::MidiCCs;

    type SysExMessage = ();
    type BackgroundTask = Task;

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn task_executor(&mut self) -> TaskExecutor<Self> {
        let slots = self.slots.clone();
        Box::new(move |task| match task {
            Task::Build(osc, seed) => slots.build(osc, seed),
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
        self.sample_rate = buffer_config.sample_rate;
        let mut engine = Engine::new(buffer_config.sample_rate);
        for osc in 0..OSCS {
            let seed = self.params.osc[osc].seed.value() as u32;
            self.requested[osc] = seed;
            self.slots.wanted[osc].store(seed, Ordering::Release);
            engine.take_chaos(osc, Arc::new(chaos(seed).0));
        }
        self.engine = Some(engine);
        self.controls = Some(self.params.block(0, Clock::default()));
        true
    }

    fn reset(&mut self) {
        if let Some(engine) = self.engine.as_mut() {
            engine.all_notes_off();
        }
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        self.follow_seeds(context);
        if self.engine.is_none() {
            return ProcessStatus::Normal;
        }
        let transport = context.transport();
        let tempo = transport.tempo.unwrap_or(120.0);
        let position = if transport.playing { transport.pos_beats() } else { None };
        let total = buffer.samples();
        let channels = buffer.as_slice();
        let (left, right) = channels.split_at_mut(1);
        let (left, right) = (&mut left[0], &mut right[0]);

        let mut next_event = context.next_event();
        let mut start = 0;
        while start < total {
            while let Some(event) = next_event {
                if event.timing() as usize > start {
                    break;
                }
                self.handle(event);
                next_event = context.next_event();
            }
            let until = next_event.map(|e| (e.timing() as usize).max(start + 1)).unwrap_or(total);
            let end = total.min(start + MAX_BLOCK).min(until);
            let n = end - start;
            let clock = Clock { tempo, beats: position.map(|p| p + start as f64 / self.sample_rate as f64 * tempo / 60.0) };
            let controls = self.params.block(n as u32, clock);
            if let Some(engine) = self.engine.as_mut() {
                engine.render(&controls, &mut left[start..end], &mut right[start..end]);
            }
            self.controls = Some(controls);
            start = end;
        }
        if let Some(engine) = self.engine.as_ref() {
            self.telemetry.publish(&engine.shown());
        }
        ProcessStatus::KeepAlive
    }
}

impl ClapPlugin for Raycaster {
    const CLAP_ID: &'static str = "com.abdicant.raycaster";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Shapes turning once per cycle, read by a falling ray");
    const CLAP_MANUAL_URL: Option<&'static str> = None;
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::Instrument, ClapFeature::Synthesizer, ClapFeature::Stereo];
}

impl Vst3Plugin for Raycaster {
    const VST3_CLASS_ID: [u8; 16] = *b"RaycasterAbdcant";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[Vst3SubCategory::Instrument, Vst3SubCategory::Synth];
}

nih_export_clap!(Raycaster);
nih_export_vst3!(Raycaster);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_seed_that_was_moved_past_is_never_built() {
        let slots = ChaosSlots::new();
        slots.wanted[1].store(5, Ordering::Release);
        slots.build(1, 4);
        assert!(slots.ready[1].lock().unwrap().is_none());
        slots.build(1, 5);
        assert_eq!(slots.ready[1].lock().unwrap().as_ref().map(|(s, _)| *s), Some(5));
    }
}
