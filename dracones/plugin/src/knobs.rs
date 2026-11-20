//! The panel's knobs, shared by the parameters, the engine and the panel. The 25 armed
//! knobs each carry four modulation arms (one per global modulator); their ids are the
//! mockup's, and they seed where each knob's arms point, so the arms never move between
//! sessions.

/// One armed knob.
#[derive(Clone, Copy, Debug)]
pub struct Knob {
    pub id: &'static str,
    pub name: &'static str,
    pub default: f32,
}

const fn k(id: &'static str, name: &'static str, default: f32) -> Knob {
    Knob { id, name, default }
}

pub const ARMED_COUNT: usize = 25;

/// Oscillator `o`'s five (Pitch, Gain, Position/Fold, Cutoff, Resonance) are 5o..5o+5;
/// then each effect's Feedback and Custom; then Fuzz Gain, Fuzz HP, Morph Slant and
/// Reverb Time.
pub const ARMED: [Knob; ARMED_COUNT] = [
    k("o0_pitch", "I Pitch", 0.5),
    k("o0_gain", "I Gain", 0.78),
    k("o0_cus", "I Position", 0.3),
    k("o0_cut", "I Cutoff", 0.6),
    k("o0_res", "I Resonance", 0.25),
    k("o1_pitch", "II Pitch", 0.5),
    k("o1_gain", "II Gain", 0.6),
    k("o1_cus", "II Position", 0.55),
    k("o1_cut", "II Cutoff", 0.5),
    k("o1_res", "II Resonance", 0.35),
    k("o2_pitch", "III Pitch", 0.5),
    k("o2_gain", "III Gain", 0.45),
    k("o2_cus", "III Fold", 0.4),
    k("o2_cut", "III Cutoff", 0.45),
    k("o2_res", "III Resonance", 0.3),
    k("f0_fb", "Chorus Feedback", 0.45),
    k("f0_cus", "Chorus Amount", 0.5),
    k("f1_fb", "Tube Feedback", 0.55),
    k("f1_cus", "Tube Drive", 0.4),
    k("f2_fb", "Phaser Feedback", 0.4),
    k("f2_cus", "Phaser Speed", 0.35),
    k("p_fg", "Fuzz Gain", 0.15),
    k("p_fh", "Fuzz HP", 0.2),
    k("p_sl", "Morph Slant", 0.5),
    k("p_rt", "Reverb Time", 0.62),
];

pub const PITCH: usize = 0;
pub const GAIN: usize = 1;
pub const CUSTOM: usize = 2;
pub const CUTOFF: usize = 3;
pub const RESONANCE: usize = 4;

/// Oscillator `o`'s knob `p` (PITCH .. RESONANCE).
pub const fn osc(o: usize, p: usize) -> usize {
    5 * o + p
}
/// Effect `e`'s Feedback and Custom.
pub const fn feedback(e: usize) -> usize {
    15 + 2 * e
}
pub const fn custom(e: usize) -> usize {
    16 + 2 * e
}
pub const FUZZ_GAIN: usize = 21;
pub const FUZZ_HP: usize = 22;
pub const SLANT: usize = 23;
pub const REVERB_TIME: usize = 24;

/// The row whose red and slate copies knob `i` follows: an oscillator's own row, an
/// insert effect's docked row (`rows[e]`, from the Insert Order), the bottom row copy I.
pub fn row(i: usize, rows: [usize; 3]) -> usize {
    match i {
        0..=14 => i / 5,
        15..=20 => rows[(i - 15) / 2],
        _ => 0,
    }
}

pub fn is_pitch(i: usize) -> bool {
    i < 15 && i % 5 == PITCH
}

/// The modulators' names as the host sees them, in arm order.
pub const MOD_NAMES: [&str; 4] = ["Silver", "Red", "Green", "Slate"];
pub const MOD_IDS: [&str; 4] = ["silver", "red", "green", "slate"];

