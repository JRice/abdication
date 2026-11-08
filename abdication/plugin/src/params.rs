//! Host parameters, built at startup from the Faust voice's own UI
//! description (all its DSPs: the voice, a unison copy of its Dual and
//! Wavetable, its free modulators and its texture), and the calls bus. Add or change a control
//! in the .dsp files, regenerate the Rust code, and it shows up here with
//! the same name, range and menu. A control two DSPs both declare is one
//! host parameter that sets both.
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
use crate::mod_dsp::ModDsp;
use crate::osc_dsp::OscDsp;
use crate::texture_dsp::TextureDsp;

const GLIDE_MIN_MS: f32 = 30.0;
const GLIDE_MAX_MS: f32 = 300.0;

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
const VOICE_CONTROLS: [&str; 13] = ["freq", "gain", "gate", "seed", "bend", "modwheel", "aftertouch", "glide",
                                      "slot", "volume", "tempo", "beats", "copy"];

/// Which of a voice's Faust DSPs a control belongs to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Dsp {
    /// The voice itself (dsp.rs).
    Voice,
    /// A unison copy of its Dual and Wavetable (osc_dsp.rs).
    Osc,
    /// Its free modulators, Mod Env and Mod LFO (mod_dsp.rs).
    Mod,
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
/// of a voice's DSPs).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Targets([Option<FaustParam>; 4]);

impl Targets {
    fn add(&mut self, param: FaustParam) {
        let free = self.0.iter_mut().find(|t| t.is_none()).expect("a control in more than four DSPs");
        *free = Some(param);
    }

    pub fn iter(&self) -> impl Iterator<Item = FaustParam> + '_ {
        self.0.iter().flatten().copied()
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
}

/// Where the voice's plugin-driven controls live in its Faust DSPs.
#[derive(Clone, Copy)]
pub struct VoiceSlots {
    pub freq: Targets,
    pub gain: Targets,
    pub gate: Targets,
    pub seed: Targets,
    pub bend: Targets,
    pub modwheel: Targets,
    pub aftertouch: Targets,
    pub glide: Targets,
    pub slot: Targets,
    /// The Volume knob's travel (the voice applies Volume's sends).
    pub volume: Targets,
    /// Host tempo (BPM) and song position (beats, modulo 48) for the synced LFO.
    pub tempo: Targets,
    pub beats: Targets,
    /// Which unison copy a copy of the Dual and Wavetable is.
    pub copy: Targets,
    /// Unison on/off (a host parameter): how many copies the plugin runs.
    pub unison: Targets,
    /// Whether the plugin runs the texture DSP (host parameters): Events
    /// Depth above 0, or Calls Level above its bottom, unless muted.
    pub events_depth: Targets,
    pub events_mute: Targets,
    pub calls_level: Targets,
    pub calls_mute: Targets,
}

pub struct SynthParams {
    entries: Vec<Entry>,
    slots: VoiceSlots,
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
        collector.dsp = Dsp::Mod;
        ModDsp::build_user_interface_static(&mut collector);
        collector.dsp = Dsp::Texture;
        TextureDsp::build_user_interface_static(&mut collector);
        collector.dsp = Dsp::CallBus;
        CallBusDsp::build_user_interface_static(&mut collector);

        // A plugin-driven control, wherever the DSPs declare it.
        let find = |name: &str| {
            let mut targets = Targets::default();
            for dsp in [Dsp::Voice, Dsp::Osc, Dsp::Mod, Dsp::Texture] {
                if let Some(&index) = collector.voice_slots.get(&(dsp, name.to_string())) {
                    targets.add(FaustParam { dsp, index });
                }
            }
            assert!(targets.iter().next().is_some(), "no Faust DSP has a '{name}' control");
            targets
        };
        let host = |id: &str| collector.entries.iter().find(|e| e.id == id).unwrap_or_else(|| panic!("no '{id}' control")).faust;
        let slots = VoiceSlots {
            freq: find("freq"),
            gain: find("gain"),
            gate: find("gate"),
            seed: find("seed"),
            bend: find("bend"),
            modwheel: find("modwheel"),
            aftertouch: find("aftertouch"),
            glide: find("glide"),
            slot: find("slot"),
            volume: find("volume"),
            tempo: find("tempo"),
            beats: find("beats"),
            copy: find("copy"),
            unison: host("unison.mode"),
            events_depth: host("events.depth"),
            events_mute: host("events.mute"),
            calls_level: host("calls.level"),
            calls_mute: host("calls.mute"),
        };

        Self {
            entries: collector.entries,
            slots,
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
                    // Anything typed is on: 30 ms (or less) is the knob's first step.
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

    /// The host parameter behind an id ("filter.cutoff", "master.volume").
    pub fn ptr(&self, id: &str) -> Option<ParamPtr> {
        match id {
            "master.volume" => return Some(self.volume.as_ptr()),
            "master.portamento" => return Some(self.portamento.as_ptr()),
            _ => {}
        }
        self.entries.iter().find(|e| e.id == id).map(|e| e.control.ptr())
    }

    /// Every parameter id, in Faust order (plus the master volume).
    pub fn ids(&self) -> impl Iterator<Item = &str> {
        self.entries.iter().map(|e| e.id.as_str()).chain(["master.volume", "master.portamento"])
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
}

impl Default for Collector {
    fn default() -> Self {
        Self { dsp: Dsp::Voice, groups: Vec::new(), meta: HashMap::new(), entries: Vec::new(),
               voice_slots: HashMap::new() }
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
        self.entries.push(Entry { id, group, faust, control });
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
        let range = match (self.meta(param, "scale"), skew) {
            (_, Some(factor)) => FloatRange::Skewed { min, max, factor },
            (Some("log"), None) => FloatRange::Skewed { min, max, factor: FloatRange::skew_factor(-2.0) },
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
