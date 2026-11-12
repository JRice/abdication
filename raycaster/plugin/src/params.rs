//! The host parameters. Every modulatable control is built from its range in
//! `dsp::targets`, so the engine moves it through exactly the travel the knob shows, and
//! each has its own dynamic modulator (`ModParams`).

use std::sync::Arc;

use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;

use crate::dsp::engine::{BlockControls, FilterKind, OscSetup};
use crate::dsp::engine::MAX_UNISON;
use crate::dsp::envelope::{Curves, Times, DEFAULT_ATTACK_BOW, DEFAULT_FALL_BOW};
use crate::dsp::modulator::{Clock, Division, LfoShape, ModSettings, DEFAULT_DECAY_BOW};
use crate::dsp::shapes::{Element, MAX_SEED};
use crate::dsp::targets::{
    global_range, global_target, osc_range, target_name, GlobalTarget, OscTarget, Range, GLOBAL_TARGETS, OSCS, OSC_TARGETS, ROMAN,
    TARGETS,
};

const SMOOTH_MS: f32 = 20.0;

/// The number in a typed value: units are optional.
fn leading_number(text: &str) -> Option<f32> {
    let t = text.trim().trim_start_matches(['+', 'x', '×', ' ']);
    let end = t.find(|c: char| !(c.is_ascii_digit() || c == '.' || c == '-' || c == '−')).unwrap_or(t.len());
    t[..end].replace('−', "-").parse().ok()
}

fn ranged(name: String, r: Range) -> FloatParam {
    let range = if r.factor == 1.0 {
        FloatRange::Linear { min: r.min, max: r.max }
    } else {
        FloatRange::Skewed { min: r.min, max: r.max, factor: r.factor }
    };
    FloatParam::new(name, r.default, range).with_smoother(SmoothingStyle::Linear(SMOOTH_MS))
}

fn shown(param: FloatParam, show: impl Fn(f32) -> String + Send + Sync + 'static) -> FloatParam {
    param.with_value_to_string(Arc::new(show)).with_string_to_value(Arc::new(leading_number))
}

fn percent(v: f32) -> String {
    format!("{:.0} %", v * 100.0)
}

fn signed(v: f32, digits: usize) -> String {
    let sign = if v > 0.0005 { "+" } else if v < -0.0005 { "−" } else { "" };
    format!("{sign}{:.*}", digits, v.abs())
}

pub fn hz(v: f32) -> String {
    if v >= 1_000.0 {
        format!("{:.1} kHz", v / 1_000.0)
    } else if v >= 10.0 {
        format!("{v:.0} Hz")
    } else {
        format!("{v:.2} Hz")
    }
}

pub fn seconds(v: f32) -> String {
    if v < 1.0 {
        format!("{:.0} ms", v * 1_000.0)
    } else {
        format!("{v:.2} s")
    }
}

/// The SVF's response: LP, BP, HP and the blends between.
fn mode_text(v: f32) -> String {
    match v {
        v if v < 0.02 => "LP".into(),
        v if (v - 0.5).abs() < 0.02 => "BP".into(),
        v if v > 0.98 => "HP".into(),
        v if v < 0.5 => format!("LP·BP {:.0}", v * 200.0),
        v => format!("BP·HP {:.0}", (v - 0.5) * 200.0),
    }
}

fn squish_text(v: f32) -> String {
    let ratio = 4f32.powf(v);
    if v.abs() < 0.005 {
        "none".into()
    } else if ratio > 1.0 {
        format!("{ratio:.2}× wide")
    } else {
        format!("{:.2}× tall", 1.0 / ratio)
    }
}

fn skew_text(v: f32) -> String {
    if v.abs() < 0.005 {
        "none".into()
    } else {
        format!("{}{:.0}°", if v < 0.0 { "−" } else { "" }, v.abs().atan().to_degrees())
    }
}

fn tilt_text(v: f32) -> String {
    if v.abs() < 0.005 {
        "none".into()
    } else {
        format!("{} −{:.0} %", if v > 0.0 { "top" } else { "base" }, 100.0 - 100.0 / (1.0 + 1.5 * v.abs()))
    }
}

