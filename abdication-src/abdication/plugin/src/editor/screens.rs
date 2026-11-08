//! What the amber screens show. Most pictures are computed from the
//! current parameter values (and, for waves and samples, the real tables),
//! so they follow knob moves and automation. While a note plays, the
//! screens whose values move follow the newest voice instead: the pulse
//! width, the filter's response, the free modulators. The Dual and
//! Wavetable screens are oscilloscopes of the real oscillators (see
//! `scopes`).

use std::f32::consts::PI;

use nih_plug_vizia::vizia::vg::Path;

use super::draw::{Painter, Text, AMBER_DIM};
use super::layout::{Rect, Screen};
use super::scopes::Scopes;
use crate::params::SynthParams;
use crate::scope::{Moving, Tap};
use crate::{translib, wavelib};

/// What the screens draw from this frame.
pub struct Live {
    pub dual: Vec<f32>,
    pub wavetable: Vec<f32>,
    /// Whether a note is playing (the traces and `moving` are its).
    pub playing: bool,
    scopes_moving: [Option<f32>; 8],
    /// The pitch the screens draw for (the last note played).
    pub note_hz: f32,
}

impl Live {
    pub fn from(scopes: &mut Scopes, params: &SynthParams) -> Self {
        use Moving::*;
        let all = [PulseWidth, Cutoff, Resonance, Morph, Bandpass, Hpf, ModEnv, ModLfo];
        Self {
            dual: scopes.trace(params, Tap::Dual),
            wavetable: scopes.trace(params, Tap::Wavetable),
            playing: scopes.is_live(),
            scopes_moving: all.map(|m| scopes.moving(m)),
            note_hz: scopes.note_hz(),
        }
    }

    /// A moving value of the playing voice, if one plays.
    fn get(&self, value: Moving) -> Option<f32> {
        self.scopes_moving[value as usize]
    }
}

/// The note value a synced LFO's Rate knob (travel 0..1) picks, slowest
/// first (as voice.lib's noteValues).
pub fn note_value(travel: f32) -> &'static str {
    const NAMES: [&str; 15] = ["4 bars", "2 bars", "1 bar", "1/2.", "1/2", "1/4.", "1/2T", "1/4", "1/8.", "1/4T",
                               "1/8", "1/8T", "1/16", "1/16T", "1/32"];
    NAMES[((travel.clamp(0.0, 1.0) * 14.0 + 0.5) as usize).min(14)]
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
        Screen::Fundamental => fundamental(p, params, live, area),
        Screen::DualScope => {
            let label = format!("{} · {} · {when}", name(params, "dual.algorithm"), name(params, "dual.waves"));
            scope_trace(p, &live.dual, &label, area);
        }
        Screen::DriveCurve => drive_curve(p, params, r.inset(5.0, 5.0, 5.0, 5.0)),
        Screen::WavetableScope => {
            let label = match name(params, "wavetable.oscmod") {
                m if m == "Off" => when.to_string(),
                m => format!("{m} · {when}"),
            };
            scope_trace(p, &live.wavetable, &label, area);
        }
        Screen::Slot(i) => {
            let id = ["wavetable.start", "wavetable.mid", "wavetable.end"][i];
            centre_line(p, area);
            // Each wave as the OscMod reshapes it (at the knob's depth).
            let cycle = osc_mod_cycle(plain(params, id) as usize, plain(params, "wavetable.oscmod") as i32,
                                      plain(params, "wavetable.oscmod_depth"), live.note_hz, 240);
            let n = cycle.len() - 1;
            p.trace(&plot(area, n, |t| cycle[(t * n as f32).round() as usize] * 0.85), 1.0);
        }
        Screen::Noise => noise(p, params, area),
        Screen::Transient => transient(p, params, area),
        Screen::Filter => filter(p, params, live, area),
        Screen::Envelope => envelope(p, params, area),
        Screen::ModEnv => mod_env(p, params, live, area),
        Screen::ModLfo => mod_lfo(p, params, live, area),
    }
}

fn centre_line(p: &mut Painter, r: Rect) {
    p.screen_line(r.x - 4.0, r.y + r.h / 2.0, r.right() + 4.0, r.y + r.h / 2.0);
}

// ---- Oscillators -------------------------------------------------------------

