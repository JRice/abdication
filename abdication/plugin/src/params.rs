//! Host parameters, built at startup from the Faust voice's own UI
//! description (all its DSPs: the voice, a unison copy of its Dual and
//! Wavetable, and its texture), and the calls bus. Add or change a control
//! in the .dsp files, regenerate the Rust code, and it shows up here with
//! the same name, range and menu. A control two DSPs both declare is one
//! host parameter that sets both.
//!
//! Then each modulatable knob's modulators (modulation.rs), which are the
//! plugin's own: "mod.<knob>.<field>", e.g. "mod.filter_cutoff.lfo".
//!
//! Parameter IDs are "group.label" slugs, so they stay stable across
//! builds (saved projects keep their settings) as long as the Faust
//! group and label don't change.

use std::collections::HashMap;
use std::sync::Arc;

use faust_types::{FaustDsp, ParamIndex, UI};
use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;

use crate::callbus_dsp::CallBusDsp;
use crate::dsp::VoiceDsp;
use crate::modulation::{self, Dest, ModSettings, Shape, Travel, TARGETS};
use crate::osc_dsp::OscDsp;
use crate::texture_dsp::TextureDsp;

const GLIDE_MIN_MS: f32 = 20.0;
const GLIDE_MAX_MS: f32 = 200.0;

/// The Portamento knob (0..1) as seconds: 0 is off (polyphonic); the
/// rest of the travel runs 30-300 ms.
fn glide_seconds(knob: f32) -> f32 {
    if knob <= 0.0 {
        0.0
    } else {
        (GLIDE_MIN_MS + (GLIDE_MAX_MS - GLIDE_MIN_MS) * knob.min(1.0)) / 1000.0
    }
}

/// Faust controls the plugin drives itself instead of exposing to the host.
const VOICE_CONTROLS: [&str; 9] = ["freq", "gain", "gate", "seed", "bend", "glide", "slot", "copy", "ramp"];

/// Which of a voice's Faust DSPs a control belongs to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Dsp {
    /// The voice itself (dsp.rs).
    Voice,
    /// A unison copy of its Dual and Wavetable (osc_dsp.rs).
    Osc,
    /// Its events and calls (texture_dsp.rs).
    Texture,
    /// The calls' echo and reverb, one for the instrument (callbus_dsp.rs).
    CallBus,
}

/// A control in one of a voice's DSPs.
#[derive(Clone, Copy, Debug)]
pub struct FaustParam {
    pub dsp: Dsp,
    pub index: ParamIndex,
}

impl PartialEq for FaustParam {
    fn eq(&self, other: &Self) -> bool {
        self.dsp == other.dsp && self.index.0 == other.index.0
    }
}

/// Every DSP control one value goes to (a control declared by several
/// of a voice's DSPs), and, for a modulatable knob, its place in
/// `modulation::TARGETS` (the engine then moves it per voice).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Targets {
    params: [Option<FaustParam>; 4],
    pub modulated: Option<u8>,
}

impl Targets {
    fn add(&mut self, param: FaustParam) {
        let free = self.params.iter_mut().find(|t| t.is_none()).expect("a control in more than four DSPs");
        *free = Some(param);
    }

    pub fn iter(&self) -> impl Iterator<Item = FaustParam> + '_ {
        self.params.iter().flatten().copied()
    }
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
    faust: Targets,
    control: Control,
    /// A float knob's range, as the host's normalized value maps it.
    travel: Option<Travel>,
}

