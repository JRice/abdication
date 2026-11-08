//! Oscilloscope traces of the oscillators' own outputs (before the mix and
//! the filter), for the panel. The voice has extra outputs for the panel:
//! one per scoped oscillator ("taps"), then the values that move while it
//! plays (pulse width, the filter, the free modulators). A trace comes
//! either from rendering one voice offline with the current settings, or
//! live from the newest voice, which the audio thread copies into a
//! `LiveScope` along with those moving values.

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Mutex;

use crate::engine::note_freq;
use crate::voice::{Unison, VoiceUnit, MAX_BLOCK};
use crate::params::SynthParams;

/// Points in a rendered trace.
pub const POINTS: usize = 320;

/// Which oscillator a trace shows (the voice's output channel).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tap {
    Dual = 1,
    Wavetable = 2,
}

/// A voice's outputs (see voice::VoiceUnit): the voice itself, the two
/// taps (the centre unison copy's Dual and Wavetable), then `Moving`.
pub const OUTPUTS: usize = 3 + MOVING;

/// The voice's moving values, in output order after the taps.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Moving {
    PulseWidth,
    Cutoff,
    Resonance,
    Morph,
    Bandpass,
    Hpf,
    ModEnv,
    ModLfo,
}
pub const MOVING: usize = 8;

/// The pitch the offline traces render at: the last note played (C3
/// until then). The audio thread writes it; the editor reads it.
pub struct ScopePitch(AtomicU32);

impl Default for ScopePitch {
    fn default() -> Self {
        Self(AtomicU32::new(note_freq(48).to_bits()))
    }
}

impl ScopePitch {
    pub fn set(&self, hz: f32) {
        self.0.store(hz.to_bits(), Ordering::Relaxed);
    }
    pub fn get(&self) -> f32 {
        f32::from_bits(self.0.load(Ordering::Relaxed))
    }
}

/// `cycles` periods of `period` samples starting at the first rising zero
/// crossing at or after `from` (or at `from` if there's none within a
/// period), resampled to `POINTS` values.
pub fn align(samples: &[f32], period: f32, cycles: f32, from: usize) -> Vec<f32> {
    let last = samples.len().saturating_sub(2);
    let search_end = (from + period.ceil() as usize).min(last);
    let start = (from..search_end)
        .find(|&i| samples[i] <= 0.0 && samples[i + 1] > 0.0)
        .map(|i| i as f32 + samples[i] / (samples[i] - samples[i + 1]))
        .unwrap_or(from as f32);
    let span = period * cycles;
    (0..POINTS)
        .map(|k| {
            let x = start + span * k as f32 / (POINTS - 1) as f32;
            let i = (x as usize).min(last);
            samples[i] + (samples[i + 1] - samples[i]) * (x - i as f32)
        })
        .collect()
}

const RENDER_RATE: f32 = 48_000.0;
/// How long the offline note plays before the snapshot.
const SETTLE_SECONDS: f32 = 0.08;

/// Render one voice offline at `note_hz` and return `cycles` cycles of
/// `tap`, whose own pitch is `tap_hz` (the note moved by that
/// oscillator's octave / semitone settings).
pub fn render(params: &SynthParams, note_hz: f32, tap_hz: f32, tap: Tap, cycles: f32) -> Vec<f32> {
    let tap_hz = tap_hz.clamp(10.0, 8000.0);
    let mut dsp = VoiceUnit::default();
    dsp.init(RENDER_RATE, &params.slots());
    for (param, value) in params.faust_values() {
        dsp.set(param, value);
    }
    dsp.unison = Unison::from_value(params.ptr("unison.mode").map_or(0.0, |p| unsafe { p.unmodulated_plain_value() }));
    let slots = params.slots();
    // The scoped oscillators sound even when the mix has them at 0 (where
    // the voice would skip computing them).
    for id in ["mix.dual", "mix.wavetable"] {
        if let Some(t) = params.faust_index(id) {
            dsp.set(t, 1.0);
        }
    }
    dsp.set(slots.freq, note_hz.clamp(20.0, 8000.0));
    dsp.set(slots.gain, 1.0);
    dsp.set(slots.seed, 0.5);
    dsp.set(slots.gate, 1.0);

    let period = RENDER_RATE / tap_hz;
    let settle = (SETTLE_SECONDS * RENDER_RATE) as usize;
    let total = settle + (period * (cycles + 1.5)) as usize + 2;
    let mut out = Vec::with_capacity(total);
    let mut chunks = [[0.0f32; MAX_BLOCK]; OUTPUTS];
    while out.len() < total {
        let n = (total - out.len()).min(MAX_BLOCK);
        dsp.compute(&mut chunks.each_mut().map(|c| &mut c[..n]));
        out.extend_from_slice(&chunks[tap as usize][..n]);
    }
    align(&out, period, cycles, settle)
}

/// Samples kept for live traces: one cycle down to ~12 Hz at 48 kHz.
const LIVE_LEN: usize = 4096;

/// One block of the newest voice, as the audio thread hands it over.
pub struct LiveBlock<'a> {
    pub dual: &'a [f32],
    pub wavetable: &'a [f32],
    /// The moving values at the end of the block.
    pub moving: [f32; MOVING],
    pub note_hz: f32,
    pub sample_rate: f32,
}

/// The newest voice's two tapped oscillators (the latest samples of each)
/// and its moving values, written by the audio thread (which never waits:
/// if the editor is reading, that block is skipped) and read by the editor.
pub struct LiveScope(Mutex<LiveBuffer>);

