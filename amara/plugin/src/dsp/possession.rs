//! Now and then something else takes over the voice. It arrives without warning and leaves
//! the same way, while the language underneath keeps going.

use super::codec::{Frame, ORDER};
use super::random::Rng;
use super::speaker::Target;

/// Frames kept for playing back: about eight seconds at the usual frame rate.
const HISTORY: usize = 512;
const SLOWEST_GAP_S: f32 = 60.0;
const FASTEST_RATIO: f32 = 0.03;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Kind {
    /// A throat the size of a well, and the pitch to go with it.
    Giant,
    /// Tiny and very high.
    Mite,
    /// Pulses so slow you can count them.
    Fry,
    /// Several of it at once, an octave and a fifth down.
    Legion,
    /// The resonances narrow until they ring.
    Whistle,
    /// The mouth stops, and the voice carries on behind it.
    Stuck,
    /// What it just said, backwards.
    Reverse,
    /// The last few frames, over and over.
    Stutter,
    /// The line goes to pieces.
    Scramble,
    /// The pitch climbs out of the range of voices.
    Howl,
}
use Kind::*;

const KINDS: [Kind; 10] = [Giant, Mite, Fry, Legion, Whistle, Stuck, Reverse, Stutter, Scramble, Howl];

pub struct Possession {
    rng: Rng,
    active: Option<Kind>,
    last: Kind,
    left: f32,
    length: f32,
    until_next: f32,
    extremity: f32,
    frequency: f32,
    seized: bool,
    history: Box<[Frame; HISTORY]>,
    head: usize,
    /// Frames back from the head that playback has reached.
    reach: usize,
    loop_length: usize,
    held: [f32; ORDER],
}

impl Possession {
    pub fn new() -> Self {
        Self {
            rng: Rng::new(0x666),
            active: None,
            last: Howl,
            left: 0.0,
            length: 1.0,
            until_next: 8.0,
            extremity: 0.6,
            frequency: 0.0,
            seized: false,
            history: Box::new([Frame::SILENT; HISTORY]),
            head: 0,
            reach: 0,
            loop_length: 4,
            held: [0.0; ORDER],
        }
    }

    /// `frequency` 0 never, 1 nearly always. `seized` forces it to keep happening.
    pub fn set_knobs(&mut self, frequency: f32, extremity: f32, seized: bool) {
        self.frequency = frequency;
        self.extremity = extremity;
        self.seized = seized;
    }

    pub fn active(&self) -> Option<Kind> {
        self.active
    }

    fn mean_gap(&self) -> f32 {
        SLOWEST_GAP_S * FASTEST_RATIO.powf(self.frequency)
    }

    fn begin(&mut self) {
        let kind = loop {
            let pick = KINDS[self.rng.below(KINDS.len())];
            if pick != self.last {
                break pick;
            }
        };
        self.length = self.rng.range(0.2, 0.7 + 2.3 * self.extremity);
        self.left = self.length;
        self.active = Some(kind);
        self.last = kind;
        self.reach = 0;
        self.loop_length = 2 + self.rng.below(10);
        let newest = self.history[(self.head + HISTORY - 1) % HISTORY];
        self.held = newest.ratios;
    }

    /// Once per frame, before the codec sees the tract.
    pub fn advance(&mut self, dt: f32) {
        if self.active.is_some() {
            self.left -= dt;
            if self.left <= 0.0 {
                self.active = None;
                self.until_next = self.rng.exponential(self.mean_gap());
                if self.seized {
                    self.begin();
                }
            }
        } else if self.seized {
            self.begin();
        } else if self.frequency > 0.0 {
            self.until_next -= dt;
            if self.until_next <= 0.0 {
                self.begin();
            } else {
                self.until_next = self.until_next.min(self.mean_gap() * 6.0);
            }
        }
    }

    /// Changes to the tract and source, before coding.
    pub fn shape(&self, target: &mut Target) {
        let x = self.extremity;
        let progress = 1.0 - (self.left / self.length.max(1e-3)).clamp(0.0, 1.0);
        let scale_tract = |target: &mut Target, by: f32| target.formants.iter_mut().for_each(|f| *f *= by);
        match self.active {
            Some(Giant) => {
                scale_tract(target, 1.0 - 0.52 * x);
                target.f0 *= 1.0 - 0.6 * x;
            }
            Some(Mite) => {
                scale_tract(target, 1.0 + 0.9 * x);
                target.f0 *= 1.0 + 2.4 * x;
            }
            Some(Fry) => {
                target.f0 = target.f0 + (24.0 - target.f0) * x;
                target.voicing = target.voicing.max(0.9);
                target.rough = target.rough.max(0.5 * x);
            }
            Some(Howl) => {
                target.f0 *= 2f32.powf(4.0 * x * progress);
                target.voicing = target.voicing.max(0.7);
                target.gain = target.gain.max(0.12);
            }
            _ => {}
        }
    }

