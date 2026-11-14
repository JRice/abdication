// plugin-reverb.dsp: the calls' reverb, for the Rust plugin: Zita's, as in
// Abdication's calls bus, with its settings opened. Inputs: the calls and
// their echo, left and right. Outputs: those through the reverb, by Mix (1, as
// Abdication had it, is all reverb: the calls are heard only through it).

import("stdfaust.lib");

// Control smoothing that starts at the control's value (after a reset too), so
// nothing glides in from the DSP's own defaults, and lands on it exactly once
// it's within a millionth (a one-pole only ever nears it).
smoo = _ <: si.smooth(0.999 * (1 - first)), _ : \(y, x).(select2(abs(y - x) < 1e-6, y, x))
with { first = 1 - 1'; };

// Zita's pre-delay line holds 8192 samples.
predelay = hslider("predelay", 30, 0, 150, 0.1) : min(8000 / ma.SR * 1000);
decay    = hslider("decay", 5, 0.5, 20, 0.01);
damp     = hslider("damp", 2500, 1000, 12000, 1);
mix      = hslider("mix", 1, 0, 1, 0.001) : smoo;

// Lows (below 300 Hz) decay a little faster than the mids, as Abdication's did (4 s and 5 s).
reverb = re.zita_rev1_stereo(predelay, 300, damp, 0.8 * decay, decay, 96000);

process = _, _ <: reverb, _, _ : route(4, 4, (1, 1), (2, 3), (3, 2), (4, 4)) : blend, blend
with { blend(w, d) = d + mix * (w - d); };
