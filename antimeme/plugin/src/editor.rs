//! The plugin's panel: "Slices, in Tibetan". Vector graphics on a 960 x 640 logical
//! canvas, scaled as a whole, with glitches that answer the output limiter.

mod glitch;
mod layout;
mod paint;
mod panel;

use std::sync::Arc;

use nih_plug::prelude::Editor;
use nih_plug_vizia::{create_vizia_editor, ViziaState, ViziaTheming};

use crate::params::AntimemeParams;
use crate::telemetry::Telemetry;

pub fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (layout::WIDTH as u32, layout::HEIGHT as u32))
}

pub fn create(params: Arc<AntimemeParams>, telemetry: Arc<Telemetry>) -> Option<Box<dyn Editor>> {
    create_vizia_editor(params.editor_state.clone(), ViziaTheming::Custom, move |cx, _| {
        panel::Panel::new(cx, params.clone(), telemetry.clone());
    })
}
