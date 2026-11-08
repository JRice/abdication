#!/usr/bin/env python3
"""Generate the Strega capture: one long test signal and its knob schedule.

Writes strega_test.wav (play it into External Substance In), schedule.md (what
to set when) and schedule.json (the same timeline, for the analysis). The
timeline below is the single source of truth for all three.
"""
import json
import wave
from dataclasses import dataclass, field
from pathlib import Path

import numpy as np

SR = 48000
OUT = Path(__file__).parent
SWEEP_LEVEL_DB = -18
CHIRP_LEVEL_DB = -6


def db(x):
    return 10 ** (x / 20)


def silence(seconds):
    return np.zeros(round(seconds * SR))


def hann(n):
    return np.hanning(n) if n > 1 else np.ones(n)


def chirp(f0, f1, seconds=0.15, level_db=CHIRP_LEVEL_DB):
    """A short linear chirp: found by matched filter even under heavy noise."""
    t = np.arange(round(seconds * SR)) / SR
    phase = 2 * np.pi * (f0 * t + (f1 - f0) * t * t / (2 * seconds))
    return db(level_db) * np.sin(phase) * hann(len(t))


UP = chirp(300, 3000)
DOWN = chirp(3000, 300)


def sweep(seconds=4.0, f0=20.0, f1=20000.0, level_db=SWEEP_LEVEL_DB):
    """Exponential sine sweep, low enough in level to stay clear of slew limiting."""
    t = np.arange(round(seconds * SR)) / SR
    k = np.log(f1 / f0)
    y = np.sin(2 * np.pi * f0 * seconds / k * (np.exp(t / seconds * k) - 1))
    fade = np.minimum(1, np.minimum(t, seconds - t) / 0.01)
    return db(level_db) * y * fade


def noise(seconds=2.0, level_db=SWEEP_LEVEL_DB):
    rng = np.random.default_rng(1)
    y = rng.standard_normal(round(seconds * SR)) * db(level_db) / 2
    fade = np.minimum(1, np.minimum(np.arange(len(y)), len(y) - 1 - np.arange(len(y))) / (0.01 * SR))
    return y * fade


def sine(freq, seconds, level_db):
    t = np.arange(round(seconds * SR)) / SR
    fade = np.minimum(1, np.minimum(t, seconds - t) / 0.005)
    return db(level_db) * np.sin(2 * np.pi * freq * t) * fade


def seq(*parts):
    return np.concatenate(parts)


def time_probe():
    return seq(silence(6), UP, silence(8), DOWN, silence(8))


def tone_probe():
    return seq(sweep(), silence(1), noise(), silence(2))


def decay_probe():
    return seq(UP, silence(18))


def level_staircase():
    return seq(*[sine(f, 0.5, level) for f in (100, 1000, 4000) for level in range(-30, 1, 6)])


def dry_probe():
    return seq(silence(0.5), UP, silence(0.85), DOWN, silence(0.85), sweep(), silence(0.5))


@dataclass
class Take:
    name: str
    set: dict
    signal: np.ndarray
    gap: float = 8.0
    note: str = ""
    start: float = field(default=0.0, init=False)
    length: float = field(default=0.0, init=False)


def clock(position):
    return f"{position}:00"


