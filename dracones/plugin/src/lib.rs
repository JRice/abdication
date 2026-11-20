//! Hic Sunt Dracones (HC SVNT DRACONES on its panel): a free-running drone synth by
//! Abdicant, made of pieces of its siblings (Argent, Abdication, Antimeme, Apse), as a
//! CLAP and VST3 instrument. `dsp` and `engine` are the sound; this file is the wrapper.

pub mod dsp;
mod editor;
pub mod engine;
pub mod knobs;
mod params;
mod telemetry;

use std::num::NonZeroU32;
use std::sync::Arc;

use nih_plug::prelude::*;

use dsp::mods::Transport;
use engine::Engine;
use params::DraconesParams;
use telemetry::Telemetry;

pub struct Dracones {
    params: Arc<DraconesParams>,
    telemetry: Arc<Telemetry>,
    engine: Option<Engine>,
}

impl Default for Dracones {
    fn default() -> Self {
        Self { params: Arc::new(DraconesParams::default()), telemetry: Arc::default(), engine: None }
    }
}

impl Plugin for Dracones {
    const NAME: &'static str = "Hic Sunt Dracones";
    const VENDOR: &'static str = "Abdicant";
    const URL: &'static str = "";
    const EMAIL: &'static str = "";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: None,
        main_output_channels: NonZeroU32::new(2),
        ..AudioIOLayout::const_default()
    }];

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(self.params.clone(), self.telemetry.clone())
    }

    fn initialize(&mut self, _layout: &AudioIOLayout, buffer_config: &BufferConfig,
                  _context: &mut impl InitContext<Self>) -> bool {
        self.engine = Some(Engine::new(buffer_config.sample_rate));
        true
    }

    fn process(&mut self, buffer: &mut Buffer, _aux: &mut AuxiliaryBuffers,
               context: &mut impl ProcessContext<Self>) -> ProcessStatus {
        let Some(engine) = self.engine.as_mut() else {
            return ProcessStatus::Normal;
        };
        let t = context.transport();
        let transport = Transport { tempo: t.tempo.unwrap_or(120.0), beats: if t.playing { t.pos_beats() } else { None } };
        let controls = self.params.controls();
        let channels = buffer.as_slice();
        if let [left, right, ..] = channels {
            engine.process(left, right, &controls, transport);
        }
        self.telemetry.publish(&engine.report);
        ProcessStatus::KeepAlive
    }
}

impl ClapPlugin for Dracones {
    const CLAP_ID: &'static str = "com.abdicant.dracones";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("A free-running drone synth: here be dragons");
    const CLAP_MANUAL_URL: Option<&'static str> = None;
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] =
        &[ClapFeature::Instrument, ClapFeature::Synthesizer, ClapFeature::Stereo];
}

impl Vst3Plugin for Dracones {
    const VST3_CLASS_ID: [u8; 16] = *b"DraconesAbdicant";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[Vst3SubCategory::Instrument, Vst3SubCategory::Synth];
}

nih_export_clap!(Dracones);
nih_export_vst3!(Dracones);