/// How each modulatable control shows its value.
fn osc_format(k: OscTarget) -> fn(f32) -> String {
    match k {
        OscTarget::Fine => |v| format!("{} ¢", signed(v, 0)),
        OscTarget::Level => percent,
        OscTarget::Squish => squish_text,
        OscTarget::Skew => skew_text,
        OscTarget::Tilt => tilt_text,
        OscTarget::Center => |v| signed(v, 2),
        OscTarget::Ray => |v| if v < 0.005 { "on axis".into() } else { format!("{v:.2}") },
        OscTarget::Orbit => |v| if v < 0.005 { "off".into() } else { format!("{v:.2}") },
        OscTarget::OrbitRate => hz,
    }
}

#[derive(Params)]
pub struct OscParams {
    #[id = "el"]
    pub element: EnumParam<Element>,
    /// Which Chaos shape, when the element is Chaos. The panel rolls a new one on every click.
    #[id = "seed"]
    pub seed: IntParam,
    #[id = "oct"]
    pub octave: IntParam,
    #[id = "semi"]
    pub semi: IntParam,
    #[id = "fine"]
    pub fine: FloatParam,
    #[id = "lvl"]
    pub level: FloatParam,
    #[id = "squish"]
    pub squish: FloatParam,
    #[id = "skew"]
    pub skew: FloatParam,
    #[id = "tilt"]
    pub tilt: FloatParam,
    #[id = "center"]
    pub center: FloatParam,
    #[id = "ray"]
    pub ray: FloatParam,
    #[id = "orbit"]
    pub orbit: FloatParam,
    #[id = "ohz"]
    pub orbit_hz: FloatParam,
    /// A note starts the shape wherever it happens to be, rather than upright.
    #[id = "free"]
    pub free: BoolParam,
}

impl OscParams {
    fn new(index: usize) -> Self {
        let roman = ROMAN[index];
        let control = |k: OscTarget| {
            let mut r = osc_range(k);
            if k == OscTarget::Level && index > 0 {
                r.default = 0.0;
            }
            let name = if k == OscTarget::OrbitRate { "Orbit Rate" } else { crate::dsp::targets::osc_label(k) };
            shown(ranged(format!("{roman} {name}"), r), osc_format(k))
        };
        Self {
            element: EnumParam::new(format!("{roman} Element"), if index == 0 { Element::Earth } else { Element::Fire }),
            seed: IntParam::new(format!("{roman} Chaos Seed"), 4_821 + 2_000 * index as i32, IntRange::Linear { min: 1, max: MAX_SEED })
                .with_value_to_string(Arc::new(|v| format!("{v:05}")))
                .with_string_to_value(Arc::new(|t| leading_number(t).map(|v| v as i32))),
            octave: IntParam::new(format!("{roman} Octave"), 0, IntRange::Linear { min: -3, max: 3 })
                .with_value_to_string(Arc::new(|v| if v == 0 { "0".into() } else { format!("{v:+}") })),
            semi: IntParam::new(format!("{roman} Semitone"), 0, IntRange::Linear { min: -12, max: 12 })
                .with_value_to_string(Arc::new(|v| if v == 0 { "0 st".into() } else { format!("{v:+} st") })),
            fine: control(OscTarget::Fine),
            level: control(OscTarget::Level),
            squish: control(OscTarget::Squish),
            skew: control(OscTarget::Skew),
            tilt: control(OscTarget::Tilt),
            center: control(OscTarget::Center),
            ray: control(OscTarget::Ray),
            orbit: control(OscTarget::Orbit),
            orbit_hz: control(OscTarget::OrbitRate),
            free: BoolParam::new(format!("{roman} Free Running"), false),
        }
    }

