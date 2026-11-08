// plugin-callbus.dsp — the calls' own effects, for the Rust plugin: one for
// the whole instrument, taking every voice's calls. Their echo goes into a
// reverb, with no dry path: the calls are heard only through it. The reverb is
// heard by nothing else, so the rest of the patch stays dry. The plugin stops
// computing this DSP once the calls are off and the tail has died away.

import("stdfaust.lib");
te = library("texture.lib");

// The outer box stands in for the one Faust names after the file when a program
// has several top-level groups: the plugin skips the outermost box of a DSP's UI.
panel(x) = vgroup("callbus", hgroup("[12]Calls", x));
echoAmount = panel(hslider("[4]Echo", 0.5, 0, 1, 0.01)) : si.smoo;

// Mute closes both ends of the bus, so the tail goes too: the plugin clears
// what's left once it has faded, so nothing old comes back on unmuting.
open = panel(checkbox("[0]Mute")) : \(m).(1 - m) : si.smoo;
gated = *(open), *(open);

// The echoes differ in length, for width.
echoes = _, _ <: (_, _), (te.echo(0.36, echoAmount), te.echo(0.48, echoAmount)) :> _, _;

process = gated : echoes : re.zita_rev1_stereo(30, 300, 2500, 4.0, 5.0, 96000) : gated;
