//! Four latched notes. A note fades in when pressed and stays: letting go of the key
//! doesn't end it. Release fades out every note whose key isn't down. A fifth note cuts
//! the oldest outright, with a few milliseconds of fade so it doesn't click.
//!
//! Each note has one position moving toward 0 or 1 at the fade rate, and its level is
//! that position raised to the curve. So fading in and out follow the same curve, and a
//! note pressed again while fading out simply turns round where it is.

pub const NOTES: usize = 4;
/// How long a stolen note takes to go.
pub const CUT_S: f32 = 0.006;
/// Each harmonic voice's curve is this much steeper than the one below it, so a note
/// blooms upward as it arrives and recedes from the top as it goes.
const HARMONIC_LAG: f32 = 0.6;

#[derive(Clone, Copy, Default, Debug)]
pub struct Note {
    pub key: u8,
    pub sounding: bool,
    /// The key is down on the controller.
    held: bool,
    target: f32,
    pub position: f32,
    /// When it began: the smallest is the oldest.
    age: u64,
    /// 1, unless it's being cut to make room.
    cut: f32,
    /// The note waiting for this slot once the cut is done, and whether its key is down.
    pending: Option<(u8, bool)>,
}

#[derive(Default)]
pub struct Notes {
    notes: [Note; NOTES],
    clock: u64,
}

impl Notes {
    pub fn get(&self, slot: usize) -> &Note {
        &self.notes[slot]
    }

    fn start(&mut self, slot: usize, key: u8, held: bool) {
        self.clock += 1;
        self.notes[slot] =
            Note { key, sounding: true, held, target: 1.0, position: 0.0, age: self.clock, cut: 1.0, pending: None };
    }

    /// A key goes down. Returns the slot if a new note starts in it at once.
    pub fn press(&mut self, key: u8) -> Option<usize> {
        if let Some(n) = self.notes.iter_mut().find(|n| n.pending.map(|(k, _)| k) == Some(key)) {
            n.pending = Some((key, true));
            return None;
        }
        if let Some(n) = self.notes.iter_mut().find(|n| n.sounding && n.pending.is_none() && n.key == key) {
            n.target = 1.0;
            n.held = true;
            return None;
        }
        if let Some(slot) = self.notes.iter().position(|n| !n.sounding) {
            self.start(slot, key, true);
            return Some(slot);
        }
        let oldest = (0..NOTES).filter(|&i| self.notes[i].pending.is_none()).min_by_key(|&i| self.notes[i].age);
        let slot = oldest.unwrap_or_else(|| (0..NOTES).min_by_key(|&i| self.notes[i].age).unwrap_or(0));
        self.notes[slot].pending = Some((key, true));
        None
    }

    /// A key comes up. The note carries on.
    pub fn lift(&mut self, key: u8) {
        for n in &mut self.notes {
            if n.key == key && n.pending.is_none() {
                n.held = false;
            }
            if let Some((k, _)) = n.pending {
                if k == key {
                    n.pending = Some((k, false));
                }
            }
        }
    }

    /// Every note whose key isn't down begins to fade out.
    pub fn release(&mut self) {
        for n in &mut self.notes {
            if n.sounding && !n.held {
                n.target = 0.0;
            }
        }
    }

    /// One sample on. `fade_step` is 1 / (fade time x sample rate). Returns the slots in
    /// which a new note began, after a cut.
    #[inline]
    pub fn step(&mut self, fade_step: f32, cut_step: f32) -> [bool; NOTES] {
        let mut started = [false; NOTES];
        for slot in 0..NOTES {
            let n = &mut self.notes[slot];
            if let Some((key, held)) = n.pending {
                n.cut -= cut_step;
                if n.cut <= 0.0 {
                    self.start(slot, key, held);
                    started[slot] = true;
                }
                continue;
            }
            if !n.sounding {
                continue;
            }
            if n.position < n.target {
                n.position = (n.position + fade_step).min(n.target);
            } else if n.position > n.target {
                n.position = (n.position - fade_step).max(n.target);
            }
            if n.target == 0.0 && n.position <= 0.0 {
                n.sounding = false;
            }
        }
        started
    }