    pub fn control(&self, k: OscTarget) -> &FloatParam {
        match k {
            OscTarget::Fine => &self.fine,
            OscTarget::Level => &self.level,
            OscTarget::Squish => &self.squish,
            OscTarget::Skew => &self.skew,
            OscTarget::Tilt => &self.tilt,
            OscTarget::Center => &self.center,
            OscTarget::Ray => &self.ray,
            OscTarget::Orbit => &self.orbit,
            OscTarget::OrbitRate => &self.orbit_hz,
        }
    }
}

/// One control's dynamic modulator: an LFO and an attack-decay envelope.
#[derive(Params)]
pub struct ModParams {
    #[id = "shape"]
    pub shape: EnumParam<LfoShape>,
    #[id = "rate"]
    pub rate: FloatParam,
    #[id = "sync"]
    pub sync: BoolParam,
    #[id = "div"]
    pub division: EnumParam<Division>,
    #[id = "retrig"]
    pub retrig: BoolParam,
    #[id = "lfo"]
    pub lfo: FloatParam,
    #[id = "att"]
    pub attack: FloatParam,
    #[id = "dec"]
    pub decay: FloatParam,
    #[id = "env"]
    pub env: FloatParam,
    #[id = "acurve"]
    pub attack_curve: FloatParam,
    #[id = "dcurve"]
    pub decay_curve: FloatParam,
    #[id = "wheel"]
    pub wheel: FloatParam,
    #[id = "touch"]
    pub touch: FloatParam,
    #[id = "note"]
    pub note: FloatParam,
    #[id = "vel"]
    pub vel: FloatParam,
}

fn amount(name: String) -> FloatParam {
    amount_within(name, 1.0, 0.0)
}

/// A bipolar modulation amount reaching ±`limit` of the control's range.
fn amount_within(name: String, limit: f32, default: f32) -> FloatParam {
    shown(
        FloatParam::new(name, default, FloatRange::Linear { min: -limit, max: limit }).with_smoother(SmoothingStyle::Linear(SMOOTH_MS)),
        |v| format!("{} %", signed(v * 100.0, 0)),
    )
    .with_string_to_value(Arc::new(|t| leading_number(t).map(|v| v / 100.0)))
}

/// A segment's bow: 0 straight, positive bowed up, negative down (see `envelope::curve`).
pub fn bow(name: String, default: f32) -> FloatParam {
    shown(FloatParam::new(name, default, FloatRange::Linear { min: -1.0, max: 1.0 }), |v| {
        if v.abs() < 0.005 {
            "straight".into()
        } else {
            format!("{} {:.0} %", if v > 0.0 { "up" } else { "down" }, v.abs() * 100.0)
        }
    })
    .with_string_to_value(Arc::new(|t| leading_number(t).map(|v| v / 100.0)))
}

fn time(name: String, default: f32, min: f32, max: f32) -> FloatParam {
    shown(FloatParam::new(name, default, FloatRange::Skewed { min, max, factor: FloatRange::skew_factor(-2.0) }), seconds)
}

impl ModParams {
    fn new(target: usize) -> Self {
        let name = target_name(target);
        Self {
            shape: EnumParam::new(format!("{name} LFO Shape"), LfoShape::Sine),
            rate: shown(
                FloatParam::new(format!("{name} LFO Rate"), 1.0, FloatRange::Skewed { min: 0.01, max: 50.0, factor: FloatRange::skew_factor(-2.0) }),
                hz,
            ),
            sync: BoolParam::new(format!("{name} LFO Sync"), false),
            division: EnumParam::new(format!("{name} LFO Division"), Division::Quarter),
            retrig: BoolParam::new(format!("{name} LFO Retrig"), true),
            // Half the range either way: past that the knob is all coarse, rarely-wanted travel.
            lfo: amount_within(format!("{name} LFO Amount"), 0.5, 0.0),
            attack: time(format!("{name} Env Attack"), 0.005, 0.001, 10.0),
            decay: time(format!("{name} Env Decay"), 0.4, 0.005, 10.0),
            env: amount(format!("{name} Env Amount")),
            attack_curve: bow(format!("{name} Env Attack Curve"), 0.0),
            decay_curve: bow(format!("{name} Env Decay Curve"), DEFAULT_DECAY_BOW),
            wheel: amount(format!("{name} Mod Wheel Amount")),
            touch: amount(format!("{name} Aftertouch Amount")),
            // The cutoff follows the keyboard halfway by default, as the old Key knob did.
            note: amount_within(format!("{name} Note Amount"), 1.0, if target == global_target(GlobalTarget::Cutoff) { 0.5 } else { 0.0 }),
            vel: amount(format!("{name} Velocity Amount")),
        }
    }

