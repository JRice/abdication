"""Build the ABDrum audition library from source material.

    python build_library.py transients
    python build_library.py noise
    python build_library.py add-render "<file.wav>" <hit count> "<name>"
    python build_library.py conform

conform brings every library file to mono / 44.1 kHz / 24-bit, trims transients
(onset at -50 dB, tail below -60 dB) and peak-normalizes them, level-matches noise,
and turns noise files it hasn't seen before into seamless loops. Originals of
changed files are copied to abdrum library/.originals/ first.

add-render slices an evenly spaced multi-hit render, peak-normalizes each hit, and files
it into the nearest transient group.

Each group folder is written once and then left alone: a group whose `.complete`
marker exists is skipped, and existing files are never overwritten. After that the
folders are hand-curated; add and delete files freely.
"""
import hashlib, os, re, shutil, subprocess, sys, tempfile
import numpy as np

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
DRUMS, NOISE_SRC = os.path.join(ROOT, "drum samples"), os.path.join(ROOT, "noise samples")
LIB = os.path.join(ROOT, "abdrum library")
SR = 44100
ABDICATION_RENDERS = {"Jade": "Jade Transients.wav", "Mech": "Mech Transients.wav"}

# Curation decisions, kept as data. Matched by substring against source file names.
NOISE = {
    "steady":    ["SOURCE 018 ", "SOURCE 023 ", "SOURCE 028 ", "SOURCE 145 ", "SOURCE 148 ", "SOURCE 149 ",
                  "SOURCE 156 ", "SOURCE 192 ", "SOURCE 197 ", "SOURCE 214 ", "SOURCE 232B "],
    "crackle":   ["SP02", "SP03", "SP04", "SP05", "SP16", "SP17", "SP22", "SP23", "SOURCE 161 "],
    "modulated": ["SOURCE 158 ", "SOURCE 159 ", "SOURCE 165 ", "SOURCE 235A ", "SP13", "SP18", "SP19", "SP20", "SP21",
                  "SP24", "SP25", "SP26", "SP27", "SP28", "SP29", "SP30", "SP31", "SP32", "SP33"],
}
NOISE_LOOP_S = {"steady": 1.0, "crackle": 4.0, "modulated": 4.0}
HAT_PREFIXES = ("CH ", "OH ", "HH", "Shaker")
BODY_PREFIXES = ("BD ", "Tom ", "Bass ")   # heads that carry the start of their own body: high-passed
MULTI_HIT_RENDERS = {"Addictive Mallets.wav"}   # in drum samples/, but added via add-render, not as heads
TRANSIENT_GROUPS = ["thump", "knock", "spark"]

# ---------- io ----------
def load(path, start_s=0.0, dur_s=None):
    cmd = ["ffmpeg", "-v", "error", "-ss", str(start_s), "-i", path] + (["-t", str(dur_s)] if dur_s else [])
    raw = subprocess.run(cmd + ["-map", "0:a:0", "-f", "f32le", "-ac", "1", "-ar", str(SR), "-"], capture_output=True, check=True).stdout
    return np.frombuffer(raw, np.float32).astype(np.float64)

def encode(x):
    with tempfile.TemporaryDirectory() as d:
        p = os.path.join(d, "x.wav"); save(p, x); return open(p, "rb").read()

def save(path, x):
    if os.path.exists(path): return
    os.makedirs(os.path.dirname(path), exist_ok=True)
    subprocess.run(["ffmpeg", "-v", "error", "-f", "f32le", "-ar", str(SR), "-ac", "1", "-i", "-", "-c:a", "pcm_s24le", path],
                   input=x.astype(np.float32).tobytes(), check=True)

def group_done(group_dir): return os.path.exists(os.path.join(group_dir, ".complete"))
def mark_done(group_dir): os.makedirs(group_dir, exist_ok=True); open(os.path.join(group_dir, ".complete"), "w").close()

