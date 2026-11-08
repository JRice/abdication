# What the Strega's delay measured

From one 14-minute pass of `capture/strega_test.wav` through a real Strega, recorded
at 48 kHz on 3 Nov 2026. `tools/compare.py` repeats every measurement on the model
(`plugin/examples/render_pass.rs`) and prints the two side by side.

## Layout

Three delay chips in **parallel**, each on its own clock. The tap times don't keep a
fixed ratio, which is why they're tabulated per chip in `voice.rs`.

| Time knob | A (s) | B (s) | C (s) |
|---|---|---|---|
| full CCW | 3.138 | 5.764 | not found |
| 9:00 | 1.212 | 2.359 | 9.476 |
| 10:00 | 0.545 | 1.071 | 2.681 |
| 12:00 | 0.197 | 0.377 | 0.778 |
| 1:00 | 0.098 | 0.174 | 0.332 |
| 2:00 | 0.073 | 0.123 | 0.225 |
| 3:00 | 0.049 | 0.076 | 0.126 |
| full CW | 0.036 | 0.048 | not found |

Relative output: A 0 dB, B -1.4 dB, C -5.8 dB.

## Feedback

- A feeds itself -2.65 dB per pass and B -8 dB, even with Decay fully down. C doesn't.
  So the shortest possible tail is about 3 seconds of repeats from A.
- Decay is **cross-feedback**, A to B and B to A, with C coupled about 15 dB more weakly.
- That path loses lows below about 420 Hz, and it's level-dependent: big signals hang on,
  quiet ones fall at chip A's own -27 dB per second, all the way into silence.
- Self-oscillation starts between Decay 2:00 and 4:00, at about 800 to 1000 Hz, peaking at
  half the recording's full level.

## Filter and Absorb are outside the loop

This surprised me, since the manual says the Filter feeds Decay. Going from Filter fully
open to 10:00 at Decay 2:00 cut the chirp's echoes by about 25 dB but barely changed how
fast they decayed, and at Filter closed the repeats still fell only 2.7 dB per pass.

The Filter is a low-Q (0.4) two-pole lowpass, from 98 Hz at full CCW to 8.5 kHz at 3:00.
Absorb lets back in highs (2.2 a^3) and adds resonance at the cutoff (5.1 a^2, with Q
from 0.4 to 2 and the cutoff dropping about 22 %). That form fits the sweeps to within
about 0.6 dB at two Filter settings.

## The chip

- Silent at idle while the clock is fast: below the interface's -92.5 dB floor at Time
  12:00, even with chip C at 778 ms. Noise rises about 60 dB across the Time range.
- A plain leaky delta modulator was 30 dB too noisy at short delays and had a dead zone.
  Accumulating the comparator's error fixes both, which agrees with the chip being
  described as delta-sigma.
- Whine near 14 kHz at full CCW, close to A's 14.0 kHz bit rate for 44,000 bits.
- No loud tone at half or a quarter of a chip's bit rate, and hiss at long delays is broad:
  about flat from 50 Hz to 3 kHz at full CCW, -25 dB overall. An early version of the model
  made such a tone (about 2 kHz at Time 8:00), from chip C sitting in a perfect idle toggle.
  The decoder now cancels that toggle, near-silence can't lock into the quarter-rate one, and
  the comparator gets noisy at slow clocks. That brought the full CCW noise to within about
  2 dB of the Strega from 200 Hz to 1.6 kHz. A Whine control brings the tone back by choice.
- Low burble: tones at 35, 69, 104, 138, 173 Hz at Time 9:00, and at 111, 223, 334 Hz at
  10:00. They track the clock, one kick about every 133 bits, loudest on the slowest chip.

## Levels

Dry passes at -6 dB as recorded, with an AC-coupling highpass near 45 Hz. Wet clipping
sets in around -9 dBFS at the input. Strength at 12:00 adds about 7 dB.

## How close the model is

Close: tap times and gains; Filter and Absorb (usually within 1 to 3 dB up to 12 kHz);
noise with a closed Filter at Time 9:00 and full CCW; Decay 9:00 ring-down; level and
pitch region of self-oscillation; falling all the way into silence.

Not there yet:
- At Decay 12:00 and 2:00 the tail lasts about half as long as the Strega's. That stretch
  of the Decay knob is touchy, close to the point where it sustains.
- Noise above 3 kHz runs up to 10 dB hot around Time 12:00 to 2:00. C's delay at full
  CCW is still a guess.
- Self-oscillation is a little cleaner and narrower than the Strega's.

## Not measured

Agitation and Interference: their ranges come from the manual, the routing depths and
Interference's shape are guesses. Strength above 12:00 is assumed. Stereo Spread is an
addition; the Strega is mono.

## Caveats in the recording

- The Filter was effectively closed from about 0:23 until the Filter takes began (the
  Time takes' response and noise match the `filter-7` take), so noise with the Filter
  open comes from the sweep at the end.
- The last two takes, `levels-slow` and `time-sweep`, were re-recorded and spliced in.
  They reached the delay about 16 dB lower than `levels-fast`. The dry leak at Blend full
  CW (-64.5 dB at 1 kHz, -72.5 dB at 100 Hz) didn't change, so the interface and the input
  jack weren't the cause. The leak doesn't follow Strength either, so it can't separate
  Strength from External Constant. Either way it only affects those two takes: the
  model's input level comes from the takes before them.

## An open question: the jump

Moving Time away from long delays gives a burst the model doesn't have: +17 dB and
weighted to the lows for about 2.5 s at 13:47, ending abruptly, and +10 dB at 0:34.
A quick move in the model just fades out. Possibly the Time control bleeding into the
audio, since the chip's reference voltage is known to shift with clock modulation; possibly
something in the memory. Telling them apart needs a recording with the knob moves known.
