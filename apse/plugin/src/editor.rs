//! The panel: vector graphics on a fixed logical canvas, scaled as a whole.

mod paint;
mod panel;

use std::sync::Arc;

use nih_plug::prelude::Editor;
use nih_plug_vizia::{create_vizia_editor, ViziaState, ViziaTheming};

use crate::params::ApseParams;
use crate::telemetry::Telemetry;

pub fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (panel::WIDTH as u32, panel::HEIGHT as u32))
}

pub fn create(params: Arc<ApseParams>, telemetry: Arc<Telemetry>) -> Option<Box<dyn Editor>> {
    create_vizia_editor(params.editor_state.clone(), ViziaTheming::Custom, move |cx, _| {
        panel::Panel::new(cx, params.clone(), telemetry.clone());
    })
}