/// Where the voice's plugin-driven controls live in its Faust DSPs.
#[derive(Clone, Copy)]
pub struct VoiceSlots {
    pub freq: Targets,
    pub gain: Targets,
    pub gate: Targets,
    pub seed: Targets,
    pub bend: Targets,
    pub glide: Targets,
    pub slot: Targets,
    /// Which unison copy a copy of the Dual and Wavetable is.
    pub copy: Targets,
    /// Samples the modulated controls take to reach a new value (the block's length).
    pub ramp: Targets,
    /// The modulatable knobs (modulation::TARGETS), and their travel.
    pub mod_targets: [Targets; modulation::COUNT],
    pub mod_travel: [Travel; modulation::COUNT],
    /// Unison on/off (a host parameter): how many copies the plugin runs.
    pub unison: Targets,
    /// Whether the plugin runs the texture DSP (host parameters): Events
    /// Depth above 0, or Calls Level above its bottom, unless muted.
    pub events_depth: Targets,
    pub events_mute: Targets,
    pub calls_level: Targets,
    pub calls_mute: Targets,
    /// Each oscillator slot's controls that the plugin's Ray reads (RAY_FIELDS).
    pub ray: [[Targets; RAY_FIELDS.len()]; 2],
}

/// What the plugin's Ray reads of slot N (osc_N.mode, mix.osc_N, ray_N.*),
/// in this order (voice.rs's RayField).
pub const RAY_FIELDS: [&str; 13] = ["osc_N.mode", "mix.osc_N", "ray_N.element", "ray_N.chaos_seed", "ray_N.free",
                                     "ray_N.squish", "ray_N.skew", "ray_N.tilt", "ray_N.center", "ray_N.ray",
                                     "ray_N.orbit", "ray_N.orbit_rate", "ray_N.fine"];

pub struct SynthParams {
    entries: Vec<Entry>,
    slots: VoiceSlots,
    /// Each modulatable knob's modulators, in modulation::TARGETS order.
    pub mods: Vec<ModParams>,
    /// Plugin-level output volume (not part of the Faust voice).
    pub volume: FloatParam,
    /// 0 = polyphonic; above 0 = mono with a glide of 30-300 ms.
    pub portamento: FloatParam,
    /// Editor window size and scale (fixed; not saved).
    pub editor_state: Arc<ViziaState>,
}

