from capture import *
from scipy.signal import welch
import subprocess
y, sr = recording()
def tone_amp(seg,f0):
    w=np.hanning(len(seg)); S=np.fft.rfft(seg*w); fr=np.fft.rfftfreq(len(seg),1/sr)
    return float(db(2*abs(S[np.argmin(abs(fr-f0))])/w.sum()))
print("--- does the leak follow Strength? 100 Hz at -30 dBFS, first cell of each take, before any echo")
for nm,win in (("levels-fast",.028),("levels-fast-strength-12",.028),("levels-slow",.18)):
    t=take_named(nm); print(nm, round(tone_amp(segment(y,sr,t["start"]+.005,win),100)+30,1),"dB re input")
bands=[50,100,200,400,800,1600,3200,6400]
def spec(seg):
    f,P=welch(seg,sr,nperseg=4096); return " ".join(f"{b}:{10*np.log10(P[np.argmin(abs(f-b))]*f[1]+1e-14):.0f}" for b in bands)
print("--- real jumps"); 
for a,b in ((26,1),(34.0,.25),(825,2),(827.75,1),(828.75,1),(830.25,1)): print(a, round(db(rms(segment(y,sr,a,b))),1), spec(segment(y,sr,a,b)))
print("--- model: silent input, Time held long, then moved")
def run(label, times):
    # times: list of (seconds, time knob) steps; Decay down, Filter open, Blend full
    total=times[-1][0]+6
    np.zeros(int(total*sr),"<f4").tofile(str(WORK/"z.f32"))
    rows=[]
    for (t0,k0),(t1,k1) in zip(times,times[1:]):
        n=max(1,int((t1-t0)/.02))
        for i in range(n): rows.append([t0+(t1-t0)*i/n,1,k0+(k1-k0)*i/n,0,1,0,0])
    rows.append([times[-1][0],1,times[-1][1],0,1,0,0])
    open(WORK/"jm.csv","w").write("\n".join(",".join(f"{v:.3f}" for v in r) for r in rows)+"\n")
    subprocess.run(["../plugin/target/release/examples/render_pass",str(WORK/"z.f32"),str(WORK/"jm.csv"),str(WORK/"jo.f32")],check=True)
    o=np.fromfile(WORK/"jo.f32","<f4"); print(label)
    print("  ", " ".join(f"{t:.0f}:{db(rms(o[int(t*sr):int((t+.5)*sr)])):.0f}/{abs(o[int(t*sr):int((t+.5)*sr)]).max():.2f}" for t in np.arange(8,total-.5,.5)))
run("CCW to CW in 0.5 s", [(0,0),(10,0),(10.5,1)])
run("CCW to 12:00 in 0.5 s", [(0,0),(10,0),(10.5,.5)])
run("CCW to CW over 10 s", [(0,0),(10,0),(20,1)])
run("CW to CCW in 0.5 s", [(0,1),(10,1),(10.5,0)])
