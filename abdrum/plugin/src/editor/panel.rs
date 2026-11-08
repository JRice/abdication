//! The whole panel as one vizia view (Abdication's approach): it draws
//! every card and control itself, hit-tests the mouse against the layout,
//! and changes parameters through nih-plug's gesture events, so the host
//! records automation and undo exactly as for its own controls.
//!
//! Knobs: drag up/down (Shift: fine), scroll, double-click to reset. Level
//! bars: click or drag along them. Chips step on a click; steppers step
//! with their arrows and open their list from the middle. Hovering or
//! touching a control shows it on the screen. ⟲ turns a card over. The
//! square before a card's name switches the part off and back on.
//!
//! Rings: the band just outside a knob's arc is its envelope ring (the send
//! from the free envelope). Drag it up or down to set how far, and which
//! way, the envelope moves the knob; double-click it to clear it.
//!
//! The die (top bar) rolls a random sound (randomize.rs); right-click it to
//! go back to the sound before. The export button renders one hit to a WAV:
//! drag from it into the DAW, or click it to just save the file.

use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;

use nih_plug::prelude::ParamPtr;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::util::ModifiersExt;
use nih_plug_vizia::widgets::RawParamEvent;

use super::draw::{self, part_accent, KnobGeometry, MenuGrid, Painter, Ring, Text, DIM, INK, MOD, STONE};
use super::drag;
use super::randomize::{self, Dice};
use super::screens::Message;
use crate::export::Exporter;
use super::layout::{Control, Kind, Layout, Part, Rect, Side, HEIGHT, WIDTH};
use super::scopes::Scopes;
use super::screens;
use super::values::{self, plain};
use crate::params::DrumParams;
use crate::scope::HitScope;

/// Logical pixels of mouse travel for a knob's full range.
const KNOB_TRAVEL: f32 = 200.0;
/// Shift-drag moves this much slower.
const FINE: f32 = 0.12;
const MENU_ROW: f32 = 19.0;
const MENU_MAX_ROWS: usize = 22;
/// A stepper's arrow zones.
const ARROW_W: f32 = 18.0;
/// The panel's housekeeping tick (scopes, meter, hover fix): about 15 Hz.
const TICK: Duration = Duration::from_millis(66);
/// The meter falls this much per tick (about 20 dB a second).
const METER_FALL: f32 = 0.85;
/// A part switched back on with no level to return to comes on at -6 dB.
const POWER_ON_LEVEL: f32 = 0.501_187;
/// Pitch knobs land on notes (A440 equal temperament) or whole semitones;
/// Shift (fine) frees them.
const SNAP_HZ: [&str; 1] = ["tone.pitch"];
const SNAP_SEMITONES: [&str; 4] = ["body.offset", "sample_1.pitch", "sample_2.pitch", "noise.loop_pitch"];
/// Shown the other way round: 0 dB (off) at the left.
const INVERTED: [&str; 1] = ["compressor.threshold"];

enum PanelEvent {
    Tick,
}

/// What the mouse is over.
#[derive(Clone, Copy, PartialEq)]
enum Target {
    Control(usize),
    /// A knob's envelope ring.
    Ring(usize),
    Die,
    Export,
    Flip(Part),
    Power(Part),
}

pub struct Panel {
    params: Arc<DrumParams>,
    layout: Layout,
    ptrs: Vec<ParamPtr>,
    /// Each control's envelope ring ("<id>_mod"), if it has one.
    rings: Vec<Option<ParamPtr>>,
    flipped: [bool; 5],
    /// Each part's level from before its square switched it off.
    levels_before_off: [Option<f32>; 5],
    hover: Option<Target>,
    /// The control shown on the screen (its ring, if `focus_ring`).
    focus: Option<usize>,
    focus_ring: bool,
    drag: Option<Drag>,
    menu: Option<Menu>,
    scopes: Scopes,
    meter_peak: Arc<AtomicU32>,
    meter: f32,
    sample_rate: Arc<AtomicU32>,
    /// Said on the screen while nothing's focused (the die, an export).
    message: Option<Message>,
    /// Every parameter's value from before the last roll of the die.
    before_roll: Option<Vec<(ParamPtr, f32)>>,
    die_face: u8,
    exporter: Exporter,
    /// The export button is held down here (a drag out starts once it moves).
    export_press: Option<(f32, f32)>,
    /// Keeps the ticker thread running; cleared when the panel goes away.
    ticking: Arc<AtomicBool>,
}

impl Drop for Panel {
    fn drop(&mut self) {
        self.ticking.store(false, Ordering::Relaxed);
    }
}

