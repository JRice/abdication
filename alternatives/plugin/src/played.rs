//! The notes Alternatives puts its bumps on: those held, and those let go since
//! the last note started (their tails are still sounding). A new note clears
//! out the ones let go, so a legato line moves its bumps along with it.

use crate::notes::{self, Source};

pub const MAX_NOTES: usize = 8;

#[derive(Default)]
pub struct Played {
    held: Vec<u8>,
    /// The current set: the held notes, or the last held ones.
    set: Vec<u8>,
    /// Whether any note has arrived (MIDI uses Root until one has).
    heard: bool,
}

impl Played {
    /// A note starts: it joins those held, and the ones let go leave.
    pub fn note_on(&mut self, note: u8) {
        self.heard = true;
        let held = &self.held;
        self.set.retain(|n| held.contains(n));
        if !self.held.contains(&note) {
            self.held.push(note);
        }
        if !self.set.contains(&note) {
            if self.set.len() == MAX_NOTES {
                self.set.remove(0);
            }
            self.set.push(note);
        }
    }

    /// A note ends: it stays in the set (its tail is still sounding) until
    /// the next note starts.
    pub fn note_off(&mut self, note: u8) {
        self.held.retain(|n| *n != note);
    }

    /// Forget everything (the plugin was reset).
    pub fn clear(&mut self) {
        *self = Self::default();
    }

    /// The notes in play, in Hz (at least one, at most MAX_NOTES).
    pub fn hz(&self, source: Source, root: i32) -> Vec<f32> {
        if source == Source::Root || !self.heard || self.set.is_empty() {
            vec![notes::hz(root as f32)]
        } else {
            self.set.iter().map(|n| notes::hz(*n as f32)).collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn midi(p: &Played) -> Vec<f32> {
        p.hz(Source::Midi, 48)
    }

    #[test]
    fn root_until_a_note_arrives() {
        let mut p = Played::default();
        assert_eq!(midi(&p), vec![notes::hz(48.0)]);
        p.note_on(60);
        assert_eq!(midi(&p), vec![notes::hz(60.0)]);
        assert_eq!(p.hz(Source::Root, 50), vec![notes::hz(50.0)]);
    }

    #[test]
    fn a_chord_stays_after_its_release_until_the_next_note() {
        let mut p = Played::default();
        for n in [60, 64, 67] {
            p.note_on(n);
        }
        p.note_off(64);
        p.note_off(60);
        p.note_off(67);
        assert_eq!(p.set, vec![60, 64, 67], "the chord stays as it rings out");
        p.note_on(62);
        assert_eq!(p.set, vec![62], "a new note clears out the ones let go");
    }

    #[test]
    fn a_legato_line_moves_along() {
        let mut p = Played::default();
        p.note_on(60);
        p.note_on(62);
        assert_eq!(p.set, vec![60, 62], "both held");
        p.note_off(60);
        p.note_on(64);
        assert_eq!(p.set, vec![62, 64]);
    }

    #[test]
    fn at_most_eight_the_newest() {
        let mut p = Played::default();
        for n in 50..60 {
            p.note_on(n);
        }
        assert_eq!(p.set, (52..60).collect::<Vec<u8>>());
        assert_eq!(midi(&p).len(), MAX_NOTES);
    }
}
