//! What the amber screens show. Most pictures are computed from the
//! current parameter values (and, for waves and samples, the real tables),
//! so they follow knob moves and automation. While a note plays, the
//! screens whose values move follow the newest voice instead: the filter's
//! response, the selected knob's modulators. The oscillator slots' screens
//! are oscilloscopes of the real oscillators (see `scopes`).

use std::f32::consts::PI;

use nih_plug_vizia::vizia::vg::Path;

use super::draw::{Painter, Text, AMBER, AMBER_DIM, SCREEN_GLASS, SOURCE_COLORS};
use super::layout::{Rect, Screen, SLOT_DUAL, SLOT_WAVETABLE, SLOT_WINDOWS};
use super::light;
use super::scopes::Scopes;
use crate::modulation;
use crate::params::SynthParams;
use crate::windows::{self, WindowsSettings};
use crate::scope::{Moving, Tap, MOVING};
use crate::{ortholib, translib, wavelib};

/// What the screens draw from this frame.
pub struct Live {
    /// The two oscillator slots' traces.
    pub traces: [Vec<f32>; 2],
    /// Whether a note is playing (the traces and `moving` are its).
    pub playing: bool,
    scopes_moving: [Option<f32>; MOVING],
    /// Where the newest voice's modulators have each knob (travel), while a note plays.
    moved: Option<[f32; modulation::COUNT]>,
    /// The newest voice's sources, and each wavetable's place on its path.
    sources: Option<[f32; modulation::SOURCE_COUNT]>,
    paths: Option<[[f32; 2]; 2]>,
    /// The pitch the screens draw for (the last note played).
    pub note_hz: f32,
}

impl Live {
    pub fn from(scopes: &mut Scopes, params: &SynthParams) -> Self {
        use Moving::*;
        let all = [Cutoff, Resonance, A, B, Hpf, HpfPeak];
        let live = scopes.is_live();
        Self {
            traces: [scopes.trace(params, Tap::Osc1), scopes.trace(params, Tap::Osc2)],
            playing: live,
            scopes_moving: all.map(|m| scopes.moving(m)),
            moved: live.then(|| std::array::from_fn(|t| scopes.moved(t).unwrap_or(0.0))),
            sources: scopes.sources().filter(|_| live),
            paths: scopes.paths().filter(|_| live),
            note_hz: scopes.note_hz(),
        }
    }

    /// A moving value of the playing voice, if one plays.
    fn get(&self, value: Moving) -> Option<f32> {
        self.scopes_moving[value as usize]
    }
}

/// Plain value of a parameter (0 if the id is unknown).
fn plain(params: &SynthParams, id: &str) -> f32 {
    params.ptr(id).map(|p| unsafe { p.unmodulated_plain_value() }).unwrap_or(0.0)
}

fn name(params: &SynthParams, id: &str) -> String {
    params
        .ptr(id)
        .map(|p| unsafe { p.normalized_value_to_string(p.unmodulated_normalized_value(), false) })
        .unwrap_or_default()
}

/// A polyline through `f(t)` for t in 0..1, where f gives -1..1 (up is +1).
fn plot(r: Rect, n: usize, f: impl Fn(f32) -> f32) -> Path {
    let mut p = Path::new();
    for i in 0..=n {
        let t = i as f32 / n as f32;
        let (x, y) = (r.x + t * r.w, r.y + r.h / 2.0 - f(t).clamp(-1.2, 1.2) * r.h / 2.0);
        if i == 0 { p.move_to(x, y) } else { p.line_to(x, y) }
    }
    p
}

pub fn draw(p: &mut Painter, params: &SynthParams, live: &Live, screen: Screen, r: Rect) {
    let area = r.inset(8.0, 8.0, 8.0, 8.0);
    let when = if live.playing { "live" } else { "at note-on + 80 ms" };
    match screen {
        Screen::Scope(slot) => {
            let k = slot + 1;
            let label = match plain(params, &format!("osc_{k}.mode")) as usize {
                SLOT_DUAL => format!("{} · {}/{} · {when}", name(params, &format!("dual_{k}.algorithm")),
                                     short_wave(&name(params, &format!("dual_{k}.wave_1"))),
                                     short_wave(&name(params, &format!("dual_{k}.wave_2")))),
                SLOT_WAVETABLE => match name(params, &format!("wt_{k}.oscmod")) {
                    m if m == "Off" => when.to_string(),
                    m => format!("{m} · {when}"),
                },
                SLOT_WINDOWS => {
                    let on = (1..=3).filter(|i| plain(params, &format!("windows_{k}.int_{i}_on")) > 0.5).count();
                    format!("{} · {on} on · {when}", name(params, &format!("windows_{k}.base")))
                }
                _ => format!("{} · {when}", name(params, &format!("ray_{k}.element"))),
            };
            if plain(params, &format!("osc_{k}.mode")) as usize == SLOT_WINDOWS {
                windows_scope(p, params, live, k, &live.traces[slot], &label, area);
            } else {
                scope_trace(p, &live.traces[slot], &label, area);
            }
        }
        Screen::DriveCurve => drive_curve(p, params, r),
        Screen::WavePath(slot) => wave_path(p, params, live, slot, r.inset(6.0, 6.0, 6.0, 6.0)),
        Screen::Noise => noise(p, params, area),
        Screen::Transient => transient(p, params, area),
        Screen::Filter => filter(p, params, live, area),
        Screen::Meter(source) => meter(p, live, source, r),
        Screen::Harmonic => harmonic(p, params, area),
    }
}