struct Drag {
    control: usize,
    /// Dragging the control's ring rather than the control.
    ring: bool,
    /// Where the drag is anchored (re-anchored when Shift changes so the value never jumps).
    anchor: f32,
    anchor_value: f32,
    fine: bool,
}

struct Menu {
    control: usize,
    items: Vec<String>,
    /// The normalized parameter value each item sets.
    values: Vec<f32>,
    selected: Option<usize>,
    rect: Rect,
    grid: MenuGrid,
    hover: Option<usize>,
}

fn part_index(part: Part) -> usize {
    Part::ALL.iter().position(|p| *p == part).unwrap_or(0)
}

impl Panel {
    pub fn new(cx: &mut Context, params: Arc<DrumParams>, live: Arc<HitScope>, meter_peak: Arc<AtomicU32>,
               sample_rate: Arc<AtomicU32>) -> Handle<'_, Self> {
        let layout = Layout::build();
        let ptrs = layout.controls.iter().map(|c| params.ptr(c.id).unwrap_or_else(|| panic!("no parameter '{}'", c.id))).collect();
        let rings = layout
            .controls
            .iter()
            .map(|c| matches!(c.kind, Kind::Knob { .. }).then(|| params.ptr(&format!("{}_mod", c.id))).flatten())
            .collect();
        let scopes = Scopes::new(&params, live);
        let ticking = Arc::new(AtomicBool::new(true));
        let ticker = ticking.clone();
        Self {
            params, layout, ptrs, rings, flipped: [false; 5], levels_before_off: [None; 5], hover: None, focus: None,
            focus_ring: false, drag: None, menu: None,
            scopes, meter_peak, meter: 0.0, sample_rate, message: None, before_roll: None, die_face: 5,
            exporter: Exporter::default(), export_press: None, ticking,
        }
        .build(cx, move |cx| {
            // vizia's timers never fire under baseview, but events sent through
            // a context proxy are delivered every frame (as in Abdication).
            let mut proxy = cx.get_proxy();
            std::thread::spawn(move || {
                while ticker.load(Ordering::Relaxed) {
                    std::thread::sleep(TICK);
                    let _ = proxy.emit(PanelEvent::Tick);
                }
            });
        })
        .width(Stretch(1.0))
        .height(Stretch(1.0))
    }

    // ---- Parameter access ------------------------------------------------------

    fn id(&self, i: usize) -> &'static str {
        self.layout.controls[i].id
    }

    fn value(&self, i: usize) -> f32 {
        unsafe { self.ptrs[i].unmodulated_normalized_value() }
    }

    /// The value as the control shows it (some run the other way round).
    fn shown(&self, i: usize) -> f32 {
        if INVERTED.contains(&self.id(i)) { 1.0 - self.value(i) } else { self.value(i) }
    }

    fn steps(&self, i: usize) -> usize {
        unsafe { self.ptrs[i].step_count() }.unwrap_or(0)
    }

    fn begin(cx: &mut EventContext, p: ParamPtr) {
        cx.emit(RawParamEvent::BeginSetParameter(p));
    }
    fn set(cx: &mut EventContext, p: ParamPtr, v: f32) {
        cx.emit(RawParamEvent::SetParameterNormalized(p, v.clamp(0.0, 1.0)));
    }
    fn end(cx: &mut EventContext, p: ParamPtr) {
        cx.emit(RawParamEvent::EndSetParameter(p));
    }
    /// A complete one-shot change (click, wheel notch, menu pick).
    fn change(cx: &mut EventContext, p: ParamPtr, v: f32) {
        Self::begin(cx, p);
        Self::set(cx, p, v);
        Self::end(cx, p);
    }

    fn snaps(&self, i: usize) -> bool {
        SNAP_HZ.contains(&self.id(i)) || SNAP_SEMITONES.contains(&self.id(i))
    }

    /// A pitch knob's normalized value moved to the nearest note or whole
    /// semitone, `steps` notes up or down from there.
    fn snapped(&self, i: usize, normalized: f32, steps: f32) -> f32 {
        let p = self.ptrs[i];
        let plain = unsafe { p.preview_plain(normalized.clamp(0.0, 1.0)) };
        let snapped = if SNAP_HZ.contains(&self.id(i)) {
            values::note_hz(values::note_number(plain).round() + steps)
        } else {
            plain.round() + steps
        };
        unsafe { p.preview_normalized(snapped) }
    }

    /// A ring's depth as its knob shows it: -1..1 of the knob's travel.
    fn ring_shown(&self, i: usize) -> Option<f32> {
        let depth = unsafe { self.rings[i]?.unmodulated_plain_value() };
        Some(if INVERTED.contains(&self.id(i)) { -depth } else { depth })
    }

    fn set_ring_shown(&self, cx: &mut EventContext, i: usize, shown: f32) {
        let Some(p) = self.rings[i] else { return };
        let depth = shown.clamp(-1.0, 1.0) * if INVERTED.contains(&self.id(i)) { -1.0 } else { 1.0 };
        Self::set(cx, p, unsafe { p.preview_normalized(depth) });
    }

    fn set_shown(&self, cx: &mut EventContext, i: usize, shown: f32) {
        let v = if INVERTED.contains(&self.id(i)) { 1.0 - shown } else { shown };
        Self::set(cx, self.ptrs[i], v);
    }

    /// Step a menu-like control by one, wrapping. A sample stepper wraps
    /// within its set (as the DSP does).
    fn step(&self, cx: &mut EventContext, i: usize, up: bool) {
        let id = self.id(i);
        if id.ends_with(".sample") {
            let (_, index, count) = values::layer_choice(&self.params, id.split('.').next().unwrap_or(""));
            let next = (index as i64 + if up { 1 } else { -1 }).rem_euclid(count as i64) as f32;
            Self::change(cx, self.ptrs[i], next / self.steps(i).max(1) as f32);
            return;
        }
        let steps = self.steps(i);
        if steps == 0 {
            return;
        }
        let index = (self.value(i) * steps as f32).round() as i64;
        let next = (index + if up { 1 } else { -1 }).rem_euclid(steps as i64 + 1);
        Self::change(cx, self.ptrs[i], next as f32 / steps as f32);
    }

    // ---- What's shown --------------------------------------------------------

    fn flipped(&self, part: Part) -> bool {
        self.flipped[part_index(part)]
    }

    fn visible(&self, c: &Control) -> bool {
        match (c.card, c.side) {
            (None, _) | (_, Side::Always) => true,
            (Some(part), Side::Front) => !self.flipped(part),
            (Some(part), Side::Back) => self.flipped(part),
        }
    }

    fn sample_set(&self, part: Part) -> usize {
        match part {
            Part::Sample1 | Part::Sample2 => plain(&self.params, &format!("{}.set", part.group())) as usize,
            _ => 0,
        }
    }

    fn accent(&self, c: &Control) -> u32 {
        c.card.map_or(STONE, |part| part_accent(part, self.sample_set(part)))
    }

    /// Whether a control does anything with the current settings (if not,
    /// it's dimmed, but still adjustable).
    fn applies(&self, c: &Control) -> bool {
        let p = |id: &str| plain(&self.params, id);
        // A control's knob, or its ring, reaching above (or below) a value.
        let ringed = |id: &str| p(&format!("{id}_mod")) != 0.0;
        let group = c.id.split('.').next().unwrap_or("");
        let part_on = match c.card {
            Some(Part::Sample1 | Part::Sample2) => p(&format!("{group}.level")) > 0.0 || p(&format!("{group}.body_send")) > 0.0,
            Some(_) => p(&format!("{group}.level")) > 0.0,
            None => true,
        };
        if c.kind == Kind::Level {
            return true;
        }
        part_on
            && match c.id {
                "noise.loop" | "noise.loop_pitch" => p("noise.blend") > 0.0 || ringed("noise.blend"),
                "noise.generator" => p("noise.blend") < 1.0 || ringed("noise.blend"),
                "tone.split" | "tone.fast_time" | "tone.slow_time" | "tone.vel_pitch" => p("tone.drop") > 0.0 || ringed("tone.drop"),
                "tone.tail_level" => p("tone.snap") > 0.0 || ringed("tone.snap"),
                "drive.type" => p("drive.amount") > 0.0 || ringed("drive.amount"),
                "compressor.timing" => p("compressor.threshold") < 0.0 || ringed("compressor.threshold"),
                _ => true,
            }
    }

    // ---- Geometry ----------------------------------------------------------------

    fn transform(bounds: BoundingBox) -> (f32, f32, f32) {
        let s = (bounds.w / WIDTH).min(bounds.h / HEIGHT);
        (s, bounds.x, bounds.y)
    }

    fn to_panel(cx: &EventContext, x: f32, y: f32) -> (f32, f32) {
        let (s, ox, oy) = Self::transform(cx.bounds());
        ((x - ox) / s, (y - oy) / s)
    }

    fn hit(&self, x: f32, y: f32) -> Option<Target> {
        if self.layout.die.contains(x, y) {
            return Some(Target::Die);
        }
        if self.layout.export.contains(x, y) {
            return Some(Target::Export);
        }
        for card in &self.layout.cards {
            if card.flip.is_some_and(|r| r.contains(x, y)) {
                return Some(Target::Flip(card.part));
            }
            if card.power.contains(x, y) {
                return Some(Target::Power(card.part));
            }
        }
        let on_ring = |c: &Control| match c.kind {
            Kind::Knob { size, .. } => KnobGeometry::of(c.rect, size).on_ring(x, y),
            _ => false,
        };
        if let Some(i) = (0..self.layout.controls.len())
            .find(|&i| self.rings[i].is_some() && self.visible(&self.layout.controls[i]) && on_ring(&self.layout.controls[i]))
        {
            return Some(Target::Ring(i));
        }
        self.layout
            .controls
            .iter()
            .position(|c| self.visible(c) && c.rect.contains(x, y))
            .map(Target::Control)
    }

    fn open_menu(&mut self, i: usize) {
        let id = self.id(i);
        let steps = self.steps(i).max(1);
        let (items, values, selected): (Vec<String>, Vec<f32>, Option<usize>) = if id.ends_with(".sample") {
            let (set, index, _) = values::layer_choice(&self.params, id.split('.').next().unwrap_or(""));
            let hits = values::set_hits(set);
            let values = (0..hits.len()).map(|k| k as f32 / steps as f32).collect();
            (hits.into_iter().map(String::from).collect(), values, Some(index))
        } else {
            let p = self.ptrs[i];
            let items = (0..=steps).map(|k| unsafe { p.normalized_value_to_string(k as f32 / steps as f32, false) }).collect();
            let values = (0..=steps).map(|k| k as f32 / steps as f32).collect();
            (items, values, Some((self.value(i) * steps as f32).round() as usize))
        };
        let rows = items.len().min(MENU_MAX_ROWS);
        let cols = items.len().div_ceil(rows);
        let longest = items.iter().map(|s| s.chars().count()).max().unwrap_or(4) as f32;
        let col_w = (longest * 6.7 + 20.0).clamp(64.0, 300.0);
        let control = self.layout.controls[i].rect;
        let (w, h) = ((col_w * cols as f32 + 12.0).max(control.w), rows as f32 * MENU_ROW + 12.0);
        let x = (control.x + control.w / 2.0 - w / 2.0).clamp(6.0, WIDTH - 6.0 - w);
        let below = control.bottom() + 4.0;
        let y = if below + h <= HEIGHT - 6.0 {
            below
        } else if control.y - 4.0 - h >= 6.0 {
            control.y - 4.0 - h
        } else {
            ((HEIGHT - h) / 2.0).max(6.0)
        };
        let grid = MenuGrid { rows, col_w, row_h: MENU_ROW };
        self.menu = Some(Menu { control: i, items, values, selected, rect: Rect::new(x, y, w, h), grid, hover: None });
    }

    fn menu_item_at(menu: &Menu, x: f32, y: f32) -> Option<usize> {
        (0..menu.items.len()).find(|&k| menu.grid.cell(menu.rect, k).contains(x, y))
    }

    // ---- Mouse -------------------------------------------------------------------

    /// Works around a vizia bug on Windows (see Abdication's panel): once the
    /// pointer has left past the window's edge, clicks go to the window until
    /// it's told the pointer is back.
    fn keep_hover_alive(cx: &mut EventContext) {
        let bounds = cx.bounds();
        let (x, y) = (cx.mouse().cursorx, cx.mouse().cursory);
        if x >= bounds.x && y >= bounds.y && x < bounds.x + bounds.w && y < bounds.y + bounds.h {
            cx.emit_custom(Event::new(WindowEvent::MouseEnter).origin(Entity::root()).direct(Entity::root()));
        }
    }

    fn mouse_down(&mut self, cx: &mut EventContext, x: f32, y: f32) {
        self.mouse_up(cx);
        if let Some(menu) = self.menu.take() {
            if let Some(k) = Self::menu_item_at(&menu, x, y) {
                Self::change(cx, self.ptrs[menu.control], menu.values[k]);
                self.focus = Some(menu.control);
            }
            return;
        }
        let i = match self.hit(x, y) {
            Some(Target::Flip(part)) => {
                let k = part_index(part);
                self.flipped[k] = !self.flipped[k];
                return;
            }
            Some(Target::Power(part)) => {
                self.toggle_power(cx, part);
                return;
            }
            Some(Target::Die) => {
                self.roll(cx);
                return;
            }
            Some(Target::Export) => {
                self.export_press = Some((x, y));
                return;
            }
            Some(Target::Ring(i)) => {
                self.focus = Some(i);
                self.focus_ring = true;
                if let Some(p) = self.rings[i] {
                    Self::begin(cx, p);
                    let depth = self.ring_shown(i).unwrap_or(0.0);
                    self.drag = Some(Drag { control: i, ring: true, anchor: y, anchor_value: depth, fine: cx.modifiers().shift() });
                    cx.capture();
                }
                return;
            }
            Some(Target::Control(i)) => i,
            None => return,
        };
        self.focus = Some(i);
        self.focus_ring = false;
        let c = self.layout.controls[i];
        let p = self.ptrs[i];
        match c.kind {
            Kind::Knob { .. } => {
                Self::begin(cx, p);
                self.drag = Some(Drag { control: i, ring: false, anchor: y, anchor_value: self.shown(i), fine: cx.modifiers().shift() });
                cx.capture();
            }
            Kind::Level => {
                Self::begin(cx, p);
                let v = (x - c.rect.x) / c.rect.w;
                Self::set(cx, p, v);
                self.drag = Some(Drag { control: i, ring: false, anchor: x, anchor_value: v.clamp(0.0, 1.0), fine: cx.modifiers().shift() });
                cx.capture();
            }
            Kind::Chip => self.step(cx, i, true),
            Kind::Choice { value } => Self::change(cx, p, value / self.steps(i).max(1) as f32),
            Kind::Stepper => {
                if x < c.rect.x + ARROW_W {
                    self.step(cx, i, false);
                } else if x > c.rect.right() - ARROW_W {
                    self.step(cx, i, true);
                } else {
                    self.open_menu(i);
                }
            }
            Kind::Toggle => Self::change(cx, p, if self.value(i) > 0.5 { 0.0 } else { 1.0 }),
        }
    }

    /// A random sound (all but Volume): first remembering this one, for the
    /// right-click back.
    fn roll(&mut self, cx: &mut EventContext) {
        let ptrs: Vec<(&str, ParamPtr)> = self.params.ids().filter(|id| *id != "master.volume")
            .filter_map(|id| Some((id, self.params.ptr(id)?))).collect();
        self.before_roll = Some(ptrs.iter().map(|(_, p)| (*p, unsafe { p.unmodulated_normalized_value() })).collect());
        let seed = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(1, |d| d.as_nanos() as u64);
        let counts = [0, 1, 2].map(|set| values::set_hits(set).len());
        let (kind, patch) = randomize::roll(&mut Dice::new(seed), counts);
        for (id, p) in ptrs {
            let v = match patch.iter().find(|(k, _)| *k == id) {
                Some((_, plain)) => unsafe { p.preview_normalized(*plain) },
                None => unsafe { p.default_normalized_value() },
            };
            Self::change(cx, p, v);
        }
        self.die_face = (seed % 6) as u8 + 1;
        self.focus = None;
        self.message = Some(Message {
            title: "DIE".into(),
            big: kind.into(),
            caption: "roll again for another;".into(),
            detail: "right-click the die for the sound before".into(),
        });
    }

    /// Back to the sound before the last roll.
    fn unroll(&mut self, cx: &mut EventContext) {
        let Some(before) = self.before_roll.take() else { return };
        for (p, v) in before {
            Self::change(cx, p, v);
        }
        self.focus = None;
        self.message = Some(Message {
            title: "DIE".into(), big: "Back".into(), caption: "the sound before the last roll".into(), detail: String::new(),
        });
    }

    /// Render a hit to a WAV file; drag it out if `drag`, else just say where it went.
    fn export(&mut self, drag: bool) {
        let seed = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(1, |d| d.subsec_nanos() | 1);
        let sample_rate = f32::from_bits(self.sample_rate.load(Ordering::Relaxed));
        self.focus = None;
        self.message = Some(match self.exporter.export(&self.params, sample_rate, seed) {
            Ok(path) => {
                let name = path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
                let folder = path.parent().map(|d| d.display().to_string()).unwrap_or_default();
                let dragging = drag && drag::drag_file(path);
                let how = if dragging { "drop it in your DAW" } else if drag { "saved (dragging isn't available here)" } else { "saved; drag the button to drop a hit in your DAW" };
                Message { title: "EXPORT".into(), big: name, caption: how.into(), detail: format!("in {folder}") }
            }
            Err(e) => Message { title: "EXPORT".into(), big: "Couldn't save".into(), caption: e.to_string(), detail: String::new() },
        });
    }

    /// Off, or back on at the level it had before (else -6 dB).
    fn toggle_power(&mut self, cx: &mut EventContext, part: Part) {
        let Some(p) = self.params.ptr(&format!("{}.level", part.group())) else { return };
        let k = part_index(part);
        let level = unsafe { p.unmodulated_normalized_value() };
        if level > 0.0 {
            self.levels_before_off[k] = Some(level);
            Self::change(cx, p, 0.0);
        } else {
            let on = self.levels_before_off[k].take().unwrap_or_else(|| unsafe { p.preview_normalized(POWER_ON_LEVEL) });
            Self::change(cx, p, on);
        }
    }

    fn mouse_move(&mut self, cx: &mut EventContext, x: f32, y: f32) {
        if let Some(menu) = &mut self.menu {
            let hover = Self::menu_item_at(menu, x, y);
            if hover != menu.hover {
                menu.hover = hover;
                cx.needs_redraw();
            }
            return;
        }
        if self.drag.is_some() && cx.mouse().left.state != MouseButtonState::Pressed {
            self.mouse_up(cx); // the button is up but we never heard the release
        }
        if let Some((px, py)) = self.export_press {
            if cx.mouse().left.state != MouseButtonState::Pressed {
                self.export_press = None;
            } else if (x - px).abs() + (y - py).abs() > 4.0 {
                // Dragging off the button: render a hit and drag it out.
                self.export_press = None;
                self.export(true);
                cx.needs_redraw();
                return;
            }
        }
        if let Some((i, ring)) = self.drag.as_ref().map(|d| (d.control, d.ring)) {
            let c = self.layout.controls[i];
            let horizontal = c.kind == Kind::Level && !ring;
            let at = if horizontal { x } else { y };
            let fine = cx.modifiers().shift();
            let shown_now = if ring { self.ring_shown(i).unwrap_or(0.0) } else { self.shown(i) };
            let Some(drag) = self.drag.as_mut() else { return };
            if fine != drag.fine {
                drag.anchor_value = shown_now;
                drag.anchor = at;
                drag.fine = fine;
            }
            let speed = if fine { FINE } else { 1.0 };
            let moved = if horizontal { (at - drag.anchor) / c.rect.w } else { (drag.anchor - at) / KNOB_TRAVEL };
            let v = drag.anchor_value + moved * speed;
            if ring {
                self.set_ring_shown(cx, i, v);
                cx.needs_redraw();
                return;
            }
            let v = if self.snaps(i) && !fine { self.snapped(i, v, 0.0) } else { v };
            self.set_shown(cx, i, v);
            cx.needs_redraw();
            return;
        }
        let hover = self.hit(x, y);
        if hover != self.hover {
            self.hover = hover;
            match hover {
                Some(Target::Control(i)) => {
                    self.focus = Some(i);
                    self.focus_ring = false;
                }
                Some(Target::Ring(i)) => {
                    self.focus = Some(i);
                    self.focus_ring = true;
                }
                _ => {}
            }
            cx.needs_redraw();
        }
    }

    fn mouse_up(&mut self, cx: &mut EventContext) {
        if self.export_press.take().is_some() {
            // Clicked without dragging: just save the file.
            self.export(false);
        }
        if let Some(drag) = self.drag.take() {
            let p = if drag.ring { self.rings[drag.control] } else { Some(self.ptrs[drag.control]) };
            if let Some(p) = p {
                Self::end(cx, p);
            }
            cx.release();
        }
    }

    /// Knobs and level bars reset on a double-click; for anything else a
    /// quick second click is just another click (so arrows step every time).
    fn repeat_click(&mut self, cx: &mut EventContext, x: f32, y: f32, double: bool) {
        match self.hit(x, y) {
            Some(Target::Ring(i)) if self.menu.is_none() => {
                if double {
                    self.mouse_up(cx);
                    if let Some(p) = self.rings[i] {
                        Self::change(cx, p, unsafe { p.default_normalized_value() });
                    }
                    self.focus = Some(i);
                    self.focus_ring = true;
                }
            }
            Some(Target::Control(i)) if self.menu.is_none() && matches!(self.layout.controls[i].kind, Kind::Knob { .. } | Kind::Level) => {
                if double {
                    self.mouse_up(cx);
                    let p = self.ptrs[i];
                    Self::change(cx, p, unsafe { p.default_normalized_value() });
                    self.focus = Some(i);
                }
            }
            _ => self.mouse_down(cx, x, y),
        }
    }

    fn scroll(&mut self, cx: &mut EventContext, x: f32, y: f32, dy: f32) {
        if self.menu.is_some() || self.drag.is_some() || dy == 0.0 {
            return;
        }
        let up = dy > 0.0;
        let i = match self.hit(x, y) {
            Some(Target::Control(i)) => i,
            Some(Target::Ring(i)) => {
                if let (Some(p), Some(depth)) = (self.rings[i], self.ring_shown(i)) {
                    let notch = if cx.modifiers().shift() { 0.002 } else { 0.01 };
                    Self::begin(cx, p);
                    self.set_ring_shown(cx, i, depth + if up { notch } else { -notch });
                    Self::end(cx, p);
                }
                self.focus = Some(i);
                self.focus_ring = true;
                return;
            }
            _ => return,
        };
        let c = self.layout.controls[i];
        match c.kind {
            Kind::Chip | Kind::Stepper | Kind::Choice { .. } => self.step(cx, i, up),
            Kind::Knob { .. } | Kind::Level => {
                let p = self.ptrs[i];
                let v = self.shown(i);
                let next = if self.snaps(i) && !cx.modifiers().shift() {
                    self.snapped(i, v, if up { 1.0 } else { -1.0 })
                } else if self.steps(i) > 0 {
                    let n = self.steps(i) as f32;
                    ((v * n).round() + if up { 1.0 } else { -1.0 }) / n
                } else {
                    let notch = if cx.modifiers().shift() { 0.002 } else { 0.01 };
                    v + if up { notch } else { -notch }
                };
                Self::begin(cx, p);
                self.set_shown(cx, i, next);
                Self::end(cx, p);
            }
            Kind::Toggle => {}
        }
        self.focus = Some(i);
        self.focus_ring = false;
    }

    fn tick(&mut self, cx: &mut EventContext) {
        Self::keep_hover_alive(cx);
        let mut changed = self.scopes.refresh(&self.params);
        let peak = f32::from_bits(self.meter_peak.swap(0, Ordering::Relaxed));
        let next = peak.max(self.meter * METER_FALL);
        if (next - self.meter).abs() > 1e-4 {
            self.meter = next;
            changed = true;
        }
        if changed {
            cx.needs_redraw();
        }
    }

    // ---- Drawing -----------------------------------------------------------------

    fn draw_control(&self, p: &mut Painter, i: usize) {
        let c = self.layout.controls[i];
        let dragging = self.drag.as_ref().filter(|d| d.control == i);
        let hot = self.hover == Some(Target::Control(i)) || dragging.is_some_and(|d| !d.ring);
        let ring_hot = self.hover == Some(Target::Ring(i)) || dragging.is_some_and(|d| d.ring);
        let ring = self.ring_shown(i).map(|depth| Ring { depth, hot: ring_hot });
        let focused = self.focus == Some(i);
        let accent = self.accent(&c);
        let text = values::text(&self.params, c.id);
        match c.kind {
            Kind::Knob { size, bipolar } => p.knob(c.rect, size, self.shown(i), bipolar, c.label, &text, accent, focused, hot, ring),
            Kind::Level => {
                // Volume shows the output meter, on its own scale.
                let meter = (c.id == "master.volume").then(|| unsafe { self.ptrs[i].preview_normalized(self.meter) });
                p.level(c.rect, c.label, self.value(i), &text, accent, hot, meter)
            }
            Kind::Chip => {
                let color = if c.card.is_some() { accent } else { draw::PHOSPHOR };
                p.chip(c.rect, &text, color, hot)
            }
            Kind::Choice { value } => {
                let on = (plain(&self.params, c.id) - value).abs() < 0.5;
                let tint = draw::SET_TINTS[(value as usize).min(2)];
                p.choice(c.rect, values::SET_NAMES[(value as usize).min(2)], on, tint, hot)
            }
            Kind::Stepper => {
                let group = c.id.split('.').next().unwrap_or("");
                if c.id.ends_with(".sample") {
                    let (set, index, count) = values::layer_choice(&self.params, group);
                    let name = values::set_hits(set).get(index).copied().unwrap_or("");
                    p.stepper(c.rect, name, &format!("{}/{count}", index + 1), hot)
                } else {
                    p.stepper(c.rect, values::short_loop(&text), "", hot)
                }
            }
            Kind::Toggle => p.toggle(c.rect, c.label, &text, self.value(i) > 0.5, accent, hot),
        }
    }

    fn draw_cards(&self, p: &mut Painter) {
        for (k, card) in self.layout.cards.iter().enumerate() {
            let accent = part_accent(card.part, self.sample_set(card.part));
            let flipped = self.flipped(card.part);
            p.card(card.rect);
            let on = plain(&self.params, &format!("{}.level", card.part.group())) > 0.0;
            let hot = self.hover == Some(Target::Power(card.part));
            p.card_title(card.title, card.part.title(), accent, if flipped { "detail" } else { "" }, on, hot);
            if let Some(r) = card.flip {
                p.flip_button(r, flipped, self.hover == Some(Target::Flip(card.part)));
            }
            if !flipped {
                p.scope(card.scope, &self.scopes.traces[k], accent);
            }
        }
    }

    fn draw_effects_frame(&self, p: &mut Painter) {
        let r = self.layout.effects;
        p.card(r);
        p.text(r.x + 12.0, r.y + 14.0, "Effects", Text::label(11.0, DIM).left().spaced(0.14));
    }

    /// The free envelope's frame, title and picture (its knobs draw with the rest).
    fn draw_envelope_frame(&self, p: &mut Painter) {
        let r = self.layout.envelope;
        p.card(r);
        p.text(r.x + 12.0, r.y + 14.0, "Envelope", Text::label(11.0, MOD).left().spaced(0.14));
        p.text(r.right() - 12.0, r.y + 14.0, "→ rings", Text::label(11.0, DIM).right().spaced(0.1));
        let curve = self.layout.envelope_curve;
        p.fill_rect(curve, 3.0, draw::GROUND);
        p.curve(curve.inset(4.0, 3.0, 4.0, 0.0), &screens::mod_envelope(&self.params), MOD, draw::CARD);
    }

    fn draw_top_bar(&self, p: &mut Painter) {
        let r = self.layout.top_bar;
        let cy = r.y + r.h / 2.0;
        let title = Text::label(20.0, INK).left().spaced(0.28);
        p.text(r.x + 4.0, cy, "ABDrum", title);
        let w = p.text_width("ABDrum", title);
        p.text(r.x + 14.0 + w, cy + 2.0, "Abdicant", Text::label(12.0, DIM).left().spaced(0.14));
        p.die(self.layout.die, self.die_face, self.hover == Some(Target::Die));
        let label = if drag::AVAILABLE { "Drag WAV" } else { "Save WAV" };
        p.export_button(self.layout.export, label, self.hover == Some(Target::Export) || self.export_press.is_some());
    }
}

