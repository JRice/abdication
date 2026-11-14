// plugin-calls.dsp: the callers, for the Rust plugin (calls.lib). Inputs: the
// incoming audio, left and right (only listened to, for Follow). Outputs: the
// calls, left and right, before their echo (the plugin's) and reverb
// (plugin-reverb.dsp). The plugin sets every control here; the labels are what
// it looks them up by. Each caller's note and gate are the plugin's too: Root,
// or a held MIDI note each.

import("stdfaust.lib");
cl = library("calls.lib");

maxCallers = 8;

// Control smoothing that starts at the control's value (after a reset too), so
// nothing glides in from the DSP's own defaults, and lands on it exactly once
// it's within a millionth (a one-pole only ever nears it).
smoo = _ <: si.smooth(0.999 * (1 - first)), _ : \(y, x).(select2(abs(y - x) < 1e-6, y, x))
with { first = 1 - 1'; };

// Set by the plugin once, before the first sample: this instance's own streams.
seed     = hslider("seed", 2000, 0, 1000000, 1);
activity = hslider("activity", 0.45, 0, 1, 0.01) : smoo;
rate     = hslider("rate", 1, 0.25, 3, 0.01) : smoo;
level    = hslider("level", -24, -60, 0, 0.1) : ba.db2linear : smoo;
follow   = hslider("follow", 1, 0, 1, 0.01) : smoo;
width    = hslider("width", 1, 0, 1, 0.01) : smoo;

// A caller fades in when its note starts (30 ms) and out after it ends (to
// -60 dB in about 1.4 s; Abdication's faded with its voice's amp envelope).
// It isn't computed once it has faded.
caller(k) = (cl.calls(seed + 811 * k, note, activity, rate) : *(env), _ : panned) : par(i, 2, cl.only(on))
with {
    note = hslider("note%k", 130.81, 20, 8000, 0.01);
    gate = hslider("gate%k", 0, 0, 1, 1);
    env  = gate : an.amp_follower_ar(0.01, 0.2);
    on   = (gate > 0) | (env > 1e-5);
    // Width 1 spreads the notes across the field, 0 puts them all in the middle.
    panned(v, pan) = v * cos(a), v * sin(a) with { a = (0.5 + width * (pan - 0.5)) * ma.PI / 2; };
};

// Follow: 0 the calls run at their Level whatever comes in; 1 they're at their
// Level while the input is above -20 dBFS, fading out as it falls to -60.
following(l, r) = 1 - follow * (1 - amount)
with {
    env    = max(abs(l), abs(r)) : an.amp_follower_ar(0.005, 0.25);
    amount = cl.clip(0, 1, (ba.linear2db(max(env, 1e-6)) + 60) / 40);
};

process(l, r) = par(k, maxCallers, caller(k)) :> *(g), *(g)
with { g = level * following(l, r); };
