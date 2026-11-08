"""Render audition WAVs of the ABDrum voice offline (no plugin needed).

    python tools/render.py <output folder>

Regenerates the Faust C++, builds tools/render/render.cpp against it, plays
each preset in tools/presets.py (a loud and a soft hit, or a preset's own
sequence), and writes one 24-bit WAV per preset with a short report.
Needs the native Faust compiler and g++."""
import json
import os
import subprocess
import sys

import wave

import numpy as np

sys.path.insert(0, os.path.dirname(__file__))
from presets import PRESETS  # noqa: E402

RATE = 44100
BUILD = "build"


def build():
    os.makedirs(BUILD, exist_ok=True)
    subprocess.run(["faust", "-I", "dsp", "-cn", "VoiceDsp", "dsp/plugin-voice.dsp", "-o", f"{BUILD}/voice.cpp"], check=True)
    subprocess.run(["faust", "-I", "dsp", "-cn", "FxDsp", "dsp/plugin-fx.dsp", "-o", f"{BUILD}/fx.cpp"], check=True)
    subprocess.run(["faust", "-I", "dsp", "-cn", "BodyDsp", "dsp/plugin-body.dsp", "-o", f"{BUILD}/body.cpp"], check=True)
    subprocess.run(["faust", "-I", "dsp", "-cn", "NoiseDsp", "dsp/plugin-noise.dsp", "-o", f"{BUILD}/noise.cpp"], check=True)
    subprocess.run(["g++", "-O2", "-std=c++17", "-I", BUILD, "tools/render/render.cpp", "-o", f"{BUILD}/render"], check=True)
    index = json.load(open("plugin/src/samplelib.json"))
    global LIBRARY_INDEX
    LIBRARY_INDEX = index
    with open(f"{BUILD}/samplelib_index.txt", "w") as f:
        f.write("\n".join([f"L {s} {n}" for s, n in zip(index["starts"], index["lens"])]
                           + [f"H {s} {n}" for s, n in zip(index["hit_starts"], index["hit_lens"])]))
    return index["labels"]


def write_wav24(path, x):
    pcm = (np.clip(x, -1, 1) * (2**23 - 1)).astype("<i4")
    frames = np.frombuffer(pcm.tobytes(), np.uint8).reshape(-1, 4)[:, :3].tobytes()
    with wave.open(path, "wb") as w:
        w.setnchannels(1)
        w.setsampwidth(3)
        w.setframerate(RATE)
        w.writeframes(frames)


def loop_index(labels, name):
    matches = [i for i, label in enumerate(labels) if name in label]
    assert len(matches) == 1, f"loop '{name}': {len(matches)} matches"
    return matches[0]


def resolve(path, value, labels):
    """Presets name loops and hits; the voice wants numbers. Yields (path, value)."""
    if path == "Noise/Loop":
        yield path, loop_index(labels, value)
    elif path.endswith("/Hit"):                  # ("knock", "Jade 018") -> Set + Sample
        group, name = value
        sets = LIBRARY_INDEX["sets"]; s = sets.index(group)
        first, count = LIBRARY_INDEX["set_first"][s], LIBRARY_INDEX["set_counts"][s]
        names = LIBRARY_INDEX["hit_labels"][first:first + count]
        matches = [i for i, n in enumerate(names) if name in n]
        assert len(matches) == 1, f"hit '{name}' in {group}: {len(matches)} matches"
        layer = path.rsplit("/", 1)[0]
        yield f"{layer}/Set", s
        yield f"{layer}/Sample", matches[0] + 1
    else:
        yield path, value


def resolved(values, labels):
    return [pv for path, value in values.items() for pv in resolve(path, value, labels)]


def script(preset, labels):
    lines = []
    for path, value in resolved(preset["set"], labels):
        lines.append(f"set /ABDrum/{path.replace(' ', '_')} {value}")
    rng = np.random.default_rng(len(preset["name"]))
    for hit in preset.get("hits", [(1.0, 1.2), (0.6, 1.2)]):
        velocity, seconds, *changes = hit
        for path, value in resolved(changes[0] if changes else {}, labels):
            lines.append(f"set /ABDrum/{path.replace(' ', '_')} {value}")
        lines.append(f"hit {velocity} {rng.random():.6f} {seconds}")
    return lines


def render(lines, raw):
    subprocess.run([f"{BUILD}/render", "plugin/src/samplelib.bin", f"{BUILD}/samplelib_index.txt"],
                   input="\n".join(lines + [f"write {raw}"]), text=True, check=True)
    return np.fromfile(raw, dtype="<f4").astype(np.float64)


def report(x):
    peak = np.abs(x).max()
    first = x[: int(0.6 * RATE)]
    env = 20 * np.log10(np.sqrt(np.convolve(first**2, np.ones(441) / 441, "same")) + 1e-9)
    ip = int(np.argmax(env))
    fall = lambda db: next((i - ip for i in range(ip, len(env)) if env[i] < env[ip] - db), None)
    t20, t40 = fall(20), fall(40)
    ms = lambda n: f"{n / RATE * 1000:.0f}" if n is not None else ">600"
    return f"peak {20 * np.log10(peak + 1e-12):6.1f} dBFS  -20 dB at {ms(t20)} ms, -40 dB at {ms(t40)} ms"


def main(out_dir):
    labels = build()
    os.makedirs(out_dir, exist_ok=True)
    for n, preset in enumerate(PRESETS, 1):
        x = render(script(preset, labels), f"{BUILD}/out.raw")
        assert np.isfinite(x).all(), f"{preset['name']}: non-finite output"
        name = f"{n:02d} {preset['name']}.wav"
        write_wav24(os.path.join(out_dir, name), x)
        print(f"{name:34} {report(x)}")


if __name__ == "__main__":
    main(sys.argv[1])
