// Offline renderer for ABDrum: plays scripted hits through the voice and then
// the effects (as the plugin engine does: one voice here, the effects after
// it) and writes raw float32 mono audio. Driven by tools/render.py; not part
// of the plugin.
//
// Reads commands from stdin, one per line:
//   list                         print every control path
//   set <path> <value>           set a control: /ABDrum/... (the voice), falling
//                                back to the effects (/ABDrumFX/...)
//   hit <velocity> <seed> <sec>  trigger a hit and render <sec> seconds
//   write <file>                 write what has been rendered, then clear it
//
// argv[1], argv[2]: the sample library (samplelib.bin) and a plain-text
// index: "L start length" per loop, then "H start length" per hit.

#include <cmath>
#include <cstdint>
#include <cstdio>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

#include <faust/dsp/dsp.h>
#include <faust/gui/MapUI.h>
#include <faust/gui/meta.h>

static std::vector<int16_t> g_data;           // 16-bit, as embedded in the plugin
static std::vector<std::pair<long, long>> g_loops, g_hits;

// Mirrors plugin/src/samplelib.rs: wrapped, linearly interpolated reads.
float sl_loop(int k, float pos) {
    if (k < 0 || k >= (int)g_loops.size() || !std::isfinite(pos)) return 0.0f;
    const auto [start, len] = g_loops[k];
    double p = std::fmod((double)pos, (double)len);
    if (p < 0) p += len;
    long i = (long)p;
    float frac = (float)(p - i);
    float a = g_data[start + i] / 32768.0f, b = g_data[start + (i + 1) % len] / 32768.0f;
    return a + (b - a) * frac;
}

// Mirrors plugin/src/samplelib.rs: interpolated reads, silent outside the hit.
float sl_hit(int k, float pos) {
    if (k < 0 || k >= (int)g_hits.size() || !std::isfinite(pos) || pos < 0) return 0.0f;
    const auto [start, len] = g_hits[k];
    if (pos >= len - 1) return 0.0f;
    long i = (long)pos;
    float frac = pos - i;
    float a = g_data[start + i] / 32768.0f, b = g_data[start + i + 1] / 32768.0f;
    return a + (b - a) * frac;
}

#include "voice.cpp"
#include "body.cpp"
#include "noise.cpp"
#include "fx.cpp"

