from capture import *
from scipy.signal import welch
y, sr = recording(); sig = test_signal()
def tone_amp(seg, f0):
    w=np.hanning(len(seg)); S=np.fft.rfft(seg*w); fr=np.fft.rfftfreq(len(seg),1/sr)
    return float(db(2*abs(S[np.argmin(abs(fr-f0))])/w.sum()))
print("--- dry leak: output before the first echo can arrive")
# time-sweep: 1 kHz at -12 dBFS, Time full CW, first echo at 36 ms
t=take_named("time-sweep"); s=t["start"]
print("time-sweep start, 1 kHz, 8..32 ms:", round(tone_amp(segment(y,sr,s+0.008,0.024),1000),1), "dB (input -12)")
# levels takes: each new frequency's first cell is -30 dBFS; each step up is +6 dB, so use chirps for reference
n=int(.15*sr)
def chirp_leak(start):
    a=segment(sig,sr,start,0.03); 
    return a
# Chirp leak in earlier takes, from matched filter at zero lag, already measured about -70.6 for a -6 dBFS chirp
from scipy.signal import fftconvolve, hilbert
tt=np.arange(n)/sr; tpl=np.sin(2*np.pi*(300*tt+(3000-300)*tt*tt/(2*0.15)))*np.hanning(n)
def zero_lag(start):
    seg=segment(y,sr,start-0.01,0.05+0.15); c=fftconvolve(seg,tpl[::-1],mode="valid")/np.dot(tpl,tpl); return float(db(np.abs(hilbert(c)).max()))
for nm in ("time-full-cw","time-3","time-12","time-full-ccw"):
    tk=take_named(nm); print(nm,"chirp leak at zero lag", round(zero_lag(tk["start"]+6),1), "(input -6)")
for nm in ("decay-9","decay-12","loop-filter"):
    tk=take_named(nm); print(nm,"chirp leak at zero lag", round(zero_lag(tk["start"]),1), "(input -6)")
print("--- levels takes: 1 kHz in the window before the first echo, cell by cell")
for nm,first_echo in (("levels-fast",0.034),("levels-fast-strength-12",0.034),("levels-slow",0.19)):
    tk=take_named(nm); row=[]
    for i,lv in enumerate(range(-30,1,6)):
        c=tk["start"]+3.0+i*0.5
        # Only the first cell is clean of its predecessor at 1 kHz; later cells contain the delayed previous level
        row.append((lv, round(tone_amp(segment(y,sr,c+0.005,first_echo-0.008),1000)-lv,1)))
    print(nm,row)
print("--- splice hunt: level in 100 ms steps around the last two takes")
a=take_named("levels-fast-strength-12"); b=take_named("levels-slow")
gap0=a["start"]+a["length"]; print("gap between",round(gap0,2),"and",round(b["start"],2))
for t0 in np.arange(gap0-0.5,b["start"]+1.0,0.5):
    seg=segment(y,sr,t0,0.5); print(f"{t0:8.2f} rms {db(rms(seg)):6.1f} peak {abs(seg).max():.4f}")
print("--- the jumps")
for lo,hi in ((22,36),(822,834)):
    print(f"{lo}-{hi} s, per 0.25 s: rms/peak")
    print(" ".join(f"{t0:.2f}:{db(rms(segment(y,sr,t0,.25))):.0f}/{abs(segment(y,sr,t0,.25)).max():.2f}" for t0 in np.arange(lo,hi,.25)))
