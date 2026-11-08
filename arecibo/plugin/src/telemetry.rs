//! What the audio thread tells the panel, without locks.

use std::sync::atomic::{AtomicU32, Ordering};

use crate::dsp::wear::CHANNELS;

/// A little over what the waterfall reads at a time.
const SAMPLES: usize = 2_048;

pub struct Telemetry {
    reduction_db: AtomicU32,
    agc_gains: [AtomicU32; CHANNELS],
    samples: [AtomicU32; SAMPLES],
    written: AtomicU32,
}

impl Default for Telemetry {
    fn default() -> Self {
        Self {
            reduction_db: AtomicU32::new(0),
            agc_gains: std::array::from_fn(|_| AtomicU32::new(1f32.to_bits())),
            samples: std::array::from_fn(|_| AtomicU32::new(0)),
            written: AtomicU32::new(0),
        }
    }
}

impl Telemetry {
    /// Once per audio block.
    pub fn publish(&self, reduction_db: f32, agc_gains: [f32; CHANNELS]) {
        // Positive floats order like their bits, so fetch_max keeps the deepest.
        self.reduction_db.fetch_max(reduction_db.max(0.0).to_bits(), Ordering::Relaxed);
        for (slot, gain) in self.agc_gains.iter().zip(agc_gains) {
            slot.store(gain.to_bits(), Ordering::Relaxed);
        }
    }

    /// For the waterfall.
    #[inline]
    pub fn push_sample(&self, sample: f32) {
        let written = self.written.load(Ordering::Relaxed);
        self.samples[written as usize % SAMPLES].store(sample.to_bits(), Ordering::Relaxed);
        self.written.store(written.wrapping_add(1), Ordering::Release);
    }

    pub fn take_reduction_db(&self) -> f32 {
        f32::from_bits(self.reduction_db.swap(0, Ordering::Relaxed))
    }

    pub fn agc_gain(&self, channel: usize) -> f32 {
        f32::from_bits(self.agc_gains[channel].load(Ordering::Relaxed))
    }

    /// The newest samples, oldest first. It may tear across a block now and then, which
    /// on a waterfall nobody could see.
    pub fn latest(&self, into: &mut [f32]) {
        let written = self.written.load(Ordering::Acquire) as usize;
        let count = into.len();
        for (i, slot) in into.iter_mut().enumerate() {
            let at = (written + SAMPLES * 2 - count + i) % SAMPLES;
            *slot = f32::from_bits(self.samples[at].load(Ordering::Relaxed));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn latest_comes_out_in_order() {
        let telemetry = Telemetry::default();
        (0..5_000).for_each(|i| telemetry.push_sample(i as f32));
        let mut out = [0.0; 4];
        telemetry.latest(&mut out);
        assert_eq!(out, [4_996.0, 4_997.0, 4_998.0, 4_999.0]);
    }
}
