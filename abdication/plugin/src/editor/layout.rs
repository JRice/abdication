//! Where everything goes, in logical pixels on a 1600 x 1000 panel. The
//! editor scales the whole panel uniformly, so these numbers never change
//! with the window size. The arrangement follows the design mockup: a top
//! bar, then three rows of plates.

pub const WIDTH: f32 = 1600.0;
pub const HEIGHT: f32 = 1000.0;
/// Walnut end cheeks.
pub const CHEEK: f32 = 22.0;

const GAP: f32 = 14.0;
const HEADER: f32 = 26.0;
const INNER_GAP: f32 = 10.0;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub const fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }
    pub fn right(self) -> f32 {
        self.x + self.w
    }
    pub fn bottom(self) -> f32 {
        self.y + self.h
    }
    pub fn center(self) -> (f32, f32) {
        (self.x + self.w / 2.0, self.y + self.h / 2.0)
    }
    pub fn contains(self, x: f32, y: f32) -> bool {
        x >= self.x && x < self.right() && y >= self.y && y < self.bottom()
    }
    pub fn inset(self, left: f32, top: f32, right: f32, bottom: f32) -> Self {
        Self::new(self.x + left, self.y + top, self.w - left - right, self.h - top - bottom)
    }
    /// Split off the top `h` pixels; the rest starts `gap` below.
    pub fn take_top(self, h: f32, gap: f32) -> (Self, Self) {
        (Self { h, ..self }, Self::new(self.x, self.y + h + gap, self.w, self.h - h - gap))
    }
    /// Split off the bottom `h` pixels; the rest ends `gap` above.
    pub fn take_bottom(self, h: f32, gap: f32) -> (Self, Self) {
        (Self::new(self.x, self.bottom() - h, self.w, h), Self { h: self.h - h - gap, ..self })
    }
    pub fn take_left(self, w: f32, gap: f32) -> (Self, Self) {
        (Self { w, ..self }, Self::new(self.x + w + gap, self.y, self.w - w - gap, self.h))
    }
    pub fn take_right(self, w: f32, gap: f32) -> (Self, Self) {
        (Self::new(self.right() - w, self.y, w, self.h), Self { w: self.w - w - gap, ..self })
    }
    /// Columns sized by weight, like CSS `flex: <weight> 1 0`.
    pub fn columns(self, weights: &[f32], gap: f32) -> Vec<Self> {
        let total: f32 = weights.iter().sum();
        let free = self.w - gap * (weights.len() as f32 - 1.0);
        let mut x = self.x;
        weights
            .iter()
            .map(|w| {
                let width = free * w / total;
                let r = Self::new(x, self.y, width, self.h);
                x += width + gap;
                r
            })
            .collect()
    }
    /// A `w` x `h` box centred in this one.
    pub fn centered(self, w: f32, h: f32) -> Self {
        Self::new(self.x + (self.w - w) / 2.0, self.y + (self.h - h) / 2.0, w, h)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Kind {
    /// A rotary knob, labelled below (or to the left when `inline`).
    Knob { size: f32, bipolar: bool, inline: bool },
    /// A vertical fader.
    Fader,
    /// A latching button with an LED.
    Toggle,
    /// ‹ value ›: the arrows step, the value opens the full list.
    Stepper,
    /// value ▾: opens the full list.
    Dropdown,
    /// A segment's curve (Raycaster's curve button): a tiny picture of the
    /// bow, dragged up or down to bend it; `rising` is which way the segment runs.
    Curve { rising: bool },
}

/// The prefix of the Modulations panel's controls: "mod.*.lfo" is the LFO
/// amount of whichever knob is selected (the panel fills in the knob).
pub const SELECTED_MOD: &str = "mod.*.";

#[derive(Clone, Copy, Debug)]
pub struct Control {
    pub id: &'static str,
    pub label: &'static str,
    pub kind: Kind,
    pub rect: Rect,
    pub show: Show,
}

/// When something on the panel shows: always (None), or while oscillator
/// slot `.0` (0 or 1) is in mode `.1` (SLOT_DUAL or SLOT_WAVETABLE). A slot's
/// plate holds both engines' controls in the same place.
pub type Show = Option<(usize, usize)>;
pub const SLOT_DUAL: usize = 0;
pub const SLOT_WAVETABLE: usize = 1;
pub const SLOT_RAY: usize = 2;
/// The slots' mode parameters, by slot.
pub const SLOT_MODES: [&str; 2] = ["osc_1.mode", "osc_2.mode"];
/// Ray's Element menu: Chaos's value.
pub const RAY_CHAOS: f32 = 6.0;
/// Parameters the panel sets but doesn't show: each Ray's Chaos seed (a new
/// one is rolled with every click on Chaos).
pub const UNSHOWN: [&str; 2] = ["ray_1.chaos_seed", "ray_2.chaos_seed"];

/// Things showing under `a` and under `b` can be on the panel together.
pub fn together(a: Show, b: Show) -> bool {
    match (a, b) {
        (Some((sa, ma)), Some((sb, mb))) => sa != sb || ma == mb,
        _ => true,
    }
}

/// What a screen shows (drawn from the current parameter values).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Screen {
    /// Oscillator slot k's output: live while a note plays, otherwise
    /// rendered offline (`scope`).
    Scope(usize),
    /// The multidrive's current transfer curve (top bar).
    DriveCurve,
    /// Slot k's Wavetable's wave n (0..3), through its OscMod.
    Wave(usize, usize),
    Noise,
    Transient,
    Filter,
    /// The amp envelope's shape.
    Amp,
    /// The selected knob's LFO (and, live, where it has the knob).
    ModLfo,
    /// The selected knob's envelope.
    ModEnv,
    /// Every modulation that's set, as text (click a line to select its knob).
    ModList,
}

