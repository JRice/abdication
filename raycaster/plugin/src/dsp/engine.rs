//! Raycaster's voices: twelve (or one, in mono with glide), each two ray-cast oscillators into
//! a tube drive and a filter (2-pole SVF or 4-pole ladder). The oscillators run at the host
//! rate (their leaps are already smoothed); each voice is then upsampled for the drive and
//! filter, and the whole instrument brought back down once. Every modulatable control is moved per
//! voice by its own dynamic modulator before the voice uses it.

use std::sync::Arc;

use nih_plug::prelude::Enum;

use super::chorus::Chorus;
use super::envelope::{Adsr, Curves, Rates, Times};
use super::filters::{halfband, Decimator, Interpolator, Ladder, Svf, Tube, TubeDrive};
use super::geometry::{Bend, Outline};
use super::limiter::Limiter;
use super::modulator::{Clock, ModSettings, ModState};
use super::random::Rng;
use super::rayosc::{OscBlock, RayOsc};
use super::shapes::{Element, ELEMENTS};
use super::targets::{
    global_target, osc_target, range_of, GlobalTarget, OscTarget, Range, GLOBAL_TARGETS, OSCS, OSC_TARGETS, TARGETS,
};

pub const VOICES: usize = 12;
/// The most voices one note can stack in unison.
pub const MAX_UNISON: usize = 6;
/// Room for each voice and one being faded out: a stolen voice fades over `FADE_S` in a
/// slot of its own while the new note starts clean in another.
const SLOTS: usize = 2 * VOICES;
/// Unison voices are spread this far across the stereo field, either side.
const UNISON_PAN: f32 = 0.6;
/// The mod wheel and aftertouch are smoothed over this long, so their 128 steps don't zip.
const CONTROLLER_S: f32 = 0.012;
const FADE_S: f32 = 0.004;
pub const OVERSAMPLE: usize = 2;
/// The longest block the engine takes at once, in host samples.
pub const MAX_BLOCK: usize = 32;
const BEND_SEMITONES: f32 = 2.0;
const FILTER_ENV_OCTAVES: f32 = 6.0;
const VOICE_GAIN: f32 = 0.4;
/// Filter coefficients follow the envelope every this many oversampled samples.
const COEFFICIENT_STEP: usize = 4;
/// Room left above an oscillator's own partials for its orbit's sidebands.
const MONO_STACK: usize = 16;
const UNPRIMED: u32 = u32::MAX;

#[derive(Enum, Clone, Copy, PartialEq, Eq, Debug)]
pub enum FilterKind {
    #[id = "svf"]
    #[name = "SVF"]
    Svf,
    #[id = "ladder"]
    #[name = "Ladder"]
    Ladder,
}

#[derive(Clone, Copy, Debug)]
pub struct OscSetup {
    pub element: Element,
    pub octave: i32,
    pub semi: i32,
    /// Free running: a note starts the shape wherever, not upright.
    pub free: bool,
}

/// Everything the knobs set, once a block. `base` is each modulatable control's own
/// position in its normalized travel, before modulation.
#[derive(Clone, Debug)]
pub struct BlockControls {
    pub base: [f32; TARGETS],
    pub mods: [ModSettings; TARGETS],
    pub oscs: [OscSetup; OSCS],
    pub filter: FilterKind,
    pub keytrack: f32,
    pub filter_env: Times,
    pub amp_env: Times,
    pub velocity: f32,
    pub mono: bool,
    /// Voices per note, and their spread in cents, end to end.
    pub unison: usize,
    pub detune: f32,
    pub glide_s: f32,
    pub volume: f32,
    pub clock: Clock,
}

impl Default for BlockControls {
    fn default() -> Self {
        let mut base = [0.0; TARGETS];
        for (t, b) in base.iter_mut().enumerate() {
            let r = range_of(t);
            *b = r.normalize(r.default);
        }
        base[osc_target(1, OscTarget::Level)] = 0.0;
        Self {
            base,
            mods: [ModSettings::default(); TARGETS],
            oscs: [
                OscSetup { element: Element::Earth, octave: 0, semi: 0, free: false },
                OscSetup { element: Element::Fire, octave: 0, semi: 0, free: false },
            ],
            filter: FilterKind::Ladder,
            keytrack: 0.5,
            filter_env: Times { attack_s: 0.003, decay_s: 0.45, sustain: 0.25, release_s: 0.3, curves: Curves::default() },
            amp_env: Times { attack_s: 0.004, decay_s: 0.6, sustain: 0.8, release_s: 0.25, curves: Curves::default() },
            velocity: 0.5,
            mono: false,
            unison: 1,
            detune: 20.0,
            glide_s: 0.08,
            volume: 0.7,
            clock: Clock::default(),
        }
    }
}

impl BlockControls {
    /// Sets control `index` to a plain value (for tests and the renderer).
    pub fn set(&mut self, index: usize, value: f32) {
        self.base[index] = range_of(index).normalize(value);
    }
}

