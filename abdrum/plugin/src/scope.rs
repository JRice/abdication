//! The panel's scopes: each part's output (Tone, Body, Noise, Sample 1,
//! Sample 2) for the latest hit, frozen once it has rung out.
//!
//! The engine records the hit's first second into a `Recorder` (its own
//! buffers, allocated once) and copies what's new into a shared `HitScope`
//! whenever the editor isn't reading it, so the audio thread never waits.
//! The editor turns a capture into traces with `HitScope::traces`.

use std::sync::{Arc, Mutex};

/// Tone, Body, Noise, Sample 1, Sample 2: the voice's outputs, in order.
pub const PARTS: usize = 5;
/// How much of a hit is kept.
pub const CAPTURE_SECONDS: f32 = 1.0;
/// A trace ends where its part has fallen this far below its peak...
const END_BELOW_PEAK: f32 = 0.01; // -40 dB
/// ...but shows at least this much.
const MIN_WINDOW_S: f32 = 0.005;
/// Below this a part is silent: nothing to show.
const SILENT: f32 = 1e-5;

/// One hit's part outputs, as far as they have been recorded.
pub struct Capture {
    pub parts: [Vec<f32>; PARTS],
    pub len: usize,
    /// Changes with every hit, so the editor can tell a new one.
    pub hit: u64,
    pub sample_rate: f32,
}

impl Capture {
    fn new() -> Self {
        Self { parts: Default::default(), len: 0, hit: 0, sample_rate: 44_100.0 }
    }

    fn allocate(&mut self, sample_rate: f32) {
        let n = (CAPTURE_SECONDS * sample_rate) as usize;
        for p in &mut self.parts {
            *p = vec![0.0; n];
        }
        self.len = 0;
        self.sample_rate = sample_rate;
    }
}

/// A part's trace: `(min, max)` of each of `columns` slices of its window,
/// scaled so the part's peak is 1. Empty if the part was silent.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Trace {
    pub columns: Vec<(f32, f32)>,
    /// The part's own peak level (before scaling).
    pub peak: f32,
    pub window_s: f32,
}

/// The latest hit, shared between the audio thread and the editor.
pub struct HitScope(Mutex<Capture>);

impl Default for HitScope {
    fn default() -> Self {
        Self(Mutex::new(Capture::new()))
    }
}

impl HitScope {
    /// The current hit's id and recorded length.
    pub fn current(&self) -> (u64, usize) {
        self.0.lock().map(|c| (c.hit, c.len)).unwrap_or((0, 0))
    }

    /// The hit's id and traces, if it's a hit other than `seen` or has grown since.
    pub fn traces_since(&self, seen: (u64, usize), columns: usize) -> Option<((u64, usize), [Trace; PARTS])> {
        let c = self.0.lock().ok()?;
        if (c.hit, c.len) == seen || c.len == 0 {
            return None;
        }
        Some(((c.hit, c.len), traces(&c, columns)))
    }
}

/// Every part's trace from a capture, each windowed to its own ring.
pub fn traces(c: &Capture, columns: usize) -> [Trace; PARTS] {
    std::array::from_fn(|i| trace(&c.parts[i][..c.len], c.sample_rate, columns))
}

fn trace(x: &[f32], sample_rate: f32, columns: usize) -> Trace {
    let peak = x.iter().fold(0.0f32, |m, s| m.max(s.abs()));
    if peak < SILENT || columns == 0 {
        return Trace { columns: Vec::new(), peak, window_s: 0.0 };
    }
    let last = x.iter().rposition(|s| s.abs() > peak * END_BELOW_PEAK).unwrap_or(0) + 1;
    let len = last.max((MIN_WINDOW_S * sample_rate) as usize).min(x.len()).max(1);
    let cols = (0..columns)
        .map(|c| {
            let (a, b) = (c * len / columns, ((c + 1) * len / columns).max(c * len / columns + 1).min(len));
            x[a..b].iter().fold((f32::MAX, f32::MIN), |(lo, hi), s| (lo.min(*s / peak), hi.max(*s / peak)))
        })
        .collect();
    Trace { columns: cols, peak, window_s: len as f32 / sample_rate }
}

/// The engine's side: records one voice's parts from its hit on.
pub struct Recorder {
    local: Capture,
    /// The voice being recorded, until it finishes or the buffer is full.
    pub voice: Option<usize>,
    published: usize,
    share: Option<Arc<HitScope>>,
}

impl Default for Recorder {
    fn default() -> Self {
        Self { local: Capture::new(), voice: None, published: 0, share: None }
    }
}

