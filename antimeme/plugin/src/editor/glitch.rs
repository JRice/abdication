//! The panel's misbehaviour, as plain data for the painter to apply. Two sources: the
//! limiter, whose depth sets how bad things get, and a rare, faint twitch at rest.
//!
//! Nothing here inverts or blanks more than a strip of the window, and strips are spaced
//! apart in time, so the panel never flashes as a whole.

use super::layout::{HEIGHT, TIME};

/// Limiting this deep is the worst it gets.
const FULL_DEPTH_DB: f32 = 12.0;
/// Below this the limiter is barely touching the signal: no reaction.
const MIN_DEPTH_DB: f32 = 0.05;
const RELEASE_S: f32 = 0.4;

const TWITCH_INTENSITY: f32 = 0.16;
const TWITCH_S: f32 = 0.15;

const MAX_TEARS: f32 = 7.0;
const TEAR_TRAVEL: f32 = 110.0;
const BAND_TRAVEL: f32 = 150.0;
const GHOST_TRAVEL: f32 = 26.0;

const INVERT_FROM: f32 = 0.45;
const INVERT_MAX_FRACTION: f32 = 0.15;
const INVERT_GAP_S: f32 = 0.4;

/// The numerals' cuts: height of each band, its resting offset, its idle slip, and
/// whether it's one of the thin red ones. They add up to the numerals' height.
pub const BANDS: [Band; 8] = [
    Band { height: 50.0, offset: 0.0, slip: Slip::None, red: false },
    Band { height: 18.0, offset: 0.0, slip: Slip::A, red: false },
    Band { height: 58.0, offset: 14.0, slip: Slip::None, red: false },
    Band { height: 8.0, offset: 86.0, slip: Slip::B, red: true },
    Band { height: 48.0, offset: -6.0, slip: Slip::None, red: false },
    Band { height: 22.0, offset: 30.0, slip: Slip::A, red: false },
    Band { height: 4.0, offset: -40.0, slip: Slip::B, red: true },
    Band { height: 64.0, offset: 0.0, slip: Slip::None, red: false },
];

#[derive(Clone, Copy)]
pub struct Band {
    pub height: f32,
    pub offset: f32,
    pub slip: Slip,
    pub red: bool,
}

/// Each band's occasional jump, as in the design: where in the cycle, how far.
#[derive(Clone, Copy, PartialEq)]
pub enum Slip {
    None,
    A,
    B,
}

impl Slip {
    /// The band's displacement at `t` seconds.
    pub fn at(self, t: f32) -> f32 {
        let step = |period: f32, keys: &[(f32, f32)]| {
            let phase = (t / period).fract();
            keys.iter().rev().find(|(at, _)| phase >= *at).map_or(0.0, |(_, x)| *x)
        };
        match self {
            Slip::None => 0.0,
            Slip::A => step(12.0, &[(0.72, 38.0), (0.75, -12.0), (0.78, 0.0)]),
            Slip::B => step(19.0, &[(0.45, -70.0), (0.47, 16.0), (0.50, 0.0)]),
        }
    }
}

/// A horizontal slice of the whole panel, pushed sideways.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Tear {
    pub y: f32,
    pub height: f32,
    pub shift: f32,
}

/// A row's label or value, wrong for a moment.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Corruption {
    Clean,
    /// The label replaced by unknown symbols, from this seed.
    Symbols(u32),
    /// The label borrowed from another row.
    Swapped(usize),
}

#[derive(Clone, PartialEq, Debug)]
pub struct Frame {
    pub band_shift: [f32; BANDS.len()],
    /// A red copy of the numerals: x, y, opacity.
    pub ghost: Option<(f32, f32, f32)>,
    pub tears: Vec<Tear>,
    /// A strip with its colors inverted: y, height.
    pub inverted: Option<(f32, f32)>,
    pub rows: [Corruption; TIME],
    /// One numeral replaced: which, by which digit.
    pub scramble: Option<(usize, u32)>,
}

