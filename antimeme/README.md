# Antimeme

A noisy delay after the Time/Filter experiment in the Make Noise Strega, by Abdicant:
a CLAP/VST3 effect. Hand-written Rust on nih-plug, unlike its Faust-based siblings:
the delay chip is a memory with a varying number of clock steps per sample.

## Status

Calibrated against one capture of a real Strega; `docs/measurements.md` says what was
measured, how close the model comes, and what's still off. The defaults are the delay
settings from the favourite patch, fave1.

## The panel

"Slices, in Tibetan". The host still sees the parameters' English names.

Sliders: the middle moves the value (a press there jumps to it), the top edge the Agitation
send (red), the bottom edge the Interference send (white). A send's bar runs from the value
to where the modulation reaches, and the readout shows its amount while you're on it. Shift
for fine, double-click to reset, scroll to nudge. Time has its own slider under the
numerals, and the numerals themselves drag up and down.

Terms, top to bottom, first column then second: Decay, Filter, Absorb, Whine, Strength;
Blend, Spread, Level, Speed, Angle. Left: Agitation, Interference. དུས་ཚོད is Time.

The glitches answer the output limiter (ceiling -0.5 dBFS; push Level to meet it), and get
worse with depth up to 12 dB: numerals slip and ghost, slices of the panel tear sideways,
labels turn to symbols or swap places, and strips invert. Inverted strips are limited to 15 %
of the window and 2.5 a second, so the panel never flashes as a whole. At rest, the
numerals still slip now and then, with a faint twitch every half minute or so.

## Install

From this folder, in PowerShell (needs Rust and Visual Studio's C++ build tools, as
ABDrum does):

```powershell
powershell -ExecutionPolicy Bypass -File plugin\install.ps1
```

Rescan in Bitwig. Close any project using Antimeme before reinstalling.

## Layout

```
capture/              the test signal and knob schedule for measuring a Strega
docs/measurements.md  what the Strega measured
tools/                the analysis (Python, numpy, scipy); compare.py is the one to rerun
plugin/src/dsp/       the sound, free of any plugin API
  chip.rs             the delay chip: modulator, integrators, RAM, clock, burble
  voice.rs            one channel: three chips, cross-feedback, Filter and Absorb
  agitation.rs        the looping rise and fall
  limiter.rs          the output's safety limiter
  engine.rs           Strength, the voices, Blend, Level, limiter, resampling
plugin/src/params.rs  host parameters
plugin/src/telemetry.rs  what the audio thread tells the panel
plugin/src/editor/    the panel: layout, painter, glitch logic, events
plugin/assets/fonts/  Noto Serif Tibetan subsets; tools/build_fonts.py makes them
plugin/examples/      headless numeric checks: cargo run --release --example probe_chip
```
