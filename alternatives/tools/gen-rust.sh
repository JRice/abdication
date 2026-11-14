#!/usr/bin/env bash
# Regenerate the plugin's Rust DSP code from the Faust source. Needs the
# native Faust compiler (2.70+). Run from this folder (alternatives/).
set -euo pipefail
faust -t 0 -I dsp -lang rust -cn AlternativesDsp dsp/plugin.dsp -o plugin/src/dsp.rs
echo "regenerated plugin/src/dsp.rs"
