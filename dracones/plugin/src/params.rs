//! Host parameters. Every control on the panel has one, fully named (the panel itself has
//! no text), including the 100 arm depths ("II Cutoff ← Green").

use std::sync::Arc;

use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;

use crate::dsp::filters::{self, Character};
use crate::dsp::inserts::{mix_gains, DIVISION_NAMES, EFFECT_NAMES, MIX_DEFAULT, ORDERS};
use crate::dsp::{argon, fuzz, harmlib, inserts, mods, pitch, tables};
use crate::dsp::mods::COPIES;
use crate::engine::{Controls, RED_SLOPES, RED_TIMES, SLATE_HOLDS};
use crate::knobs::{self, ARMED, ARMED_COUNT, MOD_IDS, MOD_NAMES};

pub struct DraconesParams {
    pub editor_state: Arc<ViziaState>,
    pub knobs: Vec<FloatParam>,
    /// Per knob, per modulator.
    pub depths: Vec<[FloatParam; 4]>,
    pub filters: [EnumParam<FilterChoice>; 3],
    pub fizmo_table: IntParam,
    pub argon_table: IntParam,
    pub divisions: [IntParam; 3],
    pub order: IntParam,
    pub morph_shape: IntParam,
    pub morph_depth: FloatParam,
    /// Each delay's dry/wet Mix (per effect, travelling with its plaque).
    pub mixes: [FloatParam; 3],
    /// Red's and slate's three copies (I, II, III).
    pub red_time: [FloatParam; COPIES],
    pub red_slope: [FloatParam; COPIES],
    pub slate_rate: [IntParam; COPIES],
    pub output: FloatParam,
}

#[derive(Enum, Debug, PartialEq, Eq, Clone, Copy)]
pub enum FilterChoice {
    Wasp,
    #[name = "SVF"]
    Svf,
}

const NUMERALS: [&str; 3] = ["I", "II", "III"];

fn percent() -> Arc<dyn Fn(f32) -> String + Send + Sync> {
    Arc::new(|v| format!("{:.0} %", v * 100.0))
}

/// A percentage that reads "off" at zero.
fn percent_or_off() -> Arc<dyn Fn(f32) -> String + Send + Sync> {
    Arc::new(|v| if v <= 0.0 { "off".into() } else { format!("{:.0} %", v * 100.0) })
}

/// The copies' host ids: copy I keeps the first version's id, so saved sessions keep it.
fn copy_id(base: &str, k: usize) -> String {
    if k == 0 { base.into() } else { format!("{base}{}", k + 1) }
}

/// The Output slider: unity at 80 % of its travel, +6 dB at the top, silence at the bottom.
pub fn output_gain(v: f32) -> f32 {
    let v = v.clamp(0.0, 1.0);
    if v >= 0.8 { 10f32.powf(30.0 * (v - 0.8) / 20.0) } else { (v / 0.8).powi(3) }
}

/// Reverb Time knob → (T40 in seconds, how much of the reverb is heard): the bottom 8 %
/// fades it out entirely.
pub fn reverb_time(v: f32) -> (f32, f32) {
    const FADE: f32 = 0.08;
    let v = v.clamp(0.0, 1.0);
    let fade = (v / FADE).min(1.0);
    let t = 0.2 * 150f32.powf(((v - FADE) / (1.0 - FADE)).max(0.0));
    (t, fade)
}

/// Morph Slant knob → dB per octave of harmonics.
pub fn slant_db(v: f32) -> f32 {
    12.0 * (v.clamp(0.0, 1.0) - 0.5)
}

/// Gain knob → gain: off to unity, squared so the low end is quiet.
pub fn osc_gain(v: f32) -> f32 {
    let v = v.clamp(0.0, 1.0);
    v * v
}

/// How each armed knob reads back as text.
fn formatter(i: usize) -> Arc<dyn Fn(f32) -> String + Send + Sync> {
    use knobs::*;
    match i {
        _ if is_pitch(i) => Arc::new(|v| pitch::name(2.0 * v - 1.0)),
        _ if i < 15 && i % 5 == GAIN => Arc::new(|v| match osc_gain(v) {
            g if g <= 0.0 => "off".into(),
            g => format!("{:.1} dB", 20.0 * g.log10()),
        }),
        _ if i < 15 && i % 5 == CUTOFF => Arc::new(|v| format!("{:.0} Hz", filters::cutoff_hz(v))),
        _ if i < 15 && i % 5 == RESONANCE => Arc::new(|v| format!("Q {:.2}", filters::resonance_q(v))),
        _ if i == custom(inserts::PHASER) => Arc::new(|v| format!("{:.2} Hz", inserts::phaser_hz(v))),
        FUZZ_HP => Arc::new(|v| format!("{:.0} Hz", fuzz::hp_hz(v))),
        SLANT => Arc::new(|v| format!("{:+.1} dB/oct", slant_db(v))),
        REVERB_TIME => Arc::new(|v| match reverb_time(v) {
            (_, f) if f <= 0.0 => "off".into(),
            (t, f) if f < 1.0 => format!("{t:.1} s ({:.0} %)", f * 100.0),
            (t, _) => format!("{t:.1} s"),
        }),
        _ => percent(),
    }
}

