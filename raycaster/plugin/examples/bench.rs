//! How long one ray takes on each element (nanoseconds), for keeping the ray cheap.
use std::time::Instant;

use raycaster::dsp::geometry::{Bend, Bent};
use raycaster::dsp::shapes::ELEMENTS;

fn main() {
    for e in ELEMENTS {
        let outline = e.outline();
        let mut b = Box::<Bent>::default();
        b.bend(&outline, Bend { squish: 0.2, skew: 0.1, tilt: 0.1 });
        let center = [b.centroid[0] + 0.3, b.centroid[1]];
        let scale = 2.0 / b.reach(center);
        let n = 2_000_000;
        let started = Instant::now();
        let mut sum = 0.0f32;
        for i in 0..n {
            sum += b.read(i as f32 * 0.000_731, center, scale, 0.4);
        }
        let ns = started.elapsed().as_nanos() as f64 / n as f64;
        println!("{:6} {:4} edges: {ns:6.1} ns per ray ({sum:.1})", e.name(), outline.len());
    }
}
