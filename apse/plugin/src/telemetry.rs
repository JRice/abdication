//! What the audio thread tells the panel, without locks.

use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Default)]
pub struct Telemetry {
    wet_peak: AtomicU32,
    reduction_db: AtomicU32,
}

/// Positive floats order like their bits, so fetch_max keeps the largest.
fn raise(slot: &AtomicU32, value: f32) {
    slot.fetch_max(value.max(0.0).to_bits(), Ordering::Relaxed);
}

fn take(slot: &AtomicU32) -> f32 {
    f32::from_bits(slot.swap(0, Ordering::Relaxed))
}

impl Telemetry {
    /// Once per audio block.
    pub fn publish(&self, wet_peak: f32, reduction_db: f32) {
        raise(&self.wet_peak, wet_peak);
        raise(&self.reduction_db, reduction_db);
    }

    /// The loudest wet sample since the panel last asked.
    pub fn take_wet_peak(&self) -> f32 {
        take(&self.wet_peak)
    }

    /// The deepest limiting since the panel last asked.
    pub fn take_reduction_db(&self) -> f32 {
        take(&self.reduction_db)
    }
}