/// A wave picker's picture: a wavetable's wave as its OscMod reshapes it
/// (at the knob's depth), or an LFO's shape (`lfo`), with its name when it's
/// one of the LFO's own.
pub fn wave_box(p: &mut Painter, params: &SynthParams, live: &Live, id: &str, lfo: bool, r: Rect, hot: bool) {
    let area = r.inset(5.0, 5.0, 5.0, 5.0);
    centre_line(p, area);
    let value = plain(params, id) as usize;
    let alpha = if hot { 1.0 } else { 0.9 };
    if lfo {
        let shape = modulation::LfoShape::from_menu(value);
        let wave = modulation::sketch(shape, 80);
        p.trace(&plot(area, 79, |t| wave[(t * 79.0).round() as usize] * 0.85), alpha);
        return;
    }
    let k = &id["wt_".len()..="wt_1".len() - 1];
    let cycle = osc_mod_cycle(value, plain(params, &format!("wt_{k}.oscmod")) as i32,
                              plain(params, &format!("wt_{k}.oscmod_depth")), live.note_hz, 120);
    let n = cycle.len() - 1;
    p.trace(&plot(area, n, |t| cycle[(t * n as f32).round() as usize] * 0.85), alpha);
}

/// A wavetable's path, under its five waves (`r` spans them): envelope A's
/// outline, wave to wave: the attack up from 1 to 2, the decay down to 3, the
/// sustain bouncing between 3 and 4, the release down to 5. While a note
/// plays, a dot where the newest voice is.
fn wave_path(p: &mut Painter, params: &SynthParams, live: &Live, slot: usize, r: Rect) {
    // The waves' centres (as layout's wavetable places them: 66 wide, 24 apart).
    let centre = |k: f32| r.x - 6.0 + (33.0 + 90.0 * k) / 426.0 * (r.w + 12.0);
    let sustain = plain(params, "env_a.sustain");
    let (abow, drbow) = (plain(params, "env_a.attack_curve"), plain(params, "env_a.dr_curve"));
    let y = |level: f32| r.bottom() - level * r.h;
    let level = |x: f32| -> f32 {
        use modulation::along;
        if x < centre(1.0) {
            along(0.0, 1.0, (x - centre(0.0)) / (centre(1.0) - centre(0.0)), abow)
        } else if x < centre(2.0) {
            along(1.0, sustain, (x - centre(1.0)) / (centre(2.0) - centre(1.0)), drbow)
        } else if x < centre(3.0) {
            sustain
        } else {
            along(sustain, 0.0, (x - centre(3.0)) / (centre(4.0) - centre(3.0)), drbow)
        }
    };
    for k in 0..5 {
        p.screen_line(centre(k as f32), r.y - 3.0, centre(k as f32), r.bottom() + 3.0);
    }
    let mut outline = Path::new();
    for i in 0..=160 {
        let x = centre(0.0) + (centre(4.0) - centre(0.0)) * i as f32 / 160.0;
        if i == 0 { outline.move_to(x, y(level(x))) } else { outline.line_to(x, y(level(x))) }
    }
    p.stroke_colour(&outline, SOURCE_COLORS[modulation::ENV_A], 1.3, 0.8);
    // The sustain's bounce.
    let mut zigzag = Path::new();
    for i in 0..=8 {
        let x = centre(2.0) + (centre(3.0) - centre(2.0)) * i as f32 / 8.0;
        let zy = r.y + 6.0 + if i % 2 == 1 { 8.0 } else { 0.0 };
        if i == 0 { zigzag.move_to(x, zy) } else { zigzag.line_to(x, zy) }
    }
    p.trace(&zigzag, 0.8);
    if let Some([morph, fade]) = live.paths.map(|paths| paths[slot]) {
        let at = centre(morph) + (centre(4.0) - centre(morph)) * fade;
        p.dot(at, y(level(at)), 3.0, 0xffd27a);
    }
}

