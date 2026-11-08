//! What the audio thread tells the panel, without locks: the newest voice's modulated
//! controls and where its oscillators are in their turns and orbits.

use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use crate::dsp::engine::Shown;
use crate::dsp::targets::{OSCS, TARGETS};

pub struct Telemetry {
    sounding: AtomicBool,
    values: [AtomicU32; TARGETS],
    orbit: [AtomicU32; OSCS],
    turn: [AtomicU32; OSCS],
}

impl Default for Telemetry {
    fn default() -> Self {
        Self {
            sounding: AtomicBool::new(false),
            values: std::array::from_fn(|_| AtomicU32::new(0)),
            orbit: std::array::from_fn(|_| AtomicU32::new(0)),
            turn: std::array::from_fn(|_| AtomicU32::new(0)),
        }
    }
}

fn store(slot: &AtomicU32, v: f32) {
    slot.store(v.to_bits(), Ordering::Relaxed);
}

fn load(slot: &AtomicU32) -> f32 {
    f32::from_bits(slot.load(Ordering::Relaxed))
}

impl Telemetry {
    /// Once per audio buffer.
    pub fn publish(&self, shown: &Shown) {
        self.sounding.store(shown.sounding, Ordering::Relaxed);
        for (slot, v) in self.values.iter().zip(&shown.values) {
            store(slot, *v);
        }
        for o in 0..OSCS {
            store(&self.orbit[o], shown.orbit[o]);
            store(&self.turn[o], shown.turn[o]);
        }
    }

    pub fn snapshot(&self) -> Shown {
        Shown {
            sounding: self.sounding.load(Ordering::Relaxed),
            values: std::array::from_fn(|t| load(&self.values[t])),
            orbit: std::array::from_fn(|o| load(&self.orbit[o])),
            turn: std::array::from_fn(|o| load(&self.turn[o])),
        }
    }
}
