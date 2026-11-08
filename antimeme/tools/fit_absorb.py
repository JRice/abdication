from capture import *
from scipy.optimize import least_squares
y, sr = recording(); sig = test_signal()
fc_list = np.array([200,300,420,600,800,1000,1200,1500,2000,2500,3000,4000,5000,6500,8000,10000,12000,16000],float)
def response(start, length=4.6):
    a = segment(sig, sr, start, length); b = segment(y, sr, start, length); w=np.hanning(len(a))
    A,B=np.fft.rfft(a*w),np.fft.rfft(b*w); f=np.fft.rfftfreq(len(a),1/sr)
    return np.array([10*np.log10(np.sum(abs(B[(f>=c/2**(1/6))&(f<c*2**(1/6))])**2)/np.sum(abs(A[(f>=c/2**(1/6))&(f<c*2**(1/6))])**2)) for c in fc_list])
base = response(take_named("filter-full-cw")["start"])
def model(f, fc, Q, h, b):
    s = 1j*f/fc
    D = s*s+s/Q+1
    return 20*np.log10(abs((1 + h*s*s + b*s/Q)/D))
def FILTER_HZ(k): return 60*(330**k)
for name, flt, ab in [("filter-7",0,0),("filter-9",.2,0),("filter-11",.4,0),("filter-1",.6,0),("filter-3",.8,0),
    ("absorb-7-filter-12",.5,0),("absorb-9-filter-12",.5,.2),("absorb-12-filter-12",.5,.5),("absorb-3-filter-12",.5,.8),("absorb-full-cw-filter-12",.5,1),
    ("absorb-7-filter-10",.3,0),("absorb-10-filter-10",.3,.3),("absorb-1-filter-10",.3,.6),("absorb-full-cw-filter-10",.3,1)]:
    d = response(take_named(name)["start"])-base
    def err(p): return model(fc_list,*p)-d
    fit = least_squares(err,[FILTER_HZ(flt),0.7,0.01,0.1],bounds=([20,0.3,0,0],[24000,6,4,12]))
    print(f"{name:28s} fc {fit.x[0]:7.0f} (law {FILTER_HZ(flt):6.0f})  Q {fit.x[1]:.2f}  h {fit.x[2]:.3f}  b {fit.x[3]:.3f}  err {np.sqrt(np.mean(fit.fun**2)):.1f} dB")