    /// How much worse it makes the line.
    pub fn line_damage(&self) -> f32 {
        if self.active == Some(Scramble) { 0.5 + 0.5 * self.extremity } else { 0.0 }
    }

    /// Changes to what came out of the decoder, which it also remembers.
    pub fn after_decoding(&mut self, decoded: Frame) -> Frame {
        self.history[self.head] = decoded;
        self.head = (self.head + 1) % HISTORY;
        let newest_back = |reach: usize, head: usize| (head + 2 * HISTORY - 1 - reach.min(HISTORY - 1)) % HISTORY;
        match self.active {
            Some(Whistle) => {
                let narrowed = 1.0 + 0.9 * self.extremity;
                Frame { ratios: decoded.ratios.map(|r| r * narrowed), ..decoded }
            }
            Some(Stuck) => Frame { ratios: self.held, ..decoded },
            Some(Reverse) => {
                // Backwards at speaking rate: two steps back per frame, as time moves on too.
                self.reach = (self.reach + 2).min(HISTORY - 1);
                self.history[newest_back(self.reach, self.head)]
            }
            Some(Stutter) => {
                // Holding still against a head that moves means one more step each frame.
                self.reach += 1;
                if self.reach > self.loop_length {
                    self.reach = 1;
                }
                self.history[newest_back(self.reach * 2, self.head)]
            }
            _ => decoded,
        }
    }

    /// Extra voices below the main one, as pitch ratios for left and right.
    pub fn legion(&self) -> Option<[f32; 2]> {
        (self.active == Some(Legion)).then_some([0.5, 0.748])
    }
}

impl Default for Possession {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FRAME: f32 = 0.015;

    #[test]
    fn it_never_happens_at_zero_and_often_at_full() {
        let count = |frequency: f32| {
            let mut possession = Possession::new();
            possession.set_knobs(frequency, 0.6, false);
            let mut events = 0;
            let mut was = false;
            for _ in 0..(600.0 / FRAME) as usize {
                possession.advance(FRAME);
                let now = possession.active().is_some();
                if now && !was {
                    events += 1;
                }
                was = now;
            }
            events
        };
        assert_eq!(count(0.0), 0);
        let (some, many) = (count(0.35), count(1.0));
        assert!((10..120).contains(&some), "{some}");
        assert!(many > some * 2, "{some} {many}");
    }

    #[test]
    fn seizing_keeps_it_going_and_every_kind_turns_up() {
        let mut possession = Possession::new();
        possession.set_knobs(0.0, 1.0, true);
        let mut seen = Vec::new();
        for _ in 0..(600.0 / FRAME) as usize {
            possession.advance(FRAME);
            let kind = possession.active();
            assert!(kind.is_some());
            if !seen.contains(&kind) {
                seen.push(kind);
            }
        }
        assert_eq!(seen.len(), KINDS.len());
    }

    #[test]
    fn reverse_plays_what_it_heard_in_the_opposite_order() {
        let mut possession = Possession::new();
        let frame = |f0: f32| Frame { f0, ..Frame::SILENT };
        for i in 0..100 {
            possession.after_decoding(frame(100.0 + i as f32));
        }
        possession.active = Some(Reverse);
        let heard: Vec<f32> = (100..110).map(|i| possession.after_decoding(frame(100.0 + i as f32)).f0).collect();
        assert!(heard.windows(2).all(|w| w[1] < w[0]), "{heard:?}");
    }

    #[test]
    fn stuck_holds_the_filter_and_lets_the_rest_through() {
        let mut possession = Possession::new();
        possession.after_decoding(Frame { ratios: [1.0; ORDER], ..Frame::SILENT });
        possession.set_knobs(0.0, 1.0, false);
        possession.begin();
        possession.active = Some(Stuck);
        let out = possession.after_decoding(Frame { ratios: [2.0; ORDER], f0: 300.0, ..Frame::SILENT });
        assert_eq!(out.ratios, [1.0; ORDER]);
        assert_eq!(out.f0, 300.0);
    }
}
