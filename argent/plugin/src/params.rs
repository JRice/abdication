//! The host parameters.

use std::sync::Arc;

use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;

use crate::dsp::engine::Controls;
use crate::dsp::osc::Reading;
use crate::dsp::tables::TABLES;

const SMOOTH_MS: f32 = 30.0;
/// SCAN moves through whole tables, so it eases further.
const SCAN_SMOOTH_MS: f32 = 120.0;
const FIZZ_OFF_DB: f32 = -48.0;
pub const SEEDS: i32 = 10_000;
pub const DEFAULT_SEED: i32 = 417;
const SPAN_NAMES: [&str; 3] = ["8ve", "12th", "15th"];

/// The number in a typed value: units are optional.
fn leading_number(text: &str) -> Option<f32> {
    let t = text.trim().trim_start_matches(['±', '+', 'x', 'N', 'o', '.', ' ']);
    let end = t.find(|c: char| !(c.is_ascii_digit() || c == '.' || c == '-')).unwrap_or(t.len());
    t[..end].parse().ok()
}

fn linear(name: &str, default: f32, min: f32, max: f32, smooth_ms: f32) -> FloatParam {
    FloatParam::new(name, default, FloatRange::Linear { min, max }).with_smoother(SmoothingStyle::Linear(smooth_ms))
}

fn shown(param: FloatParam, show: impl Fn(f32) -> String + Send + Sync + 'static) -> FloatParam {
    param.with_value_to_string(Arc::new(show)).with_string_to_value(Arc::new(leading_number))
}

fn hz(v: f32) -> String {
    if v >= 1_000.0 {
        format!("{:.1} kHz", v / 1_000.0)
    } else {
        format!("{v:.0} Hz")
    }
}

fn seconds(v: f32) -> String {
    if v >= 10.0 {
        format!("{v:.0} s")
    } else {
        format!("{v:.1} s")
    }
}

/// Which harmonics SPAN has let in, by name.
pub fn span_text(span: f32) -> String {
    let names: Vec<&str> = SPAN_NAMES.iter().enumerate().filter(|(k, _)| span > *k as f32 / 3.0 + 1e-3).map(|(_, n)| *n).collect();
    if names.is_empty() {
        "note".into()
    } else {
        names.join(" · ")
    }
}

fn curve_text(v: f32) -> String {
    match (v * 10.0).round() as i32 {
        10 => "x".into(),
        20 => "x²".into(),
        30 => "x³".into(),
        40 => "x⁴".into(),
        _ => format!("x^{v:.1}"),
    }
}

#[derive(Params)]
pub struct ArgentParams {
    #[persist = "editor-state"]
    pub editor_state: Arc<ViziaState>,

    #[id = "morph"]
    pub morph: FloatParam,
    #[id = "scan"]
    pub scan: FloatParam,
    #[id = "seed"]
    pub seed: IntParam,
    #[id = "span"]
    pub span: FloatParam,
    #[id = "tone"]
    pub tone: FloatParam,
    #[id = "fizz"]
    pub fizz: FloatParam,
    #[id = "tide"]
    pub tide: FloatParam,
    #[id = "flare"]
    pub flare: FloatParam,
    #[id = "depth"]
    pub depth: FloatParam,
    #[id = "bloom"]
    pub bloom: FloatParam,
    #[id = "tremor"]
    pub tremor: FloatParam,
    #[id = "wander"]
    pub wander: FloatParam,
    #[id = "fade"]
    pub fade: FloatParam,
    #[id = "curve"]
    pub curve: FloatParam,
    #[id = "coarse"]
    pub coarse: BoolParam,
    #[id = "glide"]
    pub glide: BoolParam,
    #[id = "release"]
    pub release: BoolParam,
    #[id = "volume"]
    pub volume: FloatParam,
    /// How much of the eye shows. The panel's own, so not automatable.
    #[id = "dread"]
    pub dread: FloatParam,
}

