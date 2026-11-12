# Abdication

A personal synthesizer (by Abdicant) built from the qualities of favorite
instruments rather than clones of any one. Not for sale, not for support.

## Shape of the project (current)

One instrument: two oscillator slots, each a Dual or a Wavetable, plus noise and a transient (Nov 2026; the Fundamental is gone, a Dual's VA Sweep covering its waves):

| Slot | Job |
|---|---|
| Osc 1, Osc 2 | each slot plays a Dual, a Wavetable or a Ray: the picker in its title (beside "Osc 1") switches it; the other engines keep their settings, hidden and not computed. Each has a fader and an Octave knob (±2) on the Mix panel. Out of the box Osc 1 is a Dual (VA Sweep at A 0.5: a plain saw) and Osc 2 a Wavetable at 0 |
| Dual | two basic waves, each picked on its own (Sine, Triangle, Saw, Square: all 16 pairs), via one of 12 Cobalt8-style algorithms, in the Cobalt8's order: VA Sweep, Spread (four phases, centre always present), PWM (pasted halves, pinched), Sync, Fractal, Window Amp Sync, Metal, Ring Mod, FM (true through-zero FM), Noise AM, Chaos, Folding (A/B macros). Built the Cobalt8's way (character/cobalt8.md): own phase per oscillator, band-limited table reads for the smooth runs, every edge an algorithm makes (sync reset, pinch, wave switch) corrected at its time within the sample (polyBLEP/polyBLAMP), FM splitting the sample at a square modulator's edge, ADAA folders; one sample of latency; live scope on the panel |
| Wavetable | four waves, 2 × 2, each one of 221 library waves (their screens show them through the OscMod); the arrows by All Waves move all four together, one wave along; Position (modulatable) morphs 1 → 2 → 3 → 4 across its travel. Band-limited per octave. OscMod (Argon8-style) + Depth: Bits 4, SR 11k/8k/4k, Simplify 6k/1.5k/800, Rectify, Tri/Sine Fold, Squish, Ring, Mirror, Sign Swap |
| Ray | Raycaster's oscillator, ported (ray/: its geometry, oscillator, shapes and outlines, unchanged): a shape turns once per cycle about a center, and a ray dropped from above reads how far it falls before it hits an edge; leaps (smoothed with a polyBLEP placed by bisecting the turn) are where the brightness comes from, and the wave is centred over its last period. Element (a pick list under the scope): Earth (O), Water (snowman), Air (square), Fire (M), Wood (crescent), Metal (S), or Chaos (three of them, bent, sized and placed at random, overlapped; every pick of Chaos rolls a new one, its seed saved with the patch but not shown). Eight knobs, all modulatable: Squish, Skew, Tilt, Center, Ray (how far right of the center the ray falls), Orbit and Rate (the center circles, to 1 kHz), Fine (±100 cents). Free (header): each note starts the shape and its orbit anywhere; off, upright, or at the unison copy's phase. Played in Rust per unison copy (ray/player.rs): the Faust copy gives each slot's pitch (so glide, bends, Global Pitch, Vintage and unison detune all apply), the copy's level and start phase, and note-on; the slot's controls are read from the copy's DSP as the engine set them for the voice. Its raw wave times 2 (`RAY_GAIN`) puts an off-centre Fire near a saw's level; a centred Earth is about 20 dB quieter (Raycaster has no loudness normalization). Not oversampled. CPU here (64-sample block of one voice, U7): 112 µs with a Dual, 136 µs with a Ray, 262 µs with a Chaos Ray orbiting (finding the shape's reach every 4 samples is the cost); a voice is 1333 µs of audio |
| Noise | white, clocked, LFSR long/short, crackle; tilt EQ (±24 dB around 2 kHz); Move wanders its level by up to ±18 dB, differently in each voice, at about Rate times a second; Mute silences it without changing the other sources' level; bypasses the drive |
| Transient | click, burst, zap, metal, chiff, or one of 111 recorded samples (Jade, Mech) with Pitch (±24 semitones, modulatable: a sample's speed, or a synthesized one's frequencies); own level, pre- or post-filter |
| Unison | the Uni button cycles Off, U5, U7: 5 or 7 copies of both oscillators (only the copies that sound are computed), center-weighted detune (0–50 cents, skewed to small values; the outer pair's offset in both), level bell 0/−3/−6/−12 dB (U5: spread ±0.4, ±1 at 0/−3/−9 dB), random phase per note. Parameter "Unison: Mode" (was the On switch, so older projects open with it off) |
| Modulations | every continuous knob and mix fader (50, Global Pitch and the Rays' eight among them; not Volume, Portamento, the Amp envelope, the stepped knobs or the settings) has its own modulators, per voice, after Raycaster's (modulation.rs): an LFO (sine, triangle, saw, ramp, square, noise, brown, S&H; 0.01–50 Hz, or synced 4 bars … 1/32 with dotted and triplets; Retrig restarts it per note, off it runs free, and synced it locks to the song), an ADSR envelope (Attack, Decay and Release each bent by a curve button, as the Amp's; retriggers from where it is), and how far velocity, aftertouch (channel or poly), the mod wheel and the key move it, each −100..100 % of the knob's own travel, curved so small amounts get more of the knob (a quarter turn from the middle is 25 %); each of those four can instead move the LFO's or the envelope's amount (the dropdown under it: the mod wheel deepening a vibrato). The Cutoff's key counts in octaves per octave (50 % by default, with its envelope +21 %, S 30 %: the old filter envelope and keytrack); Global Pitch starts with a 5 Hz LFO at 0 that the mod wheel deepens to 25 cents (+1.0 % of its ±1 octave). The engine runs them once a block (64 samples) and sets the result per voice; the voice draws a straight line to each value over the block and jumps there on note-on (voice.lib's `ramped`). Click a knob or fader to select it: a ring of dots marks it, and the Modulations panel shows and sets its modulators (Copy and Paste carry them from one knob to another, Paste greyed until something's copied; Clear zeroes its amounts). The Perform panel lists every modulation that's set (click a line to select it); while a note plays, a modulated control shows where the newest voice has it (a pink arc). A mix fader, Events Depth or Calls Level at its bottom stays off whatever moves it. Measured: a 1 ms filter pluck rendered in 64-sample blocks differs from one with the modulators run every sample by −38 dB over its first 50 ms and −78 dB after, with nothing at the block rate (−107 dB); a 50 Hz LFO by −27 dB (the lines bending its sine). CPU: all of them modulated on 8 voices costs about 7 % of a core |
| Amp | the amp envelope (Attack, Decay, Sustain, Release), not modulatable; the filter's envelope is the Cutoff's modulator. Attack, Decay and Release each have a curve button at their fader's top right (Raycaster's): a tiny picture of the segment's bow; drag it up or down to bend the segment (−1..1 as it looks on the graph, + bowed up), and while you drag, the curve shows large beside it; double-click resets. Decay and Release default to −0.6, the old single Curve at 0.5 (which they replace); Attack to +0.6, sharper (the knobs' envelopes too) |
| Perform | Global Pitch (every voice, ±1 octave; modulatable, and out of the box vibrato on the mod wheel); pitch-bend mode: ±2 st, ±1 oct, cutoff, resonance or bandpass ±50 %; the list of every modulation that's set |
| Voicing | 8-voice poly; Portamento above Off = mono (one voice, last-note priority, SH-101-style auto glide: only overlapping notes glide, without retriggering; detached notes start on pitch), glide 20–200 ms |
| Vintage | one knob of per-voice analogue tolerance (after Sequential's Vintage and Diva's trimmers): stable per-voice offsets to tuning (±8 c), cutoff (±¼ oct), envelope A/D/R (±25 %), correlated as on GForce's OB-X (two identity values per voice, mixed into each offset with its own signs, so a sharp voice tends to be brighter and quicker; weights `traitW1`/`traitW2` in voice.lib), slow per-voice pitch drift; a low-passed noise on each voice's pitch (3 cents RMS at the top, below 60 Hz) and another on its level (0.7 dB RMS, below 25 Hz), set by constants in voice.lib; wow and flutter grow with the square of the knob (Boards of Canada at the top) |
| Events | per voice, brief random bumps in the spectrum, starting on the voice's harmonics up to the 64th (texture.lib): Density (share of the time each of six slots is in an event), Depth (bump height, 0 = off), Life (1–3), Sharp (narrower bumps, toward a vocoder sound at high Depth and Density). Gain and frequency are random walks whose amount wanders by itself; a slow leveler holds the average level. They act on the voice's output, including noise and transients. Mute crossfades to the dry voice and keeps the knobs |
| Calls | per voice, phrased whistles that sweep a little, one way, between harmonics of the note (texture.lib): Level (bottom = off), Activity, Rate. Faded with the voice's amp envelope, then through a shared bus (plugin-callbus.dsp): a delay, Echo (level and feedback together, to −6 dB and 90 %), into a reverb. The bus is their only way out, and nothing else goes through it, so this is the plugin's only stereo. Mute silences the calls and their tail, which doesn't return on unmuting |
| Dice | the die in the top bar rolls a patch (dice.rs): it picks a kind of sound (pad, drone, keys, pluck, bass, lead; weighted toward pads), which sets the ranges for the envelope and filter, then follows rules: transients only under fast attacks, Osc 1 always sounding and Osc 2 often, each slot a Dual or a Wavetable with both engines rolled, Osc 2 transposed rarely and mostly by a fourth or fifth (never when silent), four related waves and usually a slow sweep of a sounding wavetable's Position, unused sources at 0, the Cutoff's envelope and key tracking, and a few other modest modulators. Perform (with Global Pitch's modulators), Volume, Portamento and the Mutes stay as they are. The ‹ beside it steps back through the last 32 patches until the editor closes |
| Output | stereo peak limiter, −1.5 dB ceiling, 50 ms release (plugin only) |

## CPU

Faust's Rust output computes every branch of a switch, so the DSPs choose
with `enable()` instead, through core.lib's `only` and `choose`: only each slot's engine,
the selected Dual algorithm, OscMod, noise type, transient type and drive
curve are computed. Also skipped while they can't be heard: a source whose
mix fader is at 0, the transient once it has faded, Vintage's noises at 0,
a modulator's LFO and envelope while their amounts are 0, the second filter stage
in 2-pole mode, Events at Depth 0 or muted, Calls at Level off, and in
Events, each slot's random draws except when a gap starts or a walk steps. The output
is bit-identical (a test renders patches to compare builds). Measured with
`cargo test --release cpu_worst_case -- --ignored --nocapture` (8 voices,
Osc 1 a Dual and Osc 2 a Wavetable, both up; ABD_OSC2=0 makes both Duals, ABD_OSC1=1
both Wavetables; ABD_UNISON=0 without unison, 5 for U5; ABD_MODS=1 runs
every knob's modulators) and, per part,
`profile_parts` in voice.rs (ABD_SET="id=value,..." sets controls).

Caveat: a disabled `enable()` outputs the last value it computed, not 0
(and freezes the state inside it). So never call `enable` directly: `only`
zeroes the output while disabled. Calling it directly left DC offsets after
a switch or fader change (test `switching_leaves_no_offset`), and nested
enables froze Events' hold and walker state.

Modulator parameters (22 per modulatable knob) are hidden from the host's
parameter list and not automatable: they're set on the panel and saved with
the patch.

## Settings (☰)

The ☰ button at the top left opens under-the-hood settings. They're saved
with the patch and the dice leave them alone.

- **Drive Grit**: the drive's built-in random bias (500 Hz jitter), which
  adds a noise floor about 17 dB under the signal even with the drive at 0.
  Default 1 (the original sound); 0 skips it.
- **Vintage Noise**: how much of Vintage's noise wobble (pitch and level)
  comes through. It only applies while Vintage is up.
- **Heat** (0–2, default 1): how hard the voice drives the mixer's saturation
  and the soft clip after the filter (after Shibboleth's path).
  1 is Shibboleth's level for oscillators at full; 0 is the clean path from
  before (bit for bit, within float rounding). At 1 the saturation's part
  of a full-level voice sits about 14–19 dB under it, at about the same
  loudness (the makeup), with peaks 1–2 dB lower.

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
                    plugin runs (the voice, a unison copy of the two oscillators)
  prototype.dsp     playable 8-voice instrument (MIDI)
  render-test.dsp   headless smoke test (repeating note)
  plugin-voice.dsp  one mono voice without its oscillators (plugin build)
  plugin-osc.dsp    one unison copy of a voice's two oscillators (plugin build;
                    7 per voice with Unison on, the centre one with it off)
  plugin-texture.dsp  one voice's events and calls, on its output (plugin
                    build; skipped while both are off)
  plugin-callbus.dsp  the calls' echo and reverb, one for the instrument
  texture.lib       events, calls, echo: no controls, no panel
                    Several DSPs, not one: as one, the Faust compiler took
                    tens of minutes and more memory than the machine had
  wavelib.lib       GENERATED: wave-library layout, menus, and the reader
                    (samples come from the plugin via wt_sample())
plugin/             Rust CLAP/VST3 plugin (nih-plug)
  src/dsp.rs, osc_dsp.rs, texture_dsp.rs, callbus_dsp.rs
                    generated from the Faust DSPs (don't edit)
  src/modulation.rs every knob's modulators: the targets, LFO, ADSR, and how
                    they move a knob in its travel
  src/voice.rs      one voice: unison copies -> the voice -> texture
  src/engine.rs     8-voice polyphony, note allocation, pitch bend, and the
                    modulators, run per voice once a block
  src/params.rs     host parameters built from the Faust controls, and each
                    knob's modulator parameters ("mod.<knob>.<field>")
  src/wavelib.rs    serves the embedded wave library (wavelib.bin, generated),
                    including band-limited reads of the basic shapes
  src/scope.rs      scope traces: offline renders of one voice, and the
                    live buffer the audio thread fills for the panel (the
                    newest voice's Osc 1 and Osc 2 outputs, and its
                    moving values: filter, and where its
                    modulators have each knob)
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
| Mixer | `mix4` — normalized four-source balance (equal power in the voice). The oscillators meet a saturation stage at their faders' own levels, before the balance scales them (`mixerHeat`, after Shibboleth's mixer): 60 % of the signal through one biased tanh, 40 % split at 120 Hz and 4 kHz into lows, mids and highs, each through its own curve (drives 1.5 / 1.25 / 0.9, biases +0.12 / −0.08 / +0.05; the whole at 1.0, +0.08), each curve moved so silence stays at 0 and quiet signals pass at unity, then a 5 Hz DC blocker. So Osc 2 up drives it harder, and detuned oscillators beat in and out of it. Not oversampled |
| Drive | Sub 37-style multidrive on one knob: soft → hard clip → tube → fold, a quarter of the travel each (curves crossfade near the boundaries), gain gentle through the lower half and steep at the top (to 24 dB); its curve is drawn beside the knob; level-compensated; fixed audio-rate bias chaos (500 Hz, 0.2) |
| Filters | own TPT state-variable filter, saturating in-loop (character always full) through a lopsided tanh (even harmonics, after Model 84/82's asymmetric state curves; `svfAsym`), resonance eased below 750 Hz (after GForce's OB-X; `svfLowQHz`, `svfLowQEase`), with LP→notch→HP morph + bandpass blend, 2/4-pole; or (Type) Raycaster's 4-pole lowpasses, ported from its filters.rs (`zdf4`, matching it to float precision): a transistor ladder (every stage saturating on its input and output) or the SH-101's (OTA stages saturating on their difference, resonance through soft-clipping diodes, half of it added back at the output so it keeps more bass), both zero-delay feedback with each stage's tanh linearized at its last input, self-oscillating at the top of Resonance (its 0.5..6 maps to their 0..1); Morph, Bandpass and 4-Pole grey out for them. After the filter, a symmetric soft clip (`postClip`, tanh(1.25 x), after Shibboleth's) that a resonant peak runs into whatever the amp envelope does, then 2 dB of makeup per unit of Heat. Utility HPF; MS-20-style resonant HPF |
| Envelopes | segment ADSR at audio rate: exact stage times, a bow per segment (`bowedAdsr`, Raycaster's curve: (1 − e^(−5 b p)) / (1 − e^(−5 b))), click-free retriggers. The voice's is the Amp's; the knobs' envelopes (modulation.rs) bend the same way |
| Modulation | basic LFO; shape modulator (sin, cos, tri, saw, ramp, log, exp; 1 ms–10 s; one-shot or loop; restarts per note); smoothed random jitter; tilt EQ. The knobs' modulators are the plugin's (modulation.rs) |

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
from the settings, except the oscilloscopes of the real oscillators: a
Dual's (two cycles, rendered offline at the last note's pitch whenever a
Dual setting changes) and a Wavetable's (one cycle, live from the newest
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
  default waves.
- Panel: the wave-browser pop-out (wave thumbnails, audition), typed
  value entry, modulation shown on the knobs, a MIDI activity light.

Deferred experiments: hosting third-party effects before the filter
(collides with polyphony).