    fn settings(&self, n: u32) -> ModSettings {
        ModSettings {
            shape: self.shape.value(),
            hz: self.rate.value(),
            sync: self.sync.value().then(|| self.division.value().beats()),
            retrig: self.retrig.value(),
            lfo: self.lfo.smoothed.next_step(n),
            attack_s: self.attack.value(),
            decay_s: self.decay.value(),
            env: self.env.smoothed.next_step(n),
            attack_curve: self.attack_curve.value(),
            decay_curve: self.decay_curve.value(),
            wheel: self.wheel.smoothed.next_step(n),
            touch: self.touch.smoothed.next_step(n),
            note: self.note.smoothed.next_step(n),
            vel: self.vel.smoothed.next_step(n),
        }
    }

    /// Moved by the LFO or the envelope.
    pub fn active(&self) -> bool {
        self.lfo.value().abs() > 1e-4 || self.env.value().abs() > 1e-4
    }

    /// Moved by the mod wheel or aftertouch.
    pub fn held(&self) -> bool {
        [&self.wheel, &self.touch, &self.note, &self.vel].iter().any(|p| p.value().abs() > 1e-4)
    }
}

#[derive(Params)]
pub struct RaycasterParams {
    #[persist = "editor-state"]
    pub editor_state: Arc<ViziaState>,

    #[nested(array, group = "Oscillator")]
    pub osc: [OscParams; OSCS],

    #[id = "ftype"]
    pub filter: EnumParam<FilterKind>,
    #[id = "drive"]
    pub drive: FloatParam,
    #[id = "cutoff"]
    pub cutoff: FloatParam,
    #[id = "res"]
    pub resonance: FloatParam,
    #[id = "fenv"]
    pub filter_env: FloatParam,
    #[id = "fmode"]
    pub filter_mode: FloatParam,
    #[id = "fa"]
    pub filter_attack: FloatParam,
    #[id = "fd"]
    pub filter_decay: FloatParam,
    #[id = "fs"]
    pub filter_sustain: FloatParam,
    #[id = "fr"]
    pub filter_release: FloatParam,
    #[id = "fac"]
    pub filter_attack_curve: FloatParam,
    #[id = "fdc"]
    pub filter_decay_curve: FloatParam,
    #[id = "frc"]
    pub filter_release_curve: FloatParam,
    #[id = "aa"]
    pub amp_attack: FloatParam,
    #[id = "ad"]
    pub amp_decay: FloatParam,
    #[id = "as"]
    pub amp_sustain: FloatParam,
    #[id = "ar"]
    pub amp_release: FloatParam,
    #[id = "aac"]
    pub amp_attack_curve: FloatParam,
    #[id = "adc"]
    pub amp_decay_curve: FloatParam,
    #[id = "arc"]
    pub amp_release_curve: FloatParam,
    #[id = "vel"]
    pub velocity: FloatParam,
    #[id = "mono"]
    pub mono: BoolParam,
    #[id = "unison"]
    pub unison: IntParam,
    #[id = "detune"]
    pub detune: FloatParam,
    #[id = "glide"]
    pub glide: FloatParam,
    #[id = "width"]
    pub width: FloatParam,
    #[id = "chorus"]
    pub chorus: FloatParam,
    #[id = "vintage"]
    pub vintage: FloatParam,
    #[id = "vol"]
    pub volume: FloatParam,

    #[nested(array, group = "Modulator")]
    pub mods: [ModParams; TARGETS],
}