struct Voice {
    note: u8,
    velocity: f32,
    /// The key is down.
    gate: bool,
    /// When the note started; a note's unison voices share it.
    age: u64,
    /// This voice's place in its unison stack: semitones off the note, stereo gains, and
    /// its share of the level.
    detune: f32,
    pan: [f32; 2],
    share: f32,
    /// Polyphonic aftertouch on this voice's key, and the aftertouch it feels (smoothed).
    poly_pressure: f32,
    touch: f32,
    pitch: f32,
    target: f32,
    amp: Adsr,
    flt: Adsr,
    oscs: [RayOsc; OSCS],
    chorus: [Chorus; OSCS],
    mods: Vec<ModState>,
    /// Each control's modulated, normalized position in the last block.
    values: [f32; TARGETS],
    step: [f32; OSCS],
    level: [f32; OSCS],
    /// Which outline each oscillator's history was primed for: an element's index, 64 plus a
    /// count of Chaos outlines, or `UNPRIMED`.
    primed: [u32; OSCS],
    /// Stolen: on its way out over `FADE_S`, whatever its envelope says.
    fading: bool,
    fade: f32,
    up: [Interpolator; 2],
    tube: [Tube; 2],
    svf: [Svf; 2],
    ladder: [Ladder; 2],
}

impl Voice {
    fn new(index: usize, os_rate: f32) -> Self {
        Self {
            note: 60,
            velocity: 1.0,
            gate: false,
            age: 0,
            detune: 0.0,
            pan: [1.0, 1.0],
            share: 1.0,
            poly_pressure: 0.0,
            touch: 0.0,
            pitch: 60.0,
            target: 60.0,
            amp: Adsr::default(),
            flt: Adsr::default(),
            oscs: Default::default(),
            chorus: std::array::from_fn(|o| Chorus::new(((index * OSCS + o) as f32 * 0.382).fract())),
            mods: (0..TARGETS).map(|t| ModState::new((index * TARGETS + t) as u32 * 7_919 + 101)).collect(),
            values: [0.0; TARGETS],
            step: [0.0; OSCS],
            level: [0.0; OSCS],
            primed: [UNPRIMED; OSCS],
            fading: false,
            fade: 1.0,
            up: Default::default(),
            tube: [Tube::new(os_rate); 2],
            svf: [Svf::default(); 2],
            ladder: [Ladder::default(); 2],
        }
    }

    fn sounding(&self) -> bool {
        !self.amp.idle()
    }

    /// Sounding and not being faded out: what counts against the polyphony.
    fn playing(&self) -> bool {
        self.sounding() && !self.fading
    }

    fn fade_out(&mut self) {
        self.fading = true;
        self.gate = false;
    }

    /// Silences the voice at once and forgets it.
    fn stop(&mut self) {
        self.amp.clear();
        self.flt.clear();
        self.clear();
    }

    fn clear(&mut self) {
        self.fading = false;
        self.fade = 1.0;
        self.oscs.iter_mut().for_each(|o| {
            o.clear();
            o.restart();
        });
        self.chorus.iter_mut().for_each(Chorus::clear);
        self.up.iter_mut().for_each(Interpolator::clear);
        self.tube.iter_mut().for_each(Tube::clear);
        self.svf.iter_mut().for_each(Svf::clear);
        self.ladder.iter_mut().for_each(Ladder::clear);
        self.step = [0.0; OSCS];
        self.level = [0.0; OSCS];
    }
}

/// What the panel shows of the newest voice.
#[derive(Clone, Copy, Debug)]
pub struct Shown {
    pub sounding: bool,
    pub values: [f32; TARGETS],
    pub orbit: [f32; OSCS],
    pub turn: [f32; OSCS],
}

impl Default for Shown {
    fn default() -> Self {
        Self { sounding: false, values: [0.0; TARGETS], orbit: [0.0; OSCS], turn: [0.0; OSCS] }
    }
}

pub struct Engine {
    rate: f32,
    os_rate: f32,
    elements: Vec<Arc<Outline>>,
    chaos: [Arc<Outline>; OSCS],
    /// Counts Chaos outlines taken, so a voice can tell its shape changed.
    chaos_count: [u32; OSCS],
    voices: Vec<Voice>,
    age: u64,
    held: [u8; MONO_STACK],
    held_len: usize,
    mono: bool,
    /// The slots playing in mono (its unison stack).
    mono_slots: [usize; MAX_UNISON],
    mono_count: usize,
    bend: f32,
    /// Mod wheel and channel aftertouch, 0..1, as received and as smoothed.
    wheel_target: f32,
    wheel: f32,
    pressure: f32,
    /// Starting phases for free-running and unison voices.
    rng: Rng,
    taps: [f32; 63],
    decimators: [Decimator; 2],
    limiter: Limiter,
    osc_buffers: [[[f32; MAX_BLOCK]; 2]; OSCS],
    mix: [[f32; MAX_BLOCK * OVERSAMPLE]; 2],
}

