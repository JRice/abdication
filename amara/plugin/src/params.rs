//! The host parameters.

use std::sync::Arc;

use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;

use crate::dsp::engine::Controls;
use crate::dsp::language::DEFAULT_STRANGENESS;
use crate::dsp::speaker::SpeakerKnobs;

const LEVEL_SMOOTH_MS: f32 = 20.0;
const LANGUAGES: i32 = 1_000;
const STRANGENESS_STEPS: f32 = 50.0;
const DEVILISH_LANGUAGE: i32 = 666;
const DEVILISH_POSSESSION: f32 = 0.66;
/// A language with a good throat on it, which calls itself tɑχ.
const DEFAULT_LANGUAGE: i32 = 512;

fn percent(value: f32) -> String {
    format!("{:.0} %", value * 100.0)
}

/// The number in a typed value: units are optional.
fn leading_number(text: &str) -> Option<f32> {
    text.trim().trim_end_matches(|c: char| !c.is_ascii_digit()).parse().ok()
}

fn knob(name: &str, default: f32) -> FloatParam {
    FloatParam::new(name, default, FloatRange::Linear { min: 0.0, max: 1.0 })
        .with_value_to_string(Arc::new(percent))
        .with_string_to_value(Arc::new(|text| leading_number(text).map(|v| v / 100.0)))
}

/// A value that's best heard on a log scale.
fn scaled(name: &str, default: f32, min: f32, max: f32, unit: &'static str, decimals: usize) -> FloatParam {
    FloatParam::new(name, default, FloatRange::Skewed { min, max, factor: FloatRange::skew_factor(-1.2) })
        .with_value_to_string(Arc::new(move |v| format!("{v:.decimals$}{unit}")))
        .with_string_to_value(Arc::new(leading_number))
}

fn possession_for(language: i32, knob: f32) -> f32 {
    if language == DEVILISH_LANGUAGE { knob.max(DEVILISH_POSSESSION) } else { knob }
}

#[derive(Params)]
pub struct AmaraParams {
    #[persist = "editor-state"]
    pub editor_state: Arc<ViziaState>,

    #[id = "language"]
    pub language: IntParam,
    #[id = "strangeness"]
    pub strangeness: FloatParam,
    #[id = "rate"]
    pub rate: FloatParam,
    #[id = "sparsity"]
    pub sparsity: FloatParam,
    #[id = "temper"]
    pub temper: FloatParam,
    #[id = "swing"]
    pub swing: FloatParam,
    #[id = "breath"]
    pub breath: FloatParam,
    #[id = "throat"]
    pub throat: FloatParam,
    #[id = "pitch"]
    pub pitch: FloatParam,
    #[id = "possession"]
    pub possession: FloatParam,
    #[id = "extremity"]
    pub extremity: FloatParam,
    #[id = "line"]
    pub line: FloatParam,
    #[id = "fidelity"]
    pub fidelity: FloatParam,
    #[id = "level"]
    pub level: FloatParam,
    #[id = "seize"]
    pub seize: BoolParam,
    #[id = "gated"]
    pub per_note: BoolParam,
}

impl Default for AmaraParams {
    fn default() -> Self {
        Self {
            editor_state: crate::editor::default_state(),
            language: IntParam::new("Language", DEFAULT_LANGUAGE, IntRange::Linear { min: 0, max: LANGUAGES - 1 }),
            strangeness: knob("Strangeness", DEFAULT_STRANGENESS),
            rate: scaled("Rate", 5.0, 1.0, 16.0, " syl/s", 1),
            sparsity: knob("Sparsity", 0.2),
            temper: knob("Temper", 0.45),
            swing: knob("Swing", 0.5),
            breath: knob("Breath", 0.1),
            throat: scaled("Throat", 1.2, 0.6, 2.0, " x", 2),
            pitch: scaled("Pitch", 90.0, 40.0, 400.0, " Hz", 0),
            possession: knob("Possession", 0.35),
            extremity: knob("Extremity", 0.6),
            line: knob("Line", 0.05),
            fidelity: knob("Fidelity", 0.55),
            level: FloatParam::new(
                "Level",
                util::db_to_gain(-3.0),
                FloatRange::Skewed { min: util::db_to_gain(-48.0), max: util::db_to_gain(12.0), factor: FloatRange::gain_skew_factor(-48.0, 12.0) },
            )
            .with_smoother(SmoothingStyle::Logarithmic(LEVEL_SMOOTH_MS))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(1))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
            seize: BoolParam::new("Seize", false),
            per_note: BoolParam::new("One Syllable per Note", false),
        }
    }
}

impl AmaraParams {
    /// In steps, since every change builds a whole new language.
    pub fn strangeness(&self) -> f32 {
        (self.strangeness.unmodulated_plain_value() * STRANGENESS_STEPS).round() / STRANGENESS_STEPS
    }

    /// Whether the chosen language is the one that wants more than you offered.
    pub fn devilish(&self) -> bool {
        self.language.unmodulated_plain_value() == DEVILISH_LANGUAGE
    }

    /// What Possession really is: the knob, unless the language won't have it. The knob
    /// keeps its own setting for when you leave.
    pub fn possession(&self) -> f32 {
        possession_for(self.language.value(), self.possession.value())
    }

    /// Everything but the level, once a block. `note_hz` overrides the Pitch knob.
    pub fn controls(&self, note_hz: Option<f32>) -> Controls {
        Controls {
            speaker: SpeakerKnobs {
                rate: self.rate.value(),
                sparsity: self.sparsity.value(),
                temper: self.temper.value(),
                swing: self.swing.value(),
                breath: self.breath.value(),
                tract: 1.0 / self.throat.value(),
                pitch_hz: note_hz.unwrap_or(self.pitch.value()),
                per_note: self.per_note.value(),
            },
            language: self.language.value() as u32,
            strangeness: self.strangeness(),
            possession: self.possession(),
            extremity: self.extremity.value(),
            seized: self.seize.value(),
            line: self.line.value(),
            fidelity: self.fidelity.value(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn language_666_holds_possession_at_its_floor() {
        assert_eq!(possession_for(666, 0.1), DEVILISH_POSSESSION);
        assert_eq!(possession_for(666, 0.9), 0.9);
    }

    #[test]
    fn every_other_language_leaves_the_knob_alone() {
        assert_eq!(possession_for(665, 0.1), 0.1);
        assert_eq!(possession_for(667, 0.1), 0.1);
    }
}