impl View for Panel {
    fn element(&self) -> Option<&'static str> {
        Some("abdrum-panel")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|e: &PanelEvent, _| match e {
            PanelEvent::Tick => self.tick(cx),
        });
        event.map(|e: &RawParamEvent, _| {
            if let RawParamEvent::ParametersChanged = e {
                cx.needs_redraw();
            }
        });
        event.map(|e: &WindowEvent, meta| {
            let (mx, my) = (cx.mouse().cursorx, cx.mouse().cursory);
            let (x, y) = Self::to_panel(cx, mx, my);
            match *e {
                WindowEvent::MouseMove(px, py) => {
                    let (x, y) = Self::to_panel(cx, px, py);
                    self.mouse_move(cx, x, y);
                    return;
                }
                WindowEvent::MouseDown(MouseButton::Left) => {
                    self.mouse_down(cx, x, y);
                    meta.consume();
                }
                WindowEvent::MouseDoubleClick(MouseButton::Left) => {
                    self.repeat_click(cx, x, y, true);
                    meta.consume();
                }
                WindowEvent::MouseTripleClick(MouseButton::Left) => {
                    self.repeat_click(cx, x, y, false);
                    meta.consume();
                }
                WindowEvent::MouseUp(MouseButton::Left) => self.mouse_up(cx),
                WindowEvent::MouseDown(MouseButton::Right) => {
                    if self.menu.take().is_some() {
                        meta.consume();
                    } else if self.hit(x, y) == Some(Target::Die) {
                        self.unroll(cx);
                        meta.consume();
                    }
                }
                WindowEvent::MouseScroll(_, dy) => {
                    self.scroll(cx, x, y, dy);
                    meta.consume();
                }
                WindowEvent::KeyDown(Code::Escape, _) => self.menu = None,
                WindowEvent::MouseLeave => self.hover = None,
                _ => return,
            }
            cx.needs_redraw();
        });
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        let (s, ox, oy) = Self::transform(cx.bounds());
        canvas.save();
        canvas.translate(ox, oy);
        canvas.scale(s, s);
        let mut p = Painter::new(canvas);

        p.fill_rect(Rect::new(0.0, 0.0, WIDTH, HEIGHT), 0.0, draw::GROUND);
        self.draw_top_bar(&mut p);
        self.draw_cards(&mut p);
        self.draw_effects_frame(&mut p);
        self.draw_envelope_frame(&mut p);
        let focus = self.focus.map(|i| &self.layout.controls[i]);
        let ring = self.focus.filter(|_| self.focus_ring).and_then(|i| self.ring_shown(i));
        screens::draw(&mut p, &self.params, focus, ring, self.message.as_ref(), self.layout.screen);

        for (i, c) in self.layout.controls.iter().enumerate() {
            if !self.visible(c) {
                continue;
            }
            let dimmed = !self.applies(c);
            if dimmed {
                p.canvas.save();
                p.canvas.set_global_alpha(0.35);
            }
            self.draw_control(&mut p, i);
            if dimmed {
                p.canvas.restore();
            }
        }

        if let Some(menu) = &self.menu {
            let accent = self.accent(&self.layout.controls[menu.control]);
            p.menu(menu.rect, &menu.items, menu.grid, menu.selected, menu.hover, accent);
        }
        canvas.restore();
    }
}