impl SynthParams {
    pub fn from_faust() -> Self {
        let mut collector = Collector::default(); // starts on the voice
        VoiceDsp::build_user_interface_static(&mut collector);
        collector.dsp = Dsp::Osc;
        OscDsp::build_user_interface_static(&mut collector);
        collector.dsp = Dsp::Texture;
        TextureDsp::build_user_interface_static(&mut collector);
        collector.dsp = Dsp::CallBus;
        CallBusDsp::build_user_interface_static(&mut collector);

        // A plugin-driven control, wherever the DSPs declare it.
        let find = |name: &str| {
            let mut targets = Targets::default();
            for dsp in [Dsp::Voice, Dsp::Osc, Dsp::Texture] {
                if let Some(&index) = collector.voice_slots.get(&(dsp, name.to_string())) {
                    targets.add(FaustParam { dsp, index });
                }
            }
            assert!(targets.iter().next().is_some(), "no Faust DSP has a '{name}' control");
            targets
        };
        // The modulatable knobs know their place.
        for (t, target) in TARGETS.iter().enumerate() {
            let entry = collector.entries.iter_mut().find(|e| e.id == target.id)
                .unwrap_or_else(|| panic!("no modulatable control '{}'", target.id));
            assert!(entry.travel.is_some(), "'{}' isn't a float knob", target.id);
            entry.faust.modulated = Some(t as u8);
        }
        let host = |id: &str| collector.entries.iter().find(|e| e.id == id).unwrap_or_else(|| panic!("no '{id}' control")).faust;
        let entry = |id: &str| collector.entries.iter().find(|e| e.id == id).unwrap();
        let mods = TARGETS.iter().enumerate().map(|(t, target)| {
            let name = match &entry(target.id).control {
                Control::Float(p) => p.name().to_string(),
                _ => unreachable!(),
            };
            ModParams::new(t, &name)
        }).collect();
        let slots = VoiceSlots {
            freq: find("freq"),
            gain: find("gain"),
            gate: find("gate"),
            seed: find("seed"),
            bend: find("bend"),
            glide: find("glide"),
            slot: find("slot"),
            copy: find("copy"),
            ramp: find("ramp"),
            mod_targets: std::array::from_fn(|t| host(TARGETS[t].id)),
            mod_travel: std::array::from_fn(|t| entry(TARGETS[t].id).travel.unwrap()),
            unison: host("unison.mode"),
            events_depth: host("events.depth"),
            events_mute: host("events.mute"),
            calls_level: host("calls.level"),
            calls_mute: host("calls.mute"),
            ray: std::array::from_fn(|slot| {
                std::array::from_fn(|f| host(&RAY_FIELDS[f].replace('N', &(slot + 1).to_string())))
            }),
        };

        Self {
            entries: collector.entries,
            slots,
            mods,
            volume: FloatParam::new(
                "Master: Volume",
                util::db_to_gain(-6.0),
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
            portamento: FloatParam::new("Master: Portamento", 0.0, FloatRange::Linear { min: 0.0, max: 1.0 })
                .with_value_to_string(Arc::new(|v| {
                    let seconds = glide_seconds(v);
                    if seconds > 0.0 { format!("{:.0} ms", seconds * 1000.0) } else { "Off".into() }
                }))
                .with_string_to_value(Arc::new(|s| {
                    let s = s.trim().trim_end_matches("ms").trim();
                    if s.eq_ignore_ascii_case("off") {
                        return Some(0.0);
                    }
                    let ms: f32 = s.parse().ok()?;
                    // Anything typed is on: 20 ms (or less) is the knob's first step.
                    Some(((ms - GLIDE_MIN_MS) / (GLIDE_MAX_MS - GLIDE_MIN_MS)).clamp(0.001, 1.0))
                })),
            editor_state: crate::editor::default_state(),
        }
    }

    /// Portamento time in seconds; 0 means polyphonic.
    pub fn glide_seconds(&self) -> f32 {
        glide_seconds(self.portamento.value())
    }

    pub fn slots(&self) -> VoiceSlots {
        self.slots
    }

    /// The Faust controls behind a parameter id, e.g. "unison.mode".
    pub fn faust_index(&self, id: &str) -> Option<Targets> {
        self.entries.iter().find(|e| e.id == id).map(|e| e.faust)
    }

    /// The host parameter behind an id ("filter.cutoff", "master.volume",
    /// "mod.filter_cutoff.lfo").
    pub fn ptr(&self, id: &str) -> Option<ParamPtr> {
        match id {
            "master.volume" => return Some(self.volume.as_ptr()),
            "master.portamento" => return Some(self.portamento.as_ptr()),
            _ => {}
        }
        if let Some(rest) = id.strip_prefix("mod.") {
            let (knob, field) = rest.rsplit_once('.')?;
            let t = TARGETS.iter().position(|t| mod_slug(t.id) == knob)?;
            return self.mods[t].ptr(field);
        }
        self.entries.iter().find(|e| e.id == id).map(|e| e.control.ptr())
    }

    /// Every parameter id: Faust order, the master knobs, then the modulators.
    pub fn ids(&self) -> impl Iterator<Item = &str> {
        self.entries.iter().map(|e| e.id.as_str()).chain(["master.volume", "master.portamento"])
            .chain(self.mods.iter().flat_map(|m| m.ids.iter().map(String::as_str)))
    }

    /// The modulator settings of every modulatable knob, as the engine reads them.
    pub fn mod_settings(&self) -> impl Iterator<Item = ModSettings> + '_ {
        self.mods.iter().map(ModParams::settings)
    }

    /// Current plain value of every Faust-backed parameter.
    pub fn faust_values(&self) -> impl Iterator<Item = (Targets, f32)> + '_ {
        self.entries.iter().map(|e| (e.faust, e.control.plain()))
    }
}

unsafe impl Params for SynthParams {
    fn param_map(&self) -> Vec<(String, ParamPtr, String)> {
        let mut map: Vec<_> = self
            .entries
            .iter()
            .map(|e| (e.id.clone(), e.control.ptr(), e.group.clone()))
            .collect();
        map.push(("master.volume".into(), self.volume.as_ptr(), "Master".into()));
        map.push(("master.portamento".into(), self.portamento.as_ptr(), "Master".into()));
        for m in &self.mods {
            for (id, field) in m.ids.iter().zip(MOD_FIELDS) {
                map.push((id.clone(), m.ptr(field).unwrap(), "Modulation".into()));
            }
        }
        map
    }

}

