// Cobalt8X -> Abdication: a Bitwig controller script.
//
// The Cobalt8X's knobs drive the Abdication on the selected track, by
// parameter name, with no MIDI learn. Keys, pitch bend, mod wheel,
// sustain and pressure reach the track as usual (record-arm it, or let
// Bitwig's input follow selection). The map is the one in
// docs/cobalt8x-mapping.md (CC numbers from Modal's Cobalt8 manual).
//
// Install: copy this file into Documents\Bitwig Studio\Controller Scripts\
// (any subfolder), then Settings > Controllers > Add controller >
// Abdicant > "Cobalt8X for Abdication", and pick the Cobalt's MIDI input.
// Set the Cobalt to Local Off. Absolute knobs: a knob jumps the parameter
// to its own position when first turned.
//
// Plain ES5 (Bitwig's script engine), no build step.

loadAPI(17);
host.setShouldFailOnDeprecatedUse(false);
host.defineController("Abdicant", "Cobalt8X for Abdication", "1.0", "157d4d59-da5c-4422-ae81-d2af63e13ae3", "Abdicant");
host.defineMidiPorts(1, 0);

/** Cobalt CC -> Abdication parameter (its name as Bitwig shows it). */
var MAP = {
    // Oscillator 1 -> Osc 1's Dual
    62: "Dual 1: Algorithm",
    16: "Dual 1: A",
    19: "Dual 1: B",
    14: "Dual 1: Semi",
    15: "Unison: Detune",
    // Oscillator 2 -> Osc 2's Wavetable
    63: "WT 2: OscMod",
    17: "WT 2: Position",
    20: "WT 2: OscMod Depth",
    30: "Osc 2: Octave",
    31: "WT 2: Position LFO Rate",
    18: "Mix: Osc 2",
    // Filter
    34: "Filter: Cutoff",
    35: "Filter: Resonance",
    33: "Filter: Morph LP-Notch-HP",
    // Filter EG -> the Cutoff's envelope (its modulator)
    32: "Filter: Cutoff Env Amount",
    22: "Filter: Cutoff Env Attack",
    23: "Filter: Cutoff Env Decay",
    24: "Filter: Cutoff Env Sustain",
    25: "Filter: Cutoff Env Release",
    // Amp EG -> Amp
    26: "Amp: Attack",
    27: "Amp: Decay",
    28: "Amp: Sustain",
    // LFO1 -> the Cutoff's LFO
    36: "Filter: Cutoff LFO Rate",
    39: "Filter: Cutoff LFO Shape",
    37: "Filter: Cutoff LFO Amount",
    // Mod envelope -> Dual 1 A's envelope (Depth: its amount)
    43: "Dual 1: A Env Attack",
    44: "Dual 1: A Env Decay",
    45: "Dual 1: A Env Sustain",
    46: "Dual 1: A Env Release",
    49: "Dual 1: A Env Amount",
    // LFO2 and LFO3 -> noise and transient
    47: "Mix: Noise",
    48: "Noise: Tilt",
    50: "Noise: Clock Ratio",
    3: "Noise: Bits",
    13: "Transient: Level",
    // Other (Drift sends an NRPN instead: see NRPN_MAP)
    73: "Drive: Amount",
    119: "Master: Volume"
};

/**
 * NRPN number -> parameter. Drift sends an NRPN (CC 99/98 pick the number,
 * CC 6/38 carry the value); its number isn't documented, so any NRPN not
 * listed here also goes to DRIFT, and its number is shown once so it can
 * be listed.
 */
var NRPN_MAP = {};
var DRIFT = "Vintage: Amount";

/** CCs whose knob runs the other way (the Cobalt's LFO1 Rate). */
var INVERTED = { 36: true };

/** The device name the knobs look for on the selected track. */
var TARGET = "Abdication";
/** Devices of the selected track's chain that are searched for it. */
var SLOTS = 16;

/** Per slot of the device bank: its name and its parameters (name -> id). */
var slots = [];

