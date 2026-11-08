from capture import *
from scipy.signal import fftconvolve, hilbert, find_peaks, welch
y, sr = recording()
n = int(0.15*sr); t = np.arange(n)/sr
tpl = np.sin(2*np.pi*(300*t+(3000-300)*t*t/(2*0.15)))*np.hanning(n)
def peaks(seg, top=30, rel=0.02):
    c = fftconvolve(seg, tpl[::-1], mode="valid")/np.dot(tpl,tpl)
    m = np.abs(hilbert(c))
    pk,_ = find_peaks(m, height=m.max()*rel, distance=int(0.002*sr))
    return m, pk
for name in ["decay-9","decay-12","decay-2","decay-4","loop-filter","loop-absorb"]:
    take = take_named(name)
    seg = segment(y, sr, take["start"], 18)
    m, pk = peaks(seg)
    print("==",name, "before", round(db(rms(segment(y,sr,take["start"]-2,2))),1), "floor", round(db(np.median(m)),1), "tail(last 4s)", round(db(rms(seg[-4*sr:])),1))
    print("  ", " ".join(f"{p/sr*1000:.0f}/{db(m[p]):.1f}" for p in pk[:28]))
    f,P = welch(seg[int(1*sr):int(4*sr)], sr, nperseg=4096)
    print("   spectrum dB:", " ".join(f"{fc}:{10*np.log10(P[np.argmin(abs(f-fc))]*f[1]+1e-14):.0f}" for fc in [50,100,200,400,800,1600,3200,6400,12800]))
print("== self-oscillation")
take = take_named("self-oscillation")
for t0 in range(0,40,5):
    seg = segment(y,sr,take["start"]+t0,5)
    f,P = welch(seg,sr,nperseg=16384)
    top = f[np.argsort(P)[-5:]][::-1]
    print(f"  {t0:3d}s rms {db(rms(seg)):.1f} peak {abs(seg).max():.3f} top {[round(float(v)) for v in top]}", " ".join(f"{fc}:{10*np.log10(P[np.argmin(abs(f-fc))]*f[1]+1e-14):.0f}" for fc in [50,100,200,400,800,1600,3200,6400,12800]))
