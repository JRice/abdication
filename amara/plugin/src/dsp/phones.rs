//! Every sound any of the languages can have, and how the tract makes it. Frequencies are
//! for an adult male tract; the Throat knob scales them all.

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Manner {
    Vowel,
    Stop,
    Ejective,
    Fricative,
    Nasal,
    Tap,
    Trill,
    Approximant,
    GlottalStop,
}
use Manner::*;

#[derive(Clone, Copy, Debug)]
pub struct Phone {
    pub symbol: &'static str,
    pub manner: Manner,
    pub voiced: bool,
    /// Higher is more vowel-like. Clusters must rise toward the nucleus.
    pub sonority: u8,
    /// F1-F3 for a vowel, or the place's locus for a consonant.
    pub formants: [f32; 3],
    /// How much a consonant's formants follow the neighbouring vowel instead of its locus.
    pub blend: f32,
    /// Where a burst or frication resonates.
    pub noise: [f32; 3],
    /// How broad those resonances are, against a vowel's. Noise through a narrow resonance
    /// is a whistle, and real frication has broad humps instead.
    pub width: f32,
    /// How loud it is, relative to an open vowel.
    pub gain: f32,
    /// Duration in seconds at normal rate: the floor, and what it has room to be.
    pub floor: f32,
    pub inherent: f32,
    /// Pharyngealized, uvular or pharyngeal: backs and lowers the vowels beside it.
    pub backing: bool,
}

const fn vowel(symbol: &'static str, f1: f32, f2: f32, f3: f32, inherent: f32) -> Phone {
    Phone {
        symbol,
        manner: Vowel,
        voiced: true,
        sonority: 10,
        formants: [f1, f2, f3],
        blend: 1.0,
        noise: [f1, f2, f3],
        width: 1.0,
        gain: 1.0,
        floor: 0.055,
        inherent,
        backing: false,
    }
}

const LABIAL: [f32; 3] = [260.0, 800.0, 2200.0];
const ALVEOLAR: [f32; 3] = [260.0, 1750.0, 2600.0];
const POSTALVEOLAR: [f32; 3] = [280.0, 2100.0, 2700.0];
const VELAR: [f32; 3] = [280.0, 1850.0, 2300.0];
const UVULAR: [f32; 3] = [480.0, 1150.0, 2700.0];
const PHARYNGEAL: [f32; 3] = [820.0, 1150.0, 2600.0];
const GLOTTAL: [f32; 3] = [500.0, 1400.0, 2500.0];

const fn stop(symbol: &'static str, voiced: bool, locus: [f32; 3], noise: [f32; 3], backing: bool) -> Phone {
    Phone { symbol, manner: Stop, voiced, sonority: 1, formants: locus, blend: 0.35, noise, width: 5.0, gain: 0.26, floor: 0.05, inherent: 0.085, backing }
}

const fn ejective(symbol: &'static str, locus: [f32; 3], noise: [f32; 3], backing: bool) -> Phone {
    Phone { symbol, manner: Ejective, voiced: false, sonority: 0, formants: locus, blend: 0.3, noise, width: 5.0, gain: 0.5, floor: 0.07, inherent: 0.11, backing }
}

const fn fricative(symbol: &'static str, voiced: bool, locus: [f32; 3], (noise, width): ([f32; 3], f32), gain: f32, backing: bool) -> Phone {
    Phone { symbol, manner: Fricative, voiced, sonority: 3, formants: locus, blend: 0.4, noise, width, gain, floor: 0.06, inherent: 0.12, backing }
}

const fn nasal(symbol: &'static str, locus: [f32; 3]) -> Phone {
    Phone { symbol, manner: Nasal, voiced: true, sonority: 5, formants: [270.0, locus[1], locus[2]], blend: 0.35, noise: locus, width: 1.0, gain: 0.5, floor: 0.045, inherent: 0.075, backing: false }
}

const fn sonorant(symbol: &'static str, manner: Manner, formants: [f32; 3], gain: f32, (floor, inherent): (f32, f32), sonority: u8, backing: bool) -> Phone {
    Phone { symbol, manner, voiced: true, sonority, formants, blend: 0.3, noise: formants, width: 1.0, gain, floor, inherent, backing }
}

