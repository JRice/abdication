//! What the audio thread tells the panel, without locks.

use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Default)]
pub struct Telemetry {
    reduction_db: AtomicU32,
}

impl Telemetry {
    /// Once per audio block.
    pub fn publish(&self, reduction_db: f32) {
        // Positive floats order like their bits, so fetch_max keeps the deepest.
        self.reduction_db.fetch_max(reduction_db.max(0.0).to_bits(), Ordering::Relaxed);
    }

    /// The deepest limiting since the panel last asked.
    pub fn take_reduction_db(&self) -> f32 {
        f32::from_bits(self.reduction_db.swap(0, Ordering::Relaxed))
    }
}
