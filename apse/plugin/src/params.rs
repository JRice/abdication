//! The host parameters.

use std::sync::Arc;

use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;

use crate::dsp::choir::{vowel_name, ChoirControls};
use crate::dsp::engine::{Controls, PREDELAY_MAX_S, SIZE_MAX, SIZE_MIN, TIME_MAX_S, TIME_MIN_S};
use crate::dsp::modes::{MODE_COUNT, MODE_NAMES};

const FAST_SMOOTH_MS: f32 = 20.0;
/// Size and Pre Delay move lines that are being read, which bends pitch: slowly does it.
const LINE_SMOOTH_MS: f32 = 300.0;

/// The number in a typed value: units are optional.
fn leading_number(text: &str) -> Option<f32> {
    text.trim().trim_end_matches(|c: char| !c.is_ascii_digit()).parse().ok()
}

fn percent(value: f32) -> String {
    format!("{:.0} %", value * 100.0)
}

/// A 0–1 knob shown in percent.
fn knob(name: &str, default: f32) -> FloatParam {
    ranged_percent(name, default, 0.0, 1.0)
}

fn ranged_percent(name: &str, default: f32, min: f32, max: f32) -> FloatParam {
    FloatParam::new(name, default, FloatRange::Linear { min, max })
        .with_smoother(SmoothingStyle::Linear(FAST_SMOOTH_MS))
        .with_value_to_string(Arc::new(percent))
        .with_string_to_value(Arc::new(|text| leading_number(text).map(|v| v / 100.0)))
}

/// Shows seconds as seconds until they're short enough to read better in ms.
pub fn format_seconds(s: f32) -> String {
    match s {
        s if s < 1.0 => format!("{:.0} ms", s * 1000.0),
        s if s < 10.0 => format!("{s:.2} s"),
        s => format!("{s:.1} s"),
    }
}

/// Typed times: "2.5" or "2.5 s" is seconds, "300 ms" is milliseconds.
fn parse_seconds(text: &str) -> Option<f32> {
    let number = leading_number(text)?;
    Some(if text.trim_end().ends_with("ms") { number / 1000.0 } else { number })
}

fn seconds(name: &str, default: f32, min: f32, max: f32, skew: f32) -> FloatParam {
    FloatParam::new(name, default, FloatRange::Skewed { min, max, factor: FloatRange::skew_factor(skew) })
        .with_value_to_string(Arc::new(format_seconds))
        .with_string_to_value(Arc::new(parse_seconds))
}

fn gain(name: &str, min_db: f32, max_db: f32) -> FloatParam {
    FloatParam::new(
        name,
        1.0,
        FloatRange::Skewed { min: util::db_to_gain(min_db), max: util::db_to_gain(max_db), factor: FloatRange::gain_skew_factor(min_db, max_db) },
    )
    .with_smoother(SmoothingStyle::Logarithmic(FAST_SMOOTH_MS))
    .with_unit(" dB")
    .with_value_to_string(formatters::v2s_f32_gain_to_db(1))
    .with_string_to_value(formatters::s2v_f32_gain_to_db())
}

/// A typed vowel: AH, OH or OO, or a number in percent along them.
fn parse_vowel(text: &str) -> Option<f32> {
    let text = text.trim().to_uppercase();
    match text.as_str() {
        "AH" => Some(0.0),
        "OH" => Some(0.5),
        "OO" => Some(1.0),
        _ => leading_number(&text).map(|v| v / 100.0),
    }
}

fn color_name(v: f32) -> String {
    match v {
        v if v < -0.005 => format!("Dark {:.0} %", -v * 100.0),
        v if v > 0.005 => format!("Bright {:.0} %", v * 100.0),
        _ => "Flat".to_string(),
    }
}

#[derive(Params)]
pub struct ApseParams {
    #[persist = "editor-state"]
    pub editor_state: Arc<ViziaState>,

    #[id = "mode"]
    pub mode: IntParam,
    #[id = "time"]
    pub time: FloatParam,
    #[id = "size"]
    pub size: FloatParam,
    #[id = "predelay"]
    pub predelay: FloatParam,
    #[id = "diffusion"]
    pub diffusion: FloatParam,
    #[id = "mod"]
    pub modulation: FloatParam,
    #[id = "damp"]
    pub damp: FloatParam,
    #[id = "color"]
    pub color: FloatParam,
    #[id = "width"]
    pub width: FloatParam,
    #[id = "mix"]
    pub mix: FloatParam,
    #[id = "level"]
    pub level: FloatParam,
    #[id = "freeze"]
    pub freeze: BoolParam,

