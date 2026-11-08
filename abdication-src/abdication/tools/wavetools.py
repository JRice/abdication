"""Shared helpers for building Abdication's wave library (Python 3, numpy).

A "wave set" is an array of single cycles, shape (frames, 2048), float.
Sets are stored as Serum-style wavetable WAVs (float32 mono, 2048-sample
frames, 'clm ' chunk) in tables/.
"""
import struct
import numpy as np
import scipy.io.wavfile as wavfile

FRAME = 2048


def masked_sine(mask, slices, n=FRAME):
    """One cycle of the bitmask oscillator (same rules as the original
    Faust engine): a sine cut into `slices`; mask bits decide per slice
    whether it is kept, inverted, and time-reversed. Bits repeat every 8."""
    p = np.arange(n) / n
    i = np.floor(p * slices).astype(int)
    bit = lambda k: (mask >> (k % 8)) & 1
    keep, inv, rev = bit(i), bit(i * 3 + 1), bit(i * 5 + 2)
    local = p * slices - i
    q = (i + np.where(rev == 1, 1 - local, local)) / slices
    return keep * np.where(inv == 1, -1, 1) * np.sin(2 * np.pi * q)


def harmonic_profile(waves, harmonics=256):
    """Unit-length harmonic magnitude vectors: phase-free 'what it sounds like'."""
    mags = np.abs(np.fft.rfft(waves, axis=1))[:, 1:harmonics + 1]
    return mags / (np.linalg.norm(mags, axis=1, keepdims=True) + 1e-12)


def distinct(waves, threshold):
    """Indices of waves that differ from every earlier kept wave: a wave is
    a near-duplicate when its harmonic profile matches one at >= threshold."""
    prof = harmonic_profile(waves)
    kept = []
    for k in range(len(waves)):
        if all(prof[k] @ prof[j] < threshold for j in kept):
            kept.append(k)
    return kept


def normalize(waves):
    """Remove DC, start each at phase 0 of its fundamental, peak to 1."""
    out = []
    for w in waves:
        F = np.fft.rfft(w - w.mean())
        F[0] = 0
        shift = np.angle(F[1]) + np.pi / 2 if abs(F[1]) > 1e-9 else 0.0
        F *= np.exp(-1j * np.arange(len(F)) * shift)
        c = np.fft.irfft(F, len(w))
        out.append(c / (np.abs(c).max() + 1e-12))
    return np.array(out)


def write_wavetable(path, waves, label="abdication"):
    """Serum-style wavetable: float32 frames of 2048 + a 'clm ' chunk."""
    wavfile.write(path, 48000, waves.astype(np.float32).ravel())
    data = open(path, "rb").read()
    clm = b"clm " + struct.pack("<I", 48) + f"<!>2048 10000000 wavetable ({label})".encode().ljust(48, b"\0")
    riff = data[:12] + clm + data[12:]
    open(path, "wb").write(riff[:4] + struct.pack("<I", len(riff) - 8) + riff[8:])


def read_wavetable(path):
    sr, x = wavfile.read(path)
    x = np.asarray(x, dtype=np.float64)
    return x.reshape(-1, FRAME)
