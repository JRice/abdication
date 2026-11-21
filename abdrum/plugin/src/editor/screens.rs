//! The amber screen: whatever was last touched, drawn. Decays show their
//! envelope, the pitch controls the pitch drop, the noise filter its
//! response, and lists (samples, loops, modes) a few entries around the
//! current one. Anything else shows its value large.

use super::draw::{Painter, Text, PHOSPHOR, PHOSPHOR_DIM};
use super::layout::{Control, Kind, Rect};
use super::values::{self, plain, snap_ratio};
use crate::params::DrumParams;

const POINTS: usize = 160;

enum Picture {
    /// A pitch: its nearest note and how far off it is, as a little meter.
    Tuning { hz: f32, caption: String },
    Curve { ys: Vec<f32>, caption: String },
    List { items: Vec<String>, current: usize },
    Value { text: String, fill: f32, caption: &'static str },
}

/// Two-stage envelope (as drum.lib's twoStage), in ms.
fn envelope(attack: f32, fast: f32, slow: f32, tail: f32) -> Vec<f32> {
    let window = (slow * 1.15).max(fast * 3.0) + attack;
    let decay = |t: f32, t60: f32| (-6.9078 * t / t60.max(0.1)).exp();
    (0..POINTS)
        .map(|i| {
            let t = i as f32 / (POINTS - 1) as f32 * window;
            let rise = if attack > 0.0 { (t / attack).min(1.0) } else { 1.0 };
            let t = (t - attack).max(0.0);
            rise * ((1.0 - tail) * decay(t, fast) + tail * decay(t, slow))
        })
        .collect()
}

fn envelope_of(params: &DrumParams, group: &str) -> Picture {
    let p = |name: &str| plain(params, &format!("{group}.{name}"));
    let slow = p("decay");
    let (attack, fast, tail) = if group == "body" { (0.0, slow, 0.0) } else { (p("attack"), slow * snap_ratio(p("snap")), p("tail_level")) };
    let caption = if group == "body" {
        format!("rings for {} (to −60 dB)", values::ms(slow))
    } else {
        let attack = if attack > 0.0 { format!("atk {} · ", values::ms(attack)) } else { String::new() };
        format!("{attack}fast {} · tail {:.0} % · slow {}", values::ms(fast), tail * 100.0, values::ms(slow))
    };
    Picture::Curve { ys: envelope(attack, fast, slow, tail), caption }
}

/// The free envelope's shape (as mod.lib), 0..1, over its own length.
pub fn mod_envelope(params: &DrumParams) -> Vec<f32> {
    let p = |name: &str| plain(params, &format!("envelope.{name}"));
    let (attack, decay, curve) = (p("attack"), p("decay"), p("curve"));
    let window = attack + decay * 1.15;
    (0..POINTS)
        .map(|i| {
            let t = i as f32 / (POINTS - 1) as f32 * window;
            let rise = if attack > 0.0 { (t / attack).min(1.0) } else { 1.0 };
            let x = ((t - attack) / decay.max(0.1)).clamp(0.0, 1.0);
            rise * (1.0 - x).powf(1.0 + 6.0 * curve)
        })
        .collect()
}

fn mod_envelope_picture(params: &DrumParams, caption: String) -> Picture {
    Picture::Curve { ys: mod_envelope(params), caption }
}

/// A ring's depth in words (shown the way its knob runs).
pub fn ring_text(depth: f32) -> String {
    format!("{:+.0} %", depth * 100.0).replace('-', "−")
}

fn pitch_drop(params: &DrumParams) -> Picture {
    let p = |name: &str| plain(params, &format!("tone.{name}"));
    let (drop, split, fast, slow, hold) = (p("drop"), p("split"), p("fast_time"), p("slow_time"), p("hold"));
    let window = 150f32.max(hold + slow * 2.5);
    let ys = (0..POINTS)
        .map(|i| {
            let t = (i as f32 / (POINTS - 1) as f32 * window - hold).max(0.0);
            let st = drop * split * (-3.0 * t / fast).exp() + drop * (1.0 - split) * (-3.0 * t / slow).exp();
            st / drop.max(1.0)
        })
        .collect();
    let held = if hold > 0.0 { format!(", after {}", values::ms(hold)) } else { String::new() };
    let caption = format!("+{drop:.0} st: {:.0} % over {}, the rest over {}{held}", split * 100.0, values::ms(fast), values::ms(slow));
    Picture::Curve { ys, caption }
}

/// The noise filter's response, 40 Hz to 16 kHz (an approximation of
/// drum.lib's: bandpass skirts steepening with Q, the highpass at 12 dB/oct).
fn noise_filter(params: &DrumParams) -> Picture {
    let p = |name: &str| plain(params, &format!("noise.{name}"));
    let (bp, q, hp) = (p("bandpass"), p("q"), p("highpass"));
    let ys = (0..POINTS)
        .map(|i| {
            let hz = 40.0 * (16_000f32 / 40.0).powf(i as f32 / (POINTS - 1) as f32);
            let mut db = -24.0 * q * (hz / bp).log2().abs();
            if hz < hp {
                db -= 12.0 * (hp / hz).log2();
            }
            1.0 + db.max(-48.0) / 48.0
        })
        .collect();
    let caption = format!("bandpass {} · Q {q:.2} · highpass {}", values::text(params, "noise.bandpass"), values::text(params, "noise.highpass"));
    Picture::Curve { ys, caption }
}

/// The pad filter's feedback gain for resonance r at cutoff hz (as fx.lib's
/// filterK: the top rings for 2 s).
pub fn filter_k(hz: f32, r: f32) -> f32 {
    let top = (6.9078 / 2.0 / (2.0 * std::f32::consts::PI * hz)).min(0.5);
    4.0 * (1.0 - top.powf(r.clamp(0.0, 1.0).powf(1.5))).powi(4)
}

/// The pad filter's gain (dB) at hz: four equal poles, the inverted feedback
/// around them, and half of it added back at the output (fx.lib's sh101, at
/// low levels).
pub fn filter_db(cutoff: f32, r: f32, hz: f32) -> f32 {
    let k = filter_k(cutoff, r);
    // One pole at w = hz / cutoff: 1 / (1 + jw); four of them: p^4.
    let w = hz / cutoff;
    let (re, im) = {
        let (a, b) = (1.0 / (1.0 + w * w), -w / (1.0 + w * w));
        let (a2, b2) = (a * a - b * b, 2.0 * a * b);
        (a2 * a2 - b2 * b2, 2.0 * a2 * b2)
    };
    let (dr, di) = (1.0 + k * re, k * im);
    let mag = (1.0 + 0.5 * k) * (re * re + im * im).sqrt() / (dr * dr + di * di).sqrt();
    20.0 * mag.max(1e-9).log10()
}

/// The pad filter's response, 20 Hz to 20 kHz, −48..+24 dB.
fn pad_filter(params: &DrumParams) -> Picture {
    let (cutoff, r) = (plain(params, "filter.cutoff"), plain(params, "filter.resonance"));
    if cutoff >= values::FILTER_OFF {
        return Picture::Value { text: "Off".into(), fill: 1.0, caption: "the SH-101's 24 dB/oct lowpass; turn Cutoff down" };
    }
    let db_at = |i: usize| filter_db(cutoff, r, 20.0 * 1000f32.powf(i as f32 / (POINTS - 1) as f32));
    let ys = (0..POINTS).map(|i| ((db_at(i) + 48.0) / 72.0).clamp(0.0, 1.0)).collect();
    let peak = (0..POINTS).map(db_at).fold(f32::MIN, f32::max);
    let caption = format!("cutoff {} · res {:.0} % · peak {:+.0} dB", values::text(params, "filter.cutoff"), r * 100.0, peak);
    Picture::Curve { ys, caption }
}

/// Chop's gain from a hit (as fx.lib's chop, its 0.3 ms edges drawn
/// square), at `points` times over `window` seconds.
fn chop_gain(params: &DrumParams, window: f32, points: usize) -> Vec<f32> {
    let (hold, rate) = (plain(params, "chop.hold"), plain(params, "chop.rate"));
    let (hz, depth) = (values::chop_hz(rate), if rate > 1e-4 { values::chop_depth(rate) } else { 0.0 });
    (0..points)
        .map(|i| {
            let t = i as f32 / (points - 1) as f32 * window;
            let held = if hold >= values::CHOP_OFF { 1.0 } else { ((hold / 1000.0 + 0.001 - t) / 0.001).clamp(0.0, 1.0) };
            let open = (2.0 * std::f32::consts::PI * hz * t).cos() >= 0.0;
            held * if open { 1.0 } else { 1.0 - depth }
        })
        .collect()
}

/// How long Chop's picture runs: past the hold, or a few chops' worth.
fn chop_window(params: &DrumParams) -> f32 {
    let (hold, rate) = (plain(params, "chop.hold"), plain(params, "chop.rate"));
    if hold < values::CHOP_OFF { hold / 1000.0 * 1.25 + 0.005 } else if rate > 1e-4 { 8.0 / values::chop_hz(rate) } else { 0.25 }
}

/// The effects panel's little picture of Chop's gate.
pub fn chop_shape(params: &DrumParams) -> Vec<f32> {
    chop_gain(params, chop_window(params), 240)
}

fn chop_picture(params: &DrumParams) -> Picture {
    let window = chop_window(params);
    let caption = match (plain(params, "chop.hold") < values::CHOP_OFF, plain(params, "chop.rate") > 1e-4) {
        (false, false) => "off: each hit rings out".to_string(),
        (true, false) => format!("open {} after each hit, then shut", values::text(params, "chop.hold")),
        (false, true) => format!("chops at {}, open at each hit", values::text(params, "chop.rate")),
        (true, true) => format!("chops at {} for {}, then shut", values::text(params, "chop.rate"), values::text(params, "chop.hold")),
    };
    Picture::Curve { ys: chop_gain(params, window, 480).iter().map(|g| 0.08 + 0.84 * g).collect(), caption }
}

/// Every value of a menu parameter (or a sample set's hits), and the current one.
fn list(params: &DrumParams, id: &str) -> Option<(Vec<String>, usize)> {
    let group = id.split('.').next()?;
    if id.ends_with(".sample") || id.ends_with(".set") {
        let (set, index, _) = values::layer_choice(params, group);
        return Some((values::set_hits(set).into_iter().map(String::from).collect(), index));
    }
    let p = params.ptr(id)?;
    let steps = unsafe { p.step_count() }?;
    let items = (0..=steps).map(|k| unsafe { p.normalized_value_to_string(k as f32 / steps as f32, false) }).collect();
    let current = (unsafe { p.unmodulated_normalized_value() } * steps as f32).round() as usize;
    Some((items, current))
}

/// A line of help for the controls that do more than their name says.
fn caption(id: &str) -> &'static str {
    match id {
        "tone.shape" => "sine → triangle → saturation → fold",
        "body.strike" => "mallet hardness, and brightness with it",
        "body.material" => "membrane → plate → bar, all inharmonic",
        "body.damping" => "above the middle the high modes die sooner",
        "body.offset" => "from Tone's resting pitch",
        "noise.blend" => "generator ↔ loop",
        "noise.density" => "continuous → sparse grains",
        "noise.clap" => "2–5 bursts before the envelope",
        "noise.bite" => "gritty at the hit, clean as it decays",
        "noise.spread" => "Metal: the 808's ratios at 0; harmonic ‹ › stretched",
        "tone.osc_2" => "how Osc 2 meets Tone: FM · Tangle · Shame · VOSIM",
        "tone.ratio" => "Osc 2 at this multiple of Tone's pitch (drop too)",
        "tone.amount" => "FM: its depth · the others: how much replaces Tone",
        "tone.feedback" => "FM: Tone modulates itself; noise at the top",
        "tone.morph" => "Tangle, Shame: which combination (crossfading)",
        "tone.window" => "VOSIM: how fast each burst fades (formant width)",
        "tone.rattle" => "Tone ring-modulated by pink noise: a rattle",
        "tone.damp" => "lowpass gate: open at the hit, darker as it fades",
        "body.clip" => "overloads the ring: loud hits squash, lopsided",
        "crush.amount" => "sample rate first, then bits",
        "drive.amount" => "the first third blends it under the clean hit",
        "ring_mod.amount" => "deeper, and its carrier climbs 1.07× → 3.7×",
        "comb.amount" => "tuned to Tone's pitch; stretched, bell-like",
        "compressor.threshold" => "4:1, hard knee, auto makeup; 0 dB is off",
        "voice.variation" => "hit-to-hit: pitch, decays, strike, level",
        "tone.pitch" => "the resting pitch (note 60 with Keytrack)",
        _ if id.ends_with("body_send") => "how hard this hit strikes the Body",
        _ if id.starts_with("sample") && id.ends_with(".decay") => "fades the hit; the top plays it whole",
        _ => "",
    }
}

