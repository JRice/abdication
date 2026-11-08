//! Turns a language into a stream of articulations and sings it through a tract: plans a
//! phrase at a time (rhythm, accents, a boundary tone), then moves the formants, pitch
//! and effort toward each target in turn.

use super::language::{devoiced, spirantized, Language, Morpheme, Stress, NONE};
use super::phones::{id, phone, Manner, Phone};
use super::random::Rng;

pub const FORMANTS: usize = 6;
const MAX_PLANNED: usize = 256;
const MAX_ARTICULATIONS: usize = 700;
/// Higher formants don't depend on the phone, only on the tract's length.
const HIGH_FORMANTS: [f32; 3] = [3500.0, 4500.0, 5500.0];
const BANDWIDTHS: [f32; FORMANTS] = [60.0, 80.0, 120.0, 175.0, 280.0, 400.0];
/// The Rate knob's setting at which the phones' listed durations hold, chosen so the knob
/// reads about right in syllables a second across the languages (`examples/rate.rs`).
const REFERENCE_RATE: f32 = 3.4;
const ACCENT_GLIDE_S: f32 = 0.055;
const GAIN_GLIDE_S: f32 = 0.004;
const DECLINATION_ST_PER_S: f32 = 1.3;
const TEMPER_PITCH_ST: f32 = 9.0;
const ASPIRATION_WIDTH: f32 = 5.0;
const LET_GO_S: f32 = 0.03;
const NOISE_BANDWIDTH: f32 = 130.0;

/// What the tract is doing right now, ready for the codec.
#[derive(Clone, Copy, Debug)]
pub struct Target {
    pub formants: [f32; FORMANTS],
    pub bandwidths: [f32; FORMANTS],
    /// The source's slope: closer to one is darker.
    pub tilt: f32,
    pub f0: f32,
    /// 0 all noise, 1 all buzz.
    pub voicing: f32,
    pub gain: f32,
    /// How unstable the vocal folds are.
    pub rough: f32,
}

#[derive(Clone, Copy)]
pub struct SpeakerKnobs {
    /// Syllables a second.
    pub rate: f32,
    /// 0 barely stops for breath, 1 mostly silence.
    pub sparsity: f32,
    /// 0 hushed, 1 furious.
    pub temper: f32,
    /// How far temper moves through and between phrases.
    pub swing: f32,
    pub breath: f32,
    /// Multiplies every formant: a long throat is below one.
    pub tract: f32,
    pub pitch_hz: f32,
    /// Speaks only when told to, one syllable at a time, in tune.
    pub per_note: bool,
}

/// A phone or boundary, for the running transcript.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Mark {
    Phone(u8),
    Long,
    Stress,
    Word,
    Phrase,
}

#[derive(Clone, Copy)]
struct Planned {
    phone: u8,
    stressed: bool,
    long: bool,
    /// In the phrase's last syllable: the lengthened one.
    final_syllable: bool,
    /// Semitones above the baseline.
    accent: f32,
    word_start: bool,
    syllable_start: bool,
}

#[derive(Clone, Copy)]
struct Articulation {
    formants: [f32; 3],
    /// How fast the formants move here.
    glide_s: f32,
    wide: f32,
    voicing: f32,
    gain: f32,
    rough: f32,
    seconds: f32,
    accent: f32,
    mark: Option<Mark>,
    stress_mark: bool,
    word_mark: bool,
    long_mark: bool,
}

const SILENT: Articulation = Articulation {
    formants: [500.0, 1400.0, 2500.0],
    glide_s: 0.02,
    wide: 1.0,
    voicing: 0.0,
    gain: 0.0,
    rough: 0.0,
    seconds: 0.0,
    accent: 0.0,
    mark: None,
    stress_mark: false,
    word_mark: false,
    long_mark: false,
};

pub struct Speaker {
    language: Language,
    rng: Rng,
    planned: Vec<Planned>,
    script: Vec<Articulation>,
    at: usize,
    /// Seconds left in the current articulation.
    left: f32,
    /// Seconds since the phrase began, and how long it runs.
    elapsed: f32,
    phrase_length: f32,
    phrase_temper: f32,
    temper_drift: f32,
    upcoming_pause: f32,
    previous_root: usize,
    /// In per-note mode: where the next syllable starts in the planned phrase.
    next_syllable: usize,
    per_note: bool,
    formants: [f32; 3],
    formants_smoother: [f32; 3],
    gain: f32,
    voicing: f32,
    accent: f32,
    wide: f32,
    rough: f32,
    wander: f32,
    pitch_hz: f32,
    marks: [Mark; 8],
    mark_count: usize,
}