#[derive(Clone, Debug)]
pub struct Plate {
    pub title: &'static str,
    pub rect: Rect,
}

/// Static text placed on a plate (captions, legends).
#[derive(Clone, Debug)]
pub struct Caption {
    pub text: &'static str,
    pub x: f32,
    pub y: f32,
    pub show: Show,
}

/// What a momentary button does.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Action {
    /// Copy the selected knob's modulators...
    CopyMods,
    /// ... onto the selected knob's (greyed until something's copied).
    PasteMods,
    /// Clear the selected knob's modulation amounts.
    ClearMods,
    /// Move slot k's Wavetable's four waves together, one wave along the library.
    ShiftWaves { slot: usize, up: bool },
    /// Roll a new patch.
    Dice,
    /// Go back to the patch before the last roll.
    DiceBack,
    /// Open or close the settings overlay.
    Settings,
}

#[derive(Clone, Copy, Debug)]
pub struct Button {
    pub action: Action,
    pub label: &'static str,
    pub rect: Rect,
    pub show: Show,
}

#[derive(Clone, Debug, Default)]
pub struct Layout {
    /// Momentary buttons (not parameters).
    pub actions: Vec<Button>,
    pub top_bar: Rect,
    /// The amber readout in the top bar (last touched control).
    pub readout: Rect,
    pub plates: Vec<Plate>,
    pub screens: Vec<(Screen, Rect, Show)>,
    pub captions: Vec<Caption>,
    pub controls: Vec<Control>,
    /// The settings overlay (shown from the ☰ button)...
    pub settings: Rect,
    /// ... and where its controls start in `controls` (they run to the end,
    /// and are drawn and hit only while it's open).
    pub settings_from: usize,
    /// Where the Modulations panel names the selected knob.
    pub mod_title: Rect,
    /// What's being built shows when (see `Show`).
    show: Show,
}

impl Layout {
    /// The controls on the panel itself, or (`overlay`) those in the settings.
    pub fn range(&self, overlay: bool) -> std::ops::Range<usize> {
        if overlay { self.settings_from..self.controls.len() } else { 0..self.settings_from }
    }
}

/// A knob's cell (the dial plus room for its label); small knobs get a
/// narrower cell so they fit under a fader.
pub fn knob_cell(size: f32) -> (f32, f32) {
    let min_width = if size < 40.0 { 44.0 } else { 52.0 };
    (size.max(min_width), size + 30.0)
}
const SMALL_KNOB: f32 = 36.0;
pub const FADER_CELL: (f32, f32) = (44.0, 0.0);
pub const TOGGLE_H: f32 = 28.0;
/// A curve button's side.
pub const CURVE_SIZE: f32 = 14.0;

type KnobSpec = (&'static str, &'static str, f32, bool);

impl Layout {
    fn control(&mut self, id: &'static str, label: &'static str, kind: Kind, rect: Rect) {
        self.controls.push(Control { id, label, kind, rect, show: self.show });
    }

    fn action(&mut self, action: Action, label: &'static str, rect: Rect) {
        self.actions.push(Button { action, label, rect, show: self.show });
    }

    fn knob(&mut self, (id, label, size, bipolar): KnobSpec, x: f32, y: f32) {
        let (w, h) = knob_cell(size);
        self.control(id, label, Kind::Knob { size, bipolar, inline: false }, Rect::new(x, y, w, h));
    }

    /// Knobs spread across `area` (CSS `justify-content: space-between`;
    /// a single knob is centred), aligned to the area's top.
    fn knob_row(&mut self, area: Rect, knobs: &[KnobSpec]) {
        let widths: Vec<f32> = knobs.iter().map(|k| knob_cell(k.2).0).collect();
        let used: f32 = widths.iter().sum();
        let (mut x, gap) = match knobs.len() {
            1 => (area.x + (area.w - used) / 2.0, 0.0),
            n => (area.x, (area.w - used) / (n as f32 - 1.0)),
        };
        for (k, w) in knobs.iter().zip(widths) {
            self.knob(*k, x, area.y);
            x += w + gap;
        }
    }

    /// The tallest knob cell in a row.
    fn row_height(knobs: &[KnobSpec]) -> f32 {
        knobs.iter().map(|k| knob_cell(k.2).1).fold(0.0, f32::max)
    }

    fn plate(&mut self, title: &'static str, rect: Rect) -> Rect {
        self.plates.push(Plate { title, rect });
        rect.inset(14.0, 10.0, 14.0, 12.0)
    }

