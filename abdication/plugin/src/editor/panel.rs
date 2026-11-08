//! The whole front panel as one vizia view. It draws every plate and
//! control itself (see `draw`), hit-tests the mouse against the layout,
//! and changes parameters through nih-plug's gesture events, so the host
//! records automation and undo exactly as for its own controls.
//!
//! Sends: most knobs and faders take a send from each free modulator,
//! drawn as rings round a knob (Env inside, LFO outside) or bars beside a
//! fader (Env left, LFO right). Drag one up or down to set it (clockwise /
//! up is positive), double-click clears it, scroll nudges it.

use std::cell::RefCell;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use nih_plug::prelude::ParamPtr;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::util::ModifiersExt;
use nih_plug_vizia::widgets::RawParamEvent;

use super::draw::{fader_send_at, KnobGeometry, MenuGrid, Painter, Sends, Source, THUMB_W};
use super::layout::{send_ids, Action, Control, Kind, Layout, Rect, CHEEK, HEIGHT, WIDTH};
use super::scopes::Scopes;
use super::screens::{self, Live};
use crate::dice;
use crate::params::SynthParams;
use crate::scope::{LiveScope, ScopePitch};

/// Logical pixels of mouse travel for a knob's full range.
const KNOB_TRAVEL: f32 = 220.0;
/// Shift-drag moves this much slower.
const FINE: f32 = 0.12;
const MENU_ROW: f32 = 22.0;
const MENU_MAX_ROWS: usize = 24;
/// The panel's housekeeping tick (live scope, hover fix): about 15 Hz.
const TICK: Duration = Duration::from_millis(66);

/// Menus of library waves, which show each wave beside its name.
const WAVE_MENUS: [&str; 4] = ["wavetable.start", "wavetable.mid", "wavetable.end", "mod_lfo.shape"];
/// The wavetable's three waves, which its arrows move together.
const WAVETABLE_WAVES: [&str; 3] = ["wavetable.start", "wavetable.mid", "wavetable.end"];
/// How many patches the dice's back button remembers.
const DICE_HISTORY: usize = 32;

/// A send's depth runs -1..1, twice a knob's span: the same mouse travel
/// moves it as far round the knob as it would move the knob.
const SEND_TRAVEL: f32 = 2.0 * KNOB_TRAVEL;

/// What the mouse is on: a control, or one of its sends.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Target {
    control: usize,
    send: Option<Source>,
}

impl Target {
    fn control(control: usize) -> Self {
        Self { control, send: None }
    }
}

/// Sent to the panel by its ticker thread.
enum PanelEvent {
    Tick,
}

pub struct Panel {
    params: Arc<SynthParams>,
    layout: Layout,
    /// The parameter behind each control (and the Y parameter of the pad).
    ptrs: Vec<ParamPtr>,
    y_ptrs: Vec<Option<ParamPtr>>,
    /// Each control's sends [Env, LFO], where it has them.
    sends: Vec<[Option<ParamPtr>; 2]>,
    hover: Option<Target>,
    /// The momentary button under the mouse.
    hover_action: Option<usize>,
    /// The settings overlay is showing.
    settings_open: bool,
    /// What the top bar's readout shows.
    focus: Option<Target>,
    /// The kind of sound last rolled, for the readout until the mouse moves on.
    rolled: Option<&'static str>,
    /// The patch before each roll, newest last. Gone when the panel closes.
    dice_history: Vec<Vec<(ParamPtr, f32)>>,
    drag: Option<Drag>,
    menu: Option<Menu>,
    /// The scopes and the newest voice's moving values (polled by the
    /// ticker; offline traces re-rendered while drawing when settings change).
    scopes: RefCell<Scopes>,
    /// Keeps the ticker thread running; cleared when the panel goes away.
    ticking: Arc<AtomicBool>,
}

impl Drop for Panel {
    fn drop(&mut self) {
        self.ticking.store(false, Ordering::Relaxed);
    }
}

struct Drag {
    target: Target,
    /// Where the current relative drag is anchored (re-anchored when Shift
    /// changes so the value never jumps).
    anchor_y: f32,
    anchor_value: f32,
    fine: bool,
}

