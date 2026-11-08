//! The plugin's own panel: the design mockup's five part cards (each with
//! a flip side), the screen and the effects row, drawn as vector graphics
//! on a 1000 x 540 logical canvas and scaled as a whole.

mod drag;
mod draw;
pub mod layout;
mod panel;
pub mod randomize;
mod scopes;
mod screens;
mod values;

use std::sync::atomic::AtomicU32;
use std::sync::Arc;

use nih_plug::prelude::Editor;
use nih_plug_vizia::{create_vizia_editor, ViziaState, ViziaTheming};

use crate::params::DrumParams;
use crate::scope::HitScope;

/// Opens at 120 %: 1200 x 648.
pub fn default_state() -> Arc<ViziaState> {
    ViziaState::new_with_default_scale_factor(|| (layout::WIDTH as u32, layout::HEIGHT as u32), 1.2)
}

/// `scope`: the latest hit's parts; `meter`: the output's peak since the
/// panel last looked; `sample_rate`: the host's (f32 bits), for exports.
pub fn create(params: Arc<DrumParams>, scope: Arc<HitScope>, meter: Arc<AtomicU32>, sample_rate: Arc<AtomicU32>)
    -> Option<Box<dyn Editor>> {
    create_vizia_editor(params.editor_state.clone(), ViziaTheming::Custom, move |cx, _| {
        panel::Panel::new(cx, params.clone(), scope.clone(), meter.clone(), sample_rate.clone());
    })
}
