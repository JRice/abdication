// plugin-osc.dsp — one unison copy of a voice's two oscillators, for
// the Rust plugin, which runs seven per voice with Unison on and only the
// centre one with it off. No inputs. Outputs: Osc 1, Osc 2 at the copy's
// unison level (summed into the voice, plugin-voice.dsp), then as they are
// (the centre copy's feed the panel's scopes); then, for a slot on Ray (which
// the plugin plays), each slot's pitch, the copy's unison level and start
// phase, and note-on.

import("stdfaust.lib");
v = library("voice.lib");

// Driven by the plugin, as in plugin-voice.dsp.
freq  = hslider("freq", 110, 20, 8000, 0.01);
gate  = button("gate");
seed  = hslider("seed", 0, 0, 1, 0.000001);
glide = hslider("glide", 0, 0, 1, 0.0001);
copy  = hslider("copy", 3, 0, 6, 1);         // which copy: 0..6, 3 the centre

process = v.unisonCopy(freq, gate, seed, glide, copy);
