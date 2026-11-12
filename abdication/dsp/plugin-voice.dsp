// plugin-voice.dsp — one monophonic voice for the Rust plugin. The plugin
// runs eight copies and drives freq, gain and gate from MIDI itself, so
// this file has no polyphony declarations. Inputs: its summed unison
// copies of Osc 1 and Osc 2 (plugin-osc.dsp).

import("stdfaust.lib");
v = library("voice.lib");

freq = hslider("freq", 110, 20, 8000, 0.01);
gain = hslider("gain", 0.8, 0, 1, 0.01);
gate = button("gate");
seed = hslider("seed", 0, 0, 1, 0.000001);   // random per note (plugin engine)
// Portamento time in seconds for this note, set by the engine (0 when it
// doesn't glide); the Portamento knob's sends can move it.
glide = hslider("glide", 0, 0, 1, 0.0001);

// Outputs: the voice, then the panel's moving values (see voice.lib).
process(osc1, osc2) = v.voice(freq, gain, gate, seed, glide, osc1, osc2);
