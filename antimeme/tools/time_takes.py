from capture import *
from scipy.signal import fftconvolve, hilbert, find_peaks, welch, butter, sosfiltfilt
y, sr = recording(); sig = test_signal()
n = int(0.15*sr); t = np.arange(n)/sr
hann = np.hanning(n)
def chirp(f0,f1): return np.sin(2*np.pi*(f0*t+(f1-f0)*t*t/(2*0.15)))*hann
up, down = chirp(300,3000), chirp(3000,300)
def mf(seg, tpl):
    c = fftconvolve(seg, tpl[::-1], mode="valid")/np.dot(tpl,tpl)
    return np.abs(hilbert(c))
for take in schedule():
    if not take["name"].startswith("time-") or take["name"]=="time-sweep": continue
    s = take["start"]; L = take["length"]
    print("==", take["name"], "start", s)
    # self noise: first 5 s of take
    noise = segment(y, sr, s+0.5, 5)
    f, P = welch(noise, sr, nperseg=8192)
    peak = f[np.argmax(P[10:])+10]
    bands = {b: db(np.sqrt(P[(f>=b[0])&(f<b[1])].sum()*(f[1]-f[0]))) for b in [(20,200),(200,1000),(1000,4000),(4000,10000),(10000,24000)]}
    print("  noise rms %.1f dB, peak bin %.0f Hz, bands %s" % (db(rms(noise)), peak, {k:round(v,1) for k,v in bands.items()}))
    # after up chirp at s+6: search with both templates from s+6 to s+L+8
    t0 = s+6
    seg = segment(y, sr, t0, L-6+8)
    for name, tpl, at in (("up",up,0.0),("down",down,8.15)):
        m = mf(seg, tpl)
        thr = max(np.median(m)*6, m.max()*0.15)
        pk,_ = find_peaks(m, height=thr, distance=int(0.02*sr))
        order = pk[np.argsort(m[pk])[::-1]][:10]
        print(f"  {name}: median {db(np.median(m)):.1f} dB:", ", ".join(f"{p/sr-at:.3f}s/{db(m[p]):.1f}" for p in sorted(order)))
