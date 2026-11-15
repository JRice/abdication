//! One voice's Orthogonal: the oscillator slots set to it, played for each
//! unison copy that runs. Each slot is two granular players (Top and
//! Bottom), each on its own set, mixed by the slot's Mix (a plain crossfade).
//! A note plays each set's recording nearest it (L, M or H:
//! ortholib::recording_for), at the slot's pitch (the Faust copy gives it, so
//! glide, bends and unison detune apply) moved by the player's own Pitch.
//!
//! Where a player's grains start: its Position (0..1 of the set, modulatable)
//! plus how far its Speed has carried it since the note began. At 100 % Speed
//! that's the recording's own pace (0 % leaves Position alone, 200 % twice
//! the pace); while the note is held it stops at ortholib::HOLD_S into the
//! set, as the recording's note did, and goes on into the tail at the release.

use super::bank::{Reader, SampleSet};
use super::grains::{GrainBlock, Granular, GRAIN_S};
use crate::engine::note_freq;
use crate::ortholib::{self, HOLD_S, ROOTS, SETS};

/// The slot modes (voice.lib's Mode menu): Orthogonal is the plugin's.
pub const MODE_ORTHO: f32 = 3.0;

/// The library's level in the voice. The recordings keep their own crest
/// (peaks ~13 dB over their RMS), and the voice saturates (the mixer's heat,
/// the drive's curve even at 0, the filter's loop, the clip after it) at the
/// levels it was built for, a saw's: at a saw's peaks (this at 3.2) a held set
/// came out with 6 dB of its peaks squashed (crest 12.7 dB down to 6.6) and
/// its distortion 10 dB under it. 12 dB lower, its crest is 11 dB and the
/// distortion 20 dB under; it plays some 8 dB quieter than a saw.
/// (voice.rs's `jeremys_bounce` and `ortho_levels` measure it.)
pub const ORTHO_GAIN: f32 = 0.8;

/// What a slot's Orthogonal reads (params::ORTHO_FIELDS, in that order).
#[derive(Clone, Copy, Debug, Default)]
pub struct OrthoSettings {
    pub mode: f32,
    pub fader: f32,
    /// Top's and Bottom's sets, positions (0..1), speeds (%) and pitches
    /// (the Pitch knob's travel, -1..1: pitch.rs).
    pub set: [f32; 2],
    pub position: [f32; 2],
    pub speed: [f32; 2],
    pub pitch: [f32; 2],
    pub mix: f32,
    /// How far either way of the position grains start (ms, the whole spread).
    pub scatter_ms: f32,
}

impl OrthoSettings {
    pub fn from_values(v: [f32; 12]) -> Self {
        let [mode, fader, top, bottom, top_pos, bottom_pos, top_speed, bottom_speed, mix, scatter_ms, top_pitch,
             bottom_pitch] = v;
        Self { mode, fader, set: [top, bottom], position: [top_pos, bottom_pos], speed: [top_speed, bottom_speed], mix,
               scatter_ms, pitch: [top_pitch, bottom_pitch] }
    }

    /// The slot plays Orthogonal, and sounds.
    pub fn playing(&self) -> bool {
        self.mode == MODE_ORTHO && self.fader != 0.0
    }

    /// Player `h`'s set (an index into SETS).
    pub fn set_of(&self, h: usize) -> usize {
        (self.set[h].max(0.0) as usize).min(SETS.len() - 1)
    }

    /// Player `h`'s set's length, seconds.
    fn length_s(&self, h: usize) -> f32 {
        SETS[self.set_of(h)].frames as f32 / ortholib::RATE
    }

    /// Top's and Bottom's levels: a plain crossfade, half each in the middle.
    fn levels(&self) -> [f32; 2] {
        let mix = self.mix.clamp(0.0, 1.0);
        [1.0 - mix, mix]
    }

    /// Player `h`'s pitch at a slot pitch of `hz`.
    fn hz(&self, h: usize, hz: f32) -> f32 {
        hz * (crate::pitch::semitones(self.pitch[h]) / 12.0).exp2()
    }
}

/// How far a player's Speed has carried it this note.
#[derive(Clone, Copy, Default)]
struct Travel {
    seconds: f32,
    /// Its Position at note-on (seconds), which the hold counts from.
    from: f32,
}

