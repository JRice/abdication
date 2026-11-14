"""Builds Argent's recorded tables (`assets/recorded.bin`) from recordings of transwave
sweeps, such as the Phizmo sweeps in `argent/fizmo waves`.

    python plugin/tools/recorded.py "fizmo waves" plugin/assets/recorded.bin

Each recording is one held note whose wave holds, sweeps through a few others, and holds
again. For each one this finds:

- the pitch: C1 or C2 (whichever the recording's lowest line says), then refined until
  the harmonics' phases hold steady from cycle to cycle;
- every single cycle's harmonics, from windows exactly one period long, up to 16 kHz
  (the MP3s keep nothing above that);
- the keyframes: the held first wave (averaged over the time it holds), the turning
  points of the sweep (where a straight blend between its ends strays furthest, at most
  three), and the held last wave.

Keys keep their magnitudes (RMS over the cycles used) and phases (of the averaged cycle).
Argent spreads them evenly across a 64-frame table and glides between them.

Asset layout, little-endian: b"ARGR", u16 version 1, u16 count; then per table: u8 name
length, name (UTF-8), u16 harmonics, u8 keys, and per key per harmonic: f32 magnitude,
f32 phase. Each table's loudest key has an RMS of 1. Tables are in the order of the
leading number in each file name; the name is the rest of it.

Needs numpy, scipy and ffmpeg (for MP3s).
"""

import re
import struct
import subprocess
import sys
import tempfile
from pathlib import Path

import numpy as np
from scipy.io import wavfile

SR = 44_100
C1, C2 = 32.703, 65.406
TOP_HZ = 16_000.0
SETTLE_S = 0.06
HELD = 0.08          # how close a held wave stays to itself (distance between shapes)
TURN = 0.12          # how far a sweep strays from a straight blend to count as a turn
MAX_TURNS = 3


def load(path: Path) -> np.ndarray:
    with tempfile.TemporaryDirectory() as tmp:
        wav = Path(tmp) / "x.wav"
        subprocess.run(["ffmpeg", "-loglevel", "error", "-y", "-i", str(path), "-ac", "1", "-ar", str(SR), str(wav)], check=True)
        _, x = wavfile.read(wav)
    return x.astype(float) / 32768.0


def cycles(x: np.ndarray, f0: float, harmonics: int, periods: int = 1):
    """Complex harmonic amplitudes over windows of whole periods (so harmonics are orthogonal)."""
    length = periods * SR / f0
    k = np.arange(1, harmonics + 1)
    out, times, start = [], [], 0.0
    while start + length + 1 < len(x):
        i0, n = int(start), int(round(length))
        t = (np.arange(n) + i0) / SR
        out.append(2 * np.exp(-2j * np.pi * np.outer(k, f0 * t)).dot(x[i0:i0 + n]) / n)
        times.append((start + length / 2) / SR)
        start += length
    return np.array(times), np.array(out)


def pitch(x: np.ndarray) -> float:
    tail = x[len(x) // 2:]
    spectrum = np.abs(np.fft.rfft(tail * np.hanning(len(tail)), 1 << 20))
    freqs = np.fft.rfftfreq(1 << 20, 1 / SR)
    line = lambda hz: spectrum[np.searchsorted(freqs, hz)]
    guess = C1 if line(C1) > 0.1 * line(C2) else C2
    best = (guess, -1.0)
    for f in np.arange(guess * 0.985, guess * 1.015, guess * 0.0002):
        _, F = cycles(tail, f, 40, periods=4)
        steady = np.abs(F.mean(axis=0)).sum() / (np.abs(F).mean(axis=0).sum() + 1e-12)
        if steady > best[1]:
            best = (f, steady)
    return best[0]


def shape(F: np.ndarray) -> np.ndarray:
    m = np.abs(F)
    return m / (np.linalg.norm(m, axis=-1, keepdims=True) + 1e-12)


def turns(S: np.ndarray, lo: int, hi: int, found: list):
    """Douglas-Peucker on the path of shapes: where the sweep strays from a straight blend."""
    if hi - lo < 2:
        return
    u = np.linspace(0, 1, hi - lo + 1)[1:-1, None]
    d = np.linalg.norm(S[lo + 1:hi] - ((1 - u) * S[lo] + u * S[hi]), axis=1)
    i = int(np.argmax(d))
    if d[i] > TURN:
        found.append((d[i], lo + 1 + i))
        turns(S, lo, lo + 1 + i, found)
        turns(S, lo + 1 + i, hi, found)


def key(F: np.ndarray) -> np.ndarray:
    return np.sqrt((np.abs(F) ** 2).mean(axis=0)) * np.exp(1j * np.angle(F.mean(axis=0)))


def keys_of(x: np.ndarray):
    f0 = pitch(x)
    harmonics = int(TOP_HZ / f0)
    t, F = cycles(x, f0, harmonics)
    S = shape(F)
    first = int(np.searchsorted(t, SETTLE_S))
    start_shape = S[first:first + 4].mean(axis=0)
    a = first
    while a + 1 < len(S) and np.linalg.norm(S[a + 1] - start_shape) < HELD:
        a += 1
    end_shape = S[-4:].mean(axis=0)
    b = len(S) - 1
    while b - 1 > a and np.linalg.norm(S[b - 1] - end_shape) < HELD:
        b -= 1
    found = []
    turns(S, a, b, found)
    inside = sorted(i for _, i in sorted(found, reverse=True)[:MAX_TURNS])
    keys = [key(F[first:a + 1])] + [key(F[max(i - 1, 0):i + 2]) for i in inside] + [key(F[b:])]
    return f0, harmonics, keys


def main(folder: str, out_path: str):
    files = sorted(Path(folder).glob("*.mp3"), key=lambda p: int(p.name.split()[0]))
    out = bytearray(b"ARGR") + struct.pack("<HH", 1, len(files))
    for path in files:
        f0, harmonics, keys = keys_of(load(path))
        loudest = max(np.sqrt((np.abs(k) ** 2).sum() / 2) for k in keys)
        name = re.sub(r"^\d+ \d+ ", "", path.stem).encode()
        out += struct.pack("<B", len(name)) + name + struct.pack("<HB", harmonics, len(keys))
        for k in keys:
            for m, p in zip(np.abs(k) / loudest, np.angle(k)):
                out += struct.pack("<ff", m, p)
        print(f"{path.stem:28s} {f0:7.3f} Hz, {harmonics} harmonics, {len(keys)} keys")
    Path(out_path).write_bytes(out)
    print(f"{len(out)} bytes -> {out_path}")


if __name__ == "__main__":
    main(*sys.argv[1:3])
