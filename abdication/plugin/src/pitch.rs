//! An oscillator slot's Pitch knob: its travel, -1..1, as an interval. Out
//! from the middle, each way: up to 99 cents (continuous, a cent per 0.4 %
//! of the travel), then 1 to 7 semitones, then 1 and 2 octaves (in steps). The same curve is in
//! dsp/voice.lib's `slotPitch`, where it plays; a test holds the two together.

/// Where each part of the travel ends (as a share of a half's travel).
pub const CENTS_END: f32 = 0.4;
pub const SEMIS_END: f32 = 0.82;
pub const OCTAVE_END: f32 = 0.91;
/// The travel each semitone step takes.
pub const SEMI_STEP: f32 = (SEMIS_END - CENTS_END) / 7.0;

/// The interval at `position`, in semitones.
pub fn semitones(position: f32) -> f32 {
    let a = position.abs().min(1.0);
    let semis = if a < CENTS_END {
        (a / CENTS_END).min(0.99)
    } else if a < SEMIS_END {
        1.0 + ((a - CENTS_END) / SEMI_STEP).floor().min(6.0)
    } else if a < OCTAVE_END {
        12.0
    } else {
        24.0
    };
    semis.copysign(position)
}

/// The middle of the knob's travel that plays `semitones` (a step, an
/// octave, or cents below a semitone; anything else rounds to the nearest
/// of those).
pub fn position(semitones: f32) -> f32 {
    let a = semitones.abs();
    let at = if a < 0.995 {
        a.min(0.99) * CENTS_END
    } else if a < 7.5 {
        CENTS_END + (a.round().clamp(1.0, 7.0) - 0.5) * SEMI_STEP
    } else if a < 18.0 {
        (SEMIS_END + OCTAVE_END) / 2.0
    } else {
        (OCTAVE_END + 1.0) / 2.0
    };
    at.copysign(semitones)
}

/// How the knob reads: "0", "+37 c", "-5 st", "+1 oct".
pub fn name(position: f32) -> String {
    let semis = semitones(position);
    let sign = if semis > 0.0 { "+" } else { "-" };
    let a = semis.abs();
    if a < 0.005 {
        "0".into()
    } else if a < 1.0 {
        format!("{sign}{:.0} c", a * 100.0)
    } else if a < 12.0 {
        format!("{sign}{a:.0} st")
    } else {
        format!("{sign}{:.0} oct", a / 12.0)
    }
}

/// A typed interval: "37 c", "-5 st", "+1 oct", or a bare number of semitones.
pub fn parse(text: &str) -> Option<f32> {
    let text = text.trim().to_ascii_lowercase();
    let number_end = text.find(|c: char| !(c.is_ascii_digit() || c == '.' || c == '-' || c == '+')).unwrap_or(text.len());
    let value: f32 = text[..number_end].trim_start_matches('+').parse().ok()?;
    let unit = text[number_end..].trim();
    let semis = match unit {
        "c" | "ct" | "cent" | "cents" => value / 100.0,
        "oct" | "octave" | "octaves" => value * 12.0,
        _ => value,
    };
    Some(position(semis))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_curve() {
        assert_eq!(semitones(0.0), 0.0);
        assert!((semitones(0.2) - 0.5).abs() < 1e-6);
        assert!((semitones(-0.3999) + 0.99).abs() < 1e-3);
        assert_eq!(semitones(0.4), 1.0);
        assert_eq!(semitones(-0.81), -7.0);
        assert_eq!(semitones(0.85), 12.0);
        assert_eq!(semitones(1.0), 24.0);
        // Out from the middle it only ever rises.
        let mut last = 0.0;
        for i in 0..=1000 {
            let s = semitones(i as f32 / 1000.0);
            assert!(s >= last);
            last = s;
        }
    }

    #[test]
    fn positions_play_their_intervals() {
        for semis in [0.0, 0.37, -0.99, 1.0, 5.0, -7.0, 12.0, -12.0, 24.0] {
            assert!((semitones(position(semis)) - semis).abs() < 1e-4, "{semis}");
        }
        assert_eq!(name(position(5.0)), "+5 st");
        assert_eq!(name(position(-12.0)), "-1 oct");
        assert_eq!(name(position(0.37)), "+37 c");
        assert_eq!(name(0.0), "0");
        assert_eq!(parse("-5 st").map(semitones), Some(-5.0));
        assert_eq!(parse("+2 oct").map(semitones), Some(24.0));
        assert!((parse("37 c").map(semitones).unwrap() - 0.37).abs() < 1e-4);
        assert_eq!(parse("7").map(semitones), Some(7.0));
    }
}
