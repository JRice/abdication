# What the recording says the machine is

Source: Oscillator Sink's circuit-bent Yamaha R100 video, `artifact/circuit_bent_reverb.mp3`, compared with the R100 service manual and Circuitbenders' mod notes. Times below are positions in that audio.

## Measured

| Clock | RAM loop period | 16,384 / period | Independent evidence at the same point |
|---|---|---|---|
| Low (7:20) | 11.587 s | 1414 Hz | Spectrum nulls every ~1414 Hz, plus a line there |
| Mid (13:50) | 2.426 s | 6752 Hz | Line at 6753 Hz |
| Mid (14:15) | 2.101 s | 7797 Hz | Line at 7798 Hz, null at ~7.8 kHz |

So the loop is 16,384 clock ticks long, and the nulls and the whine are at the clock.

- **Memory:** 16,384 16-bit words. This fits the manual's parts list, which reads as a 256 Kbit pseudo-static RAM (TC51832, 32K x 8). The OCR is rough, so this part number is a best reading.
- **Nulls:** the sin(x)/x pattern of a zero-order hold at the clock rate. Between them, mirrored images of the audio, which are the "glisten".
- **Output filter:** a fixed cliff near 14 kHz at every clock, so the images pass through. The model's filter corner is 13.3 kHz, set by matching this cliff.
- **Whine:** lines at the clock and its harmonics. This is what the video's narrator notches out.
- **Stereo offset:** right lags left by close to half a clock tick in the dry beats section: about 42 us with the clock near 10.5 kHz and 63 us near 7 kHz, against 48 and 71 for exactly half. Those clocks are read off the spectrogram, so they're approximate. Near the top of the knob it's within a few microseconds of zero.

## From the documents

- 16-bit, 31.25 kHz stock clock, 20 Hz to 12 kHz (service manual; the OCR is rough but the values are legible).
- One YM3901 DSP and one PCM54 DAC, with a comparator and analog multiplexers, which fits one DAC serving both channels in turn. The narrator also hears the stereo offset grow as the clock falls. In the model, both inputs convert together and right comes out half a tick after left.
- Pre-emphasis and de-emphasis appear in the block diagram. Time constants aren't legible, so 50/15 us is an assumption.
- The Reclock knob sets the clock of the RAM and both converters. The matrix has 16 switches, and nothing happens until two or more are on. The loop "grabs whatever is in the sample RAM" and is usually non-destructive (Circuitbenders).

## Inferred, not measured

- **Which line each switch is.** The video only describes each row: tones, fast tremolo and waveshaping, rhythms that follow the clock, crunch. Assigning fast address lines, middling ones, slow ones and data lines follows from what each would do, and the rhythm row following the clock fits. A top-row switch is the sign bit, as a stand-in for the "danger" switch.
- **How shorted lines resolve.** Low wins, with the Contention knob as an invention to let the other side win sometimes.
- **The reverb programs.** Yamaha's firmware isn't available, so these are figure-of-eight networks, tapped lines and a reversal, all in one ring. They take the names of the unit's program types, not its algorithms.
- **Rot.** Nothing in the recording shows it, and the real loop is non-destructive. It's what might happen to pseudo-static RAM whose refresh comes from a slow clock, and it's off by default.

## Checked against the model

`examples/render.rs` renders scenes with an arpeggio into the model. Compared with the recording, the 1414 Hz scene reproduces the null spacing and the lobes' fall with frequency to within a few dB, the frozen-loop period measures 11.58698 s against 11.58699 s expected, and the cliff lands in the same place. With no input, about half of the switch pairs stay silent, and the rest sing, mostly as noise-heavy washes, some tonal and some rhythmic, depending on the program. That's emergent rather than tuned to the recording.

Not reproduced: the particular rhythms and combs of his unit, since they depend on its firmware and wiring.
