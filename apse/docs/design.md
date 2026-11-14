# Apse: design

Started 2026-11-13. A reverb loosely after Spire's, built from the published design its Plate
modes point to rather than from Spire's own settings.

## Where it comes from

The structural analysis is `abdicant-syn/research/spire-reverb.md`. In short: Spire's binary holds
a readable table for seven reverb modes, and the Plate modes' layout (four input diffusers; per
side two tank allpasses and two long delays; seven signed output taps per side; a six-tap
multitap; predelay) matches Jon Dattorro's plate in *Effect Design, Part 1* (JAES, 1997). Other
modes differ: Hall has six input diffusers, Sparkle and Ambience three, and dMension and Aura have
diffusion switched off and very short tanks. Apse had a Dimension mode after dMension; it was
dropped on listening (2026-11-13), as was a second plate: one plate is enough. No Spire values were copied; the analysis didn't
record them and the measurements below are against Dattorro's published reference.

## Signal flow

```text
in L+R → mono → pre-delay → bandwidth lowpass → 0–6 input allpasses → tank
                    └→ six early-reflection taps (stereo) ───────────────────────┐
tank: two branches, each fed by the input plus the other branch's output         │
  allpass (moving) → delay → damp → ×decay → allpass (moving) → delay → ×decay   │
seven signed taps per side, mostly from the far branch → late ────────────── + ←─┘
  → Width (M/S) → Color (HP then LP) → Mix → Level → limiter
```

The tank's line lengths and tap positions are Dattorro's (samples at 29 761 Hz), scaled by the
mode's size, the Size knob and the host rate. Every tank line reads with cubic interpolation, since
Size and modulation keep them moving; taps read linearly. Departures from the reference:

- All four tank allpasses wander, each on its own sine at unrelated rates (×1, 1.137, 0.719,
  0.853 of the mode's rate), the second pair half as far. The reference moves only the first pair.
- A 6 Hz DC blocker sits beside each branch's damping, so Freeze can't hold anything sub-audible.
- Decay is set from Time: one trip round the figure of eight multiplies by Decay four times, so
  Decay = 10^(−2 · loop / (4 · Time)) for a 40 dB fall, capped at 0.99995 unless frozen.
- Early reflections are six pre-delay taps (7–42 ms at spread 1, scaled by Size), panned
  alternately, added to the output only.

## Modes

| Mode | Diffusers | Size | Tank diffusion | Wobble | Taps | Early | Wet low cut |
|---|---|---|---|---|---|---|---|
| Plate | 4 (reference) | 1.0 | 0.70, 0.50 | 0.54 ms at 1 Hz | even | — | 20 Hz |
| Hall | 6 | 1.4 | 0.65, 0.50 | 0.9 ms at 0.35 Hz | even | 0.35 | 25 Hz |
| Sparkle | 3, under 2 ms | 0.5 | 0.72, 0.62 | 0.12 ms at 2.3 Hz | allpass taps ×1.5, others ×0.25 | — | 350 Hz |
| Ambience | 3 | 0.42 | 0.60, 0.50 | 0.3 ms at 0.8 Hz | near branch up | 0.8, tight | 60 Hz |
| Aura | none | 0.3 | 0.75, 0.60 | 0.7 ms at 0.5 Hz | five (outer pair off) | — | 30 Hz |
| Choir | 4 (reference) | 1.15 | 0.72, 0.55 | 0.7 ms at 0.4 Hz | even, then sung (see Choir) | — | 70 Hz |

Each mode's late level was trimmed so a 2-second tail lands within about half a dB of the others.

## Choir

Added 2026-11-14, after the Strymon BigSky's Chorale. Its documented controls (vowel AH/OH/OO and
mixes or random, resonance as the vowel filters' Q, modulation as random pitch and timbre
variation, tone with "breath") set the target; its internals weren't recoverable from the plugin,
so this is an original design aimed at the same three things: a smooth sustained tail, several
coordinated vocal resonances, and slow, decorrelated variation that reads as an ensemble.

