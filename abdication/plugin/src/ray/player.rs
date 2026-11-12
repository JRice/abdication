//! One voice's Ray: the oscillator slots set to Ray, played for each unison
//! copy that runs. The Faust copy (plugin-osc.dsp) gives each slot's pitch,
//! the copy's unison level and start phase and the note-on; the slot's
//! controls are read from the copy's DSP, where the engine has set them for
//! this voice (modulated). Ported from Raycaster's engine.

use std::sync::OnceLock;

use super::geometry::{Bend, Outline};
use super::random::Rng;
use super::rayosc::{OscBlock, RayOsc};
use super::shapes::{self, Element, ELEMENTS};

/// The slot modes (voice.lib's Mode menu): Ray is the plugin's.
pub const MODE_RAY: f32 = 2.0;

/// What a slot's Ray reads (params::RAY_FIELDS, in that order).
#[derive(Clone, Copy, Debug, Default)]
pub struct RaySettings {
    pub mode: f32,
    pub fader: f32,
    pub element: f32,
    pub seed: f32,
    pub free: f32,
    pub squish: f32,
    pub skew: f32,
    pub tilt: f32,
    pub center: f32,
    pub ray: f32,
    pub orbit: f32,
    pub orbit_hz: f32,
    pub fine: f32,
}

impl RaySettings {
    /// From the values of params::RAY_FIELDS, in order.
    pub fn from_values(v: [f32; 13]) -> Self {
        let [mode, fader, element, seed, free, squish, skew, tilt, center, ray, orbit, orbit_hz, fine] = v;
        Self { mode, fader, element, seed, free, squish, skew, tilt, center, ray, orbit, orbit_hz, fine }
    }

    /// The slot plays Ray, and sounds.
    pub fn playing(&self) -> bool {
        self.mode == MODE_RAY && self.fader != 0.0
    }
}

/// A Ray's raw wave is quieter than the other engines' (a turning shape
/// rarely spans its whole reach): this brings an off-centre Fire, Raycaster's
/// second oscillator's default, to about a saw's level in the mixer.
pub const RAY_GAIN: f32 = 2.0;

/// The six elements' outlines, made once.
fn elements() -> &'static [Outline] {
    static OUTLINES: OnceLock<Vec<Outline>> = OnceLock::new();
    OUTLINES.get_or_init(|| ELEMENTS.iter().map(|e| e.outline()).collect())
}

/// Never a key a shape has (see `RaySlot::primed`).
const UNPRIMED: u32 = u32::MAX;

#[derive(Default)]
struct RaySlot {
    osc: RayOsc,
    /// Which shape the oscillator's past was primed for: an element's index,
    /// 64 plus a count of Chaos outlines, or UNPRIMED.
    primed: u32,
    /// Turns per sample at the end of the last block (0 before the first).
    step: f32,
}

/// One voice's Ray oscillators: per unison copy, per slot.
pub struct RayPlayer {
    slots: Vec<[RaySlot; 2]>,
    /// Each slot's Chaos outline, for the seed it was made from, and how many
    /// have been made (so an oscillator can tell its shape changed).
    chaos: [Option<(u32, Outline)>; 2],
    chaos_count: [u32; 2],
    rng: Rng,
    rate: f32,
    right: Vec<f32>,
}

impl RayPlayer {
    pub fn new(copies: usize, seed: u32) -> Self {
        Self {
            slots: (0..copies).map(|_| [RaySlot { primed: UNPRIMED, ..Default::default() },
                                         RaySlot { primed: UNPRIMED, ..Default::default() }]).collect(),
            chaos: [None, None],
            chaos_count: [0; 2],
            rng: Rng::new(seed),
            rate: 48_000.0,
            right: Vec::new(),
        }
    }

    pub fn init(&mut self, rate: f32, max_block: usize) {
        self.rate = rate;
        self.right = vec![0.0; max_block];
        self.clear();
    }

    pub fn clear(&mut self) {
        for slot in self.slots.iter_mut().flatten() {
            slot.osc.clear();
            slot.primed = UNPRIMED;
            slot.step = 0.0;
        }
    }

    /// Make sure slot `s`'s shape is ready (a Chaos outline is made when its
    /// seed changes); its key for `RaySlot::primed`.
    pub fn prepare(&mut self, s: usize, settings: &RaySettings) -> u32 {
        match Element::from_index(settings.element as usize) {
            Element::Chaos => {
                let seed = settings.seed.max(1.0) as u32;
                if self.chaos[s].as_ref().map(|(made, _)| *made) != Some(seed) {
                    self.chaos[s] = Some((seed, shapes::chaos(seed).0));
                    self.chaos_count[s] = self.chaos_count[s].wrapping_add(1) % 1_000_000;
                }
                64 + self.chaos_count[s]
            }
            e => e.index() as u32,
        }
    }

    /// Play slot `s` of unison copy `copy` for one block into `out` (replacing
    /// it): `hz` is the slot's pitch at the block's end, `phase` the copy's
    /// start phase, `note_on` whether a note started in this block.
    #[allow(clippy::too_many_arguments)]
    pub fn render(&mut self, copy: usize, s: usize, key: u32, settings: &RaySettings, hz: f32, phase: f32,
                  note_on: bool, out: &mut [f32]) {
        let n = out.len();
        let element = Element::from_index(settings.element as usize);
        let outline = match element {
            Element::Chaos => &self.chaos[s].as_ref().expect("prepared").1,
            e => &elements()[e.index()],
        };
        let slot = &mut self.slots[copy][s];
        if note_on {
            // Free: the shape (and its orbit) start anywhere; otherwise
            // upright, or at the copy's unison phase.
            if settings.free > 0.5 {
                slot.osc.set_phase(self.rng.unit(), self.rng.unit());
            } else {
                slot.osc.set_phase(phase, 0.0);
            }
            slot.osc.clear();
            slot.primed = UNPRIMED;
            slot.step = 0.0;
        }
        let step = hz * (settings.fine / 1200.0).exp2() / self.rate;
        let start = if slot.step > 0.0 { slot.step } else { step };
        let block = OscBlock {
            bend: Bend { squish: settings.squish, skew: settings.skew, tilt: settings.tilt },
            center_x: settings.center,
            probe: settings.ray,
            orbit_depth: settings.orbit,
            orbit_step: settings.orbit_hz / self.rate,
            step: [start, step],
            width_turn: 0.0,
            level: [RAY_GAIN, RAY_GAIN],
        };
        // A new note, or a new shape under a held one: its past is the wrong wave.
        if slot.primed != key {
            slot.osc.prime(outline, &block);
            slot.primed = key;
        }
        out.fill(0.0);
        let right = &mut self.right[..n];
        right.fill(0.0);
        slot.osc.render(outline, &block, out, right);
        slot.step = step;
    }
}
