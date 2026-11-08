"""Contact sheet (PNG) and audition (WAV, each wave 0.4 s at C3) for a
wave set.  usage: python3 tools/preview-set.py tables/<set>.wav <out-dir> [title]"""
import os
import sys
sys.path.insert(0, "tools")
import numpy as np
import scipy.io.wavfile as wavfile
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from wavetools import read_wavetable

path, out_dir = sys.argv[1], sys.argv[2]
name = os.path.splitext(os.path.basename(path))[0]
title = sys.argv[3] if len(sys.argv) > 3 else name
waves = read_wavetable(path)
n, cols = len(waves), 12
rows = int(np.ceil(n / cols))

fig, axs = plt.subplots(rows, cols, figsize=(cols * 1.6, rows * 1.15 + 0.4), facecolor="#fafafa")
for i, ax in enumerate(np.atleast_1d(axs).flat):
    ax.set_xticks([]); ax.set_yticks([])
    for s in ax.spines.values():
        s.set_visible(False)
    if i < n:
        ax.plot(waves[i], color="#2b5c8a", lw=0.9)
        ax.axhline(0, color="#ccc", lw=0.5)
        ax.set_ylim(-1.1, 1.1)
        ax.set_title(str(i + 1), fontsize=8, color="#444", pad=1)
fig.suptitle(f"{title}: {n} waves", fontsize=10, color="#222", y=0.995)
plt.tight_layout(rect=(0, 0, 1, 0.97))
fig.savefig(os.path.join(out_dir, f"{name}-waves.png"), dpi=110)

sr, f, dur, gap = 48000, 130.81, 0.4, 0.06
t = np.arange(int(dur * sr))
phase = (t * f / sr) % 1
fade = np.minimum(1, np.minimum(t, len(t) - 1 - t) / (0.01 * sr))
parts = []
for w in waves:
    F = np.fft.rfft(w)
    F[int(20000 / f) + 1:] = 0                       # band-limit for C3
    c = np.fft.irfft(F, len(w))
    parts += [np.interp(phase * len(c), np.arange(len(c) + 1), np.append(c, c[0])) * fade * 0.5,
              np.zeros(int(gap * sr))]
wavfile.write(os.path.join(out_dir, f"{name}-audition.wav"), sr, np.concatenate(parts).astype(np.float32))
print(f"{name}: {n} waves -> {out_dir}")