impl Recorder {
    pub fn share(&mut self, share: Arc<HitScope>) {
        self.share = Some(share);
    }

    /// Allocates both buffers (here, not on the audio thread).
    pub fn init(&mut self, sample_rate: f32) {
        self.local.allocate(sample_rate);
        self.voice = None;
        self.published = 0;
        if let Some(Ok(mut s)) = self.share.as_ref().map(|s| s.0.lock()) {
            s.allocate(sample_rate);
        }
    }

    /// A new hit on `voice`.
    pub fn start(&mut self, voice: usize) {
        self.voice = Some(voice);
        self.local.len = 0;
        self.local.hit += 1;
        self.published = 0;
    }

    pub fn stop(&mut self) {
        self.voice = None;
    }

    /// One sample of every part.
    #[inline]
    pub fn push(&mut self, parts: [f32; PARTS]) {
        let at = self.local.len;
        if at >= self.local.parts[0].len() {
            self.voice = None;
            return;
        }
        for (buf, s) in self.local.parts.iter_mut().zip(parts) {
            buf[at] = s;
        }
        self.local.len = at + 1;
    }

    /// Copy what's new to the editor, unless it's reading right now.
    pub fn publish(&mut self) {
        let Some(share) = &self.share else { return };
        let Ok(mut s) = share.0.try_lock() else { return };
        if s.parts[0].len() != self.local.parts[0].len() {
            return; // shared after init: nothing to copy into
        }
        if s.hit != self.local.hit {
            s.hit = self.local.hit;
            s.len = 0;
            self.published = 0;
        }
        let to = self.local.len;
        let from = self.published.min(to);
        for (dst, src) in s.parts.iter_mut().zip(&self.local.parts) {
            dst[from..to].copy_from_slice(&src[from..to]);
        }
        s.len = to;
        s.sample_rate = self.local.sample_rate;
        self.published = to;
    }

    /// The local capture (for rendering offline, without a share).
    pub fn capture(&self) -> &Capture {
        &self.local
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_trace_windows_to_its_own_ring_and_scales_to_its_peak() {
        let sr = 1000.0;
        // A 100 Hz-ish decaying wave, 0.5 peak, gone to -40 dB by ~0.2 s.
        let x: Vec<f32> = (0..1000).map(|i| 0.5 * (-(i as f32) / 43.0).exp() * (i as f32 * 0.6).sin().signum()).collect();
        let t = trace(&x, sr, 50);
        assert_eq!(t.columns.len(), 50);
        assert!((t.peak - 0.5).abs() < 1e-3);
        assert!((0.18..0.22).contains(&t.window_s), "window {}", t.window_s);
        assert!(t.columns.iter().all(|(lo, hi)| *lo >= -1.0 && *hi <= 1.0 && lo <= hi));
        assert!(trace(&[0.0; 100], sr, 50).columns.is_empty(), "silence has no trace");
    }

    #[test]
    fn the_share_follows_the_recording_and_restarts_on_a_new_hit() {
        let share = Arc::new(HitScope::default());
        let mut r = Recorder::default();
        r.share(share.clone());
        r.init(1000.0);
        r.start(0);
        for i in 0..100 {
            r.push([1.0, 0.5, 0.0, 0.0, (i % 2) as f32]);
        }
        r.publish();
        let (seen, t) = share.traces_since((0, 0), 10).expect("new data");
        assert_eq!(seen, (1, 100));
        assert!(t[2].columns.is_empty() && !t[0].columns.is_empty());
        assert!(share.traces_since(seen, 10).is_none(), "nothing new");
        r.start(1);
        r.push([1.0; PARTS]);
        r.publish();
        assert_eq!(share.traces_since(seen, 10).unwrap().0, (2, 1));
    }

    /// Regression (clap-validator): re-initializing mid-hit, at another
    /// sample rate, once sliced past the recording and crashed.
    #[test]
    fn reinitializing_mid_hit_is_safe() {
        let share = Arc::new(HitScope::default());
        let mut r = Recorder::default();
        r.share(share.clone());
        r.init(1000.0);
        r.start(0);
        (0..50).for_each(|_| r.push([0.1; PARTS]));
        r.publish();
        r.init(2000.0);
        r.publish();
        r.push([0.1; PARTS]);
        r.publish();
        r.start(1);
        (0..3).for_each(|_| r.push([0.2; PARTS]));
        r.publish();
        assert_eq!(share.current().1, 3);
    }
}
