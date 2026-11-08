//! What the audio thread tells the panel, without locks.

use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Default)]
pub struct Telemetry {
    reduction_db: AtomicU32,
    agitation_phase: AtomicU32,
    interference: AtomicU32,
}

impl Telemetry {
    /// Once per audio block.
    pub fn publish(&self, reduction_db: f32, agitation_phase: f32, interference: f32) {
        // Positive floats order like their bits, so fetch_max keeps the deepest.
        self.reduction_db.fetch_max(reduction_db.max(0.0).to_bits(), Ordering::Relaxed);
        self.agitation_phase.store(agitation_phase.to_bits(), Ordering::Relaxed);
        self.interference.store(interference.to_bits(), Ordering::Relaxed);
    }

    /// The deepest limiting since the panel last asked.
    pub fn take_reduction_db(&self) -> f32 {
        f32::from_bits(self.reduction_db.swap(0, Ordering::Relaxed))
    }

    pub fn agitation_phase(&self) -> f32 {
        f32::from_bits(self.agitation_phase.load(Ordering::Relaxed))
    }

    pub fn interference(&self) -> f32 {
        f32::from_bits(self.interference.load(Ordering::Relaxed))
    }
}