/// A performance source's level now (the key from the middle), or nothing.
fn meter(p: &mut Painter, live: &Live, source: usize, r: Rect) {
    let Some(sources) = live.sources else { return };
    let v = sources[source];
    let colour = SOURCE_COLORS[source];
    let inner = r.inset(1.5, 1.5, 1.5, 1.5);
    if source == modulation::KEY {
        let mid = inner.y + inner.h / 2.0;
        let to = mid - (v * 5.0).clamp(-1.0, 1.0) * inner.h / 2.0;
        p.flat(Rect::new(inner.x, to.min(mid), inner.w, (to - mid).abs().max(1.0)), colour);
    } else {
        let h = v.clamp(0.0, 1.0) * inner.h;
        p.flat(Rect::new(inner.x, inner.bottom() - h, inner.w, h), colour);
    }
}

fn centre_line(p: &mut Painter, r: Rect) {
    p.screen_line(r.x - 4.0, r.y + r.h / 2.0, r.right() + 4.0, r.y + r.h / 2.0);
}

// ---- Oscillators -------------------------------------------------------------

/// A basic wave's name as the scope's label writes it ("saw").
fn short_wave(name: &str) -> String {
    match name {
        "Triangle" => "tri".into(),
        "Square" => "sqr".into(),
        n => n.to_lowercase(),
    }
}

/// An oscillator's real output, scaled to fit, with a label.
fn scope_trace(p: &mut Painter, trace: &[f32], label: &str, r: Rect) {
    centre_line(p, r);
    let peak = trace.iter().fold(0.05f32, |m, v| m.max(v.abs()));
    let n = trace.len().max(2) - 1;
    p.trace(&plot(r, n, |t| trace.get((t * n as f32).round() as usize).copied().unwrap_or(0.0) / peak * 0.9), 1.0);
    p.text(r.x, r.y + 2.0, label, Text::mono(9.0, AMBER_DIM).left());
}

/// A Windows slot's settings as the scope draws them: while a note plays,
/// where the newest voice's modulators have its X, Phase, Len and Pitch (as
/// the pink arcs show), else the knobs.
fn windows_settings(params: &SynthParams, live: &Live, k: usize) -> WindowsSettings {
    let value = |id: String| -> f32 {
        let moved = live.moved.as_ref().zip(modulation::target_index(&id)).map(|(m, t)| m[t]);
        match (moved, params.ptr(&id)) {
            (Some(n), Some(ptr)) => unsafe { ptr.preview_plain(n) },
            _ => plain(params, &id),
        }
    };
    WindowsSettings::from_values(crate::params::WINDOWS_FIELDS.map(|f| value(f.replace('N', &k.to_string()))))
}

/// The trace turned (circularly, as it's one cycle) to line up best with the
/// plain wave from the base cycle's start, so the windows sit where they open.
fn line_up(trace: &[f32], ideal: &[f32]) -> Vec<f32> {
    let n = ideal.len();
    if trace.len() < n + 1 {
        return trace.to_vec();
    }
    let score = |shift: usize| -> f32 { (0..n).map(|i| trace[(i + shift) % n] * ideal[i]).sum() };
    let best = (0..n).max_by(|&a, &b| score(a).total_cmp(&score(b))).unwrap_or(0);
    (0..=n).map(|i| trace[(i + best) % n]).collect()
}

/// A Windows slot's scope in additive light: each open window a faint fill
/// of its interrupt's colour between bright lines at its edges, overlaps
/// adding up; the trace drawn dark where the fill gets bright.
fn windows_scope(p: &mut Painter, params: &SynthParams, live: &Live, k: usize, trace: &[f32], label: &str, r: Rect) {
    let s = windows_settings(params, live, k);
    let n = trace.len().max(2) - 1;
    let step = (live.note_hz / 48_000.0) as f64;
    let trace = line_up(trace, &windows::ideal(&s, step, n));
    let x = |phase: f64| r.x + phase as f32 * r.w;
    let mut dark = Vec::new();
    for (a, b, lit) in windows::spans(&s).into_iter().filter(|s| s.2 != 0) {
        let (colour, invert) = light::fill(SCREEN_GLASS, lit);
        p.flat(Rect::new(x(a), r.y, x(b) - x(a), r.h), colour);
        if invert {
            dark.push((x(a), x(b)));
        }
    }
    for (at, i) in windows::edges(&s) {
        p.screen_line_coloured(x(at), r.y, x(at), r.bottom(), light::edge(i), 1.5);
    }
    centre_line(p, r);
    let peak = trace.iter().fold(0.05f32, |m, v| m.max(v.abs()));
    let path = plot(r, n, |t| trace.get((t * n as f32).round() as usize).copied().unwrap_or(0.0) / peak * 0.9);
    p.trace(&path, 1.0);
    for (a, b) in dark {
        p.trace_dark(&path, Rect::new(a, r.y - 4.0, b - a, r.h + 8.0));
    }
    p.text(r.x, r.y + 2.0, label, Text::mono(9.0, AMBER_DIM).left());
}

