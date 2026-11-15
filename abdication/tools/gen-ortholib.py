"""Build the Orthogonal oscillator's sample library from a folder of WAVs.

    python tools/gen-ortholib.py D:\\samples\\synth\\Orthogonal

The folder holds one set per sound, three recordings each: "_<T> <Name> L.wav",
"... M.wav" and "... H.wav", played at C2, C3 and C4, every one four bars at
98 BPM (two held, two of tail). <T> is the category: B Bass, L Lead, P Pad,
V Vocal. Each recording is

  - folded to mono: (L+R)/2, unless that loses more than MID_LOSS_DB of the
    set's energy (very wide sounds, whose channels partly cancel), when the
    left channel is used alone, for all three of the set
  - peak-normalized and trimmed after its last 10 ms above -60 dB (re its
    peak), with a 10 ms fade; the start is untouched, so every recording
    keeps the bar timing
  - given a playback gain that brings its held part (the first two bars) to
    TARGET_RMS_DB, so every sound and every octave of it plays as loud
  - stored as 16-bit FLAC (TPDF-dithered).

Writes:
  plugin/src/ortholib.bin       every recording's FLAC stream, back to back
  plugin/src/ortholib_index.rs  the sets: name, category, length, and per
                                recording where its stream is and its gain;
                                plus each set's overview (the panel's picture)
  dsp/ortholib.lib              the sample menus for the Faust controls

Needs numpy and ffmpeg on the PATH (decoding 24-bit WAVs, encoding FLAC).
Set indices are menu order (category, then name): adding a sound moves the
ones after it in saved patches."""
import json
import os
import re
import subprocess
import sys

import numpy as np

RATE = 44100
MID_LOSS_DB = -3.0
TARGET_RMS_DB = -18.0
HELD_S = 2 * 4 * 60 / 98          # two bars of 4/4 at 98 BPM
TRIM_DB = -60.0
WINDOW = RATE // 100              # 10 ms
OVERVIEW = 240                    # columns in a set's picture
CATEGORIES = [("B", "Bass"), ("L", "Lead"), ("P", "Pad"), ("V", "Vocal")]
VERSIONS = "LMH"
FILE = re.compile(r"^_([BLPV]) (.+) ([LMH])\.wav$", re.IGNORECASE)

src = sys.argv[1] if len(sys.argv) > 1 else r"D:\samples\synth\Orthogonal"
root = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..")


def decode(path):
    raw = subprocess.run(["ffmpeg", "-v", "error", "-i", path, "-f", "f32le", "-ac", "2", "-ar", str(RATE), "-"],
                         capture_output=True, check=True).stdout
    return np.frombuffer(raw, np.float32).reshape(-1, 2).astype(np.float64)


def flac(pcm16):
    return subprocess.run(["ffmpeg", "-v", "error", "-f", "s16le", "-ar", str(RATE), "-ac", "1", "-i", "-",
                           "-map_metadata", "-1", "-fflags", "+bitexact", "-c:a", "flac",
                           "-compression_level", "12", "-f", "flac", "-"],
                          input=pcm16.astype("<i2").tobytes(), capture_output=True, check=True).stdout


def rms_db(x):
    return 10 * np.log10(np.mean(x * x) + 1e-30)


# Group the files into sets. A set missing a recording that a set whose name
# starts with its own has (a typo, "Dark Peda L") joins that one.
sets = {}
for f in sorted(os.listdir(src)):
    m = FILE.match(f)
    if not m:
        print(f"skipping {f}")
        continue
    cat, name, version = m.group(1).upper(), m.group(2), m.group(3).upper()
    sets.setdefault((cat, name), {})[version] = os.path.join(src, f)
for key in [k for k, v in sets.items() if len(v) < 3]:
    for other, files in sets.items():
        if other != key and other[0] == key[0] and other[1].startswith(key[1]) \
                and not set(files) & set(sets[key]):
            print(f"'{key[1]}' joins '{other[1]}'")
            files.update(sets.pop(key))
            break
for key, files in sets.items():
    assert len(files) == 3, f"{key[1]} has only {sorted(files)}"

