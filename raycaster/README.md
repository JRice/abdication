# Raycaster

A twelve-voice synthesizer, as a CLAP and VST3 plugin. Each oscillator is a shape turning
once per cycle about a center. A ray drops from above and measures how far it falls before
it hits an edge, and that distance is the waveform. Where the ray jumps between edges, or
misses and reads 0, the wave leaps, and the leaps are where the brightness comes from.

Design notes: `raycaster-design.md` in the Abdicant Synths project.

## Build and install

    powershell -ExecutionPolicy Bypass -File plugin\install.ps1

Quit Bitwig fully first (it keeps plugin processes alive), then rescan.

## The oscillators (pages I and II)

| Control | What it does |
|---|---|
| Element | Earth (O), Water (snowman), Air (square), Fire (M), Wood (crescent), Metal (S), or Chaos |
| Chaos | Three elements, each squished, skewed, tilted, sized and placed at random, overlapped. Every click rolls a new one; the seed is saved with the patch but not shown |
| Octave, Semi, Fine | Pitch |
| Level | The oscillator's level in the voice |
| Squish | Up to 4x wide or 4x tall |
| Skew | A shear, up to 45 degrees |
| Tilt | Perspective: the top (or, below centre, the base) leans away and shrinks |
| Center | How far right of the shape's middle it turns |
| Ray | How far right of the center the ray falls (only positive) |
| Orbit, Rate | The center circles this far, this fast (to 1 kHz) |
| Free | Free running: each note starts the shape (and its orbit) wherever, instead of upright |

Symmetry multiplies pitch: a centered square repeats four times a turn and sounds two
octaves up; squish or skew it and it repeats twice (an octave up); tilt it and it sounds at
the note.

The square window shows one turn of the resulting wave, live while a note plays. The wave
is centred (its mean over the last period is removed) before anything else sees it.

Chorus, in the top bar, is shared by both oscillators but runs on each separately, per
voice, before the filter: a 14 ms delay swept by Chorus % of itself at 1 Hz, the right
side a quarter cycle behind the left, mixed 50/50. Its knob runs 0–33 %; at 0 it is off
entirely.

Unison stacks up to six voices per note, spread over Detune (in cents, end to end) and
across the stereo field; a stack shares a note's level. Without Free, a stack starts
evenly staggered round the turn. Twelve voices in all, so ×2 plays six notes, ×3 four.

## Filter and amp

Tube drive (amount only, up to +40 dB, level-matched so it adds harmonics rather than
volume, and a true bypass at 0) into a 2-pole SVF or a 4-pole ladder, with its own envelope
(Env, in octaves) and key tracking. The SVF's Mode sweeps lowpass through bandpass to
highpass (it's dimmed with the ladder chosen). Amp ADSR with velocity. Poly (12 voices)
or Mono with SH-101-style glide: overlapping notes glide without retriggering. A stolen
voice fades out over 4 ms while the new note starts in a spare slot.

Every attack, decay and release (and the modulator envelopes' attack and decay) has a small
curve button at its knob's top right. Hold it and drag up or down: the curve bows up or
down, shown large while held. Double-click resets it.

The output limiter holds peaks to −1.5 dBFS, releasing over 40 ms.

## The dynamic modulator

Every control with a ring has its own modulator, per voice: an LFO (sine, saw, ramp,
square, noise, brown, S&H; free 0.01-50 Hz or synced to the host, with Retrig) and an
attack-decay envelope started by each note, plus how far the mod wheel and aftertouch
(channel or polyphonic) move it, either way. Click a knob to show its modulator along the
bottom; the selected knob's face fills with a miniature of the page's backdrop. On the
knob, the amber arc is the LFO's range, the teal arc the envelope's, and the dot is where
the newest voice has it now. The envelopes' own knobs don't have one.

## Source

- `plugin/src/dsp/`: shapes and their outline data, geometry (bends and the ray), the ray
  oscillator, chorus, modulators, envelopes, filters, the engine.
- `plugin/src/editor/`: the panel, its painter and the elements' backdrops.
- `plugin/examples/render.rs`: renders notes (held together, or in turn) to WAV and
  reports CPU; the keys are listed at its top.
  `plugin/examples/bench.rs`: times one ray on each element.

Fonts: Barlow Condensed and IBM Plex Mono, under the SIL Open Font License (see
`plugin/assets/fonts`).
