//! The host parameters.

use std::sync::Arc;

use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;

use crate::dsp::engine::{Controls, StripKnobs};
use crate::dsp::wear::CHANNELS;

const SMOOTH_MS: f32 = 30.0;
/// The Tune knob's reach either way. It's cubed, so the fine distinctions near zero, where
/// the interesting things happen, get most of its travel.
const MOST_SHIFT_HZ: f32 = 60.0;

pub fn shift_hz(knob: f32) -> f32 {
    MOST_SHIFT_HZ * knob * knob * knob
}

fn shift_knob(hz: f32) -> f32 {
    (hz / MOST_SHIFT_HZ).clamp(-1.0, 1.0).cbrt()
}

/// Signed, except for zero, which is neither.
fn shift_label(hz: f32) -> String {
    let rounded = (hz * 10.0).round() / 10.0;
    if rounded == 0.0 { "0.0 Hz".to_string() } else { format!("{rounded:+.1} Hz") }
}

fn percent(value: f32) -> String {
    format!("{:.0} %", value * 100.0)
}

/// The number in a typed value: units are optional.
fn leading_number(text: &str) -> Option<f32> {
    text.trim().trim_end_matches(|c: char| !c.is_ascii_digit()).parse().ok()
}

fn knob(name: &str, default: f32) -> FloatParam {
    FloatParam::new(name, default, FloatRange::Linear { min: 0.0, max: 1.0 })
        .with_smoother(SmoothingStyle::Linear(SMOOTH_MS))
        .with_value_to_string(Arc::new(percent))
        .with_string_to_value(Arc::new(|text| leading_number(text).map(|v| v / 100.0)))
}

/// A value best heard on a log scale.
fn scaled(name: String, default: f32, min: f32, max: f32, unit: &'static str, decimals: usize) -> FloatParam {
    FloatParam::new(name, default, FloatRange::Skewed { min, max, factor: FloatRange::skew_factor(-1.6) })
        .with_smoother(SmoothingStyle::Logarithmic(SMOOTH_MS))
        .with_value_to_string(Arc::new(move |v| format!("{v:.decimals$}{unit}")))
        .with_string_to_value(Arc::new(leading_number))
}

fn gain(name: String, default_db: f32, min_db: f32, max_db: f32) -> FloatParam {
    FloatParam::new(
        name,
        util::db_to_gain(default_db),
        FloatRange::Skewed { min: util::db_to_gain(min_db), max: util::db_to_gain(max_db), factor: FloatRange::gain_skew_factor(min_db, max_db) },
    )
    .with_smoother(SmoothingStyle::Logarithmic(SMOOTH_MS))
    .with_unit(" dB")
    .with_value_to_string(formatters::v2s_f32_gain_to_db(1))
    .with_string_to_value(formatters::s2v_f32_gain_to_db())
}

#[derive(Params)]
pub struct StripParams {
    #[id = "tune"]
    pub tune: FloatParam,
    #[id = "peak"]
    pub peak: FloatParam,
    #[id = "gain"]
    pub gain: FloatParam,
}

/// (tune Hz, peak Hz)
const STRIP_DEFAULTS: [(f32, f32); CHANNELS] = [(1.3, 420.0), (-0.7, 1_300.0), (3.1, 2_900.0), (-1.9, 680.0)];
const STRIP_NAMES: [&str; CHANNELS] = ["A", "B", "C", "D"];
const STRIP_GAIN_DB: f32 = 1.6;

impl StripParams {
    fn new(channel: usize) -> Self {
        let (tune, peak) = STRIP_DEFAULTS[channel];
        let name = STRIP_NAMES[channel];
        Self {
            tune: FloatParam::new(format!("{name} Tune"), shift_knob(tune), FloatRange::Linear { min: -1.0, max: 1.0 })
                .with_smoother(SmoothingStyle::Linear(SMOOTH_MS))
                .with_value_to_string(Arc::new(|v| shift_label(shift_hz(v))))
                .with_string_to_value(Arc::new(|text| text.trim().trim_end_matches(|c: char| !c.is_ascii_digit()).parse().ok().map(shift_knob))),
            peak: scaled(format!("{name} Peak"), peak, 60.0, 9_000.0, " Hz", 0),
            gain: gain(format!("{name} Gain"), STRIP_GAIN_DB, -12.0, 12.0),
        }
    }