// ---------------------------------------------------------------------------
// Building parameters from Faust's UI callbacks
// ---------------------------------------------------------------------------

struct Collector {
    /// The DSP whose UI is being collected.
    dsp: Dsp,
    groups: Vec<String>,
    meta: HashMap<i32, HashMap<String, String>>,
    entries: Vec<Entry>,
    voice_slots: HashMap<(Dsp, String), ParamIndex>,
    /// The range of the float control being added.
    travel: Option<Travel>,
}

impl Default for Collector {
    fn default() -> Self {
        Self { dsp: Dsp::Voice, groups: Vec::new(), meta: HashMap::new(), entries: Vec::new(),
               voice_slots: HashMap::new(), travel: None }
    }
}

impl Collector {
    /// The innermost named group ("Filter", "Envelope", ...). The outermost
    /// box is the .dsp file's own name, so it's skipped.
    fn group(&self) -> String {
        self.groups.iter().skip(1).last().cloned().unwrap_or_default()
    }

    fn meta(&self, param: ParamIndex, key: &str) -> Option<&str> {
        self.meta.get(&self.meta_key(param))?.get(key).map(String::as_str)
    }

    /// Metadata keys, distinct per DSP (each numbers its controls from 0).
    fn meta_key(&self, param: ParamIndex) -> i32 {
        param.0 + (self.dsp as i32) * (1 << 20)
    }

    /// Display name: "Panel: Label", so related controls sort together in
    /// the host's list ("Mix: Noise", "Transient: Level").
    fn display(&self, label: &str) -> String {
        match self.group() {
            g if g.is_empty() => label.to_string(),
            g => format!("{g}: {label}"),
        }
    }

    fn push(&mut self, label: &str, param: ParamIndex, control: Control) {
        let group = self.group();
        let id = format!("{}.{}", slug(&group), slug(label));
        let target = FaustParam { dsp: self.dsp, index: param };
        // Declared by an earlier DSP too: one parameter, set in both.
        if let Some(entry) = self.entries.iter_mut().find(|e| e.id == id) {
            debug_assert_eq!(entry.control.plain(), control.plain(), "'{id}' differs between DSPs");
            entry.faust.add(target);
            return;
        }
        let mut faust = Targets::default();
        faust.add(target);
        let travel = match &control {
            Control::Float(_) => self.travel.take(),
            _ => None,
        };
        self.entries.push(Entry { id, group, faust, control, travel });
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
        if VOICE_CONTROLS.contains(&label) {
            self.voice_slots.insert((self.dsp, label.into()), param);
            return;
        }
        // [scale:log] -> strong skew toward the low end; [skew:f] -> nih-plug
        // skew factor f directly (< 1 gives the low end more knob travel).
        let skew = self.meta(param, "skew").and_then(|f| f.parse::<f32>().ok());
        let factor = match (self.meta(param, "scale"), skew) {
            (_, Some(factor)) => factor,
            (Some("log"), None) => FloatRange::skew_factor(-2.0),
            _ => 1.0,
        };
        let range = if factor == 1.0 { FloatRange::Linear { min, max } } else { FloatRange::Skewed { min, max, factor } };
        self.travel = Some(Travel { min, max, factor });
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
                .with_value_to_string(Arc::new(move |v| {
                    names.get(v as usize).cloned().unwrap_or_else(|| v.to_string())
                }))
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
        // Only the gate is a button; momentary controls make poor host params.
        if VOICE_CONTROLS.contains(&label) {
            self.voice_slots.insert((self.dsp, label.into()), param);
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
            let slot = self.meta_key(p);
            self.meta.entry(slot).or_default().insert(key.into(), value.into());
        }
    }
}

// ---------------------------------------------------------------------------
// The knobs' modulators
// ---------------------------------------------------------------------------

/// A modulatable knob's id as it appears in its modulators' ids
/// ("filter.cutoff" -> "filter_cutoff").
pub fn mod_slug(id: &str) -> String {
    id.replace('.', "_")
}