fn note_hz(semitones: f32) -> f32 {
    440.0 * ((semitones - 69.0) / 12.0).exp2()
}

impl Engine {
    pub fn new(rate: f32) -> Self {
        let os_rate = rate * OVERSAMPLE as f32;
        let earth = Arc::new(Element::Earth.outline());
        Self {
            rate,
            os_rate,
            elements: ELEMENTS.iter().map(|e| Arc::new(e.outline())).collect(),
            chaos: [earth.clone(), earth],
            chaos_count: [0; OSCS],
            voices: (0..SLOTS).map(|i| Voice::new(i, os_rate)).collect(),
            age: 0,
            held: [0; MONO_STACK],
            held_len: 0,
            mono: false,
            mono_slots: [0; MAX_UNISON],
            mono_count: 0,
            bend: 0.0,
            wheel_target: 0.0,
            wheel: 0.0,
            pressure: 0.0,
            rng: Rng::new(0x5EED),
            taps: halfband(),
            decimators: Default::default(),
            limiter: Limiter::new(rate),
            osc_buffers: [[[0.0; MAX_BLOCK]; 2]; OSCS],
            mix: [[0.0; MAX_BLOCK * OVERSAMPLE]; 2],
        }
    }

    /// A finished Chaos outline for oscillator `osc`; the old one comes back to be dropped
    /// somewhere other than the audio thread.
    pub fn take_chaos(&mut self, osc: usize, outline: Arc<Outline>) -> Arc<Outline> {
        self.chaos_count[osc] = self.chaos_count[osc].wrapping_add(1) % (UNPRIMED - 64);
        std::mem::replace(&mut self.chaos[osc], outline)
    }

    pub fn reduction_db(&self) -> f32 {
        self.limiter.reduction_db()
    }

    /// Starts voice `k` of a note's `n` (its unison stack) in `slot`.
    fn start(&mut self, slot: usize, note: u8, velocity: f32, c: &BlockControls, k: usize, n: usize) {
        let place = if n > 1 { k as f32 / (n - 1) as f32 * 2.0 - 1.0 } else { 0.0 };
        // Free oscillators start anywhere; a fixed unison stack starts evenly staggered.
        let phases: [[f32; 2]; OSCS] = std::array::from_fn(|o| {
            if c.oscs[o].free {
                [self.rng.unit(), self.rng.unit()]
            } else {
                [k as f32 / n as f32, 0.0]
            }
        });
        let v = &mut self.voices[slot];
        if v.sounding() {
            v.stop();
        } else {
            v.clear();
        }
        for (osc, [turn, orbit]) in v.oscs.iter_mut().zip(phases) {
            osc.set_phase(turn, orbit);
        }
        v.note = note;
        v.velocity = velocity;
        v.gate = true;
        v.age = self.age;
        v.detune = 0.5 * place * c.detune / 100.0;
        let pan = UNISON_PAN * place;
        v.pan = [(1.0 - pan).sqrt(), (1.0 + pan).sqrt()];
        v.share = 1.0 / (n as f32).sqrt();
        v.poly_pressure = 0.0;
        v.touch = self.pressure;
        v.target = note as f32;
        v.pitch = v.target;
        v.primed = [UNPRIMED; OSCS];
        v.amp.trigger();
        v.flt.trigger();
        for (m, s) in v.mods.iter_mut().zip(&c.mods) {
            m.note_on(s, true);
        }
    }

    fn release_voice(&mut self, slot: usize) {
        let v = &mut self.voices[slot];
        v.gate = false;
        v.amp.release();
        v.flt.release();
    }

    pub fn note_on(&mut self, note: u8, velocity: f32, c: &BlockControls) {
        self.follow_mode(c.mono);
        if self.mono {
            self.mono_on(note, velocity, c);
            return;
        }
        let n = c.unison.clamp(1, MAX_UNISON);
        // The same note struck again starts afresh rather than picking up its own tail.
        for v in self.voices.iter_mut().filter(|v| v.note == note && v.playing()) {
            v.fade_out();
        }
        // Make room for the whole stack: released notes go first, then the oldest.
        while self.voices.iter().filter(|v| v.playing()).count() + n > VOICES {
            let v = &self.voices;
            let Some(victim) = (0..SLOTS).filter(|&i| v[i].playing()).min_by_key(|&i| (v[i].gate, v[i].age)) else { break };
            let age = v[victim].age;
            for v in self.voices.iter_mut().filter(|v| v.playing() && v.age == age) {
                v.fade_out();
            }
        }
        self.age += 1;
        for k in 0..n {
            let slot = self.free_slot();
            self.start(slot, note, velocity, c, k, n);
        }
    }