pub const PHONES: [Phone; 47] = [
    vowel("i", 280.0, 2250.0, 3000.0, 0.11),
    vowel("e", 420.0, 2000.0, 2600.0, 0.125),
    vowel("ɛ", 550.0, 1800.0, 2500.0, 0.135),
    vowel("a", 760.0, 1300.0, 2500.0, 0.155),
    vowel("ɑ", 720.0, 1000.0, 2500.0, 0.155),
    vowel("ɔ", 570.0, 850.0, 2500.0, 0.14),
    vowel("o", 430.0, 800.0, 2500.0, 0.125),
    vowel("u", 300.0, 870.0, 2300.0, 0.11),
    vowel("ɨ", 320.0, 1500.0, 2300.0, 0.09),
    vowel("ə", 500.0, 1400.0, 2500.0, 0.07),
    stop("p", false, LABIAL, [700.0, 1500.0, 3000.0], false),
    stop("b", true, LABIAL, [700.0, 1500.0, 3000.0], false),
    stop("t", false, ALVEOLAR, [1800.0, 3400.0, 4800.0], false),
    stop("d", true, ALVEOLAR, [1800.0, 3400.0, 4800.0], false),
    stop("k", false, VELAR, [1300.0, 2200.0, 3400.0], false),
    stop("g", true, VELAR, [1300.0, 2200.0, 3400.0], false),
    stop("q", false, UVULAR, [900.0, 1400.0, 2800.0], true),
    stop("ɢ", true, UVULAR, [900.0, 1400.0, 2800.0], true),
    stop("tˤ", false, [320.0, 1150.0, 2600.0], [1500.0, 3000.0, 4400.0], true),
    stop("dˤ", true, [320.0, 1150.0, 2600.0], [1500.0, 3000.0, 4400.0], true),
    ejective("pʼ", LABIAL, [800.0, 1700.0, 3200.0], false),
    ejective("tʼ", ALVEOLAR, [2000.0, 3600.0, 5000.0], false),
    ejective("kʼ", VELAR, [1400.0, 2300.0, 3500.0], false),
    ejective("qʼ", UVULAR, [950.0, 1500.0, 2900.0], true),
    fricative("f", false, LABIAL, ([1400.0, 3000.0, 5200.0], 11.0), 0.16, false),
    fricative("θ", false, ALVEOLAR, ([1500.0, 3200.0, 5600.0], 11.0), 0.14, false),
    fricative("s", false, ALVEOLAR, ([2000.0, 4600.0, 6200.0], 6.0), 0.3, false),
    fricative("z", true, ALVEOLAR, ([2000.0, 4600.0, 6200.0], 6.0), 0.28, false),
    fricative("sˤ", false, [320.0, 1150.0, 2600.0], ([1700.0, 3900.0, 5600.0], 6.0), 0.3, true),
    fricative("ʃ", false, POSTALVEOLAR, ([1900.0, 2900.0, 4300.0], 7.0), 0.32, false),
    fricative("x", false, VELAR, ([1100.0, 2000.0, 3100.0], 7.0), 0.3, false),
    fricative("χ", false, UVULAR, ([900.0, 1300.0, 2800.0], 6.0), 0.4, true),
    fricative("ʁ", true, UVULAR, ([900.0, 1300.0, 2800.0], 6.0), 0.4, true),
    fricative("ħ", false, PHARYNGEAL, ([950.0, 1250.0, 2700.0], 9.5), 0.35, true),
    fricative("ʕ", true, PHARYNGEAL, ([850.0, 1200.0, 2600.0], 10.0), 0.55, true),
    fricative("h", false, GLOTTAL, ([500.0, 1400.0, 2500.0], 6.0), 0.14, false),
    fricative("v", true, LABIAL, ([1400.0, 3000.0, 5200.0], 11.0), 0.16, false),
    nasal("m", LABIAL),
    nasal("n", ALVEOLAR),
    nasal("ŋ", VELAR),
    sonorant("l", Approximant, [360.0, 1050.0, 2600.0], 0.7, (0.045, 0.075), 7, false),
    sonorant("ɾ", Tap, [400.0, 1500.0, 2200.0], 0.65, (0.03, 0.04), 6, false),
    sonorant("ʀ", Trill, [480.0, 1200.0, 2500.0], 0.65, (0.075, 0.1), 6, true),
    sonorant("r", Trill, [420.0, 1400.0, 2200.0], 0.65, (0.07, 0.095), 6, false),
    sonorant("j", Approximant, [280.0, 2200.0, 2900.0], 0.75, (0.04, 0.07), 8, false),
    sonorant("w", Approximant, [300.0, 700.0, 2200.0], 0.75, (0.04, 0.07), 8, false),
    Phone { symbol: "ʔ", manner: GlottalStop, voiced: false, sonority: 0, formants: GLOTTAL, blend: 1.0, noise: GLOTTAL, width: 1.0, gain: 0.0, floor: 0.04, inherent: 0.06, backing: false },
];

/// The index of the phone with this symbol.
pub fn id(symbol: &str) -> u8 {
    PHONES.iter().position(|p| p.symbol == symbol).unwrap_or_else(|| panic!("no phone {symbol}")) as u8
}

pub fn phone(id: u8) -> &'static Phone {
    &PHONES[id as usize]
}

/// A front vowel, for languages with harmony.
pub fn is_front(id: u8) -> bool {
    phone(id).formants[1] > 1600.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symbols_are_unique() {
        for (i, a) in PHONES.iter().enumerate() {
            assert!(PHONES[i + 1..].iter().all(|b| b.symbol != a.symbol), "{}", a.symbol);
        }
    }
}
