//! One voice's DSPs: its free modulators (Mod Env and Mod LFO, `ModDsp`),
//! seven unison copies of its Dual and Wavetable (`OscDsp`; five run with
//! Unison on U5, only the centre one with it off), the rest of the voice (`VoiceDsp`),
//! which takes the modulators and the copies' sum as inputs, and its events
//! and calls (`TextureDsp`, skipped while both are off). They're separate
//! Faust programs for the compiler's sake (see dsp/voice.lib), and so the
//! parts that aren't sounding cost nothing.

use faust_types::FaustDsp;

use crate::dsp::VoiceDsp;
use crate::mod_dsp::ModDsp;
use crate::osc_dsp::OscDsp;
use crate::texture_dsp::TextureDsp;
use crate::params::{Dsp, Targets, VoiceSlots};
use crate::scope::{MOVING, OUTPUTS};

/// The most samples one `compute` call handles.
pub const MAX_BLOCK: usize = 64;
const COPIES: usize = 7;
/// The copy that plays with Unison off (and feeds the scopes).
const CENTRE: usize = 3;
/// `VoiceDsp` outputs after the moving values, for `TextureDsp`: pitch, amp envelope.
const TEXTURE_TAPS: usize = 2;
/// Blocks `TextureDsp` keeps running after the events go off, while Mute's
/// crossfade (about 20 ms per e-fold) finishes: 270 ms.
const EVENTS_FADE_BLOCKS: u32 = 200;

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

    /// The copies that run (voice.lib's U5 leaves out copies 1 and 5).
    fn copies(self) -> &'static [usize] {
        match self {
            Unison::Off => &[CENTRE],
            Unison::U5 => &[0, 2, CENTRE, 4, 6],
            Unison::U7 => &[0, 1, 2, CENTRE, 4, 5, 6],
        }
    }
}

pub struct VoiceUnit {
    voice: Box<VoiceDsp>,
    mods: Box<ModDsp>,
    copies: Vec<OscDsp>,
    texture: Box<TextureDsp>,
    /// Which copies play.
    pub unison: Unison,
    /// Whether the events and the calls are on (`TextureDsp` runs if either is).
    pub events_on: bool,
    pub calls_on: bool,
    /// Blocks left of the events' fade after they went off.
    events_fade: u32,
    /// The modulators' latest block: [Mod Env, Mod LFO].
    mod_out: [Block; 2],
    /// One copy's latest block: Dual and Wavetable at its unison level, then as they are.
    copy_out: [Block; 4],
    /// The copies summed: [Dual, Wavetable].
    sums: [Block; 2],
    /// The voice's own outputs: the voice, its moving values, then its
    /// pitch and amp envelope.
    voice_out: [Block; 1 + MOVING + TEXTURE_TAPS],
    /// The texture's latest block: the voice with its events, then its calls.
    texture_out: [Block; 3],
}

impl Default for VoiceUnit {
    fn default() -> Self {
        Self {
            voice: Box::new(VoiceDsp::new()),
            mods: Box::new(ModDsp::new()),
            copies: (0..COPIES).map(|_| OscDsp::new()).collect(),
            texture: Box::new(TextureDsp::new()),
            unison: Unison::Off,
            events_on: false,
            calls_on: false,
            events_fade: 0,
            mod_out: [[0.0; MAX_BLOCK]; 2],
            copy_out: [[0.0; MAX_BLOCK]; 4],
            sums: [[0.0; MAX_BLOCK]; 2],
            voice_out: [[0.0; MAX_BLOCK]; 1 + MOVING + TEXTURE_TAPS],
            texture_out: [[0.0; MAX_BLOCK]; 3],
        }
    }
}

impl VoiceUnit {
    pub fn init(&mut self, sample_rate: f32, slots: &VoiceSlots) {
        self.voice.init(sample_rate as i32);
        self.mods.init(sample_rate as i32);
        self.texture.init(sample_rate as i32);
        for (k, copy) in self.copies.iter_mut().enumerate() {
            copy.init(sample_rate as i32);
            for t in slots.copy.iter() {
                copy.set_param(t.index, k as f32);
            }
        }
    }

    pub fn instance_clear(&mut self) {
        self.voice.instance_clear();
        self.mods.instance_clear();
        self.texture.instance_clear();
        self.copies.iter_mut().for_each(|c| c.instance_clear());
    }

    /// Set a control in every DSP that has it.
    pub fn set(&mut self, targets: Targets, value: f32) {
        for t in targets.iter() {
            match t.dsp {
                Dsp::Voice => self.voice.set_param(t.index, value),
                Dsp::Mod => self.mods.set_param(t.index, value),
                Dsp::Osc => self.copies.iter_mut().for_each(|c| c.set_param(t.index, value)),
                Dsp::Texture => self.texture.set_param(t.index, value),
                Dsp::CallBus => {} // the engine's
            }
        }
    }

