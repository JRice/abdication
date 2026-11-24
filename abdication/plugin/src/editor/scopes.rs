//! What the panel's live screens show, and when they update. While a note
//! plays, the newest voice feeds the panel (the panel polls it 15 times a
//! second): its two oscillators' outputs for the two scopes, and its
//! moving values (pulse width, the filter, where its modulators have each knob) for the
//! other screens. With nothing playing, the scopes fall back to rendering
//! a voice offline whenever a setting that shapes them changes, and the
//! other screens draw from the knobs.

use std::sync::Arc;
use std::time::{Duration, Instant};

use nih_plug::prelude::ParamPtr;

use crate::params::SynthParams;
use crate::modulation;
use crate::scope::{self, LiveScope, Moving, ScopePitch, Tap, MOVING};

/// Live data older than this means nothing is playing.
const LIVE_TIMEOUT: Duration = Duration::from_millis(250);

fn plain(params: &SynthParams, id: &str) -> f32 {
    params.ptr(id).map(|p| unsafe { p.unmodulated_plain_value() }).unwrap_or(0.0)
}

/// Whether a slot plays its Dual (rather than its Wavetable).
fn plays_dual(params: &SynthParams, tap: Tap) -> bool {
    plain(params, &format!("osc_{}.mode", tap as usize)) < 0.5
}

/// The pitch an oscillator plays for a note: the note moved by its slot's Pitch.
fn tap_hz(params: &SynthParams, tap: Tap, note_hz: f32) -> f32 {
    let semis = crate::pitch::semitones(plain(params, &format!("osc_{}.pitch", tap as usize)));
    note_hz * 2f32.powf(semis / 12.0)
}

/// Cycles each scope shows: two of a Dual, one of a Wavetable.
fn cycles(params: &SynthParams, tap: Tap) -> f32 {
    if plays_dual(params, tap) { 2.0 } else { 1.0 }
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
            self.trace = scope::render(params, note_hz, hz, self.tap, cycles(params, self.tap));
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
    moved: [f32; modulation::COUNT],
    ortho: [[f32; 2]; 2],
    sources: [f32; modulation::SOURCE_COUNT],
    paths: [[f32; 2]; 2],
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
            OfflineScope::new(params, Tap::Osc1, &["osc_1.", "dual_1.", "wt_1.", "ray_1.", "windows_1.", "unison.", "vintage."]),
            OfflineScope::new(params, Tap::Osc2, &["osc_2.", "dual_2.", "wt_2.", "ray_2.", "windows_2.", "unison.", "vintage."]),
        ];
        Self { live_scope, pitch, seen: 0, live: None, offline }
    }

    /// Poll the newest voice; true if anything on screen changed.
    pub fn tick(&mut self, params: &SynthParams) -> bool {
        if let Some(snapshot) = self.live_scope.snapshot_since(self.seen) {
            self.seen = snapshot.written;
            let trace = |tap| snapshot.trace(tap, tap_hz(params, tap, snapshot.note_hz), cycles(params, tap));
            self.live = Some(Live { at: Instant::now(), traces: [trace(Tap::Osc1), trace(Tap::Osc2)],
                                    moving: snapshot.moving, moved: snapshot.moved, ortho: snapshot.ortho,
                                    sources: snapshot.sources, paths: snapshot.paths });
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

    /// Where the newest voice's modulators have knob `t` (travel), while one plays.
    /// The newest voice's modulation sources.
    pub fn sources(&self) -> Option<[f32; modulation::SOURCE_COUNT]> {
        self.live.as_ref().map(|l| l.sources)
    }

    /// The newest voice's wavetables' places on their paths.
    pub fn paths(&self) -> Option<[[f32; 2]; 2]> {
        self.live.as_ref().map(|l| l.paths)
    }

    pub fn moved(&self, t: usize) -> Option<f32> {
        self.live.as_ref().map(|l| l.moved[t])
    }

    /// Where the newest voice's Orthogonal player `half` (0 Top, 1 Bottom) of
    /// slot `slot` is in its set (0..1), while a note plays it.
    pub fn ortho_position(&self, slot: usize, half: usize) -> Option<f32> {
        self.live.as_ref().map(|l| l.ortho[slot][half]).filter(|p| p.is_finite())
    }

    /// The trace a scope shows now: live, or rendered offline; flat while
    /// Balance has turned all the way from it, where the voice doesn't
    /// compute that oscillator (and for an Orthogonal, which has no scope).
    pub fn trace(&mut self, params: &SynthParams, tap: Tap) -> Vec<f32> {
        let k = tap as usize - 1;
        let ortho = plain(params, &format!("osc_{}.mode", tap as usize)) == crate::ortho::player::MODE_ORTHO;
        if !crate::voice::slot_sounds(k, plain(params, "mix.balance")) || ortho {
            return vec![0.0; scope::POINTS];
        }
        match &self.live {
            Some(live) => live.traces[k].clone(),
            None => self.offline[k].refresh(params, self.pitch.get()).to_vec(),
        }
    }

    /// The pitch the screens draw for (the last note played).
    pub fn note_hz(&self) -> f32 {
        self.pitch.get()
    }
}
