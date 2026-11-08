//! The dice: a random patch that's meant to be playable. It first picks what
//! kind of sound to make, and that sets the ranges for the envelope and filter.
//! The rest follows what a person programming that sound would do: most offsets
//! stay at zero, and those that don't land on musical values. Controls that
//! wouldn't be heard are left at rest.
//!
//! Performance setup (Perform, Volume, Portamento) and the three Mutes are left
//! as they are. No UI here: `roll` returns plain values, so it can be tested.

/// Parameters a roll leaves alone.
pub fn kept(id: &str) -> bool {
    id.starts_with("perform.") || id.starts_with("settings.") || id == "master.volume" || id == "master.portamento" || id.ends_with(".mute")
}

/// The wave library's sets, as ranges of the menus' indices: waves in a set are
/// related to one another, so a wavetable's three usually come from one.
const WAVE_SETS: [(i32, i32); 3] = [(0, 118), (118, 170), (170, 217)]; // A8, M8, M16
/// The basic shapes at the end of the library, for the LFO: sine, triangle.
const LFO_SHAPES: [f32; 2] = [217.0, 218.0];
const WAVE_COUNT: i32 = 221;
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
    /// The chances of each texture on this kind of sound.
    events: f32,
    calls: f32,
    unison: f32,
    noise: f32,
    /// Whether things move slowly: the wavetable's and the LFO's periods.
    slow: bool,
}

