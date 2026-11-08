"""Strega and model side by side, by the same measurements."""
from capture import *
from scipy.signal import fftconvolve, hilbert, find_peaks, welch
real, sr = recording(); sim, _ = simulation(); sig = test_signal()
T_S = take_named("time-sweep")
n = int(0.15*sr); tt = np.arange(n)/sr
tpl = np.sin(2*np.pi*(300*tt+(3000-300)*tt*tt/(2*0.15)))*np.hanning(n)
def echoes(y, start, seconds, rel=0.05):
    seg = segment(y, sr, start, seconds)
    c = fftconvolve(seg, tpl[::-1], mode="valid")/np.dot(tpl,tpl); m=np.abs(hilbert(c))
    pk,_ = find_peaks(m, height=m.max()*rel, distance=int(0.002*sr))
    return " ".join(f"{p/sr*1000:.0f}/{db(m[p]):.0f}" for p in pk[:14])
print("--- echoes (ms/dB), real then model")
for name in ("time-2","time-12","decay-9","decay-12"):
    t = take_named(name); at = t["start"]+(6 if name.startswith("time") else 0)
    print(name); print("  real ", echoes(real,at,2)); print("  model", echoes(sim,at,2))
print("--- ring-down, dB per second")
for name in ("decay-9","decay-12","decay-2","decay-4"):
    t = take_named(name)
    for label,y in (("real",real),("model",sim)):
        print(f"{name:9s}{label:6s}", " ".join(f"{db(rms(segment(y,sr,t['start']+i,1))):4.0f}" for i in range(0,18)))
t = take_named("self-oscillation")
bands=[50,100,200,400,800,1600,3200,6400,12800]
def spec(seg):
    f,P = welch(seg,sr,nperseg=8192)
    return " ".join(f"{10*np.log10(P[np.argmin(abs(f-b))]*f[1]+1e-14):5.0f}" for b in bands)
print("--- spectra at", bands)
for label,y in (("real",real),("model",sim)):
    seg = segment(y,sr,t["start"]+10,10)
    f,P = welch(seg,sr,nperseg=16384); top=[round(float(v)) for v in f[np.argsort(P)[-3:]][::-1]]
    print(f"self-osc {label:6s} rms {db(rms(seg)):5.1f} peak {abs(seg).max():.2f} top {top}", spec(seg))
for label,y,at in (("real",real,812),("model",sim,812)):
    seg = segment(y,sr,at,8); print(f"Time CCW open {label:6s} rms {db(rms(seg)):5.1f}", spec(seg))
for name in ("time-12","time-10","time-9","time-full-ccw"):
    t=take_named(name)
    for label,y in (("real",real),("model",sim)):
        seg=segment(y,sr,t["start"]+0.5,5); print(f"{name:14s} closed {label:6s} rms {db(rms(seg)):5.1f}", spec(seg))
print("--- time sweep, band levels every 5 s, real then model")
for t0 in range(0,40,5):
    for label,y in (("real",real),("model",sim)):
        seg=segment(y,sr,T_S["start"]+t0,5); f,P=welch(seg,sr,nperseg=8192); df=f[1]
        print(f"{t0:3d} {label:6s}", " ".join(f"{10*np.log10(P[(f>=a)&(f<b)].sum()*df+1e-14):5.0f}" for a,b in [(50,200),(200,800),(800,1200),(1200,3000),(3000,6000),(6000,12000),(12000,20000)]))
print("--- levels: fundamental out vs in, dB (-30..0)")
for name in ("levels-fast",):
    t=take_named(name)
    for label,y in (("real",real),("model",sim)):
        seg=segment(y,sr,t["start"],t["length"]); i=0
        for f0 in (100,1000,4000):
            row=[]
            for lv in range(-30,1,6):
                s=seg[int(i*.5*sr)+int(.15*sr):int((i+1)*.5*sr)-int(.05*sr)]; w=np.hanning(len(s))
                S=abs(np.fft.rfft(s*w));fr=np.fft.rfftfreq(len(s),1/sr)
                row.append(round(float(db(2*S[np.argmin(abs(fr-f0))]/w.sum())-lv),1)); i+=1
            print(f"  {label:6s}{f0:5d}", row)
print("--- sweeps: response vs dry-referred, real/model")
cs=[100,300,800,2000,5000,8000,12000,16000]
def resp(y,start):
    a=segment(sig,sr,start,4.6);b=segment(y,sr,start,4.6);w=np.hanning(len(a));A,B=np.fft.rfft(a*w),np.fft.rfft(b*w);f=np.fft.rfftfreq(len(a),1/sr)
    return [round(float(10*np.log10(np.sum(abs(B[(f>=c/2**(1/6))&(f<c*2**(1/6))])**2)/np.sum(abs(A[(f>=c/2**(1/6))&(f<c*2**(1/6))])**2))),1) for c in cs]
print(cs)
for name in ("filter-full-cw","filter-11","absorb-12-filter-12","absorb-full-cw-filter-12","absorb-full-cw-filter-10"):
    t=take_named(name); print(f"{name:26s} real", resp(real,t["start"])); print(f"{'':26s} model", resp(sim,t["start"]))
