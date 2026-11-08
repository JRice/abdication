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
    /// Two parameters on one square: this control's id is X, `y` is Y.
    XyPad { y: &'static str },
}

#[derive(Clone, Copy, Debug)]
pub struct Control {
    pub id: &'static str,
    pub label: &'static str,
    pub kind: Kind,
    pub rect: Rect,
}

/// What a screen shows (drawn from the current parameter values).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Screen {
    Fundamental,
    /// The Dual oscillator's output (rendered by `scope`).
    DualScope,
    /// The wavetable oscillator's output: live while a note plays.
    WavetableScope,
    /// The multidrive's current transfer curve (top bar).
    DriveCurve,
    Slot(usize),
    Noise,
    Transient,
    Filter,
    Envelope,
    /// The free envelope's shape (and, live, its level).
    ModEnv,
    /// The free LFO's wave (and, live, its value).
    ModLfo,
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
}

/// What a momentary button does.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Action {
    /// Set the Mod Env to the Envelope's filter shape.
    CopyFilter,
    /// ... or to its amp shape (with the pad's offsets).
    CopyAmp,
    /// Move Start, Mid and End together, one wave along the library.
    ShiftWaves { up: bool },
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
}

#[derive(Clone, Debug, Default)]
pub struct Layout {
    /// Momentary buttons (not parameters).
    pub actions: Vec<Button>,
    pub top_bar: Rect,
    /// The amber readout in the top bar (last touched control).
    pub readout: Rect,
    pub plates: Vec<Plate>,
    pub screens: Vec<(Screen, Rect)>,
    pub captions: Vec<Caption>,
    pub controls: Vec<Control>,
    /// The settings overlay (shown from the ☰ button)...
    pub settings: Rect,
    /// ... and where its controls start in `controls` (they run to the end,
    /// and are drawn and hit only while it's open).
    pub settings_from: usize,
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

type KnobSpec = (&'static str, &'static str, f32, bool);

impl Layout {
    fn control(&mut self, id: &'static str, label: &'static str, kind: Kind, rect: Rect) {
        self.controls.push(Control { id, label, kind, rect });
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
        self.screens.push((screen, rect));
    }

    fn caption(&mut self, text: &'static str, x: f32, y: f32) {
        self.captions.push(Caption { text, x, y });
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
        // Top: the Fundamental and the Dual, then the free modulators.
        let c = row1.columns(&[250.0, 320.0, 290.0, 230.0], GAP);
        l.fundamental(c[0]);
        l.dual(c[1]);
        l.mod_env(c[2]);
        l.mod_lfo(c[3]);
        // Middle: the wavetable, then noise, transient and the mix.
        let c = row2.columns(&[760.0, 220.0, 150.0, 290.0], GAP);
        l.wavetable(c[0]);
        l.noise(c[1]);
        l.transient(c[2]);
        l.mix(c[3]);
        // Bottom: tone and shaping.
        let c = row3.columns(&[500.0, 464.0, 210.0, 300.0], GAP);
        l.filter(c[0]);
        l.envelope(c[1]);
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
        let rect = Rect::new(button.x, self.top_bar.bottom() + 8.0, 236.0, 150.0);
        self.settings = rect;
        let body = rect.inset(14.0, 40.0, 14.0, 12.0);
        self.knob_row(body, &[("settings.drive_grit", "Drive Grit", 52.0, false),
                              ("settings.vintage_noise", "Vtg Noise", 52.0, false)]);
    }

