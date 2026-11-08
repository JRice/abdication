//! Host parameters, built at startup from the Faust DSPs' own UI
//! descriptions (the same scheme as Abdication): the voice's controls, then
//! the effects'. Add or change a control in
//! the .dsp files, regenerate `dsp.rs`, and it shows up here with the same
//! name, range and menu.
//!
//! Parameter IDs are "group.label" slugs, so they stay stable across
//! builds (saved projects keep their settings) as long as the Faust group
//! and label don't change.

use std::collections::HashMap;
use std::sync::Arc;

use faust_types::{FaustDsp, ParamIndex, UI};
use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;

use crate::body::BodyDsp;
use crate::dsp::VoiceDsp;
use crate::fx::FxDsp;

/// Faust controls the engine drives itself instead of exposing to the host.
const ENGINE_CONTROLS: [&str; 17] = [
    "freq", "gain", "gate", "seed", "tune", "hit", "vel", "env_attack", "env_decay", "env_curve", "env_velocity",
    "body_hz", "body_material", "body_t60", "body_bright", "body_position", "body_gate",
];
/// The voice's Envelope settings, which the engine passes on to the effects.
pub const ENVELOPE_IDS: [&str; 4] = ["envelope.attack", "envelope.decay", "envelope.curve", "envelope.velocity"];

/// Which Faust DSP a parameter belongs to.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Target {
    /// The voice: one copy per overlapping hit.
    Voice,
    /// The effects: one per pad, on the sum of its hits.
    Fx,
    /// The Body: one per voice, fed by it (no host parameters of its own).
    Body,
}

pub enum Control {
    Float(FloatParam),
    Int(IntParam),
    Bool(BoolParam),
}

impl Control {
    fn ptr(&self) -> ParamPtr {
        match self {
            Control::Float(p) => p.as_ptr(),
            Control::Int(p) => p.as_ptr(),
            Control::Bool(p) => p.as_ptr(),
        }
    }

    fn plain(&self) -> f32 {
        match self {
            Control::Float(p) => p.value(),
            Control::Int(p) => p.value() as f32,
            Control::Bool(p) => p.value() as i32 as f32,
        }
    }
}

struct Entry {
    id: String,
    group: String,
    target: Target,
    faust: ParamIndex,
    control: Control,
}

/// Where the engine-driven controls live in the Faust DSPs.
#[derive(Clone, Copy)]
pub struct VoiceSlots {
    pub freq: ParamIndex,
    pub gain: ParamIndex,
    pub gate: ParamIndex,
    pub seed: ParamIndex,
    /// Effects: the pitch Ring mod and Comb are tuned to, a pulse per hit
    /// and its velocity, and the Envelope settings (in ENVELOPE_IDS order).
    pub tune: ParamIndex,
    pub hit: ParamIndex,
    pub vel: ParamIndex,
    pub envelope: [ParamIndex; 4],
    /// The Body's settings (pitch, material, decay, brightness, position:
    /// the voice's last five outputs, in order) and its hit pulse.
    pub body: [ParamIndex; 5],
    pub body_gate: ParamIndex,
}

pub struct DrumParams {
    entries: Vec<Entry>,
    slots: VoiceSlots,
    /// Plugin-level output volume (not part of the Faust voice).
    pub volume: FloatParam,
    /// The panel's size (fixed; not saved with projects).
    pub editor_state: Arc<ViziaState>,
}

impl DrumParams {
    pub fn from_faust() -> Self {
        let mut collector = Collector { target: Target::Voice, ..Default::default() };
        VoiceDsp::build_user_interface_static(&mut collector);
        collector.target = Target::Fx;
        FxDsp::build_user_interface_static(&mut collector);
        collector.target = Target::Body;
        BodyDsp::build_user_interface_static(&mut collector);
        let find = |name: &str| {
            *collector.voice_slots.get(name).unwrap_or_else(|| panic!("Faust DSPs have no '{name}' control"))
        };
        let slots = VoiceSlots {
            freq: find("freq"),
            gain: find("gain"),
            gate: find("gate"),
            seed: find("seed"),
            tune: find("tune"),
            hit: find("hit"),
            vel: find("vel"),
            envelope: [find("env_attack"), find("env_decay"), find("env_curve"), find("env_velocity")],
            body: [find("body_hz"), find("body_material"), find("body_t60"), find("body_bright"), find("body_position")],
            body_gate: find("body_gate"),
        };

        Self {
            entries: collector.entries,
            slots,
            volume: FloatParam::new(
                "Master: Volume",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-48.0),
                    max: util::db_to_gain(6.0),
                    factor: FloatRange::gain_skew_factor(-48.0, 6.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(20.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(1))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
            editor_state: crate::editor::default_state(),
        }
    }

    /// The host parameter behind an id, for the panel.
    pub fn ptr(&self, id: &str) -> Option<ParamPtr> {
        if id == "master.volume" {
            return Some(self.volume.as_ptr());
        }
        self.entries.iter().find(|e| e.id == id).map(|e| e.control.ptr())
    }

    /// Every parameter id.
    pub fn ids(&self) -> impl Iterator<Item = &str> {
        self.entries.iter().map(|e| e.id.as_str()).chain(["master.volume"])
    }

    pub fn slots(&self) -> VoiceSlots {
        self.slots
    }

    /// The DSP and Faust index behind a parameter id, e.g. "tone.pitch".
    pub fn faust_index(&self, id: &str) -> Option<(Target, ParamIndex)> {
        self.entries.iter().find(|e| e.id == id).map(|e| (e.target, e.faust))
    }

    /// The current plain value of a Faust-backed parameter.
    pub fn value(&self, id: &str) -> Option<f32> {
        self.entries.iter().find(|e| e.id == id).map(|e| e.control.plain())
    }

    /// The Envelope settings (ENVELOPE_IDS order), for the effects.
    pub fn envelope(&self) -> [f32; 4] {
        ENVELOPE_IDS.map(|id| self.value(id).unwrap_or(0.0))
    }

    /// Current plain value of every Faust-backed parameter.
    pub fn faust_values(&self) -> impl Iterator<Item = (Target, ParamIndex, f32)> + '_ {
        self.entries.iter().map(|e| (e.target, e.faust, e.control.plain()))
    }
}