impl Default for DraconesParams {
    fn default() -> Self {
        let knobs = (0..ARMED_COUNT)
            .map(|i| {
                FloatParam::new(ARMED[i].name, ARMED[i].default, FloatRange::Linear { min: 0.0, max: 1.0 })
                    .with_value_to_string(formatter(i))
            })
            .collect();
        let depths = (0..ARMED_COUNT)
            .map(|i| {
                std::array::from_fn(|m| {
                    FloatParam::new(format!("{} ← {}", ARMED[i].name, MOD_NAMES[m]), 0.0,
                                    FloatRange::Linear { min: 0.0, max: 1.0 })
                        .with_value_to_string(percent())
                })
            })
            .collect();
        let divisions_default = [1, 3, 5];
        Self {
            editor_state: crate::editor::default_state(),
            knobs,
            depths,
            filters: std::array::from_fn(|o| {
                EnumParam::new(format!("{} Filter", NUMERALS[o]), if o == 1 { FilterChoice::Svf } else { FilterChoice::Wasp })
            }),
            fizmo_table: IntParam::new("I Table", 3, IntRange::Linear { min: 0, max: tables::TABLES as i32 - 1 })
                .with_value_to_string(Arc::new(|t| match tables::table_name(t as usize) {
                    Some(name) => format!("{} ({name})", t + 1),
                    None => format!("{} (drawn)", t + 1),
                })),
            argon_table: IntParam::new("II Table", 1, IntRange::Linear { min: 0, max: argon::TABLES as i32 - 1 })
                .with_value_to_string(Arc::new(|t| format!("A8 {:02}–{:02}", 5 * t + 1, 5 * t + 5))),
            divisions: std::array::from_fn(|e| {
                IntParam::new(format!("{} Division", EFFECT_NAMES[e]), divisions_default[e],
                              IntRange::Linear { min: 0, max: DIVISION_NAMES.len() as i32 - 1 })
                    .with_value_to_string(Arc::new(|d| DIVISION_NAMES[d.clamp(0, 6) as usize].into()))
            }),
            order: IntParam::new("Insert Order", 0, IntRange::Linear { min: 0, max: ORDERS.len() as i32 - 1 })
                .with_value_to_string(Arc::new(|p| {
                    ORDERS[p.clamp(0, 5) as usize].map(|e| EFFECT_NAMES[e]).join(" · ")
                })),
            morph_shape: IntParam::new("Morph Shape", 0, IntRange::Linear { min: 0, max: harmlib::NAMES.len() as i32 })
                .with_value_to_string(Arc::new(|s| match s {
                    0 => "Off".into(),
                    s => harmlib::NAMES[(s as usize - 1).min(harmlib::NAMES.len() - 1)].into(),
                })),
            morph_depth: FloatParam::new("Morph Depth", 1.0, FloatRange::Linear { min: 0.0, max: 1.0 })
                .with_value_to_string(percent_or_off()),
            mixes: std::array::from_fn(|e| {
                FloatParam::new(format!("{} Delay Mix", EFFECT_NAMES[e]), MIX_DEFAULT, FloatRange::Linear { min: 0.0, max: 1.0 })
                    .with_value_to_string(Arc::new(|v| {
                        let (dry, wet) = mix_gains(v);
                        if wet <= 0.0 { "off".into() } else if dry <= 0.0 { "wet only".into() } else { format!("{:.0} %", v * 100.0) }
                    }))
            }),
            red_time: std::array::from_fn(|k| {
                FloatParam::new(format!("Red {} Time", NUMERALS[k]), RED_TIMES[k], FloatRange::Linear { min: 0.0, max: 1.0 })
                    .with_value_to_string(Arc::new(|v| format!("{:.2} Hz", mods::red_hz(v))))
            }),
            red_slope: std::array::from_fn(|k| {
                FloatParam::new(format!("Red {} Slope", NUMERALS[k]), RED_SLOPES[k], FloatRange::Linear { min: 0.0, max: 1.0 })
                    .with_value_to_string(percent())
            }),
            slate_rate: std::array::from_fn(|k| {
                IntParam::new(format!("Slate {} Hold", NUMERALS[k]), SLATE_HOLDS[k] as i32, IntRange::Linear { min: 0, max: 2 })
                    .with_value_to_string(Arc::new(|r| ["1/4", "1 bar", "4 bars"][r.clamp(0, 2) as usize].into()))
            }),
            output: FloatParam::new("Output", 0.8, FloatRange::Linear { min: 0.0, max: 1.0 })
                .with_value_to_string(Arc::new(|v| match output_gain(v) {
                    g if g <= 0.0 => "-inf dB".into(),
                    g => format!("{:+.1} dB", 20.0 * g.log10()),
                })),
        }
    }
}