```text
tank (as Plate, a little larger, slower movement, 70 Hz low cut)
  → six voices, each fed its own L/R mix of the tail:
      wandering delay (11–29 ms, ±4 ms at Ensemble 100 %: up to ~20 cents)
      + breath (noise × the tail's envelope)
      → three bandpass formants (F1–F3 of AH/OH/OO, after Peterson and Barney),
        scaled by the voice's own vocal-tract size (0.94–1.13, wandering ±7 %)
        and at its own drifting vowel
  → panned across the field, plus 15 % of the unsung tail for body
```

The formants sit after the tank, outside the feedback, so Time and Freeze behave as in every
other mode and the vowel changes at once. Each voice's three wanderers (pitch, size, vowel) are
independent cosine-eased random walks with periods around 1, 2 and 3 s, so nothing moves in step.
Formant shapes update every 16 samples; the delay's wander is interpolated per sample.

Level: narrower formants pass less of a broad tail but as much of a note that lands in them, so
the makeup gain goes with Q^0.35. On the listening phrase (plucked saws and a snare, Mix 100 %)
the Choir comes within 0.5 dB of Plate at default Resonance, and within about 2 dB of it across
the whole Resonance range; on a noise burst it reads about 4 dB lower. OO is a few dB quieter
than AH, as it should be. It costs 2.6 % of a core at 48 kHz, the most of any mode.

## Measurements

Time is the fall to -40 dB, not the usual -60: on listening, a 30 s RT60 tail was gone to the
ear after about 40 dB, so the number now reads as what's heard, and the top of the range lasts
45 s by the RT60 measure.

`cargo run --release --example measure`: a 50 ms noise burst, Mix 100 %, Damp 35 %, 48 kHz.
T40 and T20 (extrapolated to 40 dB) come from Schroeder integration; level is wet energy over
input energy; correlation is L/R from 0.2 to 1 s.

| Mode | T40 (asked 2 s) | T20 | Level | L/R correlation |
|---|---|---|---|---|
| Plate | 1.92 | 1.84 | −1.2 dB | −0.01 |
| Hall | 2.04 | 1.88 | −1.6 dB | −0.00 |
| Sparkle | 1.96 | 1.83 | −0.4 dB | 0.01 |
| Ambience | 1.72 | 1.71 | −1.5 dB | −0.03 |
| Aura | 1.83 | 1.78 | −0.9 dB | 0.00 |
| Choir | 2.16 | 2.06 | −5.1 dB (noise; matched on music) | 0.38 |

At 8 s asked, all measure 7.0–7.2 s. Short settings bottom out where one trip round the tank
already takes that long (Hall can't go much under 0.7 s). Freeze holds every mode for 12 s within
a few dB without growing (a test). The heaviest setting (Hall, Size 200 %, Mod 100 %) takes 1.4 %
of one core at 48 kHz.

## Panel

"Fresco", chosen 2026-11-13 from three mock-ups (half-dome, broken glass, fresco) on the Apse
panel design canvas. Dark plaster mottled with irregular stains (not gradients, which band at
these levels) and a few cracks kept to the bare wall; Time in Cinzel inside a worn gilded halo on
the springline, its thirteen rays lighting up with the Time knob; the five modes as Roman
numerals; sliders as grooves with a gold tick; Freeze as a sanctuary lamp, lit red while the tail
is held; the tail's level over the last eight seconds and the limiter under it. The Choir's five
controls sit under the tail graph, tarnished (dim labels, dull ticks) unless Choir is chosen; the
rows were made smaller to fit them.

## Open

- Spire's six-tap multitap may feed the tank rather than the output; here it's output only.
- Spire's two mode-specific EQ entries (frequency/Q/gain) aren't modelled; each mode has a fixed
  wet low cut instead.
- The input is summed to mono, as in the reference.
