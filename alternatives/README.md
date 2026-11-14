# Alternatives

Abdication's Events as a CLAP/VST3 effect, by Abdicant: brief, random bumps and holes in
the spectrum of whatever comes in, landing on the harmonics of the notes being played.
Each of up to 12 slots waits a random gap, then raises a narrow bump on a harmonic (a walk
wanders its gain and frequency, and can turn it over into a cut), then waits again. Both
channels get the same bumps, so the image stays put. A slow leveler holds the average
level, so it reshapes the sound rather than swelling it. No panel: the host shows the
parameters. Faust DSP (`dsp/`), compiled to Rust.

## The notes

**Source**, MIDI (the default) or Root. With MIDI, route the notes of the track it's on
(in Bitwig: the device's note input, or put it after the synth in the same chain), and
each bump picks one of the notes in play: those held, and those let go since the last note
started, so a release tail keeps its bumps until the next note. Up to 8, the newest. Until
a note arrives it plays **Root**, which Source Root always does.

## Parameters

| Parameter | What it does |
|---|---|
| Density | share of the time each slot is in a bump (80 %) |
| Depth | a bump's height, 0–18 dB (6; Abdication's ran to 12, off by default). 0 is the input untouched |
| Life | how long bumps last: 1 is about 400 ms low down to 40 ms at 9 kHz (2) |
| Sharp | narrower bumps above 1, toward a vocoder at high Depth and Density (1) |
| Slots | how many bumps can be under way at once, 1–12 (6, as Abdication had; unused slots aren't computed) |
| Reach | the highest harmonic a bump starts on, favouring the low ones (64, as Abdication had) |
| Walk | 0 a still bell; 1 wanders in gain and frequency, and can turn a bump over (60 %) |
| Drift | how far Walk wanders by itself, every few seconds (27.5 %: Abdication's fixed 0.605 ± 0.275) |
| Cut | how much deeper a turned-over bump cuts than it boosts (3.33×, Abdication's) |
| Hold Level | the leveler, on by default (3 s) |
| Mix | dry/wet |
| Output | −24…+12 dB |

CPU here: 2.6 % of a core at 6 slots, 6 % at 12 (`cargo test --release cpu -- --ignored --nocapture`).

## Building

```powershell
powershell -ExecutionPolicy Bypass -File plugin\install.ps1
```

After changing `dsp/`, run `tools/gen-rust.sh` (native Faust 2.70+) to regenerate
`plugin/src/dsp.rs`. Tests: `cargo test --release` in `plugin/`.
