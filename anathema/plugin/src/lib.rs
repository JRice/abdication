//! Anathema: an early digital reverb with its clock in your hand and its memory bus
//! shorted, as a CLAP and VST3 effect. `dsp` is the sound; this file is the wrapper.

pub mod dsp;
mod editor;
mod params;
mod telemetry;

use std::num::NonZeroU32;
use std::sync::Arc;

use nih_plug::prelude::*;

use dsp::engine::{Controls, Engine};
use params::{clock_hz, AnathemaParams};
use telemetry::Telemetry;

pub struct Anathema {
    params: Arc<AnathemaParams>,
    telemetry: Arc<Telemetry>,
    engine: Option<Engine>,
}

impl Default for Anathema {
    fn default() -> Self {
        Self { params: Arc::new(AnathemaParams::default()), telemetry: Arc::default(), engine: None }
    }
}

impl Plugin for Anathema {
    const NAME: &'static str = "Anathema";
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
        self.engine = Some(Engine::new(buffer_config.sample_rate));
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
        let params = &self.params;
        let knobs = params.knobs();
        engine.set_knobs(&knobs);
        let mut reduction = 0.0f32;
        for mut frame in buffer.iter_samples() {
            let controls = Controls {
                clock_hz: clock_hz(params.reclock.smoothed.next()),
                drive: params.drive.smoothed.next(),
                direct: params.direct.value(),
                whine: params.whine.value(),
                level: params.level.smoothed.next(),
            };
            let mut channels = frame.iter_mut();
            let (Some(left), Some(right)) = (channels.next(), channels.next()) else {
                continue;
            };
            let [l, r] = engine.process([*left, *right], &controls);
            *left = l;
            *right = r;
            reduction = reduction.max(engine.reduction_db());
        }
        self.telemetry.publish(reduction);
        // With the wrong switches closed it sings with no input, so it never sleeps.
        ProcessStatus::KeepAlive
    }
}

impl ClapPlugin for Anathema {
    const CLAP_ID: &'static str = "com.abdicant.anathema";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("An early digital reverb, reclocked and shorted");
    const CLAP_MANUAL_URL: Option<&'static str> = None;
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::Reverb, ClapFeature::Glitch, ClapFeature::Stereo];
}

impl Vst3Plugin for Anathema {
    const VST3_CLASS_ID: [u8; 16] = *b"AnathemaAbdicant";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[Vst3SubCategory::Fx, Vst3SubCategory::Reverb];
}

nih_export_clap!(Anathema);
nih_export_vst3!(Anathema);
