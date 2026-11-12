//! The die: a random patch that's likely to be usable. First a kind of drum
//! (kick, snare, hat, ...), then each of its settings from that kind's
//! usual range; now and then (EXTREME) a setting goes anywhere in its whole
//! range instead, so the odd one's wild but most are playable. Then some
//! seasoning: an effect or two, a ring or two, a sample layer.
//!
//! A patch is a list of (parameter id, plain value); everything it doesn't
//! name goes back to its default (but Volume, which the die never touches).

/// How often a setting ignores its kind's range and lands anywhere.
const EXTREME: f32 = 0.03;

pub type Patch = Vec<(&'static str, f32)>;

/// A small, seedable random source (xorshift).
pub struct Dice(u64);

impl Dice {
    pub fn new(seed: u64) -> Self {
        Self(seed.max(1))
    }

    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }

    /// Uniform 0..1.
    pub fn unit(&mut self) -> f32 {
        (self.next() >> 40) as f32 / (1u64 << 24) as f32
    }

    pub fn chance(&mut self, p: f32) -> bool {
        self.unit() < p
    }

    /// Two dice averaged: favours the middle of a range.
    fn middling(&mut self) -> f32 {
        (self.unit() + self.unit()) / 2.0
    }

    /// One of `n`, by weight.
    fn weighted(&mut self, weights: &[f32]) -> usize {
        let mut x = self.unit() * weights.iter().sum::<f32>();
        for (i, w) in weights.iter().enumerate() {
            if x < *w {
                return i;
            }
            x -= w;
        }
        weights.len() - 1
    }

    fn int(&mut self, lo: i32, hi: i32) -> f32 {
        (lo + (self.unit() * (hi - lo + 1) as f32) as i32).min(hi) as f32
    }
}

/// A setting's usual range for this kind of drum.
#[derive(Clone, Copy)]
enum Range {
    /// Linear, lo..hi.
    Lin(f32, f32),
    /// Even in log (pitches, times).
    Log(f32, f32),
}

/// The whole ranges, for the rare extreme draws (as voice.lib / plugin-fx.dsp).
fn whole(id: &str) -> (f32, f32, bool) {
    match id {
        "tone.pitch" => (20.0, 2000.0, true),
        "tone.drop" => (0.0, 72.0, false),
        "tone.fast_time" => (1.0, 20.0, true),
        "tone.slow_time" => (20.0, 500.0, true),
        "tone.decay" | "noise.decay" | "body.decay" => (20.0, 4000.0, true),
        "noise.bandpass" => (100.0, 12000.0, true),
        "noise.highpass" => (20.0, 8000.0, true),
        "body.offset" | "noise.loop_pitch" | "sample_1.pitch" | "sample_2.pitch" => (-24.0, 24.0, false),
        "sample_1.decay" | "sample_2.decay" => (5.0, 2000.0, true),
        "tone.attack" | "noise.attack" => (0.0, 10.0, false),
        "envelope.attack" => (0.0, 1000.0, false),
        "envelope.decay" => (5.0, 4000.0, true),
        "compressor.threshold" => (-40.0, 0.0, false),
        "filter.cutoff" => (20.0, 20_000.0, true),
        _ => (0.0, 1.0, false),
    }
}

struct Builder<'a> {
    dice: &'a mut Dice,
    patch: Patch,
}

impl Builder<'_> {
    fn draw(&mut self, id: &'static str, range: Range) -> f32 {
        let (min, max, log) = whole(id);
        let (lo, hi, log) = if self.dice.chance(EXTREME) {
            (min, max, log)
        } else {
            match range {
                Range::Lin(lo, hi) => (lo, hi, false),
                Range::Log(lo, hi) => (lo, hi, true),
            }
        };
        let t = self.dice.middling();
        let v = if log && lo > 0.0 { lo * (hi / lo).powf(t) } else { lo + (hi - lo) * t };
        self.set(id, v.clamp(min, max))
    }

    /// A linear draw in lo..hi (whole range 0..1 unless `whole` says otherwise).
    fn lin(&mut self, id: &'static str, lo: f32, hi: f32) -> f32 {
        self.draw(id, Range::Lin(lo, hi))
    }

    fn log(&mut self, id: &'static str, lo: f32, hi: f32) -> f32 {
        self.draw(id, Range::Log(lo, hi))
    }

    /// `a` with probability `p`, else `b`.
    fn either(&mut self, id: &'static str, p: f32, a: f32, b: f32) -> f32 {
        let v = if self.dice.chance(p) { a } else { b };
        self.set(id, v)
    }

    fn set(&mut self, id: &'static str, v: f32) -> f32 {
        self.patch.retain(|(k, _)| *k != id);
        self.patch.push((id, v));
        v
    }

    fn sample_layer(&mut self, layer: u8, set: usize, level: (f32, f32), counts: [usize; 3]) {
        let g = if layer == 1 { ["sample_1.level", "sample_1.set", "sample_1.sample", "sample_1.pitch", "sample_1.decay"] }
                else { ["sample_2.level", "sample_2.set", "sample_2.sample", "sample_2.pitch", "sample_2.decay"] };
        self.lin(g[0], level.0, level.1);
        self.set(g[1], set as f32);
        let n = self.dice.int(1, counts[set] as i32);
        self.set(g[2], n);
        if self.dice.chance(0.3) {
            self.lin(g[3], -5.0, 5.0);
        }
        if self.dice.chance(0.3) {
            self.log(g[4], 60.0, 600.0);
        }
    }
}

