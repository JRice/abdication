"""Chop a recording of one-shots (separated by silence) into individual
samples: trim leading/trailing silence, peak-normalize, save as float32
mono WAVs plus a manifest.

  python3 tools/chop-oneshots.py <recording.wav> <out-dir> <prefix>

Defaults suit short transients: a hit ends after 150 ms below -50 dB;
trimming keeps everything above -70 dB; peaks go to -1 dBFS; a 1 ms fade
closes each tail."""
import json
import os
import sys
import numpy as np
import scipy.io.wavfile as wavfile

GAP_S, SPLIT_DB, TRIM_DB, PEAK_DBFS, FADE_S = 0.15, -50.0, -70.0, -1.0, 0.001

src, out_dir, prefix = sys.argv[1], sys.argv[2], sys.argv[3]
sr, x = wavfile.read(src)
if x.dtype.kind == "i":
    x = x / float(2 ** (8 * x.dtype.itemsize - 1))
x = np.asarray(x, dtype=np.float64)
mono = x.mean(axis=1) if x.ndim > 1 else x
level = np.abs(mono)

# 1. Find hits: runs above SPLIT_DB, ended by GAP_S of quiet.
frame = int(sr * 0.005)
env = level[: len(level) // frame * frame].reshape(-1, frame).max(axis=1)
loud = 20 * np.log10(env + 1e-12) > SPLIT_DB
hits, start, quiet = [], None, 0
for k, on in enumerate(loud):
    if on:
        start = k if start is None else start
        quiet = 0
    elif start is not None:
        quiet += 1
        if quiet * frame / sr > GAP_S:
            hits.append((start * frame, (k - quiet + 1) * frame))
            start, quiet = None, 0
if start is not None:
    hits.append((start * frame, len(env) * frame))

# 2. Trim each to the TRIM_DB span (searching a little around the hit),
#    normalize, fade the tail, save.
os.makedirs(out_dir, exist_ok=True)
gate = 10 ** (TRIM_DB / 20)
peak_target = 10 ** (PEAK_DBFS / 20)
manifest = []
for n, (a, b) in enumerate(hits, 1):
    lo, hi = max(0, a - frame), min(len(mono), b + int(GAP_S * sr))
    above = np.flatnonzero(level[lo:hi] > gate)
    s, e = lo + above[0], lo + above[-1] + 1
    clip = mono[s:e].copy()
    clip *= peak_target / np.abs(clip).max()
    fade = min(len(clip), int(FADE_S * sr))
    clip[-fade:] *= np.linspace(1, 0, fade)
    name = f"{prefix}-{n:03d}.wav"
    wavfile.write(os.path.join(out_dir, name), sr, clip.astype(np.float32))
    manifest.append({"file": name, "source_seconds": round(s / sr, 4),
                     "length_ms": round(1000 * len(clip) / sr, 1)})

json.dump({"source": os.path.basename(src), "sample_rate": sr, "format": "float32 mono",
           "peak_dbfs": PEAK_DBFS, "trim_db": TRIM_DB, "samples": manifest},
          open(os.path.join(out_dir, f"{prefix}.json"), "w"), indent=1)
lengths = [m["length_ms"] for m in manifest]
print(f"{len(manifest)} samples, {min(lengths)}-{max(lengths)} ms (median {np.median(lengths)} ms) -> {out_dir}")