    /// The readout on the left; unison, vintage, portamento (0 = poly),
    /// drive (with its curve) and volume on the right.
    fn top_bar(&mut self, bar: Rect) {
        self.top_bar = bar;
        // ☰ in the corner, then the readout.
        let settings = Rect::new(bar.x + 14.0, bar.y + 12.0, 30.0, 36.0);
        self.actions.push(Button { action: Action::Settings, label: "", rect: settings });
        self.readout = Rect::new(settings.right() + 10.0, bar.y + 13.0, 352.0, 34.0);
        // The dice, then the step back from them.
        let dice = Rect::new(self.readout.right() + 10.0, bar.y + 12.0, 40.0, 36.0);
        let back = Rect::new(dice.right() + 4.0, dice.y, 26.0, dice.h);
        self.actions.push(Button { action: Action::Dice, label: "", rect: dice });
        self.actions.push(Button { action: Action::DiceBack, label: "‹", rect: back });
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

    fn fundamental(&mut self, rect: Rect) {
        let body = self.plate("Fundamental", rect);
        let body = self.header_stepper(body, "fundamental.wave", "Wave", 104.0);
        let knobs = [("fundamental.pulse_width", "Pulse W", 44.0, false),
                     ("fundamental.pw_lfo_rate", "PW Rate", 44.0, false),
                     ("fundamental.pw_lfo_depth", "PW Depth", 44.0, false)];
        let (row, body) = body.take_bottom(Self::row_height(&knobs), INNER_GAP);
        self.knob_row(row, &knobs);
        self.screen(Screen::Fundamental, body);
    }

    fn dual(&mut self, rect: Rect) {
        let body = self.plate("Dual", rect);
        let (header, body) = body.take_top(HEADER, INNER_GAP);
        let (algorithm, header) = header.take_right(150.0, 6.0);
        let (waves, _) = header.take_right(92.0, 0.0);
        self.control("dual.algorithm", "Algorithm", Kind::Stepper, algorithm);
        self.control("dual.waves", "Waves", Kind::Stepper, waves);
        let knobs = [("dual.a", "A", 52.0, false), ("dual.b", "B", 52.0, false), ("dual.semi", "Semi", 44.0, true)];
        let (row, body) = body.take_bottom(Self::row_height(&knobs), INNER_GAP);
        self.knob_row(row, &knobs);
        self.screen(Screen::DualScope, body);
    }

    /// The free envelope: its shape above five faders, like the Envelope;
    /// buttons in the header copy the Envelope's filter or amp shape.
    fn mod_env(&mut self, rect: Rect) {
        let body = self.plate("Mod Env", rect);
        let (header, body) = body.take_top(HEADER, INNER_GAP);
        let mut x = header.right();
        for (action, label) in [(Action::CopyAmp, "cp amp"), (Action::CopyFilter, "cp flt")] {
            x -= 64.0;
            self.actions.push(Button { action, label, rect: Rect::new(x, header.y - 1.0, 64.0, TOGGLE_H) });
            x -= 6.0;
        }
        let (faders, screen) = body.take_bottom(140.0, INNER_GAP);
        self.screen(Screen::ModEnv, screen);
        self.fader_row(faders, &[("mod_env.attack", "Attack"), ("mod_env.decay", "Decay"),
                                 ("mod_env.sustain", "Sustain"), ("mod_env.release", "Release"),
                                 ("mod_env.curve", "Curve")]);
    }

    /// The free LFO: Sync in the header, its wave, then shape and rate.
    fn mod_lfo(&mut self, rect: Rect) {
        let body = self.plate("Mod LFO", rect);
        let (header, body) = body.take_top(HEADER, INNER_GAP);
        let (sync, _) = header.take_right(70.0, 0.0);
        self.control("mod_lfo.sync", "Sync", Kind::Toggle, Rect::new(sync.x, header.y - 1.0, sync.w, TOGGLE_H));
        let knob = ("mod_lfo.rate", "Rate", 52.0, false);
        let (row, screen) = body.take_bottom(knob_cell(52.0).1, INNER_GAP);
        self.screen(Screen::ModLfo, screen);
        let (knob_area, shape) = row.take_right(knob_cell(52.0).0 + 10.0, 8.0);
        self.knob(knob, knob_area.x + 10.0, knob_area.y);
        let (caption, shape) = shape.take_top(12.0, 6.0);
        self.caption("Shape", caption.x, caption.y + 6.0);
        self.control("mod_lfo.shape", "Shape", Kind::Stepper, Rect::new(shape.x, shape.y, shape.w, 26.0));
    }

    fn wavetable(&mut self, rect: Rect) {
        let body = self.plate("Wavetable", rect);
        // Arrows beside the title move all three waves together.
        let (x, y) = (body.x + 104.0, body.y);
        self.caption("All waves", x, y + HEADER / 2.0);
        for (up, label, offset) in [(false, "‹", 56.0), (true, "›", 86.0)] {
            self.actions.push(Button { action: Action::ShiftWaves { up }, label, rect: Rect::new(x + offset, y, 26.0, HEADER) });
        }
        let body = self.header_stepper(body, "wavetable.oscmod", "OscMod", 144.0);
        self.caption("OscMod", body.right() - 144.0 - 50.0, body.y - INNER_GAP - HEADER / 2.0);
        let (scope, right) = body.take_left(body.w * 0.42, 16.0);
        self.screen(Screen::WavetableScope, scope);
        let knobs = [("wavetable.lfo_rate", "LFO Rate", 52.0, false),
                     ("wavetable.oscmod_depth", "OscMod Depth", 52.0, false)];
        let (row, slots) = right.take_bottom(Self::row_height(&knobs), INNER_GAP);
        let mut x = row.x;
        for k in knobs {
            self.knob(k, x, row.y);
            x += knob_cell(k.2).0 + 34.0;
        }
        let slot_ids = [("wavetable.start", "Start"), ("wavetable.mid", "Mid"), ("wavetable.end", "End")];
        for (i, (cell, (id, label))) in slots.columns(&[1.0, 1.0, 1.0], 12.0).into_iter().zip(slot_ids).enumerate() {
            let (caption, cell) = cell.take_top(12.0, 4.0);
            self.caption(label, caption.x, caption.y + 6.0);
            let (button, screen) = cell.take_bottom(26.0, 4.0);
            self.screen(Screen::Slot(i), screen);
            self.control(id, label, Kind::Stepper, button);
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

    /// Slim: its level and pitch live in the Mix plate.
    fn transient(&mut self, rect: Rect) {
        let body = self.plate("Transient", rect);
        let (_, body) = body.take_top(HEADER, INNER_GAP);
        let (stepper, body) = body.take_top(HEADER, INNER_GAP);
        self.control("transient.type", "Type", Kind::Stepper, stepper);
        let knobs = [("transient.decay", "Decay", 44.0, false)];
        let (row, body) = body.take_bottom(Self::row_height(&knobs), INNER_GAP);
        self.knob_row(row, &knobs);
        let (post, body) = body.take_bottom(TOGGLE_H, INNER_GAP);
        self.toggle("transient.post_filter", "Post", post.x, post.y, post.w);
        self.screen(Screen::Transient, body);
    }

    /// A fader per source, its octave (Noise: clock) knob below; the
    /// transient's level and pitch too, though it's outside the balance.
    fn mix(&mut self, rect: Rect) {
        let body = self.plate("Mix", rect);
        let (_, body) = body.take_top(HEADER, INNER_GAP);
        let faders = [("mix.fundamental", "Fund"), ("mix.dual", "Dual"), ("mix.wavetable", "WT"), ("mix.noise", "Noise"),
                      ("transient.level", "Trans")];
        let below = [("fundamental.octave", "Octave"), ("dual.octave", "Octave"), ("wavetable.octave", "Octave"),
                     ("noise.clock_ratio", "Clock"), ("transient.sample_pitch", "Pitch")];
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
        let mut x = header.right();
        for (id, label, w) in [("filter.invert_env", "Inv Env", 78.0), ("filter.4_pole", "4-Pole", 74.0)] {
            x -= w;
            self.control(id, label, Kind::Toggle, Rect::new(x, header.y - 1.0, w, TOGGLE_H));
            x -= 6.0;
        }
        let (screen, grid) = body.take_left(212.0, 16.0);
        self.screen(Screen::Filter, screen);
        let rows = [
            [("filter.cutoff", "Cutoff", 56.0, false), ("filter.resonance", "Resonance", 48.0, false),
             ("filter.env_amount", "Env Amt", 48.0, false), ("filter.utility_hpf", "HPF", 44.0, false)],
            [("filter.morph_lp_notch_hp", "Morph", 44.0, false), ("filter.bandpass", "Bandpass", 44.0, false),
             ("filter.keytrack", "Keytrack", 44.0, false), ("", "", 0.0, false)],
        ];
        let cells = grid.columns(&[1.0; 4], 6.0);
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

    fn envelope(&mut self, rect: Rect) {
        let body = self.plate("Envelope", rect);
        let (header, body) = body.take_top(HEADER, INNER_GAP);
        self.caption("— Filter", header.right() - 120.0, header.y + 13.0);
        self.caption("··· Amp", header.right() - 52.0, header.y + 13.0);
        let (xy_col, left) = body.take_right(168.0, 16.0);
        let (caption, xy_col) = xy_col.take_top(12.0, 6.0);
        self.caption("Amp offset", caption.x, caption.y + 6.0);
        let pad = Rect::new(xy_col.x, xy_col.y, 168.0, 168.0);
        // Left/right lengthens the amp decay; up/down raises its sustain.
        self.control("envelope.amp_decay", "Amp offset", Kind::XyPad { y: "envelope.amp_sustain" }, pad);
        let (faders, screen) = left.take_bottom(140.0, INNER_GAP);
        self.screen(Screen::Envelope, screen);
        self.fader_row(faders, &[("envelope.attack", "Attack"), ("envelope.decay", "Decay"),
                                 ("envelope.sustain", "Sustain"), ("envelope.release", "Release"),
                                 ("envelope.curve", "Curve")]);
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

    fn perform(&mut self, rect: Rect) {
        let body = self.plate("Perform", rect);
        let (_, body) = body.take_top(HEADER, 8.0);
        let mut y = body.y;
        for (caption, dest, depth) in [("Mod wheel", "perform.mw_dest", "perform.mw_depth"),
                                       ("Aftertouch", "perform.at_dest", "perform.at_depth")] {
            self.caption(caption, body.x, y + 6.0);
            let (w, _) = knob_cell(36.0);
            self.control(dest, caption, Kind::Dropdown, Rect::new(body.x, y + 16.0, body.w - w - 8.0, 26.0));
            self.knob((depth, "Depth", 36.0, true), body.right() - w, y);
            y += 74.0;
        }
        self.caption("Pitch bend", body.x, y + 6.0);
        self.control("perform.bend_mode", "Pitch bend", Kind::Dropdown, Rect::new(body.x, y + 16.0, body.w, 26.0));
    }
}

/// The ids of a control's sends ("<id>_env", "<id>_lfo"); the panel shows
/// whichever exist as its rings.
pub fn send_ids(id: &str) -> [String; 2] {
    [format!("{id}_env"), format!("{id}_lfo")]
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
        let mut on_panel: Vec<&str> = layout.controls.iter().map(|c| c.id).collect();
        on_panel.extend(layout.controls.iter().filter_map(|c| match c.kind {
            Kind::XyPad { y } => Some(y),
            _ => None,
        }));
        for id in &on_panel {
            assert!(params.ptr(id).is_some(), "panel control '{id}' has no parameter");
        }
        // Sends are drawn as their control's rings.
        let sends: Vec<String> = layout.controls.iter().flat_map(|c| send_ids(c.id)).collect();
        for id in params.ids().filter(|id| !sends.iter().any(|s| s == id)) {
            let n = on_panel.iter().filter(|c| **c == id).count();
            assert_eq!(n, 1, "parameter '{id}' appears {n} times on the panel");
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
            for c in &layout.controls[layout.range(false)] {
                assert!(!overlap(b.rect, c.rect), "{:?} overlaps {}", b.action, c.id);
            }
            for (s, r) in &layout.screens {
                assert!(!overlap(b.rect, *r), "{:?} overlaps screen {s:?}", b.action);
            }
            for other in &layout.actions[i + 1..] {
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
                for b in &layer[i + 1..] {
                    assert!(!overlap(a.rect, b.rect), "{} overlaps {}", a.id, b.id);
                }
                for (s, r) in layout.screens.iter().filter(|_| !overlay) {
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
