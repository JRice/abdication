//! One voice's Windows: the oscillator slots set to Windows, played for each
//! unison copy that runs. As with Ray (ray/player.rs), the Faust copy gives
//! each slot's pitch, the copy's start phase and the note-on, and the slot's
//! controls are read from the copy's DSP, where the engine has set them for
//! this voice (modulated).

use super::{WindowsOsc, WindowsSettings};

/// Windows' level in the mixer: the base alone is a full-scale wave, a
/// Dual's saw's level.
pub const WINDOWS_GAIN: f32 = 1.0;

#[derive(Clone, Copy, Debug, Default)]
struct WindowsSlot {
    osc: WindowsOsc,
    /// Cycles per sample at the end of the last block (0 before the first).
    step: f64,
}

/// One voice's Windows oscillators: per unison copy, per slot.
pub struct WindowsPlayer {
    slots: Vec<[WindowsSlot; 2]>,
    rate: f32,
}

impl WindowsPlayer {
    pub fn new(copies: usize) -> Self {
        Self { slots: vec![[WindowsSlot::default(); 2]; copies], rate: 48_000.0 }
    }

    pub fn init(&mut self, rate: f32) {
        self.rate = rate;
        self.clear();
    }

    pub fn clear(&mut self) {
        self.slots.iter_mut().flatten().for_each(|s| *s = WindowsSlot::default());
    }

    /// Play slot `s` of unison copy `copy` for one block into `out`
    /// (replacing it): `hz` is the slot's pitch at the block's end, `phase`
    /// the copy's start phase, `note_on` whether a note started in this block.
    #[allow(clippy::too_many_arguments)]
    pub fn render(&mut self, copy: usize, s: usize, settings: &WindowsSettings, hz: f32, phase: f32, note_on: bool,
                  out: &mut [f32]) {
        let slot = &mut self.slots[copy][s];
        let step = (hz / self.rate) as f64;
        if note_on || !slot.osc.started() {
            slot.osc.start(phase as f64, settings, step);
            slot.step = 0.0;
        }
        let start = if slot.step > 0.0 { slot.step } else { step };
        slot.osc.render(settings, (start, step), out);
        out.iter_mut().for_each(|x| *x *= WINDOWS_GAIN);
        slot.step = step;
    }
}

#[cfg(test)]
pub(crate) mod tests {
    /// An in-place radix-2 FFT (re, im; length a power of 2).
    fn fft(re: &mut [f64], im: &mut [f64]) {
        let n = re.len();
        let mut j = 0;
        for i in 1..n {
            let mut bit = n >> 1;
            while j & bit != 0 {
                j ^= bit;
                bit >>= 1;
            }
            j |= bit;
            if i < j {
                re.swap(i, j);
                im.swap(i, j);
            }
        }
        let mut len = 2;
        while len <= n {
            let a = -2.0 * std::f64::consts::PI / len as f64;
            for start in (0..n).step_by(len) {
                for k in 0..len / 2 {
                    let (c, s) = ((a * k as f64).cos(), (a * k as f64).sin());
                    let (p, q) = (start + k, start + k + len / 2);
                    let (xr, xi) = (re[q] * c - im[q] * s, re[q] * s + im[q] * c);
                    re[q] = re[p] - xr;
                    im[q] = im[p] - xi;
                    re[p] += xr;
                    im[p] += xi;
                }
            }
            len <<= 1;
        }
    }

    /// How far (dB) the power away from `hz`'s harmonics sits under the whole:
    /// a steady tone's aliasing (and noise). Takes the largest power-of-2 start of `x`.
    pub(crate) fn alias_db(x: &[f32], hz: f64, rate: f64) -> f64 {
        let n = 1 << (usize::BITS - 1 - x.len().leading_zeros());
        let hann = |k: usize| 0.5 - 0.5 * (2.0 * std::f64::consts::PI * k as f64 / n as f64).cos();
        let mut re: Vec<f64> = (0..n).map(|k| x[k] as f64 * hann(k)).collect();
        let mut im = vec![0.0; n];
        fft(&mut re, &mut im);
        let (mut total, mut off) = (0.0, 0.0);
        for b in 1..n / 2 {
            let f = b as f64 * rate / n as f64;
            let p = re[b] * re[b] + im[b] * im[b];
            let h = (f / hz).round();
            let bins_away = ((f - h * hz) * n as f64 / rate).abs();
            total += p;
            if h < 1.0 || bins_away > 4.0 {
                off += p;
            }
        }
        10.0 * (off / total).log10()
    }
}