unsafe impl Params for DrumParams {
    fn param_map(&self) -> Vec<(String, ParamPtr, String)> {
        let mut map: Vec<_> = self.entries.iter().map(|e| (e.id.clone(), e.control.ptr(), e.group.clone())).collect();
        map.push(("master.volume".into(), self.volume.as_ptr(), "Master".into()));
        map
    }
}

// ---------------------------------------------------------------------------
// Building parameters from Faust's UI callbacks
// ---------------------------------------------------------------------------

struct Collector {
    /// The DSP whose controls are being collected.
    target: Target,
    groups: Vec<String>,
    meta: HashMap<i32, HashMap<String, String>>,
    entries: Vec<Entry>,
    voice_slots: HashMap<String, ParamIndex>,
}

impl Default for Collector {
    fn default() -> Self {
        Self { target: Target::Voice, groups: Vec::new(), meta: HashMap::new(), entries: Vec::new(), voice_slots: HashMap::new() }
    }
}

impl Collector {
    /// The innermost named group ("Tone", "Body", ...). The outermost box
    /// is the .dsp file's own name, so it's skipped.
    fn group(&self) -> String {
        self.groups.iter().skip(1).last().cloned().unwrap_or_default()
    }

    fn meta(&self, param: ParamIndex, key: &str) -> Option<&str> {
        self.meta.get(&param.0)?.get(key).map(String::as_str)
    }

    /// Display name: "Panel: Label", so related controls sort together in
    /// the host's list ("Tone: Pitch", "Noise: Clap").
    fn display(&self, label: &str) -> String {
        match self.group() {
            g if g.is_empty() => label.to_string(),
            g => format!("{g}: {label}"),
        }
    }

    fn push(&mut self, label: &str, param: ParamIndex, control: Control) {
        let group = self.group();
        self.entries.push(Entry { id: format!("{}.{}", slug(&group), slug(label)), group, target: self.target, faust: param, control });
    }

    /// Unit strings must be 'static for nih-plug; there are only a handful,
    /// created once at startup.
    fn unit(&self, param: ParamIndex) -> &'static str {
        match self.meta(param, "unit") {
            Some(u) => Box::leak(format!(" {u}").into_boxed_str()),
            None => "",
        }
    }

    fn float(&mut self, label: &str, param: ParamIndex, init: f32, min: f32, max: f32, step: f32) {
        if ENGINE_CONTROLS.contains(&label) {
            self.voice_slots.insert(label.into(), param);
            return;
        }
        // [scale:log] -> strong skew toward the low end; [scale:gain] (linear
        // gain, 0 = off) likewise, so the quiet end gets most of the travel
        // (-40 dB sits at about a third, -20 dB near the middle).
        let range = match self.meta(param, "scale") {
            Some("log" | "gain") => FloatRange::Skewed { min, max, factor: FloatRange::skew_factor(-2.0) },
            _ => FloatRange::Linear { min, max },
        };
        let digits = if step > 0.0 { (-step.log10()).ceil().max(0.0) as usize } else { 3 };
        let mut p = FloatParam::new(self.display(label), init, range)
            .with_unit(self.unit(param))
            .with_value_to_string(formatters::v2s_f32_rounded(digits));
        if step > 0.0 {
            p = p.with_step_size(step);
        }
        self.push(label, param, Control::Float(p));
    }

    fn entry(&mut self, label: &str, param: ParamIndex, init: f32, min: f32, max: f32) {
        let range = IntRange::Linear { min: min as i32, max: max as i32 };
        let mut p = IntParam::new(self.display(label), init as i32, range).with_unit(self.unit(param));
        if let Some(names) = self.meta(param, "style").and_then(parse_menu) {
            let names = Arc::new(names);
            let lookup = names.clone();
            p = p
                .with_value_to_string(Arc::new(move |v| names.get(v as usize).cloned().unwrap_or_else(|| v.to_string())))
                .with_string_to_value(Arc::new(move |s| {
                    lookup.iter().position(|n| n.eq_ignore_ascii_case(s.trim())).map(|i| i as i32)
                }));
        }
        self.push(label, param, Control::Int(p));
    }
}

