//! Host parameters, built at startup from the Faust voice's own UI
//! description (all its DSPs: a unison copy of its Dual and Wavetable, its
//! mixer, and the rest of the voice). Add or change a control
//! in the .dsp files, regenerate the Rust code, and it shows up here with
//! the same name, range and menu. A control two DSPs both declare is one
//! host parameter that sets both.
//!
//! Then how far each modulation source moves each modulatable knob
//! (modulation.rs), which are the plugin's own: "mod.<knob>.<source>",
//! e.g. "mod.filter_cutoff.x" (LFO X on the cutoff). Envelope B's on the
//! cutoff is the Filter plate's Env B fader ("filter.env_b").
//!
//! Parameter IDs are "group.label" slugs, so they stay stable across
//! builds (saved projects keep their settings) as long as the Faust
//! group and label don't change.

use std::collections::HashMap;
use std::sync::Arc;

use faust_types::{FaustDsp, ParamIndex, UI};
use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;

use crate::dsp::VoiceDsp;
use crate::mix_dsp::MixDsp;
use crate::modulation::{self, Travel, SOURCES, SOURCE_COUNT, TARGETS};
use crate::osc_dsp::OscDsp;
use crate::pitch;

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
const VOICE_CONTROLS: [&str; 15] = ["freq", "gain", "gate", "seed", "bend", "glide", "slot", "copy", "ratio", "ramp",
                                    "vibrato", "morph1", "fade1", "morph2", "fade2"];

/// Which of a voice's Faust DSPs a control belongs to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Dsp {
    /// The voice from its filter on (dsp.rs).
    Voice,
    /// The voice up to its filter: mixer, noise, transient, drive (mix_dsp.rs).
    Mix,
    /// A unison copy of its Dual and Wavetable (osc_dsp.rs).
    Osc,
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
/// of a voice's DSPs); for a modulatable knob, its place in
/// `modulation::TARGETS` (the engine then moves it per voice); and for one
/// of the modulators' own controls, its place in `modulation::FIELDS` (the
/// engine reads it).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Targets {
    params: [Option<FaustParam>; 6],
    pub modulated: Option<u8>,
    pub field: Option<u8>,
}

