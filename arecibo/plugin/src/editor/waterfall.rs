//! The receiver's display: what's on the band, scrolling sideways over time. It's where you
//! can see the machine thinking. Works on the panel's thread, not the audio's.

use std::collections::VecDeque;
use std::f32::consts::PI;

pub const COLUMNS: usize = 128;
pub const ROWS: usize = 40;
pub const SHADES: usize = 6;
pub const WINDOW: usize = 512;
const LOWEST_HZ: f32 = 60.0;
const HIGHEST_HZ: f32 = 11_000.0;
const FLOOR_DB: f32 = -78.0;
const CEILING_DB: f32 = -18.0;

/// In-place radix-2, as the display doesn't need anything clever.
fn fft(re: &mut [f32], im: &mut [f32]) {
    let n = re.len();
    let mut j = 0;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j ^= bit;
        if i < j {
            re.swap(i, j);
            im.swap(i, j);
        }
    }
    let mut size = 2;
    while size <= n {
        let step = -2.0 * PI / size as f32;
        for start in (0..n).step_by(size) {
            for k in 0..size / 2 {
                let (sin, cos) = (step * k as f32).sin_cos();
                let (a, b) = (start + k, start + k + size / 2);
                let (tr, ti) = (cos * re[b] - sin * im[b], sin * re[b] + cos * im[b]);
                (re[b], im[b]) = (re[a] - tr, im[a] - ti);
                (re[a], im[a]) = (re[a] + tr, im[a] + ti);
            }
        }
        size <<= 1;
    }
}

pub struct Waterfall {
    columns: VecDeque<[u8; ROWS]>,
    window: [f32; WINDOW],
    re: [f32; WINDOW],
    im: [f32; WINDOW],
    /// The FFT bins each row covers.
    row_bins: [(usize, usize); ROWS],
}

impl Waterfall {
    pub fn new(sample_rate: f32) -> Self {
        let bin_hz = sample_rate / WINDOW as f32;
        let row_bins = std::array::from_fn(|row| {
            let edge = |r: usize| LOWEST_HZ * (HIGHEST_HZ / LOWEST_HZ).powf(r as f32 / ROWS as f32);
            let low = ((edge(row) / bin_hz) as usize).clamp(1, WINDOW / 2 - 1);
            (low, ((edge(row + 1) / bin_hz) as usize).clamp(low + 1, WINDOW / 2))
        });
        Self {
            columns: VecDeque::from(vec![[0; ROWS]; COLUMNS]),
            window: std::array::from_fn(|i| 0.5 - 0.5 * (2.0 * PI * i as f32 / (WINDOW - 1) as f32).cos()),
            re: [0.0; WINDOW],
            im: [0.0; WINDOW],
            row_bins,
        }
    }

    /// Takes in the newest samples as one new column, and tells whether anything showed.
    pub fn add(&mut self, samples: &[f32; WINDOW]) -> bool {
        for ((re, sample), window) in self.re.iter_mut().zip(samples).zip(&self.window) {
            *re = sample * window;
        }
        self.im = [0.0; WINDOW];
        fft(&mut self.re, &mut self.im);
        // A sine at full scale should read 0 dB. A Hann window halves its peak.
        let normal = 4.0 / WINDOW as f32;
        let mut column = [0u8; ROWS];
        for (shade, (low, high)) in column.iter_mut().zip(&self.row_bins) {
            let power = (*low..*high).map(|b| self.re[b] * self.re[b] + self.im[b] * self.im[b]).fold(0.0, f32::max) * normal * normal;
            let db = 10.0 * power.max(1e-12).log10();
            *shade = (((db - FLOOR_DB) / (CEILING_DB - FLOOR_DB)).clamp(0.0, 1.0) * SHADES as f32) as u8;
        }
        let showing = column.iter().any(|s| *s > 0) || self.columns.back().is_some_and(|c| c.iter().any(|s| *s > 0));
        self.columns.pop_front();
        self.columns.push_back(column);
        showing
    }

    /// Oldest first. Each is `ROWS` shades, bottom row first, 0 meaning nothing there.
    pub fn columns(&self) -> impl Iterator<Item = &[u8; ROWS]> {
        self.columns.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tone(hz: f32, rate: f32) -> [f32; WINDOW] {
        std::array::from_fn(|i| 0.5 * (2.0 * PI * hz * i as f32 / rate).sin())
    }

    #[test]
    fn a_tone_shows_up_at_its_own_height_and_silence_shows_nothing() {
        let mut waterfall = Waterfall::new(24_000.0);
        assert!(!waterfall.add(&[0.0; WINDOW]));
        let height = |waterfall: &mut Waterfall, hz: f32| {
            waterfall.add(&tone(hz, 24_000.0));
            let column = waterfall.columns().last().unwrap();
            (0..ROWS).max_by_key(|r| column[*r]).unwrap()
        };
        let (low, high) = (height(&mut waterfall, 300.0), height(&mut waterfall, 5_000.0));
        assert!(high > low + 10, "{low} {high}");
    }

    #[test]
    fn old_columns_scroll_away() {
        let mut waterfall = Waterfall::new(24_000.0);
        waterfall.add(&tone(1_000.0, 24_000.0));
        for _ in 0..COLUMNS {
            waterfall.add(&[0.0; WINDOW]);
        }
        assert!(waterfall.columns().all(|c| c.iter().all(|s| *s == 0)));
    }
}
