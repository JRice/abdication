# ABDrum

A personal drum synth (by Abdicant), sibling to Abdication. One drum voice
per plugin instance, hosted in Bitwig's Drum Machine. No sequencer. The
design decisions are in the Abdication Synth project (abdrum-design.md).

## Status

Tone, Body, Noise and two sample layers, with Variation, the effects chain
(Crush -> Drive -> Ring mod -> Comb -> Filter -> Compressor), and its own panel, as a
CLAP/VST3 plugin.

## The panel

Five cards across the top, one per part, each with a scope of the latest
hit (frozen once it has rung out; after you change a setting it shows a hit
rendered with the new settings, until the next one you play). The front of
a card has the essential controls; ⟲ turns it over for the rest (velocity
amounts, envelope shape, the noise extras); the square before its name
switches the part off and back on (to the level it had, else -6 dB).
Below: the screen, which draws whatever you last touched (envelopes, the
pitch drop, the noise filter, the nearest note, lists), the free envelope,
and the effects with Variation. Top right: Volume, with the output meter
in its bar.

Knobs: drag up and down (Shift for fine), scroll, double-click to reset.
Pitch knobs (Tone Pitch, Body Tune, the sample and loop pitches) land on
notes (A440) or whole semitones; Shift-drag goes between them. Level bars:
click or drag along them; their travel is spread like a fader's, so the
quiet end has room. Chips (Generator, Drive type, Comp timing) step on a
click. Sample and loop pickers: the arrows step, the name opens the whole
list. Controls that do nothing with the current settings are dimmed (still
adjustable), and so is a part that's off.

Rings: every knob on the cards and in the effects has an envelope ring,
the band just outside its arc (it lights up under the mouse). Drag the ring
up or down to send the free envelope to that knob: at the envelope's peak
the knob moves that far (up to its whole travel), either way. Double-click
a ring to clear it. The envelope runs per hit, at audio rate, so short
envelopes stay sharp; the effects' rings follow the latest hit.

The die (top bar) rolls a random sound: first a kind of drum (kick,
snare, hat, tom or percussion, clap, metal, microsound), then each of its
settings from that kind's usual range, with the odd one (about one in 30)
anywhere at all; then maybe an effect, a ring or two, a sample layer.
Volume is never touched. Right-click the die to go back to the sound
before the last roll.

The export button renders one hit (note 60, full velocity, through the
effects and Volume, until it has rung out) to a 24-bit WAV in
Music\ABDrum. Drag from the button and drop it in the DAW; just click it to
save the file. Every take re-rolls Variation, so dragging again gives a
slightly different hit, like playing the pad again. (Dragging is Windows
only; if that part ever fails to build, install.ps1 builds without it and
the button saves instead.)

## Install

From this folder, in PowerShell (needs Rust and Visual Studio's C++ build
tools, as Abdication does; no Faust or Python):

```powershell
powershell -ExecutionPolicy Bypass -File plugin\install.ps1
```

It installs to your user CLAP and VST3 folders; rescan in Bitwig. Close any
project using ABDrum before reinstalling.

## How it plays

- Put one ABDrum on each Drum Machine pad. Note-offs are ignored: a hit
  rings as long as its envelopes say. Up to five hits overlap per pad
  (a new hit never cuts a ringing one off with a click); the fifth takes
  the oldest.
- Choke groups work: a choke fades the pad's ringing hits in about 5 ms.
- The note only matters with Tone's Keytrack on: MIDI note 60 (C3 in
  Bitwig) plays at the Pitch knob.
- The effects run once per pad, on the sum of its hits: the compressor
  ducks the hit still ringing, and Drive glues overlapping hits together.
  A choke fades the effects' tails too.
- Master: Volume, then a safety limiter at -0.5 dBFS.
- Note names follow Bitwig's (note 60 is C3; A 440 Hz is A3).

## Layout

