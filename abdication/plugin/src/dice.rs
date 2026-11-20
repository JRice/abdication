//! The dice: a random patch that's meant to be playable. It first picks what
//! kind of sound to make, and that sets the ranges for the envelope and filter.
//! The rest follows what a person programming that sound would do: most
//! modulators stay off, and those that don't land on musical values. Controls
//! that wouldn't be heard are left at rest.
//!
//! Performance setup (Perform and its Global Pitch's modulators, Volume,
//! Portamento) and the three Mutes are left as they are. No UI here: `roll`
//! returns plain values, so it can be tested.

use crate::modulation::{self, ModSettings};
use crate::params::mod_slug;
use crate::pitch;
use crate::wavelib::WAVE_SETS;
#[cfg(test)]
use crate::wavelib::{BASIC_BASE, WAVE_COUNT};

/// Parameters a roll leaves alone (unison's Eco among them: it's a CPU choice).
pub fn kept(id: &str) -> bool {
    id.starts_with("perform.") || id.starts_with("mod.perform_") || id.starts_with("settings.") || id == "master.volume" || id == "master.portamento" || id.ends_with(".mute")
        || id == "unison.eco"
}

/// The init patch (the panel's ∅): every parameter at its default, then
/// these, whatever the defaults are: Osc 1 a Dual's plain saw (VA Sweep at
/// A 0.5, no sub) alone, and every other source and module off.
pub const INIT: [(&str, f32); 12] = [
    ("osc_1.mode", 0.0), ("osc_1.pitch", 0.0), ("dual_1.algorithm", 0.0), ("dual_1.a", 0.5), ("dual_1.b", 0.0),
    ("mix.osc_1", 1.0), ("mix.osc_2", 0.0), ("mix.noise", 0.0), ("transient.level", 0.0),
    ("drive.amount", 0.0), ("vintage.amount", 0.0), ("unison.mode", 0.0),
];

/// Synthesized transients, then the recorded samples.
const TRANSIENT_SYNTHS: i32 = 5;
const TRANSIENT_COUNT: i32 = 116;
/// An attack longer than this would swallow a transient.
const TRANSIENT_MAX_ATTACK: f32 = 0.03;

/// A small xorshift generator: all a roll needs, and repeatable from a seed.
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        let mut rng = Self(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1);
        (0..8).for_each(|_| { rng.unit(); });
        rng
    }

    /// 0..1.
    fn unit(&mut self) -> f32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 40) as f32 / (1u64 << 24) as f32
    }

    fn range(&mut self, (lo, hi): (f32, f32)) -> f32 {
        lo + (hi - lo) * self.unit()
    }

    /// Evenly in ratio rather than in value: for times and frequencies.
    fn log_range(&mut self, (lo, hi): (f32, f32)) -> f32 {
        lo * (hi / lo).powf(self.unit())
    }

    fn int(&mut self, lo: i32, end: i32) -> i32 {
        (lo + (self.unit() * (end - lo) as f32) as i32).min(end - 1)
    }

    fn chance(&mut self, p: f32) -> bool {
        self.unit() < p
    }

    fn weighted<T: Copy>(&mut self, choices: &[(T, f32)]) -> T {
        let total: f32 = choices.iter().map(|c| c.1).sum();
        let mut left = self.unit() * total;
        for (value, weight) in choices {
            left -= weight;
            if left < 0.0 {
                return *value;
            }
        }
        choices[choices.len() - 1].0
    }

    fn sign(&mut self) -> f32 {
        if self.chance(0.5) { 1.0 } else { -1.0 }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Kind {
    Pad,
    Drone,
    Keys,
    Pluck,
    Bass,
    Lead,
}

impl Kind {
    pub fn name(self) -> &'static str {
        match self {
            Kind::Pad => "Pad",
            Kind::Drone => "Drone",
            Kind::Keys => "Keys",
            Kind::Pluck => "Pluck",
            Kind::Bass => "Bass",
            Kind::Lead => "Lead",
        }
    }
}

/// What a kind of sound asks of the envelope and filter. Ranges are (lo, hi).
struct Profile {
    attack: (f32, f32),
    decay: (f32, f32),
    sustain: (f32, f32),
    release: (f32, f32),
    amp_sustain: (f32, f32),
    cutoff: (f32, f32),
    env_amount: (f32, f32),
    /// The chances of unison and noise on this kind of sound.
    unison: f32,
    noise: f32,
    /// The chance of a Harmonic EQ shape.
    harmonic: f32,
    /// Whether things move slowly: the wavetable's and the LFO's periods.
    slow: bool,
}

