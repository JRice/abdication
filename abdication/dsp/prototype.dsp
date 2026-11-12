// prototype.dsp — playable 8-voice prototype of the shared core.
// Play it from the Faust web IDE (Poly mode) or compile it to a plugin.

declare name "abdication-prototype";
declare options "[midi:on][nvoices:8]";

import("stdfaust.lib");
v = library("voice.lib");

// The host's MIDI note handling drives these three by name; hidden
// where the host supports it.
freq = hslider("freq [hidden:1]", 110, 20, 8000, 0.01);
gain = hslider("gain [hidden:1]", 0.8, 0, 1, 0.01);
gate = button("gate");                  // same label as voice.lib's noteOn
seed = hslider("seed [hidden:1]", 0, 0, 1, 0.000001);   // random per note (plugin engine)

// The centre unison copy only (the plugin runs seven with Unison on). The
// knobs' modulators are the plugin's, so here every knob stays put.
copy = v.unisonCopy(freq, gate, seed, 0, 3) : !, !, _, _;
process = copy : v.voice(freq, gain, gate, seed, 0) : _, si.block(v.panelTaps) <: _, _;
