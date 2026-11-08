//! The host parameters. The names follow the Strega's panel.

use std::sync::Arc;

use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;

use crate::dsp::agitation::{speed_hz, speed_knob};
use crate::dsp::engine::Controls;
use crate::dsp::voice::{time_knob, time_seconds};

const SMOOTH_MS: f32 = 20.0;
const TIME_SMOOTH_MS: f32 = 60.0;

fn percent(value: f32) -> String {
    format!("{:.0} %", value * 100.0)
}

/// The number at the start of a typed value: units are optional.
fn leading_number(text: &str) -> Option<f32> {
    text.trim().trim_end_matches(|c: char| !c.is_ascii_digit()).parse().ok()
}

fn parse_percent(text: &str) -> Option<f32> {
    leading_number(text).map(|value| value / 100.0)
}

fn knob(name: &str, default: f32) -> FloatParam {
    FloatParam::new(name, default, FloatRange::Linear { min: 0.0, max: 1.0 })
        .with_smoother(SmoothingStyle::Linear(SMOOTH_MS))
        .with_value_to_string(Arc::new(percent))
        .with_string_to_value(Arc::new(parse_percent))
}

/// A modulation send, -100..100 %.
fn send(name: &str, default: f32) -> FloatParam {
    FloatParam::new(name, default, FloatRange::Linear { min: -1.0, max: 1.0 })
        .with_smoother(SmoothingStyle::Linear(SMOOTH_MS))
        .with_value_to_string(Arc::new(percent))
        .with_string_to_value(Arc::new(parse_percent))
}

/// The defaults are the settings in the favourite Strega patch, fave1.
#[derive(Params)]
pub struct AntimemeParams {
    #[persist = "editor-state"]
    pub editor_state: Arc<ViziaState>,

    #[id = "time"]
    pub time: FloatParam,
    #[id = "decay"]
    pub decay: FloatParam,
    #[id = "filter"]
    pub filter: FloatParam,
    #[id = "absorb"]
    pub absorb: FloatParam,
    #[id = "blend"]
    pub blend: FloatParam,
    #[id = "strength"]
    pub strength: FloatParam,
    #[id = "level"]
    pub level: FloatParam,
    #[id = "spread"]
    pub spread: FloatParam,
    #[id = "whine"]
    pub whine: FloatParam,

    #[id = "agit_speed"]
    pub agitation_speed: FloatParam,
    #[id = "agit_angle"]
    pub agitation_angle: FloatParam,
    #[id = "agit_time"]
    pub agitation_to_time: FloatParam,
    #[id = "agit_filter"]
    pub agitation_to_filter: FloatParam,
    #[id = "agit_decay"]
    pub agitation_to_decay: FloatParam,
    #[id = "agit_absorb"]
    pub agitation_to_absorb: FloatParam,
    #[id = "agit_whine"]
    pub agitation_to_whine: FloatParam,

    #[id = "intf_time"]
    pub interference_to_time: FloatParam,
    #[id = "intf_filter"]
    pub interference_to_filter: FloatParam,
    #[id = "intf_decay"]
    pub interference_to_decay: FloatParam,
    #[id = "intf_absorb"]
    pub interference_to_absorb: FloatParam,
    #[id = "intf_whine"]
    pub interference_to_whine: FloatParam,
}

impl Default for AntimemeParams {
    fn default() -> Self {
        Self {
            editor_state: crate::editor::default_state(),
            time: knob("Time", 0.1)
                .with_smoother(SmoothingStyle::Linear(TIME_SMOOTH_MS))
                .with_value_to_string(Arc::new(|v| format!("{:.0} ms", time_seconds(v) * 1000.0)))
                .with_string_to_value(Arc::new(|text| leading_number(text).map(|ms| time_knob(ms / 1000.0)))),
            decay: knob("Decay", 0.7),
            filter: knob("Filter", 0.35),
            absorb: knob("Absorb", 0.35),
            blend: knob("Blend", 0.35),
            strength: knob("Strength", 0.0),
            level: FloatParam::new(
                "Level",
                util::db_to_gain(0.0),
                FloatRange::Skewed { min: util::db_to_gain(-48.0), max: util::db_to_gain(12.0), factor: FloatRange::gain_skew_factor(-48.0, 12.0) },
            )
            .with_smoother(SmoothingStyle::Logarithmic(SMOOTH_MS))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(1))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
            spread: knob("Spread", 0.0),
            whine: knob("Whine", 0.0),

            agitation_speed: knob("Agitation Speed", 0.25)
                .with_value_to_string(Arc::new(|v| format!("{:.2} Hz", speed_hz(v))))
                .with_string_to_value(Arc::new(|text| leading_number(text).map(speed_knob))),
            agitation_angle: knob("Agitation Angle", 0.1),
            agitation_to_time: send("Agitation > Time", 0.0),
            agitation_to_filter: send("Agitation > Filter", 0.25),
            agitation_to_decay: send("Agitation > Decay", 0.0),
            agitation_to_absorb: send("Agitation > Absorb", 0.0),
            agitation_to_whine: send("Agitation > Whine", 0.0),

            interference_to_time: send("Interference > Time", 0.0),
            interference_to_filter: send("Interference > Filter", 0.0),
            interference_to_decay: send("Interference > Decay", 0.0),
            interference_to_absorb: send("Interference > Absorb", 0.0),
            interference_to_whine: send("Interference > Whine", 0.0),
        }
    }
}

impl AntimemeParams {
    /// Advance the smoothers one sample and read every control.
    pub fn next_controls(&self) -> Controls {
        Controls {
            time: self.time.smoothed.next(),
            decay: self.decay.smoothed.next(),
            filter: self.filter.smoothed.next(),
            absorb: self.absorb.smoothed.next(),
            blend: self.blend.smoothed.next(),
            strength: self.strength.smoothed.next(),
            level: self.level.smoothed.next(),
            spread: self.spread.smoothed.next(),
            whine: self.whine.smoothed.next(),
            agitation_speed: self.agitation_speed.smoothed.next(),
            agitation_angle: self.agitation_angle.smoothed.next(),
            agitation_to_time: self.agitation_to_time.smoothed.next(),
            agitation_to_filter: self.agitation_to_filter.smoothed.next(),
            agitation_to_decay: self.agitation_to_decay.smoothed.next(),
            agitation_to_absorb: self.agitation_to_absorb.smoothed.next(),
            agitation_to_whine: self.agitation_to_whine.smoothed.next(),
            interference_to_time: self.interference_to_time.smoothed.next(),
            interference_to_filter: self.interference_to_filter.smoothed.next(),
            interference_to_decay: self.interference_to_decay.smoothed.next(),
            interference_to_absorb: self.interference_to_absorb.smoothed.next(),
            interference_to_whine: self.interference_to_whine.smoothed.next(),
        }
    }
}