fn profile(kind: Kind) -> Profile {
    let base = Profile { attack: (0.005, 0.02), decay: (0.2, 1.0), sustain: (0.2, 0.6), release: (0.1, 0.6),
                         amp_sustain: (0.0, 0.5), cutoff: (400.0, 3000.0), env_amount: (0.5, 3.0),
                         unison: 0.1, noise: 0.25, harmonic: 0.15, slow: false };
    match kind {
        Kind::Pad => Profile { attack: (0.25, 3.0), decay: (0.5, 4.0), sustain: (0.5, 1.0), release: (1.0, 6.0),
                               amp_sustain: (0.5, 1.0), cutoff: (500.0, 5000.0), env_amount: (0.0, 1.5),
                               unison: 0.4, noise: 0.5, harmonic: 0.3, slow: true },
        Kind::Drone => Profile { attack: (1.0, 8.0), decay: (2.0, 8.0), sustain: (0.8, 1.0), release: (3.0, 10.0),
                                 amp_sustain: (0.8, 1.0), cutoff: (300.0, 4000.0), env_amount: (0.0, 0.8),
                                 unison: 0.4, noise: 0.55, harmonic: 0.3, slow: true },
        Kind::Keys => base,
        Kind::Pluck => Profile { attack: (0.005, 0.008), decay: (0.1, 0.7), sustain: (0.0, 0.1), release: (0.1, 0.6),
                                 amp_sustain: (0.0, 0.1), cutoff: (200.0, 1500.0), env_amount: (2.0, 5.0),
                                 unison: 0.1, noise: 0.15, harmonic: 0.15, slow: false },
        Kind::Bass => Profile { attack: (0.005, 0.012), decay: (0.1, 0.6), sustain: (0.3, 0.8), release: (0.05, 0.25),
                                amp_sustain: (0.3, 1.0), cutoff: (150.0, 1200.0), env_amount: (0.5, 3.5),
                                unison: 0.05, noise: 0.1, harmonic: 0.2, slow: false },
        Kind::Lead => Profile { attack: (0.005, 0.05), decay: (0.2, 0.8), sustain: (0.5, 0.9), release: (0.1, 0.5),
                                amp_sustain: (0.5, 1.0), cutoff: (800.0, 6000.0), env_amount: (0.2, 2.5),
                                unison: 0.3, noise: 0.2, harmonic: 0.25, slow: false },
    }
}

/// A rolled patch: the kind of sound, and a plain value per parameter id.
pub struct Patch {
    pub kind: Kind,
    pub values: Vec<(String, f32)>,
}

impl Patch {
    fn set(&mut self, id: &str, value: f32) {
        match self.values.iter_mut().find(|(k, _)| k == id) {
            Some(entry) => entry.1 = value,
            None => self.values.push((id.to_string(), value)),
        }
    }

    pub fn get(&self, id: &str) -> Option<f32> {
        self.values.iter().find(|(k, _)| k == id).map(|(_, v)| *v)
    }
}

/// Osc 2's Pitch, in semitones: usually none, then an octave either way,
/// rarely two; now and then an interval, nearly always a fourth or a fifth.
fn osc2_semitones(rng: &mut Rng) -> f32 {
    if rng.chance(0.06) {
        return rng.weighted(&[(5.0, 0.45), (7.0, 0.45), (3.0, 0.05), (4.0, 0.05)]) * rng.sign();
    }
    12.0 * rng.weighted(&[(0.0, 0.76), (-1.0, 0.1), (1.0, 0.1), (-2.0, 0.02), (2.0, 0.02)])
}

/// Four waves: usually neighbours in one set (WAVE_SETS: A8, M8, M16, as
/// ranges of the menus' indices; waves in a set are related to one another),
/// moving in one direction.
fn wave_quad(rng: &mut Rng) -> [f32; 4] {
    let (lo, end) = rng.weighted(&[(WAVE_SETS[0], 0.55), (WAVE_SETS[1], 0.25), (WAVE_SETS[2], 0.2)]);
    let first = rng.int(lo, end);
    if rng.chance(0.3) {
        return [first, rng.int(lo, end), rng.int(lo, end), rng.int(lo, end)].map(|w| w as f32);
    }
    let wrap = |w: i32| lo + (w - lo).rem_euclid(end - lo);
    let direction = if rng.chance(0.5) { 1 } else { -1 };
    let mut waves = [first; 4];
    for k in 1..4 {
        waves[k] = waves[k - 1] + direction * rng.int(1, 8);
    }
    waves.map(|w| wrap(w) as f32)
}

/// A Dual wave (Sine, Triangle, Saw, Square): saws and squares the most.
fn dual_wave(rng: &mut Rng) -> f32 {
    rng.weighted(&[(2.0, 0.45), (3.0, 0.25), (1.0, 0.15), (0.0, 0.15)])
}

/// The per-slot knobs the roll may modulate, by slot.
const DUAL_A: [&str; 2] = ["dual_1.a", "dual_2.a"];
const DUAL_B: [&str; 2] = ["dual_1.b", "dual_2.b"];
const WT_POSITION: [&str; 2] = ["wt_1.position", "wt_2.position"];
const WT_DEPTH: [&str; 2] = ["wt_1.oscmod_depth", "wt_2.oscmod_depth"];
const OSC_FADER: [&str; 2] = ["mix.osc_1", "mix.osc_2"];

/// A modulation the roll may add: (knob, source, smallest, largest amount).
type Pick = (&'static str, &'static str, f32, f32);

/// A modulator field at rest, as a fresh patch has it.
fn mod_default(field: &str) -> f32 {
    let d = ModSettings::default();
    match field {
        "shape" => 0.0,
        "rate" => d.hz,
        "sync" => 0.0,
        "division" => modulation::DEFAULT_DIVISION as f32,
        "retrig" => 1.0,
        "attack" => d.attack,
        "decay" => d.decay,
        "release" => d.release,
        "attack_curve" => modulation::DEFAULT_ATTACK_BOW,
        "decay_curve" | "release_curve" => modulation::DEFAULT_BOW,
        _ => 0.0, // sustain and the amounts
    }
}

