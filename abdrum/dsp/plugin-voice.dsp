// plugin-voice.dsp — one drum voice for the Rust plugin. The plugin drives
// freq (note pitch), gain (velocity), gate and seed (random per hit), and
// sums the voice's five outputs (one per part, for the panel's scopes).

declare name "ABDrum";
import("stdfaust.lib");
v = library("voice.lib");

freq = hslider("freq", 261.6256, 20, 8000, 0.01);
gain = hslider("gain", 0.8, 0, 1, 0.01);
gate = button("gate");
seed = hslider("seed", 0, 0, 1, 0.000001);

process = v.voice(freq, gain, gate, seed);