order = sorted(sets, key=lambda k: ([c for c, _ in CATEGORIES].index(k[0]), k[1].lower()))
rng = np.random.default_rng(98)
blob, entries = bytearray(), []
for cat, name in order:
    stereo = {v: decode(sets[(cat, name)][v]) for v in VERSIONS}
    loss = min(10 * np.log10(np.mean(((x[:, 0] + x[:, 1]) / 2) ** 2) / np.mean(x * x)) for x in stereo.values())
    left = loss < MID_LOSS_DB
    recs, length = [], 0
    for v in VERSIONS:
        x = stereo[v][:, 0] if left else stereo[v].mean(axis=1)
        x = x / np.abs(x).max()
        n = len(x) // WINDOW
        env = np.sqrt(np.mean(x[:n * WINDOW].reshape(n, WINDOW) ** 2, axis=1))
        end = min(len(x), (np.nonzero(env > 10 ** (TRIM_DB / 20))[0][-1] + 2) * WINDOW)
        x = x[:end].copy()
        x[-WINDOW:] *= 0.5 + 0.5 * np.cos(np.linspace(0, np.pi, WINDOW))
        dither = (rng.random(len(x)) - rng.random(len(x)))
        pcm = np.clip(np.round(x * 32766 + dither), -32767, 32767)
        stored = pcm / 32767
        gain = 10 ** ((TARGET_RMS_DB - rms_db(stored[:int(HELD_S * RATE)])) / 20)
        stream = flac(pcm)
        recs.append((len(blob), len(stream), len(pcm), gain, stored * gain))
        blob += stream
        length = max(length, len(pcm))
    # The picture: the middle recording's peaks, column by column over the set's length.
    mid = np.abs(recs[1][4])
    cols = np.zeros(OVERVIEW)
    edges = np.linspace(0, length, OVERVIEW + 1).astype(int)
    for c in range(OVERVIEW):
        seg = mid[edges[c]:min(edges[c + 1], len(mid))]
        cols[c] = seg.max() if len(seg) else 0.0
    overview = np.round(255 * cols / max(cols.max(), 1e-9)).astype(int)
    entries.append((cat, name, length, recs, overview, left))
    print(f"{cat} {name:28s} {'left' if left else 'mid '} {length / RATE:5.2f} s  "
          f"gains {' '.join(f'{20 * np.log10(r[3]):+5.1f}' for r in recs)} dB", flush=True)

out = os.path.join(root, "plugin", "src")
with open(os.path.join(out, "ortholib.bin"), "wb") as f:
    f.write(blob)
with open(os.path.join(out, "ortholib_index.rs"), "w", encoding="utf-8") as f:
    f.write("// GENERATED by tools/gen-ortholib.py - do not edit.\n")
    f.write(f"pub const RATE: f32 = {RATE}.0;\n")
    f.write(f"pub const OVERVIEW: usize = {OVERVIEW};\n")
    f.write(f"pub const CATEGORIES: [&str; {len(CATEGORIES)}] = {[n for _, n in CATEGORIES]};\n".replace("'", '"'))
    f.write(f"pub static SETS: [SetInfo; {len(entries)}] = [\n")
    for cat, name, length, recs, overview, left in entries:
        rec = ", ".join(f"Rec {{ start: {r[0]}, bytes: {r[1]}, frames: {r[2]}, gain: {r[3]:.6f} }}" for r in recs)
        f.write(f"    SetInfo {{ name: {json.dumps(name)}, category: {[c for c, _ in CATEGORIES].index(cat)}, frames: {length}, "
                f"recs: [{rec}],\n      overview: {list(map(int, overview))} }},\n")
    f.write("];\n")

menu = ";".join(f"'{name}':{i}" for i, (_, name, *_r) in enumerate(entries))
last = len(entries) - 1
with open(os.path.join(root, "dsp", "ortholib.lib"), "w", encoding="utf-8") as f:
    f.write(f"""// GENERATED by tools/gen-ortholib.py - do not edit.
// {len(entries)} sample sets for the Orthogonal oscillator (played by the plugin;
// set k is plugin/src/ortholib_index.rs's SETS[k]).

sets = {len(entries)};

// An Orthogonal's two sample menus, in its panel's group G, starting on sets a and b.
top(G, a)    = G(nentry("[1]Top [style:menu{{{menu}}}]", a, 0, {last}, 1));
bottom(G, b) = G(nentry("[2]Bottom [style:menu{{{menu}}}]", b, 0, {last}, 1));
""")
print(f"{len(entries)} sets, {len(blob) / 1e6:.1f} MB of FLAC")