/// Where knob `i` is with its modulators applied: depth 1 moves it up to half its
/// range. Red is unipolar (0..1), so it only pushes upward; the others are bipolar.
/// Pitch arms follow a cubic law, so small depths stay within cents.
pub fn modulated(i: usize, value: f32, depths: &[f32; 4], mods: &[f32; 4]) -> f32 {
    let mut v = value;
    for m in 0..4 {
        let d = depths[m].clamp(0.0, 1.0);
        if d > 0.0 {
            let d = if is_pitch(i) { d * d * d } else { d };
            v += 0.5 * d * mods[m];
        }
    }
    v.clamp(0.0, 1.0)
}

/// mulberry32, as the mockup's script has it.
pub struct Mulberry(u32);

impl Mulberry {
    pub fn new(seed: u32) -> Self {
        Self(seed)
    }

    pub fn next(&mut self) -> f32 {
        self.0 = self.0.wrapping_add(0x6D2B_79F5);
        let mut t = self.0;
        t = (t ^ (t >> 15)).wrapping_mul(1 | t);
        t = t.wrapping_add((t ^ (t >> 7)).wrapping_mul(61 | t)) ^ t;
        ((t ^ (t >> 14)) as f64 / 4_294_967_296.0) as f32
    }
}

/// FNV-1a over the id (the mockup's hashStr).
pub fn hash(s: &str) -> u32 {
    s.bytes().fold(2_166_136_261u32, |h, b| (h ^ b as u32).wrapping_mul(16_777_619))
}

/// Where a knob's four arms point (radians, clockwise from +x as the screen has it),
/// random per knob but seeded from its id, and at least 54° apart.
pub fn arm_angles(id: &str) -> [f32; 4] {
    let mut r = Mulberry::new(hash(id));
    let mut out: Vec<f32> = Vec::with_capacity(4);
    let mut guard = 0;
    while out.len() < 4 && guard < 800 {
        guard += 1;
        let a = r.next() * std::f32::consts::TAU;
        let ok = out.iter().all(|b| {
            let d = (a - b).abs() % std::f32::consts::TAU;
            d.min(std::f32::consts::TAU - d) > 0.95
        });
        if ok {
            out.push(a);
        }
    }
    while out.len() < 4 {
        out.push(out.len() as f32 * std::f32::consts::FRAC_PI_2);
    }
    [out[0], out[1], out[2], out[3]]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rows_follow_the_docks() {
        let rows = [2, 0, 1];
        assert_eq!(row(osc(1, CUTOFF), rows), 1);
        assert_eq!(row(feedback(0), rows), 2);
        assert_eq!(row(custom(2), rows), 1);
        assert_eq!(row(REVERB_TIME, rows), 0);
    }

    #[test]
    fn indices() {
        assert_eq!(ARMED[osc(2, CUSTOM)].name, "III Fold");
        assert_eq!(ARMED[feedback(1)].name, "Tube Feedback");
        assert_eq!(ARMED[custom(2)].name, "Phaser Speed");
        assert_eq!(ARMED[REVERB_TIME].id, "p_rt");
        assert!((0..15).filter(|&i| is_pitch(i)).count() == 3);
    }

    #[test]
    fn arms_spread_and_stay_put() {
        for k in ARMED {
            let a = arm_angles(k.id);
            assert_eq!(a, arm_angles(k.id));
            for i in 0..4 {
                for j in 0..i {
                    let d = (a[i] - a[j]).abs() % std::f32::consts::TAU;
                    assert!(d.min(std::f32::consts::TAU - d) > 0.94, "{}", k.id);
                }
            }
        }
    }

    #[test]
    fn modulation_reaches_half_and_pitch_is_gentle() {
        let full = [1.0, 0.0, 0.0, 0.0];
        assert!((modulated(osc(0, CUTOFF), 0.5, &full, &[1.0, 0.0, 0.0, 0.0]) - 1.0).abs() < 1e-6);
        assert!((modulated(osc(0, PITCH), 0.5, &[0.2, 0.0, 0.0, 0.0], &[1.0; 4]) - 0.504).abs() < 1e-6);
        // Red only pushes up.
        assert!(modulated(5, 0.5, &[0.0, 1.0, 0.0, 0.0], &[0.0, 0.0, 0.0, 0.0]) == 0.5);
    }
}