def timeline():
    takes = [Take("dry", {"Blend": "full CCW"}, dry_probe(), gap=20,
                  note="Calibrates your interface's round-trip latency.")]

    # Time: knob positions from shortest delay toward longest.
    takes.append(Take("time-full-cw", {"Blend": "full CW", "Time": "full CW"}, time_probe()))
    for p in (3, 2, 12, 10, 9):
        takes.append(Take(f"time-{p}", {"Time": clock(p)}, time_probe()))
    takes.append(Take("time-full-ccw", {"Time": "full CCW"}, time_probe(), gap=12))

    # Filter, Absorb fully down, at the shortest delay.
    takes.append(Take("filter-7", {"Time": "full CW", "Filter": "full CCW"}, tone_probe()))
    for p in (9, 11, 1, 3):
        takes.append(Take(f"filter-{p}", {"Filter": clock(p)}, tone_probe()))
    takes.append(Take("filter-full-cw", {"Filter": "full CW"}, tone_probe()))

    # Absorb at two Filter settings.
    takes.append(Take("absorb-7-filter-12", {"Filter": "12:00", "Absorb": "full CCW"}, tone_probe()))
    for p in (9, 12, 3):
        takes.append(Take(f"absorb-{p}-filter-12", {"Absorb": clock(p)}, tone_probe()))
    takes.append(Take("absorb-full-cw-filter-12", {"Absorb": "full CW"}, tone_probe()))
    takes.append(Take("absorb-7-filter-10", {"Filter": "10:00", "Absorb": "full CCW"}, tone_probe()))
    for p in (10, 1):
        takes.append(Take(f"absorb-{p}-filter-10", {"Absorb": clock(p)}, tone_probe()))
    takes.append(Take("absorb-full-cw-filter-10", {"Absorb": "full CW"}, tone_probe(), gap=12))

    # Decay: a single chirp recirculating at a medium delay.
    takes.append(Take("decay-9", {"Time": "1:00", "Filter": "full CW", "Absorb": "full CCW", "Decay": "9:00"}, decay_probe()))
    for p in (12, 2, 4):
        takes.append(Take(f"decay-{p}", {"Decay": clock(p)}, decay_probe()))
    takes.append(Take("self-oscillation", {"Decay": "full CW"}, silence(30), gap=12,
                      note="No input. Turn Level down if it gets painful, and tell me you did."))
    takes.append(Take("loop-filter", {"Decay": "2:00", "Filter": "10:00"}, decay_probe()))
    takes.append(Take("loop-absorb", {"Filter": "12:00", "Absorb": "12:00"}, decay_probe(), gap=12))

    # Level: how the input clips and slews.
    takes.append(Take("levels-fast", {"Time": "full CW", "Decay": "full CCW", "Filter": "full CW", "Absorb": "full CCW"}, level_staircase()))
    takes.append(Take("levels-fast-strength-12", {"Strength": "12:00"}, level_staircase()))
    takes.append(Take("levels-slow", {"Strength": "full CCW", "Time": "12:00"}, level_staircase(), gap=10))

    # A slow, continuous Time sweep: maps the knob, and shows the repeats bending.
    takes.append(Take("time-sweep", {"Time": "full CW"}, sine(1000, 40, -12), gap=2,
                      note="As it starts, turn Time smoothly from full CW to full CCW over about 30 seconds, then hold."))
    return takes


def write_wav(path, y, chunk=SR * 10):
    """24-bit mono, written in chunks: this runs on small machines too."""
    with wave.open(str(path), "wb") as w:
        w.setnchannels(1)
        w.setsampwidth(3)
        w.setframerate(SR)
        for start in range(0, len(y), chunk):
            pcm = (np.clip(y[start:start + chunk], -1, 1) * (2 ** 23 - 1)).astype("<i4")
            w.writeframes(pcm.view("u1").reshape(-1, 4)[:, :3].tobytes())


def stamp(seconds):
    return f"{int(seconds // 60)}:{int(seconds % 60):02d}"


def write_schedule(takes):
    lines = ["| Starts | Take | By then, set | Note |", "|---|---|---|---|"]
    for t in takes:
        moves = ", ".join(f"{k} {v}" for k, v in t.set.items())
        lines.append(f"| {stamp(t.start)} | {t.name} | {moves} | {t.note} |")
    (OUT / "schedule.md").write_text("\n".join(lines) + "\n")
    (OUT / "schedule.json").write_text(json.dumps(
        {"sample_rate": SR, "takes": [
            {"name": t.name, "start": t.start, "length": t.length, "set": t.set} for t in takes]},
        indent=1))


def main():
    takes = timeline()
    parts, now = [silence(12)], 12.0
    for t in takes:
        t.start, t.length = now, len(t.signal) / SR
        parts += [t.signal, silence(t.gap)]
        now += t.length + t.gap
    write_wav(OUT / "strega_test.wav", seq(*parts))
    write_schedule(takes)
    print(f"{len(takes)} takes, {stamp(now)} long")


if __name__ == "__main__":
    main()
