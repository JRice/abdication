//! The whole front panel as one vizia view. It draws every plate and
//! control itself (see `draw`), hit-tests the mouse against the layout,
//! and changes parameters through nih-plug's gesture events, so the host
//! records automation and undo exactly as for its own controls.
//!
//! Modulation (Nov 2026, after Elektron's): each source (envelopes A, B, C,
//! LFOs X, Y, Z, the mod wheel, aftertouch, key and velocity) has a square
//! button. Press one and the panel dims, showing lit the knobs it moves and
//! how far; dragging any knob then sets how far the source moves it (a
//! double-click clears it). Press it again (or Esc, or click away) to
//! finish. Otherwise a knob shows a dot in each shade of the sources moving
//! it and, while a note plays, where the newest voice has it.

use std::cell::RefCell;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use nih_plug::prelude::ParamPtr;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::util::ModifiersExt;
use nih_plug_vizia::widgets::RawParamEvent;

use super::draw::{Marks, MenuGrid, Painter, Text, AMBER, LABEL, MUTED, SOURCE_COLORS};
use super::layout::{synced, Action, Control, Kind, Layout, Rect, Show, Strip, CHEEK, ENGINES, ENGINE_PREFIXES, HEIGHT,
                    RAY_CHAOS, SLOT_MODES, WIDTH};
use super::scopes::Scopes;
use super::screens::{self, Live};
use crate::modulation::{self, LfoShape, SOURCES, SOURCE_COUNT};
use crate::params::SynthParams;
use crate::scope::{LiveScope, ScopePitch};
use crate::{dice, presets, wavelib};

/// Logical pixels of mouse travel for a knob's full range.
const KNOB_TRAVEL: f32 = 220.0;
/// Shift-drag moves this much slower.
const FINE: f32 = 0.12;
const MENU_ROW: f32 = 22.0;
const MENU_MAX_ROWS: usize = 24;
/// The panel's housekeeping tick (live scope, hover fix): about 15 Hz.
const TICK: Duration = Duration::from_millis(66);
/// How many patches the dice's back button remembers.
const DICE_HISTORY: usize = 32;
/// The wave picker's cells, and how many to a row.
const WAVE_CELL: (f32, f32) = (56.0, 54.0);
const WAVE_COLUMNS: usize = 16;
const WAVE_HEADER: f32 = 20.0;

/// Sent to the panel by its ticker thread.
enum PanelEvent {
    Tick,
}

pub struct Panel {
    params: Arc<SynthParams>,
    layout: Layout,
    /// The parameter behind each control (a rate knob's is its division's
    /// while it's synced: see `ptr`).
    ptrs: Vec<ParamPtr>,
    /// For each control: the modulatable knob it is, if it is one.
    knob_target: Vec<Option<usize>>,
    hover: Option<usize>,
    /// The momentary button under the mouse.
    hover_action: Option<usize>,
    /// The settings overlay is showing.
    settings_open: bool,
    /// The source being assigned (its button held down), if any.
    assigning: Option<usize>,
    /// What the top bar's readout shows.
    focus: Option<usize>,
    /// A message for the readout until the mouse moves on (the kind of sound
    /// last rolled, say): its name and value.
    rolled: Option<(&'static str, String)>,
    /// The patch before each roll, newest last. Gone when the panel closes.
    dice_history: Vec<Vec<(ParamPtr, f32)>>,
    drag: Option<Drag>,
    menu: Option<Menu>,
    /// A preset being named, before it's saved.
    naming: Option<Naming>,
    /// Each slot's preset last loaded or saved: (its engine, its name).
    preset_names: [Option<(usize, String)>; 2],
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
    /// What the drag sets: the control's parameter, or (assigning) a depth.
    ptr: ParamPtr,
    /// Where the current relative drag is anchored (re-anchored when Shift
    /// changes so the value never jumps).
    anchor: f32,
    anchor_value: f32,
    fine: bool,
    /// A horizontal slider: it moves with x.
    across: bool,
}

#[derive(Clone, Copy, PartialEq)]
enum MenuKind {
    /// A parameter's settings, as a list.
    List,
    /// The wave picker (a wavetable's waves, or an LFO's shapes).
    Waves { lfo: bool },
    /// An Orthogonal's sets, by category.
    Samples,
    /// Slot k's presets for the engine it plays.
    Presets { slot: usize },
}

struct Menu {
    kind: MenuKind,
    /// The control it sets (not for presets).
    control: usize,
    items: Vec<String>,
    rect: Rect,
    /// Where each item is, and the headers over groups of them.
    cells: Vec<Rect>,
    headers: Vec<(&'static str, f32, f32)>,
    hover: Option<usize>,
    /// A preset menu's files.
    files: Vec<PathBuf>,
}

/// The preset-name entry: which slot's engine, the name so far, and its box.
/// The name starts as a suggestion (`fresh`), which the first key typed replaces.
struct Naming {
    slot: usize,
    engine: usize,
    text: String,
    fresh: bool,
    rect: Rect,
}

impl Naming {
    fn field(&self) -> Rect {
        Rect::new(self.rect.x + 14.0, self.rect.y + 34.0, self.rect.w - 28.0, 26.0)
    }
    fn save(&self) -> Rect {
        Rect::new(self.rect.right() - 14.0 - 70.0, self.rect.bottom() - 36.0, 70.0, 24.0)
    }
    fn cancel(&self) -> Rect {
        Rect::new(self.save().x - 8.0 - 70.0, self.rect.bottom() - 36.0, 70.0, 24.0)
    }
}

/// Rows in a column of an Orthogonal's set picker.
const SAMPLE_ROWS: usize = 16;

impl Panel {
    pub fn new(cx: &mut Context, params: Arc<SynthParams>, pitch: Arc<ScopePitch>, live: Arc<LiveScope>)
        -> Handle<'_, Self> {
        let layout = Layout::build();
        let ptrs = layout.controls.iter()
            .map(|c| params.ptr(c.id).unwrap_or_else(|| panic!("no parameter '{}'", c.id)))
            .collect();
        let knob_target = layout.controls.iter().map(|c| modulation::target_index(c.id)).collect();
        let scopes = RefCell::new(Scopes::new(&params, pitch, live));
        let ticking = Arc::new(AtomicBool::new(true));
        let ticker = ticking.clone();
        let panel = Self { params, layout, ptrs, knob_target, hover: None, hover_action: None, settings_open: false,
                           assigning: None, focus: None, rolled: None, dice_history: Vec::new(), drag: None,
                           menu: None, naming: None, preset_names: Default::default(), scopes, ticking };
        panel
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

