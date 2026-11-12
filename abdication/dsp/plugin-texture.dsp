// plugin-texture.dsp — one voice's texture, for the Rust plugin (see
// texture.lib): spectral events on the voice's output, and its calls.
// Inputs: the voice's output, its pitch in Hz and its amp envelope (the last two are outputs of plugin-voice.dsp). Outputs: the
// voice with its events, then its calls, left and right, for the plugin's
// calls bus (plugin-callbus.dsp). The plugin skips this DSP when both are off.

import("stdfaust.lib");
v = library("voice.lib");

gate = button("gate");

process(in, hz, amp) = v.texture(gate, in, hz, amp);
