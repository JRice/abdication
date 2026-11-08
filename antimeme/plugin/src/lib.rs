//! Antimeme: a noisy delay after the Strega's Time/Filter experiment, as a CLAP
//! and VST3 effect. `dsp` is the sound; this file is the plugin wrapper.

pub mod dsp;
mod editor;
mod params;
mod telemetry;

use std::num::NonZeroU32;
use std::sync::Arc;

use nih_plug::prelude::*;

use dsp::engine::Engine;
use params::AntimemeParams;
use telemetry::Telemetry;

pub struct Antimeme {
    params: Arc<AntimemeParams>,
    telemetry: Arc<Telemetry>,
    engine: Option<Engine>,
}

impl Default for Antimeme {
    fn default() -> Self {
        Self { params: Arc::new(AntimemeParams::default()), telemetry: Arc::default(), engine: None }
    }
}

impl Plugin for Antimeme {
    const NAME: &'static str = "Antimeme";
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
        context: &mut impl InitContext<Self>,
    ) -> bool {
        let engine = Engine::new(buffer_config.sample_rate);
        context.set_latency_samples(engine.latency());
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
        let mut reduction = 0.0f32;
        for mut frame in buffer.iter_samples() {
            let controls = self.params.next_controls();
            let mut channels = frame.iter_mut();
            let (Some(left), Some(right)) = (channels.next(), channels.next()) else {
                continue;
            };
            let [l, r] = engine.process([*left, *right], &controls);
            *left = l;
            *right = r;
            reduction = reduction.max(engine.reduction_db());
        }
        self.telemetry.publish(reduction, engine.agitation_phase(), engine.interference());
        // It makes noise with no input, so it never goes to sleep.
        ProcessStatus::KeepAlive
    }
}

impl ClapPlugin for Antimeme {
    const CLAP_ID: &'static str = "com.abdicant.antimeme";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("A noisy delay after the Strega's Time/Filter experiment");
    const CLAP_MANUAL_URL: Option<&'static str> = None;
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::Delay, ClapFeature::Distortion, ClapFeature::Stereo];
}

impl Vst3Plugin for Antimeme {
    const VST3_CLASS_ID: [u8; 16] = *b"AntimemeAbdicant";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[Vst3SubCategory::Fx, Vst3SubCategory::Delay];
}

nih_export_clap!(Antimeme);
nih_export_vst3!(Antimeme);