    // The Choir mode's own controls.
    #[id = "vowel"]
    pub vowel: FloatParam,
    #[id = "drift"]
    pub drift: FloatParam,
    #[id = "resonance"]
    pub resonance: FloatParam,
    #[id = "ensemble"]
    pub ensemble: FloatParam,
    #[id = "breath"]
    pub breath: FloatParam,
}

impl Default for ApseParams {
    fn default() -> Self {
        let d = Controls::default();
        Self {
            editor_state: crate::editor::default_state(),
            mode: IntParam::new("Mode", 0, IntRange::Linear { min: 0, max: MODE_COUNT as i32 - 1 })
                .with_value_to_string(Arc::new(|v| MODE_NAMES[v.clamp(0, MODE_COUNT as i32 - 1) as usize].to_string()))
                .with_string_to_value(Arc::new(|text| MODE_NAMES.iter().position(|name| name.eq_ignore_ascii_case(text.trim())).map(|i| i as i32))),
            time: seconds("Time", d.time_s, TIME_MIN_S, TIME_MAX_S, -2.0).with_smoother(SmoothingStyle::Logarithmic(FAST_SMOOTH_MS)),
            size: ranged_percent("Size", d.size, SIZE_MIN, SIZE_MAX).with_smoother(SmoothingStyle::Logarithmic(LINE_SMOOTH_MS)),
            predelay: seconds("Pre Delay", d.predelay_s, 0.0, PREDELAY_MAX_S, -1.0).with_smoother(SmoothingStyle::Linear(LINE_SMOOTH_MS)),
            diffusion: knob("Diffusion", d.diffusion),
            modulation: knob("Mod", d.modulation),
            damp: knob("Damp", d.damp),
            color: FloatParam::new("Color", d.color, FloatRange::Linear { min: -1.0, max: 1.0 })
                .with_smoother(SmoothingStyle::Linear(FAST_SMOOTH_MS))
                .with_value_to_string(Arc::new(color_name))
                .with_string_to_value(Arc::new(|text| leading_number(text).map(|v| v / 100.0))),
            width: ranged_percent("Width", d.width, 0.0, 2.0),
            mix: knob("Mix", d.mix),
            level: gain("Level", -24.0, 12.0),
            freeze: BoolParam::new("Freeze", false),
            vowel: knob("Vowel", d.choir.vowel)
                .with_smoother(SmoothingStyle::Linear(LINE_SMOOTH_MS))
                .with_value_to_string(Arc::new(vowel_name))
                .with_string_to_value(Arc::new(parse_vowel)),
            drift: knob("Drift", d.choir.drift),
            resonance: knob("Resonance", d.choir.resonance),
            ensemble: knob("Ensemble", d.choir.ensemble),
            breath: knob("Breath", d.choir.breath),
        }
    }
}

impl ApseParams {
    /// This sample's controls; advances the smoothers, so call it once per sample.
    pub fn controls(&self) -> Controls {
        Controls {
            time_s: self.time.smoothed.next(),
            size: self.size.smoothed.next(),
            predelay_s: self.predelay.smoothed.next(),
            diffusion: self.diffusion.smoothed.next(),
            modulation: self.modulation.smoothed.next(),
            damp: self.damp.smoothed.next(),
            color: self.color.smoothed.next(),
            width: self.width.smoothed.next(),
            mix: self.mix.smoothed.next(),
            level: self.level.smoothed.next(),
            freeze: self.freeze.value(),
            choir: ChoirControls {
                vowel: self.vowel.smoothed.next(),
                drift: self.drift.smoothed.next(),
                resonance: self.resonance.smoothed.next(),
                ensemble: self.ensemble.smoothed.next(),
                breath: self.breath.smoothed.next(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typed_times_understand_units() {
        assert_eq!(parse_seconds("2.5"), Some(2.5));
        assert_eq!(parse_seconds("2.5 s"), Some(2.5));
        assert_eq!(parse_seconds("300 ms"), Some(0.3));
    }

    #[test]
    fn times_read_naturally() {
        assert_eq!(format_seconds(0.02), "20 ms");
        assert_eq!(format_seconds(2.5), "2.50 s");
        assert_eq!(format_seconds(12.34), "12.3 s");
    }

    #[test]
    fn vowels_can_be_typed() {
        assert_eq!(parse_vowel("oh"), Some(0.5));
        assert_eq!(parse_vowel("25"), Some(0.25));
    }

    #[test]
    fn color_reads_both_ways() {
        assert_eq!(color_name(-0.4), "Dark 40 %");
        assert_eq!(color_name(0.0), "Flat");
        assert_eq!(leading_number("-40"), Some(-40.0));
    }
}
