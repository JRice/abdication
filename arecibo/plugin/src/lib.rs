//! Arecibo: a no-input mixer with a shortwave link in its feedback loop, as a CLAP and
//! VST3 effect. It runs with nothing going in. `dsp` is the sound; this is the wrapper.

pub mod dsp;
mod editor;
mod params;
mod telemetry;

use std::num::NonZeroU32;
use std::sync::Arc;

use nih_plug::prelude::*;

use dsp::engine::Engine;
use params::AreciboParams;
use telemetry::Telemetry;

/// The waterfall doesn't need the top octave.
const WATERFALL_DECIMATION: usize = 2;

pub struct Arecibo {
    params: Arc<AreciboParams>,
    telemetry: Arc<Telemetry>,
    engine: Option<Engine>,
    waterfall_sum: f32,
    waterfall_count: usize,
}

impl Default for Arecibo {
    fn default() -> Self {
        Self { params: Arc::new(AreciboParams::default()), telemetry: Arc::default(), engine: None, waterfall_sum: 0.0, waterfall_count: 0 }
    }
}

impl Plugin for Arecibo {
    const NAME: &'static str = "Arecibo";
    const VENDOR: &'static str = "Abdicant";
    const URL: &'static str = "";
    const EMAIL: &'static str = "";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),
            aux_input_ports: &[new_nonzero_u32(2)],
            names: PortNames { aux_inputs: &["Sidechain"], ..PortNames::const_default() },
            ..AudioIOLayout::const_default()
        },
        // For hosts that can't offer a sidechain: stations C and D just run on feedback.
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
    ];

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
        aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let Some(engine) = self.engine.as_mut() else {
            return ProcessStatus::Normal;
        };
        let mut reduction = 0.0f32;
        let mut first = true;
        let side_channels = aux.inputs.first().map(|side| side.as_slice_immutable());
        let side_at = |sample: usize| match side_channels {
            Some([left, right]) => [left[sample], right[sample]],
            _ => [0.0; 2],
        };
        for (sample, mut frame) in buffer.iter_samples().enumerate() {
            let controls = self.params.next_controls();
            if first {
                engine.set_controls(&controls);
                first = false;
            }
            let mut channels = frame.iter_mut();
            let (Some(left), Some(right)) = (channels.next(), channels.next()) else {
                continue;
            };
            let [l, r] = engine.process([*left, *right], side_at(sample), &controls);
            *left = l;
            *right = r;
            reduction = reduction.max(engine.reduction_db());
            self.waterfall_sum += 0.5 * (l + r);
            self.waterfall_count += 1;
            if self.waterfall_count == WATERFALL_DECIMATION {
                self.telemetry.push_sample(self.waterfall_sum / WATERFALL_DECIMATION as f32);
                self.waterfall_sum = 0.0;
                self.waterfall_count = 0;
            }
        }
        self.telemetry.publish(reduction, engine.agc_gains());
        // Nothing needs to go in for something to come out, so it never sleeps.
        ProcessStatus::KeepAlive
    }
}

impl ClapPlugin for Arecibo {
    const CLAP_ID: &'static str = "com.abdicant.arecibo";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("A no-input mixer with a shortwave link in its feedback loop");
    const CLAP_MANUAL_URL: Option<&'static str> = None;
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::Distortion, ClapFeature::Stereo];
}

impl Vst3Plugin for Arecibo {
    const VST3_CLASS_ID: [u8; 16] = *b"AreciboSkAbdican";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[Vst3SubCategory::Fx, Vst3SubCategory::Modulation];
}

nih_export_clap!(Arecibo);
nih_export_vst3!(Arecibo);