    /// Header with the title on the left and a stepper on the right.
    fn header_stepper(&mut self, inner: Rect, id: &'static str, label: &'static str, width: f32) -> Rect {
        let (header, body) = inner.take_top(HEADER, INNER_GAP);
        let (stepper, _) = header.take_right(width, 0.0);
        self.control(id, label, Kind::Stepper, stepper);
        body
    }

    fn toggle(&mut self, id: &'static str, label: &'static str, x: f32, y: f32, w: f32) {
        self.control(id, label, Kind::Toggle, Rect::new(x, y, w, TOGGLE_H));
    }

    fn screen(&mut self, screen: Screen, rect: Rect) {
        self.screens.push((screen, rect, self.show));
    }

    fn caption(&mut self, text: &'static str, x: f32, y: f32) {
        self.captions.push(Caption { text, x, y, show: self.show });
    }

    /// The whole panel.
    pub fn build() -> Self {
        let mut l = Layout::default();
        let content = Rect::new(CHEEK, 0.0, WIDTH - 2.0 * CHEEK, HEIGHT).inset(20.0, 18.0, 20.0, 18.0);
        let (top, rest) = content.take_top(60.0, GAP);
        l.top_bar(top);
        let row_h = (rest.h - 2.0 * GAP) / 3.0;
        let (row1, rest) = rest.take_top(row_h, GAP);
        let (row2, row3) = rest.take_top(row_h, GAP);
        // Top: Osc 1, then the selected knob's modulators.
        let c = row1.columns(&[580.0, 922.0], GAP);
        l.oscillator(0, c[0]);
        l.modulations(c[1]);
        // Middle: Osc 2, then noise, transient and the mix.
        let c = row2.columns(&[580.0, 330.0, 250.0, 286.0], GAP);
        l.oscillator(1, c[0]);
        l.noise(c[1]);
        l.transient(c[2]);
        l.mix(c[3]);
        // Bottom: tone and shaping.
        let c = row3.columns(&[500.0, 360.0, 314.0, 300.0], GAP);
        l.filter(c[0]);
        l.amp(c[1]);
        l.perform(c[2]);
        l.texture(c[3]);
        l.settings_overlay();
        l
    }

    /// Under-the-hood settings, floating below the ☰ button. Built last, so
    /// its controls end the list.
    fn settings_overlay(&mut self) {
        self.settings_from = self.controls.len();
        let button = self.actions.iter().find(|b| b.action == Action::Settings).unwrap().rect;
        let rect = Rect::new(button.x, self.top_bar.bottom() + 8.0, 300.0, 150.0);
        self.settings = rect;
        let body = rect.inset(14.0, 40.0, 14.0, 12.0);
        self.knob_row(body, &[("settings.heat", "Heat", 52.0, false),
                              ("settings.drive_grit", "Drive Grit", 52.0, false),
                              ("settings.vintage_noise", "Vtg Noise", 52.0, false)]);
    }

    /// The readout on the left; unison, vintage, portamento (0 = poly),
    /// drive (with its curve) and volume on the right.
    fn top_bar(&mut self, bar: Rect) {
        self.top_bar = bar;
        // ☰ in the corner, then the readout.
        let settings = Rect::new(bar.x + 14.0, bar.y + 12.0, 30.0, 36.0);
        self.action(Action::Settings, "", settings);
        self.readout = Rect::new(settings.right() + 10.0, bar.y + 13.0, 352.0, 34.0);
        // The dice, then the step back from them.
        let dice = Rect::new(self.readout.right() + 10.0, bar.y + 12.0, 40.0, 36.0);
        let back = Rect::new(dice.right() + 4.0, dice.y, 26.0, dice.h);
        self.action(Action::Dice, "", dice);
        self.action(Action::DiceBack, "‹", back);
        let size = 40.0;
        let inline = Kind::Knob { size, bipolar: false, inline: true };
        let mut right = bar.right() - 14.0;
        let mut take = |width: f32| {
            right -= width;
            let x = right;
            right -= 18.0;
            x
        };
        // Right to left: volume, the drive's curve screen, drive, porta,
        // vintage, detune; then the unison buttons.
        let volume = take(150.0);
        let curve = take(60.0);
        let drive = take(120.0);
        let porta = take(130.0);
        let vintage = take(140.0);
        let detune = take(160.0);
        for (id, label, x, width) in [("master.volume", "Volume", volume, 150.0), ("drive.amount", "Drive", drive, 120.0),
                                      ("master.portamento", "Porta", porta, 130.0),
                                      ("vintage.amount", "Vintage", vintage, 140.0),
                                      ("unison.detune", "Detune", detune, 160.0)] {
            self.control(id, label, inline, Rect::new(x, bar.y + 10.0, width, size));
        }
        self.screen(Screen::DriveCurve, Rect::new(curve, bar.y + 10.0, 60.0, size));
        let mut right = detune - 18.0;
        for (id, label) in [("unison.random_phase", "Rnd"), ("unison.mode", "Uni")] {
            right -= 56.0;
            self.toggle(id, label, right, bar.y + (bar.h - TOGGLE_H) / 2.0, 56.0);
            right -= 6.0;
        }
    }

    // ---- Plates, one method each; `build` arranges them in rows ----------