    /// A silent slot, or failing that the fade nearest its end (cut short).
    fn free_slot(&self) -> usize {
        let v = &self.voices;
        (0..SLOTS)
            .find(|&i| !v[i].sounding())
            .or_else(|| (0..SLOTS).filter(|&i| v[i].fading).min_by(|&a, &b| v[a].fade.total_cmp(&v[b].fade)))
            .unwrap_or_else(|| (0..SLOTS).min_by_key(|&i| v[i].age).unwrap_or(0))
    }

    pub fn note_off(&mut self, note: u8) {
        if self.mono {
            self.mono_off(note);
            return;
        }
        for slot in 0..SLOTS {
            if self.voices[slot].note == note && self.voices[slot].gate {
                self.release_voice(slot);
            }
        }
    }

    /// SH-101 style: overlapping notes glide and don't retrigger; a detached note starts
    /// on its own pitch. Last note has priority, and letting go returns to the one below.
    fn mono_on(&mut self, note: u8, velocity: f32, c: &BlockControls) {
        self.remove_held(note);
        let legato = self.held_len > 0 && self.mono_count > 0 && self.voices[self.mono_slots[0]].gate;
        if self.held_len < MONO_STACK {
            self.held[self.held_len] = note;
            self.held_len += 1;
        }
        if legato {
            for &slot in &self.mono_slots[..self.mono_count] {
                let v = &mut self.voices[slot];
                v.note = note;
                v.target = note as f32;
                for (m, s) in v.mods.iter_mut().zip(&c.mods) {
                    m.note_on(s, false);
                }
            }
        } else {
            for &slot in &self.mono_slots[..self.mono_count] {
                if self.voices[slot].sounding() {
                    self.voices[slot].fade_out();
                }
            }
            let n = c.unison.clamp(1, MAX_UNISON);
            self.age += 1;
            for k in 0..n {
                let slot = self.free_slot();
                self.mono_slots[k] = slot;
                self.start(slot, note, velocity, c, k, n);
            }
            self.mono_count = n;
        }
    }

    fn mono_off(&mut self, note: u8) {
        self.remove_held(note);
        if self.mono_count == 0 {
            return;
        }
        let first = &self.voices[self.mono_slots[0]];
        if first.note != note || !first.gate {
            return;
        }
        let back = (self.held_len > 0).then(|| self.held[self.held_len - 1]);
        for k in 0..self.mono_count {
            let slot = self.mono_slots[k];
            match back {
                Some(back) => {
                    self.voices[slot].note = back;
                    self.voices[slot].target = back as f32;
                }
                None => self.release_voice(slot),
            }
        }
    }

    fn remove_held(&mut self, note: u8) {
        if let Some(i) = self.held[..self.held_len].iter().position(|&n| n == note) {
            self.held.copy_within(i + 1..self.held_len, i);
            self.held_len -= 1;
        }
    }

    /// Switching between poly and mono lets every note go.
    fn follow_mode(&mut self, mono: bool) {
        if mono != self.mono {
            self.mono = mono;
            self.held_len = 0;
            for slot in 0..SLOTS {
                if self.voices[slot].gate {
                    self.release_voice(slot);
                }
            }
        }
    }

    pub fn all_notes_off(&mut self) {
        self.held_len = 0;
        for slot in 0..SLOTS {
            self.release_voice(slot);
        }
    }

    /// The mod wheel, 0..1.
    pub fn mod_wheel(&mut self, value: f32) {
        self.wheel_target = value.clamp(0.0, 1.0);
    }

    /// Channel aftertouch, 0..1: felt by every voice.
    pub fn channel_pressure(&mut self, value: f32) {
        self.pressure = value.clamp(0.0, 1.0);
    }

    /// Polyphonic aftertouch on `note`'s voices, 0..1.
    pub fn poly_pressure(&mut self, note: u8, value: f32) {
        for v in self.voices.iter_mut().filter(|v| v.note == note && v.gate) {
            v.poly_pressure = value.clamp(0.0, 1.0);
        }
    }

    /// 0..1, centre 0.5.
    pub fn pitch_bend(&mut self, value: f32) {
        self.bend = (value * 2.0 - 1.0) * BEND_SEMITONES;
    }

    fn newest(&self) -> Option<usize> {
        (0..SLOTS).filter(|&i| self.voices[i].playing()).max_by_key(|&i| (self.voices[i].gate, self.voices[i].age))
    }

    pub fn shown(&self) -> Shown {
        match self.newest() {
            Some(i) => {
                let v = &self.voices[i];
                Shown {
                    sounding: true,
                    values: v.values,
                    orbit: std::array::from_fn(|o| v.oscs[o].orbit),
                    turn: std::array::from_fn(|o| v.oscs[o].turn),
                }
            }
            None => Shown::default(),
        }
    }

