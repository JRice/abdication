# Calls

Abdication's Calls as a CLAP/VST3 effect, by Abdicant: sparse, phrased whistles, each a
small sweep between harmonics of a note, through an echo into a reverb, added on top of
whatever comes in (the input passes underneath untouched). A phrase is a motif that
recurs, a little nudged each time; its pace scales with the note, as a transposed ROM loop
would, so a chord's callers drift against each other. No panel: the host shows the
parameters. Faust DSP (`dsp/`) for the callers and reverb, compiled to Rust; the echo is
Rust, so it can follow the tempo.

## The notes

**Source**, MIDI (the default) or Root. With MIDI, route the notes of the track it's on (in
Bitwig: the device's note input, or put it after the synth in the same chain), and each
held note gets a caller of its own, as each of Abdication's voices had (up to 8; a note
let go lets its caller fade out over about a second). Until a note arrives it plays one
caller on **Root**, which Source Root always does.

## Parameters

| Parameter | What it does |
|---|---|
| Level | the calls, −60…0 dB (−24) |
| Activity | how often a phrase comes: 0 about a minute apart, 1 about half a second (45 %) |
| Rate | the phrasing's speed (1×) |
| Follow | 0 the calls run on their own; 1 (the default) they sound while the input does, fading as it falls from −20 to −60 dBFS, as Abdication's faded with each voice's amp envelope |
| Width | how far apart the calls sit, each note a new place (100 %) |
| Echo | into the echo (25 %) |
| Feedback | 0–95 % (45 %) |
| Echo Damp | a lowpass in the echo's loop, each repeat darker (20 kHz: open) |
| Sync | the echo's times follow the tempo (Division L/R) rather than Time L/R |
| Time L, Time R | 10 ms – 4 s (360 and 480 ms, Abdication's) |
| Division L, Division R | 1/32 … 1 bar, with dotted and triplets (1/8 dotted and 1/4: Abdication's times at 120 BPM) |
| Reverb | 100 % (Abdication's): the calls and their echo are heard only through the reverb; 0 dry |
| Decay | the reverb's mid decay, 0.5–20 s (5; the lows decay at 80 % of it) |
| Damp | where the reverb's highs start to die faster (2.5 kHz) |
| Pre-Delay | 0–150 ms (30; held to what Zita's pre-delay holds, about 165 ms at 48 kHz) |

The defaults are Abdication's calls with Level up and the Echo knob at half. CPU here:
1 % of a core with one caller, 4.6 % with eight (`cargo test --release cpu -- --ignored --nocapture`).

## Building

```powershell
powershell -ExecutionPolicy Bypass -File plugin\install.ps1
```

After changing `dsp/`, run `tools/gen-rust.sh` (native Faust 2.70+) to regenerate
`plugin/src/calls_dsp.rs` and `reverb_dsp.rs`. Tests: `cargo test --release` in `plugin/`.