fn profile(kind: Kind) -> Profile {
    let base = Profile { attack: (0.005, 0.02), decay: (0.2, 1.0), sustain: (0.2, 0.6), release: (0.1, 0.6),
                         amp_sustain: (0.0, 0.5), cutoff: (400.0, 3000.0), env_amount: (0.5, 3.0),
                         events: 0.2, calls: 0.05, unison: 0.1, noise: 0.25, slow: false };
    match kind {
        Kind::Pad => Profile { attack: (0.25, 3.0), decay: (0.5, 4.0), sustain: (0.5, 1.0), release: (1.0, 6.0),
                               amp_sustain: (0.5, 1.0), cutoff: (500.0, 5000.0), env_amount: (0.0, 1.5),
                               events: 0.55, calls: 0.2, unison: 0.4, noise: 0.5, slow: true },
        Kind::Drone => Profile { attack: (1.0, 8.0), decay: (2.0, 8.0), sustain: (0.8, 1.0), release: (3.0, 10.0),
                                 amp_sustain: (0.8, 1.0), cutoff: (300.0, 4000.0), env_amount: (0.0, 0.8),
                                 events: 0.6, calls: 0.35, unison: 0.4, noise: 0.55, slow: true },
        Kind::Keys => base,
        Kind::Pluck => Profile { attack: (0.005, 0.008), decay: (0.1, 0.7), sustain: (0.0, 0.1), release: (0.1, 0.6),
                                 amp_sustain: (0.0, 0.1), cutoff: (200.0, 1500.0), env_amount: (2.0, 5.0),
                                 events: 0.05, calls: 0.0, unison: 0.1, noise: 0.15, slow: false },
        Kind::Bass => Profile { attack: (0.005, 0.012), decay: (0.1, 0.6), sustain: (0.3, 0.8), release: (0.05, 0.25),
                                amp_sustain: (0.3, 1.0), cutoff: (150.0, 1200.0), env_amount: (0.5, 3.5),
                                events: 0.05, calls: 0.0, unison: 0.05, noise: 0.1, slow: false },
        Kind::Lead => Profile { attack: (0.005, 0.05), decay: (0.2, 0.8), sustain: (0.5, 0.9), release: (0.1, 0.5),
                                amp_sustain: (0.5, 1.0), cutoff: (800.0, 6000.0), env_amount: (0.2, 2.5),
                                events: 0.15, calls: 0.05, unison: 0.3, noise: 0.2, slow: false },
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

/// An oscillator's semitones: nearly always none; otherwise a fourth or a
/// fifth, either way; other intervals only rarely.
fn semitones(rng: &mut Rng) -> f32 {
    if rng.chance(0.88) {
        return 0.0;
    }
    let interval = rng.weighted(&[(5.0, 0.4), (7.0, 0.4), (12.0, 0.08), (3.0, 0.04), (4.0, 0.04), (9.0, 0.02), (10.0, 0.02)]);
    interval * rng.sign()
}

/// An oscillator's octave: usually none, then one either way, rarely two.
fn octaves(rng: &mut Rng) -> f32 {
    rng.weighted(&[(0.0, 0.76), (-1.0, 0.1), (1.0, 0.1), (-2.0, 0.02), (2.0, 0.02)])
}

/// Three waves: usually neighbours in one set, moving in one direction.
fn wave_trio(rng: &mut Rng) -> [f32; 3] {
    let (lo, end) = rng.weighted(&[(WAVE_SETS[0], 0.55), (WAVE_SETS[1], 0.25), (WAVE_SETS[2], 0.2)]);
    let start = rng.int(lo, end);
    if rng.chance(0.3) {
        return [start as f32, rng.int(lo, end) as f32, rng.int(lo, end) as f32];
    }
    let wrap = |w: i32| lo + (w - lo).rem_euclid(end - lo);
    let direction = if rng.chance(0.5) { 1 } else { -1 };
    let mid = start + direction * rng.int(1, 8);
    let last = mid + direction * rng.int(1, 16);
    [start as f32, wrap(mid) as f32, wrap(last) as f32]
}

/// A send: a modulator moving a control a little. (id, smallest, largest depth.)
type Send = (String, f32, f32);

/// Roll a patch. `ids` is every parameter: any send not chosen is cleared, so
/// that nothing is left over from the patch before.
pub fn roll<'a>(seed: u64, ids: impl Iterator<Item = &'a str>) -> Patch {
    let rng = &mut Rng::new(seed);
    let kind = rng.weighted(&[(Kind::Pad, 0.42), (Kind::Drone, 0.13), (Kind::Keys, 0.15), (Kind::Pluck, 0.1),
                              (Kind::Bass, 0.1), (Kind::Lead, 0.1)]);
    let shape = profile(kind);
    let mut patch = Patch { kind, values: Vec::new() };
    for id in ids.filter(|id| id.ends_with("_env") || id.ends_with("_lfo")) {
        patch.set(id, 0.0);
    }
    let p = &mut patch;

    // The envelope.
    let attack = rng.log_range(shape.attack);
    p.set("envelope.attack", attack);
    p.set("envelope.decay", rng.log_range(shape.decay));
    p.set("envelope.sustain", rng.range(shape.sustain));
    p.set("envelope.release", rng.log_range(shape.release));
    p.set("envelope.curve", rng.range((-0.2, 0.8)));
    p.set("envelope.amp_sustain", rng.range(shape.amp_sustain));
    p.set("envelope.amp_decay", if rng.chance(0.25) { rng.range((0.0, 0.5)) } else { 0.0 });

    // Which sources sound: at least one tonal one. A fader at 0 also saves the CPU.
    let (fundamental_chance, dual_chance, wavetable_chance) = match kind {
        Kind::Bass => (0.8, 0.35, 0.25),
        Kind::Pad | Kind::Drone => (0.4, 0.4, 0.75),
        _ => (0.55, 0.45, 0.5),
    };
    let mut fundamental = rng.chance(fundamental_chance);
    let dual = rng.chance(dual_chance);
    let wavetable = rng.chance(wavetable_chance);
    if !fundamental && !dual && !wavetable {
        fundamental = true;
    }
    let noise = rng.chance(shape.noise);
    let fader = |rng: &mut Rng, on: bool| if on { rng.range((0.4, 1.0)) } else { 0.0 };
    p.set("mix.fundamental", fader(rng, fundamental));
    p.set("mix.dual", fader(rng, dual));
    p.set("mix.wavetable", fader(rng, wavetable));
    p.set("mix.noise", if noise { rng.range((0.03, 0.25)) } else { 0.0 });

    // The fundamental. Basses sit lower.
    let wave = rng.weighted(&[(2.0, 0.4), (3.0, 0.25), (1.0, 0.2), (0.0, 0.15)]);
    let pwm = wave == 3.0;
    p.set("fundamental.wave", wave);
    p.set("fundamental.octave", match kind {
        Kind::Bass => rng.weighted(&[(-1.0, 0.6), (0.0, 0.3), (-2.0, 0.1)]),
        _ => rng.weighted(&[(0.0, 0.7), (-1.0, 0.25), (1.0, 0.05)]),
    });
    p.set("fundamental.pulse_width", rng.range((0.25, 0.75)));
    p.set("fundamental.pw_lfo_rate", rng.log_range((0.1, 3.0)));
    p.set("fundamental.pw_lfo_depth", if pwm { rng.range((0.1, 0.6)) } else { 0.0 });

    // The Dual. An interval means something only against another source; alone,
    // it would just put the patch in another key.
    let accompanied = fundamental || wavetable;
    let algorithm = rng.weighted(&[(0.0, 0.1), (1.0, 0.2), (2.0, 0.1), (3.0, 0.1), (4.0, 0.15), (5.0, 0.2), (6.0, 0.1), (7.0, 0.05)]);
    p.set("dual.algorithm", algorithm);
    p.set("dual.waves", rng.int(0, 9) as f32);
    p.set("dual.a", rng.range((0.1, 0.8)));
    p.set("dual.b", rng.range((0.0, 0.8)));
    p.set("dual.octave", if accompanied { octaves(rng) } else { 0.0 });
    p.set("dual.semi", if accompanied { semitones(rng) } else { 0.0 });

    // The wavetable.
    let [start, mid, end] = wave_trio(rng);
    p.set("wavetable.start", start);
    p.set("wavetable.mid", mid);
    p.set("wavetable.end", end);
    p.set("wavetable.octave", if fundamental || dual { octaves(rng) } else { 0.0 });
    p.set("wavetable.lfo_rate", rng.log_range(if shape.slow { (2.0, 10.0) } else { (0.5, 5.0) }));
    let oscmod = if rng.chance(0.75) { 0 } else { rng.int(1, 15) };
    p.set("wavetable.oscmod", oscmod as f32);
    p.set("wavetable.oscmod_depth", rng.range((0.2, 0.8)));

    // Unison stacks only the Dual and Wavetable: U7 a little more often than U5.
    let unison = (dual || wavetable) && rng.chance(shape.unison);
    p.set("unison.mode", if !unison { 0.0 } else if rng.chance(0.6) { 2.0 } else { 1.0 });
    p.set("unison.detune", rng.log_range((2.0, 14.0)));
    p.set("unison.random_phase", rng.chance(0.8) as i32 as f32);

    // Noise.
    p.set("noise.type", rng.weighted(&[(0.0, 0.35), (1.0, 0.35), (2.0, 0.1), (3.0, 0.05), (4.0, 0.15)]));
    p.set("noise.clock_ratio", rng.int(-2, 3) as f32);
    p.set("noise.bits", rng.int(4, 13) as f32);
    p.set("noise.crackle_density", rng.range((0.1, 0.6)));
    p.set("noise.tilt", rng.range((-12.0, 6.0)));
    p.set("noise.move", if rng.chance(0.5) { rng.range((0.2, 0.8)) } else { 0.0 });
    p.set("noise.rate", rng.log_range((0.3, 4.0)));

    // A transient only where the attack leaves room to hear it.
    let transient = attack <= TRANSIENT_MAX_ATTACK && rng.chance(0.6);
    p.set("transient.level", if transient { rng.range((0.1, 0.5)) } else { 0.0 });
    p.set("transient.type", if rng.chance(0.5) { rng.int(0, TRANSIENT_SYNTHS) } else { rng.int(TRANSIENT_SYNTHS, TRANSIENT_COUNT) } as f32);
    p.set("transient.decay", rng.log_range((10.0, 120.0)));
    p.set("transient.post_filter", (transient && rng.chance(0.3)) as i32 as f32);
    p.set("transient.sample_pitch", if rng.chance(0.2) { rng.range((-12.0, 12.0)) } else { 0.0 });

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
    p.set("filter.env_amount", env_amount);
    p.set("filter.resonance", if rng.chance(0.12) { rng.range((1.6, 4.0)) } else { rng.log_range((0.6, 1.6)) });
    p.set("filter.morph_lp_notch_hp", morph);
    p.set("filter.bandpass", bandpass);
    p.set("filter.4_pole", rng.chance(0.35) as i32 as f32);
    p.set("filter.keytrack", rng.range((0.2, 0.8)));
    p.set("filter.invert_env", ((morph >= 0.5 || bandpass >= 0.5) && rng.chance(0.2)) as i32 as f32);
    p.set("filter.utility_hpf", if kind != Kind::Bass && rng.chance(0.25) { rng.log_range((40.0, 200.0)) } else { 20.0 });

    // Vintage: a touch, and rarely enough to hear the wow.
    p.set("vintage.amount", if rng.chance(0.08) { rng.range((0.35, 0.7)) } else { rng.range((0.03, 0.35)) });

    // Texture. Rolled off, a Level or Depth at its bottom keeps the CPU free.
    let events = rng.chance(shape.events);
    p.set("events.depth", if events { rng.range((3.0, 9.0)) } else { 0.0 });
    p.set("events.density", rng.range((40.0, 100.0)));
    p.set("events.life", rng.range((1.3, 2.7)));
    p.set("events.sharp", rng.log_range((0.7, 2.5)));
    let calls = rng.chance(shape.calls);
    p.set("calls.level", if calls { rng.range((-58.0, -46.0)) } else { -80.0 });
    p.set("calls.activity", rng.range((0.25, 0.7)));
    p.set("calls.rate", rng.range((0.6, 1.4)));
    p.set("calls.echo", rng.range((0.2, 0.9)));

    // The free modulators.
    p.set("mod_env.attack", rng.log_range((0.005, 1.0)));
    p.set("mod_env.decay", rng.log_range((0.1, 3.0)));
    p.set("mod_env.sustain", rng.range((0.0, 0.5)));
    p.set("mod_env.release", rng.log_range((0.1, 3.0)));
    p.set("mod_env.curve", rng.range((-0.2, 0.8)));
    p.set("mod_lfo.rate", rng.log_range(if shape.slow { (0.05, 1.5) } else { (0.2, 6.0) }));
    p.set("mod_lfo.sync", rng.chance(0.15) as i32 as f32);
    p.set("mod_lfo.shape", if rng.chance(0.8) { LFO_SHAPES[rng.int(0, 2) as usize] } else { rng.int(0, WAVE_COUNT) as f32 });

    // A few sends, only to things that are sounding, and modest ones.
    let mut sends: Vec<Send> = vec![("filter.cutoff_lfo".into(), 0.03, 0.12), ("filter.cutoff_env".into(), 0.05, 0.3),
                                    ("drive.amount_env".into(), 0.05, 0.2)];
    let mut add = |on: bool, list: &[(&str, f32, f32)]| {
        if on {
            sends.extend(list.iter().map(|(id, lo, hi)| (id.to_string(), *lo, *hi)));
        }
    };
    add(dual, &[("dual.a_lfo", 0.05, 0.3), ("dual.a_env", 0.1, 0.4), ("dual.b_lfo", 0.05, 0.3)]);
    add(wavetable && oscmod > 0, &[("wavetable.oscmod_depth_lfo", 0.05, 0.3), ("wavetable.oscmod_depth_env", 0.1, 0.4)]);
    add(fundamental && pwm, &[("fundamental.pulse_width_lfo", 0.05, 0.25)]);
    add(noise, &[("noise.tilt_lfo", 0.05, 0.3), ("mix.noise_env", 0.05, 0.3), ("mix.noise_lfo", 0.03, 0.15)]);
    add(morph > 0.0, &[("filter.morph_lp_notch_hp_lfo", 0.05, 0.25)]);
    add(events, &[("events.depth_lfo", 0.1, 0.4), ("events.sharp_lfo", 0.05, 0.3)]);
    let count = rng.weighted(&[(0, 0.2), (1, 0.3), (2, 0.3), (3, 0.15), (4, 0.05)]);
    for _ in 0..count.min(sends.len()) {
        let k = rng.int(0, sends.len() as i32) as usize;
        let (id, lo, hi) = sends.swap_remove(k);
        let depth = rng.range((lo, hi)) * rng.sign();
        p.set(&id, depth);
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
        assert!(name("wavetable.start", 117.0).starts_with("A8"));
        assert!(name("wavetable.start", 118.0).starts_with("M8"));
        assert!(name("wavetable.start", 169.0).starts_with("M8"));
        assert!(name("wavetable.start", 170.0).starts_with("M16"));
        assert!(name("wavetable.start", 216.0).starts_with("M16"));
        assert_eq!(name("wavetable.start", 217.0), "Sine");
        assert_eq!(name("wavetable.start", 218.0), "Triangle");
        assert_eq!(unsafe { params.ptr("wavetable.start").unwrap().step_count() }, Some(WAVE_COUNT as usize - 1));
        assert_eq!(unsafe { params.ptr("transient.type").unwrap().step_count() }, Some(TRANSIENT_COUNT as usize - 1));
        assert!(name("transient.type", (TRANSIENT_SYNTHS - 1) as f32) == "Chiff");
    }

    #[test]
    fn rolls_follow_the_rules() {
        let params = SynthParams::from_faust();
        let rolls: Vec<Patch> = (0..3000).map(|s| roll_with(&params, s)).collect();
        let share = |test: &dyn Fn(&Patch) -> bool| rolls.iter().filter(|p| test(p)).count() as f32 / rolls.len() as f32;
        let v = |p: &Patch, id: &str| p.get(id).unwrap();

        // No transient under a slow attack, and plenty under a fast one.
        for p in &rolls {
            if v(p, "envelope.attack") > TRANSIENT_MAX_ATTACK {
                assert_eq!(v(p, "transient.level"), 0.0);
                assert_eq!(v(p, "transient.post_filter"), 0.0);
            }
            assert!(v(p, "mix.fundamental") + v(p, "mix.dual") + v(p, "mix.wavetable") > 0.0, "no tonal source");
        }

        // Semitones are rare, and nearly always a fourth or fifth.
        let moved = share(&|p| v(p, "dual.semi") != 0.0);
        let fourths_fifths = share(&|p| [5.0, 7.0].contains(&v(p, "dual.semi").abs()));
        println!("semi moved {:.1}%, of which fourths and fifths {:.0}%", 100.0 * moved, 100.0 * fourths_fifths / moved);
        assert!(moved > 0.03 && moved < 0.12);
        assert!(fourths_fifths / moved > 0.7);
        // Alone, the Dual doesn't get transposed.
        for p in rolls.iter().filter(|p| v(p, "mix.fundamental") == 0.0 && v(p, "mix.wavetable") == 0.0) {
            assert_eq!(v(p, "dual.semi"), 0.0);
            assert_eq!(v(p, "dual.octave"), 0.0);
        }

        // Most sends stay at zero, and the kinds of sound differ.
        let sends = |p: &Patch| p.values.iter().filter(|(id, x)| (id.ends_with("_lfo") || id.ends_with("_env")) && id != "filter.invert_env" && *x != 0.0).count();
        assert!(rolls.iter().all(|p| sends(p) <= 4));
        let slow = share(&|p| matches!(p.kind, Kind::Pad | Kind::Drone));
        assert!(slow > 0.45 && slow < 0.65, "{slow}");
        for p in rolls.iter().filter(|p| matches!(p.kind, Kind::Pad | Kind::Drone)) {
            assert!(v(p, "envelope.attack") >= 0.25 && v(p, "envelope.sustain") >= 0.5);
        }
    }

    /// Rolled patches render finite and audible, with sane levels. Slow ones get
    /// time to open.
    #[test]
    fn rolls_sound() {
        let params = SynthParams::from_faust();
        let (mut quietest, mut loudest) = (f32::MAX, 0f32);
        let mut buf = [0.0; MAX_BLOCK];
        let (mut left, mut right) = ([0.0; MAX_BLOCK], [0.0; MAX_BLOCK]);
        for seed in 0..24 {
            let patch = roll_with(&params, seed * 37 + 11);
            let mut e = Engine::new(params.slots());
            e.init(48_000.0);
            for (i, v) in params.faust_values() {
                e.set_param(i, v);
            }
            for (id, value) in &patch.values {
                // Host values are stepped and ranged; the engine takes them plain.
                if let Some(targets) = params.faust_index(id) {
                    e.set_param(targets, *value);
                }
            }
            for n in [36, 48, 55] {
                e.note_on(0, n, 0.8);
            }
            let (mut energy, mut count, mut peak) = (0.0f32, 0usize, 0f32);
            let attack = patch.get("envelope.attack").unwrap();
            let settle = ((attack * 750.0) as usize).min(750 * 3);
            for b in 0..settle + 750 {
                e.render(&mut buf);
                e.render_calls(&mut left, &mut right);
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
