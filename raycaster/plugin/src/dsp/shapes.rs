//! The six elements, and Chaos: three of them overlapped, each bent, sized and placed at
//! random from a seed.

use nih_plug::prelude::Enum;

use super::geometry::{bend_points, Bend, Outline, Point};
use super::random::Rng;
use super::shapes_data::{AIR, EARTH, FIRE, METAL, WATER, WOOD};

#[derive(Enum, Clone, Copy, PartialEq, Eq, Debug)]
pub enum Element {
    #[id = "earth"]
    Earth,
    #[id = "water"]
    Water,
    #[id = "air"]
    Air,
    #[id = "fire"]
    Fire,
    #[id = "wood"]
    Wood,
    #[id = "metal"]
    Metal,
    #[id = "chaos"]
    Chaos,
}

pub const ELEMENTS: [Element; 6] = [Element::Earth, Element::Water, Element::Air, Element::Fire, Element::Wood, Element::Metal];
pub const ALL: [Element; 7] = [
    Element::Earth,
    Element::Water,
    Element::Air,
    Element::Fire,
    Element::Wood,
    Element::Metal,
    Element::Chaos,
];

impl Element {
    pub fn name(self) -> &'static str {
        match self {
            Element::Earth => "Earth",
            Element::Water => "Water",
            Element::Air => "Air",
            Element::Fire => "Fire",
            Element::Wood => "Wood",
            Element::Metal => "Metal",
            Element::Chaos => "Chaos",
        }
    }

    pub fn index(self) -> usize {
        self as usize
    }

    /// The outline of one of the six (Chaos has none of its own).
    pub fn outline(self) -> Outline {
        let loops = match self {
            Element::Earth => EARTH,
            Element::Water => WATER,
            Element::Air => AIR,
            Element::Fire => FIRE,
            Element::Wood => WOOD,
            Element::Metal => METAL,
            Element::Chaos => EARTH,
        };
        Outline::from_loops(loops.iter().copied())
    }
}

pub const MAX_SEED: i32 = 99_999;
pub const PARTS: usize = 3;

/// Three distinct elements, each squished, skewed and tilted at random, sized 0.45..1 and
/// moved up to 0.6 either way, overlapped and fitted inside -1..1.
pub fn chaos(seed: u32) -> (Outline, [Element; PARTS]) {
    let mut rng = Rng::new(seed.wrapping_mul(2_654_435_761).wrapping_add(17));
    let mut picks = [Element::Earth; PARTS];
    let mut taken = 0;
    while taken < PARTS {
        let pick = ELEMENTS[rng.below(ELEMENTS.len())];
        if !picks[..taken].contains(&pick) {
            picks[taken] = pick;
            taken += 1;
        }
    }
    let mut loops: Vec<Vec<Point>> = Vec::new();
    for pick in picks {
        let outline = pick.outline();
        let bend = Bend { squish: rng.range(-0.7, 0.7), skew: rng.range(-0.8, 0.8), tilt: rng.range(-0.8, 0.8) };
        let (mut xs, mut ys) = (vec![0.0; outline.len()], vec![0.0; outline.len()]);
        bend_points(&outline.xs, &outline.ys, bend, &mut xs, &mut ys);
        let size = rng.range(0.45, 1.0);
        let (dx, dy) = (rng.range(-0.6, 0.6), rng.range(-0.6, 0.6));
        let mut start = 0;
        for &end in &outline.ends {
            loops.push((start..end).map(|i| [xs[i] * size + dx, ys[i] * size + dy]).collect());
            start = end;
        }
    }
    let merged = Outline::from_loops(loops.iter().map(|l| &l[..]));
    // A bend of nothing just fits the overlap back inside -1..1.
    let mut fitted = merged.clone();
    bend_points(&merged.xs, &merged.ys, Bend::default(), &mut fitted.xs, &mut fitted.ys);
    (fitted, picks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_element_has_an_outline() {
        for e in ELEMENTS {
            let o = e.outline();
            assert!(o.len() >= 4 && !o.ends.is_empty(), "{}", e.name());
            assert!(o.xs.iter().chain(&o.ys).all(|v| v.abs() <= 1.0001));
        }
        assert_eq!(Element::Earth.outline().ends.len(), 2, "the O has a hole");
    }

    #[test]
    fn chaos_is_repeatable_and_mixes_three_different_elements() {
        let (a, picks) = chaos(4821);
        let (b, again) = chaos(4821);
        assert_eq!(picks, again);
        assert_eq!(a.xs, b.xs);
        assert!(picks[0] != picks[1] && picks[1] != picks[2] && picks[0] != picks[2]);
        assert!(a.xs.iter().chain(&a.ys).all(|v| v.abs() <= 1.0001));
        let (_, other) = chaos(4822);
        assert!(other != picks || chaos(4822).0.xs != a.xs);
    }
}