impl Default for RaycasterParams {
    fn default() -> Self {
        let global = |k: GlobalTarget, show: fn(f32) -> String| {
            shown(ranged(crate::dsp::targets::target_name(crate::dsp::targets::global_target(k)), global_range(k)), show)
        };
        Self {
            editor_state: crate::editor::default_state(),
            osc: std::array::from_fn(OscParams::new),
            filter: EnumParam::new("Filter Type", FilterKind::Ladder),
            drive: global(GlobalTarget::Drive, percent),
            cutoff: global(GlobalTarget::Cutoff, hz),
            resonance: global(GlobalTarget::Resonance, percent),
            filter_env: global(GlobalTarget::FilterEnv, |v| format!("{} oct", signed(v * 6.0, 1))),
            filter_mode: global(GlobalTarget::FilterMode, mode_text),
            filter_attack: time("Filter Attack".into(), 0.003, 0.001, 10.0),
            filter_decay: time("Filter Decay".into(), 0.45, 0.005, 10.0),
            filter_sustain: shown(FloatParam::new("Filter Sustain", 0.25, FloatRange::Linear { min: 0.0, max: 1.0 }), percent),
            filter_release: time("Filter Release".into(), 0.3, 0.005, 10.0),
            filter_attack_curve: bow("Filter Attack Curve".into(), DEFAULT_ATTACK_BOW),
            filter_decay_curve: bow("Filter Decay Curve".into(), DEFAULT_FALL_BOW),
            filter_release_curve: bow("Filter Release Curve".into(), DEFAULT_FALL_BOW),
            amp_attack: time("Amp Attack".into(), 0.004, 0.001, 10.0),
            amp_decay: time("Amp Decay".into(), 0.6, 0.005, 10.0),
            amp_sustain: shown(FloatParam::new("Amp Sustain", 0.8, FloatRange::Linear { min: 0.0, max: 1.0 }), percent),
            amp_release: time("Amp Release".into(), 0.25, 0.005, 10.0),
            amp_attack_curve: bow("Amp Attack Curve".into(), DEFAULT_ATTACK_BOW),
            amp_decay_curve: bow("Amp Decay Curve".into(), DEFAULT_FALL_BOW),
            amp_release_curve: bow("Amp Release Curve".into(), DEFAULT_FALL_BOW),
            velocity: shown(FloatParam::new("Velocity", 0.5, FloatRange::Linear { min: 0.0, max: 1.0 }), percent),
            mono: BoolParam::new("Mono", false),
            unison: IntParam::new("Unison", 1, IntRange::Linear { min: 1, max: MAX_UNISON as i32 })
                .with_value_to_string(Arc::new(|v| if v == 1 { "off".into() } else { format!("×{v}") }))
                .with_string_to_value(Arc::new(|t| leading_number(t).map(|v| v as i32))),
            detune: shown(FloatParam::new("Unison Detune", 20.0, FloatRange::Linear { min: 0.0, max: 100.0 }), |v| format!("{v:.0} ¢")),
            glide: shown(
                FloatParam::new("Glide", 0.08, FloatRange::Skewed { min: 0.0, max: 1.0, factor: FloatRange::skew_factor(-1.5) }),
                |v| if v < 0.0015 { "off".into() } else { seconds(v) },
            ),
            width: global(GlobalTarget::Width, percent),
            chorus: global(GlobalTarget::Chorus, |v| if v < 0.0005 { "off".into() } else { percent(v) }),
            vintage: shown(FloatParam::new("Vintage", 0.0, FloatRange::Linear { min: 0.0, max: 1.0 }), |v| {
                if v < 0.0005 { "off".into() } else { percent(v) }
            }),
            volume: FloatParam::new(
                "Volume",
                util::db_to_gain(-6.0),
                FloatRange::Skewed { min: util::db_to_gain(-36.0), max: util::db_to_gain(6.0), factor: FloatRange::gain_skew_factor(-36.0, 6.0) },
            )
            .with_smoother(SmoothingStyle::Logarithmic(SMOOTH_MS))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(1))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
            mods: std::array::from_fn(ModParams::new),
        }
    }
}

