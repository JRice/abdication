//! What the five scopes show: the latest hit played (from the audio
//! thread), or, after a setting changes, a hit rendered with the new
//! settings, so turning a knob shows its effect without playing.
//! Whichever happened last wins.
//!
//! Rendering a hit takes a while (a second of audio), so a worker thread
//! does it; while a knob moves it renders only the newest settings.

use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

use faust_types::ParamIndex;

use crate::engine::{Engine, MAX_BLOCK};
use crate::params::{DrumParams, Target, VoiceSlots};
use crate::scope::{self, HitScope, Trace, CAPTURE_SECONDS, PARTS};

/// Columns per trace: two per logical pixel of a scope.
pub const COLUMNS: usize = 334;
const RENDER_RATE: f32 = 44_100.0;
/// The offline hit: note 60 (the Pitch knob's own pitch), fairly hard.
const NOTE: u8 = 60;
const VELOCITY: f32 = 0.9;

type Traces = [Trace; PARTS];

/// Everything an offline hit depends on.
#[derive(Clone)]
struct Settings {
    values: Vec<(Target, ParamIndex, f32)>,
    pitch: f32,
    keytrack: bool,
    envelope: [f32; 4],
}

impl PartialEq for Settings {
    // (Faust's ParamIndex has no PartialEq; the values are what change.)
    fn eq(&self, other: &Self) -> bool {
        self.pitch == other.pitch
            && self.keytrack == other.keytrack
            && self.envelope == other.envelope
            && self.values.len() == other.values.len()
            && self.values.iter().zip(&other.values).all(|(a, b)| a.2 == b.2)
    }
}

impl Settings {
    fn of(params: &DrumParams) -> Self {
        Self {
            values: params.faust_values().collect(),
            pitch: params.value("tone.pitch").unwrap_or(50.0),
            keytrack: params.value("tone.keytrack").unwrap_or(0.0) > 0.5,
            envelope: params.envelope(),
        }
    }
}

/// Play one hit offline with `settings` and take its traces.
fn render(engine: &mut Engine, settings: &Settings) -> Traces {
    for &(target, index, value) in &settings.values {
        engine.set_param(target, index, value);
    }
    engine.set_tone(settings.pitch, settings.keytrack);
    engine.set_envelope(settings.envelope);
    engine.reset();
    engine.reseed(0x1234_5678);
    engine.note_on(NOTE, VELOCITY);
    let mut out = [0.0f32; MAX_BLOCK];
    for _ in 0..(CAPTURE_SECONDS * RENDER_RATE) as usize / MAX_BLOCK {
        engine.render(&mut out);
        if engine.recorder.voice.is_none() {
            break; // rung out
        }
    }
    scope::traces(engine.recorder.capture(), COLUMNS)
}

/// The worker: renders the newest settings it has been sent, until the
/// sending side goes away.
fn work(slots: VoiceSlots, requests: Receiver<Settings>, done: Arc<Mutex<Option<Traces>>>) {
    let mut engine = Engine::new(slots);
    engine.init(RENDER_RATE);
    while let Ok(mut settings) = requests.recv() {
        while let Ok(newer) = requests.try_recv() {
            settings = newer;
        }
        let traces = render(&mut engine, &settings);
        if let Ok(mut slot) = done.lock() {
            *slot = Some(traces);
        }
    }
}

pub struct Scopes {
    live: Arc<HitScope>,
    seen: (u64, usize),
    /// Showing the live hit (it keeps updating while it rings).
    showing_live: bool,
    requested: Option<Settings>,
    requests: Sender<Settings>,
    rendered: Arc<Mutex<Option<Traces>>>,
    pub traces: Traces,
}

impl Scopes {
    pub fn new(params: &DrumParams, live: Arc<HitScope>) -> Self {
        let (requests, receiver) = channel();
        let rendered = Arc::new(Mutex::new(None));
        let (slots, slot) = (params.slots(), rendered.clone());
        thread::spawn(move || work(slots, receiver, slot));
        // A hit from before the editor opened is older than the first offline picture.
        let seen = live.current();
        Self { live, seen, showing_live: false, requested: None, requests, rendered, traces: Default::default() }
    }

    /// Bring the traces up to date; true if they changed.
    pub fn refresh(&mut self, params: &DrumParams) -> bool {
        let mut changed = false;
        let settings = Settings::of(params);
        if self.requested.as_ref() != Some(&settings) {
            // A setting changed: from now on the offline picture is the newest.
            let _ = self.requests.send(settings.clone());
            self.requested = Some(settings);
            self.showing_live = false;
        }
        if let Some(traces) = self.rendered.lock().ok().and_then(|mut r| r.take()) {
            if !self.showing_live {
                self.traces = traces;
                changed = true;
            }
        }
        if let Some((seen, traces)) = self.live.traces_since(self.seen, COLUMNS) {
            let new_hit = seen.0 != self.seen.0;
            self.seen = seen;
            if new_hit || self.showing_live {
                self.traces = traces;
                self.showing_live = true;
                changed = true;
            }
        }
        changed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_patch_renders_a_tone_trace_and_silent_parts_stay_empty() {
        let params = DrumParams::from_faust();
        let mut engine = Engine::new(params.slots());
        engine.init(RENDER_RATE);
        let started = std::time::Instant::now();
        let traces = render(&mut engine, &Settings::of(&params));
        eprintln!("offline render took {:?}", started.elapsed());
        assert_eq!(traces[0].columns.len(), COLUMNS, "Tone sounds by default");
        assert!(traces[1..].iter().all(|t| t.columns.is_empty()), "the other parts are off by default");
    }

    #[test]
    fn the_worker_delivers_and_a_live_hit_takes_over() {
        let params = DrumParams::from_faust();
        let live = Arc::new(HitScope::default());
        let mut scopes = Scopes::new(&params, live.clone());
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
        while !scopes.refresh(&params) {
            assert!(std::time::Instant::now() < deadline, "no offline render");
            thread::sleep(std::time::Duration::from_millis(10));
        }
        assert!(!scopes.traces[0].columns.is_empty());

        // A hit on the audio side (Noise only) replaces the offline picture.
        let mut r = scope::Recorder::default();
        r.share(live);
        r.init(1000.0);
        r.start(0);
        for i in 0..200 {
            r.push([0.0, 0.0, if i % 2 == 0 { 0.5 } else { -0.5 }, 0.0, 0.0]);
        }
        r.publish();
        assert!(scopes.refresh(&params));
        assert!(scopes.traces[0].columns.is_empty() && !scopes.traces[2].columns.is_empty());
    }
}
