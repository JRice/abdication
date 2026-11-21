// plugin-noise.dsp — the Noise layer (drum.lib's noiseLayer) as its own DSP,
// run only while it sounds. The voice works out all of its inputs (audio):
//   generator, Tone's pitch, bandpass Hz, loop, loop position, blend, crush,
//   density, Q, highpass Hz, Bite, the envelope (with velocity), the part
//   level, Metal's Spread and whether it's set, level (the envelope times the part level: the
//   plain path's gain).

declare name "ABDrumNoise";
import("stdfaust.lib");
dr = library("drum.lib");

process = dr.noiseLayer;
