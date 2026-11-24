# Spire: what to render

Started 2026-11-24. Reference renders from Reveal Sound's Spire for three jobs: a Spire slot
engine in Abdication (HardFM, SawPWM, Vowel, over Abdication's own wave library), three Spire
filter families as Abdication filter Types (Perfecto, Infecto, Scorpio), and another pass at
Apse's tail. Everything is built from Spire's documented behaviour and these renders; nothing is
taken from the plugin binary (no disassembly, no lifting its tables or waves).

Save the WAVs in `character/spire-renders/` (git ignores audio), named as below. Note Spire's
version, and any knob that wouldn't go neutral, in `spire-renders/notes.txt`.

## Common setup

48 kHz, 32-bit float, mono is fine for the oscillators and filters (stereo for the reverb).
Start from Spire's init patch, then:

- Osc 1 only (Osc 2–4 volume fully left), unison 1 voice, Phase at 30 (restart) for every render
- Amp envelope A 0, D 0, S max, R about 50 ms; velocity to amp off if it's on
- FX all off; master and osc volume at their defaults (write them down if you change them)
- For the oscillator renders, both filters wide open: Perfecto LP4, cutoff max, reso 0, keytrack 0
- Sweeps are Bitwig automation on the plugin parameter, straight-line ramps over the time given,
  with the note held throughout; "grid" renders are held steps of 2 s each

Values are Spire's 0–1000 display. If a knob shows another range, use the same fractions.

## Oscillators (C2 unless named)

Note which wave each HardFM render uses (Spire's default, plus one plain wave of your choice:
render the HardFM set twice, `_w1` and `_w2`).

| File | Mode | Moves | Held |
|---|---|---|---|
| `hf_A-sweep_B0_w1.wav` | HardFM | Ctrl A 0→1000 over 20 s | B 0, Wt Mix 0 |
| `hf_A-sweep_B500_w1.wav` | HardFM | Ctrl A 0→1000 over 20 s | B 500 |
| `hf_B-sweep_A500_w1.wav` | HardFM | Ctrl B 0→1000 over 40 s (its steps and detune zones) | A 500 |
| `hf_wt-sweep_A500_w1.wav` | HardFM | Wt Mix 0→1000 over 20 s | A 500, B 0 |
| `hf_A-sweep_B0_w1_C4.wav` | HardFM | as the first, at C4 | |
| `spwm_A-sweep_B500.wav` | SawPWM | Ctrl A 0→1000 over 20 s (where the four types change) | B 500 |
| `spwm_t1_B-sweep.wav` … `t4` | SawPWM | Ctrl B 0→1000 over 20 s, one file per type | A in that type |
| `spwm_t1_B-sweep_C4.wav` | SawPWM | as type 1, at C4 | |
| `vw_A-sweep_B0.wav`, `_B500`, `_B1000` | Vowel | Ctrl A 0→1000 over 20 s | B as named |
| `vw_B-sweep_A0.wav` | Vowel | Ctrl B 0→1000 over 20 s | A 0 |
| `vw_A-sweep_B500_C4.wav` | Vowel | as `_B500`, at C4 (do the formants stay put?) | |
| `vw_wt-sweep.wav` | Vowel | Wt Mix 0→1000 over 20 s, if it does anything | A 0, B 500 |

## Filters

Fourteen modes: Perfecto LP4, BP4, HP4, Peak; Infecto LP2, BP2, HP2, Notch; Scorpio RedLP2,
RedLP4, BlackLP2, BlackLP4, BlackHP, BlackBP. Filter 1 only (filter 2 out of the path, or open as
above). Do the lowpasses first (Perfecto LP4, Infecto LP2, both Scorpio LP4s); the rest after.
Name each `f_<family>-<mode>_…`, e.g. `f_perfecto-lp4_noise-grid.wav`.

| File suffix | Source | What changes |
|---|---|---|
| `_noise-grid` | Osc 1 Noise mode, Ctrl A 500 (full band), B 0 | nine 2 s steps: cutoff 250, 500, 750 × reso 0, 500, 900 (cutoff-major: 250/0, 250/500, 250/900, 500/0 …) |
| `_saw-sweep_R0` | Classic, Ctrl A 0 (saw), C2 | cutoff 0→1000 over 20 s, reso 0 |
| `_saw-sweep_R800` | as above | reso 800 |
| `_level` | as above, cutoff 500, reso 500 | osc volume at a quarter, half, full: three 2 s steps (how it drives) |
| `_selfosc` | Osc 1 volume off, C4 held 4 s | reso max, cutoff 500, keytrack 0 (skip if silent) |

Keep any filter drive or extra knob at its minimum and note it.

## Reverb (for Apse)

Stereo, reverb Mix 100 %, Predelay 0, SYNC off, every other FX off. Two sources, each a single
hit followed by 30 s of silence:

- burst: Noise mode (Ctrl A 500), amp A 0, D 5 ms, S 0, R 0
- stab: Classic saw, C3 E3 G3 for 100 ms, amp A 0, D 0, S max, R 30 ms

Render both dry first (reverb off): `rv_dry_burst.wav`, `rv_dry_stab.wav`. Then, for each reverb
mode your Spire has (list them in notes.txt; Plate 1 first, and whichever you reach for most),
with the knobs not named at their defaults and Color centred:

| File | Settings |
|---|---|
| `rv_<mode>_burst_base.wav` | Decay 500, Damp 500 |
| `rv_<mode>_burst_dec250.wav`, `_dec1000` | Decay 250, 1000 |
| `rv_<mode>_burst_damp0.wav`, `_damp1000` | Damp 0, 1000 |
| `rv_<mode>_burst_color0.wav`, `_color1000` | Color at each end |
| `rv_<mode>_burst_wide0.wav`, `_widemax` | Wide at each end |
| `rv_<mode>_burst_left.wav` | base, Osc 1 panned hard left (is the input summed to mono?) |
| `rv_<mode>_stab_base.wav` | base, the stab, for listening side by side |

The dry renders go through Apse at matched settings, so the comparison is like for like.