function init() {
    var port = host.getMidiInPort(0);
    // Everything but the knobs goes to the track: notes, pressure, pitch
    // bend, mod wheel (CC 1) and sustain (CC 64), on any channel.
    port.createNoteInput("Cobalt8X", "8?????", "9?????", "A?????", "D?????", "E?????", "B?01??", "B?40??");
    port.setMidiCallback(onMidi);

    var track = host.createCursorTrack("cobalt8x", "Cobalt8X", 0, 0, true);
    var bank = track.createDeviceBank(SLOTS);
    for (var i = 0; i < SLOTS; i++) {
        slots.push(watch(bank.getDevice(i)));
    }
    host.showPopupNotification("Cobalt8X for Abdication ready");
}

/** Follow one bank slot's device: its name and its parameters' ids. */
function watch(device) {
    var slot = { device: device, name: "", ids: {} };
    device.exists().markInterested();
    device.name().addValueObserver(function (name) {
        slot.name = name;
    });
    device.addDirectParameterIdObserver(function (ids) {
        // A different device (or parameter set): forget parameters it
        // doesn't have; names for the new ones follow.
        var present = {};
        for (var i = 0; i < ids.length; i++) {
            present[ids[i]] = true;
        }
        for (var name in slot.ids) {
            if (!present[slot.ids[name]]) {
                delete slot.ids[name];
            }
        }
    });
    device.addDirectParameterNameObserver(80, function (id, name) {
        slot.ids[name] = id;
    });
    return slot;
}

/** The first Abdication in the selected track's chain, if any. */
function abdication() {
    for (var i = 0; i < slots.length; i++) {
        if (slots[i].device.exists().get() && slots[i].name.indexOf(TARGET) === 0) {
            return slots[i];
        }
    }
    return null;
}

function onMidi(status, data1, data2) {
    if ((status & 0xF0) !== 0xB0) {
        return;
    }
    if (nrpn(data1, data2)) {
        return;
    }
    var name = MAP[data1];
    if (name === undefined) {
        reportUnmapped(data1);
        return;
    }
    if (INVERTED[data1]) {
        data2 = 127 - data2;
    }
    set(name, data2, 128);
}

/** The NRPN being received: its number's two halves and its value's. */
var incoming = { msb: 127, lsb: 127, high: 0, low: 0 };

/** Follow an NRPN's CCs; true if this CC was one of them. */
function nrpn(cc, value) {
    switch (cc) {
        case 99: incoming.msb = value; incoming.low = 0; return true;
        case 98: incoming.lsb = value; incoming.low = 0; return true;
        case 6: incoming.high = value; break;
        case 38: incoming.low = value; break;
        default: return false;
    }
    if (incoming.msb === 127 && incoming.lsb === 127) {
        return true; // no NRPN selected (the "null" number)
    }
    var number = incoming.msb * 128 + incoming.lsb;
    var name = NRPN_MAP[number];
    if (name === undefined) {
        name = DRIFT;
        report("nrpn" + number, "NRPN " + number + " -> " + DRIFT);
    }
    set(name, incoming.high * 128 + incoming.low, 16384);
    return true;
}

/** Set a parameter of the selected track's Abdication, by name. */
function set(name, value, resolution) {
    var slot = abdication();
    if (slot === null) {
        return;
    }
    var id = slot.ids[name];
    if (id === undefined) {
        println("Cobalt8X: Abdication has no parameter '" + name + "'");
        return;
    }
    slot.device.setDirectParameterValueNormalized(id, value, resolution);
}

/** Messages already shown (once each, so a turning knob doesn't flood). */
var reported = {};

function report(key, message) {
    if (reported[key]) {
        return;
    }
    reported[key] = true;
    host.showPopupNotification("Cobalt8X: " + message);
    println("Cobalt8X: " + message);
}

/** Say which CC an unmapped knob sends, so it can go in MAP. */
function reportUnmapped(cc) {
    if (cc !== 1 && cc !== 64) {
        report("cc" + cc, "CC " + cc + " isn't mapped");
    }
}

function exit() {
}
