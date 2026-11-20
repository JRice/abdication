//! The sound, with no plugin in it. New here: the Argon tables, the folder, the filters,
//! the modulators, the insert delays and the fuzz. Brought from the siblings: Argent's
//! Fizmo tables and oscillator (`tables`, `osc`, `random`), Abdication's Pitch law and
//! Harmonic EQ (`pitch`, `harmonic`, `harmlib`; their data is read from Abdication's
//! folder), and Apse's reverb (`apse`, whose Plate mode is used).

pub mod apse;
pub mod argon;
pub mod filters;
pub mod fold;
pub mod fuzz;
pub mod harmlib;
pub mod harmonic;
pub mod inserts;
pub mod mods;
pub mod osc;
pub mod pitch;
pub mod random;
pub mod tables;
