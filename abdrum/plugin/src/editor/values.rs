//! How the panel reads and words parameter values: shorter and friendlier
//! than the host's own text ("4.2 kHz", "Off", "fast 120 ms"), plus the
//! sample sets' names for the steppers.

use crate::params::DrumParams;
use crate::samplelib::HIT_NAMES;

pub const SET_NAMES: [&str; 3] = ["Thump", "Knock", "Spark"];

/// A parameter's current plain value (0 if there's no such parameter).
pub fn plain(params: &DrumParams, id: &str) -> f32 {
    params.ptr(id).map(|p| unsafe { p.unmodulated_plain_value() }).unwrap_or(0.0)
}

/// The hits of sample set `set` (0 thump, 1 knock, 2 spark), without the
/// set prefix ("Jade 023").
pub fn set_hits(set: usize) -> Vec<&'static str> {
    let prefix = format!("{}: ", SET_NAMES[set.min(2)]);
    HIT_NAMES.iter().filter_map(|n| n.strip_prefix(prefix.as_str())).map(|n| n.trim_end_matches(" head")).collect()
}

/// A sample layer's set, and which of its hits is chosen (0-based; the
/// Sample parameter counts from 1 and wraps within the set, like the DSP).
pub fn layer_choice(params: &DrumParams, group: &str) -> (usize, usize, usize) {
    let set = plain(params, &format!("{group}.set")) as usize;
    let count = set_hits(set).len().max(1);
    let index = (plain(params, &format!("{group}.sample")) as usize).saturating_sub(1) % count;
    (set, index, count)
}

/// A440 equal temperament, MIDI numbering (69 = A 440 Hz), fractional.
pub fn note_number(hz: f32) -> f32 {
    69.0 + 12.0 * (hz.max(1e-3) / 440.0).log2()
}

pub fn note_hz(note: f32) -> f32 {
    440.0 * 2f32.powf((note - 69.0) / 12.0)
}

const NOTE_NAMES: [&str; 12] = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];

/// The nearest note's name, as Bitwig names it (note 60 is C3), and how
/// far `hz` is from it in cents (−50..50).
pub fn nearest_note(hz: f32) -> (String, f32) {
    let n = note_number(hz);
    let nearest = n.round();
    let k = nearest as i32;
    (format!("{}{}", NOTE_NAMES[k.rem_euclid(12) as usize], k.div_euclid(12) - 2), (n - nearest) * 100.0)
}

fn hz(v: f32) -> String {
    if v >= 1000.0 { format!("{:.1} kHz", v / 1000.0) } else { format!("{v:.0} Hz") }
}

pub fn ms(v: f32) -> String {
    if v >= 1000.0 { format!("{:.2} s", v / 1000.0) } else if v >= 10.0 { format!("{v:.0} ms") } else { format!("{v:.1} ms") }
}

fn semis(v: f32) -> String {
    let v = (v * 10.0).round() / 10.0;
    if v.fract() == 0.0 { format!("{v:+.0} st") } else { format!("{v:+.1} st") }
}

fn percent(v: f32) -> String {
    format!("{:.0} %", v * 100.0)
}

fn off_or(v: f32, text: impl FnOnce() -> String) -> String {
    if v <= 0.0005 { "Off".into() } else { text() }
}

/// Snap 0..1 -> the fast decay as a share of the slow one (as voice.lib).
pub fn snap_ratio(snap: f32) -> f32 {
    10f32.powf(-2.0 * snap)
}

fn shape(v: f32) -> String {
    let s = v * 3.0;
    match s {
        s if s < 0.05 => "Sine",
        s if s < 0.95 => "Sine→Tri",
        s if s < 1.05 => "Triangle",
        s if s < 2.0 => "Saturate",
        _ => "Fold",
    }
    .into()
}