impl UI<f32> for Collector {
    fn open_tab_box(&mut self, label: &str) {
        self.groups.push(label.into());
    }
    fn open_horizontal_box(&mut self, label: &str) {
        self.groups.push(label.into());
    }
    fn open_vertical_box(&mut self, label: &str) {
        self.groups.push(label.into());
    }
    fn close_box(&mut self) {
        self.groups.pop();
    }

    fn add_button(&mut self, label: &str, param: ParamIndex) {
        // Only gate and hit are buttons; momentary controls make poor host params.
        if ENGINE_CONTROLS.contains(&label) {
            self.voice_slots.insert(label.into(), param);
        }
    }
    fn add_check_button(&mut self, label: &str, param: ParamIndex) {
        let name = self.display(label);
        self.push(label, param, Control::Bool(BoolParam::new(name, false)));
    }
    fn add_vertical_slider(&mut self, label: &str, p: ParamIndex, init: f32, min: f32, max: f32, step: f32) {
        self.float(label, p, init, min, max, step);
    }
    fn add_horizontal_slider(&mut self, label: &str, p: ParamIndex, init: f32, min: f32, max: f32, step: f32) {
        self.float(label, p, init, min, max, step);
    }
    fn add_num_entry(&mut self, label: &str, p: ParamIndex, init: f32, min: f32, max: f32, step: f32) {
        if step.fract() == 0.0 && min.fract() == 0.0 {
            self.entry(label, p, init, min, max);
        } else {
            self.float(label, p, init, min, max, step);
        }
    }

    fn add_horizontal_bargraph(&mut self, _: &str, _: ParamIndex, _: f32, _: f32) {}
    fn add_vertical_bargraph(&mut self, _: &str, _: ParamIndex, _: f32, _: f32) {}

    fn declare(&mut self, param: Option<ParamIndex>, key: &str, value: &str) {
        if let Some(p) = param {
            self.meta.entry(p.0).or_default().insert(key.into(), value.into());
        }
    }
}

/// "Fast Decay" -> "fast_decay", "Sample & Hold" -> "sample_hold".
fn slug(s: &str) -> String {
    let mut out = String::new();
    for c in s.chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c.to_ascii_lowercase());
        } else if !out.ends_with('_') && !out.is_empty() {
            out.push('_');
        }
    }
    out.trim_end_matches('_').to_string()
}

/// "menu{'White':0;'Pink':1}" -> ["White", "Pink"], ordered by value.
fn parse_menu(style: &str) -> Option<Vec<String>> {
    let body = style.strip_prefix("menu{")?.strip_suffix('}')?;
    let mut items: Vec<(i32, String)> = body
        .split(';')
        .filter_map(|item| {
            let (name, value) = item.rsplit_once(':')?;
            Some((value.trim().parse().ok()?, name.trim().trim_matches('\'').to_string()))
        })
        .collect();
    items.sort_by_key(|(v, _)| *v);
    Some(items.into_iter().map(|(_, n)| n).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slugs() {
        assert_eq!(slug("Fast Decay"), "fast_decay");
        assert_eq!(slug("Sample & Hold"), "sample_hold");
    }

    #[test]
    fn menus_keep_colons_in_names() {
        assert_eq!(
            parse_menu("menu{'Steady: 018 SH101 Noise':0;'Crackle: SP02':1}").unwrap(),
            vec!["Steady: 018 SH101 Noise", "Crackle: SP02"]
        );
    }

    #[test]
    fn every_faust_control_becomes_a_unique_param() {
        let params = DrumParams::from_faust();
        let map = params.param_map();
        let mut ids: Vec<_> = map.iter().map(|(id, _, _)| id.clone()).collect();
        let n = ids.len();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), n, "duplicate parameter ids");
        for id in ["tone.pitch", "body.material", "noise.loop", "noise.generator", "voice.variation", "master.volume",
                   "crush.amount", "drive.type", "drive.amount", "ring_mod.amount", "comb.amount",
                   "compressor.threshold", "compressor.timing"] {
            assert!(ids.contains(&id.to_string()), "missing {id}");
        }
        assert!(!ids.iter().any(|id| [".gate", ".seed", ".tune", ".hit"].iter().any(|s| id.ends_with(s))));
        assert_eq!(params.faust_index("drive.amount").unwrap().0, Target::Fx);
        assert_eq!(params.faust_index("tone.pitch").unwrap().0, Target::Voice);
    }

    /// Lists every host parameter: `cargo test --release -- --ignored --nocapture list_params`
    #[test]
    #[ignore]
    fn list_params() {
        for (id, ptr, _) in DrumParams::from_faust().param_map() {
            println!("{id}\t{}", unsafe { ptr.name() });
        }
    }
}