    fn knobs(&self) -> StripKnobs {
        StripKnobs { shift_hz: shift_hz(self.tune.smoothed.next()), peak_hz: self.peak.smoothed.next(), gain: self.gain.smoothed.next() }
    }
}

#[derive(Params)]
pub struct AreciboParams {
    #[persist = "editor-state"]
    pub editor_state: Arc<ViziaState>,

    #[nested(array, group = "Strip")]
    pub strips: [StripParams; CHANNELS],
    #[id = "feedback"]
    pub feedback: FloatParam,
    #[id = "cross"]
    pub cross: FloatParam,
    #[id = "sky"]
    pub sky: FloatParam,
    #[id = "width"]
    pub width: FloatParam,
    #[id = "distance"]
    pub distance: FloatParam,
    #[id = "agc"]
    pub agc: FloatParam,
    #[id = "weather"]
    pub weather: FloatParam,
    #[id = "age"]
    pub age: FloatParam,
    #[id = "input"]
    pub input: FloatParam,
    #[id = "side_input"]
    pub side_input: FloatParam,
    #[id = "dry"]
    pub dry: FloatParam,
    #[id = "level"]
    pub level: FloatParam,
}

impl Default for AreciboParams {
    fn default() -> Self {
        Self {
            editor_state: crate::editor::default_state(),
            strips: std::array::from_fn(StripParams::new),
            feedback: FloatParam::new("Feedback", 0.9, FloatRange::Linear { min: 0.0, max: 1.6 })
                .with_smoother(SmoothingStyle::Linear(SMOOTH_MS))
                .with_value_to_string(Arc::new(percent))
                .with_string_to_value(Arc::new(|text| leading_number(text).map(|v| v / 100.0))),
            cross: knob("Cross", 0.35),
            sky: knob("Sky", 0.7),
            width: scaled("Width".into(), 2_800.0, 300.0, 12_000.0, " Hz", 0),
            distance: scaled("Distance".into(), 28.0, 1.0, 160.0, " ms", 1),
            agc: scaled("AGC".into(), 0.7, 0.05, 3.0, " s", 2),
            weather: knob("Weather", 0.35),
            age: knob("Age", 0.4),
            input: gain("Input".into(), 0.0, -48.0, 24.0),
            side_input: gain("Side In".into(), 0.0, -48.0, 24.0),
            dry: knob("Dry", 0.0),
            level: gain("Level".into(), 0.0, -48.0, 12.0),
        }
    }
}

impl AreciboParams {
    /// Everything, advancing the smoothers one sample.
    pub fn next_controls(&self) -> Controls {
        Controls {
            strips: std::array::from_fn(|i| self.strips[i].knobs()),
            feedback: self.feedback.smoothed.next(),
            cross: self.cross.smoothed.next(),
            sky: self.sky.smoothed.next(),
            width_hz: self.width.smoothed.next(),
            distance_s: self.distance.smoothed.next() / 1_000.0,
            agc_release_s: self.agc.smoothed.next(),
            weather: self.weather.smoothed.next(),
            age: self.age.smoothed.next(),
            input: self.input.smoothed.next(),
            side_input: self.side_input.smoothed.next(),
            dry: self.dry.smoothed.next(),
            level: self.level.smoothed.next(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_tune_knob_round_trips_and_is_fine_near_zero() {
        for hz in [-60.0, -3.1, 0.0, 0.7, 40.0] {
            assert!((shift_hz(shift_knob(hz)) - hz).abs() < 0.01, "{hz}");
        }
        assert!(shift_hz(0.25) < 1.0);
    }

    #[test]
    fn zero_has_no_sign() {
        assert_eq!(shift_label(-0.01), "0.0 Hz");
        assert_eq!(shift_label(1.26), "+1.3 Hz");
        assert_eq!(shift_label(-0.7), "-0.7 Hz");
    }
}
