"""Check that the plugin's Rust DSP matches the offline C++ renderer.

    python tools/compare-backends.py

Renders every preset's first hit (velocity 1, fixed seed, note 60) through
both Faust backends and reports the largest sample difference. Guards
against Faust Rust-backend bugs (Abdication met one with nested selects)."""
import os
import re
import subprocess
import sys

import numpy as np

sys.path.insert(0, os.path.dirname(__file__))
import render as R  # noqa: E402
from presets import PRESETS  # noqa: E402

DIR = os.path.abspath("build/compare")
SEED, SECONDS = 0.25, 1.0


def slug(s):
    return re.sub(r"_+", "_", re.sub(r"[^a-z0-9]", "_", s.lower())).strip("_")


labels = R.build()
os.makedirs(DIR, exist_ok=True)
for n, preset in enumerate(PRESETS):
    first = preset.get("hits", [(1.0, SECONDS)])[0]
    values = dict(R.resolved({**preset["set"], **(first[2] if len(first) > 2 else {})}, labels))
    with open(f"{DIR}/{n:02d}.txt", "w") as f:
        f.write("".join(f"{slug(p.split('/')[0])}.{slug(p.split('/')[1])} {v}\n" for p, v in values.items()))
        f.write(f"seed {SEED}\nseconds {SECONDS}\n")
    lines = [f"set /ABDrum/{p.replace(' ', '_')} {v}" for p, v in values.items()]
    lines += ["set /ABDrum/freq 261.6256", f"hit 1.0 {SEED} {SECONDS}"]
    R.render(lines, f"{DIR}/{n:02d}.cpp.raw")

subprocess.run(["cargo", "test", "--release", "--", "--ignored", "compare", "--quiet"], cwd="plugin", check=True,
               env={**os.environ, "ABDRUM_COMPARE": DIR}, capture_output=True)
worst = 0.0
for n, preset in enumerate(PRESETS):
    a = np.fromfile(f"{DIR}/{n:02d}.cpp.raw", "<f4"); b = np.fromfile(f"{DIR}/{n:02d}.rust.raw", "<f4")
    m = min(len(a), len(b)); diff = np.abs(a[:m] - b[:m]).max(); peak = np.abs(a[:m]).max()
    worst = max(worst, diff / (peak + 1e-12))
    print(f"{preset['name']:22} peak {peak:.3f}  max diff {diff:.2e}  ({20 * np.log10(diff / peak + 1e-12):6.1f} dB re peak)")
print("OK" if worst < 1e-3 else "MISMATCH")
