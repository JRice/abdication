//! One of an Orthogonal's two granular players (after the granular engine of
//! Arturia's Pigments, with its settings fixed): 16 overlapping grains, each
//! 180 ms under a trapezoid window (a quarter of it to rise, a quarter to
//! fall), a new one every 180/16 ms, starting at a random spot within the
//! scatter around the player's position and playing forward from there, each
//! detuned at random by up to ±15 cents, all at full level.
//!
//! The grains are summed and divided by the root of the sum of their
//! windows' squares: grains from scattered spots are unrelated, so that holds
//! the level steady as they come and go, and the first grain of a note
//! sounds at once (the voice's amp envelope makes the attack).

use crate::ray::random::Rng;

pub const GRAINS: usize = 16;
pub const GRAIN_S: f32 = 0.180;
/// Each of the window's slopes, as a share of the grain.
const SLOPE: f32 = 0.25;
pub const DETUNE_CENTS: f32 = 15.0;
/// Below this, the windows' sum of squares counts as nothing playing.
const QUIET: f32 = 1e-6;

#[derive(Clone, Copy, Default)]
struct Grain {
    /// Where it reads, in the recording's frames.
    pos: f64,
    /// Its detune, as a ratio.
    detune: f32,
    /// Samples played, and its length; 0 length: not playing.
    age: u32,
    len: u32,
    /// 1 / the length of a slope, in samples.
    per_slope: f32,
}

impl Grain {
    #[inline(always)]
    fn window(&self) -> f32 {
        (self.age.min(self.len - self.age) as f32 * self.per_slope).min(1.0)
    }
}

#[derive(Clone)]
pub struct Granular {
    grains: [Grain; GRAINS],
    /// Samples until the next grain starts.
    next: u32,
    rng: Rng,
}

/// What a player plays for a block.
#[derive(Clone, Copy)]
pub struct GrainBlock<'a> {
    /// The recording (silent outside it).
    pub src: &'a [f32],
    /// Where new grains start, in its frames, and how far either way (frames) they may.
    pub center: f32,
    pub scatter: f32,
    /// Frames per output sample at the note's pitch (before each grain's detune).
    pub step: f32,
    /// A grain's length in output samples.
    pub len: u32,
}

impl Granular {
    pub fn new(seed: u32) -> Self {
        Self { grains: [Grain::default(); GRAINS], next: 0, rng: Rng::new(seed) }
    }

    /// Silence: no grains playing; the next starts at once.
    pub fn clear(&mut self) {
        self.grains = [Grain::default(); GRAINS];
        self.next = 0;
    }

    fn start(&mut self, b: &GrainBlock) {
        let pos = b.center + b.scatter * self.rng.signed();
        let detune = (DETUNE_CENTS / 1200.0 * self.rng.signed()).exp2();
        // The oldest (or a free) grain makes way; with grains GRAIN_S/GRAINS
        // apart that is always one that has just finished.
        let g = self.grains.iter_mut().max_by_key(|g| if g.len == 0 { u32::MAX } else { g.age }).unwrap();
        let len = b.len.max(4);
        *g = Grain { pos: pos as f64, detune, age: 0, len, per_slope: 1.0 / (SLOPE * len as f32) };
    }

    /// Add `gain` times one block of the grains to `out`.
    pub fn render(&mut self, b: &GrainBlock, gain: f32, out: &mut [f32]) {
        let interval = (b.len / GRAINS as u32).max(1);
        let mut start = 0;
        while start < out.len() {
            if self.next == 0 {
                self.start(b);
                self.next = interval;
            }
            let end = out.len().min(start + self.next as usize);
            self.run(b, gain, &mut out[start..end]);
            self.next -= (end - start) as u32;
            start = end;
        }
    }

    /// The grains over a stretch in which none starts.
    fn run(&mut self, b: &GrainBlock, gain: f32, out: &mut [f32]) {
        let mut sum = [0.0f32; 64];
        let mut power = [0.0f32; 64];
        for chunk_start in (0..out.len()).step_by(64) {
            let n = (out.len() - chunk_start).min(64);
            sum[..n].fill(0.0);
            power[..n].fill(0.0);
            for g in self.grains.iter_mut().filter(|g| g.len > 0) {
                let step = (b.step * g.detune) as f64;
                let run = n.min((g.len - g.age) as usize);
                for i in 0..run {
                    let w = g.window();
                    sum[i] += w * read(b.src, g.pos);
                    power[i] += w * w;
                    g.pos += step;
                    g.age += 1;
                }
                if g.age >= g.len {
                    g.len = 0;
                }
            }
            for i in 0..n {
                if power[i] > QUIET {
                    out[chunk_start + i] += gain * sum[i] / power[i].sqrt();
                }
            }
        }
    }
}

