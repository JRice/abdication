//! The controls that each have their own dynamic modulator, and their ranges. The host
//! parameters are built from these same ranges, so the engine can move a control in its
//! normalized travel and get back exactly the value the knob would show.

pub const OSCS: usize = 2;

/// Per oscillator, in panel order.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OscTarget {
    Fine,
    Level,
    Squish,
    Skew,
    Tilt,
    Center,
    Ray,
    Orbit,
    OrbitRate,
}

pub const OSC_TARGETS: [OscTarget; 9] = [
    OscTarget::Fine,
    OscTarget::Level,
    OscTarget::Squish,
    OscTarget::Skew,
    OscTarget::Tilt,
    OscTarget::Center,
    OscTarget::Ray,
    OscTarget::Orbit,
    OscTarget::OrbitRate,
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GlobalTarget {
    Drive,
    Cutoff,
    Resonance,
    FilterEnv,
    Width,
    Chorus,
    /// The SVF's response, lowpass through bandpass to highpass. Last, so adding it kept
    /// every other control's modulator where saved patches expect it.
    FilterMode,
}

pub const GLOBAL_TARGETS: [GlobalTarget; 7] = [
    GlobalTarget::Drive,
    GlobalTarget::Cutoff,
    GlobalTarget::Resonance,
    GlobalTarget::FilterEnv,
    GlobalTarget::Width,
    GlobalTarget::Chorus,
    GlobalTarget::FilterMode,
];

pub const TARGETS: usize = OSCS * OSC_TARGETS.len() + GLOBAL_TARGETS.len();

pub const fn osc_target(osc: usize, target: OscTarget) -> usize {
    osc * OSC_TARGETS.len() + target as usize
}

pub const fn global_target(target: GlobalTarget) -> usize {
    OSCS * OSC_TARGETS.len() + target as usize
}

/// A control's travel: `min..max`, curved by `factor` exactly as nih-plug's skewed range
/// (normalized = ((v - min) / (max - min)) ^ factor). A factor of 1 is linear.
#[derive(Clone, Copy, Debug)]
pub struct Range {
    pub min: f32,
    pub max: f32,
    pub factor: f32,
    pub default: f32,
}

impl Range {
    pub const fn linear(min: f32, max: f32, default: f32) -> Self {
        Self { min, max, factor: 1.0, default }
    }

    pub fn normalize(&self, v: f32) -> f32 {
        (((v - self.min) / (self.max - self.min)).clamp(0.0, 1.0)).powf(self.factor)
    }

    pub fn unnormalize(&self, n: f32) -> f32 {
        let n = n.clamp(0.0, 1.0);
        let n = if self.factor == 1.0 { n } else { n.powf(1.0 / self.factor) };
        self.min + n * (self.max - self.min)
    }

    pub fn bipolar(&self) -> bool {
        self.min < 0.0 && self.max > 0.0
    }
}

/// 2^x, as nih-plug's `skew_factor`.
pub fn skew(x: f32) -> f32 {
    x.exp2()
}

pub fn osc_range(target: OscTarget) -> Range {
    match target {
        OscTarget::Fine => Range::linear(-100.0, 100.0, 0.0),
        OscTarget::Level => Range::linear(0.0, 1.0, 0.8),
        OscTarget::Squish | OscTarget::Skew | OscTarget::Tilt => Range::linear(-1.0, 1.0, 0.0),
        OscTarget::Center => Range::linear(-1.3, 1.3, 0.0),
        OscTarget::Ray => Range::linear(0.0, 1.5, 0.0),
        OscTarget::Orbit => Range::linear(0.0, 0.8, 0.0),
        OscTarget::OrbitRate => ORBIT_HZ,
    }
}

/// How fast the center circles, in hertz.
pub const ORBIT_HZ: Range = Range { min: 0.02, max: 1_000.0, factor: 0.2, default: 0.3 };

pub fn global_range(target: GlobalTarget) -> Range {
    match target {
        GlobalTarget::Drive => Range::linear(0.0, 1.0, 0.0),
        GlobalTarget::Cutoff => CUTOFF,
        GlobalTarget::Resonance => Range::linear(0.0, 1.0, 0.15),
        GlobalTarget::FilterEnv => Range::linear(-1.0, 1.0, 0.3),
        GlobalTarget::Width => Range::linear(0.0, 1.0, 0.4),
        // Past a third of the delay the sweep is more vibrato than chorus.
        GlobalTarget::Chorus => Range::linear(0.0, 0.33, 0.0),
        GlobalTarget::FilterMode => Range::linear(0.0, 1.0, 0.0),
    }
}

pub const CUTOFF: Range = Range { min: 20.0, max: 20_000.0, factor: 0.23, default: 2_400.0 };

/// The range of target `index`.
pub fn range_of(index: usize) -> Range {
    if index < OSCS * OSC_TARGETS.len() {
        osc_range(OSC_TARGETS[index % OSC_TARGETS.len()])
    } else {
        global_range(GLOBAL_TARGETS[index - OSCS * OSC_TARGETS.len()])
    }
}

pub fn osc_label(target: OscTarget) -> &'static str {
    match target {
        OscTarget::Fine => "Fine",
        OscTarget::Level => "Level",
        OscTarget::Squish => "Squish",
        OscTarget::Skew => "Skew",
        OscTarget::Tilt => "Tilt",
        OscTarget::Center => "Center",
        OscTarget::Ray => "Ray",
        OscTarget::Orbit => "Orbit",
        OscTarget::OrbitRate => "Rate",
    }
}

pub fn global_label(target: GlobalTarget) -> &'static str {
    match target {
        GlobalTarget::Drive => "Drive",
        GlobalTarget::Cutoff => "Cutoff",
        GlobalTarget::Resonance => "Resonance",
        GlobalTarget::FilterEnv => "Env",
        GlobalTarget::Width => "Width",
        GlobalTarget::Chorus => "Chorus",
        GlobalTarget::FilterMode => "Mode",
    }
}

/// "I Squish", "Cutoff": what the panel and the host call target `index`.
pub fn target_name(index: usize) -> String {
    if index < OSCS * OSC_TARGETS.len() {
        let (osc, k) = (index / OSC_TARGETS.len(), OSC_TARGETS[index % OSC_TARGETS.len()]);
        let label = if k == OscTarget::OrbitRate { "Orbit Rate" } else { osc_label(k) };
        format!("{} {label}", ROMAN[osc])
    } else {
        match GLOBAL_TARGETS[index - OSCS * OSC_TARGETS.len()] {
            GlobalTarget::FilterEnv => "Filter Env".into(),
            GlobalTarget::FilterMode => "Filter Mode".into(),
            k => global_label(k).into(),
        }
    }
}

pub const ROMAN: [&str; OSCS] = ["I", "II"];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ranges_round_trip() {
        for r in [CUTOFF, ORBIT_HZ, Range::linear(-1.0, 1.0, 0.0)] {
            for v in [r.min, r.default, r.max, 0.5 * (r.min + r.max)] {
                assert!((r.unnormalize(r.normalize(v)) - v).abs() <= 1e-3 * (r.max - r.min), "{v}");
            }
        }
    }

    #[test]
    fn targets_are_numbered_without_gaps() {
        assert_eq!(osc_target(1, OscTarget::OrbitRate), 17);
        assert_eq!(global_target(GlobalTarget::FilterMode), TARGETS - 1);
        assert_eq!(target_name(osc_target(1, OscTarget::Squish)), "II Squish");
        assert_eq!(target_name(global_target(GlobalTarget::FilterEnv)), "Filter Env");
    }
}