/// One cycle (`n` + 1 points) of library wave `w` as the Wavetable's OscMod
/// reshapes it (engines.lib's oscMod) for a note at `hz`. The sample-rate
/// and Simplify modes depend on the pitch; they run at 48 kHz here.
fn osc_mod_cycle(w: usize, kind: i32, depth: f32, hz: f32, n: usize) -> Vec<f32> {
    const RATE: f32 = 48_000.0;
    let d = depth.clamp(0.0, 1.0);
    let read = |q: f32| wave_at(w, q.rem_euclid(1.0));
    let tri0 = |p: f32| 1.0 - 4.0 * ((p + 0.25).rem_euclid(1.0) - 0.5).abs();
    let pick = |first: i32, targets: [f32; 3]| targets[(kind - first).clamp(0, 2) as usize];
    let shaped = |p: f32| -> f32 {
        let dry = read(p);
        let blend = |wet: f32| (1.0 - d) * dry + d * wet;
        match kind {
            1 => {
                let q = 2f32.powf(16.0 - d * 12.0 - 1.0);
                (dry * q + 0.5).floor() / q
            }
            2..=4 => {
                let rate = (pick(2, [11_000.0, 8_000.0, 4_000.0]) / RATE).powf(d).min(0.999) * RATE;
                let step = hz.max(1.0) / rate; // one held sample, in cycles
                read((p / step).floor() * step)
            }
            8 => blend(if p < 0.5 { read(2.0 * p).abs() } else { 0.0 }),
            9 => tri0(dry * (1.0 + 4.0 * d) / 4.0),
            10 => blend((0.5 * PI * dry * 5.0).sin()),
            11 => read(p.powf(1.0 + 2.0 * d)),
            12 => dry * ((1.0 - d) + d * (2.0 * PI * p * (1.0 + 7.0 * d)).sin()),
            13 => blend(read(if p < 0.5 { 2.0 * p } else { 2.0 - 2.0 * p })),
            14 => blend(if dry > 0.0 { 1.0 - dry } else { -1.0 - dry }),
            _ => dry,
        }
    };
    if !(5..=7).contains(&kind) {
        return (0..=n).map(|i| shaped(i as f32 / n as f32)).collect();
    }
    // Simplify: a 2-pole Butterworth low-pass, run over a few cycles at
    // the real rate; the last cycle is shown.
    let top = 22_000f32.min(0.45 * RATE);
    let cutoff = top * (pick(5, [6_000.0, 1_500.0, 800.0]) / top).powf(d);
    let per_cycle = (RATE / hz.max(1.0)).max(8.0) as usize;
    let k = (PI * cutoff / RATE).tan();
    let norm = 1.0 / (1.0 + std::f32::consts::SQRT_2 * k + k * k);
    let (a0, a1, a2) = (k * k * norm, 2.0 * k * k * norm, k * k * norm);
    let (b1, b2) = (2.0 * (k * k - 1.0) * norm, (1.0 - std::f32::consts::SQRT_2 * k + k * k) * norm);
    let (mut x1, mut x2, mut y1, mut y2) = (0.0, 0.0, 0.0, 0.0);
    let mut last = Vec::with_capacity(per_cycle);
    for i in 0..4 * per_cycle {
        let x = read((i % per_cycle) as f32 / per_cycle as f32);
        let y = a0 * x + a1 * x1 + a2 * x2 - b1 * y1 - b2 * y2;
        (x2, x1, y2, y1) = (x1, x, y1, y);
        if i >= 3 * per_cycle {
            last.push(y);
        }
    }
    (0..=n).map(|i| last[(i * per_cycle / (n + 1)).min(per_cycle - 1)]).collect()
}

/// Library wave `w` at t (0..1) from its copy band-limited to 16
/// harmonics: its outline, for tiny pictures (menus) where the full wave
/// would alias into noise.
pub fn wave_outline(w: usize, t: f32) -> f32 {
    const LEVEL: usize = 4;
    let (offset, size) = (wavelib::LEVEL_OFFSETS[LEVEL], wavelib::LEVEL_SIZES[LEVEL]);
    let x = t.rem_euclid(1.0) * size as f32;
    let (i, fr) = (x as usize % size, x.fract());
    let at = |k: usize| wavelib::sample((w * wavelib::WAVE_STRIDE + offset + k % size) as i32);
    at(i) + (at(i + 1) - at(i)) * fr
}