    /// The parameter control `i` sets: a rate knob sets its division while
    /// it's synced.
    fn ptr(&self, i: usize) -> ParamPtr {
        if let Some((switch, division)) = synced(self.layout.controls[i].id) {
            if self.plain(switch) > 0.5 {
                return self.params.ptr(division).unwrap();
            }
        }
        self.ptrs[i]
    }

    fn value(&self, i: usize) -> f32 {
        unsafe { self.ptr(i).unmodulated_normalized_value() }
    }

    fn text(&self, i: usize) -> String {
        let p = self.ptr(i);
        unsafe { p.normalized_value_to_string(p.unmodulated_normalized_value(), true) }
    }

    /// Source `s`'s depth on control `i` (a modulatable knob): its parameter.
    fn depth_ptr(&self, i: usize, s: usize) -> Option<ParamPtr> {
        self.params.depth_ptr(self.knob_target[i]?, s)
    }

    /// Readout name and value for a control (assigning: the source's depth on it).
    fn describe(&self, i: usize) -> (String, String) {
        if let (Some(s), Some(p)) = (self.assigning, self.assigning.and_then(|s| self.depth_ptr(i, s))) {
            let knob = unsafe { self.ptr(i).name() }.to_string();
            let depth = unsafe { p.normalized_value_to_string(p.unmodulated_normalized_value(), true) };
            return (format!("{} → {}", SOURCES[s].label, knob.rsplit(": ").next().unwrap_or(&knob)), depth);
        }
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

    /// The control under (x, y): a curve button, then the HPF's arc, before
    /// the knob they sit by.
    fn hit(&self, x: f32, y: f32) -> Option<usize> {
        let under: Vec<(usize, &Control)> = self.layer().filter(|(_, c)| c.hit(x, y)).collect();
        let first = |test: &dyn Fn(&Control) -> bool| under.iter().find(|(_, c)| test(c)).map(|(i, _)| *i);
        first(&|c| matches!(c.kind, Kind::Curve { .. }))
            .or_else(|| first(&|c| matches!(c.kind, Kind::Arc { .. })))
            .or_else(|| under.first().map(|(i, _)| *i))
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

    fn plain(&self, id: &str) -> f32 {
        self.params.ptr(id).map_or(0.0, |p| unsafe { p.unmodulated_plain_value() })
    }

    /// Oscillator slot `slot`'s mode (layout's SLOT_*).
    fn slot_mode(&self, slot: usize) -> usize {
        (self.plain(SLOT_MODES[slot]) as usize).min(ENGINES.len() - 1)
    }

    // ---- Menus ---------------------------------------------------------------------

    fn open_menu(&mut self, i: usize) {
        let Some(steps) = self.steps(i) else { return };
        let p = self.ptr(i);
        let items: Vec<String> = (0..=steps)
            .map(|k| unsafe { p.normalized_value_to_string(k as f32 / steps as f32, false) })
            .collect();
        self.list_menu(MenuKind::List, i, items, self.layout.controls[i].rect, Vec::new());
    }

    /// A menu of `items` in columns (at most MENU_MAX_ROWS deep), under `under`.
    fn list_menu(&mut self, kind: MenuKind, control: usize, items: Vec<String>, under: Rect, files: Vec<PathBuf>) {
        let rows = items.len().clamp(1, MENU_MAX_ROWS);
        let cols = items.len().div_ceil(rows).max(1);
        let longest = items.iter().map(|s| s.chars().count()).max().unwrap_or(4) as f32;
        let col_w = (longest * 7.4 + 22.0).max(64.0);
        let (w, h) = ((col_w * cols as f32 + 12.0).max(under.w), rows as f32 * MENU_ROW + 12.0);
        let rect = Self::menu_rect(under, w, h);
        let grid = MenuGrid { rows, col_w, row_h: MENU_ROW };
        let cells = (0..items.len()).map(|k| grid.cell(rect, k)).collect();
        self.menu = Some(Menu { kind, control, items, rect, cells, headers: Vec::new(), hover: None, files });
    }

    /// A `w` x `h` menu for `control`: below it if there's room, else above,
    /// else centred; never past the cheeks.
    fn menu_rect(control: Rect, w: f32, h: f32) -> Rect {
        let x = (control.x + control.w / 2.0 - w / 2.0).clamp(CHEEK + 6.0, WIDTH - CHEEK - 6.0 - w);
        let below = control.bottom() + 4.0;
        let y = if below + h <= HEIGHT - 8.0 {
            below
        } else if control.y - 4.0 - h >= 8.0 {
            control.y - 4.0 - h
        } else {
            ((HEIGHT - h) / 2.0).max(8.0)
        };
        Rect::new(x, y, w, h)
    }

    /// The wave picker: the basic waves first, then (an LFO's) its own
    /// shapes, then the library set by set, each under its name.
    fn open_wave_menu(&mut self, i: usize, lfo: bool) {
        let only = if lfo { wavelib::LFO_ONLY.len() } else { 0 };
        let mut sections: Vec<(&'static str, std::ops::Range<usize>)> = vec![("Basic", 0..4)];
        if lfo {
            sections.push(("LFO only", 4..4 + only));
        }
        for ((lo, end), name) in wavelib::WAVE_SETS.iter().zip(["A8 · Argon8 images", "M8 · bitmask, 8 slices",
                                                                 "M16 · bitmask, 16 slices"]) {
            sections.push((name, *lo as usize + only..*end as usize + only));
        }
        let count = sections.last().map_or(0, |s| s.1.end);
        let p = self.ptr(i);
        let steps = unsafe { p.step_count() }.unwrap_or(1).max(1);
        let items: Vec<String> = (0..count)
            .map(|k| unsafe { p.normalized_value_to_string(k as f32 / steps as f32, false) })
            .collect();
        let rows: f32 = sections.iter().map(|(_, r)| r.len().div_ceil(WAVE_COLUMNS) as f32).sum();
        let (cw, ch) = WAVE_CELL;
        let w = cw * WAVE_COLUMNS as f32 + 16.0;
        let h = rows * ch + WAVE_HEADER * sections.len() as f32 + 16.0;
        let rect = Self::menu_rect(self.layout.controls[i].rect, w, h);
        let mut cells = vec![Rect::default(); count];
        let mut headers = Vec::new();
        let mut y = rect.y + 8.0;
        for (name, range) in sections {
            headers.push((name, rect.x + 10.0, y + WAVE_HEADER / 2.0));
            y += WAVE_HEADER;
            for (n, k) in range.clone().enumerate() {
                let (col, row) = (n % WAVE_COLUMNS, n / WAVE_COLUMNS);
                cells[k] = Rect::new(rect.x + 8.0 + col as f32 * cw, y + row as f32 * ch, cw, ch);
            }
            y += range.len().div_ceil(WAVE_COLUMNS) as f32 * ch;
        }
        self.menu = Some(Menu { kind: MenuKind::Waves { lfo }, control: i, items, rect, cells, headers, hover: None,
                                files: Vec::new() });
    }

    /// An Orthogonal's sets, by category: each category its own columns
    /// (at most SAMPLE_ROWS deep, shared out evenly) under its name.
    fn open_sample_menu(&mut self, i: usize) {
        use crate::ortholib::{category, CATEGORIES, SETS};
        let items: Vec<String> = SETS.iter().map(|s| s.name.to_string()).collect();
        let longest = items.iter().map(|s| s.chars().count()).max().unwrap_or(4) as f32;
        let col_w = longest * 7.4 + 22.0;
        let blocks: Vec<(usize, Vec<usize>)> = (0..CATEGORIES.len()).map(|c| (c, category(c).map(|(k, _)| k).collect())).collect();
        let columns = |n: usize| n.div_ceil(SAMPLE_ROWS).max(1);
        let total_cols: usize = blocks.iter().map(|(_, sets)| columns(sets.len())).sum();
        let deepest = blocks.iter().map(|(_, sets)| sets.len().div_ceil(columns(sets.len()))).max().unwrap_or(1);
        const HEADER_H: f32 = 22.0;
        const BLOCK_GAP: f32 = 14.0;
        let w = col_w * total_cols as f32 + BLOCK_GAP * (blocks.len() - 1) as f32 + 12.0;
        let h = HEADER_H + deepest as f32 * MENU_ROW + 12.0;
        let rect = Self::menu_rect(self.layout.controls[i].rect, w, h);
        let mut cells = vec![Rect::default(); items.len()];
        let mut headers = Vec::new();
        let mut x = rect.x + 6.0;
        for (c, sets) in &blocks {
            headers.push((CATEGORIES[*c], x + 8.0, rect.y + 6.0 + HEADER_H / 2.0));
            let per_column = sets.len().div_ceil(columns(sets.len()));
            for (n, &k) in sets.iter().enumerate() {
                let (col, row) = (n / per_column, n % per_column);
                cells[k] = Rect::new(x + col as f32 * col_w, rect.y + 6.0 + HEADER_H + row as f32 * MENU_ROW, col_w, MENU_ROW);
            }
            x += columns(sets.len()) as f32 * col_w + BLOCK_GAP;
        }
        self.menu = Some(Menu { kind: MenuKind::Samples, control: i, items, rect, cells, headers, hover: None,
                                files: Vec::new() });
    }

    /// Slot `slot`'s presets for the engine it plays.
    fn open_presets(&mut self, slot: usize, under: Rect) {
        let engine = ENGINES[self.slot_mode(slot)];
        let (mut items, mut files): (Vec<String>, Vec<PathBuf>) = presets::list(engine).into_iter().unzip();
        if items.is_empty() {
            items.push(format!("no {engine} presets yet"));
            files.push(PathBuf::new());
        }
        self.list_menu(MenuKind::Presets { slot }, usize::MAX, items, under, files);
    }

    fn menu_item_at(menu: &Menu, x: f32, y: f32) -> Option<usize> {
        menu.cells.iter().position(|c| c.contains(x, y))
    }

    /// A menu item was picked.
    fn pick(&mut self, cx: &mut EventContext, menu: Menu, k: usize) {
        match menu.kind {
            MenuKind::Presets { slot } => {
                let path = &menu.files[k];
                if !path.as_os_str().is_empty() {
                    self.load_preset(cx, slot, path, &menu.items[k]);
                }
            }
            _ => {
                let steps = self.steps(menu.control).unwrap_or(1).max(1);
                Self::change(cx, self.ptr(menu.control), k as f32 / steps as f32);
                self.picked(cx, menu.control);
                self.focus = Some(menu.control);
            }
        }
    }

    // ---- Presets -----------------------------------------------------------------------

    /// The prefix of slot `slot`'s engine `mode`'s controls ("wt_1.").
    fn preset_prefix(slot: usize, mode: usize) -> String {
        format!("{}_{}.", ENGINE_PREFIXES[mode], slot + 1)
    }

    /// Load a preset into slot `slot`'s engine: its own controls only, and
    /// only those that differ (the depths on them stay as they are).
    fn load_preset(&mut self, cx: &mut EventContext, slot: usize, path: &std::path::Path, name: &str) {
        let Some(values) = presets::load(path) else {
            self.rolled = Some(("Preset", "couldn't be read".into()));
            return;
        };
        let mode = self.slot_mode(slot);
        let prefix = Self::preset_prefix(slot, mode);
        for (key, value) in values {
            let Some(p) = self.params.ptr(&format!("{prefix}{key}")) else { continue };
            let normalized = unsafe { p.preview_normalized(value) };
            if unsafe { p.unmodulated_normalized_value() } != normalized {
                Self::change(cx, p, normalized);
            }
        }
        self.preset_names[slot] = Some((mode, name.to_string()));
        self.focus = None;
        self.rolled = Some(("Preset", name.to_string()));
    }

    fn start_naming(&mut self, cx: &mut EventContext, slot: usize, button: Rect) {
        let engine = self.slot_mode(slot);
        let text = match &self.preset_names[slot] {
            Some((mode, name)) if *mode == engine => name.clone(),
            _ => format!("{} {}", ENGINES[engine], presets::list(ENGINES[engine]).len() + 1),
        };
        let rect = Rect::new(button.right() - 320.0, button.bottom() + 6.0, 320.0, 106.0);
        self.naming = Some(Naming { slot, engine, text, fresh: true, rect });
        self.menu = None;
        cx.focus();
    }

    fn save_preset(&mut self) {
        let Some(n) = self.naming.take() else { return };
        let prefix = Self::preset_prefix(n.slot, n.engine);
        let values: Vec<(String, f32)> = self.params.ids()
            .filter_map(|id| id.strip_prefix(&prefix).map(|key| (key.to_string(), self.plain(id))))
            .collect();
        let name = presets::file_name(&n.text);
        match presets::save(ENGINES[n.engine], &name, &values) {
            Ok(_) => {
                self.preset_names[n.slot] = Some((n.engine, name.clone()));
                self.rolled = Some(("Saved", name));
            }
            Err(_) => self.rolled = Some(("Preset", "couldn't be saved".into())),
        }
        self.focus = None;
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

    /// Start dragging `ptr` from control `i` (vertical, or across for a slider).
    fn start_drag(&mut self, cx: &mut EventContext, i: usize, ptr: ParamPtr, x: f32, y: f32, across: bool) {
        Self::begin(cx, ptr);
        let anchor = if across { x } else { y };
        self.drag = Some(Drag { target: i, ptr, anchor, anchor_value: unsafe { ptr.unmodulated_normalized_value() },
                                fine: cx.modifiers().shift(), across });
        cx.capture();
    }

    fn mouse_down(&mut self, cx: &mut EventContext, x: f32, y: f32) {
        // A drag whose release never arrived (e.g. let go outside the
        // window): finish it before starting anything new.
        self.mouse_up(cx);
        if let Some(n) = &self.naming {
            if n.save().contains(x, y) {
                self.save_preset();
            } else if n.cancel().contains(x, y) || !n.rect.contains(x, y) {
                self.naming = None;
            }
            return;
        }
        if let Some(menu) = self.menu.take() {
            if let Some(k) = Self::menu_item_at(&menu, x, y) {
                self.pick(cx, menu, k);
            }
            return;
        }
        if let Some(k) = self.action_at(x, y) {
            self.act(cx, k);
            return;
        }
        // A click outside the open settings closes them.
        if self.settings_open && !self.layout.settings.contains(x, y) {
            self.close_settings();
            return;
        }
        let hit = self.hit(x, y);
        if let Some(s) = self.assigning {
            // Assigning: dragging a knob sets the source's depth on it; a
            // click anywhere else finishes.
            match hit.and_then(|i| self.depth_ptr(i, s).map(|p| (i, p))) {
                Some((i, p)) => {
                    self.focus = Some(i);
                    let across = matches!(self.layout.controls[i].kind, Kind::Slider);
                    self.start_drag(cx, i, p, x, y, across);
                }
                None => self.assigning = None,
            }
            return;
        }
        let Some(i) = hit else { return };
        self.focus = Some(i);
        let control = self.layout.controls[i];
        let p = self.ptr(i);
        match control.kind {
            Kind::Knob { .. } | Kind::Fader { .. } | Kind::Curve { .. } | Kind::Arc { .. } => {
                self.start_drag(cx, i, p, x, y, false)
            }
            // A slider: a press on the track jumps the thumb there, then drags.
            Kind::Slider => {
                Self::change(cx, p, Painter::slider_value(control.rect, x));
                self.start_drag(cx, i, p, x, y, true);
            }
            // A toggle with more than two settings (Unison: Off, U5, U7) cycles.
            Kind::Toggle if self.steps(i).is_some_and(|s| s > 1) => self.step(cx, i, true),
            Kind::Toggle => Self::change(cx, p, if self.value(i) > 0.5 { 0.0 } else { 1.0 }),
            Kind::Segments => {
                let steps = self.steps(i).unwrap_or(1).max(1);
                let k = (((x - control.rect.x) / control.rect.w) * (steps + 1) as f32).floor().clamp(0.0, steps as f32);
                Self::change(cx, p, k / steps as f32);
            }
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
            Kind::Sample => self.open_sample_menu(i),
            Kind::Wave { lfo } => self.open_wave_menu(i, lfo),
        }
    }

    /// The marks control `i` carries: the shades of the sources moving it,
    /// the source being assigned and its depth, and where the newest voice
    /// has it.
    fn marks(&self, i: usize) -> Marks {
        let Some(t) = self.knob_target[i] else { return Marks::default() };
        let pips: Vec<u32> = (0..SOURCE_COUNT).filter(|&s| self.params.depth(t, s) != 0.0).map(|s| SOURCE_COLORS[s]).collect();
        let assign = self.assigning.map(|s| (SOURCE_COLORS[s], self.params.depth(t, s)));
        let moved = if pips.is_empty() { None } else { self.scopes.borrow().moved(t) };
        Marks { pips, assign, moved }
    }

    /// Control `i`'s label: the filter's A and B say what the type makes them.
    fn label(&self, i: usize) -> &'static str {
        let c = &self.layout.controls[i];
        let ladder = self.plain("filter.type") > 0.5;
        match c.id {
            "filter.a" => if ladder { "A · Spread" } else { "A · Morph" },
            "filter.b" => if ladder { "B · Noise FM" } else { "B · BP Mix" },
            _ => c.label,
        }
    }

    /// One control, dimmed if it does nothing with the current settings
    /// (still adjustable, ready for when it applies).
    fn draw_control(&self, p: &mut Painter, live: &Live, i: usize) {
        let c = &self.layout.controls[i];
        let dimmed = !applies(&self.params, c.id) && self.assigning.is_none();
        if dimmed {
            p.canvas.save();
            p.canvas.set_global_alpha(0.32);
        }
        let hot = self.drag.as_ref().map(|d| d.target).or(self.hover) == Some(i);
        let marks = self.marks(i);
        let v = self.value(i);
        p.tint = windows_row(c.id).map(super::light::label);
        match c.kind {
            Kind::Knob { size, bipolar, label } => {
                p.knob(c.rect, size, v, bipolar, label, self.label(i), &self.text(i), hot, &marks)
            }
            Kind::Fader { bipolar, tint, cap } => {
                let colour = tint.map_or(AMBER, |s| SOURCE_COLORS[s]);
                p.tint = tint.map(|s| SOURCE_COLORS[s]).filter(|_| c.id == "filter.env_b");
                p.fader(c.rect, v, bipolar, colour, cap, c.label, hot, &marks)
            }
            Kind::Arc { cx, cy, r } => p.hpf_arc(cx, cy, r, v, hot, &marks),
            Kind::Toggle if self.steps(i).is_some_and(|s| s > 1) => {
                // Lit and showing its setting when on; its own label when off.
                let on = v > 0.0;
                let label = if on { self.text(i) } else { c.label.to_string() };
                p.toggle(c.rect, on, &label, hot)
            }
            Kind::Toggle => p.toggle(c.rect, v > 0.5, c.label, hot),
            Kind::Segments => {
                let steps = self.steps(i).unwrap_or(1);
                let ptr = self.ptr(i);
                let names: Vec<String> = (0..=steps)
                    .map(|k| unsafe { ptr.normalized_value_to_string(k as f32 / steps.max(1) as f32, false) }.to_lowercase())
                    .collect();
                let names: Vec<&str> = names.iter().map(String::as_str).collect();
                p.segments(c.rect, &names, (v * steps as f32).round() as usize, hot)
            }
            Kind::Slider => p.slider(c.rect, v, &self.text(i), hot, &marks),
            Kind::Stepper => p.stepper(c.rect, &self.text(i), hot),
            Kind::Dropdown => p.dropdown(c.rect, &self.text(i), hot),
            Kind::Curve { rising } => p.curve_button(c.rect, &self.bow_points(i, rising, 10), hot),
            Kind::Wave { lfo } => {
                p.screen(c.rect);
                screens::wave_box(p, &self.params, live, c.id, lfo, c.rect, hot);
            }
            Kind::Sample => {
                let set = unsafe { self.ptr(i).unmodulated_plain_value() } as usize;
                // Where the newest voice's player is, or (no note) where its Position is.
                let (slot, half) = sample_player(c.id);
                let live = self.scopes.borrow().ortho_position(slot, half);
                let position = live.unwrap_or_else(|| self.plain(&format!("{}_position", c.id)));
                screens::sample(p, c.rect, set, position, live.is_some(), hot);
            }
        }
        p.tint = None;
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
        let Some(i) = self.drag.as_ref().filter(|_| self.assigning.is_none()).map(|d| d.target) else { return };
        let c = &self.layout.controls[i];
        let Kind::Curve { rising } = c.kind else { return };
        let name = unsafe { self.ptr(i).name() }.to_string();
        p.curve_popup(c.rect, &name, &self.text(i), &self.bow_points(i, rising, 60), rising);
    }

    /// The button under the pointer (only ☰ while the settings are open;
    /// only the sources' while assigning).
    fn action_at(&self, x: f32, y: f32) -> Option<usize> {
        self.layout.actions.iter().position(|b| {
            b.rect.contains(x, y) && (!self.settings_open || b.action == Action::Settings) && self.shows(b.show)
                && (self.assigning.is_none() || matches!(b.action, Action::Assign(_)))
        })
    }

    fn close_settings(&mut self) {
        self.settings_open = false;
        self.hover = None;
    }

    fn act(&mut self, cx: &mut EventContext, k: usize) {
        let button = self.layout.actions[k];
        match button.action {
            Action::Assign(s) => {
                self.assigning = if self.assigning == Some(s) { None } else { Some(s) };
                self.settings_open = false;
                self.focus = None;
                self.rolled = self.assigning.map(|s| (SOURCES[s].name, "drag a knob".to_string()));
            }
            Action::ShiftWaves { slot, up } => {
                for w in 1..=5 {
                    if let Some(p) = self.params.ptr(&format!("wt_{}.wave_{w}", slot + 1)) {
                        Self::step_param(cx, p, up);
                    }
                }
            }
            Action::Presets { slot } => self.open_presets(slot, button.rect),
            Action::SavePreset { slot } => self.start_naming(cx, slot, button.rect),
            Action::Init => self.init_patch(cx),
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
        let targets = patch.values.iter()
            .filter_map(|(id, value)| self.params.ptr(id).map(|p| (p, unsafe { p.preview_normalized(*value) })))
            .collect();
        self.replace_patch(cx, targets);
        self.rolled = Some(("Dice", patch.kind.name().to_string()));
    }

    /// Every parameter back to its default, then the init patch's own few
    /// (dice::INIT): one plain oscillator, everything else off. Volume and
    /// the settings too. The ‹ steps back from it as from a roll.
    fn init_patch(&mut self, cx: &mut EventContext) {
        let targets = self.params.ids()
            .filter_map(|id| self.params.ptr(id).map(|p| {
                let init = dice::INIT.iter().find(|(i, _)| *i == id);
                (p, init.map_or(unsafe { p.default_normalized_value() }, |(_, v)| unsafe { p.preview_normalized(*v) }))
            }))
            .collect();
        self.replace_patch(cx, targets);
        self.rolled = Some(("Init", "one plain oscillator".to_string()));
    }

    /// Set each parameter to its normalized value, remembering the patch it
    /// replaces for the ‹. Only what actually changes is sent: the host hears
    /// about a change through a queue that holds 2048 events (nih-plug's CLAP
    /// wrapper; three per change), and a patch names some 1,100 values,
    /// nearly all depths already at rest.
    fn replace_patch(&mut self, cx: &mut EventContext, targets: Vec<(ParamPtr, f32)>) {
        let changes: Vec<(ParamPtr, f32)> = targets.into_iter()
            .filter(|(p, normalized)| unsafe { p.unmodulated_normalized_value() } != *normalized)
            .collect();
        if self.dice_history.len() == DICE_HISTORY {
            self.dice_history.remove(0);
        }
        self.dice_history.push(changes.iter().map(|(p, _)| (*p, unsafe { p.unmodulated_normalized_value() })).collect());
        for (p, normalized) in changes {
            Self::change(cx, p, normalized);
        }
        self.focus = None;
        self.preset_names = Default::default();
    }

    fn dice_back(&mut self, cx: &mut EventContext) {
        if let Some(patch) = self.dice_history.pop() {
            for (p, value) in patch {
                Self::change(cx, p, value);
            }
            self.focus = None;
            self.rolled = Some(("Dice", "Previous patch".to_string()));
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
        if let Some(drag) = self.drag.as_mut() {
            let control: Control = self.layout.controls[drag.target];
            let fine = cx.modifiers().shift();
            let along = if drag.across { x } else { y };
            if fine != drag.fine {
                // Re-anchor so switching speed mid-drag doesn't jump.
                drag.anchor_value = unsafe { drag.ptr.unmodulated_normalized_value() };
                drag.anchor = along;
                drag.fine = fine;
            }
            // Up (or, across, right) raises it.
            let moved = if drag.across { along - drag.anchor } else { drag.anchor - along };
            let travel = match control.kind {
                Kind::Fader { .. } if self.assigning.is_none() => control.rect.h,
                Kind::Slider => Painter::slider_travel(control.rect),
                _ => KNOB_TRAVEL,
            };
            let speed = if fine { FINE } else { 1.0 };
            Self::set(cx, drag.ptr, drag.anchor_value + moved / travel * speed);
            cx.needs_redraw();
            return;
        }
        let hover_action = self.action_at(x, y);
        if hover_action != self.hover_action {
            self.hover_action = hover_action;
            cx.needs_redraw();
        }
        let hover = self.hit(x, y).filter(|i| self.assigning.is_none() || self.knob_target[*i].is_some());
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
            Self::end(cx, drag.ptr);
            cx.release();
        }
    }

    /// Knobs, faders and the like reset on a double-click; everything else
    /// is a button.
    fn resets(&self, i: usize) -> bool {
        matches!(self.layout.controls[i].kind,
                 Kind::Knob { .. } | Kind::Fader { .. } | Kind::Curve { .. } | Kind::Slider | Kind::Arc { .. })
    }

    /// vizia delivers a quick second (and third) click only as a double
    /// (triple) click. On a knob or fader a double-click restores the
    /// default (assigning: clears the source's depth on it); anywhere else
    /// repeated clicks are ordinary presses, so clicking an arrow quickly
    /// steps every time.
    fn repeat_click(&mut self, cx: &mut EventContext, x: f32, y: f32, double: bool) {
        if self.menu.is_some() || self.naming.is_some() || self.action_at(x, y).is_some() {
            return self.mouse_down(cx, x, y);
        }
        match self.hit(x, y) {
            Some(t) if self.resets(t) => {
                if double {
                    self.reset(cx, t);
                }
            }
            _ => self.mouse_down(cx, x, y),
        }
    }

    fn reset(&mut self, cx: &mut EventContext, i: usize) {
        let p = match self.assigning {
            Some(s) => match self.depth_ptr(i, s) {
                Some(p) => p,
                None => return,
            },
            None => self.ptr(i),
        };
        Self::change(cx, p, unsafe { p.default_normalized_value() });
        self.focus = Some(i);
    }

    fn scroll(&mut self, cx: &mut EventContext, x: f32, y: f32, dy: f32) {
        if self.menu.is_some() || self.drag.is_some() || self.naming.is_some() || dy == 0.0 {
            return;
        }
        let Some(i) = self.hit(x, y) else { return };
        let up = dy > 0.0;
        let notch = if cx.modifiers().shift() { 0.002 } else { 0.01 };
        if let Some(s) = self.assigning {
            if let Some(p) = self.depth_ptr(i, s) {
                let v = unsafe { p.unmodulated_normalized_value() };
                Self::change(cx, p, v + if up { notch } else { -notch });
            }
            return;
        }
        let p = self.ptr(i);
        match self.layout.controls[i].kind {
            Kind::Stepper | Kind::Dropdown | Kind::Sample | Kind::Wave { .. } | Kind::Segments => self.step(cx, i, up),
            Kind::Knob { .. } | Kind::Fader { .. } | Kind::Curve { .. } | Kind::Slider | Kind::Arc { .. } => {
                let v = self.value(i);
                let next = match self.steps(i) {
                    Some(_) => unsafe {
                        if up { p.next_normalized_step(v, false) } else { p.previous_normalized_step(v, false) }
                    },
                    None => v + if up { notch } else { -notch },
                };
                Self::change(cx, p, next);
            }
            Kind::Toggle => {}
        }
    }

    /// The preset name entry: typing.
    fn key(&mut self, code: Code) -> bool {
        let Some(n) = &mut self.naming else { return false };
        match code {
            Code::Backspace => {
                n.text.pop();
                n.fresh = false;
            }
            Code::Enter | Code::NumpadEnter => self.save_preset(),
            Code::Escape => self.naming = None,
            _ => return false,
        }
        true
    }

    fn type_char(&mut self, c: char) {
        if let Some(n) = &mut self.naming {
            if !c.is_control() && n.text.chars().count() < 40 {
                if std::mem::take(&mut n.fresh) {
                    n.text.clear();
                }
                n.text.push(c);
            }
        }
    }
}

impl Panel {
    /// The plates, their washes and dividers, the captions.
    fn draw_furniture(&self, p: &mut Painter) {
        for plate in &self.layout.plates {
            let colours: Vec<u32> = match plate.strip {
                Strip::Section(c) => vec![c],
                Strip::Sources(list) => list.iter().map(|s| SOURCE_COLORS[*s]).collect(),
            };
            p.plate(plate.rect, plate.title, &colours);
        }
        for wash in &self.layout.washes {
            p.wash(wash.rect, SOURCE_COLORS[wash.source]);
        }
        for &(x, y, h) in &self.layout.dividers {
            p.divider(x, y, h);
        }
        for caption in self.layout.captions.iter().filter(|c| self.shows(c.show)) {
            let colour = caption.color.map_or(if caption.size > 12.0 { super::draw::TITLE } else { MUTED }, |s| SOURCE_COLORS[s]);
            p.text(caption.x, caption.y, caption.text, Text::label(caption.size, colour).left());
        }
        let (x, y) = self.layout.wordmark;
        p.wordmark(x, y);
    }

    fn draw_actions(&self, p: &mut Painter, only_sources: bool) {
        for (k, b) in self.layout.actions.iter().enumerate().filter(|(_, b)| self.shows(b.show)) {
            let hot = self.hover_action == Some(k);
            match b.action {
                Action::Assign(s) => p.assign_button(b.rect, SOURCES[s].label, SOURCE_COLORS[s], self.assigning == Some(s), hot),
                _ if only_sources => {}
                Action::Dice => p.dice(b.rect, hot),
                Action::Init => p.empty_set(b.rect, hot),
                Action::Settings => p.hamburger(b.rect, hot, self.settings_open),
                Action::SavePreset { .. } => p.save_button(b.rect, hot),
                Action::Presets { slot } => {
                    let name = match &self.preset_names[slot] {
                        Some((mode, name)) if *mode == self.slot_mode(slot) => name.clone(),
                        _ => "Presets".into(),
                    };
                    p.dropdown(b.rect, &name, hot)
                }
                _ => p.button(b.rect, b.label, hot),
            }
        }
    }

    fn draw_readout(&self, p: &mut Painter) {
        let (name, value) = match (self.focus, &self.rolled) {
            (Some(t), _) => self.describe(t),
            (None, Some((name, value))) => (name.to_string(), value.clone()),
            _ => Default::default(),
        };
        p.readout(self.layout.readout, &name, &value);
    }

    fn draw_menu(&self, p: &mut Painter, menu: &Menu) {
        match menu.kind {
            MenuKind::Waves { lfo } => {
                p.popup(menu.rect);
                for &(text, x, y) in &menu.headers {
                    p.text(x, y, text, Text::label(11.0, MUTED).left());
                }
                let selected = (self.value(menu.control) * self.steps(menu.control).unwrap_or(1) as f32).round() as usize;
                for (k, (item, cell)) in menu.items.iter().zip(&menu.cells).enumerate() {
                    let shape = if lfo { LfoShape::from_menu(k) } else { LfoShape::Wave(k) };
                    let sketch = match shape {
                        LfoShape::Wave(_) => Vec::new(),
                        _ => modulation::sketch(shape, 41),
                    };
                    let wave = |t: f32| match shape {
                        LfoShape::Wave(w) if w < 4 => shape.wave(t).unwrap_or(0.0) * 0.85,
                        LfoShape::Wave(w) => screens::wave_outline(w, t) * 0.85,
                        _ => sketch[((t * 40.0) as usize).min(40)] * 0.85,
                    };
                    p.wave_cell(*cell, item, &wave, k == selected, menu.hover == Some(k));
                }
            }
            MenuKind::Presets { slot } => {
                let current = self.preset_names[slot].as_ref().map(|(_, n)| n.as_str());
                let selected = menu.items.iter().position(|i| Some(i.as_str()) == current).unwrap_or(usize::MAX);
                p.menu(menu.rect, &menu.items, &menu.cells, &menu.headers, selected, menu.hover, None);
            }
            _ => {
                let selected = (self.value(menu.control) * self.steps(menu.control).unwrap_or(1) as f32).round() as usize;
                p.menu(menu.rect, &menu.items, &menu.cells, &menu.headers, selected, menu.hover, None);
            }
        }
    }

    fn draw_naming(&self, p: &mut Painter, n: &Naming) {
        p.popup(n.rect);
        let title = format!("Save {} preset (slot {})", ENGINES[n.engine], ["I", "II"][n.slot]);
        p.text(n.rect.x + 14.0, n.rect.y + 18.0, &title, Text::label(11.0, LABEL).left());
        p.text_field(n.field(), &n.text);
        p.button(n.cancel(), "Cancel", false);
        p.button(n.save(), "Save", true);
    }
}

/// The Orthogonal player a sample control ("ortho_2.top") is: (slot, 0 Top / 1 Bottom).
fn sample_player(id: &str) -> (usize, usize) {
    (usize::from(id.starts_with("ortho_2")), usize::from(id.ends_with("bottom")))
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
        "transient.depth" | "transient.ratio" => value("transient.type") == crate::translib::FM as f32,
        "unison.detune" | "unison.spread" | "unison.eco" => value("unison.mode") > 0.5,
        "settings.vintage_noise" => value("vintage.amount") > 0.0,
        // The SVF's own switch, with a ladder chosen.
        "filter.4_pole" => value("filter.type") == 0.0,
        // The Harmonic EQ's shape knobs, with no shape.
        "harmonic.shift" | "harmonic.depth" => value("harmonic.shape") > 0.0,
        // A Ray's orbit rate, with no orbit.
        "ray_1.orbit_rate" => value("ray_1.orbit") > 0.0,
        "ray_2.orbit_rate" => value("ray_2.orbit") > 0.0,
        // LFO Z's depth on pitch, at 0.
        "lfo_z.mode" => value("lfo_z.depth") > 0.0,
        // A Windows interrupt's shape and knobs, with it switched off.
        // "windows_1.int_2_phase": its interrupt is "windows_1.int_2".
        id if id.starts_with("windows_") && id.contains(".int_") && !id.ends_with("_on") => {
            value(&format!("{}_on", &id[.."windows_1.int_2".len()])) > 0.5
        }
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
                    if self.menu.take().is_some() || self.assigning.take().is_some() {
                        meta.consume();
                    }
                }
                WindowEvent::MouseScroll(_, dy) => {
                    self.scroll(cx, x, y, dy);
                    meta.consume();
                }
                WindowEvent::CharInput(c) => {
                    if self.naming.is_none() {
                        return;
                    }
                    self.type_char(c);
                    meta.consume();
                }
                WindowEvent::KeyDown(code, _) => {
                    if self.key(code) {
                        meta.consume();
                    } else if code == Code::Escape {
                        if self.menu.take().is_none() && self.assigning.take().is_none() {
                            self.close_settings();
                        }
                    } else {
                        return;
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

        p.raised(self.layout.top_bar, 9.0, 0x34302a, 0x24211c);
        self.draw_furniture(&mut p);
        let live = Live::from(&mut self.scopes.borrow_mut(), &self.params);
        for (screen, rect, _) in self.layout.screens.iter().filter(|(_, _, show)| self.shows(*show)) {
            p.screen(*rect);
            screens::draw(&mut p, &self.params, &live, *screen, *rect);
        }
        for i in self.layout.range(false).filter(|i| self.shows(self.layout.controls[*i].show)) {
            self.draw_control(&mut p, &live, i);
        }
        self.draw_actions(&mut p, false);

        // Assigning: everything under a veil but the source's buttons, the
        // knobs it moves (and the one under the mouse), and the readout.
        if let Some(s) = self.assigning {
            p.veil(WIDTH, HEIGHT);
            self.draw_actions(&mut p, true);
            let active = |i: usize| {
                self.knob_target[i].is_some_and(|t| self.params.depth(t, s) != 0.0)
                    || self.hover == Some(i) || self.drag.as_ref().is_some_and(|d| d.target == i)
            };
            for i in self.layout.range(false).filter(|i| self.shows(self.layout.controls[*i].show) && active(*i)) {
                self.draw_control(&mut p, &live, i);
            }
            let hint = format!("Assigning {}: drag any knob to set how far it moves it · double-click clears · \
                                {} again (or Esc) to finish", SOURCES[s].name, SOURCES[s].label);
            p.text(WIDTH / 2.0, HEIGHT - 9.0, &hint, Text::label(11.0, SOURCE_COLORS[s]));
        }
        self.draw_readout(&mut p);

        self.draw_curve_popup(&mut p);

        if self.settings_open {
            p.plate(self.layout.settings, "Settings", &[]);
            for i in self.layout.range(true) {
                self.draw_control(&mut p, &live, i);
            }
        }

        if let Some(menu) = &self.menu {
            self.draw_menu(&mut p, menu);
        }
        if let Some(n) = &self.naming {
            self.draw_naming(&mut p, n);
        }
        canvas.restore();
    }
}

/// Which interrupt row (0..3) a Windows control is on: "windows_1.int_2_x"
/// is row 1. Its labels are tinted with that interrupt's light.
fn windows_row(id: &str) -> Option<usize> {
    let rest = id.strip_prefix("windows_")?.split_once(".int_")?.1;
    let k: usize = rest.split('_').next()?.parse().ok()?;
    (1..=3).contains(&k).then(|| k - 1)
}
