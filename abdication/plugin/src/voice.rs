//! One voice's DSPs: seven unison copies of its two oscillators
//! (`OscDsp`; five run with Unison on U5, only the centre one with it off or
//! on Eco), weighed, detuned and panned here (unison.rs), each slot's Harmonic
//! EQ (harmonic.rs, in Rust) on the copies' sum, its mixer (`MixDsp`), which
//! takes those as inputs and gives the voice up to its filter, and the rest
//! of the voice (`VoiceDsp`). They're separate Faust programs for the
//! compiler's sake (see dsp/voice.lib), and so the parts that aren't sounding
//! cost nothing. With the copies spread, the voice is stereo: its Harmonic
//! EQ, mixer and the rest run twice, once for each side's sum (they're
//! nonlinear, so the copies can't share them and be panned after). The
//! knobs' modulators are the engine's (modulation.rs); it sets each
//! modulated control per voice before a block.

use faust_types::FaustDsp;

use crate::dsp::VoiceDsp;
use crate::mix_dsp::MixDsp;
use crate::osc_dsp::OscDsp;
use std::sync::Arc;

use crate::ortho::bank::{self, Bank};
use crate::ortho::player::{OrthoPlayer, OrthoSettings};
use crate::harmonic::{HarmonicEq, HarmonicSettings};
use crate::params::{Dsp, Targets, VoiceSlots, HARMONIC_FIELDS, ORTHO_FIELDS, RAY_FIELDS, UNISON_FIELDS, WINDOWS_FIELDS};
use crate::ray::player::{RayPlayer, RaySettings};
use crate::windows::player::WindowsPlayer;
use crate::windows::WindowsSettings;
use crate::scope::{MOVING, OUTPUTS, RIGHT};
use crate::unison::{self, Place, Snapshots};

/// The most samples one `compute` call handles.
pub const MAX_BLOCK: usize = 64;
const COPIES: usize = 7;
/// The copy that plays with Unison off (and feeds the scopes).
const CENTRE: usize = 3;
/// A copy's outputs: Osc 1 and Osc 2, then (for Ray, Orthogonal and Windows)
/// each slot's pitch, the copy's start phase and note-on.
const COPY_OUTPUTS: usize = 6;
const HZ: usize = 2;
const PHASE: usize = 4;
const NOTE_ON: usize = 5;
/// `MixDsp`'s outputs: the voice before its filter, the post-filter
/// transient, the pitch (`VoiceDsp`'s inputs).
const MIX_OUTPUTS: usize = 3;

type Block = [f32; MAX_BLOCK];

/// The Unison button: off, five copies or seven.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Unison {
    Off,
    U5,
    U7,
}

impl Unison {
    /// From the Unison: Mode parameter's value (0, 1, 2).
    pub fn from_value(value: f32) -> Self {
        match value.round() as i32 {
            1 => Unison::U5,
            2 => Unison::U7,
            _ => Unison::Off,
        }
    }

    /// The copies that run, left to right (U5 leaves out copies 1 and 5).
    pub(crate) fn copies(self) -> &'static [usize] {
        match self {
            Unison::Off => &[CENTRE],
            Unison::U5 => &[0, 2, CENTRE, 4, 6],
            Unison::U7 => &[0, 1, 2, CENTRE, 4, 5, 6],
        }
    }
}

/// One side of a voice from its copies' sum on: each slot's Harmonic EQ, the
/// mixer and the rest. The voice has two; the right one runs only while the
/// copies are spread.
struct Chain {
    harmonics: [HarmonicEq; 2],
    mix: Box<MixDsp>,
    voice: Box<VoiceDsp>,
    /// The mixer's outputs (MIX_OUTPUTS).
    mix_out: [Block; MIX_OUTPUTS],
    /// The voice's own outputs: the voice, its moving values.
    voice_out: [Block; 1 + MOVING],
}

impl Default for Chain {
    fn default() -> Self {
        Self {
            harmonics: Default::default(),
            mix: Box::new({
                crate::harmlib::prepare();
                MixDsp::new()
            }),
            voice: Box::new(VoiceDsp::new()),
            mix_out: [[0.0; MAX_BLOCK]; MIX_OUTPUTS],
            voice_out: [[0.0; MAX_BLOCK]; 1 + MOVING],
        }
    }
}

/// A Faust DSP's whole state copied into another of the same program (they're
/// plain numbers and arrays, which `needs_drop` vouches for).
fn copy_dsp<T>(to: &mut T, from: &T) {
    assert!(!std::mem::needs_drop::<T>());
    // SAFETY: T owns nothing (no Drop), so a bitwise copy is a whole, valid T.
    unsafe { std::ptr::copy_nonoverlapping(from as *const T, to as *mut T, 1) }
}

impl Chain {
    fn init(&mut self, sample_rate: f32) {
        self.mix.init(sample_rate as i32);
        self.voice.init(sample_rate as i32);
    }

    fn instance_clear(&mut self) {
        self.mix.instance_clear();
        self.voice.instance_clear();
        self.harmonics.iter_mut().for_each(HarmonicEq::clear);
    }

    /// Take over the other chain's state, so it carries on from where that one is.
    fn copy_from(&mut self, other: &Chain) {
        copy_dsp(&mut *self.mix, &*other.mix);
        copy_dsp(&mut *self.voice, &*other.voice);
        for (h, o) in self.harmonics.iter_mut().zip(&other.harmonics) {
            h.copy_from(o);
        }
    }

