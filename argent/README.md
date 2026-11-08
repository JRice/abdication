# Argent

A four-voice drone instrument, as a CLAP and VST3 plugin. Its source is Fizmo-style
transwaves: dense, rough wavetables read the way 1990s sample playback read them,
without band-limiting. It's played the Vhikk X way, with MORPH, SCAN and SEED, and it
has no delay or reverb. Its life is in its motion.

Design notes: `fizmo-notes.md` and `drone-design.md` in the Abdicant Synths project.

## Build and install

    powershell -ExecutionPolicy Bypass -File plugin\install.ps1

Quit Bitwig fully first (it keeps plugin processes alive), then rescan.

## Playing it

- **Notes latch.** A key starts a note fading in; letting go doesn't end it. Four
  notes at most; a fifth cuts the oldest (with a few milliseconds of fade).
- **Release** fades out every note whose key isn't down. It's a parameter, so map any
  controller button to it in Bitwig. Pressing a fading note again turns it round.
- **Fade** and **Curve** set how notes arrive and leave: level = position ^ curve, the
  same both ways. Each note's harmonics arrive after it and leave before it.

## Controls

| Group | Control | What it does |
|---|---|---|
| Materia | Morph | Position inside a table (frames). |
| | Scan | Position in the bank of 16 tables, gliding between them. Table 1 is always the Metal Comb shape; later tables grow rougher; 14-16 are jagged Grit. |
| | Seed | A different bank and different per-voice offsets. Click for a new one, drag or scroll to step. |
| | Span | Adds the octave, the octave and fifth, then two octaves to every note, one after another. |
| Lumen | Tone | The dark body: a 4-pole lowpass. |
| | Fizz | The oscillators' raw top above 2 kHz, folded partials included, laid quietly over the body. |
| Motus | Tide | How far a slow walk (minutes) moves Scan and Morph. |
| | Flare | Seconds between flares: swells that open the tone without getting louder. |
| | Depth | How far a flare opens the tone, in octaves. |
| | Bloom | The fast flicker inside each flare, on tone and wave. |
| | Tremor | A gentle tremolo near 2.9 Hz. |
| | Wander | Each voice's pitch drifting, in cents. |
| Adventus | Fade, Curve, Release | Above. |
| | Frames 64 / 32, Read Step / Glide | Frames change at the start of a cycle (step) or blend (glide). |
| | Volume | Output level, before a safety limiter at -0.5 dBFS. |

Dragging up or down inside the void changes how much of the eye shows.

## Source

- `plugin/src/dsp/`: tables (bank built per seed with an inverse FFT), oscillators,
  notes (latch, release, steal), modulation, engine.
- `plugin/src/editor/`: the panel (`panel.rs`) and its sky, eye and sentences
  (`sky.rs`, `assets/lines.txt`).
- `plugin/examples/render.rs`: renders a held chord to WAV, for measuring.

Fonts: IM Fell English and IBM Plex Mono, under the SIL Open Font License (see
`plugin/assets/fonts`).
