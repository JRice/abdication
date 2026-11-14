//! The calls' echo: a delay per side (different lengths, for width), each
//! feeding back on itself through a lowpass. Abdication's was the same delay
//! with one knob for both its level and feedback, at fixed times (0.36 and
//! 0.48 s); here they're all open, and the times can follow the tempo.

use nih_plug::prelude::Enum;

/// The longest echo, in seconds (a synced one is held to it too).
pub const MAX_SECONDS: f32 = 4.0;

/// A synced echo's length, in beats.
#[derive(Enum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Division {
    #[name = "1/32"]
    D32,
    #[name = "1/16 T"]
    D16T,
    #[name = "1/16"]
    D16,
    #[name = "1/16 ."]
    D16D,
    #[name = "1/8 T"]
    D8T,
    #[name = "1/8"]
    D8,
    #[name = "1/8 ."]
    D8D,
    #[name = "1/4 T"]
    D4T,
    #[name = "1/4"]
    D4,
    #[name = "1/4 ."]
    D4D,
    #[name = "1/2"]
    D2,
    #[name = "1/2 ."]
    D2D,
    #[name = "1 bar"]
    D1,
}

impl Division {
    pub fn beats(self) -> f32 {
        use Division::*;
        match self {
            D32 => 0.125,
            D16T => 1.0 / 6.0,
            D16 => 0.25,
            D16D => 0.375,
            D8T => 1.0 / 3.0,
            D8 => 0.5,
            D8D => 0.75,
            D4T => 2.0 / 3.0,
            D4 => 1.0,
            D4D => 1.5,
            D2 => 2.0,
            D2D => 3.0,
            D1 => 4.0,
        }
    }

    pub fn seconds(self, tempo: f64) -> f32 {
        (self.beats() as f64 * 60.0 / tempo.max(1.0)) as f32
    }
}

/// How long a change of time takes to arrive (its pitch bends meanwhile, as
/// on tape), and the time constant of the other controls' smoothing.
const TIME_GLIDE_S: f32 = 0.08;

pub struct Echo {
    lines: [Vec<f32>; 2],
    at: usize,
    sample_rate: f32,
    /// Each side's delay in samples, as it glides to its target.
    delay: [f32; 2],
    lowpass: [f32; 2],
    glide: f32,
}

/// What the echo is set to.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EchoSettings {
    /// Into the echo, 0..1 (Abdication's: 0.25, its Echo knob at half).
    pub send: f32,
    /// Feedback, 0..0.95.
    pub feedback: f32,
    /// Each side's length in seconds.
    pub seconds: [f32; 2],
    /// The lowpass in the loop, Hz (each repeat darker).
    pub damp_hz: f32,
}

impl Echo {
    pub fn new(sample_rate: f32) -> Self {
        let len = (MAX_SECONDS * sample_rate) as usize + 4;
        Self { lines: [vec![0.0; len], vec![0.0; len]], at: 0, sample_rate, delay: [0.0; 2], lowpass: [0.0; 2],
               glide: 1.0 - (-1.0 / (TIME_GLIDE_S * sample_rate)).exp() }
    }

    pub fn clear(&mut self) {
        self.lines.iter_mut().for_each(|l| l.fill(0.0));
        self.lowpass = [0.0; 2];
        self.delay = [0.0; 2];
    }

    fn samples(&self, seconds: f32) -> f32 {
        (seconds.clamp(0.001, MAX_SECONDS) * self.sample_rate).max(1.0)
    }

    /// The echoes of `input` into `output`, side by side.
    pub fn process(&mut self, settings: &EchoSettings, input: [&[f32]; 2], output: [&mut [f32]; 2]) {
        let target = settings.seconds.map(|s| self.samples(s));
        // A fresh echo starts at its length rather than gliding there.
        for side in 0..2 {
            if self.delay[side] == 0.0 {
                self.delay[side] = target[side];
            }
        }
        let damp = 1.0 - (-2.0 * std::f32::consts::PI * settings.damp_hz.min(0.45 * self.sample_rate) / self.sample_rate).exp();
        let feedback = settings.feedback.clamp(0.0, 0.95);
        let len = self.lines[0].len();
        let [out_l, out_r] = output;
        let outs = [out_l, out_r];
        for i in 0..input[0].len() {
            for side in 0..2 {
                self.delay[side] += self.glide * (target[side] - self.delay[side]);
                let back = self.delay[side];
                let whole = back.floor();
                let frac = back - whole;
                let a = (self.at + len - whole as usize) % len;
                let b = (a + len - 1) % len;
                let line = &mut self.lines[side];
                let y = line[a] + frac * (line[b] - line[a]);
                self.lowpass[side] += damp * (y - self.lowpass[side]);
                line[self.at] = settings.send * input[side][i] + feedback * self.lowpass[side];
                outs[side][i] = y;
            }
            self.at = (self.at + 1) % len;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divisions_at_a_tempo() {
        assert!((Division::D4.seconds(120.0) - 0.5).abs() < 1e-6);
        assert!((Division::D8D.seconds(120.0) - 0.375).abs() < 1e-6);
        assert!((Division::D8T.seconds(90.0) - 2.0 / 9.0).abs() < 1e-6);
        assert!((Division::D1.seconds(60.0) - 4.0).abs() < 1e-6);
    }

    /// An impulse comes back at each side's length, then again (fed back),
    /// quieter.
    #[test]
    fn an_impulse_comes_back_at_its_time() {
        let sr = 48_000.0;
        let mut e = Echo::new(sr);
        let settings = EchoSettings { send: 1.0, feedback: 0.5, seconds: [0.25, 0.4], damp_hz: 20_000.0 };
        let n = 48_000;
        let mut impulse = vec![0.0; n];
        impulse[0] = 1.0;
        let (mut l, mut r) = (vec![0.0; n], vec![0.0; n]);
        e.process(&settings, [&impulse, &impulse], [&mut l, &mut r]);
        let peak = |x: &[f32], from: usize, to: usize| {
            (from..to).max_by(|&a, &b| x[a].abs().total_cmp(&x[b].abs())).unwrap()
        };
        assert_eq!(peak(&l, 1, 18_000), 12_000);
        assert_eq!(peak(&r, 1, 25_000), 19_200);
        let second = peak(&l, 18_000, 30_000);
        assert_eq!(second, 24_000);
        assert!(l[second] < 0.6 && l[second] > 0.3, "fed back at about half: {}", l[second]);
    }

    #[test]
    fn nothing_in_nothing_out() {
        let mut e = Echo::new(48_000.0);
        let settings = EchoSettings { send: 0.0, feedback: 0.9, seconds: [0.3, 0.3], damp_hz: 5000.0 };
        let input = vec![1.0; 4800];
        let (mut l, mut r) = (vec![0.0; 4800], vec![0.0; 4800]);
        e.process(&settings, [&input, &input], [&mut l, &mut r]);
        assert!(l.iter().chain(&r).all(|s| *s == 0.0));
    }
}
