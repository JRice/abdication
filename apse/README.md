# Apse

A personal CLAP/VST3 reverb in the plate family, loosely after the one in Spire: a Dattorro-style
tank with six modes, the last a choir after the Strymon BigSky's Chorale. "Apse" is a working name. Where it comes from and how it measures is in
[docs/design.md](docs/design.md).

Install, from this folder, with any project using it closed:

```
powershell -ExecutionPolicy Bypass -File plugin\install.ps1
```

## Controls

**Time** is the decay to -40 dB, from 0.2 to 30 seconds: about where a tail drops out of hearing,
so the number is what you hear. (Rated the usual way, to -60 dB, tails last half as long again.) Drag the numerals up and down, or the
groove under them.

**Modes:** Plate, Hall, Sparkle, Ambience, Aura, Choir. They share one tank and
differ in how many diffusers feed it, how big it is, how it moves, how it's read and how much
early reflection rides on top. Changing mode fades the wet signal out, clears the memory and
fades back in.
- Plate: the reference plate.
- Hall: six diffusers, a bigger tank, slow movement and early reflections.
- Sparkle: hair-short diffusers into a small tank read mostly from its allpasses; glassy, no bass.
- Ambience: short, with strong early reflections and a quieter tail.
- Aura: no diffusers, a small tank with high diffusion inside it; it blooms rather than starts.
- Choir: the plate's tail sung by six voices, each with its own drifting pitch and vowel. Its own
  controls sit under the tail graph:
  - **Vowel** runs AH, OH, OO, sliding smoothly between them.
  - **Drift** lets each voice wander off the vowel on its own, up to halfway along.
  - **Resonance** narrows the vowel resonances, from a hint of a mouth to whistling throats.
  - **Ensemble** is how far the voices' pitch (up to about 20 cents) and size wander.
  - **Breath** is air in the voices, following the tail's level.

**Size** scales every line, 25 % to 200 %. Moving it bends the pitch of what's in the tank.
**Pre Delay** is up to half a second. **Diffusion** turns down the input diffusers, so the first
echoes come through. **Mod** is how far the tank's allpasses wander; 50 % is the mode as voiced.
**Damp** darkens the tail as it goes round. **Color** tilts the wet signal: left is a lowpass
coming down, right a highpass going up. **Width** is the wet signal's stereo width, 0 to 200 %.
**Mix** is equal-power; for a send, 100 %. **Level** is the output.

**Freeze** (the sanctuary lamp) shuts the input and holds the tail as it is, undamped. The limiter catches the output
at -0.5 dBFS.

Shift for fine, double-click to reset, scroll to nudge.

## Layout

`plugin/src/dsp`: `delay.rs` is the delay line and allpass, `filters.rs` the small filters,
`modes.rs` the tank's layout and the six modes, `tank.rs` the tank, `choir.rs` the singers, `engine.rs` everything
around it, `limiter.rs` the output guard. `plugin/src/editor/panel.rs` is the panel, "Fresco": a
cracked plaster wall in the apse of an abandoned church, Time inside a worn gilded halo whose rays
fill in as the tail grows, Freeze a sanctuary lamp. `paint.rs` has its colours, faces and marks.
`tools/build_fonts.py` cut its fonts.

`cargo run --release --example measure [time]` prints each mode's decay time, level and
stereo correlation; `--example choir_levels` the Choir's level across Resonance and Vowel;
`--example cpu` the cost of the heaviest settings; `--example render` writes WAVs (see its
header for the settings it takes).

Fonts: Cinzel, Cormorant Garamond and Spline Sans Mono, under the SIL Open Font License
(`plugin/assets/fonts`).