# ---------- dsp ----------
def db(x): return 10*np.log10(x + 1e-20)
def normalize_peak(x, dbfs=-1.0): return x * (10**(dbfs/20) / (np.abs(x).max() + 1e-12))
def fade_out(x, ms=5.0):
    n = min(len(x), int(SR*ms/1000)); y = x.copy(); y[-n:] *= np.cos(np.linspace(0, np.pi/2, n)); return y

def highpass(x, hz, q=0.707):
    w = 2*np.pi*hz/SR; a = np.sin(w)/(2*q); c = np.cos(w)
    b0, b1, b2, a0, a1, a2 = (1+c)/2, -(1+c), (1+c)/2, 1+a, -2*c, 1-a
    y = np.zeros_like(x); x1 = x2 = y1 = y2 = 0.0
    for i, v in enumerate(x):
        y[i] = (b0*v + b1*x1 + b2*x2 - a1*y1 - a2*y2)/a0; x2, x1, y2, y1 = x1, v, y1, y[i]
    return y

def spectrum(seg, n=4096):
    return np.abs(np.fft.rfft(seg*np.hanning(len(seg)), n))**2 + 1e-20, np.fft.rfftfreq(n, 1/SR)

def hf_share_db(seg, above=1000):
    m, f = spectrum(seg); return db(m[f > above].sum()/m.sum())

def features(seg):
    m, f = spectrum(seg); tot = m.sum(); band = (f > 300) & (f < 16000)
    env = np.convolve(np.abs(seg), np.ones(44)/44, "same"); ip = int(np.argmax(env))
    below = np.where(env[ip:] < env[ip]*0.1)[0]; decay_ms = (below[0] if len(below) else len(seg) - ip)/SR*1000
    return np.array([np.log2((f*m).sum()/tot), np.exp(np.log(m[band]).mean())/m[band].mean(),
                     np.log10(m[f < 300].sum()/tot + 1e-4), np.log10(m[f > 6000].sum()/tot + 1e-4), np.log2(decay_ms + 1)])

def frame_db(x, hop=44):
    n = len(x)//hop; return db((x[:n*hop].reshape(n, hop)**2).mean(1))

def segments(x, thresh_db=-50, gap_frames=30, hop=44):
    on = frame_db(x, hop) > thresh_db; out, i = [], 0
    while i < len(on):
        if not on[i]: i += 1; continue
        j = i
        while j < len(on) and on[j:j+gap_frames].any(): j += 1
        out.append((i*hop, j*hop)); i = j
    return out

def kmeans(Z, k, restarts=30, seed=1):
    rng, best = np.random.default_rng(seed), None
    for _ in range(restarts):
        C = Z[rng.choice(len(Z), k, replace=False)]
        for _ in range(300):
            L = np.argmin(((Z[:, None] - C[None])**2).sum(2), 1)
            Cn = np.array([Z[L == j].mean(0) if (L == j).any() else C[j] for j in range(k)])
            if np.allclose(Cn, C): break
            C = Cn
        inertia = ((Z - C[L])**2).sum()
        if best is None or inertia < best[0]: best = (inertia, C)
    return best[1]

# ---------- transients ----------
def clean_name(fn): return re.sub(r"(-\d+)+$", "", os.path.splitext(fn)[0]).strip()

def abdication_transients():
    for tag, fn in ABDICATION_RENDERS.items():
        x = load(os.path.join(ROOT, fn)); x /= np.abs(x).max()
        for i, (a, b) in enumerate(segments(x)):
            yield f"{tag} {i+1:03d}", fade_out(x[max(a-22, 0):b+441])

def drum_heads(head_ms=12, body_ms=45):
    T, B = int(SR*head_ms/1000), int(SR*body_ms/1000)
    for fn in sorted(set(os.listdir(DRUMS)) - MULTI_HIT_RENDERS):
        x = load(os.path.join(DRUMS, fn)); x /= np.abs(x).max() + 1e-12; on = int(np.argmax(np.abs(x) > 0.02))
        head, body = x[on:on+T], x[on+T:on+T+B]
        if len(head) < T: continue
        is_hat = fn.startswith(HAT_PREFIXES)
        click_only = len(body) < 512 or np.sqrt((body**2).mean()) < 0.03
        separable = hf_share_db(head) > -25 and (click_only or hf_share_db(head) - hf_share_db(body) > 6)
        if not (is_hat or separable): continue
        raw = fade_out(head)
        final = fade_out(highpass(head, 500)) if fn.startswith(BODY_PREFIXES) else raw
        yield clean_name(fn) + " head", raw, final