/// Slot `k`'s Ray: mostly one of the six elements, off its centre (a centred
/// shape is quiet, or an octave up), sometimes bent, with the ray moved off
/// axis, or orbiting slowly.
fn roll_ray(rng: &mut Rng, p: &mut Patch, k: usize) {
    let id = |name: &str| format!("ray_{k}.{name}");
    p.set(&id("element"), if rng.chance(0.15) { 6.0 } else { rng.int(0, 6) as f32 });
    p.set(&id("chaos_seed"), rng.int(1, 100_000) as f32);
    p.set(&id("free"), rng.chance(0.3) as i32 as f32);
    for name in ["squish", "skew", "tilt"] {
        p.set(&id(name), if rng.chance(0.35) { rng.range((-0.7, 0.7)) } else { 0.0 });
    }
    p.set(&id("center"), rng.range((0.2, 0.8)) * rng.sign());
    p.set(&id("ray"), if rng.chance(0.3) { rng.range((0.05, 0.8)) } else { 0.0 });
    let orbit = rng.chance(0.15);
    p.set(&id("orbit"), if orbit { rng.range((0.05, 0.4)) } else { 0.0 });
    p.set(&id("orbit_rate"), rng.log_range((0.1, 8.0)));
    p.set(&id("fine"), 0.0);
}

/// Slot `k`'s Windows: any base, one to three interrupts on, their windows
/// spread round the cycle, mostly short, mostly at phase 0 (a peek at the
/// hidden wave as it is) or turned a quarter or half.
fn roll_windows(rng: &mut Rng, p: &mut Patch, k: usize) {
    let id = |name: &str| format!("windows_{k}.{name}");
    p.set(&id("base"), rng.int(0, 6) as f32);
    let on = rng.weighted(&[(1, 0.4), (2, 0.35), (3, 0.25)]);
    let start = rng.range((0.0, 0.99));
    for i in 1..=3 {
        let f = |name: &str| id(&format!("int_{i}_{name}"));
        p.set(&f("on"), (i <= on) as i32 as f32);
        p.set(&f("shape"), rng.int(0, 5) as f32);
        p.set(&f("x"), (start + (i - 1) as f32 / on as f32 + rng.range((-0.05, 0.05))).rem_euclid(1.0).min(0.99));
        let phase = match rng.chance(0.15) {
            true => rng.range((0.0, 360.0)),
            false => rng.weighted(&[(0.0, 0.45), (90.0, 0.2), (180.0, 0.25), (270.0, 0.1)]),
        };
        p.set(&f("phase"), phase);
        p.set(&f("len"), if rng.chance(0.7) { rng.range((0.05, 0.25)) } else { rng.range((0.25, 0.5)) });
        // Pitch: mostly ×1, now and then a small whole ratio (×2, ×3, ×4).
        p.set(&f("pitch"), rng.weighted(&[(0.0, 0.78), (1.0, 0.1), (3f32.log2(), 0.06), (2.0, 0.06)]));
    }
}

/// Slot `k`'s Orthogonal: two sets of a category that suits the kind of sound
/// (now and then the same set twice, or another category below), near their
/// starts, mostly moving at the recordings' own pace, mixed about evenly.
fn roll_ortho(rng: &mut Rng, p: &mut Patch, k: usize, kind: Kind) {
    use crate::ortholib::category;
    let id = |name: &str| format!("ortho_{k}.{name}");
    // Categories: Bass, Lead, Pad, Vocal.
    let suits = |rng: &mut Rng| match kind {
        Kind::Bass => rng.weighted(&[(0, 0.6), (1, 0.4)]),
        Kind::Pad | Kind::Drone => rng.weighted(&[(2, 0.6), (3, 0.25), (1, 0.15)]),
        _ => rng.weighted(&[(1, 0.75), (2, 0.15), (3, 0.1)]),
    };
    let pick = |rng: &mut Rng, c: usize| {
        let sets: Vec<usize> = category(c).map(|(k, _)| k).collect();
        sets[rng.int(0, sets.len() as i32) as usize] as f32
    };
    let c = suits(rng);
    let top = pick(rng, c);
    let bottom = if rng.chance(0.15) { top } else if rng.chance(0.7) { pick(rng, c) } else { let c = suits(rng); pick(rng, c) };
    p.set(&id("top"), top);
    p.set(&id("bottom"), bottom);
    for h in ["top", "bottom"] {
        p.set(&id(&format!("{h}_position")), if rng.chance(0.7) { 0.0 } else { rng.range((0.0, 0.35)) });
        p.set(&id(&format!("{h}_speed")), rng.weighted(&[(100.0, 0.6), (50.0, 0.15), (0.0, 0.12), (150.0, 0.13)]));
        // Mostly where it was recorded, now and then an octave either way.
        let octave = rng.weighted(&[(0.0, 0.8), (12.0, 0.1), (-12.0, 0.1)]);
        p.set(&id(&format!("{h}_pitch")), pitch::position(octave));
    }
    p.set(&id("mix"), rng.range((0.3, 0.7)));
}

/// The cutoff's travel (its knob's: 20 Hz..18 kHz, log), for turning octaves
/// of envelope into an amount.
fn cutoff_travel(hz: f32) -> f32 {
    modulation::Travel { min: 20.0, max: 18_000.0, factor: 0.25 }.normalize(hz)
}

