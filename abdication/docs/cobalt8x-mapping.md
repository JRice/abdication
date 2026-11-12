# Cobalt8X → Abdication controller map (strawman)

The map below is built into a Bitwig controller script,
`bitwig/Cobalt8X-Abdication.control.js`: the Cobalt's knobs drive the
Abdication on the selected track by parameter name, with no MIDI learn.
CC numbers are from Modal's Cobalt8 manual. To change the map, edit the
script's `MAP` (and this table); Bitwig reloads scripts when they change.

## Setup

1. **Cobalt8X:** Settings → set **Local Off** (the Cobalt stops playing its
   own engine) and note its **MIDI channel**. Keep Mod Wheel Out and Pitch
   Bend Out on: the joystick then drives Abdication's Perform panel.
2. **Script:** copy `Cobalt8X-Abdication.control.js` into
   `Documents\Bitwig Studio\Controller Scripts\` (any subfolder).
3. **Bitwig:** Settings → Controllers → Add controller → Abdicant →
   **Cobalt8X for Abdication**, and pick the Cobalt as its MIDI input. If
   the Cobalt was also added as a generic keyboard, remove that, or notes
   arrive twice.
4. Select a track with Abdication on it (record-armed, or with Bitwig's
   input following selection) and play. The knobs control the first
   Abdication in that track's chain; on other tracks they do nothing.

The Cobalt's knobs are absolute: turning one jumps its parameter to the
knob's position. A knob the map doesn't use pops up its CC number in
Bitwig (once), so it can be added to the map. Drift sends an NRPN rather
than a CC; any NRPN the script's `NRPN_MAP` doesn't list goes to Vintage,
and its number pops up once.

## Map

| Cobalt control | CC | Abdication parameter |
|---|---|---|
| **Oscillator 1 → Osc 1's Dual** | | |
| Osc1 Algorithm | 62 | Dual 1: Algorithm |
| Osc1 Param A | 16 | Dual 1: A |
| Osc1 Param B | 19 | Dual 1: B |
| Osc1 Tune | 14 | Dual 1: Semi |
| Osc1 Fine | 15 | Unison: Detune |
| **Oscillator 2 → Osc 2's Wavetable** | | |
| Osc2 Algorithm | 63 | WT 2: OscMod |
| Osc2 Param A | 17 | WT 2: Position |
| Osc2 Param B | 20 | WT 2: OscMod Depth |
| Osc2 Tune | 30 | Osc 2: Octave |
| Osc2 Fine | 31 | WT 2: Position LFO Rate |
| Osc Mix | 18 | Mix: Osc 2 |
| **Filter** | | |
| Cutoff | 34 | Filter: Cutoff |
| Resonance | 35 | Filter: Resonance |
| Morph | 33 | Filter: Morph LP-Notch-HP |
| **Filter EG → the Cutoff's envelope** (its modulator) | | |
| FEG Amount | 32 | Filter: Cutoff Env Amount |
| FEG Attack / Decay / Sustain / Release | 22 / 23 / 24 / 25 | Filter: Cutoff Env Attack / Decay / Sustain / Release |
| **Amp EG → Amp** | | |
| AEG Attack / Decay / Sustain | 26 / 27 / 28 | Amp: Attack / Decay / Sustain |
| **LFO1 → the Cutoff's LFO** | | |
| LFO1 Rate / Shape | 36 / 39 | Filter: Cutoff LFO Rate (the knob reversed: it ran backwards) / Filter: Cutoff LFO Shape |
| LFO1 Depth | 37 | Filter: Cutoff LFO Amount |
| **Mod envelope → Dual 1 A's envelope** | | |
| MEG Attack / Decay / Sustain / Release | 43 / 44 / 45 / 46 | Dual 1: A Env Attack / Decay / Sustain / Release |
| MEG Depth | 49 | Dual 1: A Env Amount |
| **LFO2 and LFO3 → noise and transient** | | |
| LFO2 Rate / Depth / Shape | 47 / 48 / 50 | Mix: Noise / Noise: Tilt / Noise: Clock Ratio |
| LFO3 Rate / Depth / Shape | 3 / 13 / 21 | Noise: Bits / Transient: Level / (free) |
| **Other** | | |
| FX Amount | 73 | Drive: Amount |
| Patch Gain | 119 | Master: Volume |
| Drift | NRPN (CC 99/98, value on 6/38) | Vintage: Amount (14-bit) |

The map assumes the default slots, a Dual in Osc 1 and a Wavetable in Osc 2.
A slot switched to its other engine keeps the hidden engine's settings, so
its knobs still move them, unheard; Osc 2's Dual and Osc 1's Wavetable can
go in the map by name ("Dual 2: A", "WT 1: Position") to suit.

Not mapped (set them on the panel): the slots' modes, the Dual waves and
the wavetable waves, noise and transient types, filter switches, Perform,
the Amp's release and curve, and every other knob's modulators (any can go
in the script's map by its name, e.g. "Filter: Resonance LFO Amount" or
"Mix: Osc 1 Velocity").

## Notes

- The Cobalt sends 7-bit CCs (128 steps). Abdication smooths its controls,
  so sweeps won't zipper, but the finest settings may be a step apart.
- Menu-type parameters (Algorithm, OscMod, Shape) step through their
  options as the knob turns. The Dual's twelve algorithms are in the
  Cobalt8's own order (VA Sweep, Spread, PWM, Sync, Fractal, Window Amp
  Sync, Metal, Ring Mod, FM, Noise AM, Chaos, Folding), so the knob walks
  the same families in the same direction; the waves are picked
  separately (Dual 1: Wave 1 and Wave 2).
- The Cobalt's screen shows its own values, not Abdication's. Watch
  Bitwig's parameter view (or, later, the GUI).
- Some of these may need Shift or a menu page on the Cobalt. If a knob
  doesn't respond during learn, check the Cobalt's MIDI out filter (CCs
  allowed) and that the control sends a CC at all.
