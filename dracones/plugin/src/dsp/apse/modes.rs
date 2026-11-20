//! The six modes. All share one tank: two cross-coupled branches, each
//! allpass → delay → damping → allpass → delay, read by seven signed taps per side.
//! That's Dattorro's published plate (1997), which is also what Spire's Plate modes'
//! tables point to (see docs/design.md). The modes differ in what feeds the tank,
//! how big it is, how it moves and how it's read; the numbers are ours.
//!
//! Lengths are in samples at the reference rate of Dattorro's paper, so they can be
//! checked against it; the engine scales them to the host rate.

pub const REFERENCE_HZ: f32 = 29_761.0;

/// Per branch (left, right): first allpass, first delay, second allpass, second delay.
pub const TANK: [[f32; 4]; 2] = [[672.0, 4453.0, 1800.0, 3720.0], [908.0, 4217.0, 2656.0, 3163.0]];

/// Which branch a tap reads, relative to the output it feeds.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Side {
    Near,
    Far,
}

/// Which line in the branch a tap reads.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Section {
    Delay1,
    Allpass2,
    Delay2,
}

#[derive(Clone, Copy, Debug)]
pub struct Tap {
    pub side: Side,
    pub section: Section,
    pub at: f32,
    pub sign: f32,
}

const fn tap(side: Side, section: Section, at: f32, sign: f32) -> Tap {
    Tap { side, section, at, sign }
}

use Section::*;
use Side::*;

/// Seven taps per output, left then right: mostly from the far branch, which is what
/// decorrelates the two sides.
pub const TAPS: [[Tap; 7]; 2] = [
    [
        tap(Far, Delay1, 266.0, 1.0),
        tap(Far, Delay1, 2974.0, 1.0),
        tap(Far, Allpass2, 1913.0, -1.0),
        tap(Far, Delay2, 1996.0, 1.0),
        tap(Near, Delay1, 1990.0, -1.0),
        tap(Near, Allpass2, 187.0, -1.0),
        tap(Near, Delay2, 1066.0, -1.0),
    ],
    [
        tap(Far, Delay1, 353.0, 1.0),
        tap(Far, Delay1, 3627.0, 1.0),
        tap(Far, Allpass2, 1228.0, -1.0),
        tap(Far, Delay2, 2673.0, 1.0),
        tap(Near, Delay1, 2111.0, -1.0),
        tap(Near, Allpass2, 335.0, -1.0),
        tap(Near, Delay2, 121.0, -1.0),
    ],
];

/// The early reflections: six taps on the pre-delay line, after the pre-delay.
/// (milliseconds at spread 1, gain, pan from -1 left to 1 right)
pub const EARLY: [(f32, f32, f32); 6] = [(7.3, 0.8, -0.9), (11.9, 0.7, 0.9), (17.7, 0.6, -0.5), (23.1, 0.5, 0.7), (31.3, 0.4, 1.0), (41.9, 0.3, -1.0)];

pub struct Mode {
    pub name: &'static str,
    /// Input diffusers in series: (length, gain).
    pub diffusers: &'static [(f32, f32)],
    /// The input's lowpass, before anything else.
    pub bandwidth_hz: f32,
    /// How big the whole structure is, against the reference.
    pub size: f32,
    /// The gains of each branch's first and second allpass.
    pub tank_diffusion: [f32; 2],
    /// Peak excursion of the first allpasses (the second ones move half as far).
    pub wobble_ms: f32,
    pub wobble_hz: f32,
    /// Weights on the seven output taps.
    pub taps: [f32; 7],
    /// The tail's level.
    pub late: f32,
    /// The early reflections' level and how far apart they are.
    pub early: f32,
    pub early_spread: f32,
    /// Fixed highpass on the wet signal.
    pub low_cut_hz: f32,
    /// Whether the tail is sung by the choir (see `choir.rs`).
    pub choir: bool,
}

const EVEN: [f32; 7] = [1.0; 7];