    /// The side from its copies' sum (`sums`, the Harmonic EQ applied in place).
    fn compute(&mut self, sums: &mut [Block; 2], harmonic: Option<&HarmonicSettings>, sounding: [bool; 2],
               slot_hz: [f32; 2], rate: f32, n: usize) {
        if let Some(h) = harmonic {
            for s in (0..2).filter(|&s| sounding[s]) {
                self.harmonics[s].process(h, slot_hz[s], rate, &mut sums[s][..n]);
            }
        }
        let [osc1, osc2] = &*sums;
        let mut out = self.mix_out.each_mut().map(|o| &mut o[..n]);
        self.mix.compute(n as i32, &[&osc1[..n], &osc2[..n]], &mut out);
        let [pre, tr_post, hz] = &self.mix_out;
        let inputs: [&[f32]; 3] = [&pre[..n], &tr_post[..n], &hz[..n]];
        let mut out = self.voice_out.each_mut().map(|o| &mut o[..n]);
        self.voice.compute(n as i32, &inputs, &mut out);
    }
}

pub struct VoiceUnit {
    /// Left (or both, while the copies aren't spread) and right.
    chains: [Chain; 2],
    /// Whether the right chain is running.
    stereo: bool,
    copies: Vec<OscDsp>,
    /// The slots set to Ray, played in Rust for each copy.
    rays: RayPlayer,
    /// Where the Ray's controls live (params::RAY_FIELDS), per slot.
    ray_fields: [[Targets; RAY_FIELDS.len()]; 2],
    /// The slots set to Orthogonal, played in Rust for each copy, from the
    /// shared sample bank; where their controls live (params::ORTHO_FIELDS).
    orthos: OrthoPlayer,
    ortho_fields: [[Targets; ORTHO_FIELDS.len()]; 2],
    bank: Arc<Bank>,
    /// The slots set to Windows, played in Rust for each copy; where their
    /// controls live (params::WINDOWS_FIELDS).
    windows: WindowsPlayer,
    windows_fields: [[Targets; WINDOWS_FIELDS.len()]; 2],
    /// The gate (as the engine sets it), and whether it rose since the last block.
    gate: Targets,
    gated: bool,
    note_on: bool,
    /// Each Orthogonal's players' positions (share of their sets) after the
    /// last block; NaN for a slot that isn't one.
    ortho_positions: [[f32; 2]; 2],
    /// Which copies play.
    pub unison: Unison,
    /// Where unison's knobs live (params::UNISON_FIELDS), each copy's pitch
    /// ratio, the note's pitch and seed (as the engine set them), and the
    /// copies' places in the last block (their gains ramp from there).
    unison_fields: [Targets; UNISON_FIELDS.len()],
    ratio: Targets,
    freq: Targets,
    seed: Targets,
    note_hz: f32,
    note_seed: f32,
    last: Option<([Place; 7], usize)>,
    /// Each slot's snapshots for Eco, and whether Eco played the last block.
    snapshots: Vec<Snapshots>,
    eco: bool,
    /// Where the Harmonic EQ's controls live (params::HARMONIC_FIELDS), and
    /// each slot's pitch (the centre copy's) in the last block.
    harmonic_fields: [Targets; HARMONIC_FIELDS.len()],
    slot_hz: [f32; 2],
    sample_rate: f32,
    /// Where the modulated controls' ramp length lives in each DSP.
    ramp: Targets,
    /// One copy's latest block (COPY_OUTPUTS).
    copy_out: [Block; COPY_OUTPUTS],
    /// The copies summed, left and right: [Osc 1, Osc 2] each.
    sums: [[Block; 2]; 2],
}

impl Default for VoiceUnit {
    fn default() -> Self {
        Self {
            chains: Default::default(),
            stereo: false,
            copies: (0..COPIES).map(|_| OscDsp::new()).collect(),
            rays: RayPlayer::new(COPIES, 0x5EED),
            ray_fields: Default::default(),
            orthos: OrthoPlayer::new(COPIES, 0x0A7D),
            ortho_fields: Default::default(),
            bank: bank::shared(),
            windows: WindowsPlayer::new(COPIES),
            windows_fields: Default::default(),
            gate: Targets::default(),
            gated: false,
            note_on: false,
            ortho_positions: [[f32::NAN; 2]; 2],
            unison: Unison::Off,
            unison_fields: Default::default(),
            ratio: Targets::default(),
            freq: Targets::default(),
            seed: Targets::default(),
            note_hz: 440.0,
            note_seed: 0.0,
            last: None,
            snapshots: Vec::new(),
            eco: false,
            harmonic_fields: Default::default(),
            slot_hz: [110.0; 2],
            sample_rate: 48_000.0,
            ramp: Targets::default(),
            copy_out: [[0.0; MAX_BLOCK]; COPY_OUTPUTS],
            sums: [[[0.0; MAX_BLOCK]; 2]; 2],
        }
    }
}

