# Abdication

A personal synthesizer (by Abdicant) built from the qualities of favorite
instruments rather than clones of any one. Not for sale, not for support.

## Shape of the project (current)

One instrument: two oscillator slots, each a Dual, a Wavetable, a Ray, an Orthogonal or a Windows, plus noise and a transient (Nov 2026; the Fundamental is gone, a Dual's VA Sweep covering its waves):

| Slot | Job |
|---|---|
| I, II (Osc 1, Osc 2) | each slot plays a Dual, a Wavetable, a Ray, an Orthogonal or a Windows: the picker in its title (beside "Osc 1") switches it; the other engines keep their settings, hidden and not computed. Balance (on the Mix plate) sets their levels, and each has a Pitch knob there (pitch.rs; modulatable): out from the middle, each way, up to 99 cents (continuous), then 1–7 semitones, then 1 and 2 octaves, in steps. It replaced the Octave switch and the Dual's Semi (Nov 2026). Out of the box I is a Dual (VA Sweep at A 0.5: a plain saw) and II a Wavetable, Balance all the way to I. The panel calls them I and II, and so do the host's names ("Dual I: A"); their ids keep 1 and 2 ("dual_1.a"). Both plates are the same size and lay each engine out alike (test `the_slots_match`). Each engine has presets (see Oscillator presets) |
| Dual | two basic waves, each picked on its own (Sine, Triangle, Saw, Square: all 16 pairs), via one of 12 Cobalt8-style algorithms, in the Cobalt8's order: VA Sweep, Spread (four phases, centre always present), PWM (pasted halves, pinched), Sync, Fractal, Window Amp Sync, Metal, Ring Mod, FM (true through-zero FM), Noise AM, Chaos, Folding (A/B macros). Built the Cobalt8's way (character/cobalt8.md): own phase per oscillator, band-limited table reads for the smooth runs, every edge an algorithm makes (sync reset, pinch, wave switch) corrected at its time within the sample (polyBLEP/polyBLAMP), FM splitting the sample at a square modulator's edge, ADAA folders; one sample of latency; live scope on the panel |
| Wavetable | five waves, each one of 135 library waves (the four basic ones first: Sine, Triangle, Saw, Square; then A8, M8, M16), on a path the note follows (Nov 2026): it starts on wave 1, morphs to 2 over envelope A's attack, to 3 over its decay, bounces 3 → 4 → 3 while it sustains, Speed round trips a second (0.02–20 Hz, modulatable; or Sync, 4/1 … 1/32 with dotted and triplets, the knob then stepping through them), and crossfades to 5 over the release, from wherever it was. Position is gone. The engine works the path out per voice once a block (modulation.rs `PathState`, from its own copy of envelope A, which follows the times and curves but not Vintage's variation of them) and sets each slot's `morph` (0..3) and `fade` (0..1), which the Faust voice ramps over the block; wave 5 is only read while the release fades to it. The panel shows the five waves as small screens along the top ([1] A [2] D [3] S [4] R [5]; click one for the wave picker), envelope A's outline under them with a dot where the newest voice is, and the All Waves arrows in the plate (they move all five, one wave along). Band-limited per octave. OscMod (Argon8-style) + Depth (modulatable): Bits 4, SR 11k/8k/4k, Simplify 6k/1.5k/800, Rectify, Tri/Sine Fold, Squish, Ring, Mirror, Sign Swap |
| Ray | Raycaster's oscillator, ported (ray/: its geometry, oscillator, shapes and outlines, unchanged): a shape turns once per cycle about a center, and a ray dropped from above reads how far it falls before it hits an edge; leaps (smoothed with a polyBLEP placed by bisecting the turn) are where the brightness comes from, and the wave is centred over its last period. Element (a pick list under the scope): Earth (O), Water (snowman), Air (square), Fire (M), Wood (crescent), Metal (S), or Chaos (three of them, bent, sized and placed at random, overlapped; every pick of Chaos rolls a new one, its seed saved with the patch but not shown). Eight knobs, all modulatable: Squish, Skew, Tilt, Center, Ray (how far right of the center the ray falls), Orbit and Rate (the center circles, to 1 kHz), Fine (±100 cents). Free (header): each note starts the shape and its orbit anywhere; off, upright, or at the unison copy's phase. Played in Rust per unison copy (ray/player.rs): the Faust copy gives each slot's pitch (so glide, bends, LFO Z's vibrato, Vintage and unison detune all apply), the copy's start phase and note-on; the slot's controls are read from the copy's DSP as the engine set them for the voice. Its raw wave times 2 (`RAY_GAIN`) puts an off-centre Fire near a saw's level; a centred Earth is about 20 dB quieter (Raycaster has no loudness normalization). Not oversampled. CPU here (64-sample block of one voice, U7): 112 µs with a Dual, 136 µs with a Ray, 262 µs with a Chaos Ray orbiting (finding the shape's reach every 4 samples is the cost); a voice is 1333 µs of audio |
| Orthogonal | after Softube's Parallels (Nov 2026; ortho/): two granular players of recorded sounds, Top and Bottom, each on one of 57 sets (ortholib.rs: 3 Bass, 31 Lead, 17 Pad, 6 Vocal, from Jeremy's recordings in D:\samples\synth\Orthogonal), and a Mix between them (a plain crossfade: both at half in the middle). Each set is three recordings, made at C2, C3 and C4 (Bitwig's names: MIDI 48, 60, 72), each four bars at 98 BPM, the note held for two: a note plays the middle one from G2 through F#3, the low one below, the high one above, chosen at note-on per player, at the slot's pitch (so glide, bends, unison and the slot's Pitch knob apply) moved by the player's own Pitch (beside its Speed; the slots' curve: cents, then semitones, then octaves; not modulatable). The granular engine is fixed, after Pigments': 16 grains overlapping, each 180 ms under a trapezoid (a quarter to rise, a quarter to fall), one starting every 11.25 ms at a random spot within the Scatter setting around the player's position and playing forward from there, each detuned at random by up to ±15 cents; summed at full level and divided by the root of the windows' summed squares, so the level holds as grains come and go and a note starts at once. What's played: Position (0–1 of the set; modulatable) plus how far Speed (0–200 %, 100 % the recording's own pace, 0 % Position alone; not modulatable) has carried it since note-on: a held note stops it 4.5 s into the set (as the recording's note did) and the release lets it go on into the tail; modulators move Position past that freely. The panel shows each player's set (its peaks), its name, and a bar at its position (the newest voice's while a note plays); click one for the sets by category. Samples: folded to mono (mid, or the left channel for the eleven sets that cancel by more than 3 dB summed), trimmed past −60 dB, levelled to −18 dB RMS over the held part, stored as 16-bit FLAC (40 MB) embedded in the plugin and decoded only when a set is chosen, on a loader thread (ortho/bank.rs; about 5 MB a set in use; a set nothing has wanted for 30 s is freed; the audio thread reads through RCU-style read sections, never waiting or freeing), so the plugin loads no slower. `ORTHO_GAIN` sets its level in the voice with headroom: the recordings keep their own crest (peaks ~13 dB over RMS), and the voice saturates (mixer heat, the drive's curve even at 0, the filter's loop, the post-filter clip) at the levels it was built for, a saw's. At a saw's peaks a held set lost 6 dB of its peaks (crest 12.7 → 6.6 dB, distortion 10 dB under the signal: what Jeremy heard as clipping); 12 dB lower (now) its crest is 11 dB and the distortion 20 dB under, and it plays about 8 dB quieter than a saw. `jeremys_bounce` in voice.rs renders his test case (Heat 0 helps a little more). CPU here (64-sample block of one voice): 43 µs on Orthogonal against 48 µs on a Dual, 158 µs with U7 (6.7 ns per grain per sample) |
| Windows | Jeremy's (Nov 2026; windows/): a Base wave (Sine, Cosine, Triangle, Saw, Ramp, Square; not modulatable) and three interrupts, each a window in the base's cycle through which a hidden wave shows. Per interrupt: its switch (1, 2, 3), its Shape (Sine, Triangle, Saw, Ramp, Square; not modulatable; no Cosine, Phase covers it), X (0–0.99 of the cycle: where the window opens), Phase (0–360°: the hidden wave's shift against the base cycle, which it's aligned to, so at 0 the window peeks at the hidden wave where it would be beside the base, and moving X scans across it) and Len (0–0.5 of the cycle; 0 closed); X, Phase and Len modulatable. A window past the cycle's end carries on from its start. The wave is the base plus every open window's hidden wave, all at 1 / (1 + the interrupts switched on): not normalized, so short windows leave it quieter, and windows can cancel it to silence. Shapes start at 0 and rise where they can (the saw is the Dual's, jumping half way; the ramp is it turned over). Each cycle's settings are latched when the base's phase wraps (windows never move while open). Every jump and corner in the cycle (windows opening and closing, the waves' own edges, a change of settings at the wrap) is corrected at its time within the sample (polyBLEP, polyBLAMP; one sample of latency), each sized from the whole wave on either side of it, so events on the same spot add up; the cycle's exact mean (from the shapes' integrals) is taken off, so there's no DC and no sub-audio thump from moving X or Len. Played in Rust per unison copy, as the Ray. Aliasing on a 1.23 kHz note with three windows: 36 dB under the signal, 18 dB less than the plain wave sampled. Its base alone is a saw at a Dual saw's RMS (`WINDOWS_GAIN` 1). An interrupt's controls dim while it's off. Each interrupt also has a Pitch (Nov 2026; the small slider under its Shape, ×0.25–×64 on a log travel, ×1 a quarter along, double-click for ×1; modulatable): the hidden wave runs that many times faster inside its window, still counted from the base cycle's start, so it's hard sync (every ratio stays harmonic with the note; sweeping it is a sync sweep), held under 45 % of the sample rate on high notes. The scope draws the windows in additive light (Jeremy's softened cyan, magenta and yellow, after his CMY cube; editor/light.rs): each open window a faint fill of its colour between bright lines at its edges, overlaps adding up as a projector's light does, and the trace drawn dark wherever the fill gets bright (all three overlapping); the rows' labels take their interrupt's colour. While a note plays, the windows sit where the newest voice's modulators have them. The trace is turned to line up with the base cycle's start. CPU here (64-sample block of one voice): 32 µs against 35 µs on a Dual, 75 µs with U7 (90 µs on a Dual); with Pitch up, about the same |
| Noise | white, clocked, LFSR long/short, crackle; tilt EQ (±24 dB around 2 kHz); bypasses the drive. (Its Move, Rate and Mute on the old Texture plate are gone, Nov 2026: a Mute left on there silenced the noise while its fader still rebalanced the mix) |
| Transient | click, burst, zap, FM, chiff, or one of 111 recorded samples (Jade, Mech) with Pitch (±24 semitones, modulatable: a sample's speed, or a synthesized one's frequencies); own level, pre- or post-filter. FM (Nov 2026, in Metal's place in the menu, so patches that used Metal now strike FM): a sine carrier at the note + Pitch under two sine modulators in series (2 → 1 → carrier) at Ratio times the carrier (2:1 to 7:1 in quarters, a knob under the Trans fader), phases reset every note. Over twice the Decay knob's time (`FM_RING` in engines.lib; Jeremy wanted it to ring longer) the carrier falls as (1−x)², modulator 1's depth as (1−x)³, modulator 2's as (1−x)⁴, all reaching 0, so the clang clears before the sine does. Depth (the Transient plate's third knob, modulatable) sets the strike: modulator 1 follows the knob from a 5 % floor; modulator 2 waits a quarter of the travel, creeps to 25 % by three quarters, then catches up to 100 % with modulator 1 at the top; 100 % is 8 radians (engines.lib `FM_INDEX`). High notes are reined in so the sidebands stay under 40 % of the sample rate (aliasing under −25 dB against a 16× oversampled render). Depth and Ratio dim unless the type is FM; the other types render bit-identical to before |
| Unison | after the Virus TI, as measured in OsTIrus (Nov 2026; unison.rs; abdicant-syn/research/synth-character/ostirus.md). The Uni button cycles Off, U5, U7: 5 or 7 copies of both oscillators (only the copies that sound are computed), at evenly spaced positions, each at 1 − ¼ of its distance out (the outer pair only 2.5 dB under the centre; the old bell had it at −12 dB, which is why it sounded thin), scaled so the copies together are as loud as one. Detune (0–127, modulatable) is the Virus's measured curve for the outer pair, the panel showing its cents at A4 (64 is 4.1 ct, 127 is 21.5 ct), wider down the keyboard and narrower up it (about 0.85× per octave, from the Virus's measurements at six notes); the other pairs in proportion. Spread (0–127, modulatable) pans the copies with the Virus's law: straight-line ramps, −4 dB in the middle, the outer pair at the sides from 64, U5's inner pair at 127; the voice turns stereo (its Harmonic EQ, mixer, drive, filter and amp run once for each side, as they're nonlinear) only while its copies are spread, so a narrow or single voice costs what it did; the right side picks up from the left's state when they part. Random phase per note. Eco (the button beside Rnd) plays only the centre copy and the others from snapshots of it: every 12 cycles one cycle is resampled into a 2048-point table, which each side copy reads at its own pitch, fading to each new snapshot over two cycles. A periodic wave comes out the same; what changes inside the cycle (an orbiting Ray, an Orthogonal's grains, OscMod's sample-rate steps, noise) reaches the side copies in steps, and the copies are more alike (more correlated). On 8 voices, Osc 1 a Dual and Osc 2 a Wavetable: 25 % of a core without unison, 68 % U7 narrow, 86 % U7 spread, 50 % spread on Eco (74 % U7 before, mono); two Orthogonals 178 % → 58 % on Eco. Patches from before keep their Detune's cents at A4 (converted on load) and get Spread 64. Parameter "Unison: Mode" (was the On switch, so older projects open with it off) |
| Modulation | ten sources shared by every knob, each with its own depth on each (Nov 2026, after Elektron's assignment; modulation.rs): envelopes A, B and C (ADSRs, each with a curve for its attack and one for its decay and release; A is also the amp, at full depth, B is wired to the cutoff by the Env B fader on the Filter plate, C is free), LFOs X, Y and Z (a shape: the basic waves, Noise, Brown, S&H, or any library wave; Rate 0.01–50 Hz or synced 4/1 … 1/32; Free runs each voice's LFO on through notes, otherwise each note restarts it, and synced and free it follows the song; Z is also wired to both oscillators' pitch: Depth 0–200 cents, scaled by nothing (On), the mod wheel (MW) or aftertouch (AT); out of the box 5 Hz, 50 cents on the wheel), and the mod wheel, aftertouch (channel or poly), the key (bipolar about C4: ±1 at five octaves away) and velocity. Each source's depth on each of 81 knobs (every continuous knob, Balance, the Noise and Trans faders, Drive, the HPF, the wavetables' Speed and the LFOs' Rates among them) is a hidden parameter "mod.<knob>.<source>" (-100..100 % of the knob's travel, curved so small depths get more of the knob), saved with the patch; envelope B's on the cutoff is the Env B fader itself ("filter.env_b"), and the key's on the cutoff counts in octaves per octave (50 % by default). Assigning: each source has a square button (after Elektron's; its LED and its plate's wash in its own earth shade). Press one and the panel dims, showing lit the knobs the source moves, each with an arc of its depth; drag any knob to set its depth (double-click clears it), then press the button again (or Esc, or click away). Otherwise a knob shows a dot of each shade moving it, and while a note plays a rose arc to where the newest voice has it. The engine runs the sources once a block per voice (the LFOs' rates moved by last block's sources, so an LFO can move its own), sums each knob's depths and moves it in its travel; the voice draws a line to each value over the block (voice.lib's `ramped`). A Noise or Trans fader at its bottom stays off whatever moves it |
| Envelopes | A, B and C on one plate, side by side: an assignment button, what it's wired to, four thin full-height faders (Attack, Decay, Sustain, Release), a curve button left of the Attack (the attack's bow) and one right of the Release (the decay's and release's). A curve button is a tiny picture of the bow: drag it up or down (−1..1, + bowed up), double-click resets; while dragged it shows large. Defaults: attacks +0.6, decays and releases −0.6. A is the amp (`ampEnv` in voice.lib, per sample, its times varied per voice by Vintage); the engine runs its own block-rate copies of all three for the knobs they move |
| Perform | the mod wheel, aftertouch, key and velocity's assignment buttons, each with a meter of its level in the newest voice; the pitch bend's job: ±2 st, ±1 oct, cutoff, resonance or the filter's B ±50 %. (Global Pitch is gone: LFO Z is the vibrato) |
| Voicing | 8-voice poly; Portamento above Off = mono (one voice, last-note priority, SH-101-style auto glide: only overlapping notes glide, without retriggering; detached notes start on pitch), glide 20–200 ms |
| Vintage | one knob of per-voice analogue tolerance (after Sequential's Vintage and Diva's trimmers): stable per-voice offsets to tuning (±8 c), cutoff (±¼ oct), envelope A/D/R (±25 %), correlated as on GForce's OB-X (two identity values per voice, mixed into each offset with its own signs, so a sharp voice tends to be brighter and quicker; weights `traitW1`/`traitW2` in voice.lib), slow per-voice pitch drift; a low-passed noise on each voice's pitch (3 cents RMS at the top, below 60 Hz) and another on its level (0.7 dB RMS, below 25 Hz), set by constants in voice.lib; wow and flutter grow with the square of the knob (Boards of Canada at the top) |
| Events, Calls | moved out (Nov 2026) to effects of their own: Alternatives (`../alternatives`) and Calls (`../calls`), with their hidden settings opened and the notes coming from MIDI |
| Harmonic | the plate after the Filter's (Nov 2026, in FDSP's place: FDSP's Water, Tornado, Flange and Phase are gone): a harmonic EQ after DS Thorn's Harmonic Filter (harmonic.rs), on each oscillator slot before the mixer, at the slot's own pitch, so a shape over the harmonics lands on the same partials at every note (and on each slot's partials when their Pitch knobs differ). Shape: Off or one of Thorn's 81 Harmonic Filter presets (harmlib.rs, from Jeremy's install by tools/gen-harmlib.py), each 8192 magnitudes over harmonic number that Thorn reads at harmonic × k, its sweep moving k (a bounce of a saw swept through Amp 01 fits that within 5.5 dB RMS over 80 harmonics); Shift (±4 octaves, modulatable) moves k, 16 at 0, sliding the shape along the harmonics. 16 peaking filters (Simper's SVF bell) at harmonics 1–6, 8, 10, 12, 16, 20, 24, 32, 40, 48, 64: each band's target is the shape's mean power over the harmonics it covers, floored 30 dB under the loudest, lifted or lowered (within 12 dB) so a saw comes out as loud as it went in, scaled by Depth (0–1), plus Tilt (±6 dB per octave of harmonics, about the 8th); the gains are the targets through the inverse of the bands' overlap (±30 dB at most). Against the Thorn bounce the chain lands within 2.4–4.3 dB RMS over 64 harmonics (outside the bounce's first second, where Thorn's partial-count limit cuts in, which this doesn't copy: the filter can). Odd/Even (−1..1): a comb half the slot's period long, −1 only the odd harmonics, +1 only the even. The screen draws each of the first 64 harmonics' gain and the bands' curve. Skipped while Off with Tilt and Odd/Even at 0, and on a slot whose fader is at 0; on, about 6 µs per slot per 64-sample block here (with Shift moving, the gains re-solved every block) |
| Dice | the die in the top bar rolls a patch (dice.rs): it picks a kind of sound (pad, drone, keys, pluck, bass, lead; weighted toward pads), which sets the ranges for the envelope and filter, then follows rules (a Harmonic EQ shape now and then, 15–30 % by kind, Shift within ±1.5 octaves, its Shift sometimes moved by an LFO or the envelope; Odd/Even rarely): transients only under fast attacks, Osc 1 always sounding and Osc 2 often, each slot a Dual or a Wavetable with both engines rolled (now and then a Ray, a Windows (one to three interrupts spread round the cycle, mostly short, mostly at phase 0, 90 or 180°), or an Orthogonal on two sets of a category that suits the kind, more often in pads and drones), Osc 2's Pitch moved now and then by an octave, rarely by an interval and then mostly a fourth or fifth (never when silent); Osc 1's only by octaves, five related waves and a Speed for the sustain's bounce, unused sources at 0, the Cutoff's envelope and key tracking, and a few other modest modulators. Perform, LFO Z (the vibrato), Volume and Portamento stay as they are; every depth is reset first, then the picks add a few (an LFO, X or Y, rolled the first time a pick needs it; envelope C; or velocity). The ‹ beside it steps back through the last 32 patches until the editor closes. Only the parameters a roll changes are sent to the host: CLAP takes at most 2048 changes from the panel at once (three events each), and sending all ~1,400 values, nearly all of them modulators at rest, dropped most of the rest (test `rolls_fit_the_hosts_queue`) |
| Init | the ∅ beside the die resets the patch (dice.rs's `INIT`): every parameter to its default (Volume, Perform and the settings too), then I a Dual's plain saw alone (Balance all the way to I), with II, noise, transient, drive, Vintage, Unison and the Harmonic EQ off. For a fresh start whatever the host's own default preset holds; the ‹ steps back from it as from a roll |
| Oscillator presets | each engine (Dual, Wavetable, Ray, Orthogonal, Windows) keeps a collection of its own settings (presets.rs), shared by both slots and every project: JSON files in `%APPDATA%\Abdication\osc-presets\<Engine>\` (elsewhere `~/.config/Abdication/osc-presets/`). A slot's header has the list (the presets for the engine it plays; the last loaded or saved shows by name) and a save button, which asks for a name (pre-filled; the first key typed replaces it; Enter saves, Esc cancels; a preset of that name is replaced). A preset holds only the engine's own controls ("wt_1.wave_3" saved as "wave_3"), never the depths on them: loading one leaves every modulation as it is |
| Output | each voice at `voiceLevel` (a held saw at full velocity peaks about 6 dB under full scale; mono, or stereo while its unison copies are spread), then Volume, then a stereo peak limiter, −1.5 dB ceiling, 50 ms release (plugin only) |

## CPU

Faust's Rust output computes every branch of a switch, so the DSPs choose
with `enable()` instead, through core.lib's `only` and `choose`: only each slot's engine,
the selected Dual algorithm, OscMod, noise type, transient type and drive
curve are computed. Also skipped while they can't be heard: a source whose
Balance has turned all the way from (or whose fader is at 0), the transient once it has faded, Vintage's noises at 0,
the second filter stage
in 2-pole mode, and the Harmonic EQ while it's off. The output
is bit-identical (a test renders patches to compare builds). Measured with
`cargo test --release cpu_worst_case -- --ignored --nocapture` (8 voices,
Osc 1 a Dual and Osc 2 a Wavetable, both up; ABD_OSC2=0 makes both Duals, ABD_OSC1=1
both Wavetables; ABD_UNISON=0 without unison, 5 for U5; ABD_SPREAD=0..127, 64 by
default; ABD_ECO=1 on Eco; ABD_MODS=1 runs every knob's modulators) and, per part,
`profile_parts` in voice.rs (ABD_SET="id=value,..." sets controls); per engine
with and without unison, spread and Eco, `unison_costs` in voice.rs.

Caveat: a disabled `enable()` outputs the last value it computed, not 0
(and freezes the state inside it). So never call `enable` directly: `only`
zeroes the output while disabled. Calling it directly left DC offsets after
a switch or fader change (test `switching_leaves_no_offset`), and nested
enables froze the old Events' hold and walker state.

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
  before (within float rounding; the mixer's 3 dB push stays). At 1 the saturation's part
  of a full-level voice sits about 14–19 dB under it, at about the same
  loudness (the makeup), with peaks 1–2 dB lower.
- **Ortho Scatter** (0–500 ms, default 100): the spread around an
  Orthogonal player's position its grains start within (the whole spread,
  half each way). 0 starts every grain at the position.

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
  plugin-osc.dsp    one unison copy of a voice's two oscillators (plugin build;
                    7 per voice with Unison on, the centre one with it off)
  plugin-mix.dsp    one voice up to its filter: mixer, noise, transient,
                    drive (plugin build)
  plugin-voice.dsp  one voice from its filter on (plugin build)
  texture.lib       cheap randomness and maths (streams, exp2, wander,
                    coloured noise), for Vintage
  harmlib.lib       GENERATED: the Harmonic EQ's Shape menu
                    Several DSPs, not one: as one, the Faust compiler took
                    tens of minutes and more memory than the machine had.
                    Splitting the voice at its filter (mix, voice)
                    took the whole regeneration from ~100 s to ~16 s:
                    `choose`s in a row on the whole voice are what's slow
  wavelib.lib       GENERATED: wave-library layout, menus, and the reader
                    (samples come from the plugin via wt_sample())
plugin/             Rust CLAP/VST3 plugin (nih-plug)
  src/dsp.rs, mix_dsp.rs,
  osc_dsp.rs        generated from the Faust DSPs (don't edit)
  src/pitch.rs      the slots' Pitch knob: its travel as an interval
  src/modulation.rs the modulation sources (envelopes A-C, LFOs X-Z, the
                    played ones), the knobs they move, the wavetables' paths,
                    and how depths move a knob in its travel
  src/presets.rs    the oscillator engines' presets (files, no UI)
  src/voice.rs      one voice: unison copies -> Harmonic EQ (per slot) ->
                    mixer -> the rest
  src/harmonic.rs   the Harmonic EQ: band targets from a shape, the gain
                    solve, the bells and the Odd/Even comb
  src/harmlib.rs    the Harmonic EQ's shapes (harmlib.bin, generated)
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
  src/ortholib.rs   the Orthogonal's embedded sample sets (ortholib.bin, FLAC,
                    and ortholib_index.rs, generated) and how to decode one
  src/windows/      the Windows: mod.rs (a cycle's wave, its events and their
                    corrections; one oscillator), player.rs (a voice's slots)
  src/ortho/        the Orthogonal: bank.rs (decodes chosen sets on a loader
                    thread), grains.rs (one granular player), player.rs (a
                    voice's slots: travel, hold, which recording)
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
  gen-harmlib.py    DS Thorn's Harmonic Filter presets -> plugin/src/harmlib.bin
                    + harmlib_index.rs + dsp/harmlib.lib
  gen-ortholib.py   the Orthogonal's recordings (D:\samples\synth\Orthogonal) ->
                    plugin/src/ortholib.bin + ortholib_index.rs + dsp/ortholib.lib
                    (needs numpy and ffmpeg)
  make-bitmask-sets.py  renders the bitmask sets from the old oscillator's rules
  preview-set.py    contact sheet + audition WAV for a wave set
  chop-oneshots.py  splits a recording of one-shots into trimmed, normalized samples
  wavetools.py      shared wave-set helpers
tables/             wave library sources: wavetable WAVs (2048-sample
                    float32 frames, Serum-style 'clm ' chunk; .json =
                    provenance) or folders of single cycles
  image-wavetables/  32 Argon8 waves traced from its wave images, one
                    2048-sample cycle per WAV, in manifest.json's order
                    (DC removed, peak 1, phase as drawn) ("A8 nn")
  argon8-c2.wav     the earlier Argon8 set, from recordings (118 waves;
                    no longer built in)
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
| Mixer | Balance crossfades I and II, equal power (both −3 dB in the middle; Nov 2026, in place of the normalized four-source balance, which was too unusual too often); the noise has its own level, and the transient its own (Trans). The oscillators meet a saturation stage at Balance's levels pushed 3 dB (`mixerPush`, which carries on into the drive and out; `mixerHeat`, after Shibboleth's mixer): 60 % of the signal through one biased tanh, 40 % split at 120 Hz and 4 kHz into lows, mids and highs, each through its own curve (drives 1.5 / 1.25 / 0.9, biases +0.12 / −0.08 / +0.05; the whole at 1.0, +0.08), each curve moved so silence stays at 0 and quiet signals pass at unity, then a 5 Hz DC blocker. An oscillator Balance has turned all the way from isn't computed (a modulator moving Balance brings it back). Not oversampled |
| Drive | on the Filter plate (Nov 2026): Sub 37-style multidrive on one thin fader: soft → hard clip → tube → fold, a quarter of the travel each (curves crossfade near the boundaries), gain rising steadily through the lower half (about 7 dB at the middle) and steeply at the top (to 24 dB); its curve and name are drawn beside it; level-compensated; fixed audio-rate bias chaos (500 Hz, 0.2). Its I and II switches send each oscillator through it or around it (both on by default): with II off, an Orthogonal there stays clean while I is driven (the oscillators going round meet the mixer saturation on their own, and join after the drive; with both switches the same way only one path is computed). The noise always goes round it; a pre-filter transient goes through |
| Filters | own TPT state-variable filter, saturating in-loop (character always full) through a lopsided tanh (even harmonics, after Model 84/82's asymmetric state curves; `svfAsym`), resonance eased below 750 Hz (after GForce's OB-X; `svfLowQHz`, `svfLowQEase`), with LP→notch→HP morph (its A) + bandpass blend (its B), 2/4-pole; or (Type) Raycaster's 4-pole lowpasses, ported from its filters.rs (`zdf4`, matching it to float precision): a transistor ladder (every stage saturating on its input and output) or the SH-101's (OTA stages saturating on their difference, resonance through soft-clipping diodes, half of it added back at the output so it keeps more bass), both zero-delay feedback with each stage's tanh linearized at its last input, self-oscillating at the top of Resonance (its 0.5..6 maps to their 0..1); Their A spreads the four poles over up to an octave around the cutoff (half an octave each way: subtle at low resonance, it mostly softens and moves the resonant peak), and their B moves the cutoff by the noise oscillator, up to two octaves each way at full (the noise is computed for it even with its fader down). A and B are each type's own (voice.lib's svfType, ladderType, lp101Type; the panel names them by type: "A · Morph", "A · Spread"), so a new type brings its own; 4-Pole greys out for the ladders. The SH-101's is called 101. After the filter, a symmetric soft clip (`postClip`, tanh(1.25 x), after Shibboleth's) that a resonant peak runs into whatever the amp envelope does, then 2 dB of makeup per unit of Heat. Then the HPF, the MS-20's high-pass (`ms20HPF`; the early Korg35 version, after Stinchcombe's MS-20 study and Pirkle's Korg35 model): a Sallen-Key with a degenerate high-pass output, so a 6 dB/oct skirt under a full two-pole resonance (H(s) = s(s + 1) / (s² + (2 − K)s + 1)), zero-delay feedback through three TPT one-poles; its Peak's feedback runs through a slightly lopsided diode clipper (linearized at the last sample, as `zdf4`), which holds the self-oscillation from Peak 0.85 up at a fixed level and lets a loud input choke the resonance. Peak 0 is the plain one-pole cleanup HPF it replaced (the same output to −120 dB). Cutoff 10 Hz (off) – 8 kHz: on the panel an arc round the cutoff knob's lower left quarter, 6 o'clock off, 9 o'clock 8 kHz, with Peak a trim in the corner; modulatable. The screen draws its small-signal response |
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
voice at ~15 Hz while a note plays; an offline render otherwise). A scope
that Balance has turned all the way from is flat: the voice doesn't compute
that oscillator. The source buttons assign modulation (see Modulation);
the wave boxes and LFO shapes open the wave picker. The skin (Nov 2026):
warm umber plates, each with a saturated strip (orange the oscillators,
saffron the mix, teal the filter, cobalt the Harmonic EQ; the modulator
plates one segment per source), each source's group washed in its own
earth shade; the mockup it follows is the Design canvas "Abdication — new
panel mockup".

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
- Center/spread dual-filter mode.
- Band-limit / oversample the sync, window and bitmask waves (they alias).
- Curate the library (drop weak waves, maybe reorder) and pick better
  default waves.
- Panel: the wave-browser pop-out (wave thumbnails, audition), typed
  value entry, modulation shown on the knobs, a MIDI activity light.

Deferred experiments: hosting third-party effects before the filter
(collides with polyphony).
