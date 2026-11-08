//! One hit, rendered to a WAV file for dragging into the DAW: the pad as it
//! plays (effects, Volume, the limiter), note 60 at full velocity, until it
//! has rung out. Each take re-rolls Variation (a fresh seed) and the noise
//! carries on from the last take, so repeated drags give slightly different
//! hits, like playing the pad again.

use std::io;
use std::path::PathBuf;

use crate::engine::{Engine, Limiter, MAX_BLOCK};
use crate::params::DrumParams;

/// The note and velocity a take plays.
const NOTE: u8 = 60;
const VELOCITY: f32 = 1.0;
/// A take ends once the pad has been this quiet for HOLD_S (or at MAX_S).
const QUIET_DB: f32 = -90.0;
const HOLD_S: f32 = 0.1;
const MAX_S: f32 = 10.0;
/// Fade-out at the very end, over the last of the tail.
const FADE_S: f32 = 0.005;

/// Renders takes; keeps its engine so the noise carries on between takes.
#[derive(Default)]
pub struct Exporter {
    engine: Option<(Engine, f32)>,
}

impl Exporter {
    /// One hit with the current settings, at `sample_rate`.
    pub fn render(&mut self, params: &DrumParams, sample_rate: f32, seed: u32) -> Vec<f32> {
        if self.engine.as_ref().is_none_or(|(_, sr)| *sr != sample_rate) {
            let mut e = Engine::new(params.slots());
            e.init(sample_rate);
            self.engine = Some((e, sample_rate));
        }
        let (e, _) = self.engine.as_mut().expect("just made");
        e.apply(params);
        e.reseed(seed);
        e.note_on(NOTE, VELOCITY);
        let mut limiter = Limiter::new();
        limiter.init(sample_rate);
        let volume = params.volume.value();

        let quiet = 10f32.powf(QUIET_DB / 20.0);
        let (hold, max) = ((HOLD_S * sample_rate) as usize, (MAX_S * sample_rate) as usize);
        let mut out = Vec::with_capacity(sample_rate as usize * 2);
        let (mut block, mut quiet_run) = ([0.0f32; MAX_BLOCK], 0usize);
        while out.len() < max {
            e.render(&mut block);
            for s in block {
                let y = limiter.process(s * volume);
                quiet_run = if y.abs() < quiet { quiet_run + 1 } else { 0 };
                out.push(y);
            }
            if quiet_run >= hold && e.active_voices() == 0 {
                break;
            }
        }
        // Trim the silence, then fade the last few milliseconds.
        let end = out.iter().rposition(|s| s.abs() >= quiet).map_or(1, |i| i + 1);
        out.truncate(end);
        let fade = ((FADE_S * sample_rate) as usize).min(out.len());
        let n = out.len();
        for (k, s) in out[n - fade..].iter_mut().enumerate() {
            *s *= 1.0 - (k + 1) as f32 / fade as f32;
        }
        out
    }

    /// Render a take and write it to the export folder; returns its path.
    pub fn export(&mut self, params: &DrumParams, sample_rate: f32, seed: u32) -> io::Result<PathBuf> {
        let samples = self.render(params, sample_rate, seed);
        let dir = export_dir();
        std::fs::create_dir_all(&dir)?;
        let stamp = chrono::Local::now().format("%Y-%m-%d %H%M%S");
        let mut path = dir.join(format!("ABDrum {stamp}.wav"));
        for k in 2.. {
            if !path.exists() {
                break;
            }
            path = dir.join(format!("ABDrum {stamp} {k}.wav"));
        }
        std::fs::write(&path, wav(&samples, sample_rate as u32))?;
        Ok(path)
    }
}

/// Where takes go: Music\ABDrum in the user's folder (somewhere that lasts,
/// since a DAW may keep referring to the file).
pub fn export_dir() -> PathBuf {
    let home = std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME")).unwrap_or_default();
    PathBuf::from(home).join("Music").join("ABDrum")
}

/// A mono 24-bit PCM WAV file.
pub fn wav(samples: &[f32], sample_rate: u32) -> Vec<u8> {
    let data_len = samples.len() as u32 * 3;
    let mut b = Vec::with_capacity(44 + data_len as usize);
    let mut put = |bytes: &[u8]| b.extend_from_slice(bytes);
    put(b"RIFF");
    put(&(36 + data_len).to_le_bytes());
    put(b"WAVEfmt ");
    put(&16u32.to_le_bytes());
    put(&1u16.to_le_bytes()); // PCM
    put(&1u16.to_le_bytes()); // mono
    put(&sample_rate.to_le_bytes());
    put(&(sample_rate * 3).to_le_bytes());
    put(&3u16.to_le_bytes());
    put(&24u16.to_le_bytes());
    put(b"data");
    put(&data_len.to_le_bytes());
    for s in samples {
        let v = (s.clamp(-1.0, 1.0) * 8_388_607.0).round() as i32;
        put(&v.to_le_bytes()[..3]);
    }
    b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_take_rings_out_ends_cleanly_and_varies() {
        let params = DrumParams::from_faust();
        let mut ex = Exporter::default();
        let a = ex.render(&params, 48_000.0, 1);
        let b = ex.render(&params, 48_000.0, 2);
        let peak = a.iter().fold(0.0f32, |m, s| m.max(s.abs()));
        assert!(peak > 0.3 && peak <= 10f32.powf(-0.5 / 20.0) + 1e-4, "peak {peak}");
        // The default kick rings for most of a second, not the 10 s limit.
        assert!((0.4..4.0).contains(&(a.len() as f32 / 48_000.0)), "{} s", a.len() as f32 / 48_000.0);
        assert_eq!(*a.last().unwrap(), 0.0, "faded to nothing");
        assert_ne!(a, b, "Variation re-rolls each take");
    }

    #[test]
    fn wav_header_is_right() {
        let w = wav(&[0.0, 1.0, -1.0], 48_000);
        assert_eq!(&w[..4], b"RIFF");
        assert_eq!(u32::from_le_bytes(w[4..8].try_into().unwrap()), 36 + 9);
        assert_eq!(u32::from_le_bytes(w[24..28].try_into().unwrap()), 48_000);
        assert_eq!(u16::from_le_bytes(w[34..36].try_into().unwrap()), 24);
        assert_eq!(&w[44 + 3..44 + 6], &[0xff, 0xff, 0x7f]);
        assert_eq!(w.len(), 44 + 9);
    }
}
