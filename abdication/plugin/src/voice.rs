//! One voice's DSPs: seven unison copies of its two oscillators
//! (`OscDsp`; five run with Unison on U5, only the centre one with it off),
//! its mixer (`MixDsp`), which takes the copies' sum as inputs and gives the
//! voice up to its filter, its FDSP (`FdspDsp`, skipped while it's off), the
//! rest of the voice (`VoiceDsp`). They're separate Faust programs for the compiler's sake (see
//! dsp/voice.lib), and so the parts that aren't sounding cost nothing. The
//! knobs' modulators are the engine's (modulation.rs); it sets each
//! modulated control per voice before a block.

use faust_types::FaustDsp;

use crate::dsp::VoiceDsp;
use crate::fdsp_dsp::FdspDsp;
use crate::mix_dsp::MixDsp;
use crate::osc_dsp::OscDsp;
use std::sync::Arc;

use crate::ortho::bank::{self, Bank};
use crate::ortho::player::{OrthoPlayer, OrthoSettings};
use crate::params::{Dsp, Targets, VoiceSlots, ORTHO_FIELDS, RAY_FIELDS};
use crate::ray::player::{RayPlayer, RaySettings};
use crate::scope::{MOVING, OUTPUTS};

/// The most samples one `compute` call handles.
pub const MAX_BLOCK: usize = 64;
const COPIES: usize = 7;
/// The copy that plays with Unison off (and feeds the scopes).
const CENTRE: usize = 3;
/// A copy's outputs: Osc 1 and Osc 2 at its unison level, then as they are,
/// then (for Ray and Orthogonal) each slot's pitch, its unison level, start
/// phase and note-on.
const COPY_OUTPUTS: usize = 9;
const HZ: usize = 4;
const LEVEL: usize = 6;
const PHASE: usize = 7;
const NOTE_ON: usize = 8;
/// `MixDsp`'s outputs: the voice before its filter, the post-filter
/// transient, the pitch (`VoiceDsp`'s inputs; `FdspDsp` takes the first and last).
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
    mix: Box<MixDsp>,
    fdsp: Box<FdspDsp>,
    voice: Box<VoiceDsp>,
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
    /// The gate (as the engine sets it), and whether it rose since the last block.
    gate: Targets,
    gated: bool,
    note_on: bool,
    /// Each Orthogonal's players' positions (share of their sets) after the
    /// last block; NaN for a slot that isn't one.
    ortho_positions: [[f32; 2]; 2],
    /// Which copies play.
    pub unison: Unison,
    /// Whether FDSP is on (`FdspDsp` runs only then), and whether it ran last block.
    pub fdsp_on: bool,
    fdsp_ran: bool,
    /// Where the modulated controls' ramp length lives in each DSP.
    ramp: Targets,
    /// One copy's latest block (COPY_OUTPUTS).
    copy_out: [Block; COPY_OUTPUTS],
    /// The copies summed: [Osc 1, Osc 2].
    sums: [Block; 2],
    /// The mixer's outputs (MIX_OUTPUTS), and the voice after FDSP.
    mix_out: [Block; MIX_OUTPUTS],
    fdsp_out: [Block; 1],
    /// The voice's own outputs: the voice, its moving values, then its
    /// pitch and amp envelope.
    voice_out: [Block; 1 + MOVING],
}

impl Default for VoiceUnit {
    fn default() -> Self {
        Self {
            mix: Box::new(MixDsp::new()),
            fdsp: Box::new(FdspDsp::new()),
            voice: Box::new(VoiceDsp::new()),
            copies: (0..COPIES).map(|_| OscDsp::new()).collect(),
            rays: RayPlayer::new(COPIES, 0x5EED),
            ray_fields: Default::default(),
            orthos: OrthoPlayer::new(COPIES, 0x0A7D),
            ortho_fields: Default::default(),
            bank: bank::shared(),
            gate: Targets::default(),
            gated: false,
            note_on: false,
            ortho_positions: [[f32::NAN; 2]; 2],
            unison: Unison::Off,
            fdsp_on: false,
            fdsp_ran: false,
            ramp: Targets::default(),
            copy_out: [[0.0; MAX_BLOCK]; COPY_OUTPUTS],
            sums: [[0.0; MAX_BLOCK]; 2],
            mix_out: [[0.0; MAX_BLOCK]; MIX_OUTPUTS],
            fdsp_out: [[0.0; MAX_BLOCK]; 1],
            voice_out: [[0.0; MAX_BLOCK]; 1 + MOVING],
        }
    }
}

impl VoiceUnit {
    pub fn init(&mut self, sample_rate: f32, slots: &VoiceSlots) {
        self.mix.init(sample_rate as i32);
        self.fdsp.init(sample_rate as i32);
        self.voice.init(sample_rate as i32);
        self.ramp = slots.ramp;
        self.ray_fields = slots.ray;
        self.rays.init(sample_rate, MAX_BLOCK);
        self.ortho_fields = slots.ortho;
        self.gate = slots.gate;
        self.orthos.init(sample_rate);
        for (k, copy) in self.copies.iter_mut().enumerate() {
            copy.init(sample_rate as i32);
            for t in slots.copy.iter() {
                copy.set_param(t.index, k as f32);
            }
        }
    }

