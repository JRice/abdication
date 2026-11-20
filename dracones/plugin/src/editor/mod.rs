//! The panel: vector graphics on a fixed 1240 x 800 canvas, scaled as a whole.

mod paint;
mod panel;
mod svg;
mod world;

use std::sync::Arc;

use nih_plug::prelude::Editor;
use nih_plug_vizia::{create_vizia_editor, ViziaState, ViziaTheming};

use crate::params::DraconesParams;
use crate::telemetry::Telemetry;

pub const WIDTH: f32 = 1240.0;
pub const HEIGHT: f32 = 800.0;

pub fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (WIDTH as u32, HEIGHT as u32))
}

pub fn create(params: Arc<DraconesParams>, telemetry: Arc<Telemetry>) -> Option<Box<dyn Editor>> {
    create_vizia_editor(params.editor_state.clone(), ViziaTheming::Custom, move |cx, _| {
        panel::Panel::new(cx, params.clone(), telemetry.clone());
    })
}
