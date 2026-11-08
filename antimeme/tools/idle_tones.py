import subprocess
from capture import *
from scipy.signal import welch, find_peaks
from scipy.ndimage import median_filter
real,sr=recording()
bands=[50,100,200,400,800,1600,3200,6400,12800]
def spec(seg):
    f,P=welch(seg,sr,nperseg=8192); return " ".join(f"{10*np.log10(P[np.argmin(abs(f-b))]*f[1]+1e-14):5.0f}" for b in bands)
print("bands",bands)
print("real full CCW open, no input   rms %.1f "%db(rms(segment(real,sr,812,8))),spec(segment(real,sr,812,8)))
for time,name in ((0,"full CCW"),(.1,"8:00"),(.15,"8:30"),(.2,"9:00"),(.25,"9:30"),(.3,"10:00"),(.35,"10:30"),(.4,"11:00"),(.45,"11:30"),(.5,"12:00"),(.6,"1:00")):
    subprocess.run(["../plugin/target/release/examples/probe_idle",str(time),"0",str(WORK/"i.f32"),"1","0"],check=True)
    y=np.fromfile(WORK/"i.f32","<f4")[48000*15:]*.5
    w=np.hanning(len(y)); S=abs(np.fft.rfft(y*w))*2/w.sum(); fr=np.fft.rfftfreq(len(y),1/sr)
    base=median_filter(db(S),size=301); pk,_=find_peaks(db(S)-base,height=15,distance=20); pk=pk[np.argsort(db(S)[pk])[::-1]][:5]
    print(f"model {name:9s} open whine 0 rms {db(rms(y)):.1f} ",spec(y),"| tones"," ".join(f"{fr[p]:.0f}/{db(S[p]):.0f}" for p in pk))
