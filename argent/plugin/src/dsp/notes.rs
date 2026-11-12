//! Four latched notes, none of which ever jumps. A note fades in when pressed and stays:
//! letting go of the key doesn't end it. Pressing it again fades it out, and again brings
//! it back from wherever it has got to. Release fades out every note whose key isn't
//! down. A fifth note takes over a voice already playing: that voice slides to the new
//! pitch over the fade time, and fades (back) in from its current level.
//!
//! Each note has one position moving toward 0 or 1 at the fade rate, and its level is
//! that position raised to the curve, so fading in and out follow the same curve and any
//! change of mind simply turns it round where it is.

pub const NOTES: usize = 4;
/// Each harmonic voice's curve is this much steeper than the one below it, so a note
/// blooms upward as it arrives and recedes from the top as it goes.
const HARMONIC_LAG: f32 = 0.6;

#[derive(Clone, Copy, Default, Debug)]
pub struct Note {
    /// The key it's playing, or sliding toward.
    pub key: u8,
    pub sounding: bool,
    /// The key is down on the controller.
    held: bool,
    target: f32,
    pub position: f32,
    /// When it last took a key: the smallest is the oldest.
    age: u64,
    /// Where its pitch is now, in semitones (MIDI note numbers, fractional).
    pub pitch: f32,
    /// The length of the current slide, in semitones, so that it takes one fade time.
    slide: f32,
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

    fn tick(&mut self) -> u64 {
        self.clock += 1;
        self.clock
    }

    /// A key goes down. Returns the slot if a note starts there from silence, so its
    /// oscillators can begin afresh; a voice that slides keeps running.
    pub fn press(&mut self, key: u8) -> Option<usize> {
        // The same key again: it turns round, out if it was coming in, in if going out.
        if let Some(n) = self.notes.iter_mut().find(|n| n.sounding && n.key == key) {
            n.target = if n.target > 0.0 { 0.0 } else { 1.0 };
            n.held = true;
            return None;
        }
        let age = self.tick();
        if let Some(slot) = self.notes.iter().position(|n| !n.sounding) {
            let pitch = key as f32;
            self.notes[slot] = Note { key, sounding: true, held: true, target: 1.0, position: 0.0, age, pitch, slide: 0.0 };
            return Some(slot);
        }
        // All four are playing: take a voice that's on its way out if there is one,
        // else the oldest, and slide it.
        let oldest = |going: bool| {
            (0..NOTES).filter(|&i| (self.notes[i].target == 0.0) == going).min_by_key(|&i| self.notes[i].age)
        };
        let slot = oldest(true).or_else(|| oldest(false)).unwrap_or(0);
        let n = &mut self.notes[slot];
        n.slide = (key as f32 - n.pitch).abs();
        n.key = key;
        n.target = 1.0;
        n.held = true;
        n.age = age;
        None
    }

