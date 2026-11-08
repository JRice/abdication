# Abdicant instruments

Personal synthesizers and effects by Abdicant, built as CLAP and VST3 plugins on
[nih-plug](https://github.com/robbert-vdh/nih-plug) and played in Bitwig on Windows.
Not for sale, not for support. Each folder has its own README with the controls,
and most have design or measurement notes under `docs/`.

## The instruments

| Folder | What it is |
|---|---|
| `abdication` | The main polysynth: Fundamental, Dual (Cobalt8-style algorithms) and Wavetable oscillators, noise and transients, a saturating morphing filter, per-voice modulators with a send on nearly every control, Vintage tolerance, and texture (spectral Events and whistled Calls). Faust DSP. |
| `abdrum` | A drum voice per instance, for Bitwig's Drum Machine: Tone, a modal Body, Noise and two sample layers, a free envelope with rings on every knob, and a fixed Crush, Drive, Ring mod, Comb and Compressor chain. Faust DSP. |
| `antimeme` | The noisy delay of the Make Noise Strega, calibrated against a recording of a real one. |
| `anathema` | A circuit-bent Yamaha R100: an early digital reverb with its clock on a knob and a switch matrix across its memory bus. |
| `amara` | Babbles in seeded invented languages through a low-bitrate vocoder over a bad line, with occasional possession. |
| `arecibo` | A no-input mixer with a shortwave radio link in its feedback loop. Runs with no input; input disturbs it. |
| `argent` | A four-note latching drone: Fizmo-style transwaves played the Vhikk X way (Morph, Scan, Seed), with no delay or reverb. |
| `raycaster` | A twelve-voice synth whose oscillators are shapes turning once per cycle, read by a ray dropped from above. |

Shared material:

- `abdrum library/` holds ABDrum's sample sets (transients and noise loops). The folders
  are the source of truth. `abdrum tools/build_library.py` built them from the drum
  sample collection, which isn't in the repo.
- `abdication/tables/` holds Abdication's wave library and recorded transients.

## Building

Every plugin builds and installs the same way, on Windows, from its own folder:

```powershell
powershell -ExecutionPolicy Bypass -File plugin\install.ps1
```

This needs Rust (rustup, MSVC toolchain) and Visual Studio Build Tools with "Desktop
development with C++". It builds the `.clap` and `.vst3` bundles and copies them to the
user CLAP and VST3 folders under `%LOCALAPPDATA%\Programs\Common`. Quit Bitwig fully
first (it keeps plugin processes alive), then rescan. Tests run with `cargo test` in each
`plugin` folder.

To build and install all of them, or a few by folder name, from the repo root:

```powershell
powershell -ExecutionPolicy Bypass -File build-all.ps1
powershell -ExecutionPolicy Bypass -File build-all.ps1 argent raycaster
```

It carries on past a failed build and ends with a table of what succeeded and how long
each took.

Six of the plugins are hand-written Rust. Abdication and ABDrum are written in Faust and
compiled to Rust, so they have generated files:

- **DSP:** edit the `.dsp` and `.lib` files under `dsp/`, then run `tools/gen-rust.sh`
  (needs native Faust 2.70 or later) to regenerate `plugin/src/*_dsp.rs`. The generated
  Rust is committed, so building needs no Faust.
- **Samples:** they're embedded in the binary as generated `.bin` files, also committed.
  After changing the sources, regenerate them and then run `gen-rust.sh`:
  `abdication/tools/gen-wavelib.py` and `gen-translib.py` read `abdication/tables/`, and
  `abdrum/tools/gen-samplelib.py` reads `abdrum library/`.

## Not in the repo

Build output (`target/`, `build/`), and audio other than the two sample sets above:
reference recordings, Strega captures, renders and examples stay local.
