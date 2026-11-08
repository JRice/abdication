# Capturing the Strega

One pass of `strega_test.wav` (13:31) measures the delay's range, its taps, Filter,
Absorb, Decay and how it overloads. You follow `schedule.md`, moving a knob or two in
the gap before each take. `make_signal.py` regenerates all of it.

## Wiring

Interface line out -> External Substance In (a stereo cable is fine here). Strega
Line/Headphone out -> a line input on your interface, with a mono (TS) cable: a stereo
cable can cancel itself in a balanced input. Don't use the Modular output: it's too hot.
No other cables, except a dummy cable in Agitation Begin/End so Agitation stops.

## Bitwig

- Record at 96 kHz if your interface does it, otherwise 48 kHz. Use 24-bit.
- Track 1: `strega_test.wav`, starting at bar 1, with stretching off (Raw, no tempo
  lock), routed to the out that feeds the Strega. Output fader at 0 dB.
- Track 2: records the Strega, starting at the same moment, with no effects, and
  monitoring off if it would feed back.
- Don't change the interface's output or input gain during the pass.

## Starting positions

| Control | Position |
|---|---|
| Activation Constant, Activation Interference | full CCW |
| Tonic Modulation Interference, Time Modulation | full CCW |
| Filter CV attenuator, all other attenuators | 12:00 |
| Strength | full CCW |
| External Constant | full CW |
| Decay, Absorb | full CCW |
| Filter | full CW |
| Blend | full CCW (the first take is dry) |
| Time Fine | 12:00 |
| Level | 12:00, then leave it alone |

Before the pass, check the level: Blend full CW, Time 1:00, Decay 3:00, and play the
file's first chirps. Set the interface's input gain so peaks sit around -12 dBFS, leaving
room for self-oscillation. Then return to the positions above. "Time" means Time Coarse.

## During the pass

Follow `schedule.md`. Each row's moves should be done by the time on that row; there are
at least 8 seconds to make them. Knob positions are clock positions, and near enough is
fine. Missed one? Keep going and tell me which take.

## Afterwards

Export track 2 as a single mono WAV of the whole pass, starting exactly at bar 1 with no
normalizing, to `recordings/pass1.wav`.

Optional, and useful: a few 30 to 60 second recordings of settings you love, each with a
photo or a note of the knobs, also in `recordings/`. They're the reference for tuning by
ear once the numbers are close.
