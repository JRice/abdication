//! What the audio thread tells the panel, without locks: the frame being played, as a
//! trace, and where in the bank it is.

use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use crate::dsp::engine::Shown;

pub const TRACE: usize = 256;

pub struct Telemetry {
    trace: [AtomicU32; TRACE],
    table: AtomicU32,
    wave: AtomicU32,
    frame: AtomicU32,
    frames: AtomicU32,
    sounding: AtomicBool,
}

impl Default for Telemetry {
    fn default() -> Self {
        Self {
            trace: std::array::from_fn(|_| AtomicU32::new(0)),
            table: AtomicU32::new(0),
            wave: AtomicU32::new(0),
            frame: AtomicU32::new(0),
            frames: AtomicU32::new(64),
            sounding: AtomicBool::new(false),
        }
    }
}

/// A copy for drawing.
#[derive(Clone)]
pub struct Snapshot {
    pub trace: [f32; TRACE],
    pub shown: Shown,
}

impl Telemetry {
    /// Once per audio block.
    pub fn publish(&self, trace: &[f32; TRACE], shown: Shown) {
        for (slot, v) in self.trace.iter().zip(trace) {
            slot.store(v.to_bits(), Ordering::Relaxed);
        }
        self.table.store(shown.table.to_bits(), Ordering::Relaxed);
        self.wave.store(shown.wave.to_bits(), Ordering::Relaxed);
        self.frame.store(shown.frame as u32, Ordering::Relaxed);
        self.frames.store(shown.frames as u32, Ordering::Relaxed);
        self.sounding.store(shown.sounding, Ordering::Relaxed);
    }

    pub fn snapshot(&self) -> Snapshot {
        Snapshot {
            trace: std::array::from_fn(|i| f32::from_bits(self.trace[i].load(Ordering::Relaxed))),
            shown: Shown {
                table: f32::from_bits(self.table.load(Ordering::Relaxed)),
                wave: f32::from_bits(self.wave.load(Ordering::Relaxed)),
                frame: self.frame.load(Ordering::Relaxed) as usize,
                frames: self.frames.load(Ordering::Relaxed) as usize,
                sounding: self.sounding.load(Ordering::Relaxed),
            },
        }
    }
}