impl VoiceUnit {
    pub fn init(&mut self, sample_rate: f32, slots: &VoiceSlots) {
        self.chains.iter_mut().for_each(|c| c.init(sample_rate));
        self.harmonic_fields = slots.harmonic;
        self.unison_fields = slots.unison_fields;
        self.ratio = slots.ratio;
        self.freq = slots.freq;
        self.seed = slots.seed;
        self.snapshots = (0..2).map(|_| Snapshots::new(sample_rate)).collect();
        self.sample_rate = sample_rate;
        self.ramp = slots.ramp;
        self.ray_fields = slots.ray;
        self.rays.init(sample_rate, MAX_BLOCK);
        self.ortho_fields = slots.ortho;
        self.gate = slots.gate;
        self.orthos.init(sample_rate);
        self.windows_fields = slots.windows;
        self.windows.init(sample_rate);
        for (k, copy) in self.copies.iter_mut().enumerate() {
            copy.init(sample_rate as i32);
            for t in slots.copy.iter() {
                copy.set_param(t.index, k as f32);
            }
        }
    }

    pub fn instance_clear(&mut self) {
        self.chains.iter_mut().for_each(Chain::instance_clear);
        self.stereo = false;
        self.last = None;
        self.eco = false;
        self.copies.iter_mut().for_each(|c| c.instance_clear());
        self.rays.clear();
        self.orthos.clear();
        self.windows.clear();
    }

    /// A control's value in the copies' DSP (as the engine set it for this voice).
    fn osc_value(&self, t: Targets) -> f32 {
        t.iter().find(|p| p.dsp == Dsp::Osc).and_then(|p| self.copies[CENTRE].get_param(p.index)).unwrap_or(0.0)
    }

    /// Slot `s`'s Ray settings, as this voice has them now.
    fn ray_settings(&self, s: usize) -> RaySettings {
        RaySettings::from_values(self.ray_fields[s].map(|t| self.osc_value(t)))
    }

    /// Slot `s`'s Windows settings, as this voice has them now.
    fn windows_settings(&self, s: usize) -> WindowsSettings {
        WindowsSettings::from_values(self.windows_fields[s].map(|t| self.osc_value(t)))
    }

    /// Slot `s`'s Orthogonal settings, as this voice has them now.
    pub fn ortho_settings(&self, s: usize) -> OrthoSettings {
        OrthoSettings::from_values(self.ortho_fields[s].map(|t| self.osc_value(t)))
    }

    /// Ask the bank for the sets the slots on Orthogonal play, so they're
    /// decoded before a note needs them.
    pub fn want_samples(&self) {
        for s in 0..2 {
            let o = self.ortho_settings(s);
            if o.playing() {
                (0..2).for_each(|h| self.bank.want(o.set_of(h)));
            }
        }
    }

    /// Each Orthogonal's players' positions after the last block (NaN: not one).
    pub fn ortho_positions(&self) -> [[f32; 2]; 2] {
        self.ortho_positions
    }

    /// Set a control in every DSP that has it.
    pub fn set(&mut self, targets: Targets, value: f32) {
        if targets == self.gate {
            let gated = value > 0.5;
            self.note_on |= gated && !self.gated;
            self.gated = gated;
        } else if targets == self.freq {
            self.note_hz = value;
        } else if targets == self.seed {
            self.note_seed = value;
        }
        for t in targets.iter() {
            match t.dsp {
                Dsp::Voice => self.chains.iter_mut().for_each(|c| c.voice.set_param(t.index, value)),
                Dsp::Mix => self.chains.iter_mut().for_each(|c| c.mix.set_param(t.index, value)),
                Dsp::Osc => self.copies.iter_mut().for_each(|c| c.set_param(t.index, value)),
            }
        }
    }

    /// Unison's knobs as this voice has them now: Detune, Spread, Eco, Random Phase.
    fn unison_values(&self) -> [f32; UNISON_FIELDS.len()] {
        self.unison_fields.map(|t| self.osc_value(t))
    }

