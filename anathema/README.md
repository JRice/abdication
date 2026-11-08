# Anathema

A personal CLAP/VST3 effect after a circuit-bent Yamaha R100: an early digital reverb with its clock in your hand and wires across its memory bus. "Anathema" is a working name. How it was worked out is in [docs/measurements.md](docs/measurements.md).

Install, from this folder, with any project using it closed:

```
powershell -ExecutionPolicy Bypass -File plugin\install.ps1
```

## Controls

**Reclock** runs the converters, the RAM and the program, from 180 Hz to 44 kHz (31.25 kHz is stock, marked on the slider). Below stock, everything gets longer and darker and its images come through the fixed output filter. Moving it bends the pitch of whatever's in memory. Drag the numerals or the slider.

**Programs:** Hall, Club, Plate, Early, Echo, Reverse. Changing program keeps the memory, so the new one plays through the old one's leftovers.

**RAM Loop** stops writes, so the program reads the same memory over and over, one trip around every 16,384 ticks. Move Reclock while it's on. It's non-destructive unless Rot is up.

**Matrix:** sixteen switches onto one common wire. One alone does nothing; two or more short those lines together.
- Tones: fast address lines, and the last one is the sign bit, which takes off.
- Tremolo: middling address lines.
- Rhythm: slow address lines, whose speed follows the clock.
- Crunch: data lines, hitting both bytes of each word.

Pairs that cross address and data lines can feed themselves and sing with no input. **They can be loud:** the limiter catches them at -0.5 dBFS, so start with Level down.

**Mix** is inside the machine, so dry goes through the converters too and gets crushed at low clocks. **Direct** adds clean signal around the machine, for keeping the dry intact. For a send, use Mix 100 % and Direct 0.

**Decay, Damp, Pre Delay** are what they say, in ticks of the clock. **Drive** hits the converter, which clips hard. **Whine** adds the clock's own tone (off by default). **Rot** makes rows of cells lose their charge, worse at slow clocks, and it is destructive. **Contention** lets a shorted line win against a low one at random, which adds grain and wakes up pairs that would stay silent. **Level** is the output.

Shift for fine, double-click to reset, scroll to nudge.

## Layout

`plugin/src/dsp`: `ram.rs` is the memory and bus faults, `program.rs` the programs, `machine.rs` runs them in 16-bit integers, `analog.rs` the filters, `engine.rs` the clock and converters, `limiter.rs` the output guard. `plugin/src/editor/panel.rs` is the panel, a working one until it gets a design.

Fonts: Big Shoulders Display and Spline Sans Mono, under the SIL Open Font License (`plugin/assets/fonts`).