def fit_classifier(abd, heads):
    """Nearest-center classifier, fit on the pool the groups were agreed on:
    Abdication transients plus gated, unfiltered, non-hat drum heads."""
    fit = np.array([features(s) for _, s in abd] + [features(r) for n, r, _ in heads if not n.startswith(HAT_PREFIXES)])
    mu, sd = fit.mean(0), fit.std(0); C = kmeans((fit - mu)/sd, len(TRANSIENT_GROUPS))
    C = C[np.argsort(C[:, 0])]                           # name clusters by centroid: low -> high
    return lambda seg: TRANSIENT_GROUPS[int(np.argmin((((features(seg) - mu)/sd - C)**2).sum(1)))]

def classifier(): return fit_classifier(list(abdication_transients()), list(drum_heads()))

def build_transients():
    if all(group_done(os.path.join(LIB, "transients", g)) for g in TRANSIENT_GROUPS): print("transients already built"); return
    abd = list(abdication_transients()); heads = list(drum_heads()); classify = fit_classifier(abd, heads)
    seen, counts = set(), dict.fromkeys(TRANSIENT_GROUPS, 0)
    for name, seg in abd + [(n, f) for n, _, f in heads]:
        h = hashlib.md5(np.round(seg, 5).tobytes()).hexdigest()
        if h in seen: continue
        seen.add(h); g = classify(seg)
        save(os.path.join(LIB, "transients", g, name + ".wav"), normalize_peak(seg)); counts[g] += 1
    for g in TRANSIENT_GROUPS: mark_done(os.path.join(LIB, "transients", g))
    print("transients:", counts)

def add_render(path, count, name):
    x = load(path if os.path.isabs(path) else os.path.join(ROOT, path)); slot = len(x)/int(count); classify = classifier()
    for i in range(int(count)):
        seg = normalize_peak(fade_out(x[int(i*slot):int((i + 1)*slot)])); g = classify(seg)
        save(os.path.join(LIB, "transients", g, f"{name} {i+1:02d}.wav"), seg); print(f"{name} {i+1:02d} -> {g}")

# ---------- noise ----------
def frame_stats(x, n=2048):
    k = len(x)//n; F = x[:k*n].reshape(k, n); Z = F - F.mean(1, keepdims=True); s2 = (Z**2).mean(1) + 1e-20
    return db(s2), (Z**4).mean(1)/s2**2, n/SR

def best_window(x, family, length_s):
    lvl, kurt, fs = frame_stats(x); w = int(length_s/fs); med = np.median(lvl); best, best_score = 0, -np.inf
    for s in range(0, len(lvl) - w, max(1, int(0.5/fs))):
        L, K = lvl[s:s+w], kurt[s:s+w]
        if family == "steady":    score = -np.std(L) - abs(np.mean(L) - med)
        elif family == "crackle": score = np.median(K) if np.mean(np.abs(L - med) < 8) > 0.9 else -np.inf
        else:                     score = np.std(L) if np.percentile(L, 10) > med - 12 else -np.inf
        if score > best_score: best, best_score = s, score
    return best*fs

def seamless_loop(x, length_s, xfade_s):
    L, X = int(SR*length_s), int(SR*xfade_s); y = x[:L].copy(); t = np.linspace(0, np.pi/2, X)
    y[:X] = x[:X]*np.sin(t) + x[L:L+X]*np.cos(t)        # equal power: uncorrelated noise
    return y

def level_match(x, rms_dbfs=-20.0, peak_cap=-0.5):
    x = x - x.mean(); g = 10**(rms_dbfs/20)/np.sqrt((x**2).mean()); g = min(g, 10**(peak_cap/20)/np.abs(x).max()); return x*g

