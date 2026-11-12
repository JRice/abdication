// render-test.dsp — headless smoke test: plays a repeating note through
// the prototype voice with default settings so it can be rendered offline.

import("stdfaust.lib");
v = library("voice.lib");

gate = ba.pulsen(int(0.6 * ma.SR), int(1.0 * ma.SR));   // 0.6 s on, 0.4 s off
// The centre unison copy only; no modulators (they're the plugin's).
copy = v.unisonCopy(55, gate, 0, 0, 3) : !, !, _, _;
process = copy : v.voice(55, 0.8, gate, 0, 0) : _, si.block(v.panelTaps);
