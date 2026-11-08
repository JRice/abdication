"""Render the bitmask oscillator's 255 masks at 8 and 16 slices, keep the
audibly distinct ones (harmonic profile < 0.98 similar), and save them as
wave sets in tables/. Run from the repo root."""
import json
import sys
sys.path.insert(0, "tools")
from wavetools import masked_sine, distinct, normalize, write_wavetable
import numpy as np

THRESHOLD = 0.98
for slices in (8, 16):
    masks = np.arange(1, 256)
    waves = np.array([masked_sine(m, slices) for m in masks])
    keep = distinct(waves - waves.mean(axis=1, keepdims=True), THRESHOLD)
    out = normalize(waves[keep])
    name = f"bitmask-{slices}"
    write_wavetable(f"tables/{name}.wav", out, name)
    json.dump({"source": f"bitmask oscillator, {slices} slices", "frame_size": 2048,
               "frames": len(out), "masks": [int(m) for m in masks[keep]],
               "near_duplicate_threshold": THRESHOLD},
              open(f"tables/{name}.json", "w"), indent=1)
    print(f"{name}: {len(out)} waves")