/// Sample t (0..1) of library wave `w`, from its full-band copy.
fn wave_at(w: usize, t: f32) -> f32 {
    const STRIDE: usize = 2176;
    const SIZE: usize = 1024;
    let i = ((t.fract() * SIZE as f32) as usize).min(SIZE - 1);
    wavelib::sample((w * STRIDE + i) as i32)
}

/// The multidrive's transfer curve at the current setting (as core.lib's
/// multiDrive, without the bias jitter): input -1..1 across, output up,
/// scaled to fit.
fn drive_curve(p: &mut Painter, params: &SynthParams, frame: Rect) {
    let r = frame.inset(4.0, 4.0, 4.0, 4.0);
    let x = plain(params, "drive.amount").clamp(0.0, 1.0);
    let name = ["soft", "hard", "tube", "fold"][((4.0 * x) as usize).min(3)];
    p.text(frame.x + frame.w / 2.0, frame.bottom() + 12.0, name, Text::mono(9.5, AMBER));
    let gain = 10f32.powf(24.0 * (0.5 * x + 0.5 * x.powi(4)) / 20.0);
    let pos = 4.0 * x - 0.5;
    let k = pos.floor().clamp(0.0, 2.0);
    let fade = ((pos - k - 0.3) / 0.4).clamp(0.0, 1.0);
    let curve = |kind: f32, v: f32| match kind as i32 {
        0 => 2.0 / PI * (v * PI / 2.0).atan(),
        1 => v.clamp(-1.0, 1.0),
        2 => if v < 0.0 { v.tanh() } else { 1.6 * (v / 1.6).tanh() },
        _ => v.sin(),
    };
    let shaped = |v: f32| (1.0 - fade) * curve(k, gain * v) + fade * curve(k + 1.0, gain * v);
    let peak = (0..=100).map(|i| shaped(i as f32 / 50.0 - 1.0).abs()).fold(1e-6, f32::max);
    p.screen_line(r.x, r.y + r.h / 2.0, r.right(), r.y + r.h / 2.0);
    p.screen_line(r.x + r.w / 2.0, r.y, r.x + r.w / 2.0, r.bottom());
    p.trace(&plot(r, 120, |t| shaped(2.0 * t - 1.0) / peak * 0.9), 1.0);
}

// ---- Noise and transient -------------------------------------------------------

/// A repeatable pseudo-random value for step `n`.
fn hash(n: u32) -> f32 {
    let mut x = n.wrapping_mul(0x9e37_79b9) ^ 0x85eb_ca6b;
    x ^= x >> 15;
    x = x.wrapping_mul(0x2c1b_3c6d);
    x ^= x >> 12;
    (x & 0xffff) as f32 / 32767.5 - 1.0
}

fn noise(p: &mut Painter, params: &SynthParams, r: Rect) {
    centre_line(p, r);
    let kind = plain(params, "noise.type") as i32;
    let bits = plain(params, "noise.bits").max(1.0);
    let q = 2f32.powf(bits - 1.0);
    let quantize = |v: f32| (v * q).round() / q;
    // Steps across the screen: white is per point; clocked holds longer at
    // lower clock ratios; LFSR short repeats; crackle is sparse impulses.
    let hold = 2f32.powf(-plain(params, "noise.clock_ratio")) * 4.0;
    let density = plain(params, "noise.crackle_density");
    let f = move |t: f32| {
        let n = (t * 240.0) as u32;
        0.8 * match kind {
            0 => hash(n),
            1 => quantize(hash((n as f32 / hold) as u32)),
            2 => quantize(hash((n as f32 / hold) as u32 * 7 + 1)),
            3 => quantize(hash(((n as f32 / hold) as u32) % 9 + 3)),
            _ => {
                let v = hash(n * 3 + 11);
                if (hash(n * 5 + 7) + 1.0) / 2.0 < density * 0.15 { v.signum() } else { 0.0 }
            }
        }
    };
    p.trace(&plot(r, 240, f), 0.9);
}

/// The FM transient's two modulator depths at the strike for its Depth
/// knob `k`, as engines.lib's `fmDepths`: modulator 1 follows the knob (from
/// 5%); modulator 2 waits a quarter, creeps to 25% by three quarters, then
/// catches up to 100%.
fn fm_depths(k: f32) -> (f32, f32) {
    let d2 = if k < 0.25 { 0.0 } else if k < 0.75 { (k - 0.25) / 2.0 } else { 0.25 + 3.0 * (k - 0.75) };
    (k.max(0.05), d2)
}

/// engines.lib's `FM_INDEX`: a modulator's phase swing at 100%, in radians.
const FM_INDEX: f32 = 8.0;

