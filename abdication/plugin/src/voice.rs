//! One voice's DSPs: seven unison copies of its two oscillators
//! (`OscDsp`; five run with Unison on U5, only the centre one with it off),
//! the rest of the voice (`VoiceDsp`), which takes the copies' sum as
//! inputs, and its events and calls (`TextureDsp`, skipped while both are
//! off). They're separate Faust programs for the compiler's sake (see
//! dsp/voice.lib), and so the parts that aren't sounding cost nothing. The
//! knobs' modulators are the engine's (modulation.rs); it sets each
//! modulated control per voice before a block.

use faust_types::FaustDsp;

use crate::dsp::VoiceDsp;
use crate::osc_dsp::OscDsp;
use crate::texture_dsp::TextureDsp;
use crate::params::{Dsp, Targets, VoiceSlots, RAY_FIELDS};
use crate::ray::player::{RayPlayer, RaySettings};
use crate::scope::{MOVING, OUTPUTS};

/// The most samples one `compute` call handles.
pub const MAX_BLOCK: usize = 64;
const COPIES: usize = 7;
/// The copy that plays with Unison off (and feeds the scopes).
const CENTRE: usize = 3;
/// A copy's outputs: Osc 1 and Osc 2 at its unison level, then as they are,
/// then (for Ray) each slot's pitch, its unison level, start phase and note-on.
const COPY_OUTPUTS: usize = 9;
const HZ: usize = 4;
const LEVEL: usize = 6;
const PHASE: usize = 7;
const NOTE_ON: usize = 8;
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
    copies: Vec<OscDsp>,
    /// The slots set to Ray, played in Rust for each copy.
    rays: RayPlayer,
    /// Where the Ray's controls live (params::RAY_FIELDS), per slot.
    ray_fields: [[Targets; RAY_FIELDS.len()]; 2],
    texture: Box<TextureDsp>,
    /// Which copies play.
    pub unison: Unison,
    /// Whether the events and the calls are on (`TextureDsp` runs if either is).
    pub events_on: bool,
    pub calls_on: bool,
    /// Blocks left of the events' fade after they went off.
    events_fade: u32,
    /// Where the modulated controls' ramp length lives in each DSP.
    ramp: Targets,
    /// One copy's latest block (COPY_OUTPUTS).
    copy_out: [Block; COPY_OUTPUTS],
    /// The copies summed: [Osc 1, Osc 2].
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
            copies: (0..COPIES).map(|_| OscDsp::new()).collect(),
            rays: RayPlayer::new(COPIES, 0x5EED),
            ray_fields: Default::default(),
            texture: Box::new(TextureDsp::new()),
            unison: Unison::Off,
            events_on: false,
            calls_on: false,
            events_fade: 0,
            ramp: Targets::default(),
            copy_out: [[0.0; MAX_BLOCK]; COPY_OUTPUTS],
            sums: [[0.0; MAX_BLOCK]; 2],
            voice_out: [[0.0; MAX_BLOCK]; 1 + MOVING + TEXTURE_TAPS],
            texture_out: [[0.0; MAX_BLOCK]; 3],
        }
    }
}

impl VoiceUnit {
    pub fn init(&mut self, sample_rate: f32, slots: &VoiceSlots) {
        self.voice.init(sample_rate as i32);
        self.ramp = slots.ramp;
        self.ray_fields = slots.ray;
        self.rays.init(sample_rate, MAX_BLOCK);
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
        self.texture.instance_clear();
        self.copies.iter_mut().for_each(|c| c.instance_clear());
        self.rays.clear();
    }

    /// A control's value in the copies' DSP (as the engine set it for this voice).
    fn osc_value(&self, t: Targets) -> f32 {
        t.iter().find(|p| p.dsp == Dsp::Osc).and_then(|p| self.copies[CENTRE].get_param(p.index)).unwrap_or(0.0)
    }

    /// Slot `s`'s Ray settings, as this voice has them now.
    fn ray_settings(&self, s: usize) -> RaySettings {
        RaySettings::from_values(self.ray_fields[s].map(|t| self.osc_value(t)))
    }

    /// Set a control in every DSP that has it.
    pub fn set(&mut self, targets: Targets, value: f32) {
        for t in targets.iter() {
            match t.dsp {
                Dsp::Voice => self.voice.set_param(t.index, value),
                Dsp::Osc => self.copies.iter_mut().for_each(|c| c.set_param(t.index, value)),
                Dsp::Texture => self.texture.set_param(t.index, value),
                Dsp::CallBus => {} // the engine's
            }
        }
    }

