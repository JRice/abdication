//! Notes: their names (Bitwig's: middle C, MIDI 60, is C3), their pitch, the
//! Root parameter, and where the pitch comes from.

use std::sync::Arc;

use nih_plug::prelude::*;

/// Where the notes come from: the Root knob, or MIDI. MIDI uses Root until
/// the first note arrives (so with nothing routed to it, it plays Root).
#[derive(Enum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Source {
    Root,
    #[name = "MIDI"]
    Midi,
}

const NAMES: [&str; 12] = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];

pub fn name(note: i32) -> String {
    format!("{}{}", NAMES[note.rem_euclid(12) as usize], note.div_euclid(12) - 2)
}

/// A note name ("C2", "f#3", "Bb1") or a MIDI number.
pub fn parse(text: &str) -> Option<i32> {
    let text = text.trim();
    if let Ok(number) = text.parse() {
        return Some(number);
    }
    let mut chars = text.chars();
    let letter = chars.next()?.to_ascii_uppercase();
    let base = NAMES.iter().position(|n| n.starts_with(letter) && n.len() == 1)? as i32;
    let rest: String = chars.collect();
    let (shift, octave) = match rest.chars().next() {
        Some('#') => (1, &rest[1..]),
        Some('b') => (-1, &rest[1..]),
        _ => (0, &rest[..]),
    };
    Some(base + shift + 12 * (octave.parse::<i32>().ok()? + 2))
}

pub fn hz(note: f32) -> f32 {
    440.0 * 2f32.powf((note - 69.0) / 12.0)
}

/// The Root parameter: the note used when no MIDI note is.
pub fn root_param(default: i32) -> IntParam {
    IntParam::new("Root", default, IntRange::Linear { min: 12, max: 108 })
        .with_value_to_string(Arc::new(|n| name(n)))
        .with_string_to_value(Arc::new(parse))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_round_trip() {
        assert_eq!(name(60), "C3");
        assert_eq!(name(48), "C2");
        assert_eq!(parse("A3"), Some(69));
        assert_eq!(parse("c#2"), Some(49));
        assert_eq!(parse("Bb1"), Some(46));
        assert_eq!(parse("57"), Some(57));
        for n in 12..=108 {
            assert_eq!(parse(&name(n)), Some(n));
        }
        assert!((hz(69.0) - 440.0).abs() < 1e-3);
    }
}
