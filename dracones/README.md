# Hic Sunt Dracones

**HC SVNT DRACONES**: a free-running drone synth by Abdicant, a CLAP/VST3 instrument
made of pieces of its siblings: Argent's Fizmo tables, Abdication's image waves, Pitch law,
filter and Harmonic EQ, Antimeme's Interference and Agitation, and Apse's plate. No MIDI,
no notes: it sounds whenever it's loaded, its pitches set only by the knobs. Not for sale,
not for support.

The design (and its open questions) is in the claude.ai project as
`claude/dracones-design.md`; the panel mockup is the "Hic Sunt Dracones panel" design
canvas.

## Signal flow

```text
osc I / II / III → lowpass (Wasp | SVF; Cutoff, Res) → × Gain → insert delay (one each) → sum
sum → Fuzz (Gain, HP) → Spectral Morph (Shape, Slant) → Apse Plate (Time) → × Output → limiter (-0.5 dBFS)
```

- **I, Fizmo** (Argent's tables at its default seed 417: 16 drawn, 18 recorded). The
  seal steps the table (Shift-click steps back); Position morphs within it.
- **The seals** are small scopes: each shows its oscillator's last cycle after the
  filter (each point the mean of 1/64 of the cycle), brighter the more Gain it has.
  The numeral sits under it.
- **II, Argon-like**: Abdication's A8 image waves, in fives (A8 01–30 → six tables; 31–32
  left over), band-limited per octave. The seal steps the table, Position crossfades
  across its five.
- **III, wavefolder**: a sine into a sine fold, Fold driving it 1× to 32× (along Fold
  squared, so the first version's whole range, to 8×, is the lower half), run at 4×.
- **Pitch**: Abdication's law (cents, then 1–7 semitones, then 1–2 octaves), centred on C2.
- **Filter**: per oscillator, the lever picks the Wasp (CMOS-inverter clipping, resonance
  to oscillation) or Abdication's SVF (lopsided tanh in the loop, eased Q at low cutoffs);
  both lowpass.
- **Gain** sits before the inserts, so closing an oscillator lets its echoes ring out.
- **Inserts**: each row feeds exactly one delay; drag a plaque onto another row and they
  swap (the tank travels with its plaque and keeps ringing; inputs crossfade over 50 ms).
  Division (1/8., 1/4, 1 bar, 1 bar., 2, 3, 4 bars) never shares a value: dragging skips
  the others' values (drawn as rocks), and if automation lands two on one, the lower row's
  moves to the next free longer one (or, past 4 bars, shorter). Division and tempo changes
  crossfade two read heads over 120 ms. Tanks hold 30 s. In each loop: read → effect →
  40 Hz HP and 9 kHz LP → × Feedback (to 0.98) → write. Each delay's **Mix** (the small
  slider under its Division, travelling with its plaque) runs from off to all wet: the
  wet rises to full at the middle with the dry at unity, then the dry fades out. The
  default, 35 %, is the first version's balance (dry at unity, wet at -3 dB).
  - Chorus (Juno mode I in the tank; turns it stereo): Custom = how much chorus
    ("Chorus Amount" to the host, apart from the delay's Mix).
  - Tube: Custom = Drive, 0 to +24 dB in and the loop's gain toward 1.03, so at the top
    it sustains, held near -6 dBFS by the tube.
  - Phaser (Small Stone, four OTA stages): Custom = Speed, 0.05–8 Hz.
- **Fuzz**: Gain 0 is clean, fading in over its first 10 %; HP (20 Hz–1.2 kHz) before the
  clipping decides what's fuzzed.
- **Spectral Morph**: Abdication's Harmonic EQ with Thorn's 81 shapes (the cartouche:
  click its left half back, right half on; Off is empty sea), Slant = Tilt ±6 dB/oct,
  **Depth** the slider under the cartouche (off to 100 %; it scales the Slant too, so at
  off the Morph is off). It tracks oscillator I's pitch.
- **Reverb**: Apse's Plate, Time to 30 s (T40), wet fixed at 30 %; the bottom 8 % of Time
  fades it out.
- **Output**: the slider on the right, unity at 80 % (the long tick), +6 dB at the top.

## Modulators and arms

Four global modulators, their traces in the corners: **silver** (Interference: the
inserts' summed output through a 30 Hz pole, ×3, tanh: the drone moving itself),
**red** (Agitation: Time 0.02–8 Hz and Slope, the small knobs beside it; unipolar;
three copies, I to III, in columns),
**green** (a slow random walk, new target every 6–16 s; its trace is the last minute),
**slate** (sample and hold of white noise on the host's grid: three copies, a lever
each, the pips beside them 1, 4 or 16 quarter notes; it keeps time while the transport
is stopped).

A knob's red and slate arms follow its row's copy: oscillator row II's knobs copy II, an
insert effect the copy of the row it's docked in, the bottom row copy I. The corner
displays draw all three copies, the last-touched row's bright, the others faded.

Every modulatable knob (25) has four triangular arms, one per modulator, pointing in
directions seeded from the knob's id (so they never move). Drag an arm outward along its
own axis to deepen it (it grows to three times its length at full), double-click to zero.
Depth 1 moves the knob up to half its range; red only ever pushes up; pitch arms follow
a cubic law, so small depths stay within cents. A bead on the knob's rim shows where its
modulators have it. The host sees the 100 depths as "II Cutoff ← Green" and so on.

## Install

From this folder, in PowerShell (needs Rust and Visual Studio's C++ build tools):

```powershell
powershell -ExecutionPolicy Bypass -File plugin\install.ps1
```

Rescan in Bitwig. It reads Argent's `assets/recorded.bin` and font and Abdication's
`harmlib.bin` and `wavelib.bin` from their folders when it builds, so build it in place
in this repo.

## Layout

```
plugin/src/engine.rs     the instrument: oscillators, filters, routing, post chain
plugin/src/knobs.rs      the 25 armed knobs, how modulation moves them, arm angles
plugin/src/dsp/          argon, fold, filters, mods, inserts, fuzz (new);
                         tables, osc, random (Argent's); pitch, harmonic, harmlib
                         (Abdication's); apse/ (Apse's reverb, its Plate used)
plugin/src/params.rs     host parameters
plugin/src/editor/       the panel: world.rs (the drifting map, ships, beasts),
                         svg.rs (the mockup's path data), paint.rs, panel.rs
```

`cargo test --release` runs the checks; `cargo test --release cpu -- --ignored
--nocapture` times it (about 5 % of a core here).
