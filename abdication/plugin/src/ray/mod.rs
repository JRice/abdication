//! Raycaster's oscillator (ported from Raycaster's dsp: geometry, rayosc,
//! shapes and their outlines), as a third engine for an oscillator slot. A
//! shape turns once per cycle about a center; a ray dropped from above reads
//! how far it falls before it hits an edge, and that distance is the wave.
//! See README ("Ray").

pub mod geometry;
pub mod random;
pub mod rayosc;
pub mod shapes;
pub mod shapes_data;
pub mod player;