    /// Compute `outputs[0].len()` samples (at most MAX_BLOCK) of every
    /// output (see scope::OUTPUTS: the voice's left, the centre copy's Osc 1
    /// and Osc 2, the moving values, then the voice's right).
    pub fn compute(&mut self, outputs: &mut [&mut [f32]; OUTPUTS]) {
        let n = outputs[0].len().min(MAX_BLOCK);
        // Modulated controls reach this block's values by its end.
        self.set(self.ramp, n as f32);

        for side in &mut self.sums {
            for sum in side {
                sum[..n].fill(0.0);
            }
        }
        let [detune, spread, eco, random_phase] = self.unison_values();
        let layout = unison::layout(self.unison, detune, spread, self.note_hz);
        let gate_rose = std::mem::take(&mut self.note_on);
        // Gains ramp over the block from where the last one left them (a new
        // note starts at its own).
        let from = match self.last {
            Some((places, count)) if count == layout.count && !gate_rose => places,
            _ => layout.places,
        };
        self.last = Some((layout.places, layout.count));
        let stereo = layout.stereo() || from[..layout.count].iter().any(|p| p.left != p.right);
        let eco = eco > 0.5 && self.unison != Unison::Off;

        let rays = [0, 1].map(|s| self.ray_settings(s));
        let keys = [0, 1].map(|s| rays[s].playing().then(|| self.rays.prepare(s, &rays[s])));
        let orthos = [0, 1].map(|s| self.ortho_settings(s));
        let windows = [0, 1].map(|s| self.windows_settings(s));
        let sounding = [0, 1].map(|s| orthos[s].fader != 0.0);
        // Eco's side copies start each note (or Eco) at their phases: random
        // per note and copy with Random Phase, as the copies' own (voice.lib).
        if eco && (gate_rose || !self.eco) {
            let seed = self.note_seed;
            let phases = std::array::from_fn(|k| match random_phase > 0.5 {
                true => (0.5 + seed * (k + 1) as f32 * 7.31 + k as f32 * 0.618).fract(),
                false => 0.0,
            });
            self.snapshots.iter_mut().for_each(|s| s.start(phases));
        }
        self.eco = eco;
        for (s, o) in orthos.iter().enumerate().filter(|(_, o)| o.playing()) {
            self.orthos.advance(s, o, gate_rose, self.gated, n);
        }
        self.ortho_positions = [0, 1].map(|s| match orthos[s].playing() {
            true => [0, 1].map(|h| self.orthos.position(s, &orthos[s], h)),
            false => [f32::NAN; 2],
        });
        let bank = orthos.iter().any(OrthoSettings::playing).then(|| self.bank.clone());
        let reader = bank.as_ref().map(|b| b.read());
        let sets = [0, 1].map(|s| match (&reader, orthos[s].playing()) {
            (Some(reader), true) => Some(OrthoPlayer::sets(reader, &orthos[s])),
            _ => None,
        });
        let ratio = self.ratio.iter().next().expect("the copies' ratio").index;
        for (i, place) in layout.places().iter().enumerate() {
            let k = place.copy;
            // On Eco only the centre copy is played.
            if eco && k != CENTRE {
                continue;
            }
            self.copies[k].set_param(ratio, place.ratio);
            let mut out = self.copy_out.each_mut().map(|o| &mut o[..n]);
            self.copies[k].compute(n as i32, &[], &mut out);
            // A slot on Ray: the copy gave silence and its pitch; play it here.
            let note_on = self.copy_out[NOTE_ON][..n].iter().any(|t| *t > 0.5);
            for (s, key) in keys.iter().enumerate() {
                let Some(key) = *key else { continue };
                let (hz, phase) = (self.copy_out[HZ + s][n - 1], self.copy_out[PHASE][n - 1]);
                let mut wave = [0.0; MAX_BLOCK];
                self.rays.render(k, s, key, &rays[s], hz, phase, note_on, &mut wave[..n]);
                self.copy_out[s][..n].copy_from_slice(&wave[..n]);
            }
            // A slot on Orthogonal: likewise.
            for (s, sets) in sets.iter().enumerate() {
                let Some(sets) = *sets else { continue };
                let hz = self.copy_out[HZ + s][n - 1];
                let mut wave = [0.0; MAX_BLOCK];
                self.orthos.render(k, s, &orthos[s], sets, hz, &mut wave[..n]);
                self.copy_out[s][..n].copy_from_slice(&wave[..n]);
            }
            // A slot on Windows: likewise.
            for (s, w) in windows.iter().enumerate().filter(|(_, w)| w.playing()) {
                let (hz, phase) = (self.copy_out[HZ + s][n - 1], self.copy_out[PHASE][n - 1]);
                let mut wave = [0.0; MAX_BLOCK];
                self.windows.render(k, s, w, hz, phase, note_on, &mut wave[..n]);
                self.copy_out[s][..n].copy_from_slice(&wave[..n]);
            }
            if k == CENTRE {
                self.slot_hz = [self.copy_out[HZ][n - 1], self.copy_out[HZ + 1][n - 1]];
                outputs[1].copy_from_slice(&self.copy_out[0][..n]);
                outputs[2].copy_from_slice(&self.copy_out[1][..n]);
            }
            // Into each side at the copy's gains.
            let was = from[i];
            let ramp = 1.0 / n as f32;
            let [left, right] = &mut self.sums;
            for s in 0..2 {
                for (j, x) in self.copy_out[s][..n].iter().enumerate() {
                    let t = (j as f32 + 1.0) * ramp;
                    left[s][j] += x * (was.left + (place.left - was.left) * t);
                    right[s][j] += x * (was.right + (place.right - was.right) * t);
                }
            }
            // On Eco, the side copies from snapshots of this one.
            if eco {
                for s in (0..2).filter(|&s| sounding[s]) {
                    self.snapshots[s].play(&self.copy_out[s][..n], &self.copy_out[HZ + s][..n], self.sample_rate,
                                           layout.places(), &from, CENTRE, &mut left[s][..n], &mut right[s][..n]);
                }
            }
        }

        // The right side takes over from the left when the copies spread.
        if stereo && !self.stereo {
            let [left, right] = &mut self.chains;
            right.copy_from(left);
        }
        self.stereo = stereo;
        let harmonic = HarmonicSettings::from_values(self.harmonic_fields.map(|t| self.osc_value(t)));
        let harmonic = harmonic.on().then_some(&harmonic);
        let (slot_hz, rate) = (self.slot_hz, self.sample_rate);
        for side in 0..1 + stereo as usize {
            self.chains[side].compute(&mut self.sums[side], harmonic, sounding, slot_hz, rate, n);
        }
        let [left, right] = &self.chains;
        outputs[0].copy_from_slice(&left.voice_out[0][..n]);
        for (o, v) in outputs[3..RIGHT].iter_mut().zip(&left.voice_out[1..]) {
            o.copy_from_slice(&v[..n]);
        }
        let right = if stereo { &right.voice_out[0] } else { &left.voice_out[0] };
        outputs[RIGHT].copy_from_slice(&right[..n]);
    }
}