fn picture(params: &DrumParams, c: &Control) -> Picture {
    let group = c.id.split('.').next().unwrap_or("");
    let name = c.id.split('.').nth(1).unwrap_or("");
    match (group, name) {
        ("envelope", _) => {
            let caption = format!("attack {} · decay {} · curve {} · vel {}", values::text(params, "envelope.attack"),
                                  values::text(params, "envelope.decay"), values::text(params, "envelope.curve"), values::text(params, "envelope.velocity"));
            mod_envelope_picture(params, caption)
        }
        ("tone" | "noise", "decay" | "snap" | "tail_level" | "attack") | ("body", "decay") => envelope_of(params, group),
        ("tone", "drop" | "split" | "fast_time" | "slow_time" | "vel_pitch" | "hold" | "vel_time") => pitch_drop(params),
        ("noise", "bandpass" | "q" | "highpass" | "env_amount") => noise_filter(params),
        ("filter", "cutoff" | "resonance") => pad_filter(params),
        ("chop", _) => chop_picture(params),
        ("tone", "pitch") => Picture::Tuning { hz: plain(params, "tone.pitch"), caption: "Shift-drag for between the notes".into() },
        ("body", "offset") => {
            let hz = plain(params, "tone.pitch") * 2f32.powf(plain(params, "body.offset") / 12.0);
            Picture::Tuning { hz, caption: format!("the Body rings at {:.1} Hz ({} from Tone)", hz, values::text(params, "body.offset")) }
        }
        _ => match (c.kind, list(params, c.id)) {
            (Kind::Stepper | Kind::Chip | Kind::Choice { .. }, Some((items, current))) => Picture::List { items, current },
            _ => {
                let fill = params.ptr(c.id).map(|p| unsafe { p.unmodulated_normalized_value() }).unwrap_or(0.0);
                Picture::Value { text: values::text(params, c.id), fill, caption: caption(c.id) }
            }
        },
    }
}

