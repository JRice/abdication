//! The amp and filter envelopes: ADSR whose attack, decay and release each have their own
//! curve. A retrigger starts the attack from wherever the envelope is, so a repeated note
//! never clicks. The same curves shape the dynamic modulators' AD envelopes, and the panel
//! draws them with `curve` too.

/// How far a curve of ±1 bends: the exponent at full bow.
const CURVE_DEPTH: f32 = 5.0;

/// The fraction of the way along a segment at `progress` (0..1), for a segment that rises.
/// `bow` 0 is straight; positive bows the line up (fast first), negative down.
#[inline(always)]
pub fn curve(progress: f32, bow: f32) -> f32 {
    let k = bow * CURVE_DEPTH;
    if k.abs() < 1e-3 {
        progress
    } else {
        (1.0 - (-k * progress).exp()) / (1.0 - (-k).exp())
    }
}

/// A segment's value at `progress`, going from `from` to `to`, bowed up (positive) or down
/// as it looks on a graph, whichever way it runs.
#[inline(always)]
pub fn along(from: f32, to: f32, progress: f32, bow: f32) -> f32 {
    let bow = if to >= from { bow } else { -bow };
    from + (to - from) * curve(progress.clamp(0.0, 1.0), bow)
}

/// Each segment's bow (see `curve`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Curves {
    pub attack: f32,
    pub decay: f32,
    pub release: f32,
}

impl Default for Curves {
    /// Close to the analog-style curves the envelopes had before they could be bent.
    fn default() -> Self {
        Self { attack: DEFAULT_ATTACK_BOW, decay: DEFAULT_FALL_BOW, release: DEFAULT_FALL_BOW }
    }
}

pub const DEFAULT_ATTACK_BOW: f32 = 0.3;
pub const DEFAULT_FALL_BOW: f32 = -0.6;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Stage {
    Idle,
    Attack,
    Decay,
    Sustain,
    Release,
}

#[derive(Clone, Copy, Debug)]
pub struct Times {
    pub attack_s: f32,
    pub decay_s: f32,
    pub sustain: f32,
    pub release_s: f32,
    pub curves: Curves,
}

/// A block's envelope settings at the envelope's sample rate.
#[derive(Clone, Copy, Debug)]
pub struct Rates {
    attack: f32,
    decay: f32,
    sustain: f32,
    release: f32,
    curves: Curves,
    follow: f32,
}

/// How far through a segment of `seconds` one sample goes.
fn step(seconds: f32, rate: f32) -> f32 {
    1.0 / (seconds.max(0.0005) * rate)
}

impl Rates {
    pub fn new(t: Times, rate: f32) -> Self {
        Self {
            attack: step(t.attack_s, rate),
            decay: step(t.decay_s, rate),
            sustain: t.sustain.clamp(0.0, 1.0),
            release: step(t.release_s, rate),
            curves: t.curves,
            follow: step(0.01, rate),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Adsr {
    stage: Stage,
    pub value: f32,
    /// Where the current segment started, and how far through it the envelope is.
    from: f32,
    progress: f32,
}

impl Default for Adsr {
    fn default() -> Self {
        Self { stage: Stage::Idle, value: 0.0, from: 0.0, progress: 0.0 }
    }
}

impl Adsr {
    fn enter(&mut self, stage: Stage) {
        self.stage = stage;
        self.from = self.value;
        self.progress = 0.0;
    }

    pub fn trigger(&mut self) {
        self.enter(Stage::Attack);
    }

    pub fn release(&mut self) {
        if self.stage != Stage::Idle {
            self.enter(Stage::Release);
        }
    }

    pub fn idle(&self) -> bool {
        self.stage == Stage::Idle
    }

    pub fn releasing(&self) -> bool {
        self.stage == Stage::Release
    }

    pub fn clear(&mut self) {
        *self = Self::default();
    }

    #[inline(always)]
    pub fn next(&mut self, r: &Rates) -> f32 {
        match self.stage {
            Stage::Idle => {}
            Stage::Attack => {
                // From partway up, only the rest of the attack is left to go.
                self.progress += r.attack / (1.0 - self.from).max(1e-3);
                self.value = along(self.from, 1.0, self.progress, r.curves.attack);
                if self.progress >= 1.0 {
                    self.value = 1.0;
                    self.enter(Stage::Decay);
                }
            }
            Stage::Decay => {
                self.progress += r.decay;
                self.value = along(1.0, r.sustain, self.progress, r.curves.decay);
                if self.progress >= 1.0 {
                    self.enter(Stage::Sustain);
                }
            }
            // Follows the knob without zipper noise.
            Stage::Sustain => self.value += (r.sustain - self.value) * r.follow,
            Stage::Release => {
                self.progress += r.release;
                self.value = along(self.from, 0.0, self.progress, r.curves.release);
                if self.progress >= 1.0 {
                    self.value = 0.0;
                    self.stage = Stage::Idle;
                }
            }
        }
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RATE: f32 = 48_000.0;

    fn times(curves: Curves) -> Times {
        Times { attack_s: 0.01, decay_s: 0.1, sustain: 0.5, release_s: 0.1, curves }
    }

    #[test]
    fn attack_decay_sustain_release_on_time() {
        let r = Rates::new(times(Curves::default()), RATE);
        let mut e = Adsr::default();
        e.trigger();
        let peak = (0..(0.01 * RATE) as usize + 1).map(|_| e.next(&r)).fold(0.0, f32::max);
        assert!((peak - 1.0).abs() < 1e-6, "peaks when the attack is over");
        for _ in 0..(0.1 * RATE) as usize + 1 {
            e.next(&r);
        }
        assert!((e.value - 0.5).abs() < 1e-3, "at sustain when the decay is over: {}", e.value);
        e.release();
        for _ in 0..(0.1 * RATE) as usize + 1 {
            e.next(&r);
        }
        assert!(e.idle());
    }

    #[test]
    fn curves_bow_the_way_they_say() {
        assert!((curve(0.5, 0.0) - 0.5).abs() < 1e-6);
        assert!(curve(0.5, 0.8) > 0.6 && curve(0.5, -0.8) < 0.4);
        assert!((curve(1.0, 0.8) - 1.0).abs() < 1e-6 && curve(0.0, -0.8).abs() < 1e-6);
        // Falling, a positive bow still sits above the straight line.
        assert!(along(1.0, 0.0, 0.5, 0.8) > 0.5 && along(1.0, 0.0, 0.5, -0.8) < 0.5);
    }

    #[test]
    fn a_retrigger_rises_from_where_it_was() {
        let r = Rates::new(times(Curves::default()), RATE);
        let mut e = Adsr::default();
        e.trigger();
        for _ in 0..(0.3 * RATE) as usize {
            e.next(&r);
        }
        e.release();
        for _ in 0..(0.02 * RATE) as usize {
            e.next(&r);
        }
        let before = e.value;
        e.trigger();
        let after = e.next(&r);
        assert!(after >= before && after - before < 0.01, "{before} -> {after}");
    }
}