fn material(v: f32) -> String {
    let names = ["Membrane", "Plate", "Bar"];
    let short = ["Memb", "Plate", "Bar"];
    let s = v * 2.0;
    let nearest = s.round() as usize;
    if (s - nearest as f32).abs() < 0.1 {
        names[nearest].into()
    } else {
        let below = (s.floor() as usize).min(1);
        format!("{}→{}", short[below], short[below + 1])
    }
}

/// The value as the panel words it.
pub fn text(params: &DrumParams, id: &str) -> String {
    let v = plain(params, id);
    let host = || params.ptr(id).map(|p| unsafe { p.normalized_value_to_string(p.unmodulated_normalized_value(), true) }).unwrap_or_default();
    let group = id.split('.').next().unwrap_or("");
    match id.split('.').nth(1).unwrap_or("") {
        _ if id == "master.volume" => host(),
        "pitch" if group == "tone" => format!("{} {}", hz(v), nearest_note(v).0),
        "bandpass" | "highpass" => hz(v),
        "drop" => format!("+{v:.0} st"),
        "split" => format!("{:.0} % fast", v * 100.0),
        "shape" => shape(v),
        "material" => material(v),
        "offset" | "pitch" | "loop_pitch" => semis(v),
        "decay" if group.starts_with("sample") && v >= 1999.0 => "Whole".into(),
        "decay" | "fast_time" | "slow_time" | "attack" => ms(v),
        // Shown as the fast decay it gives (the screen draws both).
        "snap" => ms(plain(params, &format!("{group}.decay")) * snap_ratio(v)),
        "q" => format!("{v:.2}"),
        "blend" => match v {
            v if v < 0.005 => "Generator".into(),
            v if v > 0.995 => "Loop".into(),
            v => format!("{:.0} % loop", v * 100.0),
        },
        "env_amount" => off_or(v, || format!("+{:.1} oct", 4.0 * v)),
        "threshold" => off_or(-v, || format!("{v:.0} dB")),
        "amount" | "clap" | "crush" | "body_send" => off_or(v, || percent(v)),
        "level" => level_db(v),
        "keytrack" => if v > 0.5 { "On · note 60 = Pitch".into() } else { "Off".into() },
        // "Punch 3/120 ms" -> "Punch" (the screen lists the times).
        "timing" => host().split(' ').next().unwrap_or_default().to_string(),
        "generator" | "loop" | "type" | "set" => host(),
        _ => percent(v),
    }
}

pub fn level_db(v: f32) -> String {
    if v <= 0.0005 { "Off".into() } else { format!("{:.1} dB", 20.0 * v.log10()).replace('-', "−") }
}

/// A loop's name without its group ("Crackle: SP04" -> "SP04").
pub fn short_loop(name: &str) -> &str {
    name.split_once(": ").map_or(name, |(_, n)| n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn values_read_as_the_panel_words_them() {
        let params = DrumParams::from_faust();
        assert_eq!(text(&params, "tone.pitch"), "50 Hz G0");
        assert_eq!(nearest_note(440.0), ("A3".to_string(), 0.0));
        let (name, cents) = nearest_note(261.63);
        assert!(name == "C3" && cents.abs() < 0.1);
        assert!((nearest_note(50.0).1 - 35.0).abs() < 1.0, "50 Hz is G0 + 35 cents");
        assert_eq!(text(&params, "tone.drop"), "+36 st");
        assert_eq!(text(&params, "tone.decay"), "700 ms");
        assert_eq!(text(&params, "tone.snap"), "120 ms");
        assert_eq!(text(&params, "noise.bandpass"), "2.5 kHz");
        assert_eq!(text(&params, "compressor.threshold"), "Off");
        assert_eq!(text(&params, "body.material"), "Membrane");
        assert_eq!(text(&params, "sample_1.decay"), "Whole");
        assert_eq!(text(&params, "noise.generator"), "White");
        assert_eq!(set_hits(1).len(), 55);
        assert_eq!(set_hits(0)[22], "Jade 029");
        assert_eq!(material(0.75), "Plate→Bar");
    }
}
