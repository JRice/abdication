"""With a drone going in, how strong are the clock-related tones at the default Time?"""
import subprocess
from capture import *
sr=48000
want=[("C half",2063),("C quarter",1032),("B half",5964),("B quarter",2982),("A half",11300)]
for level in (0.01,0.05,0.2):
    for whine in (1,0):
        subprocess.run(["../plugin/target/release/examples/probe_idle",".1",str(whine),str(WORK/"i.f32"),"1","0",str(level)],check=True)
        y=np.fromfile(WORK/"i.f32","<f4")[48000*15:]
        w=np.hanning(len(y)); S=abs(np.fft.rfft(y*w))*2/w.sum(); fr=np.fft.rfftfreq(len(y),1/sr)
        def near(f): 
            m=(fr>f-25)&(fr<f+25); return float(db(S[m].max()))
        print(f"drone {level:<5} whine {whine} rms {db(rms(y)):6.1f} |"," ".join(f"{n} {near(f):.0f}" for n,f in want), "| median", round(float(db(np.median(S[(fr>500)&(fr<8000)]))),0))