    pub fn render(&mut self, c: &BlockControls, left: &mut [f32], right: &mut [f32]) {
        let n = left.len().min(right.len()).min(MAX_BLOCK);
        let n2 = n * OVERSAMPLE;
        self.follow_mode(c.mono);
        let dt = n as f32 / self.rate;
        let ranges: [Range; TARGETS] = std::array::from_fn(range_of);
        let amp_rates = Rates::new(c.amp_env, self.os_rate);
        let flt_rates = Rates::new(c.filter_env, self.os_rate);
        let glide = if self.mono && c.glide_s > 0.001 { 1.0 - (-dt * 3.0 / c.glide_s).exp() } else { 1.0 };
        for ch in 0..2 {
            self.mix[ch][..n2].fill(0.0);
        }

        let fade_step = 1.0 / (FADE_S * self.os_rate);
        let follow = 1.0 - (-dt / CONTROLLER_S).exp();
        self.wheel += (self.wheel_target - self.wheel) * follow;
        let wheel = self.wheel;
        for slot in 0..SLOTS {
            if !self.voices[slot].sounding() {
                continue;
            }
            let v = &mut self.voices[slot];
            v.touch += ((self.pressure + v.poly_pressure).min(1.0) - v.touch) * follow;
            for t in 0..TARGETS {
                let m = &c.mods[t];
                let moving = if m.active() { v.mods[t].step(m, dt, &c.clock) } else { 0.0 };
                let held = m.wheel * wheel + m.touch * v.touch;
                v.values[t] = (c.base[t] + moving + held).clamp(0.0, 1.0);
            }
            let plain = |t: usize, v: &Voice| ranges[t].unnormalize(v.values[t]);
            v.pitch += (v.target - v.pitch) * glide;

            for buffer in self.osc_buffers.iter_mut().flatten() {
                buffer[..n].fill(0.0);
            }
            let width = plain(global_target(GlobalTarget::Width), v);
            let chorus = plain(global_target(GlobalTarget::Chorus), v);
            for o in 0..OSCS {
                let at = |k: OscTarget, v: &Voice| plain(osc_target(o, k), v);
                let level = at(OscTarget::Level, v);
                if level < 1e-4 && v.oscs[o].silent() {
                    continue;
                }
                let setup = c.oscs[o];
                let semitones =
                    v.pitch + v.detune + self.bend + (12 * setup.octave + setup.semi) as f32 + at(OscTarget::Fine, v) / 100.0;
                let hz = note_hz(semitones);
                let step = hz / self.rate;
                let start = if v.step[o] > 0.0 { v.step[o] } else { step };
                let orbit_hz = at(OscTarget::OrbitRate, v);
                let block = OscBlock {
                    bend: Bend { squish: at(OscTarget::Squish, v), skew: at(OscTarget::Skew, v), tilt: at(OscTarget::Tilt, v) },
                    center_x: at(OscTarget::Center, v),
                    probe: at(OscTarget::Ray, v),
                    orbit_depth: at(OscTarget::Orbit, v),
                    orbit_step: orbit_hz / self.rate,
                    step: [start, step],
                    width_turn: 0.25 * width,
                    level: [v.level[o], level],
                };
                let (outline, key) = match setup.element {
                    Element::Chaos => (&self.chaos[o], 64 + self.chaos_count[o]),
                    e => (&self.elements[e.index()], e.index() as u32),
                };
                // A new note, or a new shape under a held one: its past is the wrong wave.
                if v.primed[o] != key {
                    v.oscs[o].prime(outline, &block);
                    v.primed[o] = key;
                }
                let [l, r] = &mut self.osc_buffers[o];
                v.oscs[o].render(outline, &block, &mut l[..n], &mut r[..n]);
                v.chorus[o].process(&mut l[..n], &mut r[..n], chorus, self.rate);
                v.step[o] = step;
                v.level[o] = level;
            }

            let drive = TubeDrive::new(plain(global_target(GlobalTarget::Drive), v));
            let resonance = plain(global_target(GlobalTarget::Resonance), v);
            let mode = plain(global_target(GlobalTarget::FilterMode), v);
            let env_amount = plain(global_target(GlobalTarget::FilterEnv), v) * FILTER_ENV_OCTAVES;
            let keyed = ((v.pitch - 60.0) / 12.0 * c.keytrack).exp2();
            let cutoff_base = plain(global_target(GlobalTarget::Cutoff), v) * keyed;
            let gain = VOICE_GAIN * v.share * (1.0 - c.velocity + c.velocity * v.velocity);
            let mut up = [[0.0f32; MAX_BLOCK * OVERSAMPLE]; 2];
            for ch in 0..2 {
                for i in 0..n {
                    let x = (0..OSCS).map(|o| self.osc_buffers[o][ch][i]).sum();
                    let [a, b] = v.up[ch].process(&self.taps, x);
                    up[ch][2 * i] = a;
                    up[ch][2 * i + 1] = b;
                }
            }
            for j in 0..n2 {
                let fe = v.flt.next(&flt_rates);
                let mut ae = v.amp.next(&amp_rates);
                if v.fading {
                    v.fade = (v.fade - fade_step).max(0.0);
                    ae *= v.fade;
                }
                if j % COEFFICIENT_STEP == 0 {
                    let cutoff = (cutoff_base * (env_amount * fe).exp2()).clamp(20.0, 0.45 * self.os_rate);
                    match c.filter {
                        FilterKind::Svf => {
                            v.svf[0].set(cutoff, resonance, self.os_rate);
                            let [left, right] = &mut v.svf;
                            right.follow(left);
                        }
                        FilterKind::Ladder => {
                            v.ladder[0].set(cutoff, resonance, self.os_rate);
                            let [left, right] = &mut v.ladder;
                            right.follow(left);
                        }
                    }
                }
                for ch in 0..2 {
                    let x = v.tube[ch].process(up[ch][j], &drive);
                    let y = match c.filter {
                        FilterKind::Svf => v.svf[ch].morph(x, mode),
                        FilterKind::Ladder => v.ladder[ch].low(x),
                    };
                    self.mix[ch][j] += y * ae * gain * v.pan[ch];
                }
            }
            if v.fading && v.fade <= 0.0 {
                v.stop();
            } else if !v.sounding() {
                v.clear();
            }
        }

        for i in 0..n {
            let l = self.decimators[0].process(&self.taps, self.mix[0][2 * i], self.mix[0][2 * i + 1]);
            let r = self.decimators[1].process(&self.taps, self.mix[1][2 * i], self.mix[1][2 * i + 1]);
            let [l, r] = self.limiter.process([l * c.volume, r * c.volume]);
            left[i] = l;
            right[i] = r;
        }
    }
}