/// Each modulator's fields, as the last part of their ids.
pub const MOD_FIELDS: [&str; 22] = ["shape", "rate", "sync", "division", "retrig", "lfo", "attack", "decay", "sustain",
                                    "release", "env", "vel", "touch", "wheel", "key", "vel_to", "touch_to",
                                    "wheel_to", "key_to", "attack_curve", "decay_curve", "release_curve"];

/// One modulatable knob's modulators: an LFO, an ADSR envelope, and how far
/// velocity, aftertouch, the mod wheel and the key move it, or move the LFO's
/// or envelope's amount (`*_to`). All amounts are in the knob's travel,
/// -100..100 %.
pub struct ModParams {
    pub shape: IntParam,
    pub rate: FloatParam,
    pub sync: BoolParam,
    pub division: IntParam,
    pub retrig: BoolParam,
    pub lfo: FloatParam,
    pub attack: FloatParam,
    pub decay: FloatParam,
    pub sustain: FloatParam,
    pub release: FloatParam,
    pub attack_curve: FloatParam,
    pub decay_curve: FloatParam,
    pub release_curve: FloatParam,
    pub env: FloatParam,
    pub vel: FloatParam,
    pub touch: FloatParam,
    pub wheel: FloatParam,
    pub key: FloatParam,
    pub vel_to: IntParam,
    pub touch_to: IntParam,
    pub wheel_to: IntParam,
    pub key_to: IntParam,
    /// Their ids, in MOD_FIELDS order.
    pub ids: Vec<String>,
}

/// An amount, -100..100 %: curved so small ones get more of the knob (a
/// quarter turn from the middle is 25 %), and shown to a tenth below 10 %.
fn percent(name: String, default: f32) -> FloatParam {
    let range = FloatRange::SymmetricalSkewed { min: -1.0, max: 1.0, factor: FloatRange::skew_factor(-1.0), center: 0.0 };
    FloatParam::new(name, default, range)
        .with_value_to_string(Arc::new(|v| {
            if v.abs() < 0.0995 { format!("{:+.1} %", v * 100.0) } else { format!("{:+.0} %", v * 100.0) }
        }))
        .with_string_to_value(Arc::new(|s| s.trim().trim_end_matches('%').trim().parse::<f32>().ok().map(|v| v / 100.0)))
}

fn seconds(name: String, default: f32) -> FloatParam {
    FloatParam::new(name, default, FloatRange::Skewed { min: 0.001, max: 10.0, factor: FloatRange::skew_factor(-2.0) })
        .with_value_to_string(Arc::new(|v| if v < 1.0 { format!("{:.0} ms", v * 1000.0) } else { format!("{v:.2} s") }))
        .with_string_to_value(Arc::new(|s| {
            let s = s.trim();
            match s.strip_suffix("ms") {
                Some(ms) => ms.trim().parse::<f32>().ok().map(|v| v / 1000.0),
                None => s.trim_end_matches('s').trim().parse().ok(),
            }
        }))
}

/// A segment's bow, -1..1 (modulation::curve).
pub fn bow(name: String, default: f32) -> FloatParam {
    FloatParam::new(name, default, FloatRange::Linear { min: -1.0, max: 1.0 })
        .with_value_to_string(Arc::new(|v| format!("{v:+.2}")))
}

fn menu(name: String, default: i32, names: &'static [&'static str]) -> IntParam {
    IntParam::new(name, default, IntRange::Linear { min: 0, max: names.len() as i32 - 1 })
        .with_value_to_string(Arc::new(move |v| names.get(v as usize).copied().unwrap_or("?").to_string()))
        .with_string_to_value(Arc::new(move |s| names.iter().position(|n| n.eq_ignore_ascii_case(s.trim())).map(|i| i as i32)))
}

