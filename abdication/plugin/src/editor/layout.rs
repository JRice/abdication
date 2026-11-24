//! Where everything goes, in logical pixels on a 1600 x 1000 panel. The
//! editor scales the whole panel uniformly, so these numbers never change
//! with the window size. The arrangement follows the design mockup (Nov
//! 2026, "Abdication — new panel mockup"): a top bar, then three rows of
//! plates: the two oscillator slots; the mix, noise, transient and filter;
//! the modulators (envelopes, LFOs, performance sources) and the Harmonic EQ.

pub const WIDTH: f32 = 1600.0;
pub const HEIGHT: f32 = 1000.0;
/// Walnut end cheeks.
pub const CHEEK: f32 = 22.0;

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

/// Where a knob's label goes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LabelAt {
    /// Under the dial, the value under it.
    Below,
    /// Under the dial and `dx` to the right (the cutoff's, clear of the HPF's arc).
    BelowRight(f32),
    /// Left of the dial, right-aligned (the top bar's).
    Left,
    /// Right of the dial (the filter's column).
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Kind {
    /// A rotary knob.
    Knob { size: f32, bipolar: bool, label: LabelAt },
    /// A thin vertical fader: the cell is its travel, full height, its label
    /// under it. `tint`: its source's colour (an envelope's), else amber.
    Fader { bipolar: bool, tint: Option<usize>, cap: f32 },
    /// A latching button with an LED.
    Toggle,
    /// ‹ value ›: the arrows step, the value opens the full list.
    Stepper,
    /// value ▾: opens the full list.
    Dropdown,
    /// A segment's curve (Raycaster's curve button): a tiny picture of the
    /// bow, dragged up or down to bend it; `rising` is which way the segment runs.
    Curve { rising: bool },
    /// A small horizontal slider (a Windows interrupt's Pitch, LFO Z's
    /// Depth): not for precise settings; double-click resets it.
    Slider,
    /// An Orthogonal player's sample: a picture of the set it plays, its name
    /// and where in it the player is; a click opens the sets, by category.
    Sample,
    /// A small screen showing a wave (a wavetable's, through its OscMod, or
    /// an LFO's shape); a click opens the wave picker.
    Wave { lfo: bool },
    /// The HPF: an arc round the cutoff's lower left quarter, centred at
    /// (cx, cy), radius r: 6 o'clock off, 9 o'clock 8 kHz. Dragged like a knob.
    Arc { cx: f32, cy: f32, r: f32 },
    /// A row of segments, one per setting (LFO Z's On / MW / AT).
    Segments,
}

#[derive(Clone, Copy, Debug)]
pub struct Control {
    pub id: &'static str,
    pub label: &'static str,
    pub kind: Kind,
    pub rect: Rect,
    pub show: Show,
}

impl Control {
    /// Whether (x, y) is on it: the HPF's arc by its ring, the rest by their cells.
    pub fn hit(&self, x: f32, y: f32) -> bool {
        match self.kind {
            Kind::Arc { cx, cy, r } => {
                let (dx, dy) = (x - cx, y - cy);
                let d = (dx * dx + dy * dy).sqrt();
                (r - 9.0..r + 11.0).contains(&d) && dx <= 4.0 && dy >= -4.0
            }
            _ => self.rect.contains(x, y),
        }
    }
}

/// When something on the panel shows: always (None), or while oscillator
/// slot `.0` (0 or 1) is in mode `.1` (SLOT_DUAL ... SLOT_WINDOWS). A slot's
/// plate holds every engine's controls in the same place.
pub type Show = Option<(usize, usize)>;
pub const SLOT_DUAL: usize = 0;
pub const SLOT_WAVETABLE: usize = 1;
pub const SLOT_RAY: usize = 2;
pub const SLOT_ORTHO: usize = 3;
pub const SLOT_WINDOWS: usize = 4;
/// The engines' names, by mode (the presets' folders).
pub const ENGINES: [&str; 5] = ["Dual", "Wavetable", "Ray", "Orthogonal", "Windows"];
/// Each engine's own controls' prefix, by mode ("wt" for "wt_1.wave_1").
pub const ENGINE_PREFIXES: [&str; 5] = ["dual", "wt", "ray", "ortho", "windows"];
/// A small slider's height (the thumb's travel runs across it).
pub const SLIDER_H: f32 = 12.0;
/// The slots' mode parameters, by slot.
pub const SLOT_MODES: [&str; 2] = ["osc_1.mode", "osc_2.mode"];
/// Ray's Element menu: Chaos's value.
pub const RAY_CHAOS: f32 = 6.0;
/// Parameters the panel sets but doesn't show: each Ray's Chaos seed (a new
/// one is rolled with every click on Chaos); and the synced settings that
/// a rate knob shows in place of its Hz while synced (`synced`).
pub const UNSHOWN: [&str; 7] = ["ray_1.chaos_seed", "ray_2.chaos_seed", "lfo_x.division", "lfo_y.division",
                                "lfo_z.division", "wt_1.speed_division", "wt_2.speed_division"];

/// A rate knob that, while its Sync switch is on, sets a division instead:
/// (its switch, the division's parameter).
pub fn synced(id: &str) -> Option<(&'static str, &'static str)> {
    Some(match id {
        "lfo_x.rate" => ("lfo_x.sync", "lfo_x.division"),
        "lfo_y.rate" => ("lfo_y.sync", "lfo_y.division"),
        "lfo_z.rate" => ("lfo_z.sync", "lfo_z.division"),
        "wt_1.speed" => ("wt_1.speed_sync", "wt_1.speed_division"),
        "wt_2.speed" => ("wt_2.speed_sync", "wt_2.speed_division"),
        _ => return None,
    })
}

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
    /// The multidrive's current transfer curve, and its name under it.
    DriveCurve,
    /// Slot k's Wavetable's path: envelope A's outline under its five
    /// waves, the sustain's bounce, and where the newest voice is.
    WavePath(usize),
    Noise,
    Transient,
    Filter,
    /// A performance source's level now (modulation::SOURCES index).
    Meter(usize),
    /// What the Harmonic EQ does to a slot's harmonics.
    Harmonic,
}