/// One voice's Orthogonals: per slot, each player's travel and the recording
/// it plays this note; per unison copy, per slot, the two players' grains.
pub struct OrthoPlayer {
    grains: Vec<[[Granular; 2]; 2]>,
    travel: [[Travel; 2]; 2],
    /// The recording (0 L, 1 M, 2 H) each player plays this note; None until
    /// the note's first block has a pitch.
    recording: [[Option<usize>; 2]; 2],
    rate: f32,
    /// ORTHO_GAIN (tests measure the voice at others).
    pub gain: f32,
}

impl OrthoPlayer {
    pub fn new(copies: usize, seed: u32) -> Self {
        let grains = (0..copies)
            .map(|c| std::array::from_fn(|s| std::array::from_fn(|h| {
                Granular::new(seed.wrapping_add((c * 4 + s * 2 + h) as u32 * 0x2545))
            })))
            .collect();
        Self { grains, travel: Default::default(), recording: [[None; 2]; 2], rate: 48_000.0, gain: ORTHO_GAIN }
    }

    pub fn init(&mut self, rate: f32) {
        self.rate = rate;
        self.clear();
    }

    pub fn clear(&mut self) {
        self.grains.iter_mut().flatten().flatten().for_each(Granular::clear);
        self.travel = Default::default();
        self.recording = [[None; 2]; 2];
    }

    /// Before slot `s`'s copies play a block of `n` samples: a new note starts
    /// over; then each player's Speed moves it on (held, no further than the hold).
    pub fn advance(&mut self, s: usize, settings: &OrthoSettings, note_on: bool, gated: bool, n: usize) {
        if note_on {
            self.recording[s] = [None; 2];
            for copy in &mut self.grains {
                copy[s].iter_mut().for_each(Granular::clear);
            }
        }
        let dt = n as f32 / self.rate;
        for (h, travel) in self.travel[s].iter_mut().enumerate() {
            if note_on {
                *travel = Travel { seconds: 0.0, from: settings.position[h] * settings.length_s(h) };
            }
            travel.seconds += settings.speed[h].max(0.0) / 100.0 * dt;
            if gated {
                travel.seconds = travel.seconds.min((HOLD_S - travel.from).max(0.0));
            }
        }
    }

    /// Player `h` of slot `s`: where its grains start now, as a share of its set.
    pub fn position(&self, s: usize, settings: &OrthoSettings, h: usize) -> f32 {
        settings.position[h] + self.travel[s][h].seconds / settings.length_s(h)
    }

    /// The sets a slot's players read this block (None: not decoded yet).
    pub fn sets<'a>(reader: &'a Reader, settings: &OrthoSettings) -> [Option<&'a SampleSet>; 2] {
        [0, 1].map(|h| reader.get(settings.set_of(h)))
    }

    /// Play slot `s` of unison copy `copy` for one block into `out`
    /// (replacing it), at `hz`, the slot's pitch.
    pub fn render(&mut self, copy: usize, s: usize, settings: &OrthoSettings, sets: [Option<&SampleSet>; 2], hz: f32,
                  out: &mut [f32]) {
        out.fill(0.0);
        let len = (GRAIN_S * self.rate) as u32;
        let scatter = settings.scatter_ms.max(0.0) / 2000.0 * ortholib::RATE;
        let levels = settings.levels();
        for h in 0..2 {
            let Some(set) = sets[h] else { continue };
            if levels[h] == 0.0 {
                continue;
            }
            let hz = settings.hz(h, hz);
            let rec = *self.recording[s][h].get_or_insert_with(|| ortholib::recording_for(hz));
            let step = hz / note_freq(ROOTS[rec]) * ortholib::RATE / self.rate;
            let center = self.position(s, settings, h) * SETS[settings.set_of(h)].frames as f32;
            let block = GrainBlock { src: &set.recs[rec], center, scatter, step, len };
            self.grains[copy][s][h].render(&block, self.gain * levels[h], out);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A player's Pitch moves it by the slots' curve, and its note then plays
    /// the recording nearest the moved pitch; the Mix halves each in the middle.
    #[test]
    fn pitch_and_mix() {
        let mut o = OrthoSettings { pitch: [crate::pitch::position(12.0), crate::pitch::position(-7.0)], mix: 0.5,
                                    ..Default::default() };
        assert!((o.hz(0, 220.0) - 440.0).abs() < 0.01);
        assert!((o.hz(1, 220.0) - 220.0 * (-7.0f32 / 12.0).exp2()).abs() < 0.01);
        assert_eq!(ortholib::recording_for(o.hz(0, note_freq(60))), 2);
        assert_eq!(o.levels(), [0.5, 0.5]);
        o.mix = 0.0;
        assert_eq!(o.levels(), [1.0, 0.0]);
    }
}