#[cfg(test)]
mod pitch_check {
    use super::*;
    use crate::params::SynthParams;
    use crate::pitch;

    /// The pitch a slot plays (its Pitch through voice.lib's slotPitch, as the
    /// copy reports it for the Ray) is the one pitch.rs gives the panel.
    #[test]
    fn faust_and_the_panel_agree_on_pitch() {
        let params = SynthParams::from_faust();
        let slots = params.slots();
        let mut v = VoiceUnit::default();
        v.init(48_000.0, &slots);
        v.set(slots.freq, 100.0);
        v.set(slots.gate, 1.0);
        let knob = params.faust_index("osc_1.pitch").unwrap();
        let mut position = -1.0f32;
        while position <= 1.0 {
            v.set(knob, position);
            // A note-on jumps a modulated control straight to its value.
            let copy = &mut v.copies[CENTRE];
            copy.instance_clear();
            let mut out = v.copy_out.each_mut().map(|o| &mut o[..4]);
            copy.compute(4, &[], &mut out);
            let played = 12.0 * (v.copy_out[HZ][3] / 100.0).log2();
            let shown = pitch::semitones(position);
            assert!((played - shown).abs() < 0.01, "at {position}: plays {played:.3} st, shows {shown:.3}");
            position += 0.005;
        }
    }
}

#[cfg(test)]
mod profile {
    use super::*;
    use crate::params::SynthParams;
    use std::time::Instant;

    /// One voice's Osc 1 alone (the centre copy, before the mixer and filter),
    /// 2 s at ABD_FREQ Hz (default 220), to ABD_DUMP as raw f32. ABD_SET as
    /// in profile_parts. For measuring an oscillator's own aliasing.
    #[test]
    #[ignore]
    fn dump_dual() {
        let Ok(path) = std::env::var("ABD_DUMP") else { return };
        let params = SynthParams::from_faust();
        let slots = params.slots();
        let mut v = VoiceUnit::default();
        v.init(48_000.0, &slots);
        for (t, value) in params.faust_values() {
            v.set(t, value);
        }
        for pair in std::env::var("ABD_SET").unwrap_or_default().split(',').filter(|p| !p.is_empty()) {
            let (id, x) = pair.split_once('=').unwrap();
            v.set(params.faust_index(id).unwrap(), x.parse().unwrap());
        }
        let freq: f32 = std::env::var("ABD_FREQ").ok().and_then(|f| f.parse().ok()).unwrap_or(220.0);
        v.set(slots.freq, freq);
        v.set(slots.gain, 0.8);
        v.set(slots.gate, 1.0);
        let n = MAX_BLOCK;
        let mut all = Vec::new();
        for _ in 0..(96_000 / n) {
            let mut out = v.copy_out.each_mut().map(|o| &mut o[..n]);
            v.copies[CENTRE].compute(n as i32, &[], &mut out);
            all.extend_from_slice(&v.copy_out[0][..n]);
        }
        std::fs::write(path, all.iter().flat_map(|x| x.to_le_bytes()).collect::<Vec<u8>>()).unwrap();
    }

