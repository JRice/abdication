#!/usr/bin/env bash
# Regenerate the plugin's Rust DSP code from the Faust sources: a voice's
# DSPs (see dsp/voice.lib) and the calls bus. Needs the native Faust compiler (2.70+).
# Run from the repo root. Takes about 20 seconds.
set -euo pipefail
gen() { faust -t 0 -I dsp -lang rust -cn "$2" "dsp/$1" -o "plugin/src/$3" && echo "regenerated plugin/src/$3"; }
gen plugin-voice.dsp VoiceDsp dsp.rs
gen plugin-osc.dsp OscDsp osc_dsp.rs
gen plugin-texture.dsp TextureDsp texture_dsp.rs
gen plugin-callbus.dsp CallBusDsp callbus_dsp.rs