    pub fn instance_clear(&mut self) {
        self.mix.instance_clear();
        self.fdsp.instance_clear();
        self.voice.instance_clear();
        self.copies.iter_mut().for_each(|c| c.instance_clear());
        self.rays.clear();
        self.orthos.clear();
    }

    /// A control's value in the copies' DSP (as the engine set it for this voice).
    fn osc_value(&self, t: Targets) -> f32 {
        t.iter().find(|p| p.dsp == Dsp::Osc).and_then(|p| self.copies[CENTRE].get_param(p.index)).unwrap_or(0.0)
    }

    /// Slot `s`'s Ray settings, as this voice has them now.
    fn ray_settings(&self, s: usize) -> RaySettings {
        RaySettings::from_values(self.ray_fields[s].map(|t| self.osc_value(t)))
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
        }
        for t in targets.iter() {
            match t.dsp {
                Dsp::Voice => self.voice.set_param(t.index, value),
                Dsp::Mix => self.mix.set_param(t.index, value),
                Dsp::Fdsp => self.fdsp.set_param(t.index, value),
                Dsp::Osc => self.copies.iter_mut().for_each(|c| c.set_param(t.index, value)),
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
        let orthos = [0, 1].map(|s| self.ortho_settings(s));
        let gate_rose = std::mem::take(&mut self.note_on);
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
                self.place(s, &wave[..n]);
            }
            // A slot on Orthogonal: likewise.
            for (s, sets) in sets.iter().enumerate() {
                let Some(sets) = *sets else { continue };
                let hz = self.copy_out[HZ + s][n - 1];
                let mut wave = [0.0; MAX_BLOCK];
                self.orthos.render(k, s, &orthos[s], sets, hz, &mut wave[..n]);
                self.place(s, &wave[..n]);
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

        self.compute_voice(n);
        outputs[0].copy_from_slice(&self.voice_out[0][..n]);
        for (o, v) in outputs[3..].iter_mut().zip(&self.voice_out[1..]) {
            o.copy_from_slice(&v[..n]);
        }
    }

    /// Slot `s`'s wave, played in Rust for the copy just computed, in place of
    /// the copy's (silent) own: as it is, and at the copy's unison level.
    fn place(&mut self, s: usize, wave: &[f32]) {
        let n = wave.len();
        let level = self.copy_out[LEVEL];
        self.copy_out[2 + s][..n].copy_from_slice(wave);
        for ((scaled, w), l) in self.copy_out[s][..n].iter_mut().zip(wave).zip(&level[..n]) {
            *scaled = w * l;
        }
    }

    /// The voice from the copies' sum (`sums`) to `voice_out`: the mixer,
    /// FDSP if it's on, then the rest.
    fn compute_voice(&mut self, n: usize) {
        let [osc1, osc2] = &self.sums;
        let mut out = self.mix_out.each_mut().map(|o| &mut o[..n]);
        self.mix.compute(n as i32, &[&osc1[..n], &osc2[..n]], &mut out);
        if self.fdsp_on && !self.fdsp_ran {
            // Fresh: no tail from when it last ran, and it starts up as on a new note.
            self.fdsp.instance_clear();
        }
        self.fdsp_ran = self.fdsp_on;
        let [pre, tr_post, hz] = &self.mix_out;
        let pre = if self.fdsp_on {
            let mut out = self.fdsp_out.each_mut().map(|o| &mut o[..n]);
            self.fdsp.compute(n as i32, &[&pre[..n], &hz[..n]], &mut out);
            &self.fdsp_out[0]
        } else {
            pre
        };
        let inputs: [&[f32]; 3] = [&pre[..n], &tr_post[..n], &hz[..n]];
        let mut out = self.voice_out.each_mut().map(|o| &mut o[..n]);
        self.voice.compute(n as i32, &inputs, &mut out);
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
        // ABD_FDSP=1 runs FDSP (with its Type set through ABD_SET).
        v.fdsp_on = std::env::var("ABD_FDSP").is_ok();
        let blocks: usize = std::env::var("ABD_BLOCKS").ok().and_then(|b| b.parse().ok()).unwrap_or(4000);
        let n = MAX_BLOCK;
        let (mut t_osc, mut t_voice) = (0.0f64, 0.0);
        for _ in 0..blocks {
            let t = Instant::now();
            let mut out = v.copy_out.each_mut().map(|o| &mut o[..n]);
            v.copies[CENTRE].compute(n as i32, &[], &mut out);
            t_osc += t.elapsed().as_secs_f64();
            let [d, w, ..] = &v.copy_out;
            v.sums[0][..n].copy_from_slice(&d[..n]);
            v.sums[1][..n].copy_from_slice(&w[..n]);
            let t = Instant::now();
            v.compute_voice(n);
            t_voice += t.elapsed().as_secs_f64();
        }
        // A block of 64 samples lasts 1333 us at 48 kHz.
        let us = |s: f64| s / blocks as f64 * 1e6;
        println!("per block (of 1333 us): osc copy {:.1}  voice {:.1} us", us(t_osc), us(t_voice));
    }
}