impl Speaker {
    pub fn new(seed: u32, strangeness: f32) -> Self {
        let mut speaker = Self {
            language: Language::new(seed, strangeness),
            rng: Rng::new(seed ^ 0x5EED),
            planned: Vec::with_capacity(MAX_PLANNED),
            script: Vec::with_capacity(MAX_ARTICULATIONS),
            at: 0,
            left: 0.0,
            elapsed: 0.0,
            phrase_length: 1.0,
            phrase_temper: 0.5,
            temper_drift: 0.0,
            upcoming_pause: 0.2,
            previous_root: usize::MAX,
            next_syllable: 0,
            per_note: false,
            formants: SILENT.formants,
            formants_smoother: SILENT.formants,
            gain: 0.0,
            voicing: 0.0,
            accent: 0.0,
            wide: 1.0,
            rough: 0.0,
            wander: 0.0,
            pitch_hz: 90.0,
            marks: [Mark::Word; 8],
            mark_count: 0,
        };
        speaker.script.push(Articulation { seconds: 0.05, ..SILENT });
        speaker
    }

    pub fn set_language(&mut self, seed: u32, strangeness: f32) {
        self.language = Language::new(seed, strangeness);
    }

    pub fn language_name(&self) -> String {
        self.language.name()
    }

    /// The transcript since last asked.
    pub fn take_marks(&mut self) -> &[Mark] {
        let count = self.mark_count;
        self.mark_count = 0;
        &self.marks[..count]
    }

    fn mark(&mut self, mark: Mark) {
        if self.mark_count < self.marks.len() {
            self.marks[self.mark_count] = mark;
            self.mark_count += 1;
        }
    }

    // ---- Planning a phrase ----------------------------------------------------------

    fn push_morpheme(&mut self, morpheme: Morpheme, word_start: bool, stress_at: Option<usize>, accent: f32) {
        let spirantizes = self.language.spirantizes;
        for (i, syllable) in morpheme.syllables().iter().enumerate() {
            let stressed = stress_at == Some(i);
            let accent_here = if stressed { accent } else { 0.0 };
            let mut starts_word = word_start && i == 0;
            let mut starts_syllable = true;
            let mut push = |planned: &mut Vec<Planned>, phone: u8, long: bool, accent: f32| {
                if planned.len() < MAX_PLANNED {
                    planned.push(Planned { phone, stressed, long, final_syllable: false, accent, word_start: starts_word, syllable_start: starts_syllable });
                    starts_word = false;
                    starts_syllable = false;
                }
            };
            for c in syllable.onset.iter().filter(|c| **c != NONE) {
                // Single stops between vowels relax, in languages that do that.
                let lenites = spirantizes && syllable.onset[0] == NONE && !stressed && !word_start;
                push(&mut self.planned, if lenites { spirantized(*c) } else { *c }, false, accent_here * 0.5);
            }
            push(&mut self.planned, syllable.nucleus, syllable.long, accent_here);
            for c in syllable.coda.iter().filter(|c| **c != NONE) {
                push(&mut self.planned, *c, false, accent_here * 0.3);
            }
        }
    }

    fn stress_position(&self, count: usize) -> usize {
        match self.language.stress {
            Stress::Initial => 0,
            Stress::Final => count - 1,
            Stress::Penultimate => count.saturating_sub(2),
        }
    }

    /// One phrase's worth of syllables with its pitch accents.
    fn plan(&mut self, knobs: &SpeakerKnobs) {
        self.planned.clear();
        let words = 2 + (self.rng.unit() * (5.0 - 3.0 * knobs.sparsity)) as usize;
        let mut accent = self.rng.range(3.0, 5.0);
        let mut previous_root = self.previous_root;
        let focus = self.rng.below(words);
        for word in 0..words {
            if self.rng.chance(0.3) {
                let particle = *self.language.particle(&mut self.rng);
                self.push_morpheme(particle, true, None, 0.0);
            }
            let (index, root) = self.language.root(previous_root, &mut self.rng);
            let root = *root;
            previous_root = index;
            let suffix = self.rng.chance(0.4).then(|| *self.language.affix(&mut self.rng));
            let syllables = root.count as usize + suffix.map_or(0, |s| s.count as usize);
            let stress = self.stress_position(syllables);
            let here = accent * if word == focus { 1.5 } else { 1.0 };
            self.push_morpheme(root, true, (stress < root.count as usize).then_some(stress), here);
            if let Some(suffix) = suffix {
                self.push_morpheme(suffix, false, stress.checked_sub(root.count as usize), here);
            }
            // Later accents step down.
            accent *= 0.82;
        }
        self.previous_root = previous_root;
        self.finish_phrase();
    }