impl DraconesParams {
    /// Everything the engine reads, now.
    pub fn controls(&self) -> Controls {
        Controls {
            knobs: std::array::from_fn(|i| self.knobs[i].value()),
            depths: std::array::from_fn(|i| std::array::from_fn(|m| self.depths[i][m].value())),
            filters: std::array::from_fn(|o| match self.filters[o].value() {
                FilterChoice::Wasp => Character::Wasp,
                FilterChoice::Svf => Character::Svf,
            }),
            tables: [self.fizmo_table.value() as usize, self.argon_table.value() as usize],
            divisions: std::array::from_fn(|e| self.divisions[e].value() as usize),
            order: self.order.value() as usize,
            morph_shape: self.morph_shape.value() as usize,
            morph_depth: self.morph_depth.value(),
            mixes: std::array::from_fn(|e| self.mixes[e].value()),
            red_time: std::array::from_fn(|k| self.red_time[k].value()),
            red_slope: std::array::from_fn(|k| self.red_slope[k].value()),
            slate_rate: std::array::from_fn(|k| self.slate_rate[k].value() as usize),
            output: self.output.value(),
        }
    }

    /// Every parameter with its id, for the host and the panel (ids are stable).
    pub fn entries(&self) -> Vec<(String, ParamPtr, String)> {
        let mut out = Vec::new();
        for (i, k) in ARMED.iter().enumerate() {
            let group = if i < 15 { "Oscillators" } else if i < 21 { "Inserts" } else { "Output" };
            out.push((k.id.to_string(), self.knobs[i].as_ptr(), group.to_string()));
        }
        for o in 0..3 {
            out.push((format!("o{o}_filter"), self.filters[o].as_ptr(), "Oscillators".into()));
        }
        out.push(("o0_table".into(), self.fizmo_table.as_ptr(), "Oscillators".into()));
        out.push(("o1_table".into(), self.argon_table.as_ptr(), "Oscillators".into()));
        for e in 0..3 {
            out.push((format!("f{e}_div"), self.divisions[e].as_ptr(), "Inserts".into()));
            out.push((format!("f{e}_mix"), self.mixes[e].as_ptr(), "Inserts".into()));
        }
        out.push(("order".into(), self.order.as_ptr(), "Inserts".into()));
        out.push(("p_shape".into(), self.morph_shape.as_ptr(), "Output".into()));
        out.push(("p_depth".into(), self.morph_depth.as_ptr(), "Output".into()));
        for k in 0..COPIES {
            out.push((copy_id("r_time", k), self.red_time[k].as_ptr(), "Modulators".into()));
            out.push((copy_id("r_slope", k), self.red_slope[k].as_ptr(), "Modulators".into()));
            out.push((copy_id("s_rate", k), self.slate_rate[k].as_ptr(), "Modulators".into()));
        }
        out.push(("out".into(), self.output.as_ptr(), "Output".into()));
        for (i, k) in ARMED.iter().enumerate() {
            for m in 0..4 {
                out.push((format!("{}.{}", k.id, MOD_IDS[m]), self.depths[i][m].as_ptr(), "Arms".into()));
            }
        }
        out
    }
}

unsafe impl Params for DraconesParams {
    fn param_map(&self) -> Vec<(String, ParamPtr, String)> {
        self.entries()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_are_unique_and_complete() {
        let p = DraconesParams::default();
        let e = p.entries();
        let mut ids: Vec<&String> = e.iter().map(|x| &x.0).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), e.len());
        // Knobs, filters, tables, divisions and mixes, order, shape and depth, the
        // copies' controls, output, arms.
        assert_eq!(e.len(), 25 + 3 + 2 + 3 + 3 + 1 + 2 + 9 + 1 + 100);
        assert!(ids.iter().any(|i| *i == "r_time") && ids.iter().any(|i| *i == "s_rate3"));
        assert_eq!(p.depths[8][2].name(), "II Cutoff ← Green");
    }

    #[test]
    fn output_law() {
        assert!((output_gain(0.8) - 1.0).abs() < 1e-6);
        assert!((20.0 * output_gain(1.0).log10() - 6.0).abs() < 0.01);
        assert_eq!(output_gain(0.0), 0.0);
        assert_eq!(reverb_time(0.0).1, 0.0);
        assert!((reverb_time(1.0).0 - 30.0).abs() < 0.01);
    }
}
