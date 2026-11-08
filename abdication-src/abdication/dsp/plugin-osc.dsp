// plugin-osc.dsp — one unison copy of a voice's Dual and Wavetable, for
// the Rust plugin, which runs seven per voice with Unison on and only the
// centre one with it off. Inputs: the voice's free modulators (Mod Env,
// Mod LFO; plugin-mod.dsp). Outputs: Dual, Wavetable at the copy's unison
// level (summed into the voice, plugin-voice.dsp), then as they are (the
// centre copy's feed the panel's scopes).

import("stdfaust.lib");
v = library("voice.lib");

// Driven by the plugin, as in plugin-voice.dsp.
freq  = hslider("freq", 110, 20, 8000, 0.01);
gate  = button("gate");
seed  = hslider("seed", 0, 0, 1, 0.000001);
glide = hslider("glide", 0, 0, 1, 0.0001);
copy  = hslider("copy", 3, 0, 6, 1);         // which copy: 0..6, 3 the centre

process(modEnv, modLfo) = v.unisonCopy(freq, gate, seed, glide, modEnv, modLfo, copy);
