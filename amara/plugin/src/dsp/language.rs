//! A language from a seed: an inventory, what may stand next to what, a stress rule, an
//! allophone or two, and a small pool of morphemes that keep coming back. There's no
//! grammar and nothing means anything, but near-repetition makes it sound as if it might.
//!
//! Everything is fixed-size, so changing language never allocates on the audio thread.

use super::phones::{id, is_front, phone, PHONES};
use super::random::Rng;

const MAX_INVENTORY: usize = 36;
const MAX_VOWELS: usize = 8;
const ROOTS: usize = 36;
const AFFIXES: usize = 6;
const PARTICLES: usize = 4;
const MAX_SYLLABLES: usize = 3;

/// A consonant slot in a syllable, or nothing.
pub const NONE: u8 = u8::MAX;

#[derive(Clone, Copy)]
pub struct Syllable {
    pub onset: [u8; 2],
    pub nucleus: u8,
    pub long: bool,
    pub coda: [u8; 2],
}

const EMPTY_SYLLABLE: Syllable = Syllable { onset: [NONE; 2], nucleus: 0, long: false, coda: [NONE; 2] };

#[derive(Clone, Copy)]
pub struct Morpheme {
    pub syllables: [Syllable; MAX_SYLLABLES],
    pub count: u8,
}

const EMPTY_MORPHEME: Morpheme = Morpheme { syllables: [EMPTY_SYLLABLE; MAX_SYLLABLES], count: 0 };

