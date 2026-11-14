//! Apse: a plate-family reverb with six modes, loosely after Spire's (and a choir after
//! the BigSky's Chorale), as a CLAP and VST3 effect. `dsp` is the sound; this file is the
//! wrapper.

pub mod dsp;
mod editor;
mod params;
mod telemetry;

use std::num::NonZeroU32;
use std::sync::Arc;

use nih_plug::prelude::*;

use dsp::engine::Engine;
use params::ApseParams;
use telemetry::Telemetry;

pub struct Apse {
    params: Arc<ApseParams>,
    telemetry: Arc<Telemetry>,
    engine: Option<Engine>,
}

impl Default for Apse {
    fn default() -> Self {
        Self { params: Arc::new(ApseParams::default()), telemetry: Arc::default(), engine: None }
    }
}

impl Plugin for Apse {
    const NAME: &'static str = "Apse";
    const VENDOR: &'static str = "Abdicant";
    const URL: &'static str = "";
    const EMAIL: &'static str = "";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
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

    fn initialize(
        &mut self,
        _layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        let mut engine = Engine::new(buffer_config.sample_rate);
        engine.set_mode(self.params.mode.value() as usize);
        self.engine = Some(engine);
        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let Some(engine) = self.engine.as_mut() else {
            return ProcessStatus::Normal;
        };
        engine.set_mode(self.params.mode.value() as usize);
        let mut reduction = 0.0f32;
        for mut frame in buffer.iter_samples() {
            let controls = self.params.controls();
            let mut channels = frame.iter_mut();
            let (Some(left), Some(right)) = (channels.next(), channels.next()) else {
                continue;
            };
            [*left, *right] = engine.process([*left, *right], &controls);
            reduction = reduction.max(engine.reduction_db());
        }
        self.telemetry.publish(engine.take_wet_peak(), reduction);
        // Frozen, it rings forever; otherwise it still has a tail to finish.
        ProcessStatus::KeepAlive
    }
}

impl ClapPlugin for Apse {
    const CLAP_ID: &'static str = "com.abdicant.apse";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("A plate-family reverb with six modes, one of them a choir");
    const CLAP_MANUAL_URL: Option<&'static str> = None;
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::Reverb, ClapFeature::Stereo];
}

impl Vst3Plugin for Apse {
    const VST3_CLASS_ID: [u8; 16] = *b"ApseAbdicantVerb";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[Vst3SubCategory::Fx, Vst3SubCategory::Reverb];
}

nih_export_clap!(Apse);
nih_export_vst3!(Apse);
