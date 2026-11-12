//! Where everything goes, in logical pixels on a 1000 x 540 panel (the
//! design mockup's own size; the editor scales it as a whole). The volume
//! bar (with the output meter in it) top right; five part cards across the
//! top, each with a front and a back ("flip side"); below, the screen, the
//! free envelope and the effects.

pub const WIDTH: f32 = 1000.0;
pub const HEIGHT: f32 = 540.0;

const PAD: f32 = 12.0;
const GAP: f32 = 10.0;
const CARD_W: f32 = 187.0;
const CARD_H: f32 = 326.0;
const CARD_PAD_X: f32 = 10.0;
const FRONT_KNOB: f32 = 38.0;
const BACK_KNOB: f32 = 28.0;
const LEVEL_H: f32 = 18.0;
/// Two rows of front knobs.
const KNOB_GRID_H: f32 = 132.0;

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
    pub fn contains(self, x: f32, y: f32) -> bool {
        x >= self.x && x < self.right() && y >= self.y && y < self.bottom()
    }
    pub fn inset(self, left: f32, top: f32, right: f32, bottom: f32) -> Self {
        Self::new(self.x + left, self.y + top, self.w - left - right, self.h - top - bottom)
    }
    /// Split off the left `w` pixels; the rest starts `gap` to the right.
    pub fn take_left(self, w: f32, gap: f32) -> (Self, Self) {
        (Self { w, ..self }, Self::new(self.x + w + gap, self.y, self.w - w - gap, self.h))
    }
    /// `n` equal columns with `gap` between them.
    pub fn columns(self, n: usize, gap: f32) -> Vec<Self> {
        let w = (self.w - gap * (n as f32 - 1.0)) / n as f32;
        (0..n).map(|i| Self::new(self.x + i as f32 * (w + gap), self.y, w, self.h)).collect()
    }
    /// Cells of a grid with `cols` columns, `cell_h` tall, filled row by row.
    pub fn grid(self, cols: usize, cell_h: f32, gap_x: f32, gap_y: f32, count: usize) -> Vec<Self> {
        let w = (self.w - gap_x * (cols as f32 - 1.0)) / cols as f32;
        (0..count)
            .map(|i| {
                let (c, r) = ((i % cols) as f32, (i / cols) as f32);
                Self::new(self.x + c * (w + gap_x), self.y + r * (cell_h + gap_y), w, cell_h)
            })
            .collect()
    }
}

/// The five parts, in the voice's output order (their scopes follow it).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Part {
    Tone,
    Body,
    Noise,
    Sample1,
    Sample2,
}

impl Part {
    pub const ALL: [Part; 5] = [Part::Tone, Part::Body, Part::Noise, Part::Sample1, Part::Sample2];

    pub fn title(self) -> &'static str {
        match self {
            Part::Tone => "Tone",
            Part::Body => "Body",
            Part::Noise => "Noise",
            Part::Sample1 => "Sample 1",
            Part::Sample2 => "Sample 2",
        }
    }

    /// The parameter group ("sample_1.level").
    pub fn group(self) -> &'static str {
        match self {
            Part::Tone => "tone",
            Part::Body => "body",
            Part::Noise => "noise",
            Part::Sample1 => "sample_1",
            Part::Sample2 => "sample_2",
        }
    }
}

/// Which side of its card a control is on (effects-row controls: always shown).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Front,
    Back,
    Always,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Kind {
    Knob { size: f32, bipolar: bool },
    /// A horizontal level bar along a card's foot.
    Level,
    /// A small box showing a menu parameter's value; a click steps it.
    Chip,
    /// One of a menu parameter's values as a button (the sample sets).
    Choice { value: f32 },
    /// ‹ value ›: the arrows step (within the sample's set), the middle opens the list.
    Stepper,
    /// A latching switch.
    Toggle,
}