/// The screen's title for a control: "TONE · DECAY".
fn title(c: &Control) -> String {
    let part = match c.card {
        Some(part) => part.title(),
        None if c.id.starts_with("voice") || c.id.starts_with("master") => "Voice",
        None if c.id.starts_with("envelope") => "Envelope",
        None if c.id.starts_with("filter") => "Filter",
        None if c.id.starts_with("chop") => "Chop",
        None => "Effects",
    };
    format!("{} · {}", part, c.label).to_uppercase()
}

/// The nearest note, large, and a `--|--` meter of how far off it is.
fn tuning(p: &mut Painter, body: Rect, hz: f32, caption: &str) {
    let (note, cents) = values::nearest_note(hz);
    let in_tune = cents.abs() < 1.0;
    p.text(body.x, body.y + 18.0, &note, Text::mono(28.0, PHOSPHOR).left());
    let off = if in_tune { "in tune".to_string() } else { format!("{cents:+.0} ¢").replace('-', "−") };
    p.text(body.right(), body.y + 18.0, &off, Text::mono(13.0, if in_tune { PHOSPHOR } else { PHOSPHOR_DIM }).right());
    // −50 ¢ .. +50 ¢: a rule with ticks every 25 ¢, the centre tall, and the marker.
    let (y, x0, w) = (body.y + 46.0, body.x + 2.0, body.w - 4.0);
    p.fill_rect(Rect::new(x0, y, w, 1.0), 0.0, super::draw::GLASS_LINE);
    for k in 0..=4 {
        let h = if k == 2 { 12.0 } else { 6.0 };
        p.fill_rect(Rect::new(x0 + w * k as f32 / 4.0 - 0.5, y - h / 2.0, 1.0, h), 0.0, if k == 2 { PHOSPHOR_DIM } else { super::draw::GLASS_LINE });
    }
    let mx = x0 + w * (0.5 + cents / 100.0);
    p.fill_rect(Rect::new(mx - 2.0, y - 7.0, 4.0, 14.0), 1.5, PHOSPHOR);
    p.text_in(Rect::new(body.x, body.bottom() - 12.0, body.w, 12.0), caption, Text::mono(10.0, PHOSPHOR_DIM).left());
}