impl Targets {
    fn add(&mut self, param: FaustParam) {
        let free = self.params.iter_mut().find(|t| t.is_none()).expect("a control in more than six DSPs");
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
    /// Which unison copy a copy of the Dual and Wavetable is, and its pitch
    /// against the note's (the plugin's unison layout).
    pub copy: Targets,
    pub ratio: Targets,
    /// Samples the modulated controls take to reach a new value (the block's length).
    pub ramp: Targets,
    /// LFO Z's vibrato (semitones), and each slot's wavetable path (morph,
    /// fade), set per voice.
    pub vibrato: Targets,
    pub paths: [[Targets; 2]; 2],
    /// The modulatable knobs (modulation::TARGETS), and their travel.
    pub mod_targets: [Targets; modulation::COUNT],
    pub mod_travel: [Travel; modulation::COUNT],
    /// Unison's mode (a host parameter): how many copies the plugin runs;
    /// and its knobs the plugin reads (UNISON_FIELDS).
    pub unison: Targets,
    pub unison_fields: [Targets; UNISON_FIELDS.len()],
    /// Each oscillator slot's controls that the plugin's Ray reads (RAY_FIELDS).
    pub ray: [[Targets; RAY_FIELDS.len()]; 2],
    /// Each oscillator slot's controls that the plugin's Orthogonal reads (ORTHO_FIELDS).
    pub ortho: [[Targets; ORTHO_FIELDS.len()]; 2],
    /// Each oscillator slot's controls that the plugin's Windows reads (WINDOWS_FIELDS).
    pub windows: [[Targets; WINDOWS_FIELDS.len()]; 2],
    /// The Harmonic EQ's controls (HARMONIC_FIELDS).
    pub harmonic: [Targets; HARMONIC_FIELDS.len()],
}

/// What the plugin's unison reads, in this order (voice.rs's unison_values).
pub const UNISON_FIELDS: [&str; 4] = ["unison.detune", "unison.spread", "unison.eco", "unison.random_phase"];

/// What the plugin's Harmonic EQ reads, in this order (harmonic::HarmonicSettings::from_values).
pub const HARMONIC_FIELDS: [&str; 5] = ["harmonic.shape", "harmonic.shift", "harmonic.tilt", "harmonic.depth",
                                        "harmonic.odd_even"];

/// What the plugin's Ray reads of slot N (osc_N.mode, mix.balance, ray_N.*),
/// in this order (voice.rs's RayField). Balance stands in for the slot's
/// fader: voice.rs turns it into the slot's own level.
pub const RAY_FIELDS: [&str; 13] = ["osc_N.mode", "mix.balance", "ray_N.element", "ray_N.chaos_seed", "ray_N.free",
                                     "ray_N.squish", "ray_N.skew", "ray_N.tilt", "ray_N.center", "ray_N.ray",
                                     "ray_N.orbit", "ray_N.orbit_rate", "ray_N.fine"];

/// What the plugin's Orthogonal reads of slot N, in this order
/// (ortho::player::OrthoSettings::from_values).
pub const ORTHO_FIELDS: [&str; 12] = ["osc_N.mode", "mix.balance", "ortho_N.top", "ortho_N.bottom", "ortho_N.top_position",
                                       "ortho_N.bottom_position", "ortho_N.top_speed", "ortho_N.bottom_speed",
                                       "ortho_N.mix", "settings.ortho_scatter", "ortho_N.top_pitch",
                                       "ortho_N.bottom_pitch"];

/// What the plugin's Windows reads of slot N, in this order
/// (windows::WindowsSettings::from_values): the mode, the fader, the base,
/// then each interrupt's On, Shape, X, Phase, Len and Pitch.
pub const WINDOWS_FIELDS: [&str; 21] = ["osc_N.mode", "mix.balance", "windows_N.base",
    "windows_N.int_1_on", "windows_N.int_1_shape", "windows_N.int_1_x", "windows_N.int_1_phase", "windows_N.int_1_len",
    "windows_N.int_1_pitch",
    "windows_N.int_2_on", "windows_N.int_2_shape", "windows_N.int_2_x", "windows_N.int_2_phase", "windows_N.int_2_len",
    "windows_N.int_2_pitch",
    "windows_N.int_3_on", "windows_N.int_3_shape", "windows_N.int_3_x", "windows_N.int_3_phase", "windows_N.int_3_len",
    "windows_N.int_3_pitch"];

pub struct SynthParams {
    entries: Vec<Entry>,
    slots: VoiceSlots,
    /// How far each source moves each modulatable knob: TARGETS x SOURCES,
    /// row by row (None for envelope B on the cutoff, which is the Env B fader).
    pub depths: Vec<Option<FloatParam>>,
    depth_ids: Vec<String>,
    /// Plugin-level output volume (not part of the Faust voice).
    pub volume: FloatParam,
    /// 0 = polyphonic; above 0 = mono with a glide of 30-300 ms.
    pub portamento: FloatParam,
    /// Editor window size and scale (fixed; not saved).
    pub editor_state: Arc<ViziaState>,
}

impl SynthParams {
    pub fn from_faust() -> Self {
        // The mixer first, so the host lists the controls in the panel's order.
        let mut collector = Collector::default();
        collector.dsp = Dsp::Mix;
        MixDsp::build_user_interface_static(&mut collector);
        collector.dsp = Dsp::Voice;
        VoiceDsp::build_user_interface_static(&mut collector);
        collector.dsp = Dsp::Osc;
        OscDsp::build_user_interface_static(&mut collector);

        // A plugin-driven control, wherever the DSPs declare it.
        let find = |name: &str| {
            let mut targets = Targets::default();
            for dsp in [Dsp::Mix, Dsp::Voice, Dsp::Osc] {
                if let Some(&index) = collector.voice_slots.get(&(dsp, name.to_string())) {
                    targets.add(FaustParam { dsp, index });
                }
            }
            assert!(targets.iter().next().is_some(), "no Faust DSP has a '{name}' control");
            targets
        };
        // The modulatable knobs know their place, and so do the controls
        // the engine reads.
        for (t, target) in TARGETS.iter().enumerate() {
            let entry = collector.entries.iter_mut().find(|e| e.id == target.id)
                .unwrap_or_else(|| panic!("no modulatable control '{}'", target.id));
            assert!(entry.travel.is_some(), "'{}' isn't a float knob", target.id);
            entry.faust.modulated = Some(t as u8);
        }
        for (f, id) in modulation::FIELDS.iter().enumerate() {
            let entry = collector.entries.iter_mut().find(|e| e.id == *id)
                .unwrap_or_else(|| panic!("no control '{id}' for the engine"));
            entry.faust.field = Some(f as u8);
        }
        let host = |id: &str| collector.entries.iter().find(|e| e.id == id).unwrap_or_else(|| panic!("no '{id}' control")).faust;
        let entry = |id: &str| collector.entries.iter().find(|e| e.id == id).unwrap();
        let mut depths = Vec::with_capacity(modulation::COUNT * SOURCE_COUNT);
        let mut depth_ids = Vec::with_capacity(modulation::COUNT * SOURCE_COUNT);
        for (t, target) in TARGETS.iter().enumerate() {
            let name = match &entry(target.id).control {
                Control::Float(p) => p.name().to_string(),
                _ => unreachable!(),
            };
            for s in 0..SOURCE_COUNT {
                let id = modulation::depth_id(t, s);
                let wired = !id.starts_with("mod.");
                depths.push((!wired).then(|| {
                    percent(format!("{name} {}", SOURCES[s].name), modulation::default_depth(t, s)).hide().non_automatable()
                }));
                depth_ids.push(id);
            }
        }
        let slots = VoiceSlots {
            freq: find("freq"),
            gain: find("gain"),
            gate: find("gate"),
            seed: find("seed"),
            bend: find("bend"),
            glide: find("glide"),
            slot: find("slot"),
            copy: find("copy"),
            ratio: find("ratio"),
            ramp: find("ramp"),
            vibrato: find("vibrato"),
            paths: [[find("morph1"), find("fade1")], [find("morph2"), find("fade2")]],
            mod_targets: std::array::from_fn(|t| host(TARGETS[t].id)),
            mod_travel: std::array::from_fn(|t| entry(TARGETS[t].id).travel.unwrap()),
            unison: host("unison.mode"),
            unison_fields: std::array::from_fn(|f| host(UNISON_FIELDS[f])),
            ray: std::array::from_fn(|slot| {
                std::array::from_fn(|f| host(&RAY_FIELDS[f].replace('N', &(slot + 1).to_string())))
            }),
            ortho: std::array::from_fn(|slot| {
                std::array::from_fn(|f| host(&ORTHO_FIELDS[f].replace('N', &(slot + 1).to_string())))
            }),
            windows: std::array::from_fn(|slot| {
                std::array::from_fn(|f| host(&WINDOWS_FIELDS[f].replace('N', &(slot + 1).to_string())))
            }),
            harmonic: std::array::from_fn(|f| host(HARMONIC_FIELDS[f])),
        };

        Self {
            entries: collector.entries,
            slots,
            depths,
            depth_ids,
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
    /// "mod.filter_cutoff.x").
    pub fn ptr(&self, id: &str) -> Option<ParamPtr> {
        match id {
            "master.volume" => return Some(self.volume.as_ptr()),
            "master.portamento" => return Some(self.portamento.as_ptr()),
            _ => {}
        }
        if let Some((t, s)) = modulation::parse_depth_id(id) {
            return self.depth_ptr(t, s);
        }
        self.entries.iter().find(|e| e.id == id).map(|e| e.control.ptr())
    }

    /// The parameter that is source `s`'s depth on knob `t` (envelope B's
    /// on the cutoff is the Env B fader).
    pub fn depth_ptr(&self, t: usize, s: usize) -> Option<ParamPtr> {
        match &self.depths[t * SOURCE_COUNT + s] {
            Some(p) => Some(p.as_ptr()),
            None => self.ptr(&self.depth_ids[t * SOURCE_COUNT + s]),
        }
    }

    /// Source `s`'s depth on knob `t`, as set.
    pub fn depth(&self, t: usize, s: usize) -> f32 {
        self.depth_ptr(t, s).map_or(0.0, |p| unsafe { p.unmodulated_plain_value() })
    }

    /// Every parameter id: Faust order, the master knobs, then the depths.
    pub fn ids(&self) -> impl Iterator<Item = &str> {
        self.entries.iter().map(|e| e.id.as_str()).chain(["master.volume", "master.portamento"])
            .chain(self.depth_ids.iter().zip(&self.depths).filter(|(_, p)| p.is_some()).map(|(id, _)| id.as_str()))
    }

    /// Every depth that's set: (knob, source, depth).
    pub fn depths_set(&self) -> impl Iterator<Item = (usize, usize, f32)> + '_ {
        (0..modulation::COUNT).flat_map(move |t| (0..SOURCE_COUNT).map(move |s| (t, s)))
            .map(|(t, s)| (t, s, self.depth(t, s)))
            .filter(|(_, _, d)| *d != 0.0)
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
        for (id, p) in self.depth_ids.iter().zip(&self.depths) {
            if let Some(p) = p {
                map.push((id.clone(), p.as_ptr(), "Modulation".into()));
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
    /// the host's list ("Mix: Noise", "Transient: Level"). The oscillator
    /// slots' groups end in 1 and 2 (their ids do too); the panel calls
    /// them I and II, and so do the names.
    fn display(&self, label: &str) -> String {
        let group = self.group();
        let group = match group.rsplit_once(' ') {
            Some((name, "1")) => format!("{name} I"),
            Some((name, "2")) => format!("{name} II"),
            _ => group,
        };
        match group {
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
        // [style:pitch]: an oscillator slot's Pitch, read as an interval (pitch.rs).
        if self.meta(param, "style") == Some("pitch") {
            p = p.with_value_to_string(Arc::new(pitch::name)).with_string_to_value(Arc::new(pitch::parse));
        }
        // [style:detune]: unison's Detune, with the outer pair's cents at A4.
        if self.meta(param, "style") == Some("detune") {
            p = p.with_value_to_string(Arc::new(crate::unison::detune_name));
        }
        // [style:hpf]: the HPF, off at the bottom of its travel.
        if self.meta(param, "style") == Some("hpf") {
            p = p.with_value_to_string(Arc::new(|hz| {
                if hz < 10.5 { "Off".into() } else if hz < 1000.0 { format!("{hz:.0} Hz") } else { format!("{:.2} kHz", hz / 1000.0) }
            }));
        }
        // [style:ratio]: octaves shown as the frequency ratio they make (×0.25..×64).
        if self.meta(param, "style") == Some("ratio") {
            p = p.with_value_to_string(Arc::new(pitch::ratio_name)).with_string_to_value(Arc::new(pitch::parse_ratio));
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

/// An amount, -100..100 %: curved so small ones get more of the knob (a
/// quarter turn from the middle is 25 %), and shown to a tenth below 10 %.
fn percent(name: String, default: f32) -> FloatParam {
    let range = FloatRange::SymmetricalSkewed { min: -1.0, max: 1.0, factor: FloatRange::skew_factor(-1.0), center: 0.0 };
    FloatParam::new(name, default, range)
        .with_value_to_string(Arc::new(modulation::percent_text))
        .with_string_to_value(Arc::new(|s| s.trim().trim_end_matches('%').trim().parse::<f32>().ok().map(|v| v / 100.0)))
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