    /// The level of harmonic voice `k` (0 is the note itself) of the note in `slot`.
    #[inline(always)]
    pub fn level(&self, slot: usize, curve: f32, k: usize) -> f32 {
        let n = &self.notes[slot];
        if !n.sounding {
            return 0.0;
        }
        n.position.powf(curve * (1.0 + HARMONIC_LAG * k as f32)) * n.cut
    }

    /// The slot of the newest note sounding, for the panel.
    pub fn newest(&self) -> Option<usize> {
        (0..NOTES).filter(|&i| self.notes[i].sounding).max_by_key(|&i| self.notes[i].age)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn settle(notes: &mut Notes, samples: usize) {
        for _ in 0..samples {
            notes.step(0.01, 0.25);
        }
    }

    fn sounding(notes: &Notes) -> Vec<u8> {
        let mut keys: Vec<u8> = (0..NOTES).filter(|&i| notes.get(i).sounding).map(|i| notes.get(i).key).collect();
        keys.sort();
        keys
    }

    #[test]
    fn notes_latch_when_their_keys_are_let_go() {
        let mut notes = Notes::default();
        notes.press(60);
        notes.lift(60);
        settle(&mut notes, 500);
        assert_eq!(sounding(&notes), vec![60]);
        assert_eq!(notes.get(0).position, 1.0);
    }

    #[test]
    fn release_spares_notes_whose_keys_are_down() {
        let mut notes = Notes::default();
        notes.press(60);
        notes.press(64);
        notes.lift(60);
        settle(&mut notes, 200);
        notes.release();
        settle(&mut notes, 200);
        assert_eq!(sounding(&notes), vec![64]);
    }

    #[test]
    fn a_fading_note_pressed_again_turns_round_where_it_is() {
        let mut notes = Notes::default();
        notes.press(60);
        notes.lift(60);
        settle(&mut notes, 200);
        notes.release();
        settle(&mut notes, 30);
        let midway = notes.get(0).position;
        assert!(midway > 0.6 && midway < 0.8, "{midway}");
        notes.press(60);
        notes.step(0.01, 0.25);
        assert!(notes.get(0).position > midway);
    }

    #[test]
    fn a_fifth_note_cuts_the_oldest_after_a_short_fade() {
        let mut notes = Notes::default();
        for key in [48, 52, 55, 59] {
            notes.press(key);
            notes.step(0.01, 0.25);
        }
        assert_eq!(notes.press(62), None);
        assert_eq!(sounding(&notes), vec![48, 52, 55, 59]);
        let started = (0..4).map(|_| notes.step(0.01, 0.25)).fold([false; NOTES], |a, b| std::array::from_fn(|i| a[i] | b[i]));
        assert!(started[0]);
        assert_eq!(sounding(&notes), vec![52, 55, 59, 62]);
    }

    #[test]
    fn harmonics_arrive_after_their_note_and_leave_before_it() {
        let mut notes = Notes::default();
        notes.press(60);
        settle(&mut notes, 40);
        let (note, harmonic) = (notes.level(0, 2.0, 0), notes.level(0, 2.0, 3));
        assert!(harmonic < note * 0.5, "{note} {harmonic}");
        settle(&mut notes, 200);
        assert_eq!(notes.level(0, 2.0, 3), 1.0);
    }

    #[test]
    fn a_note_that_has_faded_away_frees_its_slot() {
        let mut notes = Notes::default();
        notes.press(60);
        notes.lift(60);
        settle(&mut notes, 200);
        notes.release();
        settle(&mut notes, 200);
        assert!(sounding(&notes).is_empty());
        assert_eq!(notes.press(61), Some(0));
    }
}
