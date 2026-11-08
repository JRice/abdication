//! The host parameters.

use std::sync::Arc;

use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;

use crate::dsp::engine::{MAX_HZ, MIN_HZ};
use crate::dsp::machine::Knobs;
use crate::dsp::program::{PROGRAM_COUNT, PROGRAM_NAMES};
use crate::dsp::ram::SWITCHES;

const CLOCK_SMOOTH_MS: f32 = 90.0;
const LEVEL_SMOOTH_MS: f32 = 20.0;
const MATRIX_SIDE: usize = 4;
/// A clock where the fun begins, about a quarter of stock.
const DEFAULT_CLOCK_HZ: f32 = 7_800.0;

/// The Reclock knob is logarithmic in clock speed.
pub fn clock_hz(knob: f32) -> f32 {
    MIN_HZ * (MAX_HZ / MIN_HZ).powf(knob)
}

pub fn clock_knob(hz: f32) -> f32 {
    ((hz / MIN_HZ).ln() / (MAX_HZ / MIN_HZ).ln()).clamp(0.0, 1.0)
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
        .with_value_to_string(Arc::new(percent))
        .with_string_to_value(Arc::new(|text| leading_number(text).map(|v| v / 100.0)))
}

fn gain(name: &str, min_db: f32, max_db: f32) -> FloatParam {
    FloatParam::new(
        name,
        1.0,
        FloatRange::Skewed { min: util::db_to_gain(min_db), max: util::db_to_gain(max_db), factor: FloatRange::gain_skew_factor(min_db, max_db) },
    )
    .with_smoother(SmoothingStyle::Logarithmic(LEVEL_SMOOTH_MS))
    .with_unit(" dB")
    .with_value_to_string(formatters::v2s_f32_gain_to_db(1))
    .with_string_to_value(formatters::s2v_f32_gain_to_db())
}

/// One of the sixteen switches on the bus.
#[derive(Params)]
pub struct Switch {
    #[id = "on"]
    pub on: BoolParam,
}

#[derive(Params)]
pub struct AnathemaParams {
    #[persist = "editor-state"]
    pub editor_state: Arc<ViziaState>,

    #[id = "reclock"]
    pub reclock: FloatParam,
    #[id = "program"]
    pub program: IntParam,
    #[id = "decay"]
    pub decay: FloatParam,
    #[id = "damp"]
    pub damp: FloatParam,
    #[id = "predelay"]
    pub predelay: FloatParam,
    #[id = "mix"]
    pub mix: FloatParam,
    #[id = "direct"]
    pub direct: FloatParam,
    #[id = "drive"]
    pub drive: FloatParam,
    #[id = "whine"]
    pub whine: FloatParam,
    #[id = "rot"]
    pub rot: FloatParam,
    #[id = "contention"]
    pub contention: FloatParam,
    #[id = "level"]
    pub level: FloatParam,
    #[id = "loop"]
    pub loop_ram: BoolParam,
    #[nested(array, group = "Matrix")]
    pub matrix: [Switch; SWITCHES],
}

impl Default for AnathemaParams {
    fn default() -> Self {
        Self {
            editor_state: crate::editor::default_state(),
            reclock: FloatParam::new("Reclock", clock_knob(DEFAULT_CLOCK_HZ), FloatRange::Linear { min: 0.0, max: 1.0 })
                .with_smoother(SmoothingStyle::Linear(CLOCK_SMOOTH_MS))
                .with_value_to_string(Arc::new(|v| format!("{:.0} Hz", clock_hz(v))))
                .with_string_to_value(Arc::new(|text| leading_number(text).map(clock_knob))),
            program: IntParam::new("Program", 0, IntRange::Linear { min: 0, max: PROGRAM_COUNT as i32 - 1 })
                .with_value_to_string(Arc::new(|v| PROGRAM_NAMES[v.clamp(0, PROGRAM_COUNT as i32 - 1) as usize].to_string()))
                .with_string_to_value(Arc::new(|text| PROGRAM_NAMES.iter().position(|name| name.eq_ignore_ascii_case(text.trim())).map(|i| i as i32))),
            decay: knob("Decay", 0.65),
            damp: knob("Damp", 0.3),
            predelay: knob("Pre Delay", 0.1),
            mix: knob("Mix", 0.5),
            direct: knob("Direct", 0.0),
            drive: gain("Drive", -12.0, 24.0),
            whine: knob("Whine", 0.0),
            rot: knob("Rot", 0.0),
            contention: knob("Contention", 0.0),
            level: gain("Level", -48.0, 12.0),
            loop_ram: BoolParam::new("RAM Loop", false),
            matrix: std::array::from_fn(|i| Switch {
                on: BoolParam::new(format!("Switch {}.{}", i / MATRIX_SIDE + 1, i % MATRIX_SIDE + 1), false),
            }),
        }
    }
}

impl AnathemaParams {
    /// What the machine only needs to hear about once a block.
    pub fn knobs(&self) -> Knobs {
        Knobs {
            program: self.program.value() as usize,
            decay: self.decay.value(),
            damp: self.damp.value(),
            predelay: self.predelay.value(),
            mix: self.mix.value(),
            matrix: std::array::from_fn(|i| self.matrix[i].on.value()),
            contention: self.contention.value(),
            loop_ram: self.loop_ram.value(),
            rot: self.rot.value(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_clock_knob_round_trips() {
        for hz in [MIN_HZ, 1_414.0, 7_797.0, 31_250.0, MAX_HZ] {
            assert!((clock_hz(clock_knob(hz)) - hz).abs() < hz * 1e-3, "{hz}");
        }
    }

    #[test]
    fn typed_values_parse_with_or_without_units() {
        assert_eq!(leading_number("1414 Hz"), Some(1414.0));
        assert_eq!(leading_number(" 35 "), Some(35.0));
    }
}