def noise_name(fn):
    s = re.sub(r"^NOISE SOURCE |\s*(DI )?HRTP$|\s*Sennheiser MKH8050", "", os.path.splitext(fn)[0])
    return re.sub(r"^(\w+) (Module|Radio|Synth) ", r"\1 ", s).strip()

def build_noise():
    srcs = sorted(os.listdir(NOISE_SRC))
    for family, keys in NOISE.items():
        d = os.path.join(LIB, "noise", family)
        if group_done(d): continue
        length = NOISE_LOOP_S[family]; xfade = 0.05 if length < 2 else 0.1
        for key in keys:
            fn = next(f for f in srcs if key in f); x = load(os.path.join(NOISE_SRC, fn))
            start = best_window(x, family, length + xfade); a = int(start*SR)
            seg = x[a:a + int(SR*length) + int(SR*xfade)]
            save(os.path.join(d, noise_name(fn) + ".wav"), level_match(seamless_loop(seg, length, xfade)))
            print(family, noise_name(fn), f"window at {start:.1f}s")
        mark_done(d)

# ---------- conform ----------
STANDARD = "pcm_s24le,44100,1"
def stream_format(path):
    return subprocess.run(["ffprobe", "-v", "error", "-select_streams", "a:0", "-show_entries", "stream=codec_name,sample_rate,channels",
                           "-of", "csv=p=0", path], capture_output=True, text=True).stdout.strip()

def trim_transient(x, onset_db=-50, tail_db=-60):
    a = np.abs(x); pk = a.max()
    on = max(int(np.argmax(a > pk*10**(onset_db/20))) - 22, 0)   # keep 0.5 ms before onset
    above = np.where(a > pk*10**(tail_db/20))[0]; end = above[-1] + 1 if len(above) else len(x)
    return fade_out(x[on:end])

def conformed_record(group_dir):
    path = os.path.join(group_dir, ".conformed")
    if not os.path.exists(path):                 # first run: files already in standard format are the built ones
        names = [f for f in os.listdir(group_dir) if f.lower().endswith(".wav") and stream_format(os.path.join(group_dir, f)) == STANDARD]
        open(path, "w").write("\n".join(sorted(names)))
    return path, set(open(path).read().split("\n")) - {""}

def conform():
    changed, locked = [], []
    for family in ("transients", "noise"):
        for group in sorted(os.listdir(os.path.join(LIB, family))):
            d = os.path.join(LIB, family, group); record, seen = conformed_record(d)
            for fn in sorted(f for f in os.listdir(d) if f.lower().endswith(".wav")):
                p = os.path.join(d, fn); fmt = stream_format(p); x = load(p)
                new = fn not in seen                    # trimming and looping are one-time edits
                if family == "transients":
                    y = normalize_peak(trim_transient(x) if new else x)
                elif new:
                    xfade = min(0.05, len(x)/SR*0.1)
                    y = level_match(seamless_loop(x, len(x)/SR - xfade, xfade))
                else:
                    y = level_match(x)
                unchanged = fmt == STANDARD and len(y) == len(x) and np.abs(y - x).max() < 1e-4
                if not unchanged:
                    backup = os.path.join(LIB, ".originals", family, group, fn); os.makedirs(os.path.dirname(backup), exist_ok=True)
                    if not os.path.exists(backup): shutil.copy2(p, backup)
                    try: data = encode(y); open(p, "wb").write(data)
                    except PermissionError: locked.append(f"{group}/{fn}"); continue
                    changed.append(f"{group}/{fn}: {fmt} {len(x)/SR*1000:.0f} ms -> {len(y)/SR*1000:.0f} ms")
                seen.add(fn)
            open(record, "w").write("\n".join(sorted(seen)))
    print(f"{len(changed)} files changed"); print("\n".join(changed))
    if locked: print("could not write (open in another app, or read-only):", ", ".join(locked))

if __name__ == "__main__":
    {"transients": build_transients, "noise": build_noise, "add-render": add_render, "conform": conform}[sys.argv[1]](*sys.argv[2:])
