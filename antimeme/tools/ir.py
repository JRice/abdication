"""Impulse responses from the sweeps: tap times, gains and each tap's tone."""
from capture import *
from scipy.signal import find_peaks
y, sr = recording(); sig = test_signal()
def impulse_response(start, seconds=4.0, tail=1.0):
    n = int((seconds+tail)*sr)
    a = sig[int(start*sr):int(start*sr)+int(seconds*sr)]; b = y[int(start*sr):int(start*sr)+n]
    A = np.fft.rfft(a, n); B = np.fft.rfft(b, n)
    eps = (abs(A)**2).max()*1e-4
    return np.fft.irfft(B*np.conj(A)/(abs(A)**2+eps), n)
def show(name, start):
    h = impulse_response(start)
    env = abs(h)
    pk,_ = find_peaks(env, height=env.max()*0.04, distance=int(0.001*sr))
    print(name, "peak", round(float(env.max()),3), " ".join(f"{p/sr*1000:.1f}/{db(env[p]):.1f}" for p in pk[:40]))
    return h
d = take_named("dry"); show("dry", d["start"]+2.5)
for n in ["filter-full-cw","filter-3","absorb-full-cw-filter-12","absorb-12-filter-12"]:
    h = show(n, take_named(n)["start"])
# tone per tap, filter-full-cw
h = impulse_response(take_named("filter-full-cw")["start"])
np.save(str(WORK / "h_ffc.npy"), h)
def tap_tone(h, t_ms, half=6.0):
    c = int(t_ms/1000*sr); w = int(half/1000*sr)
    seg = h[c-w:c+w]*np.hanning(2*w); S = np.fft.rfft(seg, 8192); f = np.fft.rfftfreq(8192,1/sr)
    return [round(float(db(abs(S[np.argmin(abs(f-fc))]))),1) for fc in (200,500,1000,2000,4000,6000,8000,10000,14000,18000)]
print("freqs 200,500,1k,2k,4k,6k,8k,10k,14k,18k")
for t in (35.8,48.4,71.5,96.7,107.4): print(t, tap_tone(h,t))