    /// Oscillator slot `slot` (0 or 1): its title is a picker (Dual,
    /// Wavetable or Ray); the plate holds every engine's controls, showing
    /// the one it plays.
    fn oscillator(&mut self, slot: usize, rect: Rect) {
        let body = self.plate("", rect);
        let (header, body) = body.take_top(HEADER, INNER_GAP);
        self.caption(["Osc 1", "Osc 2"][slot], header.x, header.y + HEADER / 2.0);
        self.control(SLOT_MODES[slot], "Engine", Kind::Stepper, Rect::new(header.x + 46.0, header.y, 128.0, HEADER));
        let rest = header.inset(192.0, 0.0, 0.0, 0.0);
        self.show = Some((slot, SLOT_DUAL));
        self.dual(slot, rest, body);
        self.show = Some((slot, SLOT_WAVETABLE));
        self.wavetable(slot, rest, body);
        self.show = Some((slot, SLOT_RAY));
        self.ray(slot, rest, body);
        self.show = None;
    }

    /// A Ray (Raycaster's oscillator), laid out as a Wavetable: its scope over
    /// the element picker, then the shape's eight knobs; Free in the header.
    fn ray(&mut self, slot: usize, header: Rect, body: Rect) {
        const IDS: [[&str; 10]; 2] = [
            ["ray_1.free", "ray_1.element", "ray_1.squish", "ray_1.skew", "ray_1.tilt", "ray_1.center", "ray_1.ray",
             "ray_1.orbit", "ray_1.orbit_rate", "ray_1.fine"],
            ["ray_2.free", "ray_2.element", "ray_2.squish", "ray_2.skew", "ray_2.tilt", "ray_2.center", "ray_2.ray",
             "ray_2.orbit", "ray_2.orbit_rate", "ray_2.fine"],
        ];
        let k = |name: &str| -> &'static str {
            IDS[slot].iter().find(|id| id.ends_with(&format!(".{name}"))).copied().expect("a Ray control")
        };
        let (free, _) = header.take_right(84.0, 0.0);
        self.toggle(k("free"), "Free", free.x, header.y - 1.0, free.w);