impl ModParams {
    /// The modulators are hidden from the host's parameter list and can't be
    /// automated (Jeremy's choice: they're set on the panel); they're still
    /// saved with the patch.
    /// Knob `t` of modulation::TARGETS, whose host name is `name` ("Filter: Cutoff").
    fn new(t: usize, name: &str) -> Self {
        let n = |field: &str| format!("{name} {field}");
        // The cutoff's starts as the old filter envelope (2 octaves up from
        // 1200 Hz, sustaining at 30 %) and keytrack (half an octave per octave);
        // Global Pitch's as vibrato: a 5 Hz LFO the mod wheel deepens to
        // 25 cents (of the knob's 24 semitones).
        let cutoff = t == modulation::CUTOFF;
        let pitch = t == modulation::PITCH;
        let d = ModSettings::default();
        static DIVISION_NAMES: [&str; 15] = {
            let mut names = [""; 15];
            let mut i = 0;
            while i < 15 {
                names[i] = modulation::DIVISIONS[i].1;
                i += 1;
            }
            names
        };
        let slug = mod_slug(TARGETS[t].id);
        Self {
            shape: menu(n("LFO Shape"), 0, &modulation::SHAPE_NAMES).hide().non_automatable(),
            rate: FloatParam::new(n("LFO Rate"), if pitch { 5.0 } else { d.hz }, FloatRange::Skewed { min: 0.01, max: 50.0, factor: FloatRange::skew_factor(-2.0) })
                .with_unit(" Hz")
                .with_value_to_string(formatters::v2s_f32_rounded(2)).hide().non_automatable(),
            sync: BoolParam::new(n("LFO Sync"), false).hide().non_automatable(),
            division: menu(n("LFO Division"), modulation::DEFAULT_DIVISION as i32, &DIVISION_NAMES).hide().non_automatable(),
            retrig: BoolParam::new(n("LFO Retrig"), d.retrig).hide().non_automatable(),
            lfo: percent(n("LFO Amount"), 0.0).hide().non_automatable(),
            attack: seconds(n("Env Attack"), d.attack).hide().non_automatable(),
            decay: seconds(n("Env Decay"), d.decay).hide().non_automatable(),
            sustain: FloatParam::new(n("Env Sustain"), if cutoff { 0.3 } else { d.sustain }, FloatRange::Linear { min: 0.0, max: 1.0 })
                .with_value_to_string(formatters::v2s_f32_rounded(2)).hide().non_automatable(),
            release: seconds(n("Env Release"), d.release).hide().non_automatable(),
            attack_curve: bow(n("Env Attack Curve"), d.attack_curve).hide().non_automatable(),
            decay_curve: bow(n("Env Decay Curve"), d.decay_curve).hide().non_automatable(),
            release_curve: bow(n("Env Release Curve"), d.release_curve).hide().non_automatable(),
            env: percent(n("Env Amount"), if cutoff { 0.21 } else { 0.0 }).hide().non_automatable(),
            vel: percent(n("Velocity"), 0.0).hide().non_automatable(),
            touch: percent(n("Aftertouch"), 0.0).hide().non_automatable(),
            wheel: percent(n("Mod Wheel"), if pitch { 0.25 / 24.0 } else { 0.0 }).hide().non_automatable(),
            key: percent(n("Key"), if cutoff { 0.5 } else { 0.0 }).hide().non_automatable(),
            vel_to: menu(n("Velocity To"), 0, &modulation::DEST_NAMES).hide().non_automatable(),
            touch_to: menu(n("Aftertouch To"), 0, &modulation::DEST_NAMES).hide().non_automatable(),
            wheel_to: menu(n("Mod Wheel To"), if pitch { 1 } else { 0 }, &modulation::DEST_NAMES).hide().non_automatable(),
            key_to: menu(n("Key To"), 0, &modulation::DEST_NAMES).hide().non_automatable(),
            ids: MOD_FIELDS.iter().map(|f| format!("mod.{slug}.{f}")).collect(),
        }
    }

