//! What the panel's live screens show, and when they update. While a note
//! plays, the newest voice feeds the panel (the panel polls it 15 times a
//! second): its Dual and Wavetable outputs for the two scopes, and its
//! moving values (pulse width, the filter, the free modulators) for the
//! other screens. With nothing playing, the scopes fall back to rendering
//! a voice offline whenever a setting that shapes them changes, and the
//! other screens draw from the knobs.

use std::sync::Arc;
use std::time::{Duration, Instant};

use nih_plug::prelude::ParamPtr;

use crate::params::SynthParams;
use crate::scope::{self, LiveScope, Moving, ScopePitch, Tap, MOVING};

/// Live data older than this means nothing is playing.
const LIVE_TIMEOUT: Duration = Duration::from_millis(250);

fn plain(params: &SynthParams, id: &str) -> f32 {
    params.ptr(id).map(|p| unsafe { p.unmodulated_plain_value() }).unwrap_or(0.0)
}

/// The pitch an oscillator plays for a note: the note moved by that
/// oscillator's own octave (and, for the Dual, semitone) settings.
fn tap_hz(params: &SynthParams, tap: Tap, note_hz: f32) -> f32 {
    let semitones = match tap {
        Tap::Dual => 12.0 * plain(params, "dual.octave") + plain(params, "dual.semi"),
        Tap::Wavetable => 12.0 * plain(params, "wavetable.octave"),
    };
    note_hz * 2f32.powf(semitones / 12.0)
}

/// Cycles each scope shows.
fn cycles(tap: Tap) -> f32 {
    match tap {
        Tap::Dual => 2.0,
        Tap::Wavetable => 1.0,
    }
}

/// A trace rendered offline, again whenever one of the parameters it
/// depends on (or the pitch) changes.
struct OfflineScope {
    tap: Tap,
    inputs: Vec<ParamPtr>,
    rendered_for: Vec<f32>,
    trace: Vec<f32>,
}

impl OfflineScope {
    /// `prefixes`: parameter groups whose settings shape this trace.
    fn new(params: &SynthParams, tap: Tap, prefixes: &[&str]) -> Self {
        let inputs = params
            .ids()
            .filter(|id| prefixes.iter().any(|p| id.starts_with(p)))
            .filter_map(|id| params.ptr(id))
            .collect();
        Self { tap, inputs, rendered_for: Vec::new(), trace: Vec::new() }
    }

    fn refresh(&mut self, params: &SynthParams, note_hz: f32) -> &[f32] {
        let mut key: Vec<f32> = self.inputs.iter().map(|p| unsafe { p.unmodulated_plain_value() }).collect();
        key.push(note_hz);
        if key != self.rendered_for {
            let hz = tap_hz(params, self.tap, note_hz);
            self.trace = scope::render(params, note_hz, hz, self.tap, cycles(self.tap));
            self.rendered_for = key;
        }
        &self.trace
    }
}

/// The newest voice, as last polled.
struct Live {
    at: Instant,
    traces: [Vec<f32>; 2],
    moving: [f32; MOVING],
}

pub struct Scopes {
    live_scope: Arc<LiveScope>,
    pitch: Arc<ScopePitch>,
    seen: u64,
    live: Option<Live>,
    offline: [OfflineScope; 2],
}

impl Scopes {
    pub fn new(params: &SynthParams, pitch: Arc<ScopePitch>, live_scope: Arc<LiveScope>) -> Self {
        let offline = [
            OfflineScope::new(params, Tap::Dual, &["dual", "unison.", "mod_", "vintage."]),
            OfflineScope::new(params, Tap::Wavetable, &["wavetable.", "envelope.", "unison.", "mod_", "vintage."]),
        ];
        Self { live_scope, pitch, seen: 0, live: None, offline }
    }

    /// Poll the newest voice; true if anything on screen changed.
    pub fn tick(&mut self, params: &SynthParams) -> bool {
        if let Some(snapshot) = self.live_scope.snapshot_since(self.seen) {
            self.seen = snapshot.written;
            let trace = |tap| snapshot.trace(tap, tap_hz(params, tap, snapshot.note_hz), cycles(tap));
            self.live = Some(Live { at: Instant::now(), traces: [trace(Tap::Dual), trace(Tap::Wavetable)],
                                    moving: snapshot.moving });
            return true;
        }
        // Went quiet: back to the knobs and offline pictures, once.
        if self.live.as_ref().is_some_and(|l| l.at.elapsed() > LIVE_TIMEOUT) {
            self.live = None;
            return true;
        }
        false
    }

    pub fn is_live(&self) -> bool {
        self.live.is_some()
    }

    /// A moving value of the newest voice, while one plays.
    pub fn moving(&self, value: Moving) -> Option<f32> {
        self.live.as_ref().map(|l| l.moving[value as usize])
    }

    /// The trace a scope shows now: live, or rendered offline (also while
    /// its fader is at 0: the voice then skips computing that oscillator).
    pub fn trace(&mut self, params: &SynthParams, tap: Tap) -> Vec<f32> {
        let k = tap as usize - 1;
        let fader = match tap {
            Tap::Dual => "mix.dual",
            Tap::Wavetable => "mix.wavetable",
        };
        match &self.live {
            Some(live) if plain(params, fader) != 0.0 => live.traces[k].clone(),
            _ => self.offline[k].refresh(params, self.pitch.get()).to_vec(),
        }
    }

    /// The pitch the screens draw for (the last note played).
    pub fn note_hz(&self) -> f32 {
        self.pitch.get()
    }
}