fn transient(p: &mut Painter, params: &SynthParams, r: Rect) {
    use translib::{FM, SYNTH_TYPES};
    centre_line(p, r);
    let kind = plain(params, "transient.type") as usize;
    let decay = plain(params, "transient.decay") / 1000.0; // seconds
    if kind == FM {
        p.text(r.x + r.w / 2.0, r.y + 12.0, &name(params, "transient.type"), Text::mono(11.0, AMBER_DIM));
        // The strike across the screen, slowed to a dozen carrier cycles:
        // the carrier falls as (1-x)^2, the modulators' depths as ^3 and ^4.
        let (d1, d2) = fm_depths(plain(params, "transient.depth"));
        let ratio = 2.0 + 0.25 * plain(params, "transient.ratio");
        let f = move |x: f32| {
            let left = 1.0 - x;
            let w = 2.0 * PI * 12.0 * x;
            let m2 = (w * ratio).sin();
            let m1 = (w * ratio + FM_INDEX * d2 * left.powi(4) * m2).sin();
            (w + FM_INDEX * d1 * left.powi(3) * m1).sin() * left * left * 0.9
        };
        p.trace(&plot(r, 600, f), 0.9);
        return;
    }
    if kind < SYNTH_TYPES {
        p.text(r.x + r.w / 2.0, r.y + 12.0, &name(params, "transient.type"), Text::mono(11.0, AMBER_DIM));
        // The synthesized types: a burst under the decay envelope, across
        // the screen.
        let f = move |t: f32| (-5.0 * t).exp() * (hash((t * 300.0) as u32) * 0.5 + 0.5 * (t * 120.0 * PI).sin()) * 0.9;
        p.trace(&plot(r, 300, f), 0.9);
        return;
    }
    // A recorded sample: its peak envelope with the fade, stretched to
    // fill the screen up to where it falls silent (or the decay ends it).
    let k = kind - SYNTH_TYPES;
    let Some(&len) = translib::LENS.get(k) else { return };
    let rate = 2f32.powf(plain(params, "transient.pitch") / 12.0);
    let peak = (0..len).map(|i| translib::read(k as i32, i as f32).abs()).fold(1e-6, f32::max);
    let last = (0..len).rev().find(|&i| translib::read(k as i32, i as f32).abs() > 0.01 * peak).unwrap_or(len);
    let span = (last as f32 / translib::RATE / rate).min(decay).max(0.002);
    let columns = (r.w as usize).max(1);
    let mut path = Path::new();
    for c in 0..columns {
        let t0 = c as f32 / columns as f32 * span;
        let t1 = (c + 1) as f32 / columns as f32 * span;
        let fade = ((decay - t0) / (0.4 * decay)).clamp(0.0, 1.0);
        let (i0, i1) = ((t0 * translib::RATE * rate) as u32, (t1 * translib::RATE * rate) as u32);
        let peak = (i0..i1.min(len)).map(|i| translib::read(k as i32, i as f32).abs()).fold(0.0, f32::max) * fade;
        let x = r.x + c as f32 + 0.5;
        path.move_to(x, r.y + r.h / 2.0 - peak * r.h / 2.0);
        path.line_to(x, r.y + r.h / 2.0 + peak * r.h / 2.0);
    }
    p.trace(&path, 0.7);
}

// ---- Tone ------------------------------------------------------------------------

impl FilterShape {
    /// The MS-20 high-pass's small-signal gain (core.lib's ms20HPF):
    /// |s (s + 1) / (s^2 + (2 - K) s + 1)| at s = j*hz/cutoff. Past the
    /// onset (K >= 2) it's drawn just short of it: the diodes hold the
    /// oscillation there.
    fn hpf_mag(&self, hz: f32) -> f32 {
        let w = hz / self.hpf;
        let damping = (2.0 - hpf_k(self.hpf_peak)).max(0.02);
        // numerator j w (1 + j w) = -w^2 + j w; denominator (1 - w^2) + j damping w
        let num = (w * w * w * w + w * w).sqrt();
        let (dr, di) = (1.0 - w * w, damping * w);
        num / (dr * dr + di * di).sqrt().max(1e-9)
    }