    /// Devoices what ends the phrase, marks the last syllable, and sets a boundary tone.
    fn finish_phrase(&mut self) {
        let Some(last_vowel) = self.planned.iter().rposition(|p| phone(p.phone).manner == Manner::Vowel) else { return };
        let rises = self.rng.chance(0.22);
        let boundary = if rises { self.rng.range(3.0, 6.0) } else { self.rng.range(-5.0, -2.5) };
        for planned in &mut self.planned[last_vowel..] {
            planned.final_syllable = true;
            planned.accent = boundary;
        }
        if self.language.final_devoicing {
            if let Some(last) = self.planned.last_mut() {
                last.phone = devoiced(last.phone);
            }
        }
    }

    // ---- Planning its articulations ---------------------------------------------------

    fn next_vowel(&self, from: usize) -> Option<&'static Phone> {
        self.planned[from..].iter().map(|p| phone(p.phone)).find(|p| p.manner == Manner::Vowel)
    }

    fn duration(&self, planned: &Planned, p: &Phone, knobs: &SpeakerKnobs, temper: f32) -> f32 {
        // Klatt's rule: only what's above a phone's floor can stretch or shrink, so fast
        // speech squeezes vowels much harder than it squeezes consonants.
        let room = REFERENCE_RATE / (knobs.rate * (0.85 + 0.3 * temper));
        let mut stretch = room;
        if p.manner == Manner::Vowel {
            stretch *= if planned.stressed { 1.35 } else { self.language.unstressed_scale };
            if planned.long {
                stretch *= 1.9;
            }
        }
        if planned.final_syllable {
            stretch *= 1.55;
        }
        p.floor * room.min(1.0) + (p.inherent - p.floor).max(0.0) * stretch
    }

    /// A vowel's own formants, backed and lowered beside the throat consonants.
    fn vowel_formants(&self, index: usize, p: &Phone) -> [f32; 3] {
        let beside = |i: Option<usize>| i.and_then(|i| self.planned.get(i)).is_some_and(|n| phone(n.phone).backing);
        let backed = beside(index.checked_sub(1)) || beside(Some(index + 1));
        if backed {
            [p.formants[0] + 90.0, p.formants[1] - 260.0 * (p.formants[1] / 1500.0).min(1.3), p.formants[2]]
        } else {
            p.formants
        }
    }

    fn consonant_formants(&self, p: &Phone, vowel: Option<&Phone>) -> [f32; 3] {
        let vowel = vowel.map_or(p.formants, |v| v.formants);
        std::array::from_fn(|i| p.formants[i] + (vowel[i] - p.formants[i]) * p.blend)
    }

    fn push(&mut self, articulation: Articulation) {
        if self.script.len() < MAX_ARTICULATIONS {
            self.script.push(articulation);
        }
    }

    /// Starts a fresh script at the top.
    fn clear_script(&mut self) {
        self.script.clear();
        self.at = 0;
        self.left = 0.0;
        self.elapsed = 0.0;
    }

    /// A whole phrase, the pause after it and maybe a breath.
    fn articulate(&mut self, knobs: &SpeakerKnobs) {
        self.clear_script();
        self.articulate_phones(0..self.planned.len(), knobs);
        // The voice lets go, and then the pause and maybe a breath.
        self.push(Articulation { seconds: LET_GO_S, accent: -4.0, ..SILENT });
        self.phrase_length = self.script.iter().map(|a| a.seconds).sum();
        self.mark(Mark::Phrase);
        self.plan_pause(knobs, self.phrase_temper);
    }

    fn articulate_phones(&mut self, range: std::ops::Range<usize>, knobs: &SpeakerKnobs) {
        let temper = self.phrase_temper;
        let vowel_voicing = 1.0 - knobs.breath * 0.9;
        // A whispered tract is coupled to the open glottis, and rings far less.
        let vowel_width = 1.0 + 2.2 * knobs.breath;
        let mut creak_next = false;

        for i in range {
            let planned = self.planned[i];
            let p = phone(planned.phone);
            let seconds = self.duration(&planned, p, knobs, temper);
            let next_vowel = self.next_vowel(i);
            let locus = self.consonant_formants(p, next_vowel);
            let base = Articulation {
                formants: locus,
                accent: planned.accent,
                mark: Some(Mark::Phone(planned.phone)),
                stress_mark: planned.stressed && planned.syllable_start,
                word_mark: planned.word_start,
                long_mark: false,
                glide_s: 0.018,
                ..SILENT
            };
            match p.manner {
                Manner::Vowel => {
                    let rough = if creak_next { 0.9 } else { 0.0 } + if planned.final_syllable { 0.25 } else { 0.0 };
                    creak_next = false;
                    self.push(Articulation {
                        formants: self.vowel_formants(i, p),
                        glide_s: 0.028,
                        wide: vowel_width,
                        voicing: vowel_voicing,
                        gain: if planned.stressed { 1.0 } else { 0.72 },
                        rough,
                        seconds,
                        long_mark: planned.long,
                        ..base
                    });
                }
                Manner::Stop | Manner::Ejective => {
                    let ejective = p.manner == Manner::Ejective;
                    let bar = if p.voiced { 0.05 } else { 0.0 };
                    self.push(Articulation { voicing: 1.0, gain: bar, seconds: seconds * 0.8, ..base });
                    self.push(Articulation { formants: p.noise, glide_s: 0.002, wide: p.width, gain: p.gain, seconds: 0.012, mark: None, word_mark: false, stress_mark: false, ..base });
                    if ejective {
                        // The glottis is shut through the release, then opens with a catch.
                        self.push(Articulation { seconds: 0.03, mark: None, word_mark: false, stress_mark: false, ..base });
                        creak_next = true;
                    } else if !p.voiced {
                        let opening = next_vowel.map_or(locus, |v| v.formants);
                        self.push(Articulation { formants: opening, wide: ASPIRATION_WIDTH, gain: 0.14, seconds: 0.025, mark: None, word_mark: false, stress_mark: false, ..base });
                    }
                }
                Manner::Fricative => {
                    let source = if p.voiced { 0.55 } else { 0.0 };
                    let throat = p.backing && p.voiced;
                    self.push(Articulation {
                        formants: p.noise,
                        glide_s: 0.012,
                        wide: p.width,
                        voicing: source,
                        gain: p.gain,
                        rough: if throat { 0.7 } else { 0.0 },
                        seconds,
                        ..base
                    });
                }
                Manner::Nasal => self.push(Articulation { voicing: vowel_voicing, wide: 1.8 * vowel_width, gain: p.gain, seconds, ..base }),
                Manner::Approximant => self.push(Articulation { voicing: vowel_voicing, gain: p.gain, seconds, glide_s: 0.03, ..base }),
                Manner::Tap => {
                    self.push(Articulation { voicing: vowel_voicing, gain: p.gain * 0.35, seconds, ..base });
                }
                Manner::Trill => {
                    let beats = 2 + self.rng.below(3);
                    let each = seconds / (2 * beats) as f32;
                    for beat in 0..beats {
                        let (mark, word_mark, stress_mark) = if beat == 0 { (base.mark, base.word_mark, base.stress_mark) } else { (None, false, false) };
                        self.push(Articulation { voicing: vowel_voicing, gain: p.gain, rough: 0.3, seconds: each, mark, word_mark, stress_mark, ..base });
                        self.push(Articulation { voicing: vowel_voicing, gain: p.gain * 0.2, seconds: each, mark: None, word_mark: false, stress_mark: false, ..base });
                    }
                }
                Manner::GlottalStop => {
                    self.push(Articulation { seconds, ..base });
                    creak_next = true;
                }
            }
        }
    }

    fn plan_pause(&mut self, knobs: &SpeakerKnobs, temper: f32) {
        let mean = 0.16 + 9.0 * knobs.sparsity * knobs.sparsity * knobs.sparsity + 0.5 * knobs.sparsity;
        self.upcoming_pause = self.rng.exponential(mean).min(mean * 4.0);
        const INHALE: f32 = 0.3;
        let breath = if self.upcoming_pause > INHALE * 1.1 { self.rng.range(0.2, INHALE) } else { 0.0 };
        self.push(Articulation { seconds: (self.upcoming_pause - breath).max(0.02), ..SILENT });
        if breath > 0.0 {
            // A ragged little inhale, on a slack open tract.
            let open = phone(id("ə")).formants;
            self.push(Articulation { formants: open, wide: ASPIRATION_WIDTH, gain: 0.05 + 0.1 * temper, seconds: breath, glide_s: 0.05, ..SILENT });
        }
    }

    /// Moves on to the next phrase: its temper first, then what it says.
    fn next_phrase(&mut self, knobs: &SpeakerKnobs) {
        // A wandering value, so moods last a little and don't just jitter.
        self.temper_drift = (self.temper_drift * 0.7 + self.rng.signed() * 0.6).clamp(-1.0, 1.0);
        self.phrase_temper = (knobs.temper + 0.5 * knobs.swing * self.temper_drift).clamp(0.0, 1.0);
        self.plan(knobs);
        self.articulate(knobs);
    }

    /// Says the next syllable of the running stream, at this pitch and temper. The words
    /// still come from the language, with their stress and recurrences; only the timing
    /// and pitch are yours. Whatever it was saying stops where it is.
    pub fn say(&mut self, temper: f32, hz: f32, knobs: &SpeakerKnobs) {
        self.phrase_temper = temper.clamp(0.0, 1.0);
        self.pitch_hz = hz;
        if self.next_syllable >= self.planned.len() {
            if !self.planned.is_empty() {
                self.mark(Mark::Phrase);
            }
            self.plan(knobs);
            self.next_syllable = 0;
        }
        let start = self.next_syllable;
        let end = (start + 1..self.planned.len()).find(|i| self.planned[*i].syllable_start).unwrap_or(self.planned.len());
        self.next_syllable = end;
        self.clear_script();
        self.articulate_phones(start..end, knobs);
        self.push(Articulation { seconds: LET_GO_S, ..SILENT });
        self.phrase_length = self.script.iter().map(|a| a.seconds).sum();
    }

    /// Going to per-note mode shuts it up at once. Coming back, the next phrase just starts.
    fn change_mode(&mut self, per_note: bool) {
        self.per_note = per_note;
        self.planned.clear();
        self.next_syllable = 0;
        self.clear_script();
        self.push(Articulation { seconds: LET_GO_S, ..SILENT });
    }

    // ---- Moving the tract ----------------------------------------------------------

    /// Advances by `dt` seconds.
    pub fn step(&mut self, dt: f32, knobs: &SpeakerKnobs) {
        if knobs.per_note != self.per_note {
            self.change_mode(knobs.per_note);
        }
        self.left -= dt;
        while self.left <= 0.0 {
            if self.at >= self.script.len() {
                if self.per_note {
                    // Nothing to say until the next note. The last articulation was silence.
                    self.left = 1.0;
                    break;
                }
                self.next_phrase(knobs);
            }
            let articulation = self.script[self.at];
            self.at += 1;
            self.left += articulation.seconds.max(1e-3);
            if articulation.word_mark {
                self.mark(Mark::Word);
            }
            if articulation.stress_mark {
                self.mark(Mark::Stress);
            }
            if let Some(mark) = articulation.mark {
                self.mark(mark);
            }
            if articulation.long_mark {
                self.mark(Mark::Long);
            }
        }
        self.elapsed += dt;

        let current = self.script[self.at.saturating_sub(1)];
        let toward = |value: &mut f32, target: f32, seconds: f32| *value += (target - *value) * (1.0 - (-dt / seconds.max(1e-4)).exp());
        for i in 0..3 {
            toward(&mut self.formants_smoother[i], current.formants[i], current.glide_s * 0.5);
            let smoothed = self.formants_smoother[i];
            toward(&mut self.formants[i], smoothed, current.glide_s * 0.5);
        }
        toward(&mut self.gain, current.gain, GAIN_GLIDE_S);
        toward(&mut self.voicing, current.voicing, 0.006);
        toward(&mut self.accent, current.accent, ACCENT_GLIDE_S);
        toward(&mut self.wide, current.wide, 0.01);
        toward(&mut self.rough, current.rough, 0.02);
        // In per-note mode the pitch belongs to the last note, and it doesn't glide.
        if !self.per_note {
            toward(&mut self.pitch_hz, knobs.pitch_hz, 0.04);
        }
        toward(&mut self.wander, self.rng.signed() * 0.5, 0.2);
    }

    /// Temper right now: the phrase's, arching up toward its middle and falling away.
    fn temper_now(&self, knobs: &SpeakerKnobs) -> f32 {
        if self.per_note {
            return self.phrase_temper;
        }
        let progress = (self.elapsed / self.phrase_length.max(0.1)).min(1.0);
        let arch = (std::f32::consts::PI * progress.powf(0.8)).sin();
        (self.phrase_temper + knobs.swing * 0.45 * (arch - 0.45)).clamp(0.0, 1.0)
    }

    pub fn target(&self, knobs: &SpeakerKnobs) -> Target {
        let temper = self.temper_now(knobs);
        let range = 0.6 + 1.6 * temper;
        let declined = 2.0 - DECLINATION_ST_PER_S * self.elapsed.min(4.0);
        // Played from a keyboard it has to be in tune, so the prosody leaves pitch alone.
        let semitones = if self.per_note { 0.0 } else { declined + self.accent * range + TEMPER_PITCH_ST * (temper - 0.45) + self.wander };
        let high = HIGH_FORMANTS;
        let tract = |f: f32| f * knobs.tract;
        let formants = [
            tract(self.formants[0]),
            tract(self.formants[1]),
            tract(self.formants[2]),
            tract(high[0]),
            tract(high[1]),
            tract(high[2]),
        ];
        // A tense throat narrows the voice's resonances, and has no such effect on noise.
        let tense = 1.0 - 0.3 * temper * self.voicing;
        Target {
            formants,
            bandwidths: std::array::from_fn(|i| {
                // Broad means evenly broad: the first resonance's narrowness is a vowel's.
                let broad = ((self.wide - 1.0) / 3.0).clamp(0.0, 1.0);
                (BANDWIDTHS[i] + (NOISE_BANDWIDTH - BANDWIDTHS[i]).max(0.0) * broad) * self.wide * tense
            }),
            // The slope belongs to the vocal folds: noise made in the mouth doesn't have it.
            tilt: (0.97 - 0.3 * temper) * self.voicing.sqrt(),
            f0: self.pitch_hz * 2f32.powf(semitones / 12.0),
            voicing: self.voicing,
            gain: self.gain * (0.07 + 0.2 * temper + 0.07 * temper * temper),
            rough: (self.rough + (temper - 0.72).max(0.0) * 2.2).min(1.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DT: f32 = 0.005;

    fn knobs() -> SpeakerKnobs {
        SpeakerKnobs { rate: 5.0, sparsity: 0.2, temper: 0.5, swing: 0.5, breath: 0.1, tract: 1.0, pitch_hz: 90.0, per_note: false }
    }

    struct Listening {
        syllables: f32,
        silence: f32,
        pitch: Vec<f32>,
        gain: Vec<f32>,
    }

    fn listen(k: SpeakerKnobs, seconds: f32) -> Listening {
        let mut speaker = Speaker::new(5, 0.4);
        let steps = (seconds / DT) as usize;
        let (mut syllables, mut quiet, mut pitch, mut gain) = (0, 0, Vec::new(), Vec::new());
        for _ in 0..steps {
            speaker.step(DT, &k);
            for mark in speaker.take_marks() {
                if matches!(mark, Mark::Phone(p) if phone(*p).manner == Manner::Vowel) {
                    syllables += 1;
                }
            }
            let t = speaker.target(&k);
            if t.gain < 1e-3 {
                quiet += 1;
            } else if t.voicing > 0.5 {
                pitch.push(t.f0);
                gain.push(t.gain);
            }
        }
        Listening { syllables: syllables as f32 / seconds, silence: quiet as f32 / steps as f32, pitch, gain }
    }

    fn mean(values: &[f32]) -> f32 {
        values.iter().sum::<f32>() / values.len().max(1) as f32
    }

    #[test]
    fn it_speaks_at_about_the_rate_asked_for() {
        let slow = listen(SpeakerKnobs { rate: 3.0, sparsity: 0.0, ..knobs() }, 60.0).syllables;
        let fast = listen(SpeakerKnobs { rate: 9.0, sparsity: 0.0, ..knobs() }, 60.0).syllables;
        assert!(fast > slow * 1.6, "{slow} {fast}");
        assert!((2.0..6.0).contains(&slow) && (4.5..13.0).contains(&fast), "{slow} {fast}");
    }

    #[test]
    fn sparsity_means_silence() {
        let dense = listen(SpeakerKnobs { sparsity: 0.0, ..knobs() }, 120.0).silence;
        let sparse = listen(SpeakerKnobs { sparsity: 1.0, ..knobs() }, 120.0).silence;
        assert!(dense < 0.5, "{dense}");
        assert!(sparse > dense + 0.3, "{dense} {sparse}");
    }

    #[test]
    fn temper_raises_pitch_and_level() {
        let calm = listen(SpeakerKnobs { temper: 0.1, swing: 0.0, ..knobs() }, 60.0);
        let furious = listen(SpeakerKnobs { temper: 0.95, swing: 0.0, ..knobs() }, 60.0);
        assert!(mean(&furious.pitch) > mean(&calm.pitch) * 1.3);
        assert!(mean(&furious.gain) > mean(&calm.gain) * 2.0);
    }

    #[test]
    fn pitch_moves_like_speech_over_about_an_octave() {
        let heard = listen(knobs(), 60.0);
        let low = heard.pitch.iter().cloned().fold(f32::MAX, f32::min);
        let high = heard.pitch.iter().cloned().fold(0.0, f32::max);
        let octaves = (high / low).log2();
        assert!((0.6..2.5).contains(&octaves), "{octaves}");
    }

    #[test]
    fn swing_makes_temper_wander() {
        let spread = |swing: f32| {
            let heard = listen(SpeakerKnobs { swing, ..knobs() }, 90.0);
            let m = mean(&heard.gain);
            heard.gain.iter().map(|g| (g - m).abs()).sum::<f32>() / heard.gain.len() as f32 / m
        };
        assert!(spread(1.0) > spread(0.0) * 1.1);
    }

    #[test]
    fn the_transcript_contains_words_stress_and_phrases() {
        let mut speaker = Speaker::new(5, 0.4);
        let (mut words, mut stresses, mut phrases) = (0, 0, 0);
        for _ in 0..(60.0 / DT) as usize {
            speaker.step(DT, &knobs());
            for mark in speaker.take_marks() {
                match mark {
                    Mark::Word => words += 1,
                    Mark::Stress => stresses += 1,
                    Mark::Phrase => phrases += 1,
                    _ => {}
                }
            }
        }
        assert!(words > 30 && stresses > 30 && phrases > 5, "{words} {stresses} {phrases}");
    }

    #[test]
    fn no_phrase_outgrows_its_buffers_on_any_language() {
        for seed in 0..60 {
            let mut speaker = Speaker::new(seed, 0.4);
            let k = SpeakerKnobs { rate: 14.0, sparsity: 0.0, ..knobs() };
            for _ in 0..4_000 {
                speaker.step(DT, &k);
                speaker.take_marks();
            }
            assert!(speaker.planned.capacity() == MAX_PLANNED && speaker.script.capacity() == MAX_ARTICULATIONS);
        }
    }

    /// The filter a phone ends up as, after the codec: its power at many frequencies.
    fn decoded_response(symbol: &str, tract: f32, rate: f32, frequencies: &[f32]) -> Vec<f32> {
        use crate::dsp::codec::{decode, encode, ORDER};
        use crate::dsp::phones::id;
        let mut speaker = Speaker::new(5, 0.4);
        let k = SpeakerKnobs { tract, ..knobs() };
        speaker.planned.clear();
        speaker.planned.push(Planned { phone: id(symbol), stressed: true, long: false, final_syllable: false, accent: 0.0, word_start: true, syllable_start: true });
        speaker.articulate(&k);
        // Sit in the middle of it: the one articulation with this phone's noise resonances.
        let noisy = speaker.script.iter().position(|a| a.gain > 0.0).expect("it makes a sound");
        speaker.script[noisy].seconds = 10.0;
        speaker.at = noisy;
        speaker.left = 0.0;
        for _ in 0..200 {
            speaker.step(DT, &k);
        }
        let frame = decode(&encode(&speaker.target(&k), rate));
        let reflections: [f32; ORDER] = std::array::from_fn(|i| (frame.ratios[i] * 0.5).tanh());
        // Step the lattice back up to a polynomial.
        let mut a = [0.0f32; ORDER + 1];
        a[0] = 1.0;
        for (m, k) in reflections.iter().enumerate() {
            let before = a;
            for i in 1..=m {
                a[i] = before[i] + k * before[m + 1 - i];
            }
            a[m + 1] = *k;
        }
        frequencies
            .iter()
            .map(|hz| {
                let w = 2.0 * std::f32::consts::PI * hz / rate;
                let (mut re, mut im) = (0.0, 0.0);
                for (i, c) in a.iter().enumerate() {
                    re += c * (w * i as f32).cos();
                    im -= c * (w * i as f32).sin();
                }
                1.0 / (re * re + im * im).max(1e-12)
            })
            .collect()
    }

    /// The width of the sharpest peak in a response, at half power.
    fn sharpest_peak_hz(response: &[f32], step: f32) -> f32 {
        let mut sharpest = f32::MAX;
        for i in 1..response.len() - 1 {
            if response[i] > response[i - 1] && response[i] >= response[i + 1] {
                let half = response[i] / 2.0;
                let left = (0..i).rev().find(|j| response[*j] < half);
                let right = (i + 1..response.len()).find(|j| response[*j] < half);
                if let (Some(l), Some(r)) = (left, right) {
                    sharpest = sharpest.min((r - l) as f32 * step);
                }
            }
        }
        sharpest
    }

    #[test]
    fn frication_has_broad_humps_and_vowels_keep_their_narrow_formants() {
        const STEP: f32 = 10.0;
        let frequencies = |rate: f32| -> Vec<f32> { (5..(0.47 * rate / STEP) as usize).map(|i| i as f32 * STEP).collect() };
        for rate in [7_000.0, 10_300.0, 16_000.0] {
            let band = frequencies(rate);
            for tract in [0.6, 0.83, 1.2] {
                for symbol in ["ʃ", "ħ", "ʕ", "θ", "s", "z", "f", "v", "x", "χ", "ʁ", "h", "sˤ"] {
                    let width = sharpest_peak_hz(&decoded_response(symbol, tract, rate, &band), STEP);
                    assert!(width >= 300.0, "{symbol} at {rate} Hz, tract {tract}: {width} Hz");
                }
            }
            let width = sharpest_peak_hz(&decoded_response("a", 0.83, rate, &band), STEP);
            assert!(width < 200.0, "a at {rate}: {width} Hz");
        }
    }

    fn per_note() -> SpeakerKnobs {
        SpeakerKnobs { per_note: true, ..knobs() }
    }

    /// What one note sounds like: its vowels, how loud at its loudest, its pitch.
    fn note(speaker: &mut Speaker, temper: f32, hz: f32, seconds: f32) -> (usize, f32, f32) {
        let k = per_note();
        speaker.step(DT, &k);
        speaker.take_marks();
        speaker.say(temper, hz, &k);
        let (mut vowels, mut loudest, mut pitch) = (0, 0.0f32, 0.0);
        for _ in 0..(seconds / DT) as usize {
            speaker.step(DT, &k);
            for mark in speaker.take_marks() {
                if matches!(mark, Mark::Phone(p) if phone(*p).manner == Manner::Vowel) {
                    vowels += 1;
                }
            }
            let t = speaker.target(&k);
            loudest = loudest.max(t.gain);
            pitch = t.f0;
        }
        (vowels, loudest, pitch)
    }

    #[test]
    fn per_note_is_silent_until_told_and_silent_again_after() {
        let (mut speaker, k) = (Speaker::new(5, 0.4), per_note());
        for _ in 0..2_000 {
            speaker.step(DT, &k);
            assert!(speaker.target(&k).gain < 1e-3);
        }
        note(&mut speaker, 0.6, 110.0, 2.0);
        assert!(speaker.target(&k).gain < 1e-3);
    }

    #[test]
    fn every_note_is_exactly_one_syllable_at_exactly_its_pitch() {
        let mut speaker = Speaker::new(5, 0.4);
        for i in 0..200 {
            let hz = 80.0 + i as f32;
            let (vowels, loudest, pitch) = note(&mut speaker, 0.6, hz, 1.5);
            assert_eq!(vowels, 1, "note {i}");
            assert!(loudest > 0.01, "note {i}");
            assert!((pitch - hz).abs() < hz * 0.001, "{pitch} {hz}");
        }
    }

    #[test]
    fn velocity_is_temper() {
        let mut speaker = Speaker::new(5, 0.4);
        // Hit the same vowels at both levels by averaging over plenty of syllables.
        let level = |speaker: &mut Speaker, temper| (0..100).map(|_| note(speaker, temper, 100.0, 1.0).1).sum::<f32>();
        let (soft, hard) = (level(&mut speaker, 0.1), level(&mut speaker, 1.0));
        assert!(hard > soft * 2.0, "{soft} {hard}");
    }

    #[test]
    fn a_new_note_cuts_off_the_last_one() {
        let (mut speaker, k) = (Speaker::new(5, 0.4), per_note());
        speaker.step(DT, &k);
        speaker.say(0.5, 100.0, &k);
        speaker.step(DT, &k);
        speaker.say(0.5, 200.0, &k);
        assert_eq!(speaker.at, 0);
        speaker.step(DT, &k);
        assert_eq!(speaker.target(&k).f0, 200.0);
    }

    #[test]
    fn the_words_still_recur_through_the_stream_of_notes() {
        let mut speaker = Speaker::new(5, 0.4);
        let k = per_note();
        let (mut words, mut phrases) = (0, 0);
        for _ in 0..300 {
            speaker.step(DT, &k);
            speaker.say(0.5, 100.0, &k);
            for _ in 0..50 {
                speaker.step(DT, &k);
                for mark in speaker.take_marks() {
                    match mark {
                        Mark::Word => words += 1,
                        Mark::Phrase => phrases += 1,
                        _ => {}
                    }
                }
            }
        }
        assert!(words > 30 && phrases > 5, "{words} {phrases}");
    }

    #[test]
    fn it_goes_back_to_babbling_when_the_mode_is_switched_off() {
        let (mut speaker, on, off) = (Speaker::new(5, 0.4), per_note(), knobs());
        for _ in 0..400 {
            speaker.step(DT, &on);
        }
        assert!(speaker.target(&on).gain < 1e-3);
        let mut spoke = false;
        for _ in 0..1_000 {
            speaker.step(DT, &off);
            spoke |= speaker.target(&off).gain > 0.01;
        }
        assert!(spoke);
    }
}
