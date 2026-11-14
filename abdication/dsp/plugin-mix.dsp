// plugin-mix.dsp — one voice up to its filter, for the Rust plugin: the
// mixer, noise, transient and drive. Inputs: its summed unison copies of
// Osc 1 and Osc 2 (plugin-osc.dsp). Outputs: the voice before its filter,
// the transient when it's post-filter, and the pitch, all for
// plugin-voice.dsp (the first through plugin-fdsp.dsp while that's on).

import("stdfaust.lib");
v = library("voice.lib");

// Driven by the plugin, as in plugin-voice.dsp.
freq  = hslider("freq", 110, 20, 8000, 0.01);
gain  = hslider("gain", 0.8, 0, 1, 0.01);
gate  = button("gate");
seed  = hslider("seed", 0, 0, 1, 0.000001);
glide = hslider("glide", 0, 0, 1, 0.0001);

process(osc1, osc2) = v.mixer(freq, gain, gate, seed, glide, osc1, osc2);