        let (left, grid) = body.take_left(190.0, 16.0);
        let (picker, scope) = left.take_bottom(HEADER, INNER_GAP);
        self.screen(Screen::Scope(slot), scope);
        self.control(k("element"), "Element", Kind::Dropdown, picker);
        let knob = |name: &str, label: &'static str, bipolar: bool| -> KnobSpec { (k(name), label, 52.0, bipolar) };
        let rows = [[knob("squish", "Squish", true), knob("skew", "Skew", true), knob("tilt", "Tilt", true),
                     knob("center", "Center", true)],
                    [knob("ray", "Ray", false), knob("orbit", "Orbit", false), knob("orbit_rate", "Rate", false),
                     knob("fine", "Fine", true)]];
        let (top, bottom) = grid.take_top((grid.h - 10.0) / 2.0, 10.0);
        for (half, knobs) in [top, bottom].into_iter().zip(rows) {
            let h = Self::row_height(&knobs);
            self.knob_row(Rect::new(half.x + 10.0, half.y + (half.h - h) / 2.0, half.w - 20.0, h), &knobs);
        }
    }

    /// A Dual: its scope on the left; on the right its algorithm, its two
    /// waves, then A, B and Semi.
    fn dual(&mut self, slot: usize, _header: Rect, body: Rect) {
        let ids = [["dual_1.algorithm", "dual_1.wave_1", "dual_1.wave_2", "dual_1.a", "dual_1.b", "dual_1.semi"],
                   ["dual_2.algorithm", "dual_2.wave_1", "dual_2.wave_2", "dual_2.a", "dual_2.b", "dual_2.semi"]][slot];
        let (scope, right) = body.take_left(296.0, 18.0);
        self.screen(Screen::Scope(slot), scope);
        let (caption, rest) = right.take_top(12.0, 4.0);
        self.caption("Algorithm", caption.x, caption.y + 6.0);
        let (algorithm, rest) = rest.take_top(26.0, 10.0);
        self.control(ids[0], "Algorithm", Kind::Stepper, algorithm);
        let (captions, rest) = rest.take_top(12.0, 4.0);
        let (waves, rest) = rest.take_top(26.0, 0.0);
        for (k, (cell, label)) in waves.columns(&[1.0, 1.0], 10.0).into_iter().zip(["Wave 1", "Wave 2"]).enumerate() {
            self.caption(label, cell.x, captions.y + 6.0);
            self.control(ids[1 + k], label, Kind::Stepper, cell);
        }
        let knobs = [(ids[3], "A", 52.0, false), (ids[4], "B", 52.0, false), (ids[5], "Semi", 44.0, true)];
        let (row, _) = rest.take_bottom(Self::row_height(&knobs), 0.0);
        self.knob_row(row.inset(8.0, 0.0, 8.0, 0.0), &knobs);
    }

    /// A Wavetable: its scope (narrow) over
    /// Position and OscMod Depth, then its four waves two by two. The header
    /// moves all four waves together and picks the OscMod.
    fn wavetable(&mut self, slot: usize, header: Rect, body: Rect) {
        let ids = [["wt_1.oscmod", "wt_1.position", "wt_1.oscmod_depth", "wt_1.wave_1", "wt_1.wave_2", "wt_1.wave_3", "wt_1.wave_4"],
                   ["wt_2.oscmod", "wt_2.position", "wt_2.oscmod_depth", "wt_2.wave_1", "wt_2.wave_2", "wt_2.wave_3", "wt_2.wave_4"]][slot];
        // Arrows that move all the waves together, then the OscMod.
        self.caption("All waves", header.x, header.y + HEADER / 2.0);
        for (up, label, offset) in [(false, "‹", 56.0), (true, "›", 86.0)] {
            self.action(Action::ShiftWaves { slot, up }, label, Rect::new(header.x + offset, header.y, 26.0, HEADER));
        }
        let (oscmod, rest) = header.take_right(144.0, 8.0);
        self.control(ids[0], "OscMod", Kind::Stepper, oscmod);
        self.caption("OscMod", rest.right() - 46.0, header.y + HEADER / 2.0);

        let (left, grid) = body.take_left(190.0, 16.0);
        let knobs = [(ids[1], "Position", 52.0, false), (ids[2], "OscMod Depth", 52.0, false)];
        let (row, scope) = left.take_bottom(Self::row_height(&knobs), INNER_GAP);
        self.screen(Screen::Scope(slot), scope);
        self.knob_row(row.inset(10.0, 0.0, 10.0, 0.0), &knobs);
        let (top, bottom) = grid.take_top((grid.h - 10.0) / 2.0, 10.0);
        let mut k = 0;
        for half in [top, bottom] {
            for cell in half.columns(&[1.0, 1.0], 12.0) {
                let (stepper, screen) = cell.take_bottom(26.0, 4.0);
                self.screen(Screen::Wave(slot, k), screen);
                self.control(ids[3 + k], ["Wave 1", "Wave 2", "Wave 3", "Wave 4"][k], Kind::Stepper, stepper);
                k += 1;
            }
        }
    }

    /// The selected knob's modulators: its LFO, its envelope, and how far
    /// velocity, aftertouch, the mod wheel and the key move it. Every control
    /// here is the selected knob's ("mod.*.<field>").
    fn modulations(&mut self, rect: Rect) {
        let body = self.plate("Modulations", rect);
        let (header, body) = body.take_top(HEADER, INNER_GAP);
        let mut title = header;
        for (action, label) in [(Action::ClearMods, "clear"), (Action::PasteMods, "paste"), (Action::CopyMods, "copy")] {
            let (button, rest) = title.take_right(64.0, 6.0);
            self.action(action, label, Rect::new(button.x, header.y - 1.0, button.w, TOGGLE_H));
            title = rest;
        }
        self.mod_title = title.inset(118.0, 0.0, 6.0, 0.0);
        let c = body.columns(&[366.0, 306.0, 196.0], 24.0);
        let knob = |id: &'static str, label: &'static str, bipolar: bool| -> KnobSpec { (id, label, 44.0, bipolar) };
        let row_h = knob_cell(44.0).1;

        // LFO: its wave, then shape, sync and retrigger, then rate and amount.
        let (caption, lfo) = c[0].take_top(12.0, 6.0);
        self.caption("LFO", caption.x, caption.y + 6.0);
        let (knobs, lfo) = lfo.take_bottom(row_h, INNER_GAP);
        let (buttons, screen) = lfo.take_bottom(TOGGLE_H, INNER_GAP);
        self.screen(Screen::ModLfo, screen);
        let (shape, buttons) = buttons.take_left(150.0, 10.0);
        self.control("mod.*.shape", "Shape", Kind::Stepper, shape);
        let (sync, buttons) = buttons.take_left(70.0, 8.0);
        self.toggle("mod.*.sync", "Sync", sync.x, sync.y, sync.w);
        let (retrig, _) = buttons.take_left(78.0, 0.0);
        self.toggle("mod.*.retrig", "Retrig", retrig.x, retrig.y, retrig.w);
        // Under the shape and the buttons, as the envelope's under its screen.
        let row = Rect::new(knobs.x, knobs.y, retrig.right() - knobs.x, knobs.h);
        self.knob_row(row.inset(30.0, 0.0, 30.0, 0.0), &[knob("mod.*.rate", "Rate", false), knob("mod.*.lfo", "Amount", true)]);

        // Envelope: its shape, then A, D, S, R and amount.
        let (caption, env) = c[1].take_top(12.0, 6.0);
        self.caption("Envelope", caption.x, caption.y + 6.0);
        let (knobs, screen) = env.take_bottom(row_h, INNER_GAP);
        self.screen(Screen::ModEnv, screen);
        self.knob_row(knobs, &[knob("mod.*.attack", "Attack", false), knob("mod.*.decay", "Decay", false),
                               knob("mod.*.sustain", "Sustain", false), knob("mod.*.release", "Release", false),
                               knob("mod.*.env", "Amount", true)]);
        for (id, of, rising) in [("mod.*.attack_curve", "mod.*.attack", true), ("mod.*.decay_curve", "mod.*.decay", false),
                                 ("mod.*.release_curve", "mod.*.release", false)] {
            self.knob_curve(id, of, 44.0, rising);
        }

        // Played: velocity, aftertouch, mod wheel, key; two rows of two, each
        // with what it moves below it (the knob, or the LFO's or envelope's amount).
        let (caption, hands) = c[2].take_top(12.0, 6.0);
        self.caption("Played", caption.x, caption.y + 6.0);
        const DEST_W: f32 = 86.0;
        let columns = [hands.x, hands.right() - DEST_W];
        let rows = [[("mod.*.vel", "Velocity", "mod.*.vel_to"), ("mod.*.touch", "Aftertouch", "mod.*.touch_to")],
                    [("mod.*.wheel", "Mod Wheel", "mod.*.wheel_to"), ("mod.*.key", "Key", "mod.*.key_to")]];
        let pair_h = row_h + 22.0;
        let mut y = hands.bottom() - 2.0 * pair_h - 6.0;
        for row in rows {
            for (x, (id, label, to)) in columns.into_iter().zip(row) {
                let (w, _) = knob_cell(44.0);
                self.knob(knob(id, label, true), x + (DEST_W - w) / 2.0, y);
                self.control(to, label, Kind::Dropdown, Rect::new(x, y + row_h, DEST_W, 20.0));
            }
            y += pair_h + 6.0;
        }
    }

    fn noise(&mut self, rect: Rect) {
        let body = self.plate("Noise", rect);
        let body = self.header_stepper(body, "noise.type", "Type", 110.0);
        let knobs = [("noise.bits", "Bits", 44.0, false),
                     ("noise.crackle_density", "Density", 44.0, false), ("noise.tilt", "Tilt", 44.0, true)];
        let (row, body) = body.take_bottom(Self::row_height(&knobs), INNER_GAP);
        self.knob_row(row, &knobs);
        self.screen(Screen::Noise, body);
    }

    /// Slim: its level lives in the Mix plate.
    fn transient(&mut self, rect: Rect) {
        let body = self.plate("Transient", rect);
        let (_, body) = body.take_top(HEADER, INNER_GAP);
        let (stepper, body) = body.take_top(HEADER, INNER_GAP);
        self.control("transient.type", "Type", Kind::Stepper, stepper);
        let knobs = [("transient.decay", "Decay", 44.0, false), ("transient.pitch", "Pitch", 44.0, true)];
        let (row, body) = body.take_bottom(Self::row_height(&knobs), INNER_GAP);
        self.knob_row(row, &knobs);
        let (post, body) = body.take_bottom(TOGGLE_H, INNER_GAP);
        self.toggle("transient.post_filter", "Post", post.x, post.y, post.w);
        self.screen(Screen::Transient, body);
    }

    /// A fader per source (the two oscillator slots, the noise), its octave
    /// (Noise: clock) knob below; the transient's level too, though
    /// it's outside the balance. The slots' Modes are their titles' pickers.
    fn mix(&mut self, rect: Rect) {
        let body = self.plate("Mix", rect);
        let (_, body) = body.take_top(HEADER, INNER_GAP);
        let faders = [("mix.osc_1", "Osc 1"), ("mix.osc_2", "Osc 2"), ("mix.noise", "Noise"), ("transient.level", "Trans")];
        let below = [("osc_1.octave", "Octave"), ("osc_2.octave", "Octave"), ("noise.clock_ratio", "Clock")];
        let (knob_row, fader_area) = body.take_bottom(knob_cell(SMALL_KNOB).1, 4.0);
        self.fader_row(fader_area, &faders);
        let columns: Vec<Rect> = self.controls[self.controls.len() - faders.len()..].iter().map(|c| c.rect).collect();
        for (column, (id, label)) in columns.iter().zip(below) {
            let (w, _) = knob_cell(SMALL_KNOB);
            self.knob((id, label, SMALL_KNOB, true), column.x + (column.w - w) / 2.0, knob_row.y);
        }
    }

    fn filter(&mut self, rect: Rect) {
        let body = self.plate("Filter", rect);
        let (header, body) = body.take_top(HEADER, INNER_GAP);
        let (pole, rest) = header.take_right(74.0, 8.0);
        self.control("filter.4_pole", "4-Pole", Kind::Toggle, Rect::new(pole.x, header.y - 1.0, pole.w, TOGGLE_H));
        let (kind, _) = rest.take_right(112.0, 0.0);
        self.control("filter.type", "Type", Kind::Stepper, kind);
        let (screen, grid) = body.take_left(232.0, 16.0);
        self.screen(Screen::Filter, screen);
        // Its envelope and key tracking are the Cutoff's modulators.
        let rows = [
            [("filter.cutoff", "Cutoff", 56.0, false), ("filter.resonance", "Resonance", 48.0, false),
             ("filter.utility_hpf", "HPF", 44.0, false)],
            [("filter.morph_lp_notch_hp", "Morph", 44.0, false), ("filter.bandpass", "Bandpass", 44.0, false),
             ("", "", 0.0, false)],
        ];
        let cells = grid.columns(&[1.0; 3], 6.0);
        let row_h = grid.h / 2.0;
        for (r, knobs) in rows.iter().enumerate() {
            for (cell, k) in cells.iter().zip(knobs) {
                if k.0.is_empty() {
                    continue;
                }
                let (w, h) = knob_cell(k.2);
                let y = grid.y + r as f32 * row_h + (row_h - h) / 2.0;
                self.knob(*k, cell.x + (cell.w - w) / 2.0, y);
            }
        }
    }

    fn fader_row(&mut self, area: Rect, faders: &[(&'static str, &'static str)]) {
        let w = FADER_CELL.0;
        let gap = (area.w - w * faders.len() as f32) / (faders.len() as f32 - 1.0);
        for (i, (id, label)) in faders.iter().enumerate() {
            let rect = Rect::new(area.x + i as f32 * (w + gap), area.y, w, area.h);
            self.control(id, label, Kind::Fader, rect);
        }
    }

    /// The amp envelope (not modulatable), with a curve button beside each
    /// timed segment's fader.
    fn amp(&mut self, rect: Rect) {
        let body = self.plate("Amp", rect);
        let (_, body) = body.take_top(HEADER, INNER_GAP);
        let (faders, screen) = body.take_bottom(140.0, INNER_GAP);
        self.screen(Screen::Amp, screen);
        let faders = faders.inset(0.0, 0.0, CURVE_SIZE + 4.0, 0.0);
        self.fader_row(faders, &[("amp.attack", "Attack"), ("amp.decay", "Decay"), ("amp.sustain", "Sustain"),
                                 ("amp.release", "Release")]);
        for (id, of, rising) in [("amp.attack_curve", "amp.attack", true), ("amp.decay_curve", "amp.decay", false),
                                 ("amp.release_curve", "amp.release", false)] {
            let cell = self.rect_of(of);
            self.control(id, "Curve", Kind::Curve { rising }, Rect::new(cell.right() + 2.0, cell.y + 2.0, CURVE_SIZE, CURVE_SIZE));
        }
    }

    /// The rectangle of the control already placed for `id`.
    fn rect_of(&self, id: &str) -> Rect {
        self.controls.iter().find(|c| c.id == id).unwrap_or_else(|| panic!("no control '{id}'")).rect
    }

    /// A curve button at the top right of knob `of`'s dial.
    fn knob_curve(&mut self, id: &'static str, of: &str, size: f32, rising: bool) {
        let cell = self.rect_of(of);
        let (cx, r) = (cell.x + cell.w / 2.0, size / 2.0 - 3.0);
        self.control(id, "Curve", Kind::Curve { rising },
                     Rect::new(cx + 0.62 * r + 2.0, cell.y - 2.0, CURVE_SIZE, CURVE_SIZE));
    }

    /// Events, calls and the noise's wander, a row each, named at the left
    /// above its Mute.
    fn texture(&mut self, rect: Rect) {
        let body = self.plate("Texture", rect);
        let (_, body) = body.take_top(HEADER, 2.0);
        let size = 44.0;
        let (w, h) = knob_cell(size);
        let rows: [(&'static str, &'static str, &[(&'static str, &'static str)]); 3] = [
            ("Events", "events.mute", &[("events.density", "Density"), ("events.depth", "Depth"), ("events.life", "Life"), ("events.sharp", "Sharp")]),
            ("Calls", "calls.mute", &[("calls.level", "Level"), ("calls.activity", "Activity"), ("calls.rate", "Rate"), ("calls.echo", "Echo")]),
            ("Noise", "noise.mute", &[("noise.move", "Move"), ("noise.rate", "Rate")]),
        ];
        let (names, knobs) = body.take_left(52.0, 4.0);
        let cells = knobs.columns(&[1.0; 4], 2.0);
        for (r, (name, mute, row)) in rows.into_iter().enumerate() {
            let y = body.y + r as f32 * h;
            self.caption(name, names.x, y + 8.0);
            self.toggle(mute, "Mute", names.x, y + 20.0, names.w);
            for (cell, (id, label)) in cells.iter().zip(row) {
                self.knob((id, label, size, false), cell.x + (cell.w - w) / 2.0, y);
            }
        }
    }

    /// Global Pitch (modulatable: it starts as vibrato on the mod wheel), what
    /// pitch bend does, and the list of every modulation that's set, its top
    /// in line with the Filter's and Amp's screens.
    fn perform(&mut self, rect: Rect) {
        let body = self.plate("Perform", rect);
        let (_, body) = body.take_top(HEADER, INNER_GAP);
        let (list, left) = body.take_right(152.0, 14.0);
        self.screen(Screen::ModList, list);
        self.knob(("perform.global_pitch", "Global Pitch", 52.0, true), left.x + (left.w - knob_cell(52.0).0) / 2.0, left.y + 8.0);
        let (bend, rest) = left.take_bottom(26.0, 0.0);
        self.caption("Pitch bend", rest.x, rest.bottom() - 12.0);
        self.control("perform.bend_mode", "Pitch bend", Kind::Dropdown, bend);
    }
}

