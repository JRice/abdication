//! The running transcription, a few lines of IPA with the newest at the bottom. Whatever
//! was said under possession stays marked.

use crate::dsp::phones::phone;
use crate::dsp::speaker::Mark;

pub const LINES: usize = 3;
/// Start a new line at the next word after this many characters, or anyway at the limit.
const SOFT_WRAP: usize = 34;
const HARD_WRAP: usize = 52;

pub struct Run {
    pub text: String,
    pub possessed: bool,
}

#[derive(Default)]
struct Line {
    runs: Vec<Run>,
    characters: usize,
}

impl Line {
    fn push(&mut self, text: &str, possessed: bool) {
        self.characters += text.chars().count();
        match self.runs.last_mut() {
            Some(run) if run.possessed == possessed => run.text.push_str(text),
            _ => self.runs.push(Run { text: text.to_string(), possessed }),
        }
    }
}

pub struct Transcript {
    lines: Vec<Line>,
}

impl Transcript {
    pub fn new() -> Self {
        Self { lines: (0..LINES).map(|_| Line::default()).collect() }
    }

    pub fn lines(&self) -> impl Iterator<Item = &[Run]> {
        self.lines.iter().map(|line| line.runs.as_slice())
    }

    fn newest(&mut self) -> &mut Line {
        self.lines.last_mut().expect("there are always lines")
    }

    fn scroll(&mut self) {
        self.lines.remove(0);
        self.lines.push(Line::default());
    }

    pub fn add(&mut self, mark: Mark, possessed: bool) {
        let characters = self.newest().characters;
        match mark {
            Mark::Word | Mark::Phrase if characters >= SOFT_WRAP => self.scroll(),
            _ if characters >= HARD_WRAP => self.scroll(),
            _ => {}
        }
        let is_first = self.newest().characters == 0;
        match mark {
            Mark::Phone(id) => self.newest().push(phone(id).symbol, possessed),
            Mark::Long => self.newest().push("ː", possessed),
            Mark::Stress => self.newest().push("ˈ", possessed),
            Mark::Word if !is_first => self.newest().push(" ", possessed),
            Mark::Word => {}
            Mark::Phrase if is_first => self.newest().push("‖ ", possessed),
            Mark::Phrase => self.newest().push(" ‖ ", possessed),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text(transcript: &Transcript) -> Vec<String> {
        transcript.lines().map(|runs| runs.iter().map(|r| r.text.as_str()).collect()).collect()
    }

    #[test]
    fn it_writes_ipa_with_stress_length_and_boundaries() {
        let mut t = Transcript::new();
        for mark in [Mark::Word, Mark::Stress, Mark::Phone(3), Mark::Long, Mark::Word, Mark::Phone(0), Mark::Phrase] {
            t.add(mark, false);
        }
        assert_eq!(text(&t)[LINES - 1], "ˈaː i ‖ ");
    }

    #[test]
    fn possessed_stretches_become_their_own_runs() {
        let mut t = Transcript::new();
        t.add(Mark::Phone(0), false);
        t.add(Mark::Phone(1), true);
        t.add(Mark::Phone(2), true);
        let line: Vec<&[Run]> = t.lines().collect();
        assert_eq!(line[LINES - 1].len(), 2);
        assert!(line[LINES - 1][1].possessed);
    }

    #[test]
    fn it_wraps_at_words_and_keeps_only_the_last_lines() {
        let mut t = Transcript::new();
        for _ in 0..400 {
            t.add(Mark::Word, false);
            t.add(Mark::Phone(3), false);
        }
        assert!(text(&t).iter().all(|line| line.chars().count() <= HARD_WRAP + 3));
        assert_eq!(t.lines().count(), LINES);
    }
}