```
dsp/
  core.lib          Abdication's shared building blocks (a copy; keep in sync)
  drum.lib          the voice's DSP: envelopes, Tone, Body, Noise, hits
  voice.lib         the voice: layers + controls, one group per panel
  plugin-voice.dsp  entry point (freq, gain, gate, seed come from the plugin;
                    five outputs, one per part, which the plugin sums)
  fx.lib            the effects' DSP: Crush, Drive, Ring mod, Comb, Filter, Compressor
  mod.lib           the free envelope and its sends (the rings), for voice and effects
  plugin-fx.dsp     the effects' controls and entry point (tune, hit from the plugin)
  plugin-body.dsp   the Body as its own DSP: the plugin runs it per voice only while it
                    sounds, with its settings every 16 samples (its 24 modes' coefficients
                    were 90 % of a voice's CPU when computed every sample)
  plugin-noise.dsp  the Noise as its own DSP, likewise run only while it sounds
  samplelib.lib     GENERATED: Loop menu, hit sets, readers, per-hit send gains
plugin/             Rust CLAP/VST3 plugin (nih-plug), Abdication's wrapper pattern
  src/dsp.rs, fx.rs generated from the Faust code (don't edit)
  src/engine.rs     overlapping hits (5 voices), the effects on their sum,
                    choke, output limiter; records the latest hit for the scopes
  src/scope.rs      the latest hit's five parts, shared with the panel
  src/export.rs     one hit to a WAV file (the export button)
  src/editor/       the panel (vizia, drawn by hand as in Abdication):
                    layout, draw, panel (mouse), screens, scopes, values,
                    randomize (the die), drag (dragging a WAV out, Windows)
  src/bin/          a standalone app, for trying the panel without a host
  src/params.rs     host parameters built from the Faust controls
  src/samplelib.rs  serves the embedded loops and hits (samplelib.bin, generated)
  install.ps1       Windows: build + install for the current user
tools/
  gen-rust.sh       regenerates plugin/src/dsp.rs, fx.rs, body.rs, noise.rs (needs native Faust)
  gen-samplelib.py  "abdrum library" noise + transient folders -> plugin/src/samplelib.*,
                    dsp/samplelib.lib (run after changing the library; then
                    gen-rust.sh and rebuild)
  compare-backends.py  checks the Rust DSP against the C++ renderer, sample by sample
  render.py         offline renders, for checks (needs Faust + g++; Linux)
  presets.py        test presets (compare-backends.py plays each one)
  render/render.cpp the offline renderer's driver
```

## Controls

- **Voice:** Variation (pitch ±10 cents, decays ±10 %, strike position,
  Shape, Hardness, level ±1.5 dB, noise bandpass ±5 %, all per hit).
Some front knobs are macros: the front sets the size, the back the shape.

