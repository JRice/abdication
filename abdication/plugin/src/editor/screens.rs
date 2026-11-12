//! What the amber screens show. Most pictures are computed from the
//! current parameter values (and, for waves and samples, the real tables),
//! so they follow knob moves and automation. While a note plays, the
//! screens whose values move follow the newest voice instead: the filter's
//! response, the selected knob's modulators. The oscillator slots' screens
//! are oscilloscopes of the real oscillators (see `scopes`).

use std::f32::consts::PI;

use nih_plug_vizia::vizia::vg::Path;

use super::draw::{Painter, Text, AMBER_DIM, ENV_COLOR, LFO_COLOR};
use super::layout::{Rect, Screen, SLOT_DUAL, SLOT_WAVETABLE};
use super::scopes::Scopes;
use crate::modulation;
use crate::params::SynthParams;
use crate::scope::{Moving, Tap};
use crate::{translib, wavelib};

/// What the screens draw from this frame.
pub struct Live {
    /// The two oscillator slots' traces.
    pub traces: [Vec<f32>; 2],
    /// Whether a note is playing (the traces and `moving` are its).
    pub playing: bool,
    scopes_moving: [Option<f32>; 5],
    /// Where the newest voice's modulators have each knob (travel), while a note plays.
    moved: Option<[f32; modulation::COUNT]>,
    /// The pitch the screens draw for (the last note played).
    pub note_hz: f32,
}