/// The selected-knob field a Modulations panel control sets ("lfo" for
/// "mod.*.lfo"), if it's one of those.
pub fn selected_mod_field(id: &str) -> Option<&str> {
    id.strip_prefix(SELECTED_MOD)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::params::SynthParams;

    /// Every host parameter has exactly one control, and every control
    /// names a real parameter.
    #[test]
    fn every_param_is_on_the_panel_once() {
        let params = SynthParams::from_faust();
        let layout = Layout::build();
        let on_panel: Vec<&str> = layout.controls.iter().map(|c| c.id).collect();
        // The Modulations panel shows the selected knob's: one control per field.
        let fields: Vec<&str> = on_panel.iter().filter_map(|id| selected_mod_field(id)).collect();
        for id in on_panel.iter().filter(|id| selected_mod_field(id).is_none()) {
            assert!(params.ptr(id).is_some(), "panel control '{id}' has no parameter");
        }
        // A Ray's Chaos seed isn't shown.
        for id in params.ids().filter(|id| !UNSHOWN.contains(id)) {
            let n = match id.strip_prefix("mod.").and_then(|r| r.rsplit_once('.')) {
                // "division" is the Rate knob's while the LFO is synced.
                Some((_, field)) => fields.iter().filter(|f| **f == field || (field == "division" && **f == "rate")).count(),
                None => on_panel.iter().filter(|c| **c == id).count(),
            };
            assert_eq!(n, 1, "parameter '{id}' appears {n} times on the panel");
        }
        // Every modulatable knob is on the panel, to be selected.
        for t in crate::modulation::TARGETS {
            assert!(on_panel.contains(&t.id), "'{}' isn't on the panel", t.id);
        }
    }

    /// Momentary buttons don't overlap the controls, screens or readout, or each other.
    #[test]
    fn buttons_are_clear() {
        let layout = Layout::build();
        let overlap = |a: Rect, b: Rect| {
            a.x < b.right() - 0.5 && b.x < a.right() - 0.5 && a.y < b.bottom() - 0.5 && b.y < a.bottom() - 0.5
        };
        for (i, b) in layout.actions.iter().enumerate() {
            assert!(!overlap(b.rect, layout.readout), "{:?} overlaps the readout", b.action);
            for c in layout.controls[layout.range(false)].iter().filter(|c| together(b.show, c.show)) {
                assert!(!overlap(b.rect, c.rect), "{:?} overlaps {}", b.action, c.id);
            }
            for (s, r, _) in layout.screens.iter().filter(|(_, _, show)| together(b.show, *show)) {
                assert!(!overlap(b.rect, *r), "{:?} overlaps screen {s:?}", b.action);
            }
            for other in layout.actions[i + 1..].iter().filter(|o| together(b.show, o.show)) {
                assert!(!overlap(b.rect, other.rect), "{:?} overlaps {:?}", b.action, other.action);
            }
        }
    }

    /// Controls stay inside their plate and don't overlap each other.
    #[test]
    fn controls_fit_and_dont_overlap() {
        let layout = Layout::build();
        let panel = Rect::new(0.0, 0.0, WIDTH, HEIGHT);
        let within = |inner: Rect, outer: Rect| {
            inner.x >= outer.x - 0.5 && inner.y >= outer.y - 0.5
                && inner.right() <= outer.right() + 0.5 && inner.bottom() <= outer.bottom() + 0.5
        };
        let overlap = |a: Rect, b: Rect| {
            a.x < b.right() - 0.5 && b.x < a.right() - 0.5 && a.y < b.bottom() - 0.5 && b.y < a.bottom() - 0.5
        };
        for c in &layout.controls {
            let home = layout.plates.iter().map(|p| p.rect).chain([layout.top_bar, layout.settings])
                .find(|r| within(c.rect, *r));
            assert!(home.is_some() && within(c.rect, panel), "{} sticks out of its plate: {:?}", c.id, c.rect);
        }
        // The overlay floats over the panel: each layer is checked on its own.
        for overlay in [false, true] {
            let layer = &layout.controls[layout.range(overlay)];
            for (i, a) in layer.iter().enumerate() {
                // A slot's Dual and Wavetable share its plate: only what shows together counts.
                // A curve button may sit on its own knob's cell (it's hit first),
                // but not on another curve button.
                let curve = |c: &Control| matches!(c.kind, Kind::Curve { .. });
                for b in layer[i + 1..].iter().filter(|b| together(a.show, b.show) && curve(a) == curve(b)) {
                    assert!(!overlap(a.rect, b.rect), "{} overlaps {}", a.id, b.id);
                }
                for (s, r, _) in layout.screens.iter().filter(|(_, _, show)| !overlay && together(a.show, *show)) {
                    assert!(!overlap(a.rect, *r), "{} overlaps screen {s:?}", a.id);
                }
            }
        }
        for c in &layout.controls[layout.range(true)] {
            assert!(within(c.rect, layout.settings), "{} sticks out of the settings", c.id);
        }
    }
}

#[cfg(test)]
mod print {
    /// Prints control cells (in panel pixels): `cargo test print_cells -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn print_cells() {
        for c in super::Layout::build().controls {
            println!("{}\t{:.1}\t{:.1}\t{:.1}\t{:.1}", c.id, c.rect.x, c.rect.y, c.rect.w, c.rect.h);
        }
    }
}
