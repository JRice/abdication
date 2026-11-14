// plugin-fdsp.dsp — one voice's FDSP (see fdsp.lib), for the Rust plugin.
// Inputs: the voice before its filter and its pitch (outputs of
// plugin-mix.dsp). Output: the voice with its FDSP, for plugin-voice.dsp.
// The plugin skips this DSP while FDSP is off (Type Off, or its Mix at 0).

import("stdfaust.lib");
v = library("voice.lib");

gate = button("gate");

process(in, hz) = v.fdsp(gate, in, hz);
