# Arecibo

A personal CLAP/VST3 effect: a no-input mixer with a shortwave link in its feedback loop. Nothing has to go in for something to come out, and anything that does go in becomes a disturbance in what's already happening. "Arecibo" is a working name. How it works is in [docs/design.md](docs/design.md).

Install, from this folder, with any project using it closed:

```
powershell -ExecutionPolicy Bypass -File plugin\install.ps1
```

Put it on a track with nothing on it, and it starts by itself. It's an audio effect with a stereo sidechain input, so in Bitwig it can listen to another track as well.

## Four voices

There are four stations, each fed by its own input: **A** from the main left, **B** from the main right, **C** from the sidechain left and **D** from the sidechain right. Choose the sidechain source in the device's sidechain selector. With nothing connected, C and D just run on feedback like the others.

Every voice still hears the others through Cross, so a signal on the sidechain changes the whole loop, not only its own two stations.

A hard hit overloads the front end of the receiver it reaches, as a strong signal nearby does on a real set. Its AGC drops, its station starves for the AGC time, and then it floods back. Quiet signals don't do this: they just get pulled in. So a kick on the main input and something else on the sidechain each knock out their own two voices, and hits on both at once can silence the machine until it recovers. **Input** and **Side In** set how hard they hit, and **AGC** how long it takes to come back.

## Where to start

Sweep **Sky** first. Near 0 it's all wire: a no-input mixer, harmonically dense and aggressive. Near 1 it's all radio: narrow, breathing, full of gaps. The in-between is where they fight.

Then **AGC**, which sets the breathing. Short keeps it talking steadily, and long makes it starve and flood, with silences. After that, **Feedback** and the strips' **Gain** set how far past the edge of oscillation it runs, and each strip's **Peak** sets the frequency that strip wants to sing at. The waterfall shows what you're doing.

## Controls

**Strips A, B, C and D**
- **Tune** is how far that receiver is mistuned. Everything it hears comes out moved by that many Hz, and goes back round the loop to be moved again, so small values matter most, and the knob gives them most of its travel.
- **Peak** is the strip's resonant EQ.
- **Gain** drives its saturating input stage.
- The meter by each letter is its S-meter. Empty means the receiver is hearing very little and straining.

**The loop**
- **Feedback** is how much comes back round.
- **Cross** runs from each channel feeding itself to channels feeding each other, with mixed polarity.
- **Sky** blends each channel's return between the direct wire and the radio.
- **Width** is the bandwidth of the radio path, from a Morse filter to wide open.
- **Distance** is how far away the far end is, as delay in the loop, out to once round the planet.

**The radio**
- **AGC** is how long the receiver holds its gain after a peak before letting go.
- **Weather** is the sky: fading speed and depth, a second path arriving late, static, crashes, a distant keyed carrier, falling whistlers from lightning, and very rarely something it said seconds ago coming back.

**Age** is what's gone wrong with it. A shared power supply sags under load, and its oscillators get pulled by that load and drift as the box warms, each at its own rate over minutes. Bad joints open more when it's shaking, the sideband filter leaks a mirror image, worn pots scratch when moved, and there's a little mains on everything. It's all gradual, and at 0 the machine is new.

**Input** is how hard the main input hits the loop and **Side In** the sidechain. **Dry** adds the main input to the output unchanged (never the sidechain), and **Level** goes into a limiter at -0.5 dBFS.

Shift for fine, double-click to reset, scroll to nudge.

## Layout

`plugin/src/dsp`: `station.rs` is one channel, strip and radio; `sky.rs` the fading, atmosphere and paths; `hilbert.rs` the sideband splitter; `wear.rs` the flaws; `network.rs` the patch and the long echoes; `engine.rs` the outside. `plugin/src/editor`: `waterfall.rs` is the display, `panel.rs` a working panel. `examples/render.rs` renders settings for measuring.

Fonts: Spline Sans Mono, under the SIL Open Font License (`plugin/assets/fonts`).