/// Where a plate's coloured strip comes from: its section's colour, or one
/// segment per modulation source.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Strip {
    Section(u32),
    Sources(&'static [usize]),
}

#[derive(Clone, Debug)]
pub struct Plate {
    pub title: &'static str,
    pub rect: Rect,
    pub strip: Strip,
}

/// Static text placed on a plate (captions, legends); `color` a source's
/// shade (modulation::SOURCES index) or None for the captions' grey.
#[derive(Clone, Debug)]
pub struct Caption {
    pub text: &'static str,
    pub x: f32,
    pub y: f32,
    pub show: Show,
    pub color: Option<usize>,
    pub size: f32,
}

/// A source's group on its plate: a faint wash of its shade.
#[derive(Clone, Copy, Debug)]
pub struct Wash {
    pub source: usize,
    pub rect: Rect,
}

/// What a momentary button does.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Action {
    /// A modulation source's assignment button (modulation::SOURCES index):
    /// press it, then drag any knob to set how far the source moves it.
    Assign(usize),
    /// Move slot k's Wavetable's five waves together, one wave along the library.
    ShiftWaves { slot: usize, up: bool },
    /// Slot k's presets for the engine it plays: the list...
    Presets { slot: usize },
    /// ... and saving the engine's settings as one.
    SavePreset { slot: usize },
    /// Reset to the init patch (dice::INIT): defaults, one plain oscillator.
    Init,
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
    /// Where the wordmark is centred.
    pub wordmark: (f32, f32),
    pub plates: Vec<Plate>,
    pub washes: Vec<Wash>,
    pub screens: Vec<(Screen, Rect, Show)>,
    pub captions: Vec<Caption>,
    /// Thin grooves between parts of a plate: (x, y, height).
    pub dividers: Vec<(f32, f32, f32)>,
    pub controls: Vec<Control>,
    /// The settings overlay (shown from the ☰ button)...
    pub settings: Rect,
    /// ... and where its controls start in `controls` (they run to the end,
    /// and are drawn and hit only while it's open).
    pub settings_from: usize,
    /// What's being built shows when (see `Show`).
    show: Show,
}

impl Layout {
    /// The controls on the panel itself, or (`overlay`) those in the settings.
    pub fn range(&self, overlay: bool) -> std::ops::Range<usize> {
        if overlay { self.settings_from..self.controls.len() } else { 0..self.settings_from }
    }
}

/// A knob's cell (the dial plus room for its label below); small knobs get
/// a narrower cell so they fit under a fader.
pub fn knob_cell(size: f32) -> (f32, f32) {
    let min_width = if size < 40.0 { 44.0 } else { 52.0 };
    (size.max(min_width), size + 30.0)
}
pub const TOGGLE_H: f32 = 26.0;
/// A curve button's side.
pub const CURVE_SIZE: f32 = 13.0;
/// An assignment button's side (the performance sources' are a little bigger).
pub const ASSIGN: f32 = 30.0;

type KnobSpec = (&'static str, &'static str, f32, bool);

/// The sections' colours (the plates' strips).
pub const SECTION_OSC: u32 = 0xd4572a;
pub const SECTION_MIX: u32 = 0xe2a32c;
pub const SECTION_FILTER: u32 = 0x2b8f86;
pub const SECTION_HARMONIC: u32 = 0x4d6fb8;

impl Layout {
    fn control(&mut self, id: &'static str, label: &'static str, kind: Kind, rect: Rect) {
        self.controls.push(Control { id, label, kind, rect, show: self.show });
    }

    fn action(&mut self, action: Action, label: &'static str, rect: Rect) {
        self.actions.push(Button { action, label, rect, show: self.show });
    }

    /// A knob whose dial is centred at (cx, cy), labelled below.
    fn knob_at(&mut self, id: &'static str, label: &'static str, size: f32, bipolar: bool, cx: f32, cy: f32) {
        let (w, h) = knob_cell(size);
        self.control(id, label, Kind::Knob { size, bipolar, label: LabelAt::Below },
                     Rect::new(cx - w / 2.0, cy - size / 2.0, w, h));
    }

    /// A knob whose dial is centred at (cx, cy), labelled beside it.
    fn knob_beside(&mut self, id: &'static str, label: &'static str, size: f32, at: LabelAt, cx: f32, cy: f32) {
        let rect = match at {
            LabelAt::Left => Rect::new(cx + size / 2.0 - 118.0, cy - size / 2.0, 118.0, size),
            _ => Rect::new(cx - size / 2.0, cy - size / 2.0, size + 80.0, size),
        };
        self.control(id, label, Kind::Knob { size, bipolar: false, label: at }, rect);
    }

