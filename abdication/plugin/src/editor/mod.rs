//! The plugin's own editor: the front panel from the design mockup, drawn
//! as vector graphics on a 1600 x 1000 logical canvas and scaled as a
//! whole (fixed at 80%: 1280 x 800).

mod draw;
pub mod layout;
mod panel;
mod scopes;
mod screens;

use std::sync::Arc;

use nih_plug::prelude::Editor;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};

use crate::params::SynthParams;
use crate::scope::{LiveScope, ScopePitch};

/// 80%: 1280 x 800.
pub fn default_state() -> Arc<ViziaState> {
    ViziaState::new_with_default_scale_factor(|| (layout::WIDTH as u32, layout::HEIGHT as u32), 0.8)
}

pub fn create(params: Arc<SynthParams>, pitch: Arc<ScopePitch>, live: Arc<LiveScope>) -> Option<Box<dyn Editor>> {
    create_vizia_editor(params.editor_state.clone(), ViziaTheming::Custom, move |cx, _| {
        assets::register_noto_sans_light(cx);
        panel::Panel::new(cx, params.clone(), pitch.clone(), live.clone());
    })
}