struct LiveBuffer {
    rings: [Box<[f32; LIVE_LEN]>; 2],
    write: usize,
    moving: [f32; MOVING],
    note_hz: f32,
    sample_rate: f32,
    /// Samples written so far; tells the editor whether anything is new.
    written: u64,
}

/// A copy of the live buffers, oldest sample first.
pub struct LiveSnapshot {
    pub dual: Vec<f32>,
    pub wavetable: Vec<f32>,
    pub moving: [f32; MOVING],
    pub note_hz: f32,
    pub sample_rate: f32,
    pub written: u64,
}

impl Default for LiveScope {
    fn default() -> Self {
        Self(Mutex::new(LiveBuffer {
            rings: [Box::new([0.0; LIVE_LEN]), Box::new([0.0; LIVE_LEN])],
            write: 0,
            moving: [0.0; MOVING],
            note_hz: note_freq(48),
            sample_rate: 48_000.0,
            written: 0,
        }))
    }
}

impl LiveScope {
    /// Audio thread: append a block from the newest voice.
    pub fn push(&self, block: LiveBlock) {
        let Ok(mut b) = self.0.try_lock() else { return };
        let start = b.write;
        for (ring, samples) in [block.dual, block.wavetable].into_iter().enumerate() {
            let mut w = start;
            for &s in samples {
                b.rings[ring][w] = s;
                w = (w + 1) % LIVE_LEN;
            }
        }
        b.write = (start + block.dual.len()) % LIVE_LEN;
        b.moving = block.moving;
        b.note_hz = block.note_hz;
        b.sample_rate = block.sample_rate;
        b.written += block.dual.len() as u64;
    }

    /// Editor: the buffers, if anything was written since `seen` samples.
    pub fn snapshot_since(&self, seen: u64) -> Option<LiveSnapshot> {
        let b = self.0.lock().ok()?;
        if b.written == seen {
            return None;
        }
        let unroll = |ring: &[f32; LIVE_LEN]| [&ring[b.write..], &ring[..b.write]].concat();
        Some(LiveSnapshot {
            dual: unroll(&b.rings[0]),
            wavetable: unroll(&b.rings[1]),
            moving: b.moving,
            note_hz: b.note_hz,
            sample_rate: b.sample_rate,
            written: b.written,
        })
    }
}

impl LiveSnapshot {
    /// The most recent `cycles` cycles of `tap` at `tap_hz`, aligned like `render`.
    pub fn trace(&self, tap: Tap, tap_hz: f32, cycles: f32) -> Vec<f32> {
        let samples = match tap {
            Tap::Dual => &self.dual,
            Tap::Wavetable => &self.wavetable,
        };
        let period = (self.sample_rate / tap_hz.max(1.0)).min(LIVE_LEN as f32 / (cycles + 1.5));
        let from = samples.len().saturating_sub((period * (cycles + 1.0)) as usize + 2);
        align(samples, period, cycles, from)
    }

    pub fn get(&self, value: Moving) -> f32 {
        self.moving[value as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scope_shows_two_cycles_of_the_dual() {
        let params = SynthParams::from_faust();
        let trace = render(&params, 110.0, 110.0, Tap::Dual, 2.0);
        assert_eq!(trace.len(), POINTS);
        assert!(trace.iter().all(|v| v.is_finite()));
        let peak = trace.iter().fold(0.0f32, |m, v| m.max(v.abs()));
        assert!(peak > 0.1, "scope is silent (peak {peak})");
        // The default patch (Ring Mod, tri/tri, a 2:1 ratio) is periodic at
        // the note's pitch: the two cycles match.
        let half = POINTS / 2;
        let diff: f32 = (0..half - 1).map(|i| (trace[i] - trace[i + half]).abs()).sum::<f32>() / half as f32;
        assert!(diff < 0.1 * peak, "cycles differ by {diff}");
    }

    #[test]
    fn wavetable_tap_is_live_and_offline() {
        let params = SynthParams::from_faust();
        let offline = render(&params, 110.0, 110.0, Tap::Wavetable, 1.0);
        assert!(offline.iter().fold(0.0f32, |m, v| m.max(v.abs())) > 0.1, "wavetable tap is silent");

        // A live buffer holding a sine at 100 Hz yields one cycle starting
        // at its rising zero crossing.
        let live = LiveScope::default();
        let sine: Vec<f32> = (0..3000).map(|i| (i as f32 * 2.0 * std::f32::consts::PI / 480.0 + 1.0).sin()).collect();
        let silence = vec![0.0; sine.len()];
        let mut moving = [0.0; MOVING];
        moving[Moving::Cutoff as usize] = 1234.0;
        live.push(LiveBlock { dual: &silence, wavetable: &sine, moving, note_hz: 100.0, sample_rate: 48_000.0 });
        let snap = live.snapshot_since(0).expect("new samples");
        assert!(live.snapshot_since(snap.written).is_none(), "nothing new after a snapshot");
        assert_eq!(snap.get(Moving::Cutoff), 1234.0);
        assert!(snap.trace(Tap::Dual, 100.0, 1.0).iter().all(|v| *v == 0.0));
        let trace = snap.trace(Tap::Wavetable, 100.0, 1.0);
        assert!(trace[0].abs() < 0.02 && trace[POINTS / 4] > 0.98 && trace[3 * POINTS / 4] < -0.98);
    }
}