#[derive(Clone, Copy, Debug)]
pub struct Control {
    pub id: &'static str,
    pub label: &'static str,
    pub kind: Kind,
    pub rect: Rect,
    pub card: Option<Part>,
    pub side: Side,
}

#[derive(Clone, Copy, Debug)]
pub struct Card {
    pub part: Part,
    pub rect: Rect,
    pub title: Rect,
    /// The square before the title: the part's on/off switch.
    pub power: Rect,
    pub flip: Option<Rect>,
    pub scope: Rect,
}

#[derive(Clone, Debug, Default)]
pub struct Layout {
    pub top_bar: Rect,
    /// The die (a random sound) and the export button (drag a hit out as a WAV).
    pub die: Rect,
    pub export: Rect,
    pub cards: Vec<Card>,
    pub screen: Rect,
    pub envelope: Rect,
    /// The envelope's own little picture.
    pub envelope_curve: Rect,
    /// The pad filter (the SH-101's), between the envelope and the effects.
    pub filter: Rect,
    pub effects: Rect,
    pub controls: Vec<Control>,
}

/// (id, label, bipolar)
type KnobSpec = (&'static str, &'static str, bool);

const fn k(id: &'static str, label: &'static str) -> KnobSpec {
    (id, label, false)
}
const fn bi(id: &'static str, label: &'static str) -> KnobSpec {
    (id, label, true)
}

fn front_knobs(part: Part) -> [KnobSpec; 4] {
    match part {
        Part::Tone => [k("tone.pitch", "Pitch"), k("tone.drop", "Drop"), k("tone.shape", "Shape"), k("tone.decay", "Decay")],
        Part::Body => [bi("body.offset", "Tune"), k("body.material", "Material"), k("body.strike", "Strike"), k("body.decay", "Decay")],
        Part::Noise => [k("noise.bandpass", "Freq"), k("noise.q", "Q"), k("noise.blend", "Blend"), k("noise.decay", "Decay")],
        Part::Sample1 => [bi("sample_1.pitch", "Pitch"), k("sample_1.decay", "Decay"), k("sample_1.body_send", "Send"), k("sample_1.vel_level", "Vel")],
        Part::Sample2 => [bi("sample_2.pitch", "Pitch"), k("sample_2.decay", "Decay"), k("sample_2.body_send", "Send"), k("sample_2.vel_level", "Vel")],
    }
}

const TONE_BACK: &[KnobSpec] = &[
    k("tone.split", "Split"), k("tone.fast_time", "Fast Time"), k("tone.slow_time", "Slow Time"),
    k("tone.attack", "Attack"), k("tone.snap", "Snap"), k("tone.tail_level", "Tail"),
    k("tone.vel_level", "Vel Level"), k("tone.vel_pitch", "Vel Pitch"), k("tone.vel_shape", "Vel Shape"),
];
const BODY_BACK: &[KnobSpec] = &[
    k("body.damping", "Damping"), k("body.vel_brightness", "Vel Bright"), k("body.vel_bend", "Vel Bend"),
    k("body.vel_decay", "Vel Decay"),
];
const NOISE_BACK: &[KnobSpec] = &[
    k("noise.highpass", "Highpass"), k("noise.env_amount", "Sweep"), k("noise.attack", "Attack"),
    k("noise.snap", "Snap"), k("noise.tail_level", "Tail"), k("noise.clap", "Clap"),
    k("noise.density", "Density"), k("noise.crush", "Crush"), bi("noise.loop_pitch", "Loop Pitch"),
    k("noise.vel_level", "Vel Level"),
];

fn back_knobs(part: Part) -> &'static [KnobSpec] {
    match part {
        Part::Tone => TONE_BACK,
        Part::Body => BODY_BACK,
        Part::Noise => NOISE_BACK,
        Part::Sample1 | Part::Sample2 => &[],
    }
}

