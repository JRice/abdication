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
        let attack = if attack > 0.0 { format!("attack {} · ", values::ms(attack)) } else { String::new() };
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
    let (drop, split, fast, slow) = (p("drop"), p("split"), p("fast_time"), p("slow_time"));
    let window = 150f32.max(slow * 2.5);
    let ys = (0..POINTS)
        .map(|i| {
            let t = i as f32 / (POINTS - 1) as f32 * window;
            let st = drop * split * (-3.0 * t / fast).exp() + drop * (1.0 - split) * (-3.0 * t / slow).exp();
            st / drop.max(1.0)
        })
        .collect();
    let caption = format!("+{drop:.0} st: {:.0} % over {}, the rest over {}", split * 100.0, values::ms(fast), values::ms(slow));
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
    let caption = format!("bandpass {} · Q {q:.2} · highpass {} · 40 Hz–16 kHz", values::text(params, "noise.bandpass"), values::text(params, "noise.highpass"));
    Picture::Curve { ys, caption }
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
            let caption = format!("attack {} · decay {} · curve {} · velocity {}", values::text(params, "envelope.attack"),
                                  values::text(params, "envelope.decay"), values::text(params, "envelope.curve"), values::text(params, "envelope.velocity"));
            mod_envelope_picture(params, caption)
        }
        ("tone" | "noise", "decay" | "snap" | "tail_level" | "attack") | ("body", "decay") => envelope_of(params, group),
        ("tone", "drop" | "split" | "fast_time" | "slow_time" | "vel_pitch") => pitch_drop(params),
        ("noise", "bandpass" | "q" | "highpass" | "env_amount") => noise_filter(params),
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
            let caption = format!("at its peak the envelope moves {} {:.0} % of its travel {way}", c.label, depth.abs() * 100.0);
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
