"""Shared loading and timing for the Strega capture analysis."""
import json
import os
import tempfile
import wave
from pathlib import Path

import numpy as np

# The capture folder: set ANTIMEME_CAPTURE if it is not next to this one.
UPLOADS = Path(os.environ.get("ANTIMEME_CAPTURE", Path(__file__).parent.parent / "capture"))

# Renders and other working files.
WORK = Path(tempfile.gettempdir()) / "antimeme"
WORK.mkdir(exist_ok=True)


def load_wav(path):
    with wave.open(str(path)) as w:
        channels, width, rate, n = w.getnchannels(), w.getsampwidth(), w.getframerate(), w.getnframes()
        raw = w.readframes(n)
    b = np.frombuffer(raw, "u1").reshape(-1, width)
    x = (b[:, 0].astype(np.int32) | b[:, 1].astype(np.int32) << 8 | b[:, 2].astype(np.int32) << 16)
    x = np.where(x >= 2 ** 23, x - 2 ** 24, x).astype(np.float32) / 2 ** 23
    return x.reshape(-1, channels), rate


def schedule():
    return json.loads((UPLOADS / "schedule.json").read_text())["takes"]


def db(x):
    return 20 * np.log10(np.maximum(x, 1e-12))


def rms(x):
    return float(np.sqrt(np.mean(np.square(x))))


LATENCY = 44  # samples of interface round trip, measured from the dry take


def recording():
    x, sr = load_wav(UPLOADS / "recordings/pass1.wav")
    return x[LATENCY:, 0], sr


def simulation(path=str(WORK / "sim.f32")):
    return np.fromfile(path, "<f4"), 48000


def test_signal():
    return load_wav(UPLOADS / "strega_test.wav")[0][:, 0]


def take_named(name):
    return next(t for t in schedule() if t["name"] == name)


def segment(y, sr, start, seconds):
    return y[int(start * sr):int((start + seconds) * sr)]
