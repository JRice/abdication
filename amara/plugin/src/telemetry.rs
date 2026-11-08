//! What the audio thread tells the panel, without locks.

use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU8, Ordering};

use crate::dsp::speaker::Mark;

const MARKS: usize = 256;
const LONG: u8 = 250;
const STRESS: u8 = 251;
const WORD: u8 = 252;
const PHRASE: u8 = 253;

fn encode(mark: Mark) -> u8 {
    match mark {
        Mark::Phone(id) => id,
        Mark::Long => LONG,
        Mark::Stress => STRESS,
        Mark::Word => WORD,
        Mark::Phrase => PHRASE,
    }
}

fn decode(code: u8) -> Mark {
    match code {
        LONG => Mark::Long,
        STRESS => Mark::Stress,
        WORD => Mark::Word,
        PHRASE => Mark::Phrase,
        id => Mark::Phone(id),
    }
}

pub struct Telemetry {
    reduction_db: AtomicU32,
    possessed: AtomicBool,
    marks: [AtomicU8; MARKS],
    written: AtomicU32,
}

impl Default for Telemetry {
    fn default() -> Self {
        Self {
            reduction_db: AtomicU32::new(0),
            possessed: AtomicBool::new(false),
            marks: std::array::from_fn(|_| AtomicU8::new(0)),
            written: AtomicU32::new(0),
        }
    }
}

impl Telemetry {
    /// Once per audio block.
    pub fn publish(&self, reduction_db: f32, possessed: bool) {
        // Positive floats order like their bits, so fetch_max keeps the deepest.
        self.reduction_db.fetch_max(reduction_db.max(0.0).to_bits(), Ordering::Relaxed);
        self.possessed.store(possessed, Ordering::Relaxed);
    }

    pub fn push(&self, mark: Mark) {
        let written = self.written.load(Ordering::Relaxed);
        self.marks[written as usize % MARKS].store(encode(mark), Ordering::Relaxed);
        self.written.store(written.wrapping_add(1), Ordering::Release);
    }

    /// The deepest limiting since the panel last asked.
    pub fn take_reduction_db(&self) -> f32 {
        f32::from_bits(self.reduction_db.swap(0, Ordering::Relaxed))
    }

    pub fn possessed(&self) -> bool {
        self.possessed.load(Ordering::Relaxed)
    }

    /// Everything written since `seen`, moving it on. A panel that fell far behind just
    /// misses some: it's a transcript, not a record.
    pub fn read_since(&self, seen: &mut u32, mut each: impl FnMut(Mark)) {
        let written = self.written.load(Ordering::Acquire);
        let behind = written.wrapping_sub(*seen).min(MARKS as u32);
        for i in (0..behind).rev() {
            let at = written.wrapping_sub(i + 1) as usize % MARKS;
            each(decode(self.marks[at].load(Ordering::Relaxed)));
        }
        *seen = written;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_panel_reads_marks_in_order_and_only_once() {
        let telemetry = Telemetry::default();
        let sequence = [Mark::Word, Mark::Stress, Mark::Phone(3), Mark::Long, Mark::Phrase];
        sequence.iter().for_each(|m| telemetry.push(*m));
        let (mut seen, mut read) = (0, Vec::new());
        telemetry.read_since(&mut seen, |m| read.push(m));
        assert_eq!(read, sequence);
        telemetry.read_since(&mut seen, |_| panic!("already read"));
    }

    #[test]
    fn falling_behind_loses_the_oldest() {
        let telemetry = Telemetry::default();
        (0..MARKS + 10).for_each(|i| telemetry.push(Mark::Phone((i % 40) as u8)));
        let (mut seen, mut read) = (0, Vec::new());
        telemetry.read_since(&mut seen, |m| read.push(m));
        assert_eq!(read.len(), MARKS);
        assert_eq!(read[read.len() - 1], Mark::Phone(((MARKS + 9) % 40) as u8));
    }
}