struct Menu {
    control: usize,
    /// Items are library waves (drawn beside their names).
    waves: bool,
    items: Vec<String>,
    rect: Rect,
    grid: MenuGrid,
    hover: Option<usize>,
}

impl Panel {
    pub fn new(cx: &mut Context, params: Arc<SynthParams>, pitch: Arc<ScopePitch>, live: Arc<LiveScope>)
        -> Handle<'_, Self> {
        let layout = Layout::build();
        let ptr = |id: &str| params.ptr(id).unwrap_or_else(|| panic!("no parameter '{id}'"));
        let ptrs = layout.controls.iter().map(|c| ptr(c.id)).collect();
        let y_ptrs = layout
            .controls
            .iter()
            .map(|c| match c.kind {
                Kind::XyPad { y } => Some(ptr(y)),
                _ => None,
            })
            .collect();
        let sends = layout
            .controls
            .iter()
            .map(|c| match c.kind {
                Kind::Knob { .. } | Kind::Fader => send_ids(c.id).map(|id| params.ptr(&id)),
                _ => [None, None],
            })
            .collect();
        let scopes = RefCell::new(Scopes::new(&params, pitch, live));
        let ticking = Arc::new(AtomicBool::new(true));
        let ticker = ticking.clone();
        Self { params, layout, ptrs, y_ptrs, sends, hover: None, hover_action: None, settings_open: false, focus: None, rolled: None, dice_history: Vec::new(),
               drag: None, menu: None, scopes, ticking }
            .build(cx, move |cx| {
                // vizia's timers never fire under baseview, but events sent
                // through a context proxy are delivered every frame. The
                // proxy targets the view being built: this panel.
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

    fn value(&self, i: usize) -> f32 {
        unsafe { self.ptrs[i].unmodulated_normalized_value() }
    }

    fn text(&self, i: usize) -> String {
        let p = self.ptrs[i];
        let value = unsafe { p.unmodulated_normalized_value() };
        if self.layout.controls[i].id == "mod_lfo.rate" && self.plain("mod_lfo.sync") > 0.5 {
            return screens::note_value(value).to_string();
        }
        unsafe { p.normalized_value_to_string(value, true) }
    }

    fn plain(&self, id: &str) -> f32 {
        self.params.ptr(id).map(|p| unsafe { p.unmodulated_plain_value() }).unwrap_or(0.0)
    }

    fn send_ptr(&self, i: usize, source: Source) -> Option<ParamPtr> {
        self.sends[i][source as usize]
    }

    /// The parameter a target sets: the control's, or its send's.
    fn target_ptr(&self, t: Target) -> ParamPtr {
        t.send.and_then(|s| self.send_ptr(t.control, s)).unwrap_or(self.ptrs[t.control])
    }

    /// A send's depth, -1..1 of its control's travel.
    fn depth(&self, i: usize, source: Source) -> Option<f32> {
        self.send_ptr(i, source).map(|p| unsafe { p.unmodulated_plain_value() })
    }

    /// Readout name and value for a target.
    fn describe(&self, t: Target) -> (String, String) {
        let p = self.target_ptr(t);
        let name = unsafe { p.name() }.to_string();
        match t.send {
            Some(source) => (name, format!("{:+.0} %", 100.0 * self.depth(t.control, source).unwrap_or(0.0))),
            None => (name, self.text(t.control)),
        }
    }

    fn steps(&self, i: usize) -> Option<usize> {
        unsafe { self.ptrs[i].step_count() }
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

    /// Step a menu-like parameter by one entry, wrapping around.
    fn step_param(cx: &mut EventContext, p: ParamPtr, up: bool) {
        let Some(steps) = unsafe { p.step_count() }.filter(|s| *s > 0) else { return };
        let index = (unsafe { p.unmodulated_normalized_value() } * steps as f32).round() as i64;
        let next = (index + if up { 1 } else { -1 }).rem_euclid(steps as i64 + 1);
        Self::change(cx, p, next as f32 / steps as f32);
    }

    fn step(&self, cx: &mut EventContext, i: usize, up: bool) {
        Self::step_param(cx, self.ptrs[i], up);
    }

    // ---- Geometry ----------------------------------------------------------------

    /// Panel scale and offset for the current bounds.
    fn transform(bounds: BoundingBox) -> (f32, f32, f32) {
        let s = (bounds.w / WIDTH).min(bounds.h / HEIGHT);
        (s, bounds.x, bounds.y)
    }

    fn to_panel(cx: &EventContext, x: f32, y: f32) -> (f32, f32) {
        let (s, ox, oy) = Self::transform(cx.bounds());
        ((x - ox) / s, (y - oy) / s)
    }

    /// The send under (x, y), if any (they ring the controls, so they're
    /// checked first).
    fn send_at(&self, x: f32, y: f32) -> Option<Target> {
        self.layer().find_map(|(i, c)| {
            let source = match c.kind {
                Kind::Knob { size, inline, .. } => KnobGeometry::of(c.rect, size, inline).ring_at(x, y),
                Kind::Fader => fader_send_at(c.rect, x, y),
                _ => None,
            }?;
            self.send_ptr(i, source).map(|_| Target { control: i, send: Some(source) })
        })
    }

    fn hit(&self, x: f32, y: f32) -> Option<Target> {
        self.send_at(x, y)
            .or_else(|| self.layer().find(|(_, c)| c.rect.contains(x, y)).map(|(i, _)| Target::control(i)))
    }

    /// The controls that can be hit now: the overlay's while it's open,
    /// otherwise the panel's.
    fn layer(&self) -> impl Iterator<Item = (usize, &Control)> {
        let range = self.layout.range(self.settings_open);
        self.layout.controls[range.clone()].iter().enumerate().map(move |(k, c)| (range.start + k, c))
    }

    fn open_menu(&mut self, i: usize) {
        let Some(steps) = self.steps(i) else { return };
        let p = self.ptrs[i];
        let items: Vec<String> = (0..=steps)
            .map(|k| unsafe { p.normalized_value_to_string(k as f32 / steps as f32, false) })
            .collect();
        let rows = items.len().min(MENU_MAX_ROWS);
        let cols = items.len().div_ceil(rows);
        let longest = items.iter().map(|s| s.chars().count()).max().unwrap_or(4) as f32;
        let waves = WAVE_MENUS.contains(&self.layout.controls[i].id);
        let col_w = (longest * 7.4 + 22.0).max(64.0) + if waves { THUMB_W + 6.0 } else { 0.0 };
        let control = self.layout.controls[i].rect;
        let (w, h) = ((col_w * cols as f32 + 12.0).max(control.w), rows as f32 * MENU_ROW + 12.0);
        let x = (control.x + control.w / 2.0 - w / 2.0).clamp(CHEEK + 6.0, WIDTH - CHEEK - 6.0 - w);
        let below = control.bottom() + 4.0;
        let y = if below + h <= HEIGHT - 8.0 {
            below
        } else if control.y - 4.0 - h >= 8.0 {
            control.y - 4.0 - h
        } else {
            ((HEIGHT - h) / 2.0).max(8.0)
        };
        let grid = MenuGrid { rows, col_w, row_h: MENU_ROW };
        self.menu = Some(Menu { control: i, waves, items, rect: Rect::new(x, y, w, h), grid, hover: None });
    }

    fn menu_item_at(menu: &Menu, x: f32, y: f32) -> Option<usize> {
        (0..menu.items.len())
            .find(|&k| menu.grid.cell(menu.rect, k).contains(x, y))
    }

    // ---- Mouse -------------------------------------------------------------------

    /// Works around a vizia bug: when the pointer goes past the window's
    /// right or bottom edge (during a drag, say), vizia marks the window as
    /// "not under the pointer" and only undoes that when the platform
    /// reports the cursor re-entering, which baseview on Windows never
    /// does. Until then every click goes to the window instead of the
    /// panel. So, whenever the pointer is inside the window, say it entered.
    fn keep_hover_alive(cx: &mut EventContext) {
        let bounds = cx.bounds();
        let (x, y) = (cx.mouse().cursorx, cx.mouse().cursory);
        if x >= bounds.x && y >= bounds.y && x < bounds.x + bounds.w && y < bounds.y + bounds.h {
            cx.emit_custom(Event::new(WindowEvent::MouseEnter).origin(Entity::root()).direct(Entity::root()));
        }
    }

    fn mouse_down(&mut self, cx: &mut EventContext, x: f32, y: f32) {
        // A drag whose release never arrived (e.g. let go outside the
        // window): finish it before starting anything new.
        self.mouse_up(cx);
        if let Some(menu) = self.menu.take() {
            if let Some(k) = Self::menu_item_at(&menu, x, y) {
                let steps = self.steps(menu.control).unwrap_or(1).max(1);
                Self::change(cx, self.ptrs[menu.control], k as f32 / steps as f32);
                self.focus = Some(Target::control(menu.control));
            }
            return;
        }
        if let Some(k) = self.action_at(x, y) {
            self.act(cx, self.layout.actions[k].action);
            return;
        }
        // A click outside the open settings closes them.
        if self.settings_open && !self.layout.settings.contains(x, y) {
            self.close_settings();
            return;
        }
        let Some(target) = self.hit(x, y) else { return };
        self.focus = Some(target);
        let i = target.control;
        let control = self.layout.controls[i];
        let p = self.target_ptr(target);
        if target.send.is_some() {
            Self::begin(cx, p);
            let anchor_value = unsafe { p.unmodulated_normalized_value() };
            self.drag = Some(Drag { target, anchor_y: y, anchor_value, fine: cx.modifiers().shift() });
            cx.capture();
            return;
        }
        match control.kind {
            Kind::Knob { .. } | Kind::Fader => {
                Self::begin(cx, p);
                self.drag = Some(Drag { target, anchor_y: y, anchor_value: self.value(i), fine: cx.modifiers().shift() });
                cx.capture();
            }
            Kind::XyPad { .. } => {
                Self::begin(cx, p);
                Self::begin(cx, self.y_ptrs[i].unwrap());
                self.drag = Some(Drag { target, anchor_y: y, anchor_value: 0.0, fine: false });
                self.set_pad(cx, i, x, y);
                cx.capture();
            }
            // A toggle with more than two settings (Unison: Off, U5, U7) cycles.
            Kind::Toggle if self.steps(i).is_some_and(|s| s > 1) => self.step(cx, i, true),
            Kind::Toggle => Self::change(cx, p, if self.value(i) > 0.5 { 0.0 } else { 1.0 }),
            Kind::Stepper => {
                if x < control.rect.x + 24.0 {
                    self.step(cx, i, false);
                } else if x > control.rect.right() - 24.0 {
                    self.step(cx, i, true);
                } else {
                    self.open_menu(i);
                }
            }
            Kind::Dropdown => self.open_menu(i),
        }
    }

    /// One control, dimmed if it does nothing with the current settings
    /// (still adjustable, ready for when it applies).
    fn draw_control(&self, p: &mut Painter, i: usize) {
        let c = &self.layout.controls[i];
        let dimmed = !applies(&self.params, c.id);
        if dimmed {
            p.canvas.save();
            p.canvas.set_global_alpha(0.32);
        }
        let active = self.drag.as_ref().map(|d| d.target).or(self.hover).filter(|t| t.control == i);
        let hot = active.is_some_and(|t| t.send.is_none());
        let sends = Sends { env: self.depth(i, Source::Env), lfo: self.depth(i, Source::Lfo),
                            hot: active.and_then(|t| t.send) };
        let v = self.value(i);
        match c.kind {
            Kind::Knob { size, bipolar, inline } => {
                p.knob(c.rect, size, v, bipolar, inline, c.label, &self.text(i), hot, sends)
            }
            Kind::Fader => p.fader(c.rect, v, c.label, &self.text(i), hot, sends),
            Kind::Toggle if self.steps(i).is_some_and(|s| s > 1) => {
                // Lit and showing its setting when on; its own label when off.
                let on = v > 0.0;
                let label = if on { self.text(i) } else { c.label.to_string() };
                p.toggle(c.rect, on, &label, hot)
            }
            Kind::Toggle => p.toggle(c.rect, v > 0.5, c.label, hot),
            Kind::Stepper => p.stepper(c.rect, &self.text(i), hot),
            Kind::Dropdown => p.dropdown(c.rect, &self.text(i), hot),
            Kind::XyPad { .. } => {
                let y = unsafe { self.y_ptrs[i].unwrap().unmodulated_normalized_value() };
                p.xy_pad(c.rect, v, y);
            }
        }
        if dimmed {
            p.canvas.restore();
        }
    }

    /// The button under the pointer (only ☰ while the settings are open).
    fn action_at(&self, x: f32, y: f32) -> Option<usize> {
        self.layout.actions.iter().position(|b| {
            b.rect.contains(x, y) && (!self.settings_open || b.action == Action::Settings)
        })
    }

    fn close_settings(&mut self) {
        self.settings_open = false;
        self.hover = None;
    }

    fn act(&mut self, cx: &mut EventContext, action: Action) {
        match action {
            Action::CopyFilter | Action::CopyAmp => self.copy_envelope(cx, action),
            Action::ShiftWaves { up } => {
                for p in WAVETABLE_WAVES.iter().filter_map(|id| self.params.ptr(id)) {
                    Self::step_param(cx, p, up);
                }
            }
            Action::Dice => self.roll_dice(cx),
            Action::DiceBack => self.dice_back(cx),
            Action::Settings if self.settings_open => self.close_settings(),
            Action::Settings => {
                self.settings_open = true;
                self.hover = None;
            }
        }
    }

    /// Roll a new patch, remembering the one it replaces.
    fn roll_dice(&mut self, cx: &mut EventContext) {
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(1, |t| t.as_nanos() as u64);
        let patch = dice::roll(seed, self.params.ids());
        let targets: Vec<(ParamPtr, f32)> = patch.values.iter()
            .filter_map(|(id, value)| self.params.ptr(id).map(|p| (p, *value)))
            .collect();
        if self.dice_history.len() == DICE_HISTORY {
            self.dice_history.remove(0);
        }
        self.dice_history.push(targets.iter().map(|(p, _)| (*p, unsafe { p.unmodulated_normalized_value() })).collect());
        for (p, value) in targets {
            Self::change(cx, p, unsafe { p.preview_normalized(value) });
        }
        self.focus = None;
        self.rolled = Some(patch.kind.name());
    }

    fn dice_back(&mut self, cx: &mut EventContext) {
        if let Some(patch) = self.dice_history.pop() {
            for (p, value) in patch {
                Self::change(cx, p, value);
            }
            self.focus = None;
            self.rolled = Some("Previous patch");
        }
    }

    /// Copy the Envelope's filter or amp shape into the Mod Env (the amp's
    /// with the pad's offsets, as the voice plays it).
    fn copy_envelope(&mut self, cx: &mut EventContext, action: Action) {
        let v = |id: &str| self.plain(&format!("envelope.{id}"));
        let (decay, sustain) = (v("decay"), v("sustain"));
        let (decay, sustain) = match action {
            Action::CopyFilter => (decay, sustain),
            _ => (decay * (10.0 / decay.max(1e-4)).powf(v("amp_decay")),
                  sustain + v("amp_sustain") * (1.0 - sustain)),
        };
        let shape = [("attack", v("attack")), ("decay", decay), ("sustain", sustain), ("release", v("release")),
                     ("curve", v("curve"))];
        for (id, value) in shape {
            if let Some(p) = self.params.ptr(&format!("mod_env.{id}")) {
                Self::change(cx, p, unsafe { p.preview_normalized(value) });
            }
        }
    }

    fn set_pad(&self, cx: &mut EventContext, i: usize, x: f32, y: f32) {
        let r = self.layout.controls[i].rect.inset(7.0, 7.0, 7.0, 7.0);
        Self::set(cx, self.ptrs[i], (x - r.x) / r.w);
        Self::set(cx, self.y_ptrs[i].unwrap(), (r.bottom() - y) / r.h);
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
            // The button is up but we never heard the release.
            self.mouse_up(cx);
        }
        if let Some(target) = self.drag.as_ref().map(|d| d.target) {
            let i = target.control;
            let control: Control = self.layout.controls[i];
            if let (Kind::XyPad { .. }, None) = (control.kind, target.send) {
                self.set_pad(cx, i, x, y);
                return;
            }
            let p = self.target_ptr(target);
            let fine = cx.modifiers().shift();
            let drag = self.drag.as_mut().unwrap();
            if fine != drag.fine {
                // Re-anchor so switching speed mid-drag doesn't jump.
                drag.anchor_value = unsafe { p.unmodulated_normalized_value() };
                drag.anchor_y = y;
                drag.fine = fine;
            }
            let travel = match (control.kind, target.send) {
                (_, Some(_)) => SEND_TRAVEL,
                (Kind::Fader, None) => Painter::fader_track(control.rect).h - 16.0,
                _ => KNOB_TRAVEL,
            };
            let speed = if fine { FINE } else { 1.0 };
            Self::set(cx, p, drag.anchor_value + (drag.anchor_y - y) / travel * speed);
            cx.needs_redraw();
            return;
        }
        let hover_action = self.action_at(x, y);
        if hover_action != self.hover_action {
            self.hover_action = hover_action;
            cx.needs_redraw();
        }
        let hover = self.hit(x, y);
        if hover != self.hover {
            self.hover = hover;
            if hover.is_some() {
                self.focus = hover;
            }
            cx.needs_redraw();
        }
    }

    fn mouse_up(&mut self, cx: &mut EventContext) {
        if let Some(drag) = self.drag.take() {
            Self::end(cx, self.target_ptr(drag.target));
            if let (Some(p), None) = (self.y_ptrs[drag.target.control], drag.target.send) {
                Self::end(cx, p);
            }
            cx.release();
        }
    }

    /// Knobs, faders, sends and the pad reset on a double-click; everything
    /// else is a button.
    fn resets(&self, t: Target) -> bool {
        t.send.is_some()
            || matches!(self.layout.controls[t.control].kind, Kind::Knob { .. } | Kind::Fader | Kind::XyPad { .. })
    }

    /// vizia delivers a quick second (and third) click only as a double
    /// (triple) click. On a knob, fader or the pad a double-click restores
    /// the default; anywhere else repeated clicks are ordinary presses, so
    /// clicking an arrow quickly steps every time.
    fn repeat_click(&mut self, cx: &mut EventContext, x: f32, y: f32, double: bool) {
        match self.hit(x, y) {
            Some(t) if self.menu.is_none() && self.resets(t) => {
                if double {
                    self.reset(cx, t);
                }
            }
            _ => self.mouse_down(cx, x, y),
        }
    }

    fn reset(&mut self, cx: &mut EventContext, t: Target) {
        let pad_y = if t.send.is_none() { self.y_ptrs[t.control] } else { None };
        for p in [Some(self.target_ptr(t)), pad_y].into_iter().flatten() {
            Self::change(cx, p, unsafe { p.default_normalized_value() });
        }
        self.focus = Some(t);
    }

    fn scroll(&mut self, cx: &mut EventContext, x: f32, y: f32, dy: f32) {
        if self.menu.is_some() || self.drag.is_some() || dy == 0.0 {
            return;
        }
        let Some(target) = self.hit(x, y) else { return };
        let up = dy > 0.0;
        if target.send.is_some() {
            let p = self.target_ptr(target);
            let notch = if cx.modifiers().shift() { 0.001 } else { 0.005 };
            let v = unsafe { p.unmodulated_normalized_value() };
            Self::change(cx, p, v + if up { notch } else { -notch });
            self.focus = Some(target);
            return;
        }
        let i = target.control;
        let p = self.ptrs[i];
        match self.layout.controls[i].kind {
            Kind::Stepper | Kind::Dropdown => self.step(cx, i, up),
            Kind::Knob { .. } | Kind::Fader => {
                let v = self.value(i);
                let next = match self.steps(i) {
                    Some(_) => unsafe {
                        if up { p.next_normalized_step(v, false) } else { p.previous_normalized_step(v, false) }
                    },
                    None => {
                        let notch = if cx.modifiers().shift() { 0.002 } else { 0.01 };
                        v + if up { notch } else { -notch }
                    }
                };
                Self::change(cx, p, next);
            }
            _ => {}
        }
    }
}

/// Whether a control does anything with the current settings.
fn applies(params: &SynthParams, id: &str) -> bool {
    let value = |id: &str| params.ptr(id).map(|p| unsafe { p.unmodulated_plain_value() }).unwrap_or(0.0);
    match id {
        "fundamental.pulse_width" | "fundamental.pw_lfo_rate" | "fundamental.pw_lfo_depth" => {
            value("fundamental.wave") == 3.0 // PWM
        }
        "wavetable.oscmod_depth" => value("wavetable.oscmod") != 0.0,
        "noise.bits" => value("noise.type") == 1.0,                     // Clocked
        "noise.clock_ratio" => (1.0..=3.0).contains(&value("noise.type")), // Clocked, LFSR
        "noise.crackle_density" => value("noise.type") == 4.0,          // Crackle
        "unison.detune" => value("unison.mode") > 0.5,
        "settings.vintage_noise" => value("vintage.amount") > 0.0,
        "transient.sample_pitch" => value("transient.type") >= 5.0,     // the recorded samples
        _ => true,
    }
}

impl View for Panel {
    fn element(&self) -> Option<&'static str> {
        Some("abdication-panel")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|e: &PanelEvent, _| match e {
            PanelEvent::Tick => {
                Self::keep_hover_alive(cx);
                if self.scopes.borrow_mut().tick(&self.params) {
                    cx.needs_redraw();
                }
            }
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
                // Mouse moves redraw only when something visible changed.
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
                    }
                }
                WindowEvent::MouseScroll(_, dy) => {
                    self.scroll(cx, x, y, dy);
                    meta.consume();
                }
                WindowEvent::KeyDown(Code::Escape, _) => {
                    if self.menu.take().is_none() {
                        self.close_settings();
                    }
                }
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

        p.background(WIDTH, HEIGHT);
        p.cheek(Rect::new(0.0, 0.0, CHEEK, HEIGHT));
        p.faceplate(Rect::new(CHEEK, 0.0, WIDTH - 2.0 * CHEEK, HEIGHT));
        p.canvas.save();
        p.canvas.translate(WIDTH, 0.0);
        p.canvas.scale(-1.0, 1.0);
        p.cheek(Rect::new(0.0, 0.0, CHEEK, HEIGHT));
        p.canvas.restore();

        p.raised(self.layout.top_bar, 10.0, 0x34363b, 0x26282c);
        let (name, value) = match (self.focus, self.rolled) {
            (Some(t), _) => self.describe(t),
            (None, Some(rolled)) => ("Dice".to_string(), rolled.to_string()),
            _ => Default::default(),
        };
        p.readout(self.layout.readout, &name, &value);

        for plate in &self.layout.plates {
            p.plate(plate.rect, plate.title);
        }
        let live = Live::from(&mut self.scopes.borrow_mut(), &self.params);
        for (screen, rect) in &self.layout.screens {
            p.screen(*rect);
            screens::draw(&mut p, &self.params, &live, *screen, *rect);
        }
        for caption in &self.layout.captions {
            use super::draw::{Text, MUTED};
            p.text(caption.x, caption.y, caption.text, Text::label(11.0, MUTED).left());
        }

        for i in self.layout.range(false) {
            self.draw_control(&mut p, i);
        }

        for (k, b) in self.layout.actions.iter().enumerate() {
            let hot = self.hover_action == Some(k);
            match b.action {
                Action::Dice => p.dice(b.rect, hot),
                Action::Settings => p.hamburger(b.rect, hot, self.settings_open),
                _ => p.button(b.rect, b.label, hot),
            }
        }

        if self.settings_open {
            p.plate(self.layout.settings, "Settings");
            for i in self.layout.range(true) {
                self.draw_control(&mut p, i);
            }
        }

        if let Some(menu) = &self.menu {
            let selected = (self.value(menu.control) * self.steps(menu.control).unwrap_or(1) as f32).round() as usize;
            let wave = |item: usize, t: f32| screens::wave_outline(item, t) * 0.9;
            let thumb: Option<&dyn Fn(usize, f32) -> f32> = if menu.waves { Some(&wave) } else { None };
            p.menu(menu.rect, &menu.items, menu.grid, selected, menu.hover, thumb);
        }
        canvas.restore();
    }
}
