//! What the audio thread tells the panel: the modulators' values (all eight: silver,
//! green and the three copies each of red and slate), where they have moved each armed
//! knob, the Divisions as they're kept apart, and each oscillator's last cycle.

use std::sync::atomic::{AtomicU32, Ordering};

use crate::dsp::mods::Values;
use crate::engine::{Report, SCOPE};
use crate::knobs::ARMED_COUNT;

fn store(a: &AtomicU32, v: f32) {
    a.store(v.to_bits(), Ordering::Relaxed);
}

fn load(a: &AtomicU32) -> f32 {
    f32::from_bits(a.load(Ordering::Relaxed))
}

pub struct Telemetry {
    mods: [AtomicU32; 8],
    scopes: [[AtomicU32; SCOPE]; 3],
    moved: [AtomicU32; ARMED_COUNT],
    divisions: [AtomicU32; 3],
    /// Bumped with every report; 0 until the audio has run.
    pub reports: AtomicU32,
}

impl Default for Telemetry {
    fn default() -> Self {
        Self {
            mods: Default::default(),
            scopes: std::array::from_fn(|_| std::array::from_fn(|_| AtomicU32::new(0))),
            moved: std::array::from_fn(|_| AtomicU32::new(0)),
            divisions: Default::default(),
            reports: AtomicU32::new(0),
        }
    }
}

impl Telemetry {
    pub fn publish(&self, r: &Report) {
        r.mods.flat().iter().zip(&self.mods).for_each(|(v, a)| store(a, *v));
        for (scope, atoms) in r.scopes.iter().zip(&self.scopes) {
            scope.iter().zip(atoms).for_each(|(v, a)| store(a, *v));
        }
        r.moved.iter().zip(&self.moved).for_each(|(v, a)| store(a, *v));
        r.divisions.iter().zip(&self.divisions).for_each(|(v, a)| a.store(*v as u32, Ordering::Relaxed));
        self.reports.fetch_add(1, Ordering::Relaxed);
    }

    pub fn mods(&self) -> Values {
        Values::from_flat(std::array::from_fn(|m| load(&self.mods[m])))
    }

    /// Oscillator `o`'s last cycle.
    pub fn scope(&self, o: usize) -> [f32; SCOPE] {
        std::array::from_fn(|i| load(&self.scopes[o][i]))
    }

    pub fn moved(&self, i: usize) -> f32 {
        load(&self.moved[i])
    }

    pub fn division(&self, e: usize) -> usize {
        self.divisions[e].load(Ordering::Relaxed) as usize
    }

    pub fn running(&self) -> bool {
        self.reports.load(Ordering::Relaxed) > 0
    }
}
