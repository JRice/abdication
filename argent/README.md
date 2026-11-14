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

- **Notes latch.** A key starts a note fading in; letting go doesn't end it. Press
  the same key again and it fades out; again, and it comes back from wherever it got to.
- **Four voices, nothing jumps.** A fifth note takes over a voice already playing (one
  on its way out if there is one, else the oldest): it slides to the new pitch over the
  fade time and fades back in from its current level.
- **Release** fades out every note whose key isn't down. It's a parameter, so map any
  controller button to it in Bitwig.
- **Fade** and **Curve** set how notes arrive and leave: level = position ^ curve, the
  same both ways. Each note's harmonics arrive after it and leave before it.

## Controls

| Group | Control | What it does |
|---|---|---|
| Materia | Morph | Position inside a table (frames). |
| | Scan | Position in the bank of 34 tables, gliding between them. Tables 1-16 are drawn per seed: table 1 is always the Metal Comb shape, later ones grow rougher, 14-16 are jagged Grit. Tables 17-34 are the recorded Phizmo sweeps (the void's readout names them). |
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
- `plugin/assets/recorded.bin`: the keyframe waves of 18 recorded Phizmo sweeps (in
  `fizmo waves`), each table gliding through its sweep's keys. Rebuild it with
  `python plugin/tools/recorded.py "fizmo waves" plugin/assets/recorded.bin` (needs numpy,
  scipy, ffmpeg). It's derived from those recordings, so keep it out of anything shared.

## Recorded tables

Each sweep holds its first wave for about half a second, sweeps through two or three
more, and holds its last. The tool finds the pitch (ten of the files are at C1, eight at
C2), measures every single cycle's harmonics up to 16 kHz, and keeps the held first wave,
the sweep's turning points and the held last wave: 2 to 5 keys per table. Argent spreads
the keys evenly across 64 frames, gliding magnitudes and phases between them, and turns
loud low-heavy frames down so they sit at the drawn tables' level through the body.

| Table | Sweep | Keys |
|---|---|---|
| 17 | BELL-X-WAVE 3 | 5 |
| 18 | FAT SQUAREZ | 5 |
| 19 | TRANS BOY | 5 |
| 20 | QUITE ODD | 5 |
| 21 | TINE X WAVE | 5 |
| 22 | VOWELS | 5 |
| 23 | SYLLABLES | 5 |
| 24 | TRANSWAVE AH | 5 |
| 25 | TRANSWAVE OO | 5 |
| 26 | CYBORG | 5 |
| 27 | TRANS E PIANO | 3 |
| 28 | GREEN TRANCE | 5 |
| 29 | TRANS REZ BASS | 3 |
| 30 | STEP UP | 5 |
| 31 | REZ SQUARE | 5 |
| 32 | POLYMOD SWEEP | 5 |
| 33 | ANALOG PAD X | 3 |
| 34 | TRI SWEEP | 2 |

Fonts: IM Fell English and IBM Plex Mono, under the SIL Open Font License (see
`plugin/assets/fonts`).
