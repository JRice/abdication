from capture import *
from scipy.signal import welch, find_peaks, fftconvolve
y, sr = recording()
def tones(seg, label):
    f,P = welch(seg,sr,nperseg=1<<15)
    Pdb = 10*np.log10(P+1e-20)
    med = np.median(Pdb)
    from scipy.ndimage import median_filter
    base = median_filter(Pdb, size=201)
    pk,_ = find_peaks(Pdb-base, height=10, distance=8)
    pk = pk[np.argsort((Pdb-base)[pk])[::-1]][:14]
    print(label, "rms", round(db(rms(seg)),1), "tones:", " ".join(f"{f[p]:.0f}Hz/{Pdb[p]-base[p]:.0f}" for p in sorted(pk)))
tones(segment(y,sr,812,10),"Time full CCW, filter open")
for n in ("time-full-ccw","time-9","time-10"):
    t = take_named(n); tones(segment(y,sr,t["start"]+0.5,5), n+" (filter closed)")
# Time sweep end portion, and any time with filter open
print("-- Noise under the sine in the time sweep (bands, per 2.5 s)")
t = take_named("time-sweep")
for t0 in np.arange(0,40,5.0):
    seg = segment(y,sr,t["start"]+t0,5)
    f,P = welch(seg,sr,nperseg=8192); df=f[1]
    print(f"{t0:4.0f}s", " ".join(f"{a}-{b}:{10*np.log10(P[(f>=a)&(f<b)].sum()*df+1e-14):.0f}" for a,b in [(50,200),(200,800),(800,1200),(1200,3000),(3000,6000),(6000,12000),(12000,20000)]))
print("-- decay path tone: deconvolve the chirp at Decay 9:00")
n = int(0.15*sr); tt = np.arange(n)/sr
tpl = 0.5*np.sin(2*np.pi*(300*tt+(3000-300)*tt*tt/(2*0.15)))*np.hanning(n)
take = take_named("decay-9"); seg = segment(y,sr,take["start"],3)
N = len(seg); X=np.fft.rfft(tpl,N); Y=np.fft.rfft(seg,N); H=Y*np.conj(X)/(abs(X)**2+abs(X).max()**2*1e-3)
h=np.fft.irfft(H,N); np.save(str(WORK / "h_d9.npy"),h)
def tone(ms,half=6.0):
    c=int(ms/1000*sr);w=int(half/1000*sr); s=h[c-w:c+w]*np.hanning(2*w);S=np.fft.rfft(s,8192);f=np.fft.rfftfreq(8192,1/sr)
    return " ".join(f"{fc}:{db(abs(S[np.argmin(abs(f-fc))])):.1f}" for fc in (400,600,800,1000,1400,2000,2800))
for ms in (98,174,196,272,294,332,349,370,430,446): print(ms,tone(ms))
