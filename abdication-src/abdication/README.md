# Abdication

A personal synthesizer (by Abdicant) built from the qualities of favorite
instruments rather than clones of any one. Not for sale, not for support.

## Shape of the project (current)

One instrument, four oscillator slots plus a transient (decided Oct 2026):

| Slot | Job |
|---|---|
| Fundamental | basic wave: sine, triangle, saw (PolyBLEP), PWM (PolyBLEP, triangle LFO); single copy, never unison'd |
| Dual | a pair of basic waves (sin/sin … saw/sin, 9 pairs) via a Cobalt8-style algorithm: Ring Mod, FM, Window Amp Sync, Fractal, Sync, Spread (JP-8000-style supersaw), PWM (pasted halves, pinched), Folding (A/B macros); live scope on the panel |
| Wavetable | Start, Mid and End each pick one of 221 library waves (their screens show them through the OscMod); the arrows by the panel's title move all three together, one wave along; the position is hard-wired: Start through the attack, to Mid over the decay (filter envelope's timing and curve), then a triangle LFO (LFO Rate, 0.1–10 s) swings Mid ↔ End; MW/AT offset it. Band-limited per octave. OscMod (Argon8-style) + Depth: Bits 4, SR 11k/8k/4k, Simplify 6k/1.5k/800, Rectify, Tri/Sine Fold, Squish, Ring, Mirror, Sign Swap |
| Noise | white, clocked, LFSR long/short, crackle; tilt EQ (±24 dB around 2 kHz); Move wanders its level by up to ±18 dB, differently in each voice, at about Rate times a second; Mute silences it without changing the other sources' level; bypasses the drive |
| Transient | click, burst, zap, metal, chiff, or one of 111 recorded samples (Jade, Mech) with Sample Pitch; own level, pre- or post-filter |
| Unison | the Uni button cycles Off, U5, U7: 5 or 7 copies of Dual + Wavetable (only the copies that sound are computed), center-weighted detune (0–50 cents, skewed to small values; the outer pair's offset in both), level bell 0/−3/−6/−12 dB (U5: spread ±0.4, ±1 at 0/−3/−9 dB), random phase per note. Parameter "Unison: Mode" (was the On switch, so older projects open with it off) |
| Mod Env / Mod LFO | two free modulators per voice: an ADSR with curve, and an LFO playing any library wave (Free: 0.02–20 Hz, restarted per note; Sync: 4 bars … 1/32 with dotted and triplet values, locked to the host's bar position). Almost every knob and mix fader takes a send from each, −1..1 of its own travel (rings on the panel: Env inside, LFO outside; bars beside faders). Not sends: menus, switches, the envelopes' faders, Perform. A mix fader at exactly 0 mutes its source whatever its sends |
| Perform | mod wheel and aftertouch routes (24 destinations, ±100 %; two of them scale every Env or every LFO send); pitch-bend mode: ±2 st, ±1 oct, cutoff, resonance or bandpass ±50 % |
| Voicing | 8-voice poly; Portamento above Off = mono (one voice, last-note priority, SH-101-style auto glide: only overlapping notes glide, without retriggering; detached notes start on pitch), glide 30–300 ms |
| Vintage | one knob of per-voice analogue tolerance (after Sequential's Vintage and Diva's trimmers): stable per-voice offsets to tuning (±8 c), cutoff (±¼ oct), envelope A/D/R (±25 %), pulse width and LFO rates (±15 %), slow per-voice pitch drift; a low-passed noise on each voice's pitch (3 cents RMS at the top, below 60 Hz) and another on its level (0.7 dB RMS, below 25 Hz), set by constants in voice.lib; wow and flutter grow with the square of the knob (Boards of Canada at the top) |
| Events | per voice, brief random bumps in the spectrum, starting on the voice's harmonics up to the 64th (texture.lib): Density (share of the time each of six slots is in an event), Depth (bump height, 0 = off), Life (1–3), Sharp (narrower bumps, toward a vocoder sound at high Depth and Density). Gain and frequency are random walks whose amount wanders by itself; a slow leveler holds the average level. They act on the voice's output, including noise and transients. Mute crossfades to the dry voice and keeps the knobs |
| Calls | per voice, phrased whistles that sweep a little, one way, between harmonics of the note (texture.lib): Level (bottom = off), Activity, Rate. Faded with the voice's amp envelope, then through a shared bus (plugin-callbus.dsp): a delay, Echo (level and feedback together, to −6 dB and 90 %), into a reverb. The bus is their only way out, and nothing else goes through it, so this is the plugin's only stereo. Mute silences the calls and their tail, which doesn't return on unmuting |
| Dice | the die in the top bar rolls a patch (dice.rs): it picks a kind of sound (pad, drone, keys, pluck, bass, lead; weighted toward pads), which sets the ranges for the envelope and filter, then follows rules: transients only under fast attacks, oscillators transposed rarely and mostly by a fourth or fifth (never when alone), three related waves, unused sources at 0, a few modest sends. Perform, Volume, Portamento and the Mutes stay as they are. The ‹ beside it steps back through the last 32 patches until the editor closes |
| Output | stereo peak limiter, −1.5 dB ceiling, 50 ms release (plugin only) |

## CPU

Faust's Rust output computes every branch of a switch, so the DSPs choose
with `enable()` instead, through core.lib's `only` and `choose`: only the selected Dual
algorithm, OscMod, Fundamental wave, noise type, transient type and drive
curve are computed. Also skipped while they can't be heard: a source whose
mix fader is at 0, the transient once it has faded, Vintage's noises at 0,
a send's arithmetic while both its rings are at 0, the second filter stage
in 2-pole mode, Events at Depth 0 or muted, Calls at Level off, and in
Events, each slot's random draws except when a gap starts or a walk steps. The output
is bit-identical (a test renders patches to compare builds). Measured with
`cargo test --release cpu_worst_case -- --ignored --nocapture` (8 voices,
Dual and Wavetable up; ABD_UNISON=0 without unison, 5 for U5) and, per part,
`profile_parts` in voice.rs (ABD_SET="id=value,..." sets controls).

Caveat: a disabled `enable()` outputs the last value it computed, not 0
(and freezes the state inside it). So never call `enable` directly: `only`
zeroes the output while disabled. Calling it directly left DC offsets after
a switch or fader change (test `switching_leaves_no_offset`), and nested
enables froze Events' hold and walker state.

## Settings (☰)

The ☰ button at the top left opens under-the-hood settings. They're saved
with the patch and the dice leave them alone.

- **Drive Grit**: the drive's built-in random bias (500 Hz jitter), which
  adds a noise floor about 17 dB under the signal even with the drive at 0.
  Default 1 (the original sound); 0 skips it.
- **Vintage Noise**: how much of Vintage's noise wobble (pitch and level)
  comes through. It only applies while Vintage is up.

## Earlier plan (superseded)

One shared **core** library, then several instruments split by oscillator
engine:

- **Digital-waveform synth** — Argon8-style wavetables + MASK1-style
  bitmasks (a bitmask is a procedurally generated single-cycle wave), with
  optional PPG-style grit that can worsen as pitch drops.
- **VA** — genuinely dual-phase sub-oscillator algorithms with curated
  macros, Cobalt8-style.
- **Layering synth** (open question) — four static sources, short attack
  transients, coherent amplitude modulation via the normalized mixer.
- **Percussion** — separate project on the same core.

Interface model: MiniFreak expanded view — per module one type selector,
a live display, ~4 knobs, depth in a pop-out. Hardware control through
per-controller mapping profiles (Cobalt8X → VA, Argon8M → wavetable).

## Layout

```
dsp/
  core.lib          shared DSP blocks, no UI
  engines.lib       oscillator engines + transient, no UI
  voice.lib         the voice: core blocks + controls, as the parts the
                    plugin runs (the voice, a unison copy of Dual + Wavetable)
  mod.lib           the voice's free modulators, Mod Env and Mod LFO
  prototype.dsp     playable 8-voice instrument (MIDI)
  render-test.dsp   headless smoke test (repeating note)
  plugin-voice.dsp  one mono voice without its Dual + Wavetable (plugin build)
  plugin-osc.dsp    one unison copy of a voice's Dual + Wavetable (plugin build;
                    7 per voice with Unison on, the centre one with it off)
  plugin-mod.dsp    one voice's free modulators (plugin build)
  plugin-texture.dsp  one voice's events and calls, on its output (plugin
                    build; skipped while both are off)
  plugin-callbus.dsp  the calls' echo and reverb, one for the instrument
  texture.lib       events, calls, echo: no controls, no panel
                    Several DSPs, not one: with the modulators' sends on every
                    control, one DSP took the Faust compiler tens of minutes
                    and more memory than the machine had; split, ~20 s
  wavelib.lib       GENERATED: wave-library layout, menus, and the reader
                    (samples come from the plugin via wt_sample())
plugin/             Rust CLAP/VST3 plugin (nih-plug)
  src/dsp.rs, osc_dsp.rs, mod_dsp.rs, texture_dsp.rs, callbus_dsp.rs
                    generated from the Faust DSPs (don't edit)
  src/voice.rs      one voice: modulators -> unison copies -> the voice -> texture
  src/engine.rs     8-voice polyphony, note allocation, pitch bend
  src/params.rs     host parameters built from the Faust controls
  src/wavelib.rs    serves the embedded wave library (wavelib.bin, generated),
                    including band-limited reads of the basic shapes
  src/scope.rs      scope traces: offline renders of one voice, and the
                    live buffer the audio thread fills for the panel (the
                    newest voice's Dual and Wavetable outputs, and its
                    moving values: pulse width, filter, Mod Env/LFO)
  src/translib.rs   serves the embedded transient samples (translib.bin, generated)
  src/dice.rs       rolls a random, playable patch (no UI, so it's tested alone)
  src/editor/       the front panel: layout.rs (where everything goes),
                    draw.rs (vector painting), screens.rs (the amber
                    displays), panel.rs (mouse handling)
  src/bin/          standalone app for trying the editor (feature "standalone")
  assets/fonts/     Barlow Condensed + IBM Plex Mono (SIL OFL), embedded
  src/lib.rs        the plugin wrapper
  install.ps1       Windows: build + install for the current user
docs/
  cobalt8x-mapping.md  Cobalt8X -> Abdication map, and how to set up the script
bitwig/
  Cobalt8X-Abdication.control.js  Bitwig controller script: the Cobalt8X's
                    knobs drive Abdication (no build; copy into Controller Scripts)
tools/
  bundle.mjs        inlines local .lib files into one self-contained .dsp
  faust2lang.mjs    WebAssembly compiler front end (wasm/cmajor backends only)
  gen-rust.sh       regenerates the generated DSP files (needs native Faust; ~20 s)
  gen-wavelib.py    tables/*.wav (+ the four basic shapes) ->
                    plugin/src/wavelib.bin, wavelib_layout.rs, dsp/wavelib.lib
  gen-translib.py   tables/transients/ -> plugin/src/translib.bin + dsp/translib.lib
  make-bitmask-sets.py  renders the bitmask sets from the old oscillator's rules
  preview-set.py    contact sheet + audition WAV for a wave set
  chop-oneshots.py  splits a recording of one-shots into trimmed, normalized samples
  wavetools.py      shared wave-set helpers
tables/             wave library sources (2048-sample float32 frames,
                    Serum-style 'clm ' chunk; .json = provenance)
  argon8-c2.wav     118 waves from Jeremy's Argon8   (menu names "A8 nnn")
  bitmask-8.wav      52 bitmask waves, 8 slices      ("M8 nn")
  bitmask-16.wav     47 bitmask waves, 16 slices     ("M16 nn")
                    + Sine, Triangle, Saw, Square (generated; the Dual's waves)
  transients/       one-shot transients (float32 mono 44.1 kHz, -1 dBFS
                    peaks, silence trimmed; <set>.json = provenance), in the
                    Transient Type menu after the synthesized types:
    jade/           95 hits ("Jade nn"; two near-duplicates removed)
    mech/           16 hits ("Mech nn")
build/              generated bundles (not source)
```

## What's in the core so far

| Section | Blocks |
|---|---|
| Noise & chaos | white, **clocked** (keytrackable sample-and-hold + bit depth), LFSR long/short (8-bit chip style), procedural crackle |
| Mixer | `mix4` — normalized four-source balance (equal power in the voice) |
| Drive | Sub 37-style multidrive on one knob: soft → hard clip → tube → fold, a quarter of the travel each (curves crossfade near the boundaries), gain gentle through the lower half and steep at the top (to 24 dB); its curve is drawn beside the knob; level-compensated; fixed audio-rate bias chaos (500 Hz, 0.2) |
| Filters | own TPT state-variable filter, saturating in-loop (character always full), with LP→notch→HP morph + bandpass blend, 2/4-pole; utility HPF; MS-20-style resonant HPF |
| Envelopes | segment ADSR at audio rate: exact stage times, one Curve knob, click-free retriggers. Voice uses one envelope for filter + amp, with XY offsets raising amp sustain/decay |
| Modulation | basic LFO; shape modulator (sin, cos, tri, saw, ramp, log, exp; 1 ms–10 s; one-shot or loop; restarts per note); smoothed random jitter; tilt EQ. The voice's free modulators and sends live in voice.lib |

## Playing it

**Note:** the wave library is now served by the plugin (Faust reads it via
an external function), so the web IDE can no longer run the full voice. Play the plugin instead; the
IDE route is kept for small DSP experiments.

The Faust web IDE (<https://faustide.grame.fr>) can't read sibling
library files, so paste the bundle instead:

```bash
cd tools && npm install            # once: Faust compiler (WebAssembly)
cd .. && node tools/bundle.mjs dsp/prototype.dsp build/prototype.bundle.dsp
```

Open `build/prototype.bundle.dsp` in the IDE, switch to **Poly** mode,
and play from a MIDI keyboard or the on-screen keys.

To find the MiniFreak "sand": Noise type **Clocked**, Clock Ratio **-1**,
Bits around 4–8, and bring up the Noise fader.

## Building the plugin (Windows)

Needs Rust (rustup, MSVC toolchain) and Visual Studio Build Tools with
"Desktop development with C++". No Faust install: the generated Rust is
checked in.

```powershell
powershell -ExecutionPolicy Bypass -File plugin\install.ps1
```

That builds `Abdication.clap` and `Abdication.vst3` and copies them to
`%LOCALAPPDATA%\Programs\Common\CLAP` and `...\VST3`. Rescan in the DAW;
if they don't show up, add those folders to its plugin locations.

Eight voices, MIDI pitch bend (mode set on the Perform panel), master
Volume. Parameter names carry their panel ("Mix: Noise", "Transient:
Level"), so the host's lists and automation lanes stay readable.

**The panel** (fixed at 1280 x 800): drag knobs and faders up/down (Shift
for fine), mouse wheel for small steps, double-click to reset. ‹ › step
through a list; click its name (or any ▾ button) for the whole list. The
top readout shows the control under the mouse. The screens are drawn
from the settings, except two oscilloscopes of the real oscillators: the
Dual's (two cycles, rendered offline at the last note's pitch whenever a
Dual setting changes) and the Wavetable's (one cycle, live from the newest
voice at ~15 Hz while a note plays; an offline render otherwise).

**Changing the sound:** edit the `.dsp`/`.lib` files, run
`tools/gen-rust.sh` (Faust 2.70+), rebuild. New or renamed controls appear
as parameters automatically; renaming one resets it in saved projects.

```bash
cd plugin && cargo test     # engine + parameter tests
cargo test --release -- --ignored --nocapture cpu   # worst-case CPU (8 voices, unison)
```

## Rendering offline

```bash
node tools/bundle.mjs dsp/render-test.dsp build/render-test.bundle.dsp
node tools/node_modules/@grame/faustwasm/scripts/faust2sndfile.js \
     build/render-test.bundle.dsp build/test.wav -c 192000 -sr 48000 -bd 24
```

## Known next steps

- Envelope slider ranges.
- Oversampling for the drive shaper.
- Center/spread dual-filter mode; resonant HPF wired into the voice.
- Band-limit / oversample the sync, window and bitmask waves (they alias).
- Curate the library (drop weak waves, maybe reorder) and pick better
  default Start/Mid/End waves.
- More base waves for the dual algorithms (currently triangle only).
- Panel: the wave-browser pop-out (wave thumbnails, audition), typed
  value entry, modulation shown on the knobs, a MIDI activity light.

Deferred experiments: hosting third-party effects before the filter
(collides with polyphony).