    /// Compute `outputs[0].len()` samples (at most MAX_BLOCK) of every
    /// output (see scope::OUTPUTS: the voice, the centre copy's Osc 1 and
    /// Osc 2, then the moving values).
    pub fn compute(&mut self, outputs: &mut [&mut [f32]; OUTPUTS]) {
        let n = outputs[0].len().min(MAX_BLOCK);
        // Modulated controls reach this block's values by its end.
        self.set(self.ramp, n as f32);

        for sum in &mut self.sums {
            sum[..n].fill(0.0);
        }
        let rays = [0, 1].map(|s| self.ray_settings(s));
        let keys = [0, 1].map(|s| rays[s].playing().then(|| self.rays.prepare(s, &rays[s])));
        for &k in self.unison.copies() {
            let mut out = self.copy_out.each_mut().map(|o| &mut o[..n]);
            self.copies[k].compute(n as i32, &[], &mut out);
            // A slot on Ray: the copy gave silence and its pitch; play it here.
            let note_on = self.copy_out[NOTE_ON][..n].iter().any(|t| *t > 0.5);
            for (s, key) in keys.iter().enumerate() {
                let Some(key) = *key else { continue };
                let (hz, phase) = (self.copy_out[HZ + s][n - 1], self.copy_out[PHASE][n - 1]);
                let mut wave = [0.0; MAX_BLOCK];
                self.rays.render(k, s, key, &rays[s], hz, phase, note_on, &mut wave[..n]);
                let level = self.copy_out[LEVEL];
                self.copy_out[2 + s][..n].copy_from_slice(&wave[..n]);
                for ((scaled, w), l) in self.copy_out[s][..n].iter_mut().zip(&wave[..n]).zip(&level[..n]) {
                    *scaled = w * l;
                }
            }
            let [osc1, osc2, osc1_raw, osc2_raw, ..] = &self.copy_out;
            for (sum, copy) in self.sums.iter_mut().zip([osc1, osc2]) {
                sum[..n].iter_mut().zip(&copy[..n]).for_each(|(s, c)| *s += c);
            }
            if k == CENTRE {
                outputs[1].copy_from_slice(&osc1_raw[..n]);
                outputs[2].copy_from_slice(&osc2_raw[..n]);
            }
        }

        let [osc1, osc2] = &self.sums;
        let inputs: [&[f32]; 2] = [&osc1[..n], &osc2[..n]];
        let mut out = self.voice_out.each_mut().map(|o| &mut o[..n]);
        self.voice.compute(n as i32, &inputs, &mut out);
        self.events_fade = if self.events_on { EVENTS_FADE_BLOCKS } else { self.events_fade.saturating_sub(1) };
        if self.events_fade > 0 || self.calls_on {
            let [voice, .., hz, amp] = &self.voice_out;
            let inputs: [&[f32]; 3] = [&voice[..n], &hz[..n], &amp[..n]];
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
            all.extend_from_slice(&v.copy_out[2][..n]);
        }
        std::fs::write(path, all.iter().flat_map(|x| x.to_le_bytes()).collect::<Vec<u8>>()).unwrap();
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
        let (mut t_osc, mut t_voice, mut t_tex) = (0.0f64, 0.0, 0.0);
        for _ in 0..blocks {
            let t = Instant::now();
            let mut out = v.copy_out.each_mut().map(|o| &mut o[..n]);
            v.copies[CENTRE].compute(n as i32, &[], &mut out);
            t_osc += t.elapsed().as_secs_f64();
            let [d, w, ..] = &v.copy_out;
            let inputs: [&[f32]; 2] = [&d[..n], &w[..n]];
            let t = Instant::now();
            let mut out = v.voice_out.each_mut().map(|o| &mut o[..n]);
            v.voice.compute(n as i32, &inputs, &mut out);
            t_voice += t.elapsed().as_secs_f64();
            let [voice, .., hz, amp] = &v.voice_out;
            let inputs: [&[f32]; 3] = [&voice[..n], &hz[..n], &amp[..n]];
            let t = Instant::now();
            let mut out = v.texture_out.each_mut().map(|o| &mut o[..n]);
            v.texture.compute(n as i32, &inputs, &mut out);
            t_tex += t.elapsed().as_secs_f64();
        }
        // A block of 64 samples lasts 1333 us at 48 kHz.
        let us = |s: f64| s / blocks as f64 * 1e6;
        println!("per block (of 1333 us): osc copy {:.1}  voice {:.1}  texture {:.1} us",
                 us(t_osc), us(t_voice), us(t_tex));
    }
}