pub const MODES: [Mode; 6] = [
    Mode {
        name: "Plate",
        diffusers: &[(142.0, 0.75), (107.0, 0.75), (379.0, 0.625), (277.0, 0.625)],
        bandwidth_hz: 13_000.0,
        size: 1.0,
        tank_diffusion: [0.7, 0.5],
        wobble_ms: 0.54,
        wobble_hz: 1.0,
        taps: EVEN,
        late: 1.12,
        early: 0.0,
        early_spread: 1.0,
        low_cut_hz: 20.0,
        choir: false,
    },
    Mode {
        name: "Hall",
        diffusers: &[(142.0, 0.75), (107.0, 0.75), (379.0, 0.625), (277.0, 0.625), (523.0, 0.6), (701.0, 0.55)],
        bandwidth_hz: 9_000.0,
        size: 1.4,
        tank_diffusion: [0.65, 0.5],
        wobble_ms: 0.9,
        wobble_hz: 0.35,
        taps: EVEN,
        late: 1.2,
        early: 0.35,
        early_spread: 1.0,
        low_cut_hz: 25.0,
        choir: false,
    },
    Mode {
        name: "Sparkle",
        diffusers: &[(23.0, 0.7), (37.0, 0.7), (59.0, 0.65)],
        bandwidth_hz: 20_000.0,
        size: 0.5,
        tank_diffusion: [0.72, 0.62],
        wobble_ms: 0.12,
        wobble_hz: 2.3,
        taps: [0.25, 0.25, 1.5, 0.25, 0.25, 1.5, 0.25],
        late: 1.25,
        early: 0.0,
        early_spread: 1.0,
        low_cut_hz: 350.0,
        choir: false,
    },
    Mode {
        name: "Ambience",
        diffusers: &[(142.0, 0.7), (107.0, 0.7), (277.0, 0.6)],
        bandwidth_hz: 11_000.0,
        size: 0.42,
        tank_diffusion: [0.6, 0.5],
        wobble_ms: 0.3,
        wobble_hz: 0.8,
        taps: [0.7, 0.7, 1.0, 0.7, 1.2, 1.0, 1.2],
        late: 0.69,
        early: 0.8,
        early_spread: 0.6,
        low_cut_hz: 60.0,
        choir: false,
    },
    Mode {
        name: "Aura",
        diffusers: &[],
        bandwidth_hz: 10_000.0,
        size: 0.3,
        tank_diffusion: [0.75, 0.6],
        wobble_ms: 0.7,
        wobble_hz: 0.5,
        taps: [0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0],
        late: 0.95,
        early: 0.0,
        early_spread: 1.0,
        low_cut_hz: 30.0,
        choir: false,
    },
    Mode {
        name: "Choir",
        diffusers: &[(142.0, 0.75), (107.0, 0.75), (379.0, 0.625), (277.0, 0.625)],
        bandwidth_hz: 10_000.0,
        size: 1.15,
        tank_diffusion: [0.72, 0.55],
        wobble_ms: 0.7,
        wobble_hz: 0.4,
        taps: EVEN,
        late: 1.0,
        early: 0.0,
        early_spread: 1.0,
        low_cut_hz: 70.0,
        choir: true,
    },
];

pub const MODE_COUNT: usize = MODES.len();
pub const MODE_NAMES: [&str; MODE_COUNT] = {
    let mut names = [""; MODE_COUNT];
    let mut i = 0;
    while i < MODE_COUNT {
        names[i] = MODES[i].name;
        i += 1;
    }
    names
};

pub const MAX_DIFFUSERS: usize = 6;

/// The largest any mode makes the structure.
pub fn largest_size() -> f32 {
    MODES.iter().map(|m| m.size).fold(0.0, f32::max)
}

pub fn longest_diffuser() -> f32 {
    MODES.iter().flat_map(|m| m.diffusers.iter().map(|d| d.0)).fold(0.0, f32::max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn taps_sit_inside_their_lines() {
        for (output, taps) in TAPS.iter().enumerate() {
            for t in taps {
                let branch = if t.side == Near { output } else { 1 - output };
                let line = TANK[branch][match t.section {
                    Delay1 => 1,
                    Allpass2 => 2,
                    Delay2 => 3,
                }];
                assert!(t.at < line, "{t:?} on a line of {line}");
            }
        }
    }

    #[test]
    fn no_mode_overflows_the_diffusers() {
        assert!(MODES.iter().all(|m| m.diffusers.len() <= MAX_DIFFUSERS));
    }
}