    /// The ladder's or the SH-101's small-signal response (core.lib's
    /// ladderLP, sh101LP): four one-poles (spread over `a` octaves) with
    /// feedback k, the ladder's input raised by 0.35 k, the 101's output by
    /// 0.5 k; with the high-pass. (Its B, the noise on the cutoff, isn't drawn.)
    fn ladder_db(&self, hz: f32) -> f32 {
        let res = ((self.q - 0.5) / 5.5).clamp(0.0, 1.0);
        let (k, gain) = if self.kind == 1 { (4.2 * res, 1.0 + 0.35 * 4.2 * res) } else { (4.4 * res, 1.0 + 0.5 * 4.4 * res) };
        // Four one-poles at s = j*w: G = 1 / ((1 + j*w0)(1 + j*w1)...).
        let (mut gr, mut gi) = (1.0f32, 0.0f32);
        for i in 0..4 {
            let w = hz / (self.cutoff * (self.a * (i as f32 / 3.0 - 0.5)).exp2());
            // times (1 - j*w) / (1 + w^2)
            let d = 1.0 + w * w;
            (gr, gi) = ((gr + gi * w) / d, (gi - gr * w) / d);
        }
        // H = gain * G / (1 + k G)
        let (dr, di) = (1.0 + k * gr, k * gi);
        let den = dr * dr + di * di;
        let (hr, hi) = ((gr * dr + gi * di) / den, (gi * dr - gr * di) / den);
        let mag = gain * (hr * hr + hi * hi).sqrt() * self.hpf_mag(hz);
        20.0 * mag.max(1e-6).log10()
    }
}

/// The filter settings the response curve needs.
struct FilterShape {
    /// 0 SVF, 1 ladder, 2 101.
    kind: u8,
    cutoff: f32,
    q: f32,
    /// The type's A and B (the SVF's morph and bandpass; the ladders' spread
    /// and noise).
    a: f32,
    b: f32,
    four_pole: bool,
    hpf: f32,
    hpf_peak: f32,
}

/// The HPF's feedback K for its Peak knob (core.lib's ms20K).
fn hpf_k(peak: f32) -> f32 {
    const ONSET: f32 = 0.85;
    let r = peak.clamp(0.0, 1.0);
    if r <= ONSET {
        2.0 - 2.0 * 0.01f32.powf((r / ONSET).powf(1.5))
    } else {
        1.98 + (r - ONSET) / (1.0 - ONSET) * 1.02
    }
}

impl FilterShape {
    /// As the knobs are set.
    fn read(params: &SynthParams) -> Self {
        Self {
            kind: plain(params, "filter.type") as u8,
            cutoff: plain(params, "filter.cutoff"),
            q: plain(params, "filter.resonance").max(0.5),
            a: plain(params, "filter.a").clamp(0.0, 1.0),
            b: plain(params, "filter.b"),
            four_pole: plain(params, "filter.4_pole") > 0.5,
            hpf: plain(params, "filter.hpf"),
            hpf_peak: plain(params, "filter.hpf_peak"),
        }
    }

    /// As the playing voice has it right now, if one plays.
    fn live(params: &SynthParams, live: &Live) -> Option<Self> {
        Some(Self {
            kind: plain(params, "filter.type") as u8,
            cutoff: live.get(Moving::Cutoff)?,
            q: live.get(Moving::Resonance)?.max(0.5),
            a: live.get(Moving::A)?.clamp(0.0, 1.0),
            b: live.get(Moving::B)?,
            four_pole: plain(params, "filter.4_pole") > 0.5,
            hpf: live.get(Moving::Hpf)?,
            hpf_peak: live.get(Moving::HpfPeak)?,
        })
    }

    /// Magnitude (dB) at `hz`, following core.lib's morphing SVF and the
    /// MS-20 high-pass (the saturation is left out).
    fn db(&self, hz: f32) -> f32 {
        if self.kind > 0 {
            return self.ladder_db(hz);
        }
        // Complex response of each SVF output at s = j*w/wc.
        let w = hz / self.cutoff;
        let k = 1.0 / self.q;
        let (dr, di) = (1.0 - w * w, k * w); // denominator
        let den = dr * dr + di * di;
        let div = |nr: f32, ni: f32| ((nr * dr + ni * di) / den, (ni * dr - nr * di) / den);
        let lp = div(1.0, 0.0);
        let bp = div(0.0, w);
        let hp = div(-w * w, 0.0);
        let notch = (lp.0 + hp.0, lp.1 + hp.1);
        let mix = |a: (f32, f32), b: (f32, f32), x: f32| (a.0 * (1.0 - x) + b.0 * x, a.1 * (1.0 - x) + b.1 * x);
        let m = self.a;
        let shaped = if m <= 0.5 { mix(lp, notch, 2.0 * m) } else { mix(notch, hp, 2.0 * m - 1.0) };
        let out = mix(shaped, bp, self.b);
        let mut mag = (out.0 * out.0 + out.1 * out.1).sqrt();
        if self.four_pole {
            mag *= mag;
        }
        mag *= self.hpf_mag(hz);
        20.0 * mag.max(1e-6).log10()
    }
}