- **Tone:** Level, Pitch (20 Hz–2 kHz), Keytrack (note 60 plays at Pitch), Shape
  (sine -> triangle -> saturation -> fold), Drop (the whole pitch drop, to
  +72 st) with Split (how much of it is the fast drop), Fast Time (1–20 ms)
  and Slow Time (20–500 ms), Decay (the slow decay: the hit's length) with
  Snap (how much quicker the fast decay is: 0 the same, 1 a hundredth;
  the panel shows the fast decay it gives) and Tail Level, Attack,
  Vel Level / Pitch / Shape. Decays are times to -60 dB; drop times are
  to 5 % of the drop.
- **Body:** Level, Offset (the panel's Tune: ±24 st from Tone's resting
  pitch), Material (membrane -> plate -> bar, all inharmonic), Strike
  (mallet hardness, and brightness with it), Decay, Damping (above the
  middle the upper modes die sooner and start quieter; below, they ring
  brighter), Vel Brightness / Bend / Decay. Strike position is hidden
  (Variation).
- **Noise:** Level, Generator (white, pink, dust, sample & hold, metal: six
  squares at the 808's cymbal ratios, tuned by Tone's Pitch), Loop,
  Blend (generator <-> loop), Bandpass (the panel's Freq), Q (0 = flat,
  only the highpass acts; up to 24 dB/oct skirts, never ringing; level held
  steady), Highpass, Env Amount (the panel's Sweep: bandpass up to +4 oct
  with the envelope), Attack, Decay and Snap (as Tone's), Tail Level, Clap,
  Density, Crush, Vel Level,
  Loop Pitch (±24 st, the loop only). Loops start at a random point on
  every hit.
- **Sample 1, Sample 2:** Level, Set (thump, knock, spark), Sample (wraps
  within the set), Pitch (±24 st), Decay (fades the hit; the top plays it
  whole), Body Send, Vel Level. Body Send strikes the Body with the hit:
  each hit's energy is normalized, so a full send strikes about as hard as
  the mallet, and as the sends rise the mallet fades out. Its spectrum still
  decides what rings: sparks ring high bodies, thumps low ones.
- **Drive:** Type and Amount. Each type has its own curve, drive range and
  emphasis, so they stay distinct when pushed: Tape (soft, at most +24 dB so
  it never squares off; lows driven harder; darkens as it's pushed), Tube
  (much softer positive half: strong even harmonics; up to +30 dB), Clip
  (hard clip; highs driven harder; up to +40 dB, bright and punchy).
  Amount 0 is bypass; its first third blends the drive under the clean hit
  (density with the clicks intact), then it's all drive. Makeup holds a
  hit's peak; velocity drives it harder, as on analog machines. Everything
  goes through it. Antialiased (first-order ADAA); full Amount on high
  pitched material can still alias a little.
- **Crush:** Amount. The first 60 % lowers the sample rate (44.1 kHz to
  2 kHz, no anti-alias filter: MachineDrum SRR); from 40 % the bit depth
  falls from 16 to 4. Free-running, the same on every pad. Before Drive.
- **Ring Mod:** Amount. Deepens the ring mod while its carrier climbs from
  1.07x to 3.7x Tone's pitch (shimmer to clang); restarts on every hit.
- **Comb:** Amount (feedback and mix together). Tuned to Tone's pitch,
  inharmonic: allpasses in the loop stretch the overtones to about 1, 2.08,
  3.29, 4.63, 6.07 x (a stiff wire or bell, not a chord), at any pitch.
- **Filter** (its own panel, between the Envelope and the effects): Cutoff
  and Resonance, after the SH-101's VCF, on the pad's sum after Comb.
  Four OTA lowpass stages (24 dB/oct, -12 dB at the cutoff with no
  resonance), each saturating on its input-output difference; resonance
  fed back, inverted, through a soft diode-like limiter, and half of it
  added back at the output as on the 101 (so full resonance costs 4.4 dB
  of bass, not 14). Zero-delay feedback, so it tracks to within a few
  cents up to the top. Resonance stops at the edge of oscillation: the top
  rings for about 2 s at any cutoff and dies away. Cutoff at the top
  (20 kHz) is off, an exact bypass that costs nothing; from 16 kHz up it
  fades into the dry signal. Its envelope sweep is the Cutoff's ring. The
  screen draws its response. Effects controls snap to their settings on
  every hit, so a choke's reset doesn't make the next hit glide in.
- **Envelope** (the free one, for the rings): Attack, Decay, Curve (0 falls
  in a straight line, 1 snaps down and tails off), Velocity (how much the
  hit's velocity scales it). Each ring is a host parameter too ("Tone:
  Pitch Mod", -1..1 of the knob's travel), so it can be automated.
- **Compressor:** Threshold (0 dB = off) and Timing (Squash 0.3/50, Tight
  1/80, Punch 3/120, Snap 10/200, Open 30/400 ms attack/release). 4:1, hard
  knee; makeup restores half of a full-scale hit's gain reduction.
