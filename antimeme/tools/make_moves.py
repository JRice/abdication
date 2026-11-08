"""Export the test signal and the pass's knob moves, for examples/render_pass."""
from capture import *
import sys
def knob(v):
    if "full CW" in v: return 1.0
    if "full CCW" in v: return 0.0
    h = int(v.split(":")[0]); h = h if h > 6 else h+12
    return (h-7)/10.0
state = dict(Blend=0.0, Time=0.5, Decay=0.0, Filter=1.0, Absorb=0.0, Strength=0.0)
rows = [[0.0]+[state[k] for k in state]]
sig = test_signal()
# The last two takes reached the T/F about 16 dB lower on the real unit (Strength fully down).
for name in ("levels-slow", "time-sweep"):
    t = take_named(name); sig[int(t["start"]*48000):int((t["start"]+t["length"])*48000)] *= 10**(-16/20)
np.concatenate([sig, np.zeros(28*48000, sig.dtype)]).astype("<f4").tofile(str(WORK / "signal.f32"))
for t in schedule():
    for k,v in t["set"].items(): state[k]=knob(v)
    s = dict(state)
    # The Time takes ran with Filter closed on the real unit: the recording shows it.
    if t["name"].startswith("time-") and t["name"]!="time-sweep": s["Filter"]=0.0
    rows.append([t["start"]-4]+[s[k] for k in state])
    if t["name"] == "time-sweep":
        # He turned Time over about 30 seconds: approximate it in small steps.
        for i in range(1, 121):
            s["Time"] = 1 - i/120
            rows.append([t["start"]+i*0.25]+[s[k] for k in state])
open(str(WORK / "moves.csv"),"w").write("\n".join(",".join(f"{v:.3f}" for v in r) for r in rows)+"\n")
print(rows[:3], len(rows))
