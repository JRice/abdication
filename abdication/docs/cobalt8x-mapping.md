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
| **Oscillator 1 → I's Dual** | | |
| Osc1 Algorithm | 62 | Dual I: Algorithm |
| Osc1 Param A | 16 | Dual I: A |
| Osc1 Param B | 19 | Dual I: B |
| Osc1 Tune | 14 | Osc I: Pitch |
| Osc1 Fine | 15 | Unison: Detune |
| **Oscillator 2 → II's Wavetable** | | |
| Osc2 Algorithm | 63 | WT II: OscMod |
| Osc2 Param A | 17 | WT II: Speed |
| Osc2 Param B | 20 | WT II: OscMod Depth |
| Osc2 Tune | 30 | Osc II: Pitch |
| Osc2 Fine | 31 | Unison: Spread |
| Osc Mix | 18 | Mix: Balance |
| **Filter** | | |
| Cutoff | 34 | Filter: Cutoff |
| Resonance | 35 | Filter: Resonance |
| Morph | 33 | Filter: A (the SVF's morph; a ladder's pole spread) |
| **Filter EG → envelope B** | | |
| FEG Amount | 32 | Filter: Env B (B's depth on the cutoff) |
| FEG Attack / Decay / Sustain / Release | 22 / 23 / 24 / 25 | Env B: Attack / Decay / Sustain / Release |
| **Amp EG → envelope A** | | |
| AEG Attack / Decay / Sustain | 26 / 27 / 28 | Env A: Attack / Decay / Sustain |
| **LFO1 → LFO X** | | |
| LFO1 Rate / Shape | 36 / 39 | LFO X: Rate (the knob reversed: it ran backwards) / LFO X: Shape |
| LFO1 Depth | 37 | Filter: HPF (LFO X's depths are set on the panel) |
| **Mod envelope → envelope C** | | |
| MEG Attack / Decay / Sustain / Release | 43 / 44 / 45 / 46 | Env C: Attack / Decay / Sustain / Release |
| MEG Depth | 49 | Filter: HPF Peak |
| **LFO2 and LFO3 → noise and transient** | | |
| LFO2 Rate / Depth / Shape | 47 / 48 / 50 | Mix: Noise / Noise: Tilt / Noise: Clock Ratio |
| LFO3 Rate / Depth / Shape | 3 / 13 / 21 | Noise: Bits / Transient: Level / (free) |
| **Other** | | |
| FX Amount | 73 | Drive: Amount |
| Patch Gain | 119 | Master: Volume |
| Drift | NRPN (CC 99/98, value on 6/38) | Vintage: Amount (14-bit) |

The map assumes the default slots, a Dual in I and a Wavetable in II. A
slot switched to another engine keeps the hidden engine's settings, so
its knobs still move them, unheard; II's Dual and I's Wavetable can go in
the map by name ("Dual II: A", "WT I: Speed") to suit.

Not mapped (set them on the panel): the slots' engines, the Dual waves
and the wavetable waves, noise and transient types, filter switches,
Perform, envelope A's release and the curves. The modulation depths (how
far each source moves each knob, set with the source buttons) are hidden
from the host, so the script can't reach them.

## Notes

- The Cobalt sends 7-bit CCs (128 steps). Abdication smooths its controls,
  so sweeps won't zipper, but the finest settings may be a step apart.
- Menu-type parameters (Algorithm, OscMod, Shape) step through their
  options as the knob turns. The Dual's twelve algorithms are in the
  Cobalt8's own order (VA Sweep, Spread, PWM, Sync, Fractal, Window Amp
  Sync, Metal, Ring Mod, FM, Noise AM, Chaos, Folding), so the knob walks
  the same families in the same direction; the waves are picked
  separately (Dual I: Wave 1 and Wave 2).
- The Cobalt's screen shows its own values, not Abdication's. Watch
  Bitwig's parameter view (or, later, the GUI).
- Some of these may need Shift or a menu page on the Cobalt. If a knob
  doesn't respond during learn, check the Cobalt's MIDI out filter (CCs
  allowed) and that the control sends a CC at all.