    fn knob(&mut self, (id, label, size, bipolar): KnobSpec, x: f32, y: f32) {
        let (w, _) = knob_cell(size);
        self.knob_at(id, label, size, bipolar, x + w / 2.0, y + size / 2.0);
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

    /// A thin fader whose groove is centred on `cx`, running `h` down from `y`.
    fn fader(&mut self, id: &'static str, label: &'static str, cx: f32, y: f32, h: f32, tint: Option<usize>, cap: f32,
             bipolar: bool) {
        self.control(id, label, Kind::Fader { bipolar, tint, cap }, Rect::new(cx - 10.0, y, 20.0, h));
    }

    fn plate(&mut self, title: &'static str, rect: Rect, strip: Strip) -> Rect {
        self.plates.push(Plate { title, rect, strip });
        rect.inset(14.0, 10.0, 14.0, 12.0)
    }

    /// The plate's body under its header.
    fn body(rect: Rect) -> Rect {
        rect.inset(14.0, 46.0, 14.0, 14.0)
    }

    fn toggle(&mut self, id: &'static str, label: &'static str, x: f32, y: f32, w: f32) {
        self.control(id, label, Kind::Toggle, Rect::new(x, y, w, TOGGLE_H));
    }

    fn screen(&mut self, screen: Screen, rect: Rect) {
        self.screens.push((screen, rect, self.show));
    }

    fn caption(&mut self, text: &'static str, x: f32, y: f32) {
        self.captions.push(Caption { text, x, y, show: self.show, color: None, size: 10.5 });
    }

    fn caption_in(&mut self, text: &'static str, x: f32, y: f32, source: usize, size: f32) {
        self.captions.push(Caption { text, x, y, show: self.show, color: Some(source), size });
    }

    /// The whole panel.
    pub fn build() -> Self {
        let mut l = Layout::default();
        l.top_bar(Rect::new(42.0, 18.0, 1516.0, 60.0));
        let (r1, r2, r3, h) = (92.0, 393.0, 694.0, 287.0);
        // The oscillator slots are the same width: they're interchangeable.
        l.oscillator(0, Rect::new(42.0, r1, 751.0, h));
        l.oscillator(1, Rect::new(807.0, r1, 751.0, h));
        l.mix(Rect::new(42.0, r2, 300.0, h));
        l.noise(Rect::new(356.0, r2, 186.0, h));
        l.transient(Rect::new(556.0, r2, 250.0, h));
        l.filter(Rect::new(820.0, r2, 738.0, h));
        l.envelopes(Rect::new(42.0, r3, 500.0, h));
        l.lfos(Rect::new(556.0, r3, 370.0, h));
        l.perform(Rect::new(940.0, r3, 304.0, h));
        l.harmonic(Rect::new(1258.0, r3, 300.0, h));
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
                              ("settings.vintage_noise", "Vtg Noise", 52.0, false),
                              ("settings.ortho_scatter", "Scatter", 52.0, false)]);
    }

    /// ☰, the readout, init, dice and back on the left; the wordmark; unison,
    /// vintage, portamento (0 = poly) and volume on the right.
    fn top_bar(&mut self, bar: Rect) {
        self.top_bar = bar;
        let y = bar.y + 13.0;
        self.action(Action::Settings, "", Rect::new(bar.x + 18.0, y, 30.0, 34.0));
        self.readout = Rect::new(bar.x + 58.0, y, 318.0, 34.0);
        self.action(Action::Init, "", Rect::new(bar.x + 386.0, y, 30.0, 34.0));
        self.action(Action::Dice, "", Rect::new(bar.x + 420.0, y, 40.0, 34.0));
        self.action(Action::DiceBack, "‹", Rect::new(bar.x + 464.0, y, 26.0, 34.0));
        self.wordmark = (bar.x + 623.0, bar.y + 31.0);
        // Right to left: volume, porta, vintage, spread, detune, then the unison buttons.
        let cy = bar.y + 30.0;
        let mut right = bar.right() - 20.0;
        for (id, label) in [("master.volume", "Volume"), ("master.portamento", "Porta"), ("vintage.amount", "Vintage"),
                            ("unison.spread", "Spread"), ("unison.detune", "Detune")] {
            right -= 112.0;
            self.knob_beside(id, label, 36.0, LabelAt::Right, right + 20.0, cy);
            right -= 6.0;
        }
        for (id, label) in [("unison.eco", "Eco"), ("unison.random_phase", "Rnd"), ("unison.mode", "Uni")] {
            right -= 50.0;
            self.toggle(id, label, right, bar.y + 17.0, 46.0);
        }
        assert!(right >= self.wordmark.0 + 110.0, "the top bar's right side runs into the wordmark");
    }

    // ---- Plates, one method each; `build` arranges them -------------------

    /// Oscillator slot `slot` (0 or 1): its numeral, its engine (Dual,
    /// Wavetable, Ray, Orthogonal or Windows), the engine's presets; the plate
    /// holds every engine's controls, showing the one it plays. Both slots
    /// lay each engine out alike.
    fn oscillator(&mut self, slot: usize, rect: Rect) {
        self.plate("", rect, Strip::Section(SECTION_OSC));
        self.captions.push(Caption { text: ["I", "II"][slot], x: rect.x + 16.0, y: rect.y + 24.0, show: None, color: None,
                                     size: 19.0 });
        self.control(SLOT_MODES[slot], "Engine", Kind::Stepper, Rect::new(rect.x + 44.0, rect.y + 12.0, 128.0, 24.0));
        let save = Rect::new(rect.right() - 14.0 - 26.0, rect.y + 12.0, 26.0, 24.0);
        self.action(Action::SavePreset { slot }, "", save);
        let presets = Rect::new(save.x - 6.0 - 168.0, rect.y + 12.0, 168.0, 24.0);
        self.action(Action::Presets { slot }, "", presets);
        // What's left of the header, for an engine's own switches.
        let header = Rect::new(rect.x + 190.0, rect.y + 12.0, presets.x - 14.0 - rect.x - 190.0, 24.0);
        let body = Self::body(rect);
        self.show = Some((slot, SLOT_DUAL));
        self.dual(slot, body);
        self.show = Some((slot, SLOT_WAVETABLE));
        self.wavetable(slot, header, body);
        self.show = Some((slot, SLOT_RAY));
        self.ray(slot, header, body);
        self.show = Some((slot, SLOT_ORTHO));
        self.ortho(slot, body);
        self.show = Some((slot, SLOT_WINDOWS));
        self.windows(slot, body);
        self.show = None;
    }

    /// A Wavetable: its scope; its five waves along its path
    /// ([1] A [2] D [3] S [4] R [5]), with envelope A's outline under them;
    /// then the All Waves arrows, Speed (and Sync) and OscMod Depth. The
    /// OscMod is in the header.
    fn wavetable(&mut self, slot: usize, header: Rect, body: Rect) {
        const IDS: [[&str; 10]; 2] = [
            ["wt_1.oscmod", "wt_1.speed", "wt_1.speed_sync", "wt_1.oscmod_depth",
             "wt_1.wave_1", "wt_1.wave_2", "wt_1.wave_3", "wt_1.wave_4", "wt_1.wave_5", ""],
            ["wt_2.oscmod", "wt_2.speed", "wt_2.speed_sync", "wt_2.oscmod_depth",
             "wt_2.wave_1", "wt_2.wave_2", "wt_2.wave_3", "wt_2.wave_4", "wt_2.wave_5", ""],
        ];
        let ids = IDS[slot];
        let (oscmod, _) = header.take_right(122.0, 0.0);
        self.control(ids[0], "OscMod", Kind::Stepper, oscmod);
        self.caption("OscMod", oscmod.x - 56.0, oscmod.y + 12.0);

        let (scope, right) = body.take_left(250.0, 18.0);
        self.screen(Screen::Scope(slot), scope);
        const BOX_W: f32 = 66.0;
        const BOX_H: f32 = 50.0;
        const GAP: f32 = 24.0;
        let total = 5.0 * BOX_W + 4.0 * GAP;
        let x0 = right.x + (right.w - total) / 2.0;
        let y = body.y + 2.0;
        for k in 0..5 {
            let x = x0 + k as f32 * (BOX_W + GAP);
            self.control(ids[4 + k], ["Wave 1", "Wave 2", "Wave 3", "Wave 4", "Wave 5"][k], Kind::Wave { lfo: false },
                         Rect::new(x, y, BOX_W, BOX_H));
            if k < 4 {
                let letter = ["A", "D", "S", "R"][k];
                self.caption_in(letter, x + BOX_W + GAP / 2.0 - 4.0, y + BOX_H / 2.0, crate::modulation::ENV_A, 15.0);
            }
        }
        self.screen(Screen::WavePath(slot), Rect::new(x0, y + BOX_H + 10.0, total, 46.0));
        let ky = body.bottom() - 82.0;
        self.caption("All waves", right.x + 4.0, ky + 26.0);
        for (up, label, dx) in [(false, "‹", 4.0), (true, "›", 34.0)] {
            self.action(Action::ShiftWaves { slot, up }, label, Rect::new(right.x + dx, ky + 38.0, 26.0, 24.0));
        }
        self.knob_at(ids[1], "Speed", 50.0, false, right.x + 175.0, ky + 26.0);
        self.toggle(ids[2], "Sync", right.x + 214.0, ky + 14.0, 58.0);
        self.knob_at(ids[3], "OscMod Depth", 50.0, false, right.x + 345.0, ky + 26.0);
    }

    /// A Dual: its scope on the left; on the right its algorithm, its two
    /// waves, then A and B (its pitch is the slot's, on the Mix plate).
    fn dual(&mut self, slot: usize, body: Rect) {
        let ids = [["dual_1.algorithm", "dual_1.wave_1", "dual_1.wave_2", "dual_1.a", "dual_1.b"],
                   ["dual_2.algorithm", "dual_2.wave_1", "dual_2.wave_2", "dual_2.a", "dual_2.b"]][slot];
        let (scope, right) = body.take_left(360.0, 22.0);
        self.screen(Screen::Scope(slot), scope);
        let (caption, rest) = right.take_top(12.0, 4.0);
        self.caption("Algorithm", caption.x, caption.y + 6.0);
        let (algorithm, rest) = rest.take_top(24.0, 14.0);
        self.control(ids[0], "Algorithm", Kind::Stepper, algorithm);
        let (captions, rest) = rest.take_top(12.0, 4.0);
        let (waves, _) = rest.take_top(24.0, 0.0);
        for (k, (cell, label)) in waves.columns(&[1.0, 1.0], 12.0).into_iter().zip(["Wave 1", "Wave 2"]).enumerate() {
            self.caption(label, cell.x, captions.y + 6.0);
            self.control(ids[1 + k], label, Kind::Stepper, cell);
        }
        let cy = body.bottom() - 82.0 + 30.0;
        self.knob_at(ids[3], "A", 56.0, false, right.x + right.w * 0.28, cy);
        self.knob_at(ids[4], "B", 56.0, false, right.x + right.w * 0.72, cy);
    }

    /// A Ray (Raycaster's oscillator): its scope over the element picker,
    /// then the shape's eight knobs; Free in the header.
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
        self.toggle(k("free"), "Free", free.x, free.y - 1.0, free.w);

        let (left, grid) = body.take_left(250.0, 18.0);
        let (picker, scope) = left.take_bottom(26.0, INNER_GAP);
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
            self.knob_row(Rect::new(half.x + 24.0, half.y + (half.h - h) / 2.0, half.w - 48.0, h), &knobs);
        }
    }

    /// An Orthogonal: its two players' samples, Top over Bottom, on the left
    /// (click one to choose its set); beside each its Position and small
    /// Speed and Pitch knobs; their Mix on the right.
    fn ortho(&mut self, slot: usize, body: Rect) {
        const IDS: [[&str; 9]; 2] = [
            ["ortho_1.top", "ortho_1.bottom", "ortho_1.top_position", "ortho_1.bottom_position", "ortho_1.top_speed",
             "ortho_1.bottom_speed", "ortho_1.mix", "ortho_1.top_pitch", "ortho_1.bottom_pitch"],
            ["ortho_2.top", "ortho_2.bottom", "ortho_2.top_position", "ortho_2.bottom_position", "ortho_2.top_speed",
             "ortho_2.bottom_speed", "ortho_2.mix", "ortho_2.top_pitch", "ortho_2.bottom_pitch"],
        ];
        let ids = IDS[slot];
        const POSITION: f32 = 52.0;
        const SMALL: f32 = 36.0;
        let (knobs, samples) = body.take_right(240.0, 18.0);
        let (top, bottom) = samples.take_top((samples.h - 8.0) / 2.0, 8.0);
        let (mix, pairs) = knobs.take_right(knob_cell(POSITION).0, 8.0);
        for (h, half) in [top, bottom].into_iter().enumerate() {
            self.control(ids[h], ["Top", "Bottom"][h], Kind::Sample, half);
            let (w, rh) = knob_cell(POSITION);
            self.knob((ids[2 + h], "Position", POSITION, false), pairs.x, half.y + (half.h - rh) / 2.0);
            let (sw, sh) = knob_cell(SMALL);
            let y = half.y + (half.h - sh) / 2.0;
            self.knob((ids[4 + h], "Speed", SMALL, true), pairs.x + w + 8.0, y);
            self.knob((ids[7 + h], "Pitch", SMALL, true), pairs.x + w + 8.0 + sw + 4.0, y);
        }
        let (_, h) = knob_cell(POSITION);
        self.knob((ids[6], "Mix", POSITION, true), mix.x, body.y + (body.h - h) / 2.0);
    }

    /// A Windows: its scope over the base's picker on the left (as a Ray's);
    /// on the right a row per interrupt: its switch, its hidden wave's shape
    /// (and its Pitch under it), then X, Phase and Len.
    fn windows(&mut self, slot: usize, body: Rect) {
        const IDS: [[&str; 19]; 2] = [
            ["windows_1.base",
             "windows_1.int_1_on", "windows_1.int_1_shape", "windows_1.int_1_x", "windows_1.int_1_phase", "windows_1.int_1_len",
             "windows_1.int_1_pitch",
             "windows_1.int_2_on", "windows_1.int_2_shape", "windows_1.int_2_x", "windows_1.int_2_phase", "windows_1.int_2_len",
             "windows_1.int_2_pitch",
             "windows_1.int_3_on", "windows_1.int_3_shape", "windows_1.int_3_x", "windows_1.int_3_phase", "windows_1.int_3_len",
             "windows_1.int_3_pitch"],
            ["windows_2.base",
             "windows_2.int_1_on", "windows_2.int_1_shape", "windows_2.int_1_x", "windows_2.int_1_phase", "windows_2.int_1_len",
             "windows_2.int_1_pitch",
             "windows_2.int_2_on", "windows_2.int_2_shape", "windows_2.int_2_x", "windows_2.int_2_phase", "windows_2.int_2_len",
             "windows_2.int_2_pitch",
             "windows_2.int_3_on", "windows_2.int_3_shape", "windows_2.int_3_x", "windows_2.int_3_phase", "windows_2.int_3_len",
             "windows_2.int_3_pitch"],
        ];
        let ids = IDS[slot];
        let (left, grid) = body.take_left(250.0, 22.0);
        let (picker, scope) = left.take_bottom(26.0, INNER_GAP);
        self.screen(Screen::Scope(slot), scope);
        let (caption, picker) = picker.take_left(34.0, 6.0);
        self.caption("Base", caption.x, caption.y + 13.0);
        self.control(ids[0], "Base", Kind::Stepper, picker);
        const SWITCH: f32 = 44.0;
        const SHAPE: f32 = 120.0;
        const SMALL: f32 = 36.0;
        let (_, knob_h) = knob_cell(SMALL);
        let row_h = (grid.h - 2.0 * 8.0) / 3.0;
        for i in 0..3 {
            let row = Rect::new(grid.x, grid.y + i as f32 * (row_h + 8.0), grid.w, row_h);
            let top = row.y + (row.h - knob_h) / 2.0;
            // The switch and shape sit level with the dials' centres.
            let centre = top + SMALL / 2.0;
            let f = |k: usize| ids[1 + 6 * i + k];
            let (switch, rest) = row.take_left(SWITCH, 8.0);
            self.toggle(f(0), ["1", "2", "3"][i], switch.x, centre - TOGGLE_H / 2.0, switch.w);
            let (shape, rest) = rest.take_left(SHAPE, 18.0);
            self.control(f(1), "Shape", Kind::Stepper, Rect::new(shape.x, centre - 13.0, shape.w, 24.0));
            // Its Pitch: a thin slider under the shape, in the row's spare height.
            self.control(f(5), "Pitch", Kind::Slider, Rect::new(shape.x + 4.0, centre + 15.0, shape.w - 8.0, SLIDER_H));
            self.knob_row(Rect::new(rest.x, top, rest.w, knob_h),
                          &[(f(2), "X", SMALL, false), (f(3), "Phase", SMALL, false), (f(4), "Len", SMALL, false)]);
        }
    }

    /// I against II (Balance), each slot's Pitch under it; the noise's and
    /// the transient's levels, with the noise's Clock and the FM transient's
    /// Ratio under them.
    fn mix(&mut self, rect: Rect) {
        self.plate("Mix", rect, Strip::Section(SECTION_MIX));
        let iy = rect.y + 46.0;
        let (bx, by) = (rect.x + 82.0, iy + 50.0);
        self.knob_at("mix.balance", "Balance", 68.0, true, bx, by);
        self.captions.push(Caption { text: "I", x: bx - 54.0, y: by + 30.0, show: None, color: None, size: 13.0 });
        self.captions.push(Caption { text: "II", x: bx + 44.0, y: by + 30.0, show: None, color: None, size: 13.0 });
        let ky = rect.bottom() - 62.0;
        self.knob_at("osc_1.pitch", "Pitch I", 34.0, true, rect.x + 44.0, ky);
        self.knob_at("osc_2.pitch", "Pitch II", 34.0, true, rect.x + 120.0, ky);
        self.dividers.push((rect.x + 170.0, iy, rect.h - 60.0));
        for (k, (id, label, small, small_label, bipolar)) in [("mix.noise", "Noise", "noise.clock_ratio", "Clock", true),
                                                              ("transient.level", "Trans", "transient.ratio", "Ratio", false)]
            .into_iter().enumerate()
        {
            let cx = rect.x + 206.0 + 58.0 * k as f32;
            self.fader(id, label, cx, iy + 2.0, 136.0, None, 16.0, false);
            self.knob_at(small, small_label, 34.0, bipolar, cx, ky);
        }
    }

    fn noise(&mut self, rect: Rect) {
        self.plate("Noise", rect, Strip::Section(SECTION_MIX));
        self.control("noise.type", "Type", Kind::Stepper, Rect::new(rect.right() - 14.0 - 96.0, rect.y + 12.0, 96.0, 24.0));
        let body = Self::body(rect);
        let (row, screen) = body.take_bottom(62.0, INNER_GAP);
        self.screen(Screen::Noise, screen);
        for (k, (id, label, bipolar)) in [("noise.bits", "Bits", false), ("noise.crackle_density", "Density", false),
                                          ("noise.tilt", "Tilt", true)].into_iter().enumerate() {
            self.knob_at(id, label, 32.0, bipolar, rect.x + 40.0 + 53.0 * k as f32, row.y + 16.0);
        }
    }

    fn transient(&mut self, rect: Rect) {
        self.plate("Transient", rect, Strip::Section(SECTION_MIX));
        self.toggle("transient.post_filter", "Post", rect.right() - 14.0 - 62.0, rect.y + 11.0, 62.0);
        let body = Self::body(rect);
        let (stepper, body) = body.take_top(24.0, INNER_GAP);
        self.control("transient.type", "Type", Kind::Stepper, stepper);
        let (row, screen) = body.take_bottom(70.0, INNER_GAP);
        self.screen(Screen::Transient, screen);
        for (k, (id, label, bipolar)) in [("transient.decay", "Decay", false), ("transient.pitch", "Pitch", true),
                                          ("transient.depth", "Depth", false)].into_iter().enumerate() {
            self.knob_at(id, label, 40.0, bipolar, rect.x + 50.0 + 75.0 * k as f32, row.y + 20.0);
        }
    }

    /// The drive (which oscillators go through it, how hard, its curve), the
    /// filter's response, the cutoff with the HPF round it, envelope B's
    /// depth on the cutoff, then Resonance, A and B (whatever the type makes
    /// them). The type and 4-Pole in the header.
    fn filter(&mut self, rect: Rect) {
        self.plate("Filter", rect, Strip::Section(SECTION_FILTER));
        let pole = Rect::new(rect.right() - 14.0 - 70.0, rect.y + 11.0, 70.0, TOGGLE_H);
        self.control("filter.4_pole", "4-Pole", Kind::Toggle, pole);
        self.control("filter.type", "Type", Kind::Stepper, Rect::new(pole.x - 10.0 - 120.0, rect.y + 12.0, 120.0, 24.0));
        let body = Self::body(rect);
        let (iy, bh) = (body.y, body.h);
        // Drive.
        let dx = body.x;
        self.caption("Drive", dx + 2.0, iy + 6.0);
        self.toggle("drive.osc_i", "I", dx, iy + 18.0, 40.0);
        self.toggle("drive.osc_ii", "II", dx + 46.0, iy + 18.0, 40.0);
        self.fader("drive.amount", "Amt", dx + 18.0, iy + 54.0, bh - 54.0 - 16.0, None, 16.0, false);
        self.screen(Screen::DriveCurve, Rect::new(dx + 44.0, iy + 78.0, 42.0, 42.0));
        // The response.
        let screen = Rect::new(dx + 104.0, iy, 214.0, bh);
        self.screen(Screen::Filter, screen);
        // The cutoff and the HPF's arc; the HPF's Peak a trim in the corner.
        let (kx, ky, rr) = (screen.right() + 112.0, iy + bh / 2.0 - 6.0, 68.0);
        self.control("filter.hpf", "HPF", Kind::Arc { cx: kx, cy: ky, r: rr },
                     Rect::new(kx - rr - 10.0, ky - 10.0, rr + 14.0, rr + 20.0));
        let (w, h) = knob_cell(104.0);
        self.control("filter.cutoff", "Cutoff", Kind::Knob { size: 104.0, bipolar: false, label: LabelAt::BelowRight(46.0) },
                     Rect::new(kx - w / 2.0, ky - 52.0, w, h));
        self.knob_at("filter.hpf_peak", "Peak", 22.0, false, kx - rr - 16.0, ky + rr + 12.0);
        // Envelope B's depth, in its shade.
        let ex = kx + 92.0;
        self.caption_in("Env", ex - 8.0, iy + 6.0, crate::modulation::ENV_B, 9.5);
        self.fader("filter.env_b", "B", ex, iy + 18.0, bh - 40.0, Some(crate::modulation::ENV_B), 14.0, true);
        // Resonance, A, B.
        let colx = ex + 56.0;
        for (k, (id, label)) in [("filter.resonance", "Resonance"), ("filter.a", "A"), ("filter.b", "B")].into_iter().enumerate() {
            self.knob_beside(id, label, 40.0, LabelAt::Right, colx, iy + 26.0 + 76.0 * k as f32);
        }
    }

    /// Envelopes A, B and C, side by side, each in its own shade: its
    /// assignment button and what it's wired to, then four thin faders
    /// (Attack, Decay, Sustain, Release), the attack's curve left of them,
    /// the decay's and release's right.
    fn envelopes(&mut self, rect: Rect) {
        use crate::modulation::{ENV_A, ENV_B, ENV_C};
        const SOURCES: [usize; 3] = [ENV_A, ENV_B, ENV_C];
        self.plate("Envelopes", rect, Strip::Sources(&SOURCES));
        const IDS: [[&str; 6]; 3] = [
            ["env_a.attack", "env_a.decay", "env_a.sustain", "env_a.release", "env_a.attack_curve", "env_a.dr_curve"],
            ["env_b.attack", "env_b.decay", "env_b.sustain", "env_b.release", "env_b.attack_curve", "env_b.dr_curve"],
            ["env_c.attack", "env_c.decay", "env_c.sustain", "env_c.release", "env_c.attack_curve", "env_c.dr_curve"],
        ];
        let iy = rect.y + 44.0;
        let gh = rect.h - 44.0 - 12.0;
        let gw = (rect.w - 28.0 - 20.0) / 3.0;
        for (g, (&source, ids)) in SOURCES.iter().zip(IDS).enumerate() {
            let gx = rect.x + 14.0 + g as f32 * (gw + 10.0);
            self.washes.push(Wash { source, rect: Rect::new(gx, iy, gw, gh) });
            self.action(Action::Assign(source), "", Rect::new(gx + 10.0, iy + 10.0, ASSIGN, ASSIGN));
            self.caption_in(["Amp", "Filter", "Free"][g], gx + 48.0, iy + 20.0, source, 11.0);
            self.caption(["→ amp, full", "→ cutoff", "unassigned"][g], gx + 48.0, iy + 33.0);
            let top = iy + 58.0;
            let fh = gh - 58.0 - 26.0;
            const CELL: f32 = 21.0;
            let fx0 = gx + 26.0;
            self.control(ids[4], "Curve", Kind::Curve { rising: true }, Rect::new(gx + 7.0, top - 2.0, CURVE_SIZE, CURVE_SIZE));
            for (k, label) in ["A", "D", "S", "R"].into_iter().enumerate() {
                self.fader(ids[k], label, fx0 + k as f32 * CELL + CELL / 2.0, top, fh, Some(source), 14.0, false);
            }
            self.control(ids[5], "Curve", Kind::Curve { rising: false },
                         Rect::new(fx0 + 4.0 * CELL + 5.0, top - 2.0, CURVE_SIZE, CURVE_SIZE));
        }
    }

    /// LFOs X, Y and Z, a row each in its own shade: its assignment button,
    /// its shape (click for the picker), Rate, Sync and Free; Z also its
    /// pitch depth and what scales it (On, MW, AT).
    fn lfos(&mut self, rect: Rect) {
        use crate::modulation::LFO_X;
        const SOURCES: [usize; 3] = [LFO_X, LFO_X + 1, LFO_X + 2];
        self.plate("LFOs", rect, Strip::Sources(&SOURCES));
        const IDS: [[&str; 4]; 3] = [["lfo_x.shape", "lfo_x.rate", "lfo_x.sync", "lfo_x.free"],
                                     ["lfo_y.shape", "lfo_y.rate", "lfo_y.sync", "lfo_y.free"],
                                     ["lfo_z.shape", "lfo_z.rate", "lfo_z.sync", "lfo_z.free"]];
        let iy = rect.y + 44.0;
        let rh = (rect.h - 44.0 - 12.0 - 2.0 * 7.0) / 3.0;
        let rx = rect.x + 14.0;
        for (r, (&source, ids)) in SOURCES.iter().zip(IDS).enumerate() {
            let ry = iy + r as f32 * (rh + 7.0);
            self.washes.push(Wash { source, rect: Rect::new(rx, ry, rect.w - 28.0, rh) });
            let cy = ry + rh / 2.0;
            self.action(Action::Assign(source), "", Rect::new(rx + 10.0, cy - ASSIGN / 2.0, ASSIGN, ASSIGN));
            self.control(ids[0], "Shape", Kind::Wave { lfo: true }, Rect::new(rx + 52.0, cy - 25.0, 50.0, 50.0));
            self.knob_at(ids[1], "Rate", 30.0, false, rx + 140.0, cy - 10.0);
            self.control(ids[2], "Sync", Kind::Toggle, Rect::new(rx + 172.0, cy - 25.0, 54.0, 22.0));
            self.control(ids[3], "Free", Kind::Toggle, Rect::new(rx + 172.0, cy + 3.0, 54.0, 22.0));
            if r == 2 {
                let zx = rx + 240.0;
                self.caption_in("→ Pitch I+II", zx, cy - 18.0, source, 9.5);
                self.control("lfo_z.depth", "Depth", Kind::Slider, Rect::new(zx, cy - 7.0, 100.0, SLIDER_H));
                self.control("lfo_z.mode", "Mode", Kind::Segments, Rect::new(zx, cy + 12.0, 90.0, 20.0));
            }
        }
    }

    /// The played sources' assignment buttons (mod wheel, aftertouch, key,
    /// velocity), each with its level now; the pitch bend's job under them.
    fn perform(&mut self, rect: Rect) {
        use crate::modulation::{KEY, TOUCH, VELOCITY, WHEEL};
        const SOURCES: [usize; 4] = [WHEEL, TOUCH, KEY, VELOCITY];
        self.plate("Perform", rect, Strip::Sources(&SOURCES));
        let iy = rect.y + 46.0;
        let cw = (rect.w - 28.0 - 3.0 * 8.0) / 4.0;
        for (i, (&source, caption)) in SOURCES.iter().zip(["wheel", "touch", "± C4", "velocity"]).enumerate() {
            let cx = rect.x + 14.0 + i as f32 * (cw + 8.0);
            self.washes.push(Wash { source, rect: Rect::new(cx, iy, cw, 150.0) });
            self.action(Action::Assign(source), "", Rect::new(cx + cw / 2.0 - 17.0, iy + 14.0, 34.0, 34.0));
            self.caption(caption, cx + cw / 2.0 - 16.0, iy + 66.0);
            self.screen(Screen::Meter(source), Rect::new(cx + cw / 2.0 - 3.0, iy + 82.0, 6.0, 56.0));
        }
        self.caption("Pitch bend", rect.x + 14.0, rect.bottom() - 50.0);
        self.control("perform.bend_mode", "Pitch bend", Kind::Dropdown,
                     Rect::new(rect.x + 14.0, rect.bottom() - 40.0, rect.w - 28.0, 24.0));
    }

    fn harmonic(&mut self, rect: Rect) {
        self.plate("Harmonic", rect, Strip::Section(SECTION_HARMONIC));
        self.control("harmonic.shape", "Shape", Kind::Stepper, Rect::new(rect.right() - 14.0 - 150.0, rect.y + 12.0, 150.0, 24.0));
        let body = Self::body(rect);
        let (row, screen) = body.take_bottom(68.0, INNER_GAP);
        self.screen(Screen::Harmonic, screen);
        for (k, (id, label, bipolar)) in [("harmonic.shift", "Shift", true), ("harmonic.tilt", "Tilt", true),
                                          ("harmonic.depth", "Depth", false), ("harmonic.odd_even", "Odd/Even", true)]
            .into_iter().enumerate()
        {
            self.knob_at(id, label, 38.0, bipolar, rect.x + 46.0 + 66.0 * k as f32, row.y + 19.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::params::SynthParams;

    #[test]
    fn every_param_is_on_the_panel_once() {
        let params = SynthParams::from_faust();
        let layout = Layout::build();
        let on_panel: Vec<&str> = layout.controls.iter().map(|c| c.id).collect();
        for id in &on_panel {
            assert!(params.ptr(id).is_some(), "panel control '{id}' has no parameter");
        }
        for id in params.ids().filter(|id| !UNSHOWN.contains(id) && !id.starts_with("mod.")) {
            let n = on_panel.iter().filter(|c| **c == id).count();
            assert_eq!(n, 1, "parameter '{id}' appears {n} times on the panel");
        }
        for t in crate::modulation::TARGETS {
            assert!(on_panel.contains(&t.id), "'{}' isn't on the panel", t.id);
        }
        for s in 0..crate::modulation::SOURCE_COUNT {
            assert!(layout.actions.iter().any(|b| b.action == Action::Assign(s)), "no button for source {s}");
        }
    }

    /// The two slots are interchangeable: each engine sits the same way in both.
    #[test]
    fn the_slots_match() {
        let layout = Layout::build();
        let slot = |s: usize| layout.plates.iter().filter(|p| p.title.is_empty()).nth(s).unwrap().rect;
        let (a, b) = (slot(0), slot(1));
        assert_eq!((a.w, a.h, a.y), (b.w, b.h, b.y), "the slots' plates differ");
        let offset = b.x - a.x;
        for mode in 0..5 {
            let of = |s: usize| -> Vec<(String, Rect)> {
                layout.controls.iter().filter(|c| c.show == Some((s, mode)))
                    .map(|c| (c.id.replace("_2.", "_1."), c.rect)).collect()
            };
            let (one, two) = (of(0), of(1));
            assert_eq!(one.len(), two.len());
            for ((id1, r1), (id2, r2)) in one.iter().zip(&two) {
                assert_eq!(id1, id2);
                assert!((r2.x - r1.x - offset).abs() < 1e-3 && r1.y == r2.y && r1.w == r2.w && r1.h == r2.h,
                        "{id1} sits differently in II");
            }
        }
    }

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
        // The cutoff's arc wraps round it, and its trim sits in the corner: they may share cells.
        let shared = |a: &Control, b: &Control| {
            let filter = |c: &Control| ["filter.hpf", "filter.cutoff", "filter.hpf_peak"].contains(&c.id);
            filter(a) && filter(b)
        };
        for overlay in [false, true] {
            let layer = &layout.controls[layout.range(overlay)];
            for (i, a) in layer.iter().enumerate() {
                let curve = |c: &Control| matches!(c.kind, Kind::Curve { .. });
                for b in layer[i + 1..].iter().filter(|b| together(a.show, b.show) && curve(a) == curve(b) && !shared(a, b)) {
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
    #[test]
    #[ignore]
    fn print_cells() {
        for c in super::Layout::build().controls {
            println!("{}\t{:.1}\t{:.1}\t{:.1}\t{:.1}", c.id, c.rect.x, c.rect.y, c.rect.w, c.rect.h);
        }
    }
}
