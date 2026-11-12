//! The whole front panel as one vizia view. It draws every plate and
//! control itself (see `draw`), hit-tests the mouse against the layout,
//! and changes parameters through nih-plug's gesture events, so the host
//! records automation and undo exactly as for its own controls.
//!
//! Modulation: clicking a modulatable knob or mix fader selects it (a ring
//! of dots marks it), and the Modulations panel then shows and sets its
//! modulators; the Amp panel lists every modulation that's set (click a
//! line to select its knob). While a note plays, a modulated control shows
//! where the newest voice's modulators have it.

use std::cell::RefCell;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use nih_plug::prelude::ParamPtr;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::util::ModifiersExt;
use nih_plug_vizia::widgets::RawParamEvent;

use super::draw::{Marks, MenuGrid, Painter, Text, AMBER, AMBER_DIM, LABEL, THUMB_W, VALUE_TEXT};
use super::layout::{selected_mod_field, Action, Control, Kind, Layout, Rect, Screen, Show, CHEEK, HEIGHT, SLOT_MODES, RAY_CHAOS,
                    WIDTH};
use super::scopes::Scopes;
use super::screens::{self, Live};
use crate::dice;
use crate::modulation::{self, TARGETS};
use crate::params::{SynthParams, MOD_FIELDS};
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
const WAVE_MENUS: [&str; 8] = ["wt_1.wave_1", "wt_1.wave_2", "wt_1.wave_3", "wt_1.wave_4",
                               "wt_2.wave_1", "wt_2.wave_2", "wt_2.wave_3", "wt_2.wave_4"];
/// The wavetable's three waves, which its arrows move together.
const WAVETABLE_WAVES: [[&str; 4]; 2] = [["wt_1.wave_1", "wt_1.wave_2", "wt_1.wave_3", "wt_1.wave_4"],
                                         ["wt_2.wave_1", "wt_2.wave_2", "wt_2.wave_3", "wt_2.wave_4"]];
/// How many patches the dice's back button remembers.
const DICE_HISTORY: usize = 32;

/// The knob the Modulations panel starts on.
const FIRST_SELECTED: usize = modulation::CUTOFF;
/// The modulation amounts, which Clear zeroes.
const AMOUNTS: [&str; 6] = ["lfo", "env", "vel", "touch", "wheel", "key"];
/// Lines of the list of modulations.
const LIST_LINE: f32 = 13.0;

/// Sent to the panel by its ticker thread.
enum PanelEvent {
    Tick,
}