/// The kinds of drum the die picks from, and how often.
const KINDS: [(&str, f32); 7] = [
    ("Kick", 0.22), ("Snare", 0.18), ("Hat", 0.18), ("Tom / perc", 0.13), ("Clap", 0.06), ("Metal", 0.10), ("Microsound", 0.13),
];

/// A random patch, and what kind of drum it is. `counts`: the sample sets' sizes.
pub fn roll(dice: &mut Dice, counts: [usize; 3]) -> (&'static str, Patch) {
    let kind = dice.weighted(&KINDS.map(|(_, w)| w));
    let mut b = Builder { dice, patch: Vec::new() };
    let (thump, knock, spark) = (0, 1, 2);
    match kind {
        0 => {
            // Kick: a low, falling sine with a long tail; maybe a click, a thump layer, a skin.
            b.lin("tone.level", 0.75, 1.0);
            b.log("tone.pitch", 34.0, 72.0);
            b.lin("tone.drop", 18.0, 58.0);
            b.lin("tone.split", 0.2, 0.7);
            b.log("tone.fast_time", 1.5, 6.0);
            b.log("tone.slow_time", 40.0, 200.0);
            b.log("tone.decay", 250.0, 1400.0);
            b.lin("tone.snap", 0.15, 0.5);
            b.lin("tone.tail_level", 0.2, 0.6);
            if b.dice.chance(0.35) {
                b.lin("tone.shape", 0.1, 0.6);
            }
            if b.dice.chance(0.4) {
                b.lin("noise.level", 0.03, 0.15);
                b.log("noise.bandpass", 1000.0, 4000.0);
                b.log("noise.decay", 20.0, 80.0);
            }
            if b.dice.chance(0.35) {
                b.sample_layer(1, thump, (0.15, 0.45), counts);
            }
            if b.dice.chance(0.15) {
                b.lin("body.level", 0.1, 0.3);
                b.lin("body.material", 0.0, 0.15);
                b.lin("body.offset", 0.0, 12.0);
                b.log("body.decay", 60.0, 250.0);
            }
        }
        1 => {
            // Snare: a short tonal hit and a longer noise tail; maybe a skin, a knock.
            b.lin("tone.level", 0.35, 0.7);
            b.log("tone.pitch", 140.0, 270.0);
            b.lin("tone.drop", 4.0, 20.0);
            b.lin("tone.split", 0.5, 0.9);
            b.log("tone.decay", 100.0, 320.0);
            b.lin("noise.level", 0.35, 0.8);
            let loopy = b.dice.chance(0.2);
            b.either("noise.generator", 0.75, 0.0, 1.0); // white, else pink
            if loopy {
                b.lin("noise.blend", 0.4, 1.0);
                let l = b.dice.int(0, 44);
                b.set("noise.loop", l);
            }
            b.log("noise.bandpass", 2500.0, 7000.0);
            b.lin("noise.q", 0.15, 0.5);
            b.log("noise.highpass", 500.0, 2500.0);
            b.lin("noise.env_amount", 0.0, 0.4);
            b.log("noise.decay", 140.0, 450.0);
            b.lin("noise.snap", 0.15, 0.4);
            b.lin("noise.tail_level", 0.2, 0.45);
            if b.dice.chance(0.4) {
                b.lin("body.level", 0.2, 0.4);
                b.lin("body.offset", 4.0, 12.0);
                b.log("body.decay", 80.0, 220.0);
                b.lin("body.strike", 0.5, 0.85);
            }
            if b.dice.chance(0.4) {
                b.sample_layer(1, knock, (0.2, 0.5), counts);
            }
        }
        2 => {
            // Hat: high noise or 808-style metal, closed (mostly) or open.
            b.set("tone.level", 0.0);
            b.lin("noise.level", 0.8, 1.0);
            let generator = [4.0, 3.0, 0.0, 2.0][b.dice.weighted(&[0.5, 0.2, 0.2, 0.1])];
            b.set("noise.generator", generator);
            b.log("tone.pitch", 300.0, 620.0);
            b.log("noise.bandpass", 6000.0, 11000.0);
            b.lin("noise.q", 0.2, 0.6);
            b.log("noise.highpass", 4000.0, 7500.0);
            if b.dice.chance(0.7) {
                b.log("noise.decay", 25.0, 90.0);
            } else {
                b.log("noise.decay", 150.0, 700.0);
            }
            b.lin("noise.snap", 0.2, 0.5);
            b.lin("noise.tail_level", 0.2, 0.4);
            if b.dice.chance(0.2) {
                b.lin("noise.crush", 0.1, 0.35);
            }
            if b.dice.chance(0.3) {
                b.sample_layer(1, spark, (0.2, 0.5), counts);
            }
            if b.dice.chance(0.25) {
                b.lin("body.level", 0.15, 0.3);
                b.lin("body.material", 0.5, 1.0);
                b.lin("body.offset", 12.0, 24.0);
                b.log("body.decay", 50.0, 200.0);
                b.lin("body.strike", 0.8, 1.0);
            }
        }
        3 => {
            // Tom or percussion: a pitched hit, often with a skin.
            b.lin("tone.level", 0.55, 0.9);
            b.log("tone.pitch", 75.0, 320.0);
            b.lin("tone.drop", 2.0, 12.0);
            b.log("tone.decay", 180.0, 650.0);
            b.lin("tone.tail_level", 0.25, 0.5);
            if b.dice.chance(0.5) {
                b.lin("body.level", 0.25, 0.5);
                b.lin("body.material", 0.0, 0.3);
                b.log("body.decay", 180.0, 500.0);
            }
            if b.dice.chance(0.4) {
                b.lin("noise.level", 0.04, 0.12);
                b.log("noise.bandpass", 600.0, 2000.0);
                b.log("noise.decay", 30.0, 200.0);
            }
            if b.dice.chance(0.3) {
                b.sample_layer(1, knock, (0.15, 0.4), counts);
            }
        }
        4 => {
            // Clap: bursts of band-passed noise, then a tail.
            b.set("tone.level", 0.0);
            b.set("noise.level", 1.0);
            b.log("noise.bandpass", 900.0, 1900.0);
            b.lin("noise.q", 0.35, 0.6);
            b.log("noise.highpass", 250.0, 650.0);
            b.lin("noise.clap", 0.45, 0.9);
            b.set("noise.env_amount", 0.0);
            b.log("noise.decay", 280.0, 650.0);
            b.lin("noise.tail_level", 0.3, 0.5);
        }
        5 => {
            // Metal: a plate or bar ringing, sometimes ring-modded or combed.
            b.either("tone.level", 0.3, 0.2, 0.0);
            b.log("tone.pitch", 180.0, 900.0);
            b.lin("body.level", 0.8, 1.0);
            b.lin("body.material", 0.45, 1.0);
            b.lin("body.offset", 0.0, 19.0);
            b.log("body.decay", 250.0, 1500.0);
            b.lin("body.strike", 0.6, 1.0);
            b.lin("body.damping", 0.3, 0.65);
            if b.dice.chance(0.3) {
                b.lin("ring_mod.amount", 0.15, 0.5);
            }
            if b.dice.chance(0.2) {
                b.lin("comb.amount", 0.2, 0.6);
            }
            if b.dice.chance(0.3) {
                b.sample_layer(2, spark, (0.1, 0.3), counts);
                b.lin("sample_2.body_send", 0.5, 1.0);
            }
        }
        _ => {
            // Microsound: clicks, grains, crackle; short and bright.
            b.either("tone.level", 0.3, 0.3, 0.0);
            b.log("tone.pitch", 600.0, 2000.0);
            b.log("tone.decay", 20.0, 90.0);
            b.lin("noise.level", 0.7, 1.0);
            if b.dice.chance(0.5) {
                b.set("noise.generator", 2.0); // dust
            } else {
                b.lin("noise.blend", 0.7, 1.0);
                let l = b.dice.int(17, 44); // crackle and modulated loops
                b.set("noise.loop", l);
            }
            b.lin("noise.density", 0.4, 0.95);
            b.log("noise.bandpass", 3000.0, 10000.0);
            b.lin("noise.q", 0.4, 0.9);
            b.log("noise.highpass", 1500.0, 5000.0);
            b.log("noise.decay", 10.0, 70.0);
            b.lin("noise.tail_level", 0.0, 0.3);
            if b.dice.chance(0.5) {
                b.sample_layer(1, spark, (0.3, 0.7), counts);
            }
            if b.dice.chance(0.3) {
                b.lin("body.level", 0.3, 0.6);
                b.lin("body.material", 0.7, 1.0);
                b.log("body.decay", 30.0, 120.0);
            }
            if b.dice.chance(0.3) {
                b.lin("crush.amount", 0.1, 0.5);
            }
            if b.dice.chance(0.2) {
                b.lin("ring_mod.amount", 0.2, 0.6);
            }
        }
    }
    season(&mut b);
    (KINDS[kind].0, b.patch)
}

