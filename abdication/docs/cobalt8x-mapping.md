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
| **Oscillator 1 → Dual** | | |
| Osc1 Algorithm | 62 | Dual: Algorithm |
| Osc1 Param A | 16 | Dual: A |
| Osc1 Param B | 19 | Dual: B |
| Osc1 Tune | 14 | Dual: Semi |
| Osc1 Fine | 15 | Unison: Detune |
| **Oscillator 2 → Wavetable** | | |
| Osc2 Algorithm | 63 | Wavetable: OscMod |
| Osc2 Param A | 17 | Wavetable: LFO Rate |
| Osc2 Param B | 20 | Wavetable: OscMod Depth |
| Osc2 Tune | 30 | Wavetable: Octave |
| Osc2 Fine | 31 | Mix: Fundamental |
| Osc Mix | 18 | Mix: Wavetable |
| **Filter** | | |
| Cutoff | 34 | Filter: Cutoff |
| Resonance | 35 | Filter: Resonance |
| Morph | 33 | Filter: Morph LP-Notch-HP |
| FEG Amount | 32 | Filter: Env Amount |
| **Envelope** (filter EG = the shared envelope) | | |
| FEG Attack / Decay / Sustain / Release | 22 / 23 / 24 / 25 | Envelope: Attack / Decay / Sustain / Release |
| AEG Decay | 27 | Envelope: Amp Decay + (the pad, left/right) |
| AEG Sustain | 28 | Envelope: Amp Sustain + (the pad, up/down) |
| AEG Attack | 26 | Envelope: Curve |
| **LFO1 → Mod LFO** | | |
| LFO1 Rate / Shape | 36 / 39 | Mod LFO: Rate (the knob reversed: it ran backwards) / Shape |
| LFO1 Depth | 37 | Filter: Cutoff LFO (the LFO's send to the cutoff) |
| **Mod envelope → Mod Env** | | |
| MEG Attack / Decay / Sustain / Release | 43 / 44 / 45 / 46 | Mod Env: Attack / Decay / Sustain / Release |
| MEG Depth | 49 | Mod Env: Curve |
| **LFO2 and LFO3 → noise and transient** | | |
| LFO2 Rate / Depth / Shape | 47 / 48 / 50 | Mix: Noise / Noise: Tilt / Noise: Clock Ratio |
| LFO3 Rate / Depth / Shape | 3 / 13 / 21 | Noise: Bits / Transient: Level / (free) |
| **Other** | | |
| FX Amount | 73 | Drive: Amount |
| Patch Gain | 119 | Master: Volume |
| Drift | NRPN (CC 99/98, value on 6/38) | Vintage: Amount (14-bit) |

Not mapped (set them on the panel): Start/Mid/End waves,
noise and transient types, filter switches, Perform routing, Fundamental
wave and pulse width, Mod LFO Sync, and the rest of the sends (any send
can go in the script's map by its name, e.g. "Filter: Resonance LFO").

## Notes

- The Cobalt sends 7-bit CCs (128 steps). Abdication smooths its controls,
  so sweeps won't zipper, but the finest settings may be a step apart.
- Menu-type parameters (Algorithm, OscMod, Shape) step through their
  options as the knob turns.
- The Cobalt's screen shows its own values, not Abdication's. Watch
  Bitwig's parameter view (or, later, the GUI).
- Some of these may need Shift or a menu page on the Cobalt. If a knob
  doesn't respond during learn, check the Cobalt's MIDI out filter (CCs
  allowed) and that the control sends a CC at all.