    /// Compute `outputs[0].len()` samples (at most MAX_BLOCK) of every
    /// output (see scope::OUTPUTS: the voice, the centre copy's Dual and
    /// Wavetable, then the moving values).
    pub fn compute(&mut self, outputs: &mut [&mut [f32]; OUTPUTS]) {
        let n = outputs[0].len().min(MAX_BLOCK);
        let [env, lfo] = &mut self.mod_out;
        self.mods.compute(n as i32, &[], &mut [&mut env[..n], &mut lfo[..n]]);

        for sum in &mut self.sums {
            sum[..n].fill(0.0);
        }
        for &k in self.unison.copies() {
            let mut out = self.copy_out.each_mut().map(|o| &mut o[..n]);
            self.copies[k].compute(n as i32, &[&env[..n], &lfo[..n]], &mut out);
            let [dual, wavetable, dual_raw, wavetable_raw] = &self.copy_out;
            for (sum, copy) in self.sums.iter_mut().zip([dual, wavetable]) {
                sum[..n].iter_mut().zip(&copy[..n]).for_each(|(s, c)| *s += c);
            }
            if k == CENTRE {
                outputs[1].copy_from_slice(&dual_raw[..n]);
                outputs[2].copy_from_slice(&wavetable_raw[..n]);
            }
        }

        let [dual, wavetable] = &self.sums;
        let inputs: [&[f32]; 4] = [&env[..n], &lfo[..n], &dual[..n], &wavetable[..n]];
        let mut out = self.voice_out.each_mut().map(|o| &mut o[..n]);
        self.voice.compute(n as i32, &inputs, &mut out);
        self.events_fade = if self.events_on { EVENTS_FADE_BLOCKS } else { self.events_fade.saturating_sub(1) };
        if self.events_fade > 0 || self.calls_on {
            let [voice, .., hz, amp] = &self.voice_out;
            let inputs: [&[f32]; 5] = [&env[..n], &lfo[..n], &voice[..n], &hz[..n], &amp[..n]];
            let mut out = self.texture_out.each_mut().map(|o| &mut o[..n]);
            self.texture.compute(n as i32, &inputs, &mut out);
            outputs[0].copy_from_slice(&self.texture_out[0][..n]);
        } else {
            outputs[0].copy_from_slice(&self.voice_out[0][..n]);
        }
        for (o, v) in outputs[3..].iter_mut().zip(&self.voice_out[1..]) {
            o.copy_from_slice(&v[..n]);
        }
    }

    /// Add this voice's latest `n` samples of calls into the calls bus's input.
    pub fn add_calls(&self, n: usize, bus: &mut [[f32; MAX_BLOCK]; 2]) {
        for (sum, calls) in bus.iter_mut().zip(&self.texture_out[1..]) {
            sum[..n].iter_mut().zip(&calls[..n]).for_each(|(s, c)| *s += c);
        }
    }
}

#[cfg(test)]
mod profile {
    use super::*;
    use crate::params::SynthParams;
    use std::time::Instant;

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
        set(&mut v, "mix.dual", 1.0);
        set(&mut v, "mix.wavetable", 1.0);
        if let Ok(a) = std::env::var("ABD_DUAL_ALGO") {
            set(&mut v, "dual.algorithm", a.parse().unwrap());
        }
        // ABD_SET="id=value,id=value" sets anything else.
        for pair in std::env::var("ABD_SET").unwrap_or_default().split(',').filter(|p| !p.is_empty()) {
            let (id, x) = pair.split_once('=').unwrap();
            set(&mut v, id, x.parse().unwrap());
        }
        if let Ok(m) = std::env::var("ABD_OSCMOD") {
            set(&mut v, "wavetable.oscmod", m.parse().unwrap());
        }
        v.set(slots.freq, 220.0);
        v.set(slots.gain, 0.8);
        v.set(slots.gate, 1.0);
        let blocks: usize = std::env::var("ABD_BLOCKS").ok().and_then(|b| b.parse().ok()).unwrap_or(4000);
        let n = MAX_BLOCK;
        let (mut t_mod, mut t_osc, mut t_voice, mut t_tex) = (0.0f64, 0.0, 0.0, 0.0);
        for _ in 0..blocks {
            let [env, lfo] = &mut v.mod_out;
            let t = Instant::now();
            v.mods.compute(n as i32, &[], &mut [&mut env[..n], &mut lfo[..n]]);
            t_mod += t.elapsed().as_secs_f64();
            let t = Instant::now();
            let mut out = v.copy_out.each_mut().map(|o| &mut o[..n]);
            v.copies[CENTRE].compute(n as i32, &[&env[..n], &lfo[..n]], &mut out);
            t_osc += t.elapsed().as_secs_f64();
            let [d, w, ..] = &v.copy_out;
            let inputs: [&[f32]; 4] = [&env[..n], &lfo[..n], &d[..n], &w[..n]];
            let t = Instant::now();
            let mut out = v.voice_out.each_mut().map(|o| &mut o[..n]);
            v.voice.compute(n as i32, &inputs, &mut out);
            t_voice += t.elapsed().as_secs_f64();
            let [voice, .., hz, amp] = &v.voice_out;
            let inputs: [&[f32]; 5] = [&env[..n], &lfo[..n], &voice[..n], &hz[..n], &amp[..n]];
            let t = Instant::now();
            let mut out = v.texture_out.each_mut().map(|o| &mut o[..n]);
            v.texture.compute(n as i32, &inputs, &mut out);
            t_tex += t.elapsed().as_secs_f64();
        }
        // A block of 64 samples lasts 1333 us at 48 kHz.
        let us = |s: f64| s / blocks as f64 * 1e6;
        println!("per block (of 1333 us): mod {:.1}  osc copy {:.1}  voice {:.1}  texture {:.1} us",
                 us(t_mod), us(t_osc), us(t_voice), us(t_tex));
    }
}