/// Knobs a ring may land on, with how far it usually goes.
const RING_TARGETS: [(&str, f32); 14] = [
    ("tone.pitch_mod", 0.25), ("tone.shape_mod", 0.5), ("tone.decay_mod", 0.3), ("body.material_mod", 0.4),
    ("body.offset_mod", 0.2), ("body.decay_mod", 0.3), ("noise.bandpass_mod", 0.4), ("noise.q_mod", 0.4),
    ("noise.blend_mod", 0.5), ("noise.density_mod", 0.4), ("sample_1.pitch_mod", 0.2), ("crush.amount_mod", 0.4),
    ("drive.amount_mod", 0.4), ("comb.amount_mod", 0.4),
];

/// Variation, the envelope, an effect or two, a ring or two.
fn season(b: &mut Builder) {
    b.lin("voice.variation", 0.1, 0.4);
    b.log("envelope.decay", 60.0, 600.0);
    b.lin("envelope.curve", 0.3, 0.9);
    b.lin("envelope.velocity", 0.3, 0.8);
    if b.dice.chance(0.2) {
        b.lin("envelope.attack", 0.0, 20.0);
    }
    if b.dice.chance(0.35) {
        let t = b.dice.int(0, 2);
        b.set("drive.type", t);
        b.lin("drive.amount", 0.1, 0.5);
    }
    if b.dice.chance(0.3) {
        b.lin("compressor.threshold", -24.0, -6.0);
        let t = b.dice.int(0, 4);
        b.set("compressor.timing", t);
    }
    if b.dice.chance(0.08) {
        b.lin("crush.amount", 0.05, 0.4);
    }
    // Now and then the SH-101 filter, swept open by the envelope: a 101 pluck.
    if b.dice.chance(0.15) {
        b.log("filter.cutoff", 150.0, 3000.0);
        b.lin("filter.resonance", 0.3, 0.85);
        b.lin("filter.cutoff_mod", 0.2, 0.6);
    }
    for _ in 0..2 {
        if b.dice.chance(0.45) {
            let (id, reach) = RING_TARGETS[b.dice.int(0, RING_TARGETS.len() as i32 - 1) as usize];
            let depth = if b.dice.chance(EXTREME * 2.0) { 0.8 } else { reach * (0.3 + 0.7 * b.dice.unit()) };
            let sign = if b.dice.chance(0.5) { 1.0 } else { -1.0 };
            b.set(id, sign * depth);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::params::DrumParams;

    #[test]
    fn every_roll_names_real_parameters_within_their_ranges() {
        let params = DrumParams::from_faust();
        let mut dice = Dice::new(12345);
        let mut kinds = std::collections::HashSet::new();
        for _ in 0..500 {
            let (kind, patch) = roll(&mut dice, [58, 55, 51]);
            kinds.insert(kind);
            for (id, v) in patch {
                let p = params.ptr(id).unwrap_or_else(|| panic!("no parameter {id}"));
                let n = unsafe { p.preview_normalized(v) };
                assert!(v.is_finite() && (-0.001..=1.001).contains(&n), "{id} = {v} is out of range");
                assert_ne!(id, "master.volume");
            }
        }
        assert_eq!(kinds.len(), KINDS.len(), "every kind comes up");
    }

    #[test]
    fn rolled_patches_sound_and_stay_sane() {
        use crate::engine::{Engine, MAX_BLOCK};
        let params = DrumParams::from_faust();
        let mut dice = Dice::new(777);
        for _ in 0..40 {
            let (kind, patch) = roll(&mut dice, [58, 55, 51]);
            let mut e = Engine::new(params.slots());
            e.init(44_100.0);
            e.apply(&params);
            for (id, v) in &patch {
                if let Some((target, index)) = params.faust_index(id) {
                    e.set_param(target, index, *v);
                }
            }
            if let Some((_, v)) = patch.iter().find(|(id, _)| *id == "tone.pitch") {
                e.set_tone(*v, false);
            }
            e.note_on(60, 1.0);
            let mut out = vec![0.0f32; 22_050];
            for c in out.chunks_mut(MAX_BLOCK) {
                e.render(c);
            }
            let peak = out.iter().fold(0.0f32, |m, s| m.max(s.abs()));
            assert!(out.iter().all(|s| s.is_finite()), "{kind}: non-finite");
            assert!(peak > 0.02, "{kind}: nearly silent ({peak}): {patch:?}");
        }
    }
}