int main(int argc, char** argv) {
    if (argc < 3) { std::cerr << "usage: render <samplelib.bin> <index.txt>\n"; return 1; }
    std::ifstream bin(argv[1], std::ios::binary | std::ios::ate);
    g_data.resize(bin.tellg() / sizeof(int16_t));
    bin.seekg(0);
    bin.read(reinterpret_cast<char*>(g_data.data()), g_data.size() * sizeof(int16_t));
    std::ifstream index(argv[2]);
    std::string kind;
    for (long s, n; index >> kind >> s >> n;) (kind == "L" ? g_loops : g_hits).emplace_back(s, n);

    const int rate = 44100, block = 64;
    VoiceDsp dsp;
    dsp.init(rate);
    MapUI ui;
    dsp.buildUserInterface(&ui);
    BodyDsp body;
    body.init(rate);
    MapUI bodyui;
    body.buildUserInterface(&bodyui);
    bool body_running = false, body_snap = false;
    NoiseDsp noise;
    noise.init(rate);
    FxDsp fx;
    fx.init(rate);
    MapUI fxui;
    fx.buildUserInterface(&fxui);
    // Mirrors the engine: the effects are tuned to Tone's pitch, keytracked
    // to the note when Keytrack is on.
    auto tune = [&] {
        float pitch = ui.getParamValue("/ABDrum/Tone/Pitch");
        return ui.getParamValue("/ABDrum/Tone/Keytrack") > 0.5f ? pitch * ui.getParamValue("freq") / 261.6256f : pitch;
    };

    // ... and they follow the voice's Envelope settings, restarted by each hit.
    auto envelope = [&] {
        for (auto [from, to] : {std::pair{"Attack", "env_attack"}, {"Decay", "env_decay"}, {"Curve", "env_curve"}, {"Velocity", "env_velocity"}})
            fxui.setParamValue(to, ui.getParamValue(std::string("/ABDrum/Envelope/") + from));
    };

    std::vector<float> out;
    // Mirrors the engine: the voice's outputs are Tone, Sample 1, 2, the Body's
    // exciter and level, the Noise's level, the Body's settings (to the Body DSP
    // every BODY_STEP samples) and the Noise's inputs; the Body and the Noise run
    // only while they sound.
    const int VOICE_OUTS = 21, EXCITER = 3, BODY_LEVEL = 4, NOISE_LEVEL = 5, BODY_SETTINGS = 6, NOISE_INPUTS = 11;
    const int BODY_STEP = 16;                     // as plugin/src/engine.rs
    const char* body_settings[5] = {"body_hz", "body_material", "body_t60", "body_bright", "body_position"};
    FAUSTFLOAT parts[VOICE_OUTS][block], bodyout[block], noiseout[block], buf[block], fxbuf[block];
    FAUSTFLOAT* outs[VOICE_OUTS];
    for (int i = 0; i < VOICE_OUTS; i++) outs[i] = parts[i];
    FAUSTFLOAT* noiseins[11];
    for (int k = 0; k < 10; k++) noiseins[k] = parts[NOISE_INPUTS + k];
    noiseins[10] = parts[NOISE_LEVEL];
    FAUSTFLOAT* noiseouts[1] = {noiseout};
    auto sounding = [&](int channel, int n) {
        for (int i = 0; i < n; i++) if (std::fabs(parts[channel][i]) > 1e-6f) return true;
        return false;
    };
    auto run_noise = [&](int n) {
        if (sounding(NOISE_LEVEL, n)) noise.compute(n, noiseins, noiseouts);   // never cleared on the way back in
        else std::fill(noiseout, noiseout + n, 0.0f);
    };
    FAUSTFLOAT* fxins[1] = {buf};
    FAUSTFLOAT* fxouts[1] = {fxbuf};
    auto run_body = [&](int n) {
        if (!sounding(BODY_LEVEL, n)) { body_running = false; std::fill(bodyout, bodyout + n, 0.0f); return; }
        if (!body_running) { body.instanceClear(); body_snap = true; }
        for (int from = 0; from < n; from += BODY_STEP) {
            int to = std::min(n, from + BODY_STEP);
            for (int k = 0; k < 5; k++) bodyui.setParamValue(body_settings[k], parts[BODY_SETTINGS + k][to - 1]);
            bodyui.setParamValue("body_gate", body_snap ? 1 : 0);
            FAUSTFLOAT* ins[2] = {parts[EXCITER] + from, parts[BODY_LEVEL] + from};
            FAUSTFLOAT* outs1[1] = {bodyout + from};
            body.compute(to - from, ins, outs1);
            if (body_snap) { bodyui.setParamValue("body_gate", 0); body_snap = false; }
        }
        body_running = true;
    };
    auto render = [&](long frames) {
        for (long done = 0; done < frames; done += block) {
            int n = (int)std::min<long>(block, frames - done);
            dsp.compute(n, nullptr, outs);
            ui.setParamValue("gate", 0);              // a one-block pulse, as in the engine
            run_body(n);
            run_noise(n);
            for (int i = 0; i < n; ++i) buf[i] = parts[0][i] + bodyout[i] + noiseout[i] + parts[1][i] + parts[2][i];
            fxui.setParamValue("tune", tune());
            fx.compute(n, fxins, fxouts);
            fxui.setParamValue("hit", 0);
            out.insert(out.end(), fxbuf, fxbuf + n);
        }
    };

    std::string line;
    while (std::getline(std::cin, line)) {
        std::istringstream in(line);
        std::string cmd;
        in >> cmd;
        if (cmd == "list") {
            for (int i = 0; i < ui.getParamsCount(); i++) std::cout << ui.getParamAddress(i) << "\n";
            for (int i = 0; i < fxui.getParamsCount(); i++) std::cout << fxui.getParamAddress(i) << "\n";
        } else if (cmd == "set") {
            std::string path; float v;
            in >> path >> v;
            std::string fxpath = "/ABDrumFX" + path.substr(path.find('/', 1));
            if (ui.getParamZone(path)) ui.setParamValue(path, v);
            else if (fxui.getParamZone(fxpath)) fxui.setParamValue(fxpath, v);
            else { std::cerr << "unknown control " << path << "\n"; return 2; }
        } else if (cmd == "hit") {
            float vel, seed, sec;
            in >> vel >> seed >> sec;
            ui.setParamValue("gain", vel);
            ui.setParamValue("seed", seed);
            ui.setParamValue("gate", 1);
            fxui.setParamValue("hit", 1);
            fxui.setParamValue("vel", vel);
            envelope();
            body_snap = true;
            render((long)(sec * rate));
        } else if (cmd == "write") {
            std::string file;
            in >> file;
            std::ofstream(file, std::ios::binary).write(reinterpret_cast<char*>(out.data()), out.size() * sizeof(float));
            out.clear();
        }
    }
    return 0;
}