/// Something to say on the screen, with nothing focused (the die, an export).
pub struct Message {
    pub title: String,
    pub big: String,
    pub caption: String,
    /// A second line (where a file went).
    pub detail: String,
}

/// `ring`: show the focused control's envelope ring (`depth`, as its knob runs) rather than the control.
pub fn draw(p: &mut Painter, params: &DrumParams, focus: Option<&Control>, ring: Option<f32>, message: Option<&Message>, r: Rect) {
    p.glass(r);
    let inner = r.inset(12.0, 9.0, 12.0, 9.0);
    let head = Text::mono(11.5, PHOSPHOR);
    let Some(c) = focus else {
        let (title, big, caption, detail) = match message {
            Some(m) => (m.title.as_str(), m.big.as_str(), m.caption.as_str(), m.detail.as_str()),
            None => ("ABDRUM", "", "touch a control to see it here", ""),
        };
        p.text_in(Rect::new(inner.x, inner.y, inner.w, 14.0), title, head.left());
        let size = if big.chars().count() > 18 { 19.0 } else { 28.0 };
        p.text_in(Rect::new(inner.x, inner.y + 24.0, inner.w, 34.0), big, Text::mono(size, PHOSPHOR).left());
        let small = Text::mono(10.0, PHOSPHOR_DIM).left();
        p.text_in(Rect::new(inner.x, inner.bottom() - 30.0, inner.w, 12.0), caption, small);
        p.text_in(Rect::new(inner.x, inner.bottom() - 15.0, inner.w, 12.0), detail, small);
        return;
    };
    let picture = match ring {
        Some(depth) => {
            let way = if depth >= 0.0 { "up" } else { "down" };
            let caption = format!("at its peak: {} {way} {:.0} % of its travel", c.label, depth.abs() * 100.0);
            mod_envelope_picture(params, caption)
        }
        None => picture(params, c),
    };
    let value = match &picture {
        _ if ring.is_some() => ring_text(ring.unwrap_or(0.0)),
        Picture::List { items, current } => format!("{}/{}", current + 1, items.len()),
        Picture::Tuning { hz, .. } => format!("{hz:.1} Hz"),
        Picture::Value { .. } => String::new(),
        Picture::Curve { .. } => values::text(params, c.id),
    };
    let title_r = Rect::new(inner.x, inner.y, inner.w * 0.68, 14.0);
    let mut title = title(c);
    if ring.is_some() {
        title = format!("{title} · RING");
    }
    if c.id.ends_with(".sample") || c.id.ends_with(".set") {
        let (set, _, _) = values::layer_choice(params, c.id.split('.').next().unwrap_or(""));
        title = format!("{} · {}", c.card.map_or("", |p| p.title()), values::SET_NAMES[set]).to_uppercase();
    }
    p.text_in(title_r, &title, head.left());
    p.text_in(Rect::new(title_r.right(), inner.y, inner.w - title_r.w, 14.0), &value, head.right());

    let body = Rect::new(inner.x, inner.y + 20.0, inner.w, inner.h - 20.0);
    match picture {
        Picture::Tuning { hz, caption } => tuning(p, body, hz, &caption),
        Picture::Curve { ys, caption } => {
            p.screen_curve(Rect::new(body.x, body.y, body.w, body.h - 18.0), &ys);
            p.text_in(Rect::new(body.x, body.bottom() - 12.0, body.w, 12.0), &caption, Text::mono(10.0, PHOSPHOR_DIM).left());
        }
        Picture::List { items, current } => {
            const ROWS: usize = 7;
            let n = items.len();
            let first = if n <= ROWS { 0 } else { (current + n - 2) % n };
            for row in 0..ROWS.min(n) {
                let i = (first + row) % n;
                let on = i == current;
                let line = format!("{} {}", if on { "›" } else { " " }, values::short_loop(&items[i]));
                let y = body.y + row as f32 * 14.0;
                p.text_in(Rect::new(body.x, y, body.w, 14.0), &line, Text::mono(10.5, if on { PHOSPHOR } else { PHOSPHOR_DIM }).left());
            }
        }
        Picture::Value { text, fill, caption } => {
            p.text_in(Rect::new(body.x, body.y + 4.0, body.w, 34.0), &text, Text::mono(28.0, PHOSPHOR).left());
            let bar = Rect::new(body.x, body.y + 48.0, body.w, 4.0);
            p.fill_rect(bar, 2.0, super::draw::GLASS_LINE);
            p.fill_rect(Rect::new(bar.x, bar.y, bar.w * fill.clamp(0.0, 1.0), bar.h), 2.0, PHOSPHOR);
            p.text_in(Rect::new(body.x, bar.bottom() + 8.0, body.w, 12.0), caption, Text::mono(10.0, PHOSPHOR_DIM).left());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The screen's picture of the pad filter agrees with the DSP's
    /// measurements (engine.rs's filter tests): -12 dB at the cutoff with no
    /// resonance, 24 dB/oct, -4.4 dB of bass at full resonance.
    #[test]
    fn the_filter_picture_matches_the_dsp() {
        assert!((filter_db(1000.0, 0.0, 1000.0) + 12.0).abs() < 0.1);
        assert!((filter_db(1000.0, 0.0, 4000.0) - filter_db(1000.0, 0.0, 8000.0) - 23.3).abs() < 0.2);
        assert!((filter_db(1000.0, 1.0, 20.0) + 4.4).abs() < 0.2);
        assert!(filter_db(1000.0, 1.0, 1000.0) > 30.0);
    }
}