impl Morpheme {
    pub fn syllables(&self) -> &[Syllable] {
        &self.syllables[..self.count as usize]
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Stress {
    Initial,
    Penultimate,
    Final,
}

/// The world's common consonants, weighted roughly by the share of languages that have
/// them. Every language is mostly made of these.
const COMMON_CONSONANTS: [(&str, f32); 21] = [
    ("m", 0.96), ("k", 0.9), ("j", 0.9), ("p", 0.86), ("w", 0.82), ("n", 0.78), ("t", 0.68), ("s", 0.67), ("l", 0.68),
    ("ŋ", 0.63), ("b", 0.63), ("g", 0.57), ("h", 0.56), ("d", 0.46), ("f", 0.47), ("r", 0.44), ("ʔ", 0.37), ("ʃ", 0.37),
    ("ɾ", 0.32), ("z", 0.3), ("v", 0.27),
];

/// The register's own sounds. A language keeps a few of these as its signature, the way
/// real languages with them do: Arabic's throat sounds sit on an ordinary m, n, b, t, k.
const SIGNATURE_CONSONANTS: [(&str, f32); 16] = [
    ("q", 3.0), ("ħ", 3.0), ("ʕ", 3.0), ("χ", 3.0), ("qʼ", 2.0), ("kʼ", 2.0), ("tʼ", 1.5), ("pʼ", 0.5),
    ("tˤ", 2.0), ("dˤ", 1.5), ("sˤ", 2.0), ("ʁ", 2.0), ("ʀ", 2.0), ("ɢ", 1.0), ("x", 1.5), ("θ", 1.0),
];

/// Sounds it's strange to have without having these too: `(sound, what it implies)`.
const IMPLIED: [(&str, &str); 11] = [
    ("b", "p"), ("d", "t"), ("g", "k"), ("z", "s"), ("v", "f"), ("tʼ", "t"), ("pʼ", "p"), ("qʼ", "q"), ("ɢ", "q"), ("dˤ", "tˤ"), ("sˤ", "s"),
];

/// Five vowels is by far the commonest system, and the corners are nearly universal.
const VOWEL_SYSTEM_WEIGHTS: [f32; 5] = [0.15, 0.1, 0.38, 0.18, 0.19];
const VOWEL_SYSTEMS: [&[&str]; 5] = [
    &["i", "u"],
    &["i", "u", "ə"],
    &["i", "u", "e", "o"],
    &["i", "u", "e", "o", "ə"],
    &["i", "u", "e", "o", "ɛ", "ɔ"],
];
/// How often each vowel gets used: the corners are the gravity wells.
fn vowel_usage(symbol: &str) -> f32 {
    match symbol {
        "a" | "ɑ" => 3.0,
        "i" => 2.2,
        "u" => 1.7,
        "e" | "o" => 1.2,
        _ => 0.8,
    }
}

/// How strange a language is by default: an ordinary language with a throat full of
/// something else.
pub const DEFAULT_STRANGENESS: f32 = 0.4;
/// The most signature consonants any language takes on.
const MOST_SIGNATURES: f32 = 9.0;

/// Which syllable shapes a language allows, by weight: CV, CVC, V, CCV, CVCC, CCVC.
const SHAPE_COUNT: usize = 6;
const ONSET_CONSONANTS: [u8; SHAPE_COUNT] = [1, 1, 0, 2, 1, 2];
const CODA_CONSONANTS: [u8; SHAPE_COUNT] = [0, 1, 0, 0, 2, 1];

/// A cluster must climb at least this far in sonority toward the nucleus.
const MINIMUM_RISE: i32 = 2;
/// Tries at a sonorous sound next to the nucleus. More makes every cluster end in a glide.
const CLUSTER_DRAWS: usize = 3;

#[derive(Clone)]
pub struct Language {
    consonants: [u8; MAX_INVENTORY],
    consonant_usage: [f32; MAX_INVENTORY],
    consonant_count: usize,
    vowels: [u8; MAX_VOWELS],
    vowel_usage: [f32; MAX_VOWELS],
    vowel_count: usize,
    shapes: [f32; SHAPE_COUNT],
    long_vowels: f32,
    harmony: bool,
    pub final_devoicing: bool,
    pub spirantizes: bool,
    pub stress: Stress,
    /// How far unstressed syllables shrink: low for stress-timed, near one for syllable-timed.
    pub unstressed_scale: f32,
    roots: [Morpheme; ROOTS],
    affixes: [Morpheme; AFFIXES],
    particles: [Morpheme; PARTICLES],
}

impl Language {
    /// `strangeness` 0 is a thoroughly ordinary language, 1 hardly anything but signature.
    pub fn new(seed: u32, strangeness: f32) -> Self {
        let mut rng = Rng::new(seed.wrapping_add(1));
        let mut inventory = Inventory::default();
        // Nearly everybody has these.
        inventory.add("n", 0.78);
        inventory.add("k", 0.9);
        inventory.draw(&COMMON_CONSONANTS, 8 + rng.below(6), |weight| weight, &mut rng);

        let usage = 0.35 + 1.3 * strangeness;
        let signatures = (strangeness * MOST_SIGNATURES + rng.unit() * 1.5) as usize;
        if strangeness >= 0.15 {
            // At least one sound from far down the throat, so it's still itself.
            inventory.add(["ħ", "ʕ", "q", "χ"][rng.below(4)], usage);
        }
        inventory.draw(&SIGNATURE_CONSONANTS, signatures, |_| usage, &mut rng);
        inventory.complete();

        let mut vowels = [0; MAX_VOWELS];
        let mut vowel_usage_table = [0.0; MAX_VOWELS];
        let low = if rng.chance(strangeness) { "ɑ" } else { "a" };
        let others = VOWEL_SYSTEMS[rng.weighted(&VOWEL_SYSTEM_WEIGHTS)];
        for (i, symbol) in std::iter::once(&low).chain(others).enumerate() {
            vowels[i] = id(symbol);
            vowel_usage_table[i] = vowel_usage(symbol);
        }
        let vowel_count = others.len() + 1;

        let stress = [Stress::Initial, Stress::Penultimate, Stress::Final][rng.weighted(&[3.0, 4.0, 1.5])];
        let shapes = [rng.range(3.0, 6.0), rng.range(1.0, 4.0), rng.range(0.0, 0.6), rng.range(0.0, 1.5), rng.range(0.0, 0.8), rng.range(0.0, 0.8)];
        let mut language = Self {
            consonants: inventory.consonants,
            consonant_usage: inventory.usage,
            consonant_count: inventory.count,
            vowels,
            vowel_usage: vowel_usage_table,
            vowel_count,
            shapes,
            long_vowels: rng.range(0.0, 0.3),
            harmony: rng.chance(0.35),
            final_devoicing: rng.chance(0.4),
            spirantizes: rng.chance(0.35),
            stress,
            unstressed_scale: if rng.chance(0.55) { rng.range(0.55, 0.72) } else { rng.range(0.82, 0.98) },
            roots: [EMPTY_MORPHEME; ROOTS],
            affixes: [EMPTY_MORPHEME; AFFIXES],
            particles: [EMPTY_MORPHEME; PARTICLES],
        };
        for i in 0..ROOTS {
            let count = 1 + rng.weighted(&[3.0, 4.0, 1.5]);
            language.roots[i] = language.morpheme(count, &mut rng);
        }
        for i in 0..AFFIXES {
            language.affixes[i] = language.morpheme(1, &mut rng);
        }
        for i in 0..PARTICLES {
            language.particles[i] = language.light_morpheme(&mut rng);
        }
        language
    }

    fn consonant(&self, rng: &mut Rng) -> u8 {
        self.consonants[rng.weighted(&self.consonant_usage[..self.consonant_count])]
    }

    fn vowel(&self, rng: &mut Rng) -> u8 {
        self.vowels[rng.weighted(&self.vowel_usage[..self.vowel_count])]
    }

    /// A consonant that sits at least `MINIMUM_RISE` away in sonority, or nothing.
    fn cluster_partner(&self, rng: &mut Rng, accepts: impl Fn(i32) -> bool) -> u8 {
        (0..8).map(|_| self.consonant(rng)).find(|c| accepts(phone(*c).sonority as i32)).unwrap_or(NONE)
    }

    fn syllable(&self, nucleus: u8, rng: &mut Rng) -> Syllable {
        let shape = rng.weighted(&self.shapes);
        let mut syllable = Syllable { onset: [NONE; 2], nucleus, long: rng.chance(self.long_vowels), coda: [NONE; 2] };
        match ONSET_CONSONANTS[shape] {
            0 => {}
            1 => syllable.onset[1] = self.consonant(rng),
            _ => {
                let inner = (0..CLUSTER_DRAWS).map(|_| self.consonant(rng)).max_by_key(|c| phone(*c).sonority).unwrap_or(NONE);
                let top = phone(inner).sonority as i32;
                syllable.onset = [self.cluster_partner(rng, |s| s + MINIMUM_RISE <= top), inner];
            }
        }
        if CODA_CONSONANTS[shape] >= 1 {
            syllable.coda[0] = self.consonant(rng);
        }
        if CODA_CONSONANTS[shape] == 2 {
            let first = phone(syllable.coda[0]).sonority as i32;
            syllable.coda[1] = self.cluster_partner(rng, |s| s + MINIMUM_RISE <= first);
        }
        syllable
    }

    fn morpheme(&self, count: usize, rng: &mut Rng) -> Morpheme {
        let mut morpheme = EMPTY_MORPHEME;
        let first = self.vowel(rng);
        for slot in morpheme.syllables.iter_mut().take(count) {
            let mut nucleus = self.vowel(rng);
            if self.harmony {
                // Stay on the first vowel's side of the mouth, if the inventory allows.
                nucleus = (0..8).map(|_| self.vowel(rng)).find(|v| is_front(*v) == is_front(first)).unwrap_or(first);
            }
            *slot = self.syllable(nucleus, rng);
        }
        morpheme.count = count as u8;
        morpheme
    }

    /// One short syllable with no coda: the little words every language has.
    fn light_morpheme(&self, rng: &mut Rng) -> Morpheme {
        let mut morpheme = EMPTY_MORPHEME;
        morpheme.syllables[0] = Syllable { onset: [NONE, self.consonant(rng)], nucleus: self.vowel(rng), long: false, coda: [NONE; 2] };
        morpheme.count = 1;
        morpheme
    }

    /// Common things are very common: the low indices come up far more often.
    fn zipf(count: usize, rng: &mut Rng) -> usize {
        (rng.unit().powf(1.8) * count as f32) as usize % count
    }

    /// A root, not `previous` again: repetition is fine, stammering isn't.
    pub fn root(&self, previous: usize, rng: &mut Rng) -> (usize, &Morpheme) {
        let index = (0..4).map(|_| Self::zipf(ROOTS, rng)).find(|i| *i != previous).unwrap_or((previous + 1) % ROOTS);
        (index, &self.roots[index])
    }

    pub fn affix(&self, rng: &mut Rng) -> &Morpheme {
        &self.affixes[Self::zipf(AFFIXES, rng)]
    }

    pub fn particle(&self, rng: &mut Rng) -> &Morpheme {
        &self.particles[Self::zipf(PARTICLES, rng)]
    }

    /// What it calls itself: its commonest root, written out.
    pub fn name(&self) -> String {
        let mut name = String::new();
        for syllable in self.roots[0].syllables() {
            for c in syllable.onset.iter().filter(|c| **c != NONE) {
                name.push_str(phone(*c).symbol);
            }
            name.push_str(phone(syllable.nucleus).symbol);
            if syllable.long {
                name.push('ː');
            }
            for c in syllable.coda.iter().filter(|c| **c != NONE) {
                name.push_str(phone(*c).symbol);
            }
        }
        name
    }

    pub fn morphemes(&self) -> impl Iterator<Item = &Morpheme> {
        self.roots.iter().chain(&self.affixes).chain(&self.particles)
    }
}

/// The consonants a language is collecting, each with how often it gets used.
struct Inventory {
    consonants: [u8; MAX_INVENTORY],
    usage: [f32; MAX_INVENTORY],
    count: usize,
}

impl Default for Inventory {
    fn default() -> Self {
        Self { consonants: [0; MAX_INVENTORY], usage: [0.0; MAX_INVENTORY], count: 0 }
    }
}

impl Inventory {
    fn has(&self, symbol: &str) -> bool {
        self.consonants[..self.count].contains(&id(symbol))
    }

    fn add(&mut self, symbol: &str, usage: f32) {
        if !self.has(symbol) && self.count < MAX_INVENTORY {
            self.consonants[self.count] = id(symbol);
            self.usage[self.count] = usage;
            self.count += 1;
        }
    }

    /// Adds this many more from `choices`, more likely by weight.
    fn draw(&mut self, choices: &[(&str, f32)], count: usize, usage: impl Fn(f32) -> f32, rng: &mut Rng) {
        let mut weights = [0.0; MAX_INVENTORY];
        for ((symbol, weight), slot) in choices.iter().zip(&mut weights) {
            *slot = if self.has(symbol) { 0.0 } else { *weight };
        }
        for _ in 0..count {
            if weights[..choices.len()].iter().all(|w| *w == 0.0) {
                return;
            }
            let chosen = rng.weighted(&weights[..choices.len()]);
            self.add(choices[chosen].0, usage(choices[chosen].1));
            weights[chosen] = 0.0;
        }
    }

    /// Fills in what the sounds it has imply. Twice, because `dˤ` implies `tˤ` implies nothing
    /// new today, but a chain costs nothing to allow for.
    fn complete(&mut self) {
        for _ in 0..2 {
            for (sound, implied) in IMPLIED {
                if self.has(sound) {
                    let usage = self.consonants[..self.count].iter().position(|c| *c == id(sound)).map_or(0.3, |i| self.usage[i]);
                    self.add(implied, usage);
                }
            }
        }
    }
}

/// The voiceless partner of a voiced obstruent, for final devoicing.
pub fn devoiced(symbol_id: u8) -> u8 {
    const PAIRS: [(&str, &str); 7] = [("b", "p"), ("d", "t"), ("g", "k"), ("ɢ", "q"), ("dˤ", "tˤ"), ("z", "s"), ("ʁ", "χ")];
    let symbol = phone(symbol_id).symbol;
    PAIRS.iter().find(|(voiced, _)| *voiced == symbol).map(|(_, voiceless)| id(voiceless)).unwrap_or(symbol_id)
}

/// The fricative a stop relaxes into between vowels, in languages that do that.
pub fn spirantized(symbol_id: u8) -> u8 {
    const PAIRS: [(&str, &str); 4] = [("p", "f"), ("t", "θ"), ("k", "x"), ("q", "χ")];
    let symbol = phone(symbol_id).symbol;
    PAIRS.iter().find(|(stop, _)| *stop == symbol).map(|(_, fricative)| id(fricative)).unwrap_or(symbol_id)
}

const _: () = assert!(PHONES.len() < NONE as usize);

#[cfg(test)]
mod tests {
    use super::super::phones::Manner;
    use super::*;

    fn is_vowel(id: u8) -> bool {
        phone(id).manner == Manner::Vowel
    }

    fn sonority(id: u8) -> i32 {
        phone(id).sonority as i32
    }

    #[test]
    fn the_same_seed_gives_the_same_language_and_others_differ() {
        assert_eq!(Language::new(7, 0.4).name(), Language::new(7, 0.4).name());
        let names: std::collections::HashSet<String> = (0..50).map(|s| Language::new(s, 0.4).name()).collect();
        assert!(names.len() > 25, "{}", names.len());
    }

    #[test]
    fn every_cluster_in_every_language_obeys_sonority() {
        for seed in 0..300 {
            let language = Language::new(seed, seed as f32 / 300.0);
            for morpheme in language.morphemes() {
                assert!(morpheme.count >= 1);
                for s in morpheme.syllables() {
                    assert!(is_vowel(s.nucleus));
                    if s.onset[0] != NONE {
                        assert!(sonority(s.onset[0]) + MINIMUM_RISE <= sonority(s.onset[1]), "{seed}");
                    }
                    if s.coda[1] != NONE {
                        assert!(sonority(s.coda[1]) + MINIMUM_RISE <= sonority(s.coda[0]), "{seed}");
                    }
                    assert!(s.onset[1] == NONE || !is_vowel(s.onset[1]));
                }
            }
        }
    }

    #[test]
    fn a_handful_of_morphemes_does_most_of_the_talking() {
        let language = Language::new(3, 0.4);
        let mut rng = Rng::new(9);
        let mut common = 0;
        for _ in 0..1_000 {
            if language.root(ROOTS, &mut rng).0 < 6 {
                common += 1;
            }
        }
        assert!(common > 350, "{common}");
    }

    #[test]
    fn every_language_can_be_said_and_has_its_throat_sounds() {
        for seed in 0..100 {
            let language = Language::new(seed, DEFAULT_STRANGENESS);
            assert!(!language.name().is_empty());
            let throaty = language.consonants[..language.consonant_count].iter().filter(|c| phone(**c).backing).count();
            assert!(throaty >= 1, "{seed}");
        }
    }

    /// Sounds as said, not as listed: every morpheme, with the common ones counted more.
    fn tally(language: &Language, rng: &mut Rng) -> (Vec<u8>, Vec<u8>) {
        let (mut vowels, mut consonants) = (Vec::new(), Vec::new());
        for _ in 0..200 {
            let (_, root) = language.root(ROOTS, rng);
            for s in root.syllables() {
                vowels.push(s.nucleus);
                consonants.extend(s.onset.iter().chain(&s.coda).filter(|c| **c != NONE));
            }
        }
        (vowels, consonants)
    }

    fn share(sounds: &[u8], which: impl Fn(u8) -> bool) -> f32 {
        sounds.iter().filter(|s| which(**s)).count() as f32 / sounds.len() as f32
    }

    fn is_signature(c: u8) -> bool {
        SIGNATURE_CONSONANTS.iter().any(|(symbol, _)| id(symbol) == c)
    }

    #[test]
    fn the_corner_vowels_are_nearly_universal_and_do_most_of_the_work() {
        let mut rng = Rng::new(1);
        let (mut corners, mut front, mut has_i, mut has_u, mut five) = (0.0, 0.0, 0, 0, 0);
        const SAMPLE: usize = 300;
        for seed in 0..SAMPLE as u32 {
            let language = Language::new(seed, DEFAULT_STRANGENESS);
            let (vowels, _) = tally(&language, &mut rng);
            let system = &language.vowels[..language.vowel_count];
            has_i += system.contains(&id("i")) as usize;
            has_u += system.contains(&id("u")) as usize;
            five += (language.vowel_count == 5) as usize;
            corners += share(&vowels, |v| ["a", "ɑ", "i", "u"].contains(&phone(v).symbol));
            front += share(&vowels, is_front);
        }
        let (corners, front) = (corners / SAMPLE as f32, front / SAMPLE as f32);
        assert_eq!(has_i, SAMPLE);
        assert_eq!(has_u, SAMPLE);
        assert!(corners > 0.6, "{corners}");
        assert!(front > 0.25, "{front}");
        assert!(five > SAMPLE / 4, "{five}");
    }

    #[test]
    fn strangeness_sets_how_exotic_the_consonants_are() {
        let exotic = |strangeness: f32| {
            let mut rng = Rng::new(2);
            (0..200).map(|seed| share(&tally(&Language::new(seed, strangeness), &mut rng).1, is_signature)).sum::<f32>() / 200.0
        };
        let (none, default, full) = (exotic(0.0), exotic(DEFAULT_STRANGENESS), exotic(1.0));
        assert!(none < 0.08, "{none}");
        assert!((0.12..0.42).contains(&default), "{default}");
        assert!(full > 0.5, "{full}");
    }

    #[test]
    fn implied_sounds_are_always_there() {
        for seed in 0..300 {
            let language = Language::new(seed, 1.0);
            let has = |symbol: &str| language.consonants[..language.consonant_count].contains(&id(symbol));
            for (sound, implied) in IMPLIED {
                assert!(!has(sound) || has(implied), "{seed}: {sound} without {implied}");
            }
        }
    }
}
