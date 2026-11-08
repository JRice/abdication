#!/usr/bin/env bash
# Regenerate the plugin's Rust DSP code from the Faust sources: the voice
# (one per overlapping hit), its Body and Noise (likewise, each run only
# while it sounds)
# and the effects (one per pad, on their sum).
# Needs the native Faust compiler (2.70+). Run from the repo root.
set -euo pipefail
faust -t 0 -I dsp -lang rust -cn VoiceDsp dsp/plugin-voice.dsp -o plugin/src/dsp.rs
faust -t 0 -I dsp -lang rust -cn FxDsp dsp/plugin-fx.dsp -o plugin/src/fx.rs
faust -t 0 -I dsp -lang rust -cn BodyDsp dsp/plugin-body.dsp -o plugin/src/body.rs
faust -t 0 -I dsp -lang rust -cn NoiseDsp dsp/plugin-noise.dsp -o plugin/src/noise.rs
echo "regenerated plugin/src/dsp.rs, fx.rs, body.rs and noise.rs"
