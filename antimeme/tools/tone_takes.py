from capture import *
y, sr = recording(); sig = test_signal()
freqs = [30,50,80,120,200,300,500,800,1200,2000,3000,5000,8000,12000,16000]
def response(start, length=4.6):
    a = segment(sig, sr, start, length); b = segment(y, sr, start, length)
    w = np.hanning(len(a))
    A, B = np.fft.rfft(a*w), np.fft.rfft(b*w)
    f = np.fft.rfftfreq(len(a), 1/sr)
    out=[]
    for fc in freqs:
        m = (f>=fc/2**(1/6))&(f<fc*2**(1/6))
        out.append(10*np.log10(np.sum(abs(B[m])**2)/np.sum(abs(A[m])**2)+1e-14))
    return out
print("take".ljust(28)+"".join(f"{f:7d}" for f in freqs))
d = take_named("dry")
print("dry".ljust(28)+"".join(f"{v:7.1f}" for v in response(d["start"]+2.3, 4.2)))
for take in schedule():
    if take["name"].startswith(("filter-","absorb-")):
        print(take["name"].ljust(28)+"".join(f"{v:7.1f}" for v in response(take["start"])), " noiseburst rms %.1f"%db(rms(segment(y,sr,take["start"]+5,2))))
