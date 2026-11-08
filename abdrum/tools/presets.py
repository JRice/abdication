"""Starting points for auditioning the ABDrum voice (tools/render.py).

Each preset sets controls as "Panel/Label": value (anything left out keeps
its default) and, optionally, its own hits: (velocity, seconds[, changes]).
Without hits, a preset plays one loud and one soft hit. Times are in ms.
"""

TONE_OFF = {"Tone/Level": 0}


def sweep(path, values, seconds=0.9):
    return [(1.0, seconds, {path: v}) for v in values]


PRESETS = [
    # ---- Kicks. Kick Perkons is fitted to the median Perkons kick: 122 -> 75 -> 48 -> 40 Hz at 0/20/60/120 ms
    {"name": "Kick Perkons", "set": {
        "Tone/Pitch": 38, "Tone/Drop": 56, "Tone/Split": 0.429, "Tone/Decay": 1100, "Tone/Snap": 0.322, "Noise/Decay": 40, "Noise/Snap": 0.301, "Tone/Fast Time": 3, "Tone/Slow Time": 120,
        "Tone/Tail Level": 0.45, "Tone/Shape": 0.12,
        "Noise/Level": 0.06, "Noise/Bandpass": 1500, "Noise/Highpass": 200,
        "Noise/Tail Level": 0}},
    {"name": "Kick Sub", "set": {
        "Tone/Pitch": 42, "Tone/Drop": 12, "Tone/Split": 0, "Tone/Decay": 1800, "Tone/Snap": 0.327, "Tone/Slow Time": 60,
        "Tone/Tail Level": 0.6}},
    {"name": "Kick Click Body", "set": {
        "Tone/Pitch": 55, "Tone/Drop": 50, "Tone/Split": 0.72, "Tone/Decay": 350, "Tone/Snap": 0.295, "Body/Strike": 0.9, "Body/Damping": 0.52, "Tone/Fast Time": 2, "Tone/Slow Time": 50,
        "Tone/Tail Level": 0.25,
        "Body/Level": 0.3, "Body/Offset": 19, "Body/Decay": 60}},
    {"name": "Kick Shape Sweep", "set": {"Tone/Pitch": 50, "Tone/Decay": 400, "Tone/Snap": 0.261},
     "hits": sweep("Tone/Shape", [0, 0.33, 0.55, 0.75, 1.0])},
    {"name": "Zap", "set": {
        "Tone/Pitch": 80, "Tone/Drop": 72, "Tone/Split": 0.667, "Tone/Decay": 300, "Tone/Snap": 0.239, "Tone/Fast Time": 15, "Tone/Slow Time": 200,
        "Tone/Shape": 0.5}},

    # ---- Toms and snares
    {"name": "Tom Perkons", "set": {
        "Tone/Pitch": 95, "Tone/Drop": 7, "Tone/Split": 0.714, "Tone/Decay": 450, "Tone/Snap": 0.176, "Noise/Decay": 200, "Noise/Snap": 0.412, "Body/Strike": 0.5, "Body/Damping": 0.6, "Tone/Fast Time": 3, "Tone/Slow Time": 150,
        "Tone/Tail Level": 0.4,
        "Body/Level": 0.4, "Body/Decay": 300, 
        "Noise/Level": 0.08, "Noise/Generator": 1, "Noise/Bandpass": 800, "Noise/Q": 0.3, "Noise/Highpass": 100,
        "Noise/Tail Level": 0.3}},
    {"name": "Snare Perkons", "set": {
        "Tone/Pitch": 185, "Tone/Drop": 17, "Tone/Split": 0.706, "Tone/Decay": 180, "Tone/Snap": 0.239, "Noise/Decay": 220, "Noise/Snap": 0.322, "Body/Strike": 0.7, "Body/Damping": 0.56, "Tone/Fast Time": 3, "Tone/Slow Time": 40,
        "Tone/Level": 0.55, "Tone/Tail Level": 0.2, "Tone/Shape": 0.2,
        "Body/Level": 0.3, "Body/Offset": 7, "Body/Decay": 120, 
        "Noise/Level": 0.42, "Noise/Bandpass": 4000, "Noise/Q": 0.35, "Noise/Highpass": 1200, "Noise/Env Amount": 0.3,
        "Noise/Tail Level": 0.35}},
    {"name": "Snare 606", "set": {
        "Tone/Level": 0.6, "Tone/Pitch": 205, "Tone/Drop": 4, "Tone/Split": 1, "Tone/Decay": 220, "Tone/Snap": 0.282, "Noise/Decay": 380, "Noise/Snap": 0.367, "Tone/Fast Time": 2, 
        "Tone/Tail Level": 0.2,
        "Noise/Level": 0.5, "Noise/Bandpass": 6000, "Noise/Q": 0.3, "Noise/Highpass": 2500,
        "Noise/Tail Level": 0.4}},
    {"name": "Snare Radio Static", "set": {
        "Tone/Pitch": 190, "Tone/Drop": 13, "Tone/Split": 0.769, "Tone/Decay": 150, "Tone/Snap": 0.239, "Noise/Decay": 300, "Noise/Snap": 0.349, 
        "Noise/Level": 0.8, "Noise/Blend": 1, "Noise/Loop": "149 DSE Static Constant",
        "Noise/Bandpass": 2500, "Noise/Q": 0.3, "Noise/Highpass": 600,
        "Noise/Tail Level": 0.3}},
    {"name": "Clap", "set": {**TONE_OFF,
        "Noise/Level": 1, "Noise/Bandpass": 1100, "Noise/Q": 0.5, "Noise/Highpass": 350, "Noise/Clap": 0.7,
        "Noise/Env Amount": 0, "Noise/Decay": 450, "Noise/Snap": 0.349, "Noise/Tail Level": 0.45}},

    # ---- Hats and microsounds
    {"name": "Hat Metal", "set": {**TONE_OFF, "Tone/Pitch": 410,
        "Body/Level": 0.25, "Body/Material": 1.0, "Body/Offset": 12, "Noise/Decay": 120, "Noise/Snap": 0.239, "Body/Strike": 1, "Body/Damping": 0.4, "Body/Decay": 90, 
        "Noise/Level": 1, "Noise/Generator": 4, "Noise/Bandpass": 9000, "Noise/Q": 0.3, "Noise/Highpass": 6500,
        "Noise/Tail Level": 0.25}},
    {"name": "Hat Metal Open", "set": {**TONE_OFF, "Tone/Pitch": 410,
        "Noise/Level": 1, "Noise/Generator": 4, "Noise/Bandpass": 8500, "Noise/Q": 0.3, "Noise/Highpass": 6000,
        "Noise/Decay": 700, "Noise/Snap": 0.383, "Noise/Tail Level": 0.4}},
    {"name": "Hat Modular", "set": {**TONE_OFF,
        "Noise/Level": 1, "Noise/Generator": 3, "Noise/Bandpass": 8000, "Noise/Q": 0.6, "Noise/Highpass": 6000,
        "Noise/Decay": 70, "Noise/Snap": 0.224, "Noise/Tail Level": 0.2, "Noise/Crush": 0.25}},
    {"name": "Microsound Dust", "set": {**TONE_OFF, "Tone/Pitch": 1200,
        "Body/Level": 0.6, "Body/Material": 0.5, "Noise/Decay": 60, "Noise/Snap": 0.301, "Body/Strike": 1, "Body/Damping": 0.5, "Body/Decay": 40, 
        "Noise/Level": 1, "Noise/Generator": 2, "Noise/Bandpass": 7000, "Noise/Q": 0.8, "Noise/Highpass": 3000,
        "Noise/Tail Level": 0.3, "Noise/Density": 0.6}},
    {"name": "Crackle Tick", "set": {**TONE_OFF,
        "Noise/Level": 0.9, "Noise/Blend": 1, "Noise/Loop": "SP02", "Noise/Bandpass": 5000, "Noise/Q": 0.5,
        "Noise/Highpass": 2000, "Noise/Decay": 80, "Noise/Snap": 0.452, "Noise/Tail Level": 0.3}},

    # ---- Body on its own
    {"name": "Plate Perc", "set": {**TONE_OFF, "Tone/Pitch": 300,
        "Body/Level": 1, "Body/Material": 0.5, "Body/Decay": 900, "Body/Strike": 0.85, "Body/Damping": 0.53}},
    {"name": "Rim Knock", "set": {**TONE_OFF, "Tone/Pitch": 480,
        "Body/Level": 1, "Body/Material": 1.0, "Body/Decay": 70, "Noise/Decay": 15, "Noise/Snap": 0.287, "Body/Strike": 0.9, "Body/Damping": 0.87, 
        "Noise/Level": 0.5, "Noise/Bandpass": 2500, "Noise/Q": 0.45, "Noise/Highpass": 900,
        "Noise/Tail Level": 0}},
    {"name": "Woodblock", "set": {**TONE_OFF, "Tone/Pitch": 820,
        "Body/Level": 1, "Body/Material": 0.85, "Body/Decay": 120, "Noise/Decay": 10, "Noise/Snap": 0.261, "Body/Strike": 0.9, "Body/Damping": 0.87, 
        "Noise/Level": 0.3, "Noise/Bandpass": 3000, "Noise/Q": 0.45, "Noise/Highpass": 1200,
        "Noise/Tail Level": 0}},
    {"name": "Body Material Sweep", "set": {**TONE_OFF, "Tone/Pitch": 220,
        "Body/Level": 0.8, "Body/Decay": 700, "Body/Strike": 0.7, "Body/Damping": 0.56},
     "hits": sweep("Body/Material", [0, 0.25, 0.5, 0.75, 1.0], seconds=1.0)},

    # ---- Sample layers, loop pitch, Q
    {"name": "Kick + Thump Layer", "set": {
        "Tone/Pitch": 38, "Tone/Drop": 56, "Tone/Split": 0.429, "Tone/Decay": 900, "Tone/Snap": 0.278, "Tone/Fast Time": 3, "Tone/Slow Time": 120,
        "Tone/Tail Level": 0.45, "Tone/Shape": 0.12,
        "Sample 1/Level": 0.35, "Sample 1/Hit": ("thump", "Jade 023")}},
    {"name": "Snare + Knock + Spark", "set": {
        "Tone/Level": 0.5, "Tone/Pitch": 185, "Tone/Drop": 17, "Tone/Split": 0.706, "Tone/Decay": 180, "Tone/Snap": 0.239, "Noise/Decay": 220, "Noise/Snap": 0.322, "Tone/Fast Time": 3, "Tone/Slow Time": 40,
        "Tone/Tail Level": 0.2,
        "Noise/Level": 0.45, "Noise/Bandpass": 4000, "Noise/Q": 0.35, "Noise/Highpass": 1200,
        "Noise/Tail Level": 0.35,
        "Sample 1/Level": 0.4, "Sample 1/Hit": ("knock", "Rim Click Perkons Color"),
        "Sample 2/Level": 0.4, "Sample 2/Hit": ("spark", "Mech 012")}},
    {"name": "Spark Strikes Plate", "set": {**TONE_OFF, "Tone/Pitch": 1500,
        "Body/Level": 1, "Body/Material": 0.5, "Body/Decay": 350, "Body/Strike": 0.5, "Body/Damping": 0.3, 
        "Sample 2/Level": 0.45, "Sample 2/Hit": ("spark", "Mech 012"), "Sample 2/Body Send": 1}},
    {"name": "Knock Strikes Skin", "set": {**TONE_OFF, "Tone/Pitch": 140,
        "Body/Level": 0.85, "Body/Material": 0, "Body/Decay": 450, "Body/Strike": 0.5, "Body/Damping": 0.55, 
        "Sample 1/Level": 0.25, "Sample 1/Hit": ("knock", "Jade 018"), "Sample 1/Body Send": 1}},
    {"name": "Loop Pitch Sweep", "set": {**TONE_OFF,
        "Noise/Level": 0.9, "Noise/Blend": 1, "Noise/Loop": "149 DSE Static Constant", "Noise/Q": 0, "Noise/Highpass": 100,
        "Noise/Decay": 500, "Noise/Snap": 0.261, "Noise/Tail Level": 0.4},
     "hits": sweep("Noise/Loop Pitch", [-24, -12, 0, 12, 24])},
    {"name": "Noise Q Sweep", "set": {**TONE_OFF,
        "Noise/Level": 0.8, "Noise/Bandpass": 2500, "Noise/Highpass": 100,
        "Noise/Decay": 500, "Noise/Snap": 0.261, "Noise/Tail Level": 0.4},
     "hits": sweep("Noise/Q", [0, 0.25, 0.5, 0.75, 1.0])},

    # ---- Drive (for the backend check)
    {"name": "Kick Tape Drive", "set": {
        "Tone/Pitch": 38, "Tone/Drop": 56, "Tone/Split": 0.429, "Tone/Decay": 900, "Tone/Snap": 0.278, "Tone/Fast Time": 3, "Tone/Slow Time": 120,
        "Tone/Tail Level": 0.45, "Drive/Type": 0, "Drive/Amount": 0.6}},
    {"name": "Snare Tube Drive", "set": {
        "Tone/Level": 0.5, "Tone/Pitch": 185, "Tone/Drop": 17, "Tone/Split": 0.706, "Tone/Decay": 180, "Tone/Snap": 0.239, 
        "Noise/Level": 0.45, "Noise/Bandpass": 4000, "Noise/Q": 0.35, "Noise/Highpass": 1200,
        "Sample 2/Level": 0.4, "Sample 2/Hit": ("spark", "Mech 012"), "Drive/Type": 1, "Drive/Amount": 0.5}},
    {"name": "Hat Clip Drive", "set": {**TONE_OFF, "Tone/Pitch": 410,
        "Noise/Level": 1, "Noise/Generator": 4, "Noise/Bandpass": 9000, "Noise/Q": 0.3, "Noise/Highpass": 6500,
        "Noise/Decay": 120, "Noise/Snap": 0.239, "Drive/Type": 2, "Drive/Amount": 0.8}},

    # ---- The other effects (for the backend check)
    {"name": "Snare Crushed", "set": {
        "Tone/Level": 0.5, "Tone/Pitch": 185, "Tone/Drop": 17, "Tone/Split": 0.706, "Tone/Decay": 180, "Tone/Snap": 0.239, 
        "Noise/Level": 0.45, "Noise/Bandpass": 4000, "Noise/Q": 0.35, "Noise/Highpass": 1200, "Crush/Amount": 0.7}},
    {"name": "Tom Ring Mod", "set": {
        "Tone/Pitch": 95, "Tone/Drop": 7, "Tone/Split": 0.714, "Tone/Decay": 450, "Tone/Snap": 0.176, 
        "Body/Level": 0.4, "Body/Decay": 300, "Ring Mod/Amount": 0.6}},
    {"name": "Spark Comb", "set": {**TONE_OFF, "Tone/Pitch": 330,
        "Sample 1/Level": 0.8, "Sample 1/Hit": ("spark", "Mech 012"), "Comb/Amount": 0.85}},
    {"name": "Kick Compressed", "set": {
        "Tone/Pitch": 38, "Tone/Drop": 56, "Tone/Split": 0.429, "Tone/Decay": 900, "Tone/Snap": 0.278, "Tone/Fast Time": 3, "Tone/Slow Time": 120,
        "Tone/Tail Level": 0.45,
        "Drive/Type": 1, "Drive/Amount": 0.3, "Compressor/Threshold": -24, "Compressor/Timing": 3}},

    # ---- The free envelope and its rings (voice and effects)
    {"name": "Ring Sends", "set": {
        "Envelope/Attack": 2, "Envelope/Decay": 150, "Envelope/Curve": 0.7, "Envelope/Velocity": 0.8,
        "Tone/Pitch": 120, "Tone/Pitch Mod": 0.3, "Tone/Shape Mod": 0.6, "Tone/Decay": 300,
        "Body/Level": 0.4, "Body/Material Mod": 0.5, "Body/Decay Mod": -0.3,
        "Noise/Level": 0.5, "Noise/Bandpass Mod": -0.5, "Noise/Q Mod": 0.4, "Noise/Decay": 400,
        "Sample 1/Level": 0.4, "Sample 1/Hit": ("knock", "Jade 018"), "Sample 1/Pitch Mod": 0.25,
        "Comb/Amount Mod": 0.6, "Crush/Amount Mod": 0.4, "Compressor/Threshold": -12, "Compressor/Threshold Mod": -0.5}},

    # ---- Variation: the same snare eight times, Variation at 0.7, alternating accents
    {"name": "Variation Demo", "set": {
        "Voice/Variation": 0.7,
        "Tone/Pitch": 185, "Tone/Drop": 17, "Tone/Split": 0.706, "Tone/Decay": 180, "Tone/Snap": 0.239, "Noise/Decay": 220, "Noise/Snap": 0.322, "Body/Strike": 0.7, "Body/Damping": 0.56, "Tone/Fast Time": 3, "Tone/Slow Time": 40,
        "Tone/Level": 0.45, "Tone/Tail Level": 0.2, "Tone/Shape": 0.2,
        "Body/Level": 0.3, "Body/Offset": 7, "Body/Decay": 120, 
        "Noise/Level": 0.4, "Noise/Bandpass": 4000, "Noise/Q": 0.35, "Noise/Highpass": 1200,
        "Noise/Tail Level": 0.35},
     "hits": [(v, 0.3) for v in (1.0, 0.7) * 4]},
]
