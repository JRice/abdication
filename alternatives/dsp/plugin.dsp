// plugin.dsp: Alternatives for the Rust plugin (alternatives.lib). Stereo in,
// stereo out. The plugin sets every control here from its own parameters, and
// the notes (Root, or the MIDI notes held) itself; the labels are what it
// looks them up by.

import("stdfaust.lib");
al = library("alternatives.lib");

// Control smoothing that starts at the control's value (after a reset too), so
// nothing glides in from the DSP's own defaults, and lands on it exactly once
// it's within a millionth (a one-pole only ever nears it).
smoo = _ <: si.smooth(0.999 * (1 - first)), _ : \(y, x).(select2(abs(y - x) < 1e-6, y, x))
with { first = 1 - 1'; };

// Set by the plugin once, before the first sample: this instance's own stream.
seed = hslider("seed", 1000, 0, 1000000, 1);

notes = par(i, al.maxNotes, hslider("note%i", 130.81, 20, 8000, 0.01));
count = hslider("count", 1, 1, al.maxNotes, 1);

slots   = hslider("slots", 6, 1, al.maxSlots, 1);
density = hslider("density", 80, 0, 100, 0.1) : smoo : /(100);
depth   = hslider("depth", 6, 0, 18, 0.01) : smoo;
reach   = hslider("reach", 64, 1, 128, 1) : smoo;
life    = hslider("life", 2, 0.25, 4, 0.01) : smoo;
sharp   = hslider("sharp", 1, 0.5, 4, 0.01) : smoo;
cut     = hslider("cut", 3.33, 1, 6, 0.01) : smoo;
hold    = checkbox("hold");
// The walk wanders by itself around Walk, by up to Drift, every few seconds
// (Abdication's: 0.605 +- 0.275).
walk    = al.clip(0, 1, hslider("walk", 0.6, 0, 1, 0.001) : smoo)
          + (hslider("drift", 0.275, 0, 0.5, 0.001) : smoo) * al.wander(seed + 7, 1 / 3) : al.clip(0, 1);
mix     = hslider("mix", 1, 0, 1, 0.001) : smoo;
output  = hslider("output", 0, -24, 12, 0.01) : ba.db2linear : smoo;

wet = al.alternatives(seed, notes, count, slots, density, depth, reach, life, walk, sharp, cut, hold);

process = _, _ <: wet, _, _ : route(4, 4, (1, 1), (2, 3), (3, 2), (4, 4)) : blend, blend
with { blend(w, d) = (d + mix * (w - d)) * output; };