    pub fn ptr(&self, field: &str) -> Option<ParamPtr> {
        Some(match field {
            "shape" => self.shape.as_ptr(),
            "rate" => self.rate.as_ptr(),
            "sync" => self.sync.as_ptr(),
            "division" => self.division.as_ptr(),
            "retrig" => self.retrig.as_ptr(),
            "lfo" => self.lfo.as_ptr(),
            "attack" => self.attack.as_ptr(),
            "decay" => self.decay.as_ptr(),
            "sustain" => self.sustain.as_ptr(),
            "release" => self.release.as_ptr(),
            "attack_curve" => self.attack_curve.as_ptr(),
            "decay_curve" => self.decay_curve.as_ptr(),
            "release_curve" => self.release_curve.as_ptr(),
            "env" => self.env.as_ptr(),
            "vel" => self.vel.as_ptr(),
            "touch" => self.touch.as_ptr(),
            "wheel" => self.wheel.as_ptr(),
            "key" => self.key.as_ptr(),
            "vel_to" => self.vel_to.as_ptr(),
            "touch_to" => self.touch_to.as_ptr(),
            "wheel_to" => self.wheel_to.as_ptr(),
            "key_to" => self.key_to.as_ptr(),
            _ => return None,
        })
    }

    pub fn settings(&self) -> ModSettings {
        ModSettings {
            shape: Shape::from_index(self.shape.value() as usize),
            hz: self.rate.value(),
            sync: self.sync.value().then(|| modulation::DIVISIONS[(self.division.value() as usize).min(14)].0),
            retrig: self.retrig.value(),
            lfo: self.lfo.value(),
            attack: self.attack.value(),
            decay: self.decay.value(),
            sustain: self.sustain.value(),
            release: self.release.value(),
            attack_curve: self.attack_curve.value(),
            decay_curve: self.decay_curve.value(),
            release_curve: self.release_curve.value(),
            env: self.env.value(),
            vel: self.vel.value(),
            touch: self.touch.value(),
            wheel: self.wheel.value(),
            key: self.key.value(),
            vel_to: Dest::from_index(self.vel_to.value() as usize),
            touch_to: Dest::from_index(self.touch_to.value() as usize),
            wheel_to: Dest::from_index(self.wheel_to.value() as usize),
            key_to: Dest::from_index(self.key_to.value() as usize),
        }
    }
}

/// "Clock Ratio" -> "clock_ratio", "Amp Sustain +" -> "amp_sustain".
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

/// "menu{'Sine':0;'Triangle':1}" -> ["Sine", "Triangle"], ordered by value.
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
        assert_eq!(slug("Clock Ratio"), "clock_ratio");
        assert_eq!(slug("Amp Sustain + (X)"), "amp_sustain_x"); // punctuation collapses
        assert_eq!(slug("4-Pole"), "4_pole");
    }

    #[test]
    fn menus() {
        assert_eq!(
            parse_menu("menu{'Sine':0;'Triangle':1;'Saw':2}").unwrap(),
            vec!["Sine", "Triangle", "Saw"]
        );
    }

    #[test]
    fn every_faust_control_becomes_a_unique_param() {
        let params = SynthParams::from_faust();
        let map = params.param_map();
        let mut ids: Vec<_> = map.iter().map(|(id, _, _)| id.clone()).collect();
        let n = ids.len();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), n, "duplicate parameter ids");
        assert!(n > 30, "expected the full voice, got {n} params");
        assert!(ids.contains(&"filter.cutoff".to_string()));
        assert!(!ids.iter().any(|id| id.ends_with(".gate")));
    }

    /// Lists every host parameter: `cargo test --release -- --ignored --nocapture list_params`
    #[test]
    #[ignore]
    fn list_params() {
        for (id, ptr, _) in SynthParams::from_faust().param_map() {
            println!("{id}\t{}", unsafe { ptr.name() });
        }
    }

    #[test]
    fn names_carry_their_panel() {
        let params = SynthParams::from_faust();
        let names: Vec<String> = params
            .param_map()
            .iter()
            .map(|(_, ptr, _)| unsafe { ptr.name() }.to_string())
            .collect();
        for expected in ["Mix: Noise", "Transient: Level", "Unison: Detune", "Master: Volume"] {
            assert!(names.iter().any(|n| n == expected), "missing {expected}");
        }
    }
}