/// The effects, two rows of three small knobs; each row's last knob has its
/// mode chip to its right.
const EFFECTS: [[KnobSpec; 3]; 2] = [
    [k("voice.variation", "Variation"), k("crush.amount", "Crush"), k("drive.amount", "Drive")],
    [k("ring_mod.amount", "Ring"), k("comb.amount", "Comb"), k("compressor.threshold", "Comp")],
];
const EFFECT_CHIPS: [(&str, &str); 2] = [("drive.type", "Drive Type"), ("compressor.timing", "Comp Timing")];
/// The pad filter: Cutoff over Resonance.
const FILTER: [KnobSpec; 2] = [k("filter.cutoff", "Cutoff"), k("filter.resonance", "Resonance")];
const ENVELOPE: [KnobSpec; 4] =
    [k("envelope.attack", "Attack"), k("envelope.decay", "Decay"), k("envelope.curve", "Curve"), k("envelope.velocity", "Velocity")];
const SMALL_CELL: f32 = 54.0;
const VOLUME_W: f32 = 180.0;

impl Layout {
    pub fn build() -> Self {
        let mut l = Layout { top_bar: Rect::new(PAD, PAD, WIDTH - 2.0 * PAD, 28.0), ..Default::default() };
        // Volume, as a level bar with the meter in it; its reading goes to its right.
        let volume = Rect::new(WIDTH - PAD - 4.0 - 54.0 - VOLUME_W, PAD + 5.0, VOLUME_W, LEVEL_H);
        l.push("master.volume", "Volume", Kind::Level, volume, None, Side::Always);
        // Left of Volume's label: the export button, then the die.
        l.export = Rect::new(volume.x - 72.0 - 84.0, PAD + 4.0, 84.0, 20.0);
        l.die = Rect::new(l.export.x - 12.0 - 22.0, PAD + 3.0, 22.0, 22.0);

        let cards_y = l.top_bar.bottom() + GAP;
        for (i, part) in Part::ALL.into_iter().enumerate() {
            let rect = Rect::new(PAD + i as f32 * (CARD_W + GAP), cards_y, CARD_W, CARD_H);
            l.build_card(part, rect);
        }

        let bottom = Rect::new(PAD, cards_y + CARD_H + GAP, WIDTH - 2.0 * PAD, HEIGHT - PAD - (cards_y + CARD_H + GAP));
        let effects_w = 24.0 + 3.0 * SMALL_CELL + 2.0 * 4.0 + 8.0 + 64.0;
        let envelope_w = 24.0 + 4.0 * SMALL_CELL + 3.0 * 4.0;
        let filter_w = 24.0 + SMALL_CELL;
        let (screen, rest) = bottom.take_left(bottom.w - effects_w - filter_w - envelope_w - 3.0 * GAP, GAP);
        let (envelope, rest) = rest.take_left(envelope_w, GAP);
        let (filter, effects) = rest.take_left(filter_w, GAP);
        l.screen = screen;
        l.envelope = envelope;
        l.filter = filter;
        l.effects = effects;
        l.envelope_panel(envelope);
        l.filter_panel(filter);
        l.effects_panel(effects);
        l
    }

    fn push(&mut self, id: &'static str, label: &'static str, kind: Kind, rect: Rect, card: Option<Part>, side: Side) {
        self.controls.push(Control { id, label, kind, rect, card, side });
    }