impl Frame {
    fn clean() -> Self {
        Self { band_shift: [0.0; BANDS.len()], ghost: None, tears: Vec::new(), inverted: None, rows: [Corruption::Clean; TIME], scramble: None }
    }
}

pub struct Glitch {
    pub frame: Frame,
    intensity: f32,
    twitch_left: f32,
    next_twitch: f32,
    hold: u32,
    since_inverted: f32,
    rng: u32,
}

impl Glitch {
    pub fn new() -> Self {
        Self { frame: Frame::clean(), intensity: 0.0, twitch_left: 0.0, next_twitch: 20.0, hold: 0, since_inverted: INVERT_GAP_S, rng: 0x51ed_270b }
    }

    fn random(&mut self) -> f32 {
        self.rng ^= self.rng << 13;
        self.rng ^= self.rng >> 17;
        self.rng ^= self.rng << 5;
        self.rng as f32 / u32::MAX as f32
    }

    /// -1..1.
    fn signed(&mut self) -> f32 {
        self.random() * 2.0 - 1.0
    }

    #[cfg(test)]
    pub fn is_active(&self) -> bool {
        self.intensity > 0.0 || self.twitch_left > 0.0
    }

    /// Advance by `dt` seconds. `reduction_db` is the limiter's deepest since the last call.
    /// Returns whether the frame changed.
    pub fn update(&mut self, dt: f32, reduction_db: f32) -> bool {
        self.since_inverted += dt;
        let target = if reduction_db > MIN_DEPTH_DB { (reduction_db / FULL_DEPTH_DB).clamp(0.0, 1.0).sqrt() } else { 0.0 };
        self.intensity = target.max(self.intensity * (-dt / RELEASE_S).exp());
        if self.intensity < 0.01 {
            self.intensity = 0.0;
        }

        self.next_twitch -= dt;
        if self.next_twitch <= 0.0 {
            self.next_twitch = 18.0 + self.random() * 27.0;
            self.twitch_left = TWITCH_S;
            self.hold = 0;
        }
        self.twitch_left = (self.twitch_left - dt).max(0.0);

        let amount = self.intensity.max(if self.twitch_left > 0.0 { TWITCH_INTENSITY } else { 0.0 });
        if amount <= 0.0 {
            let was_clean = self.frame == Frame::clean();
            self.frame = Frame::clean();
            return !was_clean;
        }
        if self.hold > 0 {
            self.hold -= 1;
            return false;
        }
        self.frame = self.generate(amount);
        // It stutters: each state holds a few ticks, a strip of inverted color only one.
        self.hold = if self.frame.inverted.is_some() { 0 } else { 1 + (self.random() * 3.0) as u32 };
        true
    }

