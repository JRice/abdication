#!/usr/bin/env bash
# Regenerate the plugin's Rust DSP code from the Faust sources: the callers and
# the reverb (the echo between them is the plugin's own, in Rust). Needs the
# native Faust compiler (2.70+). Run from this folder (calls/).
set -euo pipefail
gen() { faust -t 0 -I dsp -lang rust -cn "$2" "dsp/$1" -o "plugin/src/$3" && echo "regenerated plugin/src/$3"; }
gen plugin-calls.dsp CallsDsp calls_dsp.rs
gen plugin-reverb.dsp ReverbDsp reverb_dsp.rs