    fn build_card(&mut self, part: Part, rect: Rect) {
        let inner = rect.inset(CARD_PAD_X, 8.0, CARD_PAD_X, 8.0);
        let title = Rect::new(inner.x, inner.y, inner.w - 30.0, 20.0);
        let has_back = !back_knobs(part).is_empty();
        let flip = has_back.then(|| Rect::new(inner.right() - 24.0, inner.y, 24.0, 20.0));
        let some = Some(part);

        // ---- Front, from the foot up: four knobs just above the level bar
        // (so the knob rows line up across the cards), the part's own
        // pickers above them, and the scope filling the rest.
        let knobs_top = inner.bottom() - LEVEL_H - 10.0 - KNOB_GRID_H;
        let mut pickers_top = knobs_top;
        match part {
            Part::Noise => {
                pickers_top -= 30.0;
                let (chip, stepper) = Rect::new(inner.x, pickers_top, inner.w, 22.0).take_left(48.0, 6.0);
                self.push("noise.generator", "Generator", Kind::Chip, chip, some, Side::Front);
                self.push("noise.loop", "Loop", Kind::Stepper, stepper, some, Side::Front);
            }
            Part::Sample1 | Part::Sample2 => {
                pickers_top -= 60.0;
                let set: &'static str = if part == Part::Sample1 { "sample_1.set" } else { "sample_2.set" };
                let sample: &'static str = if part == Part::Sample1 { "sample_1.sample" } else { "sample_2.sample" };
                for (v, r) in Rect::new(inner.x, pickers_top, inner.w, 20.0).columns(3, 4.0).into_iter().enumerate() {
                    self.push(set, "Set", Kind::Choice { value: v as f32 }, r, some, Side::Front);
                }
                self.push(sample, "Sample", Kind::Stepper, Rect::new(inner.x, pickers_top + 26.0, inner.w, 26.0), some, Side::Front);
            }
            _ => {}
        }
        let scope_top = inner.y + 28.0;
        let scope = Rect::new(inner.x, scope_top, inner.w, pickers_top - 10.0 - scope_top);
        let power = Rect::new(inner.x - 5.0, inner.y, 18.0, 20.0);
        self.cards.push(Card { part, rect, title, power, flip, scope });
        let knobs = front_knobs(part);
        let cells = Rect::new(inner.x, knobs_top, inner.w, KNOB_GRID_H).grid(2, 64.0, 6.0, 4.0, knobs.len());
        for ((id, label, bipolar), cell) in knobs.into_iter().zip(cells) {
            self.push(id, label, Kind::Knob { size: FRONT_KNOB, bipolar }, cell, some, Side::Front);
        }

        // ---- Back: everything else, smaller.
        let back = back_knobs(part);
        let cells = Rect::new(inner.x, inner.y + 28.0, inner.w, 240.0).grid(3, 54.0, 4.0, 2.0, back.len());
        let mut last = inner.y + 28.0;
        for (&(id, label, bipolar), cell) in back.iter().zip(cells) {
            last = cell.bottom();
            self.push(id, label, Kind::Knob { size: BACK_KNOB, bipolar }, cell, some, Side::Back);
        }
        if part == Part::Tone {
            self.push("tone.keytrack", "Keytrack", Kind::Toggle, Rect::new(inner.x, last + 8.0, inner.w, 22.0), some, Side::Back);
        }

        // ---- Both sides: the level bar.
        let level: &'static str = match part {
            Part::Tone => "tone.level",
            Part::Body => "body.level",
            Part::Noise => "noise.level",
            Part::Sample1 => "sample_1.level",
            Part::Sample2 => "sample_2.level",
        };
        let bar = Rect::new(inner.x + 40.0, inner.bottom() - LEVEL_H, inner.w - 40.0 - 54.0, LEVEL_H);
        self.push(level, "Level", Kind::Level, bar, some, Side::Always);
    }

    /// Header, then two rows of three small knobs, a mode chip after each row.
    fn effects_panel(&mut self, r: Rect) {
        let rows = r.inset(12.0, 26.0, 12.0, 4.0);
        for (row, (knobs, (chip, chip_label))) in EFFECTS.into_iter().zip(EFFECT_CHIPS).enumerate() {
            let y = rows.y + row as f32 * (SMALL_CELL + 2.0);
            for (col, (id, label, bipolar)) in knobs.into_iter().enumerate() {
                let cell = Rect::new(rows.x + col as f32 * (SMALL_CELL + 4.0), y, SMALL_CELL, SMALL_CELL);
                self.push(id, label, Kind::Knob { size: BACK_KNOB, bipolar }, cell, None, Side::Always);
            }
            let x = rows.x + 3.0 * SMALL_CELL + 2.0 * 4.0 + 8.0;
            self.push(chip, chip_label, Kind::Chip, Rect::new(x, y + BACK_KNOB / 2.0 - 9.0, rows.right() - x, 20.0), None, Side::Always);
        }
    }

    /// Header, then Cutoff over Resonance, on the effects' rows.
    fn filter_panel(&mut self, r: Rect) {
        let rows = r.inset(12.0, 26.0, 12.0, 4.0);
        for (row, (id, label, bipolar)) in FILTER.into_iter().enumerate() {
            let cell = Rect::new(rows.x, rows.y + row as f32 * (SMALL_CELL + 2.0), SMALL_CELL, SMALL_CELL);
            self.push(id, label, Kind::Knob { size: BACK_KNOB, bipolar }, cell, None, Side::Always);
        }
    }

    /// Header, the envelope's picture, then its four small knobs.
    fn envelope_panel(&mut self, r: Rect) {
        let inner = r.inset(12.0, 26.0, 12.0, 4.0);
        let knobs_y = inner.bottom() - SMALL_CELL;
        self.envelope_curve = Rect::new(inner.x, inner.y, inner.w, knobs_y - 8.0 - inner.y);
        for (i, (id, label, bipolar)) in ENVELOPE.into_iter().enumerate() {
            let cell = Rect::new(inner.x + i as f32 * (SMALL_CELL + 4.0), knobs_y, SMALL_CELL, SMALL_CELL);
            self.push(id, label, Kind::Knob { size: BACK_KNOB, bipolar }, cell, None, Side::Always);
        }
    }

    pub fn card(&self, part: Part) -> &Card {
        self.cards.iter().find(|c| c.part == part).expect("every part has a card")
    }

    /// Whether a part's card has a back to flip to.
    pub fn flips(part: Part) -> bool {
        !back_knobs(part).is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn everything_fits_and_nothing_overlaps_on_its_side() {
        let l = Layout::build();
        let panel = Rect::new(0.0, 0.0, WIDTH, HEIGHT);
        for c in &l.controls {
            assert!(c.rect.x >= panel.x && c.rect.right() <= panel.right() && c.rect.bottom() <= panel.bottom(), "{} off the panel", c.id);
            if let Some(part) = c.card {
                let card = l.card(part).rect;
                assert!(c.rect.x >= card.x && c.rect.right() <= card.right() && c.rect.bottom() <= card.bottom(), "{} leaves its card: {:?}", c.id, c.rect);
            }
        }
        for (i, a) in l.controls.iter().enumerate() {
            for b in &l.controls[i + 1..] {
                let shown_together = a.side == b.side || a.side == Side::Always || b.side == Side::Always;
                let overlap = a.rect.x < b.rect.right() && b.rect.x < a.rect.right() && a.rect.y < b.rect.bottom() && b.rect.y < a.rect.bottom();
                assert!(!(shown_together && overlap), "{} overlaps {}", a.id, b.id);
            }
        }
        assert!((l.effects.right() - (WIDTH - PAD)).abs() < 0.01);
        assert!(l.screen.w > 340.0, "the screen keeps its room: {}", l.screen.w);
        for panel in [l.envelope, l.filter, l.effects] {
            for c in l.controls.iter().filter(|c| c.card.is_none() && c.rect.y > l.screen.y) {
                if c.rect.x >= panel.x && c.rect.x < panel.right() {
                    assert!(c.rect.right() <= panel.right() - 8.0 && c.rect.bottom() <= panel.bottom(), "{} overflows its panel", c.id);
                }
            }
        }
        assert!(l.envelope_curve.h >= 30.0, "envelope picture too small: {}", l.envelope_curve.h);
    }
}