impl RaycasterParams {
    pub fn global(&self, k: GlobalTarget) -> &FloatParam {
        match k {
            GlobalTarget::Drive => &self.drive,
            GlobalTarget::Cutoff => &self.cutoff,
            GlobalTarget::Resonance => &self.resonance,
            GlobalTarget::FilterEnv => &self.filter_env,
            GlobalTarget::Width => &self.width,
            GlobalTarget::Chorus => &self.chorus,
            GlobalTarget::FilterMode => &self.filter_mode,
        }
    }

    /// The knob behind modulatable control `index`.
    pub fn target(&self, index: usize) -> &FloatParam {
        let per = OSC_TARGETS.len();
        if index < OSCS * per {
            self.osc[index / per].control(OSC_TARGETS[index % per])
        } else {
            self.global(GLOBAL_TARGETS[index - OSCS * per])
        }
    }

    /// Everything the engine needs for the next `n` samples; smoothed controls move on by `n`.
    pub fn block(&self, n: u32, clock: Clock) -> BlockControls {
        let base = std::array::from_fn(|t| {
            let param = self.target(t);
            crate::dsp::targets::range_of(t).normalize(param.smoothed.next_step(n))
        });
        BlockControls {
            base,
            mods: std::array::from_fn(|t| self.mods[t].settings(n)),
            oscs: std::array::from_fn(|o| {
                let p = &self.osc[o];
                OscSetup { element: p.element.value(), octave: p.octave.value(), semi: p.semi.value(), free: p.free.value() }
            }),
            filter: self.filter.value(),
            filter_env: Times {
                attack_s: self.filter_attack.value(),
                decay_s: self.filter_decay.value(),
                sustain: self.filter_sustain.value(),
                release_s: self.filter_release.value(),
                curves: Curves {
                    attack: self.filter_attack_curve.value(),
                    decay: self.filter_decay_curve.value(),
                    release: self.filter_release_curve.value(),
                },
            },
            amp_env: Times {
                attack_s: self.amp_attack.value(),
                decay_s: self.amp_decay.value(),
                sustain: self.amp_sustain.value(),
                release_s: self.amp_release.value(),
                curves: Curves {
                    attack: self.amp_attack_curve.value(),
                    decay: self.amp_decay_curve.value(),
                    release: self.amp_release_curve.value(),
                },
            },
            velocity: self.velocity.value(),
            mono: self.mono.value(),
            unison: self.unison.value() as usize,
            detune: self.detune.value(),
            glide_s: self.glide.value(),
            vintage: self.vintage.value(),
            volume: self.volume.smoothed.next_step(n),
            clock,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typed_values_are_understood() {
        assert_eq!(leading_number("700 Hz"), Some(700.0));
        assert_eq!(leading_number("×1.50"), Some(1.5));
        assert_eq!(leading_number("−12 ¢"), Some(-12.0));
    }

    #[test]
    fn every_target_has_a_knob_and_names_are_unique() {
        let p = RaycasterParams::default();
        let mut names: Vec<String> = (0..TARGETS).map(|t| p.target(t).name().to_string()).collect();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), TARGETS);
        let ids: Vec<String> = p.param_map().into_iter().map(|(id, _, _)| id).collect();
        let mut unique = ids.clone();
        unique.sort();
        unique.dedup();
        assert_eq!(unique.len(), ids.len());
    }

    #[test]
    fn the_engine_reads_the_knobs_as_they_are_set() {
        let p = RaycasterParams::default();
        // The host does this when the plugin starts: smoothers begin at their knobs' values.
        for o in 0..OSCS {
            for k in OSC_TARGETS {
                p.osc[o].control(k).smoothed.reset(p.osc[o].control(k).value());
            }
        }
        let c = p.block(32, Clock::default());
        let level = crate::dsp::targets::osc_target(0, OscTarget::Level);
        assert!((osc_range(OscTarget::Level).unnormalize(c.base[level]) - 0.8).abs() < 1e-4);
        assert_eq!(c.oscs[0].element, Element::Earth);
    }
}