/// How many controls each oscillator has, and how many are shared (for the panel).
pub const PER_OSC: usize = OSC_TARGETS.len();
pub const SHARED: usize = GLOBAL_TARGETS.len();

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dsp::modulator::LfoShape;

    const SR: f32 = 48_000.0;

    fn run(e: &mut Engine, c: &BlockControls, seconds: f32) -> Vec<[f32; 2]> {
        let mut out = Vec::new();
        let (mut l, mut r) = ([0.0; MAX_BLOCK], [0.0; MAX_BLOCK]);
        for _ in 0..(seconds * SR / MAX_BLOCK as f32) as usize {
            e.render(c, &mut l, &mut r);
            out.extend(l.iter().zip(&r).map(|(a, b)| [*a, *b]));
        }
        out
    }

    fn rms(x: &[[f32; 2]]) -> f32 {
        (x.iter().map(|[l, r]| l * l + r * r).sum::<f32>() / (2 * x.len().max(1)) as f32).sqrt()
    }

    #[test]
    fn silent_until_played_and_quiet_after_release() {
        let mut e = Engine::new(SR);
        let c = BlockControls::default();
        assert!(run(&mut e, &c, 0.2).iter().all(|[l, r]| *l == 0.0 && *r == 0.0));
        e.note_on(45, 1.0, &c);
        let held = run(&mut e, &c, 0.5);
        // A centred Earth is nearly a circle, so it's quiet: about -49 dB.
        assert!(rms(&held) > 0.002, "{}", rms(&held));
        e.note_off(45);
        run(&mut e, &c, 2.0);
        assert!(rms(&run(&mut e, &c, 0.2)) < 1e-4);
    }

    #[test]
    fn every_voice_then_steals_whole_stacks_and_stays_finite_with_everything_moving() {
        let mut e = Engine::new(SR);
        let mut c = BlockControls { unison: 2, ..BlockControls::default() };
        for o in 0..OSCS {
            c.oscs[o].element = [Element::Fire, Element::Chaos][o];
            c.set(osc_target(o, OscTarget::Level), 0.8);
            c.set(osc_target(o, OscTarget::Orbit), 0.4);
            c.set(osc_target(o, OscTarget::Ray), 1.2);
        }
        c.set(osc_target(1, OscTarget::OrbitRate), 400.0);
        c.set(global_target(GlobalTarget::Chorus), 0.3);
        c.set(global_target(GlobalTarget::Resonance), 1.0);
        c.set(global_target(GlobalTarget::Drive), 1.0);
        for t in 0..TARGETS {
            c.mods[t] = ModSettings { shape: LfoShape::SampleHold, hz: 9.0, lfo: 0.7, env: -0.5, ..ModSettings::default() };
        }
        for note in [36, 43, 48, 52, 55, 60, 64, 67] {
            e.note_on(note, 0.9, &c);
        }
        // Eight notes of two voices want sixteen: the two oldest notes give theirs up.
        assert_eq!(e.voices.iter().filter(|v| v.gate).count(), VOICES);
        assert!(e.voices.iter().filter(|v| v.gate).all(|v| v.note >= 48));
        let out = run(&mut e, &c, 1.0);
        assert!(out.iter().all(|[l, r]| l.is_finite() && r.is_finite() && l.abs() <= 1.0 && r.abs() <= 1.0));
        assert!(rms(&out) > 0.01);
    }

    #[test]
    fn mono_glides_between_overlapping_notes_without_retriggering() {
        let mut e = Engine::new(SR);
        let c = BlockControls { mono: true, glide_s: 0.2, ..BlockControls::default() };
        e.note_on(48, 1.0, &c);
        run(&mut e, &c, 0.3);
        e.note_on(60, 1.0, &c);
        run(&mut e, &c, 0.02);
        let midway = e.voices[0].pitch;
        assert!(midway > 48.5 && midway < 59.5, "{midway}");
        assert_eq!(e.voices.iter().filter(|v| v.sounding()).count(), 1);
        e.note_off(60);
        run(&mut e, &c, 0.5);
        assert!((e.voices[0].pitch - 48.0).abs() < 0.1, "back to the held note");
        e.note_off(48);
        run(&mut e, &c, 1.5);
        assert!(!e.voices[0].sounding());
    }

    fn spectrum_power(x: &[f32], hz: f32) -> f64 {
        let (mut re, mut im) = (0.0f64, 0.0f64);
        for (i, v) in x.iter().enumerate() {
            let a = 2.0 * std::f64::consts::PI * hz as f64 * i as f64 / SR as f64;
            re += *v as f64 * a.cos();
            im += *v as f64 * a.sin();
        }
        (re * re + im * im).sqrt()
    }

    #[test]
    fn drive_adds_harmonics_and_keeps_its_level() {
        let play = |drive: f32| {
            let mut e = Engine::new(SR);
            let mut c = BlockControls::default();
            c.set(osc_target(0, OscTarget::Center), 0.25);
            c.set(global_target(GlobalTarget::Cutoff), 20_000.0);
            c.set(global_target(GlobalTarget::FilterEnv), 0.0);
            c.set(global_target(GlobalTarget::Width), 0.0);
            c.set(global_target(GlobalTarget::Drive), drive);
            c.filter = FilterKind::Svf;
            e.note_on(45, 1.0, &c);
            let out = run(&mut e, &c, 1.0);
            let x: Vec<f32> = out[24_000..].iter().map(|s| s[0]).collect();
            let level = (x.iter().map(|v| v * v).sum::<f32>() / x.len() as f32).sqrt();
            let upper: f64 = (3..12).map(|k| spectrum_power(&x, 110.0 * k as f64 as f32)).sum();
            let mean = x.iter().sum::<f32>() / x.len() as f32;
            (level, upper / spectrum_power(&x, 110.0), mean)
        };
        let (clean_level, clean_ratio, clean_mean) = play(0.0);
        let (driven_level, driven_ratio, _) = play(1.0);
        assert!(clean_mean.abs() < 0.01 * clean_level, "the output is centred: {clean_mean}");
        assert!(driven_ratio > 3.0 * clean_ratio, "harmonics {clean_ratio} -> {driven_ratio}");
        let db = 20.0 * (driven_level / clean_level).log10();
        assert!(db.abs() < 3.0, "level moved {db} dB");
    }

    #[test]
    fn notes_and_shape_changes_start_centred() {
        // Earth with the ray off axis sits well above zero; an unprimed past let that offset
        // through for the first period, as a thump.
        let mut e = Engine::new(SR);
        let mut c = BlockControls::default();
        c.set(osc_target(0, OscTarget::Ray), 1.41);
        c.set(global_target(GlobalTarget::Cutoff), 20_000.0);
        c.set(global_target(GlobalTarget::FilterEnv), 0.0);
        c.set(global_target(GlobalTarget::Width), 0.3);
        let window = (0.01 * SR) as usize;
        let offset = |x: &[[f32; 2]]| (0..2).map(|ch| x.iter().map(|s| s[ch]).sum::<f32>() / x.len() as f32).fold(0.0f32, |a, m| a.max(m.abs()));
        let rms = |x: &[[f32; 2]]| (x.iter().map(|s| s[0] * s[0]).sum::<f32>() / x.len() as f32).sqrt();
        e.note_on(62, 1.0, &c);
        let first = run(&mut e, &c, 0.3);
        let steady = rms(&first[9_600..]);
        assert!(offset(&first[..window]) < 0.2 * steady, "note: {} vs {steady}", offset(&first[..window]));
        c.oscs[0].element = Element::Water;
        let switched = run(&mut e, &c, 0.3);
        assert!(offset(&switched[..window]) < 0.2 * steady, "switch: {} vs {steady}", offset(&switched[..window]));
    }

    #[test]
    fn a_stolen_voice_fades_out_instead_of_clicking() {
        let mut e = Engine::new(SR);
        let mut c = BlockControls::default();
        c.oscs[0].element = Element::Air;
        c.set(global_target(GlobalTarget::Cutoff), 800.0);
        c.set(global_target(GlobalTarget::FilterEnv), 0.0);
        c.set(global_target(GlobalTarget::Width), 0.5);
        c.amp_env.attack_s = 0.01;
        for k in 0..VOICES {
            e.note_on(40 + 2 * k as u8, 1.0, &c);
        }
        let before = run(&mut e, &c, 0.3);
        e.note_on(80, 1.0, &c);
        assert_eq!(e.voices.iter().filter(|v| v.playing()).count(), VOICES);
        assert_eq!(e.voices.iter().filter(|v| v.fading).count(), 1);
        let after = run(&mut e, &c, 0.3);
        assert!(e.voices.iter().all(|v| !v.fading), "the fade is over");
        // The largest step from one sample to the next, either side.
        let step = |x: &[[f32; 2]]| x.windows(2).map(|w| (w[1][0] - w[0][0]).abs().max((w[1][1] - w[0][1]).abs())).fold(0.0f32, f32::max);
        let steady = step(&before[4_800..]).max(step(&after[4_800..]));
        let at_steal = step(&[&before[before.len() - 8..], &after[..960]].concat());
        assert!(at_steal < 1.2 * steady, "steal {at_steal} vs steady {steady}");
    }

    #[test]
    fn mono_unison_stacks_detuned_voices_across_the_field() {
        let mut e = Engine::new(SR);
        let c = BlockControls { mono: true, unison: 3, detune: 30.0, ..BlockControls::default() };
        e.note_on(48, 1.0, &c);
        e.note_on(55, 1.0, &c);
        let held: Vec<&Voice> = e.voices.iter().filter(|v| v.gate).collect();
        assert_eq!(held.len(), 3);
        assert!(held.iter().all(|v| v.target == 55.0), "legato moves the whole stack");
        let mut detunes: Vec<f32> = held.iter().map(|v| v.detune * 100.0).collect();
        detunes.sort_by(f32::total_cmp);
        assert!((detunes[0] + 15.0).abs() < 1e-3 && detunes[1].abs() < 1e-3 && (detunes[2] - 15.0).abs() < 1e-3, "{detunes:?}");
        assert!(held.iter().any(|v| v.pan[0] > v.pan[1]) && held.iter().any(|v| v.pan[1] > v.pan[0]));
        let out = run(&mut e, &c, 0.3);
        assert!(rms(&out) > 0.002);
    }

    #[test]
    fn the_mod_wheel_and_aftertouch_move_their_targets() {
        let mut e = Engine::new(SR);
        let mut c = BlockControls::default();
        let cutoff = global_target(GlobalTarget::Cutoff);
        c.mods[cutoff].wheel = 0.3;
        c.mods[cutoff].touch = -0.2;
        e.note_on(60, 1.0, &c);
        e.mod_wheel(1.0);
        run(&mut e, &c, 0.1);
        let moved = e.shown().values[cutoff] - c.base[cutoff];
        assert!((moved - 0.3).abs() < 1e-3, "{moved}");
        e.channel_pressure(0.5);
        run(&mut e, &c, 0.1);
        let moved = e.shown().values[cutoff] - c.base[cutoff];
        assert!((moved - 0.2).abs() < 1e-3, "{moved}");
    }

    #[test]
    fn free_oscillators_start_anywhere() {
        let mut e = Engine::new(SR);
        let mut c = BlockControls::default();
        c.oscs[0].free = true;
        let starts: Vec<f32> = (0..4)
            .map(|k| {
                e.note_on(60 + k, 1.0, &c);
                e.voices.iter().filter(|v| v.gate && v.note == 60 + k).map(|v| v.oscs[0].turn).next().unwrap()
            })
            .collect();
        assert!(starts.iter().any(|t| *t > 0.05), "{starts:?}");
        c.oscs[0].free = false;
        e.note_on(70, 1.0, &c);
        assert_eq!(e.voices.iter().find(|v| v.note == 70).unwrap().oscs[0].turn, 0.0);
    }

    #[test]
    fn a_skewed_square_sounds_an_octave_up() {
        // The browser prototype measured this; the plugin should agree.
        let mut e = Engine::new(SR);
        let mut c = BlockControls::default();
        c.oscs[0].element = Element::Air;
        c.set(osc_target(0, OscTarget::Skew), 0.5);
        c.set(global_target(GlobalTarget::Cutoff), 20_000.0);
        c.set(global_target(GlobalTarget::FilterEnv), 0.0);
        c.set(global_target(GlobalTarget::Width), 0.0);
        e.note_on(45, 1.0, &c); // 110 Hz
        let out = run(&mut e, &c, 1.0);
        let x: Vec<f32> = out[24_000..].iter().map(|s| s[0]).collect();
        let power = |hz: f32| {
            let (mut re, mut im) = (0.0f64, 0.0f64);
            for (i, v) in x.iter().enumerate() {
                let a = 2.0 * std::f64::consts::PI * hz as f64 * i as f64 / SR as f64;
                re += *v as f64 * a.cos();
                im += *v as f64 * a.sin();
            }
            (re * re + im * im).sqrt()
        };
        assert!(power(220.0) > 30.0 * power(110.0), "220: {} 110: {}", power(220.0), power(110.0));
    }
}