    /// A key comes up. The note carries on.
    pub fn lift(&mut self, key: u8) {
        for n in self.notes.iter_mut().filter(|n| n.key == key) {
            n.held = false;
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

    /// One sample on. `fade_step` is 1 / (fade time x sample rate).
    #[inline]
    pub fn step(&mut self, fade_step: f32) {
        for n in self.notes.iter_mut().filter(|n| n.sounding) {
            if n.position < n.target {
                n.position = (n.position + fade_step).min(n.target);
            } else if n.position > n.target {
                n.position = (n.position - fade_step).max(n.target);
            }
            let to = n.key as f32;
            let by = n.slide * fade_step;
            if n.pitch < to {
                n.pitch = (n.pitch + by).min(to);
            } else if n.pitch > to {
                n.pitch = (n.pitch - by).max(to);
            }
            if n.target == 0.0 && n.position <= 0.0 {
                n.sounding = false;
            }
        }
    }

    /// The level of harmonic voice `k` (0 is the note itself) of the note in `slot`.
    #[inline(always)]
    pub fn level(&self, slot: usize, curve: f32, k: usize) -> f32 {
        let n = &self.notes[slot];
        if !n.sounding {
            return 0.0;
        }
        n.position.powf(curve * (1.0 + HARMONIC_LAG * k as f32))
    }

    /// The slot of the newest note sounding, for the panel.
    pub fn newest(&self) -> Option<usize> {
        (0..NOTES).filter(|&i| self.notes[i].sounding).max_by_key(|&i| self.notes[i].age)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FADE_STEP: f32 = 0.01;

    fn settle(notes: &mut Notes, samples: usize) {
        for _ in 0..samples {
            notes.step(FADE_STEP);
        }
    }

    fn keys(notes: &Notes) -> Vec<u8> {
        let mut keys: Vec<u8> = (0..NOTES).filter(|&i| notes.get(i).sounding).map(|i| notes.get(i).key).collect();
        keys.sort();
        keys
    }

    fn slot_of(notes: &Notes, key: u8) -> usize {
        (0..NOTES).find(|&i| notes.get(i).sounding && notes.get(i).key == key).unwrap()
    }

    #[test]
    fn notes_latch_when_their_keys_are_let_go() {
        let mut notes = Notes::default();
        notes.press(60);
        notes.lift(60);
        settle(&mut notes, 500);
        assert_eq!(keys(&notes), vec![60]);
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
        assert_eq!(keys(&notes), vec![64]);
    }

    #[test]
    fn pressing_a_note_again_fades_it_and_again_brings_it_back_from_where_it_is() {
        let mut notes = Notes::default();
        notes.press(60);
        notes.lift(60);
        settle(&mut notes, 200);
        notes.press(60);
        notes.lift(60);
        settle(&mut notes, 30);
        let midway = notes.get(0).position;
        assert!(midway > 0.6 && midway < 0.8, "{midway}");
        notes.press(60);
        notes.step(FADE_STEP);
        assert!((notes.get(0).position - (midway + FADE_STEP)).abs() < 1e-6);
        settle(&mut notes, 200);
        assert_eq!(notes.get(0).position, 1.0);
    }

    #[test]
    fn a_note_pressed_twice_while_rising_turns_round_too() {
        let mut notes = Notes::default();
        notes.press(60);
        settle(&mut notes, 40);
        notes.press(60);
        settle(&mut notes, 10);
        assert!((notes.get(0).position - 0.3).abs() < 1e-5);
        settle(&mut notes, 100);
        assert!(keys(&notes).is_empty());
    }

    #[test]
    fn a_fifth_note_slides_the_oldest_voice_over_the_fade_time() {
        let mut notes = Notes::default();
        for key in [48, 52, 55, 59] {
            notes.press(key);
            notes.step(FADE_STEP);
        }
        settle(&mut notes, 200);
        assert_eq!(notes.press(62), None, "no voice starts from silence");
        let slot = slot_of(&notes, 62);
        assert_eq!(slot, 0, "the oldest voice slides");
        assert_eq!(notes.get(slot).position, 1.0, "and its level doesn't move");
        let mut last = notes.get(slot).pitch;
        assert_eq!(last, 48.0);
        for _ in 0..50 {
            notes.step(FADE_STEP);
            let pitch = notes.get(slot).pitch;
            assert!(pitch > last && pitch - last < 0.15, "{last} -> {pitch}");
            last = pitch;
        }
        assert!((last - 55.0).abs() < 1e-3, "halfway through the fade, halfway there: {last}");
        settle(&mut notes, 60);
        assert_eq!(notes.get(slot).pitch, 62.0);
        assert_eq!(keys(&notes), vec![52, 55, 59, 62]);
    }

    #[test]
    fn a_voice_on_its_way_out_is_taken_first_and_fades_back_in_from_where_it_is() {
        let mut notes = Notes::default();
        for key in [48, 52, 55, 59] {
            notes.press(key);
        }
        for key in [48, 52, 55, 59] {
            notes.lift(key);
        }
        settle(&mut notes, 200);
        notes.press(55);
        notes.lift(55);
        settle(&mut notes, 40);
        let before = notes.get(slot_of(&notes, 55)).position;
        notes.press(64);
        let slot = slot_of(&notes, 64);
        assert_eq!(notes.get(slot).pitch, 55.0, "the fading voice was taken, not the oldest");
        notes.step(FADE_STEP);
        assert!(notes.get(slot).position > before);
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
        assert!(keys(&notes).is_empty());
        assert_eq!(notes.press(61), Some(0));
        assert_eq!(notes.get(0).pitch, 61.0);
    }
}
