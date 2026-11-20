//! Oscillator II: "Argon-like" wavetables from Abdication's own image waves (the A8 set,
//! in the library's order), grouped in fives: A8 01–05 is table 1, … A8 26–30 table 6
//! (A8 31–32 left over). Position crossfades across a table's five waves. The waves are
//! Abdication's band-limited copies (one per octave of harmonics), read here as the
//! Wavetable reads them.

include!("../../../../abdication/plugin/src/wavelib_layout.rs");

static DATA: &[u8] = include_bytes!("../../../../abdication/plugin/src/wavelib.bin");

/// Tables of five.
pub const TABLES: usize = 6;
pub const PER_TABLE: usize = 5;

#[inline(always)]
fn sample(index: usize) -> f32 {
    let i = index * 4;
    f32::from_le_bytes([DATA[i], DATA[i + 1], DATA[i + 2], DATA[i + 3]])
}

/// Library wave `w` at `phase` (0..1), from the copy whose harmonics stay under
/// ~20 kHz (or 0.45 of the rate) at `hz`.
#[inline(always)]
pub fn wave(w: usize, hz: f32, rate: f32, phase: f32) -> f32 {
    let limit = 20_000f32.min(0.45 * rate);
    let top = LEVEL_SIZES.len() - 1;
    let level = (0..top).take_while(|&k| LEVEL_HARMONICS[k] as f32 * hz > limit).count();
    let size = LEVEL_SIZES[level];
    let base = w * WAVE_STRIDE + LEVEL_OFFSETS[level];
    let x = phase * size as f32;
    let i = (x as usize).min(size - 1);
    let frac = x - i as f32;
    let (a, b) = (sample(base + i), sample(base + (i + 1) % size));
    a + (b - a) * frac
}

/// Table `table` (0..TABLES) at `position` (0..1 across its five waves).
#[inline(always)]
pub fn read(table: usize, position: f32, hz: f32, rate: f32, phase: f32) -> f32 {
    let first = table.min(TABLES - 1) * PER_TABLE + WAVE_SETS[0].0 as usize;
    let p = position.clamp(0.0, 1.0) * (PER_TABLE - 1) as f32;
    let lo = (p as usize).min(PER_TABLE - 2);
    let mix = p - lo as f32;
    let a = wave(first + lo, hz, rate, phase);
    if mix <= 0.0 { a } else { a + (wave(first + lo + 1, hz, rate, phase) - a) * mix }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_set_fills_six_tables() {
        let (start, end) = WAVE_SETS[0];
        assert_eq!(start, 0);
        assert!((end - start) as usize >= TABLES * PER_TABLE);
        assert_eq!(DATA.len() / 4, WAVE_COUNT * WAVE_STRIDE);
    }

    #[test]
    fn position_crossfades_neighbours() {
        let (hz, rate, ph) = (65.4, 48_000.0, 0.37);
        let a = wave(5, hz, rate, ph);
        let b = wave(6, hz, rate, ph);
        assert!((read(1, 0.0, hz, rate, ph) - a).abs() < 1e-6);
        assert!((read(1, 0.125, hz, rate, ph) - 0.5 * (a + b)).abs() < 1e-5);
        assert!((read(1, 1.0, hz, rate, ph) - wave(9, hz, rate, ph)).abs() < 1e-6);
    }
}