impl Live {
    pub fn from(scopes: &mut Scopes, params: &SynthParams) -> Self {
        use Moving::*;
        let all = [Cutoff, Resonance, Morph, Bandpass, Hpf];
        Self {
            traces: [scopes.trace(params, Tap::Osc1), scopes.trace(params, Tap::Osc2)],
            playing: scopes.is_live(),
            scopes_moving: all.map(|m| scopes.moving(m)),
            moved: scopes.is_live().then(|| std::array::from_fn(|t| scopes.moved(t).unwrap_or(0.0))),
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

pub fn draw(p: &mut Painter, params: &SynthParams, live: &Live, selected: usize, screen: Screen, r: Rect) {
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
                _ => format!("{} · {when}", name(params, &format!("ray_{k}.element"))),
            };
            scope_trace(p, &live.traces[slot], &label, area);
        }
        Screen::DriveCurve => drive_curve(p, params, r.inset(5.0, 5.0, 5.0, 5.0)),
        Screen::Wave(slot, i) => {
            let k = slot + 1;
            centre_line(p, area);
            // Each wave as the OscMod reshapes it (at the knob's depth).
            let cycle = osc_mod_cycle(plain(params, &format!("wt_{k}.wave_{}", i + 1)) as usize,
                                      plain(params, &format!("wt_{k}.oscmod")) as i32,
                                      plain(params, &format!("wt_{k}.oscmod_depth")), live.note_hz, 240);
            let n = cycle.len() - 1;
            p.trace(&plot(area, n, |t| cycle[(t * n as f32).round() as usize] * 0.85), 1.0);
        }
        Screen::Noise => noise(p, params, area),
        Screen::Transient => transient(p, params, area),
        Screen::Filter => filter(p, params, live, area),
        Screen::Amp => amp(p, params, area),
        Screen::ModLfo => mod_lfo(p, params, live, selected, area),
        Screen::ModEnv => mod_env(p, params, selected, area),
        // The panel writes the list itself.
        Screen::ModList => {}
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
fn drive_curve(p: &mut Painter, params: &SynthParams, r: Rect) {
    let x = plain(params, "drive.amount").clamp(0.0, 1.0);
    let gain = 10f32.powf(24.0 * (0.375 * x + 0.625 * x.powi(4)) / 20.0);
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

fn transient(p: &mut Painter, params: &SynthParams, r: Rect) {
    const SYNTH_TYPES: usize = 5;
    centre_line(p, r);
    let kind = plain(params, "transient.type") as usize;
    let decay = plain(params, "transient.decay") / 1000.0; // seconds
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
    /// The ladder's or the SH-101's small-signal response (core.lib's
    /// ladderLP, sh101LP): four one-poles with feedback k, the ladder's input
    /// raised by 0.35 k, the 101's output by 0.5 k; with the utility high-pass.
    fn ladder_db(&self, hz: f32) -> f32 {
        let res = ((self.q - 0.5) / 5.5).clamp(0.0, 1.0);
        let (k, gain) = if self.kind == 1 { (4.2 * res, 1.0 + 0.35 * 4.2 * res) } else { (4.4 * res, 1.0 + 0.5 * 4.4 * res) };
        // Four one-poles at s = j*w: G = 1 / (1 + j*w)^4.
        let w = hz / self.cutoff;
        let (mut gr, mut gi) = (1.0f32, 0.0f32);
        for _ in 0..4 {
            // times (1 - j*w) / (1 + w^2)
            let d = 1.0 + w * w;
            (gr, gi) = ((gr + gi * w) / d, (gi - gr * w) / d);
        }
        // H = gain * G / (1 + k G)
        let (dr, di) = (1.0 + k * gr, k * gi);
        let den = dr * dr + di * di;
        let (hr, hi) = ((gr * dr + gi * di) / den, (gi * dr - gr * di) / den);
        let mut mag = gain * (hr * hr + hi * hi).sqrt();
        let wh = hz / self.hpf;
        mag *= wh / (1.0 + wh * wh).sqrt();
        20.0 * mag.max(1e-6).log10()
    }
}

/// The filter settings the response curve needs.
struct FilterShape {
    /// 0 SVF, 1 ladder, 2 SH-101.
    kind: u8,
    cutoff: f32,
    q: f32,
    morph: f32,
    bandpass: f32,
    four_pole: bool,
    hpf: f32,
}

impl FilterShape {
    /// As the knobs are set.
    fn read(params: &SynthParams) -> Self {
        Self {
            kind: plain(params, "filter.type") as u8,
            cutoff: plain(params, "filter.cutoff"),
            q: plain(params, "filter.resonance").max(0.5),
            morph: plain(params, "filter.morph_lp_notch_hp").clamp(0.0, 1.0),
            bandpass: plain(params, "filter.bandpass"),
            four_pole: plain(params, "filter.4_pole") > 0.5,
            hpf: plain(params, "filter.utility_hpf"),
        }
    }

    /// As the playing voice has it right now, if one plays.
    fn live(params: &SynthParams, live: &Live) -> Option<Self> {
        Some(Self {
            kind: plain(params, "filter.type") as u8,
            cutoff: live.get(Moving::Cutoff)?,
            q: live.get(Moving::Resonance)?.max(0.5),
            morph: live.get(Moving::Morph)?.clamp(0.0, 1.0),
            bandpass: live.get(Moving::Bandpass)?,
            four_pole: plain(params, "filter.4_pole") > 0.5,
            hpf: live.get(Moving::Hpf)?,
        })
    }

    /// Magnitude (dB) at `hz`, following core.lib's morphing SVF and the
    /// utility high-pass (the saturation is left out).
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
        let m = self.morph;
        let shaped = if m <= 0.5 { mix(lp, notch, 2.0 * m) } else { mix(notch, hp, 2.0 * m - 1.0) };
        let out = mix(shaped, bp, self.bandpass);
        let mut mag = (out.0 * out.0 + out.1 * out.1).sqrt();
        if self.four_pole {
            mag *= mag;
        }
        let wh = hz / self.hpf;
        mag *= wh / (1.0 + wh * wh).sqrt();
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

// ---- Shaping ---------------------------------------------------------------------

/// An ADSR's shape across `r`: segments as wide as the square root of
/// their time (so short and long settings both stay readable), sustain a
/// fixed stretch, each timed segment bent by its bow (modulation::along).
/// `scale` maps a level (0..1) to a y. Returns the path and the x where the
/// release starts.
fn adsr_path(r: Rect, (a, d, s, rel): (f32, f32, f32, f32), (ba, bd, br): (f32, f32, f32),
             scale: impl Fn(f32) -> f32) -> (Path, f32) {
    use modulation::along;
    let width = |t: f32| t.max(0.001).sqrt();
    let (wa, wd, wr) = (width(a), width(d), width(rel));
    let hold = (wa + wd + wr) * 0.35;
    let total = wa + wd + hold + wr;
    let shape = |u: f32| -> f32 {
        let held = along(1.0, s, (u.min(wa + wd + hold) - wa) / wd, bd);
        if u < wa {
            along(0.0, 1.0, u / wa, ba)
        } else if u < wa + wd + hold {
            held
        } else {
            along(held, 0.0, (u - wa - wd - hold) / wr, br)
        }
    };
    let mut path = Path::new();
    for i in 0..=300 {
        let u = i as f32 / 300.0 * total;
        let (x, y) = (r.x + u / total * r.w, scale(shape(u)));
        if i == 0 { path.move_to(x, y) } else { path.line_to(x, y) }
    }
    (path, r.x + (wa + wd + hold) / total * r.w)
}

fn amp(p: &mut Painter, params: &SynthParams, r: Rect) {
    let times = (plain(params, "amp.attack"), plain(params, "amp.decay"), plain(params, "amp.sustain"),
                 plain(params, "amp.release"));
    let bows = (plain(params, "amp.attack_curve"), plain(params, "amp.decay_curve"), plain(params, "amp.release_curve"));
    let (path, release) = adsr_path(r, times, bows, |v| r.bottom() - v * r.h);
    p.screen_line(release, r.y, release, r.bottom());
    p.trace(&path, 1.0);
}

/// The selected knob's LFO: two cycles of its shape, as deep as its amount
/// (faint at 0), and its rate; while a note plays, a line at how far the
/// newest voice has the knob from its own value.
fn mod_lfo(p: &mut Painter, params: &SynthParams, live: &Live, t: usize, r: Rect) {
    centre_line(p, r);
    let s = params.mods[t].settings();
    let wave = modulation::sketch(s.shape, 120);
    let depth = s.lfo.abs().max(0.12) * if s.lfo < 0.0 { -1.0 } else { 1.0 };
    let alpha = if s.lfo.abs() > 1e-4 { 1.0 } else { 0.3 };
    p.trace(&plot(r, 240, |x| wave[((2.0 * x).fract() * 120.0) as usize % 120] * 0.9 * depth), alpha);
    let rate = match s.sync {
        Some(beats) => format!("{} sync", modulation::DIVISIONS.iter().find(|d| d.0 == beats).map_or("", |d| d.1)),
        None => format!("{:.2} Hz", s.hz),
    };
    let shape = modulation::SHAPE_NAMES[modulation::SHAPES.iter().position(|x| *x == s.shape).unwrap_or(0)];
    let free = if s.retrig { "" } else { " · free" };
    p.text(r.x, r.y + 2.0, &format!("{shape} · {rate}{free}"), Text::mono(9.0, LFO_COLOR).left());
    p.text(r.right(), r.y + 2.0, &format!("{:+.0} %", s.lfo * 100.0), Text::mono(9.0, LFO_COLOR).right());
    if let Some(moved) = live.moved.map(|m| m[t]).filter(|_| s.active()) {
        let base = params.ptr(modulation::TARGETS[t].id).map(|q| unsafe { q.unmodulated_normalized_value() }).unwrap_or(0.0);
        level_line(p, r, ((moved - base) * 1.8).clamp(-1.0, 1.0));
    }
}

/// The selected knob's envelope, as tall as its amount (hanging down for a
/// negative one; faint at 0), its segments bent by their curves.
fn mod_env(p: &mut Painter, params: &SynthParams, t: usize, r: Rect) {
    let s = params.mods[t].settings();
    let height = s.env.abs().max(0.12) * r.h;
    let scale = |v: f32| if s.env < 0.0 { r.y + v * height } else { r.bottom() - v * height };
    let bows = (s.attack_curve, s.decay_curve, s.release_curve);
    let (path, release) = adsr_path(r, (s.attack, s.decay, s.sustain, s.release), bows, scale);
    p.screen_line(release, r.y, release, r.bottom());
    p.trace(&path, if s.env.abs() > 1e-4 { 1.0 } else { 0.3 });
    p.text(r.right(), r.y + 2.0, &format!("{:+.0} %", s.env * 100.0), Text::mono(9.0, ENV_COLOR).right());
}

/// A bright horizontal line at `v` (-1..1, up is +1).
fn level_line(p: &mut Painter, r: Rect, v: f32) {
    let y = r.y + r.h / 2.0 - v.clamp(-1.0, 1.0) * r.h / 2.0;
    let mut path = Path::new();
    path.move_to(r.x - 4.0, y);
    path.line_to(r.right() + 4.0, y);
    p.trace(&path, 0.55);
}
