// plugin-mod.dsp — the free modulators of one voice, for the Rust plugin
// (see mod.lib). Outputs: Mod Env (0..1), Mod LFO (-1..1).

m = library("mod.lib");

process = m.env, m.lfo;