/// The recording at fractional frame `pos` (4-point Hermite), silent outside it.
#[inline(always)]
fn read(src: &[f32], pos: f64) -> f32 {
    if !(pos >= 1.0 && pos < src.len() as f64 - 2.0) {
        return 0.0;
    }
    let i = pos as usize;
    let t = (pos - i as f64) as f32;
    let (y0, y1, y2, y3) = (src[i - 1], src[i], src[i + 1], src[i + 2]);
    let c1 = 0.5 * (y2 - y0);
    let c2 = y0 - 2.5 * y1 + 2.0 * y2 - 0.5 * y3;
    let c3 = 0.5 * (y3 - y0) + 1.5 * (y1 - y2);
    ((c3 * t + c2) * t + c1) * t + y1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sine(hz: f32, seconds: f32) -> Vec<f32> {
        (0..(seconds * 44_100.0) as usize).map(|i| (i as f32 * hz / 44_100.0 * std::f32::consts::TAU).sin()).collect()
    }

    fn play(src: &[f32], center: f32, scatter: f32, step: f32, samples: usize) -> Vec<f32> {
        let mut g = Granular::new(1);
        let b = GrainBlock { src, center, scatter, step, len: (GRAIN_S * 48_000.0) as u32 };
        let mut out = vec![0.0; samples];
        for block in out.chunks_mut(64) {
            g.render(&b, 1.0, block);
        }
        out
    }

    fn rms(x: &[f32]) -> f32 {
        (x.iter().map(|v| v * v).sum::<f32>() / x.len() as f32).sqrt()
    }

    /// Scattered grains of a sine keep about its level, and start at once.
    #[test]
    fn level_holds_from_the_first_grain() {
        let src = sine(220.0, 4.0);
        let out = play(&src, 88_200.0, 4410.0, 44_100.0 / 48_000.0, 48_000);
        let early = rms(&out[200..2400]);
        let steady = rms(&out[24_000..]);
        assert!(early > 0.5, "the first grain is quiet: {early}");
        assert!(steady > 0.4 && steady < 1.2, "steady level {steady}");
        assert!(out.iter().all(|v| v.is_finite()));
    }

    /// Outside the recording it's silent.
    #[test]
    fn silent_past_the_end() {
        let src = sine(220.0, 1.0);
        let out = play(&src, 100_000.0, 0.0, 1.0, 20_000);
        assert!(out.iter().all(|v| *v == 0.0));
    }

    /// Sixteen grains overlap, one starting every 180/16 ms.
    #[test]
    fn sixteen_overlap() {
        let src = sine(220.0, 2.0);
        let mut g = Granular::new(7);
        let b = GrainBlock { src: &src, center: 1000.0, scatter: 0.0, step: 1.0, len: 8640 };
        let mut out = [0.0; 64];
        for _ in 0..(8640 / 64 + 4) {
            g.render(&b, 1.0, &mut out);
        }
        assert_eq!(g.grains.iter().filter(|g| g.len > 0).count(), GRAINS);
    }
}

#[cfg(test)]
mod bench {
    use super::*;

    /// ns per grain per sample: `cargo test --release grain_cost -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn grain_cost() {
        let src: Vec<f32> = (0..441_000).map(|i| (i as f32 * 0.01).sin()).collect();
        let mut g = Granular::new(3);
        let b = GrainBlock { src: &src, center: 100_000.0, scatter: 2205.0, step: 0.93, len: 8640 };
        let mut out = [0.0f32; 64];
        let blocks = 20_000;
        let t = std::time::Instant::now();
        for _ in 0..blocks {
            g.render(&b, 1.0, &mut out);
        }
        let ns = t.elapsed().as_secs_f64() * 1e9 / (blocks * 64 * GRAINS) as f64;
        println!("{ns:.2} ns per grain-sample ({:.1} us per 64-sample block of one player)", ns * 64.0 * 16.0 / 1000.0);
        assert!(out.iter().all(|v| v.is_finite()));
    }
}