fn fundamental(p: &mut Painter, params: &SynthParams, live: &Live, r: Rect) {
    centre_line(p, r);
    let wave = plain(params, "fundamental.wave") as i32;
    let pw = live.get(Moving::PulseWidth).unwrap_or_else(|| plain(params, "fundamental.pulse_width"));
    let f = move |t: f32| {
        let x = (t * 2.0).fract(); // two cycles
        0.8 * match wave {
            0 => (2.0 * PI * x).sin(),
            1 => 1.0 - 4.0 * (x - 0.5).abs(),
            2 => 2.0 * x - 1.0,
            _ => if x < pw { 1.0 } else { -1.0 },
        }
    };
    p.trace(&plot(r, 400, f), 1.0);
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
    let rate = 2f32.powf(plain(params, "transient.sample_pitch") / 12.0);
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

/// The filter settings the response curve needs.
struct FilterShape {
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

/// One envelope segment: `k` is core.lib's curve constant.
fn rise(k: f32, x: f32) -> f32 {
    let k = if k.abs() < 1e-3 { 1e-3 } else { k };
    ((k * x).exp() - 1.0) / (k.exp() - 1.0)
}
fn fall(k: f32, x: f32) -> f32 {
    1.0 - rise(-k, x)
}

fn envelope(p: &mut Painter, params: &SynthParams, r: Rect) {
    let a = plain(params, "envelope.attack");
    let d = plain(params, "envelope.decay");
    let s = plain(params, "envelope.sustain");
    let rel = plain(params, "envelope.release");
    let k = 6.0 * plain(params, "envelope.curve").clamp(-1.0, 1.0);
    let sus_offset = plain(params, "envelope.amp_sustain");
    let dec_offset = plain(params, "envelope.amp_decay");
    let (amp_s, amp_d) = (s + sus_offset * (1.0 - s), d * (10.0 / d.max(1e-4)).powf(dec_offset));

    // Segment widths grow with the square root of their time, so short and
    // long settings both stay readable; sustain gets a fixed stretch.
    let width = |t: f32| t.max(0.001).sqrt();
    let (wa, wd, wr) = (width(a), width(d.max(amp_d)), width(rel));
    let hold = (wa + wd + wr) * 0.35;
    let total = wa + wd + hold + wr;
    let px = |u: f32| r.x + u / total * r.w;
    let py = |v: f32| r.bottom() - v * r.h;
    p.screen_line(px(wa + wd + hold), r.y, px(wa + wd + hold), r.bottom());

    let shape = |decay: f32, sus: f32| {
        let wdd = width(decay);
        move |u: f32| -> f32 {
            if u < wa {
                rise(k, u / wa)
            } else if u < wa + wd + hold {
                let t = ((u - wa) / wdd).min(1.0);
                sus + (1.0 - sus) * fall(k, t)
            } else {
                let t = (wa + wd + hold).min(u);
                let at_release = sus + (1.0 - sus) * fall(k, ((t - wa) / wdd).min(1.0));
                at_release * fall(k, ((u - wa - wd - hold) / wr).min(1.0))
            }
        }
    };
    let line = |f: &dyn Fn(f32) -> f32| {
        let mut path = Path::new();
        for i in 0..=300 {
            let u = i as f32 / 300.0 * total;
            if i == 0 { path.move_to(px(u), py(f(u))) } else { path.line_to(px(u), py(f(u))) }
        }
        path
    };
    p.trace(&line(&shape(amp_d, amp_s)), 0.35);
    p.trace(&line(&shape(d, s)), 1.0);
}

/// The free envelope's shape (as the Envelope screen, one line); while a
/// note plays, a line at its level now.
fn mod_env(p: &mut Painter, params: &SynthParams, live: &Live, r: Rect) {
    let v = |field: &str| plain(params, &format!("mod_env.{field}"));
    let (a, d, s, rel) = (v("attack"), v("decay"), v("sustain"), v("release"));
    let k = 6.0 * v("curve").clamp(-1.0, 1.0);
    let width = |t: f32| t.max(0.001).sqrt();
    let (wa, wd, wr) = (width(a), width(d), width(rel));
    let hold = (wa + wd + wr) * 0.35;
    let total = wa + wd + hold + wr;
    let shape = |u: f32| -> f32 {
        let held = s + (1.0 - s) * fall(k, ((u.min(wa + wd + hold) - wa) / wd).clamp(0.0, 1.0));
        if u < wa {
            rise(k, u / wa)
        } else if u < wa + wd + hold {
            held
        } else {
            held * fall(k, ((u - wa - wd - hold) / wr).min(1.0))
        }
    };
    p.screen_line(r.x + (wa + wd + hold) / total * r.w, r.y, r.x + (wa + wd + hold) / total * r.w, r.bottom());
    p.trace(&plot(r, 300, |t| 2.0 * shape(t * total) - 1.0), 1.0);
    if let Some(level) = live.get(Moving::ModEnv) {
        level_line(p, r, 2.0 * level - 1.0);
    }
}

/// The free LFO's wave, two cycles, and its rate; while a note plays, a
/// line at its value now.
fn mod_lfo(p: &mut Painter, params: &SynthParams, live: &Live, r: Rect) {
    centre_line(p, r);
    let w = plain(params, "mod_lfo.shape") as usize;
    p.trace(&plot(r, 300, |t| wave_at(w, (2.0 * t).fract()) * 0.9), 1.0);
    let label = if plain(params, "mod_lfo.sync") > 0.5 {
        let travel = params.ptr("mod_lfo.rate").map(|p| unsafe { p.unmodulated_normalized_value() }).unwrap_or(0.0);
        format!("{} · {} · sync", name(params, "mod_lfo.shape"), note_value(travel))
    } else {
        let rate = params.ptr("mod_lfo.rate")
            .map(|p| unsafe { p.normalized_value_to_string(p.unmodulated_normalized_value(), true) })
            .unwrap_or_default();
        format!("{} · {rate}", name(params, "mod_lfo.shape"))
    };
    p.text(r.x, r.y + 2.0, &label, Text::mono(9.0, AMBER_DIM).left());
    if let Some(value) = live.get(Moving::ModLfo) {
        level_line(p, r, value * 0.9);
    }
}

/// A bright horizontal line at `v` (-1..1, up is +1).
fn level_line(p: &mut Painter, r: Rect, v: f32) {
    let y = r.y + r.h / 2.0 - v.clamp(-1.0, 1.0) * r.h / 2.0;
    let mut path = Path::new();
    path.move_to(r.x - 4.0, y);
    path.line_to(r.right() + 4.0, y);
    p.trace(&path, 0.55);
}