fn filter(p: &mut Painter, params: &SynthParams, live: &Live, r: Rect) {
    const TOP_DB: f32 = 18.0;
    const BOTTOM_DB: f32 = -42.0;
    let to_y = |db: f32| r.y + (TOP_DB - db.clamp(BOTTOM_DB, TOP_DB)) / (TOP_DB - BOTTOM_DB) * r.h;
    for hz in [100.0f32, 1000.0, 10000.0] {
        let x = r.x + (hz / 20.0).log10() / 3.0 * r.w;
        p.screen_line(x, r.y, x, r.bottom());
    }
    p.screen_line(r.x, to_y(0.0), r.right(), to_y(0.0));
    let curve = |shape: FilterShape| {
        let mut path = Path::new();
        let n = 160;
        for i in 0..=n {
            let t = i as f32 / n as f32;
            let hz = 20.0 * 1000f32.powf(t);
            let (x, y) = (r.x + t * r.w, to_y(shape.db(hz)));
            if i == 0 { path.move_to(x, y) } else { path.line_to(x, y) }
        }
        path
    };
    // While a note plays: the voice's response bright, the knobs' faint.
    match FilterShape::live(params, live) {
        Some(moving) => {
            p.trace(&curve(FilterShape::read(params)), 0.25);
            p.trace(&curve(moving), 1.0);
        }
        None => p.trace(&curve(FilterShape::read(params)), 1.0),
    }
}

/// An Orthogonal player's sample: set `set`'s picture (its peaks, mirrored
/// about the middle), its name small in the corner, and a bar where the
/// player is (`position`, 0..1 of the set; brighter while a note plays it).
pub fn sample(p: &mut Painter, r: Rect, set: usize, position: f32, live: bool, hot: bool) {
    p.screen(r);
    let Some(info) = ortholib::SETS.get(set) else { return };
    let area = r.inset(6.0, 18.0, 6.0, 6.0);
    let mid = area.y + area.h / 2.0;
    let n = info.overview.len();
    let mut shape = Path::new();
    for (k, &v) in info.overview.iter().enumerate() {
        let x = area.x + area.w * k as f32 / (n - 1) as f32;
        let y = mid - v as f32 / 255.0 * area.h / 2.0;
        if k == 0 { shape.move_to(x, y) } else { shape.line_to(x, y) }
    }
    for (k, &v) in info.overview.iter().enumerate().rev() {
        let x = area.x + area.w * k as f32 / (n - 1) as f32;
        shape.line_to(x, mid + v as f32 / 255.0 * area.h / 2.0);
    }
    shape.close();
    p.fill_shape(&shape, AMBER_DIM, if hot { 0.75 } else { 0.55 });
    centre_line(p, area);
    let x = area.x + area.w * position.clamp(0.0, 1.0);
    p.bar(Rect::new(x - 1.5, r.y + 4.0, 3.0, r.h - 8.0), if live { 1.0 } else { 0.7 });
    p.text(r.x + 9.0, r.y + 11.0, info.name, Text::mono(10.0, if hot { 0xffcf73 } else { 0xe9a54a }).left());
}

/// What the Harmonic EQ does to a slot's harmonics: each of the first 64 as a
/// line up from the bottom to its gain (bands and Odd/Even together), and the
/// bands' curve through them; 0 dB marked, ±30 dB tall, harmonics on a log axis.
fn harmonic(p: &mut Painter, params: &SynthParams, r: Rect) {
    use crate::harmonic::{response, HarmonicSettings, GAIN_DB};
    let s = HarmonicSettings::from_values(crate::params::HARMONIC_FIELDS.map(|id| plain(params, id)));
    let gains = s.gains();
    let x = |h: f32| r.x + r.w * (h / 0.8).ln() / (80.0f32 / 0.8).ln();
    let y = |db: f32| r.y + r.h / 2.0 - db.clamp(-GAIN_DB, GAIN_DB) / GAIN_DB * r.h / 2.0;
    p.screen_line(r.x, y(0.0), r.right(), y(0.0));
    let bands_on = s.on();
    for h in 1..=64u32 {
        let db = if bands_on { response(&gains, h as f32) + 20.0 * s.comb_gain(h).max(1e-3).log10() } else { 0.0 };
        p.screen_bar(x(h as f32), y(db), r.bottom());
    }
    let mut curve = Path::new();
    for i in 0..=160 {
        let h = 0.8 * (80.0f32 / 0.8).powf(i as f32 / 160.0);
        let (px, py) = (x(h), y(if bands_on { response(&gains, h) } else { 0.0 }));
        if i == 0 { curve.move_to(px, py) } else { curve.line_to(px, py) }
    }
    p.trace(&curve, 1.0);
}