pub struct Panel {
    params: Arc<SynthParams>,
    layout: Layout,
    /// The parameter behind each control (a Modulations panel control's is
    /// the selected knob's: see `ptr`).
    ptrs: Vec<ParamPtr>,
    /// For each control: the modulatable knob it is, if it is one.
    knob_target: Vec<Option<usize>>,
    /// For each control: the modulator field it sets (MOD_FIELDS index), if
    /// it's on the Modulations panel.
    mod_field: Vec<Option<usize>>,
    /// Every modulatable knob's modulator parameters, in MOD_FIELDS order.
    mod_ptrs: Vec<Vec<ParamPtr>>,
    /// The knob the Modulations panel shows (modulation::TARGETS index).
    selected: usize,
    /// Copied modulator settings (normalized, in MOD_FIELDS order), for Paste.
    clipboard: Option<Vec<f32>>,
    hover: Option<usize>,
    /// The momentary button under the mouse.
    hover_action: Option<usize>,
    /// The settings overlay is showing.
    settings_open: bool,
    /// What the top bar's readout shows.
    focus: Option<usize>,
    /// A message for the readout until the mouse moves on (the kind of sound
    /// last rolled, say): its name and value.
    rolled: Option<(&'static str, &'static str)>,
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
    target: usize,
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
        let mod_ptrs: Vec<Vec<ParamPtr>> = params.mods.iter()
            .map(|m| MOD_FIELDS.iter().map(|f| m.ptr(f).unwrap()).collect())
            .collect();
        let mod_field: Vec<Option<usize>> = layout.controls.iter()
            .map(|c| selected_mod_field(c.id).map(|f| MOD_FIELDS.iter().position(|m| *m == f).expect("a modulator field")))
            .collect();
        // A Modulations panel control starts out as the first knob's.
        let ptrs = layout.controls.iter().zip(&mod_field)
            .map(|(c, f)| match f {
                Some(f) => mod_ptrs[FIRST_SELECTED][*f],
                None => ptr(c.id),
            })
            .collect();
        let knob_target = layout.controls.iter().map(|c| modulation::target_index(c.id)).collect();
        let scopes = RefCell::new(Scopes::new(&params, pitch, live));
        let ticking = Arc::new(AtomicBool::new(true));
        let ticker = ticking.clone();
        Self { params, layout, ptrs, knob_target, mod_field, mod_ptrs, selected: FIRST_SELECTED, clipboard: None, hover: None,
               hover_action: None, settings_open: false, focus: None, rolled: None, dice_history: Vec::new(),
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

    /// The parameter control `i` sets. A Modulations panel control sets the
    /// selected knob's; its Rate sets the division while the LFO is synced.
    fn ptr(&self, i: usize) -> ParamPtr {
        match self.mod_field[i] {
            Some(f) => {
                let fields = &self.mod_ptrs[self.selected];
                let synced = unsafe { fields[2].unmodulated_plain_value() } > 0.5;
                let f = if MOD_FIELDS[f] == "rate" && synced { 3 } else { f };
                fields[f]
            }
            None => self.ptrs[i],
        }
    }

    fn value(&self, i: usize) -> f32 {
        unsafe { self.ptr(i).unmodulated_normalized_value() }
    }

    fn text(&self, i: usize) -> String {
        let p = self.ptr(i);
        unsafe { p.normalized_value_to_string(p.unmodulated_normalized_value(), true) }
    }

    /// The selected knob's modulator field `field` (MOD_FIELDS).
    fn selected_mod(&self, field: &str) -> ParamPtr {
        self.mod_ptrs[self.selected][MOD_FIELDS.iter().position(|f| *f == field).unwrap()]
    }

    /// Readout name and value for a control.
    fn describe(&self, i: usize) -> (String, String) {
        (unsafe { self.ptr(i).name() }.to_string(), self.text(i))
    }

    fn steps(&self, i: usize) -> Option<usize> {
        unsafe { self.ptr(i).step_count() }
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
        Self::step_param(cx, self.ptr(i), up);
        self.picked(cx, i);
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

    /// The control under (x, y): a curve button before the knob it sits on.
    fn hit(&self, x: f32, y: f32) -> Option<usize> {
        let under = |(_, c): &(usize, &Control)| c.rect.contains(x, y) && self.shows(c.show);
        let curve = |(_, c): &(usize, &Control)| matches!(c.kind, Kind::Curve { .. });
        self.layer().filter(under).find(curve).or_else(|| self.layer().find(under)).map(|(i, _)| i)
    }

    /// Every modulation that's set, one line each ("filter env 21%"), with
    /// its knob.
    fn mod_lines(&self) -> Vec<(usize, String)> {
        self.params.mod_settings().enumerate()
            .flat_map(|(t, s)| s.amounts().map(move |(source, a)| {
                (t, format!("{} {source} {}", TARGETS[t].short, modulation::percent_text(a)))
            }).collect::<Vec<_>>())
            .collect()
    }

    /// The knob of the list line under (x, y), if any.
    fn mod_line_at(&self, x: f32, y: f32) -> Option<usize> {
        let (_, r, _) = self.layout.screens.iter().find(|(s, _, _)| *s == Screen::ModList)?;
        let area = r.inset(8.0, 8.0, 8.0, 8.0);
        if !area.contains(x, y) {
            return None;
        }
        let k = ((y - area.y) / LIST_LINE) as usize;
        self.mod_lines().get(k).map(|(t, _)| *t)
    }

    /// The controls that can be hit now: the overlay's while it's open,
    /// otherwise the panel's.
    fn layer(&self) -> impl Iterator<Item = (usize, &Control)> {
        let range = self.layout.range(self.settings_open);
        self.layout.controls[range.clone()].iter().enumerate().map(move |(k, c)| (range.start + k, c))
            .filter(|(_, c)| self.shows(c.show))
    }

    /// Whether something showing under `show` is on the panel now: always,
    /// or while its oscillator slot is in that mode.
    fn shows(&self, show: Show) -> bool {
        match show {
            None => true,
            Some((slot, mode)) => self.slot_mode(slot) == mode,
        }
    }

    /// Oscillator slot `slot`'s mode (SLOT_DUAL, SLOT_WAVETABLE or SLOT_RAY).
    fn slot_mode(&self, slot: usize) -> usize {
        self.params.ptr(SLOT_MODES[slot])
            .map_or(0, |p| unsafe { p.unmodulated_plain_value() } as usize)
    }

    fn open_menu(&mut self, i: usize) {
        let Some(steps) = self.steps(i) else { return };
        let p = self.ptr(i);
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
                Self::change(cx, self.ptr(menu.control), k as f32 / steps as f32);
                self.picked(cx, menu.control);
                self.focus = Some(menu.control);
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
        if !self.settings_open {
            if let Some(t) = self.mod_line_at(x, y) {
                self.selected = t;
                return;
            }
        }
        let Some(i) = self.hit(x, y) else { return };
        self.focus = Some(i);
        // A modulatable knob or fader becomes the Modulations panel's.
        if let Some(t) = self.knob_target[i] {
            self.selected = t;
        }
        let control = self.layout.controls[i];
        let p = self.ptr(i);
        match control.kind {
            Kind::Knob { .. } | Kind::Fader | Kind::Curve { .. } => {
                Self::begin(cx, p);
                self.drag = Some(Drag { target: i, anchor_y: y, anchor_value: self.value(i), fine: cx.modifiers().shift() });
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
        let hot = self.drag.as_ref().map(|d| d.target).or(self.hover) == Some(i);
        let marks = match self.knob_target[i] {
            Some(t) => Marks {
                selected: t == self.selected,
                moved: self.scopes.borrow().moved(t).filter(|_| self.params.mods[t].settings().active()),
            },
            None => Marks::default(),
        };
        let v = self.value(i);
        match c.kind {
            Kind::Knob { size, bipolar, inline } => {
                p.knob(c.rect, size, v, bipolar, inline, c.label, &self.text(i), hot, marks)
            }
            Kind::Fader => p.fader(c.rect, v, c.label, &self.text(i), hot, marks),
            Kind::Toggle if self.steps(i).is_some_and(|s| s > 1) => {
                // Lit and showing its setting when on; its own label when off.
                let on = v > 0.0;
                let label = if on { self.text(i) } else { c.label.to_string() };
                p.toggle(c.rect, on, &label, hot)
            }
            Kind::Toggle => p.toggle(c.rect, v > 0.5, c.label, hot),
            Kind::Stepper => p.stepper(c.rect, &self.text(i), hot),
            Kind::Dropdown => p.dropdown(c.rect, &self.text(i), hot),
            Kind::Curve { rising } => p.curve_button(c.rect, &self.bow_points(i, rising, 10), hot),
        }
        if dimmed {
            p.canvas.restore();
        }
    }

    /// After a menu pick: picking Chaos (a Ray's element) rolls a new shape,
    /// every time.
    fn picked(&self, cx: &mut EventContext, i: usize) {
        let id = self.layout.controls[i].id;
        let Some(slot) = id.strip_suffix("element") else { return };
        if unsafe { self.ptr(i).unmodulated_plain_value() } != RAY_CHAOS {
            return;
        }
        if let Some(seed) = self.params.ptr(&format!("{slot}chaos_seed")) {
            let nanos = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
                .map_or(1, |d| d.subsec_nanos() ^ d.as_secs() as u32);
            let roll = (nanos.wrapping_mul(0x9E37_79B1) >> 8) % 99_999 + 1;
            Self::change(cx, seed, unsafe { seed.preview_normalized(roll as f32) });
        }
    }

    /// `points` + 1 points along curve control `i`'s segment (0..1 each way).
    fn bow_points(&self, i: usize, rising: bool, points: usize) -> Vec<(f32, f32)> {
        let bow = unsafe { self.ptr(i).unmodulated_plain_value() };
        let (from, to) = if rising { (0.0, 1.0) } else { (1.0, 0.0) };
        (0..=points).map(|k| k as f32 / points as f32).map(|f| (f, modulation::along(from, to, f, bow))).collect()
    }

    /// While a curve button is dragged: its curve, large, beside it.
    fn draw_curve_popup(&self, p: &mut Painter) {
        let Some(i) = self.drag.as_ref().map(|d| d.target) else { return };
        let c = &self.layout.controls[i];
        let Kind::Curve { rising } = c.kind else { return };
        let name = unsafe { self.ptr(i).name() }.to_string();
        p.curve_popup(c.rect, &name, &self.text(i), &self.bow_points(i, rising, 60), rising);
    }

    /// The button under the pointer (only ☰ while the settings are open;
    /// never a greyed one).
    fn action_at(&self, x: f32, y: f32) -> Option<usize> {
        self.layout.actions.iter().position(|b| {
            b.rect.contains(x, y) && (!self.settings_open || b.action == Action::Settings) && self.enabled(b.action)
                && self.shows(b.show)
        })
    }

    /// Paste is greyed until something's been copied.
    fn enabled(&self, action: Action) -> bool {
        action != Action::PasteMods || self.clipboard.is_some()
    }

    fn close_settings(&mut self) {
        self.settings_open = false;
        self.hover = None;
    }

    fn act(&mut self, cx: &mut EventContext, action: Action) {
        match action {
            Action::CopyMods => {
                let fields = &self.mod_ptrs[self.selected];
                self.clipboard = Some(fields.iter().map(|p| unsafe { p.unmodulated_normalized_value() }).collect());
                self.rolled = Some(("Modulators", "copied"));
                self.focus = None;
            }
            Action::PasteMods => {
                if let Some(values) = &self.clipboard {
                    for (p, v) in self.mod_ptrs[self.selected].iter().zip(values) {
                        Self::change(cx, *p, *v);
                    }
                }
            }
            Action::ClearMods => {
                for field in AMOUNTS {
                    let p = self.selected_mod(field);
                    Self::change(cx, p, unsafe { p.preview_normalized(0.0) });
                }
            }
            Action::ShiftWaves { slot, up } => {
                for p in WAVETABLE_WAVES[slot].iter().filter_map(|id| self.params.ptr(id)) {
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
        self.rolled = Some(("Dice", patch.kind.name()));
    }

    fn dice_back(&mut self, cx: &mut EventContext) {
        if let Some(patch) = self.dice_history.pop() {
            for (p, value) in patch {
                Self::change(cx, p, value);
            }
            self.focus = None;
            self.rolled = Some(("Dice", "Previous patch"));
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
            // The button is up but we never heard the release.
            self.mouse_up(cx);
        }
        if let Some(i) = self.drag.as_ref().map(|d| d.target) {
            let control: Control = self.layout.controls[i];
            let p = self.ptr(i);
            let fine = cx.modifiers().shift();
            let drag = self.drag.as_mut().unwrap();
            if fine != drag.fine {
                // Re-anchor so switching speed mid-drag doesn't jump.
                drag.anchor_value = unsafe { p.unmodulated_normalized_value() };
                drag.anchor_y = y;
                drag.fine = fine;
            }
            let travel = match control.kind {
                Kind::Fader => Painter::fader_track(control.rect).h - 16.0,
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
            Self::end(cx, self.ptr(drag.target));
            cx.release();
        }
    }

    /// Knobs and faders reset on a double-click; everything else is a button.
    fn resets(&self, i: usize) -> bool {
        matches!(self.layout.controls[i].kind, Kind::Knob { .. } | Kind::Fader | Kind::Curve { .. })
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

    fn reset(&mut self, cx: &mut EventContext, i: usize) {
        let p = self.ptr(i);
        Self::change(cx, p, unsafe { p.default_normalized_value() });
        self.focus = Some(i);
    }

    fn scroll(&mut self, cx: &mut EventContext, x: f32, y: f32, dy: f32) {
        if self.menu.is_some() || self.drag.is_some() || dy == 0.0 {
            return;
        }
        let Some(i) = self.hit(x, y) else { return };
        let up = dy > 0.0;
        let p = self.ptr(i);
        match self.layout.controls[i].kind {
            Kind::Stepper | Kind::Dropdown => self.step(cx, i, up),
            Kind::Knob { .. } | Kind::Fader | Kind::Curve { .. } => {
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

impl Panel {
    /// The Modulations panel's header (the knob it shows) and the list of
    /// every modulation that's set.
    fn draw_modulations(&self, p: &mut Painter) {
        let t = self.selected;
        let name = unsafe { self.params.ptr(TARGETS[t].id).map(|q| q.name().to_string()) }.unwrap_or_default();
        let r = self.layout.mod_title;
        p.text(r.x, r.y + 13.0, &name, Text::label(13.0, AMBER).left());
        let settings = self.params.mods[t].settings();
        let summary: Vec<String> = settings.amounts().map(|(s, a)| format!("{s} {}", modulation::percent_text(a))).collect();
        let summary = if summary.is_empty() { "nothing moves it".to_string() } else { summary.join("  ") };
        let x = r.x + p.text_width(&name, Text::label(13.0, AMBER)) + 18.0;
        p.text(x, r.y + 13.0, &summary, Text::mono(10.0, AMBER_DIM).left());

        let Some((_, list, _)) = self.layout.screens.iter().find(|(s, _, _)| *s == Screen::ModList) else { return };
        let area = list.inset(8.0, 8.0, 8.0, 8.0);
        let lines = self.mod_lines();
        let fit = (area.h / LIST_LINE) as usize;
        for (k, (target, line)) in lines.iter().enumerate().take(fit) {
            let last = k + 1 == fit && lines.len() > fit;
            let text = if last { format!("+ {} more", lines.len() - k) } else { line.clone() };
            let color = if *target == t { VALUE_TEXT } else { LABEL };
            p.text(area.x, area.y + LIST_LINE * (k as f32 + 0.5), &text, Text::mono(10.0, color).left());
        }
        if lines.is_empty() {
            p.text(area.x, area.y + LIST_LINE * 0.5, "nothing yet", Text::mono(10.0, AMBER_DIM).left());
        }
    }
}

/// Whether a control does anything with the current settings.
fn applies(params: &SynthParams, id: &str) -> bool {
    let value = |id: &str| params.ptr(id).map(|p| unsafe { p.unmodulated_plain_value() }).unwrap_or(0.0);
    match id {
        // VA Sweep's A picks its shape; Wave 2 is its sub. Window Amp Sync,
        // Noise AM and Folding play Wave 1 alone.
        "dual_1.wave_1" => value("dual_1.algorithm") != 0.0,
        "dual_2.wave_1" => value("dual_2.algorithm") != 0.0,
        "dual_1.wave_2" => ![5.0, 9.0, 11.0].contains(&value("dual_1.algorithm")),
        "dual_2.wave_2" => ![5.0, 9.0, 11.0].contains(&value("dual_2.algorithm")),
        "wt_1.oscmod_depth" => value("wt_1.oscmod") != 0.0,
        "wt_2.oscmod_depth" => value("wt_2.oscmod") != 0.0,
        "noise.bits" => value("noise.type") == 1.0,                     // Clocked
        "noise.clock_ratio" => (1.0..=3.0).contains(&value("noise.type")), // Clocked, LFSR
        "noise.crackle_density" => value("noise.type") == 4.0,          // Crackle
        "unison.detune" => value("unison.mode") > 0.5,
        "settings.vintage_noise" => value("vintage.amount") > 0.0,
        // The SVF's own controls, with the ladder or the SH-101 chosen.
        "filter.morph_lp_notch_hp" | "filter.bandpass" | "filter.4_pole" => value("filter.type") == 0.0,
        // A Ray's orbit rate, with no orbit.
        "ray_1.orbit_rate" => value("ray_1.orbit") > 0.0,
        "ray_2.orbit_rate" => value("ray_2.orbit") > 0.0,
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
            (None, Some((name, value))) => (name.to_string(), value.to_string()),
            _ => Default::default(),
        };
        p.readout(self.layout.readout, &name, &value);

        for plate in &self.layout.plates {
            p.plate(plate.rect, plate.title);
        }
        let live = Live::from(&mut self.scopes.borrow_mut(), &self.params);
        for (screen, rect, _) in self.layout.screens.iter().filter(|(_, _, show)| self.shows(*show)) {
            p.screen(*rect);
            screens::draw(&mut p, &self.params, &live, self.selected, *screen, *rect);
        }
        self.draw_modulations(&mut p);
        for caption in self.layout.captions.iter().filter(|c| self.shows(c.show)) {
            use super::draw::MUTED;
            p.text(caption.x, caption.y, caption.text, Text::label(11.0, MUTED).left());
        }

        for i in self.layout.range(false).filter(|i| self.shows(self.layout.controls[*i].show)) {
            self.draw_control(&mut p, i);
        }

        for (k, b) in self.layout.actions.iter().enumerate().filter(|(_, b)| self.shows(b.show)) {
            let hot = self.hover_action == Some(k);
            match b.action {
                Action::Dice => p.dice(b.rect, hot),
                Action::Settings => p.hamburger(b.rect, hot, self.settings_open),
                _ if !self.enabled(b.action) => {
                    p.canvas.save();
                    p.canvas.set_global_alpha(0.32);
                    p.button(b.rect, b.label, false);
                    p.canvas.restore();
                }
                _ => p.button(b.rect, b.label, hot),
            }
        }

        self.draw_curve_popup(&mut p);

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