impl Default for ArgentParams {
    fn default() -> Self {
        let last_table = (TABLES - 1) as f32;
        Self {
            editor_state: crate::editor::default_state(),
            morph: shown(linear("Morph", 0.3, 0.0, 1.0, SMOOTH_MS), |v| format!("{v:.2}")),
            scan: shown(linear("Scan", 1.0, 0.0, last_table, SCAN_SMOOTH_MS), move |v| format!("{:.1} / {TABLES}", v + 1.0))
                .with_string_to_value(Arc::new(|t| leading_number(t).map(|v| v - 1.0))),
            seed: IntParam::new("Seed", DEFAULT_SEED, IntRange::Linear { min: 0, max: SEEDS - 1 })
                .with_value_to_string(Arc::new(|v| format!("No. {v:04}")))
                .with_string_to_value(Arc::new(|t| leading_number(t).map(|v| v as i32))),
            span: shown(linear("Span", 0.62, 0.0, 1.0, SMOOTH_MS), span_text),
            tone: shown(
                FloatParam::new("Tone", 520.0, FloatRange::Skewed { min: 80.0, max: 12_000.0, factor: FloatRange::skew_factor(-2.0) })
                    .with_smoother(SmoothingStyle::Logarithmic(SMOOTH_MS)),
                hz,
            ),
            fizz: shown(linear("Fizz", -30.0, FIZZ_OFF_DB, 0.0, SMOOTH_MS), |v| {
                if v <= FIZZ_OFF_DB + 0.05 {
                    "off".into()
                } else {
                    format!("{v:.0} dB")
                }
            }),
            tide: shown(linear("Tide", 0.4, 0.0, 1.0, SMOOTH_MS), |v| format!("{:.0} %", v * 100.0))
                .with_string_to_value(Arc::new(|t| leading_number(t).map(|v| v / 100.0))),
            flare: shown(
                FloatParam::new("Flare", 9.0, FloatRange::Skewed { min: 2.0, max: 40.0, factor: FloatRange::skew_factor(-1.0) }),
                seconds,
            ),
            depth: shown(linear("Depth", 3.0, 0.0, 5.0, SMOOTH_MS), |v| format!("{v:.1} oct")),
            bloom: shown(linear("Bloom", 0.5, 0.0, 1.0, SMOOTH_MS), |v| format!("{:.0} %", v * 100.0))
                .with_string_to_value(Arc::new(|t| leading_number(t).map(|v| v / 100.0))),
            tremor: shown(linear("Tremor", 0.13, 0.0, 0.4, SMOOTH_MS), |v| format!("{:.0} %", v * 100.0))
                .with_string_to_value(Arc::new(|t| leading_number(t).map(|v| v / 100.0))),
            wander: shown(linear("Wander", 15.0, 0.0, 50.0, SMOOTH_MS), |v| format!("±{v:.0} ¢")),
            fade: shown(
                FloatParam::new("Fade", 20.0, FloatRange::Skewed { min: 0.2, max: 120.0, factor: FloatRange::skew_factor(-2.0) }),
                seconds,
            ),
            curve: shown(FloatParam::new("Curve", 2.0, FloatRange::Linear { min: 1.0, max: 4.0 }), curve_text),
            coarse: BoolParam::new("32 Frames", false),
            glide: BoolParam::new("Glide", false),
            release: BoolParam::new("Release", false),
            volume: FloatParam::new(
                "Volume",
                util::db_to_gain(-6.0),
                FloatRange::Skewed { min: util::db_to_gain(-36.0), max: util::db_to_gain(6.0), factor: FloatRange::gain_skew_factor(-36.0, 6.0) },
            )
            .with_smoother(SmoothingStyle::Logarithmic(SMOOTH_MS))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(0))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
            dread: FloatParam::new("Dread", 0.35, FloatRange::Linear { min: 0.0, max: 1.0 }).non_automatable(),
        }
    }
}

impl ArgentParams {
    /// Everything the engine needs, once a sample (the smoothed ones move on by one).
    pub fn next_controls(&self) -> Controls {
        let fizz_db = self.fizz.smoothed.next();
        Controls {
            morph: self.morph.smoothed.next(),
            scan: self.scan.smoothed.next(),
            span: self.span.smoothed.next(),
            tone_hz: self.tone.smoothed.next(),
            fizz: if fizz_db <= FIZZ_OFF_DB + 0.05 { 0.0 } else { util::db_to_gain(fizz_db) },
            tide: self.tide.smoothed.next(),
            flare_s: self.flare.value(),
            depth_oct: self.depth.smoothed.next(),
            bloom: self.bloom.smoothed.next(),
            tremor: self.tremor.smoothed.next(),
            wander_cents: self.wander.smoothed.next(),
            fade_s: self.fade.value(),
            curve: self.curve.value(),
            reading: Reading { stepped: !self.glide.value(), coarse: self.coarse.value() },
            volume: self.volume.smoothed.next(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn span_names_the_harmonics_it_lets_in() {
        assert_eq!(span_text(0.0), "note");
        assert_eq!(span_text(0.2), "8ve");
        assert_eq!(span_text(0.62), "8ve · 12th");
        assert_eq!(span_text(1.0), "8ve · 12th · 15th");
    }

    #[test]
    fn typed_values_are_understood() {
        assert_eq!(leading_number("700 Hz"), Some(700.0));
        assert_eq!(leading_number("±15 ¢"), Some(15.0));
        assert_eq!(leading_number("No. 0417"), Some(417.0));
        assert_eq!(leading_number("-12 dB"), Some(-12.0));
    }
}