    /// Osc 1's level (RMS and peak, dB) and the µs per block of a voice on
    /// Windows, against a Dual saw, alone and with interrupts:
    /// `cargo test --release windows_levels -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn windows_levels() {
        let params = SynthParams::from_faust();
        let slots = params.slots();
        let run = |sets: &[(&str, f32)]| {
            let mut v = VoiceUnit::default();
            v.init(48_000.0, &slots);
            for (t, value) in params.faust_values() {
                v.set(t, value);
            }
            for (id, x) in sets {
                v.set(params.faust_index(id).unwrap(), *x);
                if *id == "unison.mode" {
                    v.unison = Unison::from_value(*x);
                }
            }
            v.set(slots.freq, 110.0);
            v.set(slots.gain, 0.8);
            v.set(slots.gate, 1.0);
            let mut chunks = [[0.0f32; MAX_BLOCK]; OUTPUTS];
            let mut osc = Vec::new();
            let t = Instant::now();
            for _ in 0..1500 {
                v.compute(&mut chunks.each_mut().map(|c| &mut c[..]));
                osc.extend_from_slice(&chunks[1]);
            }
            let us = t.elapsed().as_secs_f64() * 1e6 / 1500.0;
            let tail = &osc[24_000..];
            let rms = (tail.iter().map(|x| x * x).sum::<f32>() / tail.len() as f32).sqrt();
            let peak = tail.iter().fold(0.0f32, |m, x| m.max(x.abs()));
            (20.0 * rms.log10(), 20.0 * peak.log10(), us)
        };
        let all_on = [("osc_1.mode", 4.0), ("windows_1.int_1_on", 1.0), ("windows_1.int_2_on", 1.0),
                      ("windows_1.int_3_on", 1.0)];
        let pitched = [&all_on[..], &[("windows_1.int_1_pitch", 2.0), ("windows_1.int_2_pitch", 1.0),
                                       ("windows_1.int_3_pitch", 4.58)]].concat();
        for (name, sets) in [("dual saw", &[][..]), ("windows saw", &[("osc_1.mode", 4.0)][..]),
                             ("windows 3 on", &all_on[..]), ("3 on pitched", &pitched[..])] {
            let (r, p, us) = run(sets);
            println!("{name:14} rms {r:6.1} peak {p:6.1}  {us:5.1} us/block");
            let (_, _, us) = run(&[sets, &[("unison.mode", 2.0)]].concat());
            println!("{name:14} U7                      {us:5.1} us/block");
        }
    }

    /// Osc 1's level (RMS and peak, dB) on a Dual saw and on Ray with each
    /// element, then the µs per block of a voice on Ray:
    /// `cargo test --release ray_levels -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn ray_levels() {
        let params = SynthParams::from_faust();
        let slots = params.slots();
        let run = |sets: &[(&str, f32)]| {
            let mut v = VoiceUnit::default();
            v.init(48_000.0, &slots);
            for (t, value) in params.faust_values() {
                v.set(t, value);
            }
            for (id, x) in sets {
                v.set(params.faust_index(id).unwrap(), *x);
                if *id == "unison.mode" {
                    v.unison = Unison::from_value(*x);
                }
            }
            v.set(slots.freq, 110.0);
            v.set(slots.gain, 0.8);
            v.set(slots.gate, 1.0);
            let mut chunks = [[0.0f32; MAX_BLOCK]; OUTPUTS];
            let mut osc = Vec::new();
            let t = Instant::now();
            for _ in 0..1500 {
                v.compute(&mut chunks.each_mut().map(|c| &mut c[..]));
                osc.extend_from_slice(&chunks[1]);
            }
            let us = t.elapsed().as_secs_f64() * 1e6 / 1500.0;
            let tail = &osc[24_000..];
            let rms = (tail.iter().map(|x| x * x).sum::<f32>() / tail.len() as f32).sqrt();
            let peak = tail.iter().fold(0.0f32, |m, x| m.max(x.abs()));
            (20.0 * rms.log10(), 20.0 * peak.log10(), us)
        };
        let (r, p, us) = run(&[]);
        println!("dual saw   rms {r:6.1} peak {p:6.1}  {us:5.1} us/block");
        for (e, name) in ["Earth", "Water", "Air", "Fire", "Wood", "Metal", "Chaos"].iter().enumerate() {
            for center in [0.0, 0.4] {
                let (r, p, us) = run(&[("osc_1.mode", 2.0), ("ray_1.element", e as f32), ("ray_1.center", center)]);
                println!("{name:6} c{center:.1} rms {r:6.1} peak {p:6.1}  {us:5.1} us/block");
            }
        }
        for (name, mode) in [("dual", 0.0), ("ray", 2.0)] {
            let (_, _, us) = run(&[("osc_1.mode", mode), ("unison.mode", 2.0)]);
            println!("{name} U7: {us:5.1} us/block");
        }
        let (r, p, us) = run(&[("osc_1.mode", 2.0), ("ray_1.element", 6.0), ("ray_1.orbit", 0.5), ("ray_1.orbit_rate", 200.0),
                                ("unison.mode", 2.0)]);
        println!("chaos orbiting, U7 rms {r:6.1} peak {p:6.1}  {us:5.1} us/block");
    }

    /// Osc 1's level (RMS dB, over 0.5-4 s of a held note) on a Dual saw and
    /// on Orthogonal with each set (both players on it), at C2, C3 and C4,
    /// then the us per block of a voice on Orthogonal (no unison, U7):
    /// `cargo test --release ortho_levels -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn ortho_levels() {
        let params = SynthParams::from_faust();
        let slots = params.slots();
        let bank = crate::ortho::bank::shared();
        let run = |sets: &[(&str, f32)], note: u8, blocks: usize| {
            let mut v = VoiceUnit::default();
            v.init(48_000.0, &slots);
            for (t, value) in params.faust_values() {
                v.set(t, value);
            }
            for (id, x) in sets {
                v.set(params.faust_index(id).unwrap(), *x);
                if *id == "unison.mode" {
                    v.unison = Unison::from_value(*x);
                }
            }
            v.set(slots.freq, crate::engine::note_freq(note));
            v.set(slots.gain, 0.8);
            v.set(slots.gate, 1.0);
            let mut chunks = [[0.0f32; MAX_BLOCK]; OUTPUTS];
            let mut osc = Vec::new();
            let t = Instant::now();
            for _ in 0..blocks {
                v.compute(&mut chunks.each_mut().map(|c| &mut c[..]));
                osc.extend_from_slice(&chunks[1]);
            }
            let us = t.elapsed().as_secs_f64() * 1e6 / blocks as f64;
            let tail = &osc[24_000.min(osc.len() / 2)..];
            let rms = (tail.iter().map(|x| x * x).sum::<f32>() / tail.len() as f32).sqrt();
            let peak = tail.iter().fold(0.0f32, |m, x| m.max(x.abs()));
            PEAKS.with(|p| p.borrow_mut().push(20.0 * peak.log10()));
            (20.0 * rms.log10(), us)
        };
        thread_local!(static PEAKS: std::cell::RefCell<Vec<f32>> = const { std::cell::RefCell::new(Vec::new()) });
        let (saw, _) = run(&[], 60, 3000);
        println!("dual saw   rms {saw:6.1} peak {:6.1}", PEAKS.with(|p| p.borrow_mut().pop().unwrap()));
        let mut all = Vec::new();
        let mut all_peaks = Vec::new();
        for (k, set) in crate::ortholib::SETS.iter().enumerate() {
            assert!(bank.wait_for(k, std::time::Duration::from_secs(20)));
            let levels: Vec<f32> = [48, 60, 72].iter().map(|&note| {
                run(&[("osc_1.mode", 3.0), ("ortho_1.top", k as f32), ("ortho_1.bottom", k as f32)], note, 3000).0
            }).collect();
            all.extend(levels.iter().copied());
            let peaks: Vec<f32> = PEAKS.with(|p| p.borrow_mut().drain(..).collect());
            println!("{:24} rms {:6.1} {:6.1} {:6.1}  peak {:6.1} {:6.1} {:6.1}", set.name, levels[0], levels[1], levels[2],
                     peaks[0], peaks[1], peaks[2]);
            all_peaks.extend(peaks);
        }
        all.sort_by(f32::total_cmp);
        all_peaks.sort_by(f32::total_cmp);
        println!("median rms {:.1} dB (saw {saw:.1}); peaks median {:.1}, max {:.1} dB", all[all.len() / 2],
                 all_peaks[all_peaks.len() / 2], all_peaks[all_peaks.len() - 1]);
        for (name, uni) in [("no unison", 0.0), ("U7", 2.0)] {
            let (_, us) = run(&[("osc_1.mode", 3.0), ("unison.mode", uni)], 60, 1500);
            println!("ortho {name}: {us:5.1} us/block (of 1333)");
        }
    }

    /// Jeremy's bounce (Nov 15): Osc 1 on Orthogonal, Mix on Bottom (Galaxy
    /// Sleeping), filter open, drive and noise off, C4 at full velocity. Writes
    /// the Orthogonal's own output and the voice's, with Heat at 1 and at 0,
    /// as raw f32 to ABD_DUMP_DIR: `cargo test --release jeremys_bounce -- --ignored`
    #[test]
    #[ignore]
    fn jeremys_bounce() {
        let Ok(dir) = std::env::var("ABD_DUMP_DIR") else { return };
        let params = SynthParams::from_faust();
        let slots = params.slots();
        let bank = crate::ortho::bank::shared();
        assert!(bank.wait_for(39, std::time::Duration::from_secs(20)));
        assert!(bank.wait_for(34, std::time::Duration::from_secs(20)));
        let faders: Vec<f32> = std::env::var("ABD_FADERS").map(|f| f.split(',').map(|x| x.parse().unwrap()).collect())
            .unwrap_or_default();
        let grit: f32 = std::env::var("ABD_GRIT").ok().and_then(|g| g.parse().ok()).unwrap_or(1.0);
        let heat_for_gains: f32 = std::env::var("ABD_HEAT").ok().and_then(|h| h.parse().ok()).unwrap_or(1.0);
        let gains: Vec<f32> = std::env::var("ABD_GAINS").map(|f| f.split(',').map(|x| x.parse().unwrap()).collect())
            .unwrap_or_default();
        let runs = [(1.0, 1.0, 1.0), (0.0, 1.0, 1.0)].into_iter()
            .chain(faders.into_iter().map(|f| (1.0, f, 1.0)))
            .chain(gains.into_iter().map(|g| (heat_for_gains, 1.0, g)));
        for (heat, fader, gain) in runs {
            let mut v = VoiceUnit::default();
            v.init(44_100.0, &slots);
            for (t, value) in params.faust_values() {
                v.set(t, value);
            }
            for (id, x) in [("osc_1.mode", 3.0), ("ortho_1.mix", 1.0), ("filter.cutoff", 18_000.0), ("drive.amount", 0.0),
                            ("mix.noise", 0.0), ("vintage.amount", 0.0), ("settings.heat", heat), ("mix.osc_1", fader),
                            ("settings.drive_grit", grit)] {
                v.set(params.faust_index(id).unwrap(), x);
            }
            v.orthos.gain = crate::ortho::player::ORTHO_GAIN * gain;
            v.set(slots.freq, crate::engine::note_freq(60));
            v.set(slots.gain, 1.0);
            v.set(slots.gate, 1.0);
            let mut chunks = [[0.0f32; MAX_BLOCK]; OUTPUTS];
            let (mut osc, mut voice) = (Vec::new(), Vec::new());
            for _ in 0..(5 * 44_100 / MAX_BLOCK) {
                v.compute(&mut chunks.each_mut().map(|c| &mut c[..]));
                osc.extend_from_slice(&chunks[1]);
                voice.extend_from_slice(&chunks[0]);
            }
            let bytes = |x: &[f32]| x.iter().flat_map(|v| v.to_le_bytes()).collect::<Vec<u8>>();
            std::fs::write(format!("{dir}/osc.f32"), bytes(&osc)).unwrap();
            std::fs::write(format!("{dir}/voice_heat{heat}_fader{fader}_gain{gain}.f32"), bytes(&voice)).unwrap();
        }
    }

    /// Microseconds per 64-sample block of each part of one sounding voice:
    /// `cargo test --release profile_parts -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn profile_parts() {
        let params = SynthParams::from_faust();
        let slots = params.slots();
        let mut v = VoiceUnit::default();
        v.init(48_000.0, &slots);
        for (t, value) in params.faust_values() {
            v.set(t, value);
        }
        let set = |v: &mut VoiceUnit, id: &str, x: f32| v.set(params.faust_index(id).unwrap(), x);
        set(&mut v, "mix.osc_1", 1.0);
        set(&mut v, "mix.osc_2", 1.0);
        if let Ok(a) = std::env::var("ABD_DUAL_ALGO") {
            set(&mut v, "dual_1.algorithm", a.parse().unwrap());
        }
        // ABD_SET="id=value,id=value" sets anything else.
        for pair in std::env::var("ABD_SET").unwrap_or_default().split(',').filter(|p| !p.is_empty()) {
            let (id, x) = pair.split_once('=').unwrap();
            set(&mut v, id, x.parse().unwrap());
        }
        if let Ok(m) = std::env::var("ABD_OSCMOD") {
            set(&mut v, "wt_2.oscmod", m.parse().unwrap());
        }
        v.set(slots.freq, 220.0);
        v.set(slots.gain, 0.8);
        v.set(slots.gate, 1.0);
        let blocks: usize = std::env::var("ABD_BLOCKS").ok().and_then(|b| b.parse().ok()).unwrap_or(4000);
        let n = MAX_BLOCK;
        let (mut t_osc, mut t_voice) = (0.0f64, 0.0);
        for _ in 0..blocks {
            let t = Instant::now();
            let mut out = v.copy_out.each_mut().map(|o| &mut o[..n]);
            v.copies[CENTRE].compute(n as i32, &[], &mut out);
            t_osc += t.elapsed().as_secs_f64();
            let [d, w, ..] = &v.copy_out;
            v.sums[0][0][..n].copy_from_slice(&d[..n]);
            v.sums[0][1][..n].copy_from_slice(&w[..n]);
            let t = Instant::now();
            v.chains[0].compute(&mut v.sums[0], None, [true; 2], v.slot_hz, v.sample_rate, n);
            t_voice += t.elapsed().as_secs_f64();
        }
        // A block of 64 samples lasts 1333 us at 48 kHz.
        let us = |s: f64| s / blocks as f64 * 1e6;
        println!("per block (of 1333 us): osc copy {:.1}  voice {:.1} us", us(t_osc), us(t_voice));
    }

    /// Microseconds per 64-sample block of one voice on each engine (Osc 1
    /// alone), without unison, on U7 narrow (mono) and wide (stereo), and
    /// wide on Eco: `cargo test --release unison_costs -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn unison_costs() {
        let params = SynthParams::from_faust();
        let slots = params.slots();
        let cases: &[(&str, &[(&str, f32)])] = &[
            ("dual saw", &[("osc_1.mode", 0.0)]),
            ("dual fold", &[("osc_1.mode", 0.0), ("dual_1.algorithm", 11.0)]),
            ("wavetable", &[("osc_1.mode", 1.0)]),
            ("wt oscmod", &[("osc_1.mode", 1.0), ("wt_1.oscmod", 5.0)]),
            ("ray", &[("osc_1.mode", 2.0)]),
            ("ray chaos orbit", &[("osc_1.mode", 2.0), ("ray_1.element", 6.0), ("ray_1.orbit", 0.5)]),
            ("ortho", &[("osc_1.mode", 3.0)]),
            ("windows 3", &[("osc_1.mode", 4.0), ("windows_1.int_1_on", 1.0), ("windows_1.int_2_on", 1.0), ("windows_1.int_3_on", 1.0)]),
        ];
        let configs: &[(&str, f32, f32, f32)] = &[("off", 0.0, 0.0, 0.0), ("U7 narrow", 2.0, 0.0, 0.0),
                                                  ("U7 wide", 2.0, 64.0, 0.0), ("U7 wide eco", 2.0, 64.0, 1.0)];
        for (name, sets) in cases {
            let mut line = format!("{name:16}");
            for (cname, uni, spread, eco) in configs {
                let mut v = VoiceUnit::default();
                v.init(48_000.0, &slots);
                for (t, value) in params.faust_values() { v.set(t, value); }
                v.set(params.faust_index("mix.osc_1").unwrap(), 1.0);
                v.set(params.faust_index("mix.osc_2").unwrap(), 0.0);
                v.set(params.faust_index("unison.spread").unwrap(), *spread);
                v.set(params.faust_index("unison.eco").unwrap(), *eco);
                for (id, x) in sets.iter() { v.set(params.faust_index(id).unwrap(), *x); }
                if sets.iter().any(|(_, m)| *m == 3.0) {
                    let o = v.ortho_settings(0);
                    let bank = crate::ortho::bank::shared();
                    for h in 0..2 { bank.want(o.set_of(h)); assert!(bank.wait_for(o.set_of(h), std::time::Duration::from_secs(30))); }
                }
                v.unison = Unison::from_value(*uni);
                v.set(slots.freq, 220.0); v.set(slots.gain, 0.8); v.set(slots.gate, 1.0);
                let mut chunks = [[0.0f32; MAX_BLOCK]; OUTPUTS];
                let blocks = 3000;
                let t = Instant::now();
                for _ in 0..blocks { v.compute(&mut chunks.each_mut().map(|c| &mut c[..])); }
                let us = t.elapsed().as_secs_f64() / blocks as f64 * 1e6;
                line += &format!("  {cname} {us:6.1}");
            }
            println!("{line}");
        }
    }
}
