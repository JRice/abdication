//! The panel: a sky of silver constellations, drawn by hand on a fixed logical canvas
//! and scaled as a whole.

mod panel;
mod sky;

use std::sync::Arc;

use nih_plug::prelude::Editor;
use nih_plug_vizia::{create_vizia_editor, ViziaState, ViziaTheming};

use crate::params::ArgentParams;
use crate::telemetry::Telemetry;

pub fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (panel::WIDTH as u32, panel::HEIGHT as u32))
}

pub fn create(params: Arc<ArgentParams>, telemetry: Arc<Telemetry>) -> Option<Box<dyn Editor>> {
    create_vizia_editor(params.editor_state.clone(), ViziaTheming::Custom, move |cx, _| {
        panel::Panel::new(cx, params.clone(), telemetry.clone());
    })
}
