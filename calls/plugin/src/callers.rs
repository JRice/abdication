//! Which caller plays which note. Each held MIDI note gets a caller of its own
//! (as each of Abdication's voices had its calls); a note let go lets its
//! caller fade, and the next note takes the caller that has been quiet
//! longest. With Root (or MIDI before any note arrives), caller 0 plays Root.

use crate::notes::{self, Source};

pub const MAX_CALLERS: usize = 8;

#[derive(Default)]
pub struct Callers {
    /// The note each caller plays now (None: let go, fading or silent)...
    held: [Option<u8>; MAX_CALLERS],
    /// ...and the last one it played, which it keeps while it fades.
    last: [Option<u8>; MAX_CALLERS],
    /// When each last changed, to find the longest quiet or oldest held.
    since: [u64; MAX_CALLERS],
    clock: u64,
    heard: bool,
}

impl Callers {
    pub fn note_on(&mut self, note: u8) {
        self.heard = true;
        if self.held.contains(&Some(note)) {
            return;
        }
        // The caller quiet longest, else (all held) the one held longest.
        let k = (0..MAX_CALLERS)
            .min_by_key(|&k| (self.held[k].is_some(), self.since[k]))
            .unwrap_or(0);
        self.held[k] = Some(note);
        self.last[k] = Some(note);
        self.touch(k);
    }

    pub fn note_off(&mut self, note: u8) {
        if let Some(k) = self.held.iter().position(|h| *h == Some(note)) {
            self.held[k] = None;
            self.touch(k);
        }
    }

    pub fn clear(&mut self) {
        *self = Self::default();
    }

    fn touch(&mut self, k: usize) {
        self.clock += 1;
        self.since[k] = self.clock;
    }

    /// Each caller's pitch in Hz and whether it's sounding (its gate).
    pub fn gates(&self, source: Source, root: i32) -> [(f32, bool); MAX_CALLERS] {
        let root_hz = notes::hz(root as f32);
        if source == Source::Root || !self.heard {
            return std::array::from_fn(|k| (root_hz, k == 0));
        }
        std::array::from_fn(|k| (self.last[k].map_or(root_hz, |n| notes::hz(n as f32)), self.held[k].is_some()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sounding(c: &Callers) -> Vec<(usize, f32)> {
        c.gates(Source::Midi, 48).iter().enumerate().filter(|(_, g)| g.1).map(|(k, g)| (k, g.0)).collect()
    }

    #[test]
    fn root_until_a_note_arrives() {
        let mut c = Callers::default();
        assert_eq!(sounding(&c), vec![(0, notes::hz(48.0))]);
        c.note_on(60);
        assert_eq!(sounding(&c).len(), 1);
        assert_eq!(sounding(&c)[0].1, notes::hz(60.0));
        assert_eq!(c.gates(Source::Root, 50)[0], (notes::hz(50.0), true), "Root ignores the notes");
    }

    #[test]
    fn a_note_each_and_the_quietest_is_reused() {
        let mut c = Callers::default();
        for n in [60, 64, 67] {
            c.note_on(n);
        }
        let callers: Vec<usize> = sounding(&c).iter().map(|s| s.0).collect();
        assert_eq!(callers.len(), 3, "a caller each");
        c.note_off(64);
        assert_eq!(sounding(&c).len(), 2);
        let faded = (0..MAX_CALLERS).find(|&k| c.last[k] == Some(64)).unwrap();
        // The new note goes to a caller that never played, not the one fading.
        c.note_on(70);
        assert_eq!(c.last[faded], Some(64), "the fading caller keeps its note");
        // A fading caller keeps its pitch, gate off.
        assert_eq!(c.gates(Source::Midi, 48)[faded], (notes::hz(64.0), false));
    }

    #[test]
    fn more_notes_than_callers_take_the_oldest() {
        let mut c = Callers::default();
        for n in 50..60 {
            c.note_on(n);
        }
        let notes: Vec<u8> = c.held.iter().flatten().copied().collect();
        assert_eq!(notes.len(), MAX_CALLERS);
        assert!(!notes.contains(&50) && !notes.contains(&51) && notes.contains(&59));
    }

    #[test]
    fn the_same_note_twice_is_one_caller() {
        let mut c = Callers::default();
        c.note_on(60);
        c.note_on(60);
        assert_eq!(sounding(&c).len(), 1);
    }
}
