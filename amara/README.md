# Amara

A personal CLAP/VST3 instrument that talks to itself in an invented language, over a bad line. It needs no notes: load it and it starts. Install, from this folder, with any project using it closed:

```
powershell -ExecutionPolicy Bypass -File plugin\install.ps1
```

How it works is in [docs/design.md](docs/design.md).

## Controls

**Language** picks one of 1,000 seeded languages, each with its own inventory, syllable shapes, stress rule, rhythm and word pool. What it calls itself is at the panel's top right. The panel's transcript is the IPA of what it's saying, red while possessed.

**Strangeness** is how far a language leaves the common sounds. At 0 it's a thoroughly ordinary language, and at 1 it's nearly all throat consonants and ejectives. By default it's an ordinary language with a few signature sounds. Moving it rebuilds the language, so its words change.

**Rate** is roughly syllables a second: accurate around 5, and about 12 at the top of the knob. Vowels compress much more than consonants, as in real fast speech, so it never turns into a sped-up tape.

**Sparsity** runs from barely stopping for breath to long silences and short phrases.

**Temper** runs from hushed to furious, moving pitch, range, effort, brightness, speed and roughness together. Near the top the vocal folds fall out of step and the pitch drops into a growl an octave down. **Swing** is how far temper moves, between phrases and in an arch through each one.

**Breath** runs from voice to whisper. **Throat** is the tract's length: longer is darker and larger. **Pitch** is the centre of its speaking range, and a held MIDI note replaces it.

**Per note** changes the mode entirely. It stays silent until a note arrives, and each note-on says the next syllable of its running stream of words, at exactly that note's pitch. Velocity is Temper, and note-offs are ignored. A new note cuts off the last one, and only the last of a chord counts. Rate still sets how long syllables last. Sparsity, Temper, Swing and Pitch go dim, since they do nothing here. The hardest hits still growl, which drops the pitch about an octave.

**Possession** is how often something takes the voice, from never to nearly always, and **Extremity** how far it goes. **Seize** keeps it going until you turn it off, which is automatable. The ten forms are a giant, a mite, slow countable pulses, a legion of voices in stereo, narrowing resonances that ring, a stuck mouth, speech played backwards, a stutter, a line in pieces and a howl.

**Line** is bursts of real bit errors in the codec's frames, plus lost frames that repeat. **Fidelity** is the codec's quality, from 6 kHz with 30 ms frames to 16 kHz with 10 ms frames.

**Level** goes into a limiter at -0.5 dBFS. Shift for fine, double-click to reset, scroll to nudge. Scrolling on Language moves one language at a time.

Some languages are worse than others. 666 won't let Possession below 66 %, whatever the knob says, until you leave it, and the knob's own setting comes back when you do. The tentacles behind the panel only exist while it's open, and have no effect on the sound.

## Layout

`plugin/src/dsp`: `phones.rs` is every sound and how it's made, `language.rs` the seeded languages, `speaker.rs` the phrases and tract, `codec.rs` the frames and the line, `voice.rs` the decoder's far end, `possession.rs` what takes over, `engine.rs` the chain. `examples/babble.rs` prints samples of several languages.

Fonts: Spline Sans Mono (SIL Open Font License) and a subset of DejaVu Sans for the IPA (`plugin/assets/fonts`).
