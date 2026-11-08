// plugin-body.dsp — the Body (drum.lib's modal resonator) as its own DSP.
// The voice computes its exciter, level and settings; the plugin passes the
// settings in once per block (so the 24 modes' coefficients are computed
// per block, not per sample) and runs this only while the Body sounds.
//
// Inputs: exciter, level. body_gate: a pulse on each hit (the coefficients
// jump to the new hit's instead of gliding there).

declare name "ABDrumBody";
import("stdfaust.lib");
dr = library("drum.lib");

hz       = hslider("body_hz", 100, 0, 100000, 0.001);
material = hslider("body_material", 0, 0, 1, 0.0001);
t60      = hslider("body_t60", 0.4, 0, 100, 0.0001);
bright   = hslider("body_bright", 0.4, 0, 1, 0.0001);
position = hslider("body_position", 0.3, 0, 1, 0.0001);
snap     = button("body_gate") <: _ > _';

process(exciter, level) = exciter : dr.body(hz, material, t60, bright, position, snap) : *(level);