    fn generate(&mut self, amount: f32) -> Frame {
        let mut frame = Frame::clean();
        for i in 0..BANDS.len() {
            if self.random() < 0.25 + 0.6 * amount {
                frame.band_shift[i] = self.signed() * amount * BAND_TRAVEL;
            }
        }
        if amount > 0.08 {
            frame.ghost = Some((self.signed() * amount * GHOST_TRAVEL, self.signed() * amount * GHOST_TRAVEL * 0.3, 0.25 + 0.6 * amount));
        }
        let tears = (amount * MAX_TEARS) as usize;
        for _ in 0..tears {
            let y = self.random() * HEIGHT;
            let height = 6.0 + self.random() * (18.0 + 60.0 * amount);
            let shift = self.signed() * amount * TEAR_TRAVEL;
            frame.tears.push(Tear { y, height, shift });
        }
        if amount > INVERT_FROM && self.since_inverted >= INVERT_GAP_S && self.random() < 0.5 {
            let height = (24.0 + amount * 72.0).min(HEIGHT * INVERT_MAX_FRACTION);
            frame.inverted = Some((self.random() * (HEIGHT - height), height));
            self.since_inverted = 0.0;
        }
        for i in 0..TIME {
            let roll = self.random();
            if roll < amount * 0.35 {
                frame.rows[i] = Corruption::Symbols(100 + (self.random() * 900.0) as u32);
            } else if roll < amount * 0.7 {
                frame.rows[i] = Corruption::Swapped((self.random() * TIME as f32) as usize % TIME);
            }
        }
        if amount > 0.4 && self.random() < amount {
            frame.scramble = Some(((self.random() * 4.0) as usize, (self.random() * 10.0) as u32 % 10));
        }
        frame
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TICK: f32 = 1.0 / 30.0;

    /// Run for a while at a steady depth, collecting every frame produced.
    fn frames(depth: f32, seconds: f32) -> Vec<Frame> {
        let mut glitch = Glitch::new();
        glitch.next_twitch = 1e9;
        (0..(seconds / TICK) as usize)
            .map(|_| {
                glitch.update(TICK, depth);
                glitch.frame.clone()
            })
            .collect()
    }

    fn mean_tears(frames: &[Frame]) -> f32 {
        frames.iter().map(|f| f.tears.len() as f32).sum::<f32>() / frames.len() as f32
    }

    #[test]
    fn no_limiting_means_no_glitch() {
        assert!(frames(0.0, 5.0).iter().all(|f| *f == Frame::clean()));
        assert!(frames(MIN_DEPTH_DB / 2.0, 5.0).iter().all(|f| *f == Frame::clean()));
    }

    #[test]
    fn deeper_limiting_is_worse() {
        let (shallow, deep) = (frames(1.0, 10.0), frames(12.0, 10.0));
        assert!(mean_tears(&deep) > mean_tears(&shallow) + 2.0);
        let reach = |fs: &[Frame]| fs.iter().flat_map(|f| f.band_shift).fold(0.0f32, |a, b| a.max(b.abs()));
        assert!(reach(&deep) > reach(&shallow) * 1.3);
    }

    #[test]
    fn it_settles_after_the_limiter_lets_go() {
        let mut glitch = Glitch::new();
        glitch.next_twitch = 1e9;
        glitch.update(TICK, 12.0);
        for _ in 0..(5.0 / TICK) as usize {
            glitch.update(TICK, 0.0);
        }
        assert!(!glitch.is_active());
        assert_eq!(glitch.frame, Frame::clean());
    }

    #[test]
    fn inversion_stays_a_strip_and_never_strobes() {
        let all = frames(40.0, 60.0);
        let mut last = None;
        let mut count = 0;
        for (i, f) in all.iter().enumerate() {
            if let Some((_, height)) = f.inverted {
                assert!(height <= HEIGHT * INVERT_MAX_FRACTION);
                if let Some(before) = last {
                    assert!((i - before) as f32 * TICK >= INVERT_GAP_S - TICK);
                }
                last = Some(i);
                count += 1;
            }
        }
        assert!(count > 10, "only {count} strips in a minute");
        assert!(count as f32 / 60.0 <= 3.0, "{count} strips in a minute");
    }

    #[test]
    fn it_twitches_at_rest_but_only_slightly() {
        let mut glitch = Glitch::new();
        let mut twitches = 0;
        let mut tears = 0;
        for _ in 0..(120.0 / TICK) as usize {
            if glitch.update(TICK, 0.0) {
                twitches += 1;
                tears = tears.max(glitch.frame.tears.len());
            }
        }
        assert!(twitches > 0);
        assert!(tears <= 1);
    }

    #[test]
    fn idle_slips_jump_and_return() {
        assert_eq!(Slip::None.at(9.0), 0.0);
        assert_eq!(Slip::A.at(0.0), 0.0);
        assert_eq!(Slip::A.at(12.0 * 0.73), 38.0);
        assert_eq!(Slip::A.at(12.0 * 0.76), -12.0);
        assert_eq!(Slip::A.at(12.0 * 0.80), 0.0);
        assert_eq!(BANDS.iter().map(|b| b.height).sum::<f32>(), 272.0);
    }
}