/// Roll a patch. `ids` is every parameter: every modulator is reset first, so
/// that nothing is left over from the patch before.
pub fn roll<'a>(seed: u64, ids: impl Iterator<Item = &'a str>) -> Patch {
    let rng = &mut Rng::new(seed);
    let kind = rng.weighted(&[(Kind::Pad, 0.42), (Kind::Drone, 0.13), (Kind::Keys, 0.15), (Kind::Pluck, 0.1),
                              (Kind::Bass, 0.1), (Kind::Lead, 0.1)]);
    let shape = profile(kind);
    let mut patch = Patch { kind, values: Vec::new() };
    for id in ids.filter(|id| id.starts_with("mod.") && !kept(id)) {
        let field = id.rsplit('.').next().unwrap();
        patch.set(id, mod_default(field));
    }
    let p = &mut patch;

    // The envelopes: the amp's, and the cutoff's (its modulator's), which
    // share an attack and release; the amp sustains at least as high and
    // decays at least as long.
    let attack = rng.log_range(shape.attack);
    let (decay, sustain, release) = (rng.log_range(shape.decay), rng.range(shape.sustain), rng.log_range(shape.release));
    let amp_decay = if rng.chance(0.25) { decay * (10.0 / decay).powf(rng.range((0.0, 0.5))) } else { decay };
    p.set("amp.attack", attack);
    p.set("amp.decay", amp_decay);
    p.set("amp.sustain", sustain + rng.range(shape.amp_sustain) * (1.0 - sustain));
    p.set("amp.release", release);
    // The segments' bows: decay and release around the old single Curve's
    // range (its -0.2..0.8 is a bow of 0.24..-0.96), the attack mirrored (bowed
    // up: sharp), each varied a little.
    let bow = -1.2 * rng.range((-0.2, 0.8));
    for (id, sign) in [("amp.attack_curve", -1.0), ("amp.decay_curve", 1.0), ("amp.release_curve", 1.0)] {
        p.set(id, (sign * bow + rng.range((-0.15, 0.15))).clamp(-1.0, 1.0));
    }
    p.set("mod.filter_cutoff.attack", attack.min(10.0));
    p.set("mod.filter_cutoff.decay", decay);
    p.set("mod.filter_cutoff.sustain", sustain);
    p.set("mod.filter_cutoff.release", release.min(10.0));

    // Which sources sound: Osc 1 always, Osc 2 often. A fader at 0 also saves the CPU.
    let second = rng.chance(match kind {
        Kind::Bass => 0.35,
        Kind::Pad | Kind::Drone => 0.6,
        _ => 0.45,
    });
    let noise = rng.chance(shape.noise);
    p.set("mix.osc_1", rng.range((0.5, 1.0)));
    p.set("mix.osc_2", if second { rng.range((0.3, 0.9)) } else { 0.0 });
    p.set("mix.noise", if noise { rng.range((0.03, 0.25)) } else { 0.0 });

    // The oscillators. Each slot gets both its engines rolled, whichever one
    // it plays, so switching it over finds something too. An interval means
    // something only against the other slot: alone, it would just put the
    // patch in another key.
    let mut duals = Vec::new();      // the slots sounding a Dual
    let mut wavetables = Vec::new(); // the slots sounding a Wavetable, with its OscMod
    for slot in 0..2 {
        let k = slot + 1;
        let sounds = slot == 0 || second;
        let plays_dual = rng.chance(match kind {
            Kind::Bass => 0.65,
            Kind::Pad | Kind::Drone => 0.4,
            _ => 0.55,
        });
        // Now and then an Orthogonal (sampled sounds, grained), more often in
        // pads and drones; or Ray (Raycaster's oscillator).
        let plays_ortho = rng.chance(match kind {
            Kind::Pad | Kind::Drone => 0.2,
            _ => 0.1,
        });
        let plays_ray = !plays_ortho && rng.chance(0.18);
        // Or Windows (a base wave with windows onto others).
        let plays_windows = !plays_ortho && !plays_ray && rng.chance(0.12);
        let plays_dual = plays_dual && !plays_ray && !plays_ortho && !plays_windows;
        p.set(&format!("osc_{k}.mode"),
              if plays_ortho { 3.0 } else if plays_ray { 2.0 } else if plays_windows { 4.0 } else if plays_dual { 0.0 } else { 1.0 });
        // Basses sit lower. Osc 1 moves only by octaves: it's the reference.
        let semis = match (slot, kind) {
            (0, Kind::Bass) => 12.0 * rng.weighted(&[(-1.0, 0.6), (0.0, 0.3), (-2.0, 0.1)]),
            (0, _) => 12.0 * rng.weighted(&[(0.0, 0.8), (-1.0, 0.15), (1.0, 0.05)]),
            _ if second => osc2_semitones(rng),
            _ => 0.0,
        };
        p.set(&format!("osc_{k}.pitch"), pitch::position(semis));

        // VA Sweep, Spread, PWM, Sync, Fractal, Window Amp Sync, Metal, Ring Mod,
        // FM, Noise AM, Chaos, Folding: the rougher ones more rarely.
        let algorithm = rng.weighted(&[(0.0, 0.12), (1.0, 0.16), (2.0, 0.08), (3.0, 0.12), (4.0, 0.07), (5.0, 0.07),
                                       (6.0, 0.06), (7.0, 0.08), (8.0, 0.14), (9.0, 0.04), (10.0, 0.03), (11.0, 0.03)]);
        p.set(&format!("dual_{k}.algorithm"), algorithm);
        p.set(&format!("dual_{k}.wave_1"), dual_wave(rng));
        p.set(&format!("dual_{k}.wave_2"), dual_wave(rng));
        p.set(&format!("dual_{k}.a"), rng.range((0.1, 0.8)));
        p.set(&format!("dual_{k}.b"), rng.range((0.0, 0.8)));

        let waves = wave_quad(rng);
        for (i, wave) in waves.iter().enumerate() {
            p.set(&format!("wt_{k}.wave_{}", i + 1), *wave);
        }
        p.set(&format!("wt_{k}.position"), rng.range((0.0, 0.6)));
        let oscmod = if rng.chance(0.75) { 0 } else { rng.int(1, 15) };
        p.set(&format!("wt_{k}.oscmod"), oscmod as f32);
        p.set(&format!("wt_{k}.oscmod_depth"), rng.range((0.2, 0.8)));

        roll_ray(rng, p, k);
        roll_ortho(rng, p, k, kind);
        roll_windows(rng, p, k);

        if sounds && plays_dual {
            duals.push(slot);
        } else if sounds && !plays_ray && !plays_ortho {
            wavetables.push((slot, oscmod));
        }
    }
    // A sounding wavetable usually sweeps through its waves: a slow triangle on
    // the position.
    for &(slot, _) in &wavetables {
        if rng.chance(0.6) {
            let prefix = format!("mod.{}", mod_slug(WT_POSITION[slot]));
            p.set(&format!("{prefix}.shape"), 1.0);
            p.set(&format!("{prefix}.rate"), rng.log_range(if shape.slow { (0.1, 0.5) } else { (0.2, 2.0) }));
            p.set(&format!("{prefix}.lfo"), rng.range((0.15, 0.4)));
        }
    }

    // Unison: U7 a little more often than U5.
    let unison = rng.chance(shape.unison);
    p.set("unison.mode", if !unison { 0.0 } else if rng.chance(0.6) { 2.0 } else { 1.0 });
    // Detune as the outer pair's cents at A4 (on the Virus's 0-127 curve),
    // and a spread from narrow to the outer pair at the sides.
    p.set("unison.detune", crate::unison::detune_for_cents(rng.log_range((2.0, 14.0))));
    p.set("unison.spread", rng.range((24.0, 96.0)));
    p.set("unison.random_phase", rng.chance(0.8) as i32 as f32);

    // Noise.
    p.set("noise.type", rng.weighted(&[(0.0, 0.35), (1.0, 0.35), (2.0, 0.1), (3.0, 0.05), (4.0, 0.15)]));
    p.set("noise.clock_ratio", rng.int(-2, 3) as f32);
    p.set("noise.bits", rng.int(4, 13) as f32);
    p.set("noise.crackle_density", rng.range((0.1, 0.6)));
    p.set("noise.tilt", rng.range((-12.0, 6.0)));

    // A transient only where the attack leaves room to hear it.
    let transient = attack <= TRANSIENT_MAX_ATTACK && rng.chance(0.6);
    p.set("transient.level", if transient { rng.range((0.1, 0.5)) } else { 0.0 });
    p.set("transient.type", if rng.chance(0.5) { rng.int(0, TRANSIENT_SYNTHS) } else { rng.int(TRANSIENT_SYNTHS, TRANSIENT_COUNT) } as f32);
    p.set("transient.decay", rng.log_range((10.0, 120.0)));
    p.set("transient.post_filter", (transient && rng.chance(0.3)) as i32 as f32);
    p.set("transient.pitch", if rng.chance(0.2) { rng.range((-12.0, 12.0)) } else { 0.0 });
    // FM's: mostly a bell's whole-number ratios (menu steps of 4 are 2:1 ... 7:1).
    p.set("transient.depth", rng.range((0.1, 0.8)));
    p.set("transient.ratio", if rng.chance(0.75) { 4 * rng.int(0, 6) } else { rng.int(0, 21) } as f32);

    // Drive: mostly a little, sometimes a lot, never much on a slow sound.
    let heavy = !shape.slow && rng.chance(0.15);
    p.set("drive.amount", if heavy { rng.range((0.45, 0.8)) } else { rng.range((0.03, 0.35)) });

    // The filter: usually a lowpass, with enough opening to be heard. High-pass
    // and band-pass settings are rarer and sit lower, where they leave something.
    let morph = if shape.slow || kind == Kind::Lead { rng.chance(0.15) } else { false };
    let morph = if morph { rng.range((0.15, 0.9)) } else { 0.0 };
    let bandpass = if kind != Kind::Bass && rng.chance(0.12) { rng.range((0.2, 0.8)) } else { 0.0 };
    let env_amount = rng.range(shape.env_amount);
    let mut cutoff = rng.log_range(shape.cutoff);
    if morph > 0.5 {
        cutoff = rng.log_range((100.0, 1200.0));
    }
    cutoff = cutoff.max(300.0 / 2f32.powf(env_amount));
    p.set("filter.cutoff", cutoff);
    // The envelope opens it by `env_amount` octaves, in the knob's travel;
    // now and then it closes it instead, on a high-pass or band-pass.
    let opened = cutoff_travel((cutoff * 2f32.powf(env_amount)).min(18_000.0)) - cutoff_travel(cutoff);
    let inverted = (morph >= 0.5 || bandpass >= 0.5) && rng.chance(0.2);
    p.set("mod.filter_cutoff.env", if inverted { -opened } else { opened });
    p.set("filter.resonance", if rng.chance(0.12) { rng.range((1.6, 4.0)) } else { rng.log_range((0.6, 1.6)) });
    p.set("filter.morph_lp_notch_hp", morph);
    p.set("filter.bandpass", bandpass);
    p.set("filter.4_pole", rng.chance(0.35) as i32 as f32);
    // Mostly the SVF; a lowpass now and then is one of the 4-pole lowpasses.
    let lowpass = morph == 0.0 && bandpass == 0.0;
    p.set("filter.type", if lowpass { rng.weighted(&[(0.0, 0.6), (1.0, 0.2), (2.0, 0.2)]) } else { 0.0 });
    p.set("mod.filter_cutoff.key", rng.range((0.2, 0.8)));
    // Now and then a high-pass, and a third of those ring (never oscillating).
    let hpf = kind != Kind::Bass && rng.chance(0.25);
    p.set("filter.utility_hpf", if hpf { rng.log_range((40.0, 200.0)) } else { 20.0 });
    p.set("filter.hpf_peak", if hpf && rng.chance(0.33) { rng.range((0.3, 0.7)) } else { 0.0 });

    // Vintage: a touch, and rarely enough to hear the wow.
    p.set("vintage.amount", if rng.chance(0.08) { rng.range((0.35, 0.7)) } else { rng.range((0.03, 0.35)) });

    // The Harmonic EQ now and then: a shape, slid somewhere near its middle,
    // mostly at full depth; Odd/Even rarely, and gently. (Tilt stays flat.)
    let harmonic = rng.chance(shape.harmonic);
    p.set("harmonic.shape", if harmonic { rng.int(1, crate::harmlib::NAMES.len() as i32 + 1) as f32 } else { 0.0 });
    p.set("harmonic.shift", rng.range((-1.5, 1.5)));
    p.set("harmonic.tilt", 0.0);
    p.set("harmonic.depth", if rng.chance(0.7) { 1.0 } else { rng.range((0.4, 1.0)) });
    p.set("harmonic.odd_even", if rng.chance(0.1) { rng.range((-0.7, 0.7)) } else { 0.0 });

    // A few modulators, only on things that are sounding, and modest ones.
    let mut picks: Vec<Pick> = vec![("filter.cutoff", "lfo", 0.03, 0.12), ("filter.cutoff", "vel", 0.03, 0.15),
                                    ("drive.amount", "env", 0.05, 0.2)];
    let mut add = |on: bool, list: &[Pick]| {
        if on {
            picks.extend_from_slice(list);
        }
    };
    for &slot in &duals {
        add(true, &[(DUAL_A[slot], "lfo", 0.05, 0.3), (DUAL_A[slot], "env", 0.1, 0.4), (DUAL_B[slot], "lfo", 0.05, 0.3)]);
    }
    for &(slot, oscmod) in &wavetables {
        add(true, &[(WT_POSITION[slot], "env", 0.1, 0.4)]);
        add(oscmod > 0, &[(WT_DEPTH[slot], "lfo", 0.05, 0.3), (WT_DEPTH[slot], "env", 0.1, 0.4)]);
    }
    add(noise, &[("noise.tilt", "lfo", 0.05, 0.3), ("mix.noise", "env", 0.05, 0.3), ("mix.noise", "lfo", 0.03, 0.15)]);
    add(true, &[(OSC_FADER[0], "vel", 0.05, 0.3)]);
    add(morph > 0.0, &[("filter.morph_lp_notch_hp", "lfo", 0.05, 0.25)]);
    // A shape slid along the harmonics is the Harmonic EQ's sound.
    add(harmonic, &[("harmonic.shift", "lfo", 0.03, 0.15), ("harmonic.shift", "env", 0.05, 0.25)]);
    let count = rng.weighted(&[(0, 0.2), (1, 0.3), (2, 0.3), (3, 0.15), (4, 0.05)]);
    for _ in 0..count.min(picks.len()) {
        let k = rng.int(0, picks.len() as i32) as usize;
        let (knob, source, lo, hi) = picks.swap_remove(k);
        let prefix = format!("mod.{}", mod_slug(knob));
        // Velocity only ever adds; the others go either way.
        let sign = if source == "vel" { 1.0 } else { rng.sign() };
        p.set(&format!("{prefix}.{source}"), rng.range((lo, hi)) * sign);
        match source {
            "lfo" => {
                // Mostly a sine or triangle; now and then synced, or random.
                p.set(&format!("{prefix}.shape"), rng.weighted(&[(0.0, 0.45), (1.0, 0.35), (5.0, 0.1), (7.0, 0.1)]));
                p.set(&format!("{prefix}.rate"), rng.log_range(if shape.slow { (0.05, 1.5) } else { (0.2, 6.0) }));
                if rng.chance(0.15) {
                    p.set(&format!("{prefix}.sync"), 1.0);
                    p.set(&format!("{prefix}.division"), rng.int(2, 11) as f32);
                }
                p.set(&format!("{prefix}.retrig"), rng.chance(0.6) as i32 as f32);
            }
            "env" => {
                p.set(&format!("{prefix}.attack"), rng.log_range((0.005, 1.0)));
                p.set(&format!("{prefix}.decay"), rng.log_range((0.1, 3.0)));
                p.set(&format!("{prefix}.sustain"), rng.range((0.0, 0.5)));
                p.set(&format!("{prefix}.release"), rng.log_range((0.1, 3.0)));
            }
            _ => {}
        }
    }
    patch
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::{Engine, MAX_BLOCK};
    use crate::params::SynthParams;

    fn roll_with(params: &SynthParams, seed: u64) -> Patch {
        roll(seed, params.ids())
    }

    /// A roll sets every parameter but the kept ones, and only real ones, within range.
    #[test]
    fn every_parameter_is_rolled_or_kept() {
        let params = SynthParams::from_faust();
        let patch = roll_with(&params, 1);
        for id in params.ids() {
            assert_ne!(patch.get(id).is_some(), kept(id), "'{id}' should be rolled or kept, not both or neither");
        }
        for seed in 0..500 {
            for (id, value) in roll_with(&params, seed).values {
                let p = params.ptr(&id).unwrap_or_else(|| panic!("no parameter '{id}'"));
                let (lo, hi) = unsafe { (p.preview_plain(0.0), p.preview_plain(1.0)) };
                assert!(value >= lo - 1e-3 && value <= hi + 1e-3, "'{id}' rolled {value}, outside {lo}..{hi}");
            }
        }
    }

    /// The wave menus and the transient menu have the shape the roll assumes.
    #[test]
    fn menus_are_as_assumed() {
        let params = SynthParams::from_faust();
        let name = |id: &str, plain: f32| {
            let p = params.ptr(id).unwrap();
            unsafe { p.normalized_value_to_string(p.preview_normalized(plain), false) }
        };
        for ((lo, end), prefix) in WAVE_SETS.iter().zip(["A8 ", "M8 ", "M16 "]) {
            assert!(name("wt_2.wave_4", *lo as f32).starts_with(prefix));
            assert!(name("wt_2.wave_4", (*end - 1) as f32).starts_with(prefix));
        }
        assert_eq!(WAVE_SETS[2].1 as usize, BASIC_BASE);
        assert_eq!(name("wt_2.wave_4", BASIC_BASE as f32), "Sine");
        assert_eq!(name("wt_2.wave_4", BASIC_BASE as f32 + 1.0), "Triangle");
        assert_eq!(unsafe { params.ptr("wt_1.wave_1").unwrap().step_count() }, Some(WAVE_COUNT - 1));
        assert_eq!(unsafe { params.ptr("transient.type").unwrap().step_count() }, Some(TRANSIENT_COUNT as usize - 1));
        assert!(name("transient.type", (TRANSIENT_SYNTHS - 1) as f32) == "Chiff");
        assert_eq!(name("transient.type", crate::translib::FM as f32), "FM");
        assert_eq!(name("transient.ratio", 0.0), "2:1");
        assert_eq!(name("transient.ratio", 6.0), "3.5:1");
        assert_eq!(name("transient.ratio", 20.0), "7:1");
        assert_eq!(name("dual_1.algorithm", 0.0), "VA Sweep");
        assert_eq!(name("dual_2.algorithm", 11.0), "Folding");
    }

    #[test]
    fn rolls_follow_the_rules() {
        let params = SynthParams::from_faust();
        let rolls: Vec<Patch> = (0..3000).map(|s| roll_with(&params, s)).collect();
        let share = |test: &dyn Fn(&Patch) -> bool| rolls.iter().filter(|p| test(p)).count() as f32 / rolls.len() as f32;
        let v = |p: &Patch, id: &str| p.get(id).unwrap();

        // No transient under a slow attack, and plenty under a fast one.
        for p in &rolls {
            if v(p, "amp.attack") > TRANSIENT_MAX_ATTACK {
                assert_eq!(v(p, "transient.level"), 0.0);
                assert_eq!(v(p, "transient.post_filter"), 0.0);
            }
            assert!(v(p, "mix.osc_1") > 0.0, "no tonal source");
            assert_eq!(pitch::semitones(v(p, "osc_1.pitch")) % 12.0, 0.0, "Osc 1 moves only by octaves");
        }

        // Intervals other than octaves are rare, and nearly always a fourth or fifth.
        let semis = |p: &Patch| pitch::semitones(v(p, "osc_2.pitch"));
        let moved = share(&|p| semis(p) % 12.0 != 0.0);
        let fourths_fifths = share(&|p| [5.0, 7.0].contains(&semis(p).abs()));
        println!("intervals {:.1}%, of which fourths and fifths {:.0}%", 100.0 * moved, 100.0 * fourths_fifths / moved);
        assert!(moved > 0.02 && moved < 0.1);
        assert!(fourths_fifths / moved > 0.7);
        // A silent Osc 2 doesn't get transposed.
        for p in rolls.iter().filter(|p| v(p, "mix.osc_2") == 0.0) {
            assert_eq!(v(p, "osc_2.pitch"), 0.0);
        }
        // Every engine gets played, in both slots.
        for id in ["osc_1.mode", "osc_2.mode"] {
            let dual = share(&|p| v(p, id) == 0.0);
            assert!(dual > 0.25 && dual < 0.7, "{id}: {dual}");
            let ray = share(&|p| v(p, id) == 2.0);
            assert!(ray > 0.1 && ray < 0.3, "{id}: {ray}");
            let ortho = share(&|p| v(p, id) == 3.0);
            assert!(ortho > 0.08 && ortho < 0.25, "{id}: {ortho}");
            let windows = share(&|p| v(p, id) == 4.0);
            assert!(windows > 0.04 && windows < 0.15, "{id}: {windows}");
        }

        // Most modulators stay off (beyond the cutoff's envelope and key, and
        // a wavetable's sweep), and the kinds of sound differ.
        const AMOUNTS: [&str; 6] = ["lfo", "env", "vel", "touch", "wheel", "key"];
        let extra = |p: &Patch| p.values.iter().filter(|(id, x)| {
            let field = id.rsplit('.').next().unwrap();
            id.starts_with("mod.") && AMOUNTS.contains(&field) && *x != 0.0
                && !id.starts_with("mod.filter_cutoff.env") && !id.starts_with("mod.filter_cutoff.key")
                && !(id.starts_with("mod.wt_") && id.ends_with("_position.lfo"))
        }).count();
        assert!(rolls.iter().all(|p| extra(p) <= 4));
        assert!(share(&|p| extra(p) == 0) > 0.15);
        let slow = share(&|p| matches!(p.kind, Kind::Pad | Kind::Drone));
        assert!(slow > 0.45 && slow < 0.65, "{slow}");
        for p in rolls.iter().filter(|p| matches!(p.kind, Kind::Pad | Kind::Drone)) {
            assert!(v(p, "amp.attack") >= 0.25 && v(p, "amp.sustain") >= 0.5);
        }
    }

    /// The init patch names real parameters, within their ranges, and with
    /// everything else at its default plays Osc 1's plain saw alone: as
    /// loud as the default patch without its noise, and the Harmonic EQ off.
    #[test]
    fn init_is_one_plain_oscillator() {
        let params = SynthParams::from_faust();
        for (id, v) in INIT {
            let p = params.ptr(id).unwrap_or_else(|| panic!("no parameter '{id}'"));
            let back = unsafe { p.preview_plain(p.preview_normalized(v)) };
            assert!((back - v).abs() < 1e-3, "{id} = {v} is out of its range ({back})");
        }
        assert_eq!(params.ptr("harmonic.shape").map(|p| unsafe { p.default_plain_value() }), Some(0.0));
        let play = |sets: &[(&str, f32)]| {
            let mut e = Engine::new(params.slots());
            e.init(48_000.0);
            e.load(&params);
            for (id, v) in sets {
                e.set_by_id(&params, id, *v);
            }
            e.note_on(0, 57, 0.8);
            let mut buf = [0.0; MAX_BLOCK];
            let mut out = Vec::new();
            for _ in 0..300 {
                e.render(&mut buf);
                out.extend_from_slice(&buf);
            }
            out
        };
        let init = play(&INIT);
        let saw = play(&[("mix.noise", 0.0), ("drive.amount", 0.0)]);
        assert!(init == saw, "the init patch isn't the default patch's saw alone");
        assert!(init.iter().map(|s| s * s).sum::<f32>() / init.len() as f32 > 1e-3, "the init patch is silent");
    }

    /// What a roll changes fits the host's queue for changes from the panel
    /// (nih-plug's CLAP wrapper: 2048 events, three per change; the panel
    /// sends only what differs), from a fresh patch or from any other roll
    /// (whose few modulators it puts back to rest).
    #[test]
    fn rolls_fit_the_hosts_queue() {
        let params = SynthParams::from_faust();
        let at_rest = |id: &str, v: f32| {
            let p = params.ptr(id).unwrap();
            unsafe { p.preview_normalized(v) == p.unmodulated_normalized_value() }
        };
        let mut most = 0;
        for seed in 0..300 {
            let patch = roll_with(&params, seed);
            let from_fresh = patch.values.iter().filter(|(id, v)| !at_rest(id, *v)).count();
            // Every control a roll sets, and every modulator field another roll could have moved.
            let worst = patch.values.iter().filter(|(id, v)| !id.starts_with("mod.") || !at_rest(id, *v)).count()
                + roll_with(&params, seed + 1000).values.iter().filter(|(id, v)| id.starts_with("mod.") && !at_rest(id, *v)).count();
            most = most.max(worst);
            assert!(3 * from_fresh < 2048 && 3 * worst < 2048, "seed {seed}: {from_fresh} from fresh, {worst} at worst");
        }
        println!("most changes in a roll: {most} (of {} values)", roll_with(&params, 0).values.len());
    }

    /// Rolled patches render finite and audible, with sane levels. Slow ones get
    /// time to open.
    #[test]
    fn rolls_sound() {
        let params = SynthParams::from_faust();
        let (mut quietest, mut loudest) = (f32::MAX, 0f32);
        let mut buf = [0.0; MAX_BLOCK];
        for seed in 0..24 {
            let patch = roll_with(&params, seed * 37 + 11);
            let mut e = Engine::new(params.slots());
            e.init(48_000.0);
            e.load(&params);
            for (id, value) in &patch.values {
                e.set_by_id(&params, id, *value);
            }
            for n in [36, 48, 55] {
                e.note_on(0, n, 0.8);
            }
            let (mut energy, mut count, mut peak) = (0.0f32, 0usize, 0f32);
            let attack = patch.get("amp.attack").unwrap();
            let settle = ((attack * 750.0) as usize).min(750 * 3);
            for b in 0..settle + 750 {
                e.render(&mut buf);
                assert!(buf.iter().all(|s| s.is_finite()), "seed {seed}");
                if b >= settle {
                    energy += buf.iter().map(|s| s * s).sum::<f32>();
                    count += buf.len();
                    peak = peak.max(buf.iter().fold(0.0, |p, s| p.max(s.abs())));
                }
            }
            let rms = (energy / count as f32).sqrt();
            println!("{:5} rms {:.3}, peak {:.2}", patch.kind.name(), rms, peak);
            quietest = quietest.min(rms);
            loudest = loudest.max(peak);
            assert!(rms > 1e-3, "seed {seed} ({}) is nearly silent: {rms}", patch.kind.name());
        }
        println!("quietest rms {quietest:.3}, loudest peak {loudest:.2}");
        assert!(loudest < 6.0, "peak {loudest}");
    }
}
