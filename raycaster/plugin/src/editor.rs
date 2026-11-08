//! The panel: one oscillator at a time (pages I and II) over its element's backdrop, the
//! filter and amp beside it, and the dynamic modulator for whichever control was touched
//! last along the bottom. Drawn by hand on a fixed logical canvas and scaled as a whole.

mod backdrops;
mod painter;
mod panel;

use std::sync::Arc;

use nih_plug::prelude::Editor;
use nih_plug_vizia::{create_vizia_editor, ViziaState, ViziaTheming};

use crate::params::RaycasterParams;
use crate::telemetry::Telemetry;

pub fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (panel::WIDTH as u32, panel::HEIGHT as u32))
}

pub fn create(params: Arc<RaycasterParams>, telemetry: Arc<Telemetry>) -> Option<Box<dyn Editor>> {
    create_vizia_editor(params.editor_state.clone(), ViziaTheming::Custom, move |cx, _| {
        panel::Panel::new(cx, params.clone(), telemetry.clone());
    })
}
