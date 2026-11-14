//! The panel, "Fresco": a cracked, dark plaster wall in the apse of an abandoned church,
//! with Time inside a worn gilded halo whose rays fill in as the tail grows, modes as
//! Roman numerals, sliders cut as grooves, and Freeze as a sanctuary lamp, lit while
//! the tail is held. The Choir mode's own controls sit under the tail graph, tarnished
//! while another mode is chosen.
//!
//! Sliders jump to a press and drag from there; Shift is fine, double-click resets,
//! scroll nudges. The numerals drag up and down for Time. The modes and the lamp are
//! plain buttons.

use std::collections::VecDeque;
use std::f32::consts::PI;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use nih_plug::prelude::{Param, ParamPtr};
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::vizia::vg::{self, Align, Path};
use nih_plug_vizia::widgets::RawParamEvent;

use super::paint::*;
use crate::dsp::modes::{MODES, MODE_NAMES};
use crate::params::ApseParams;
use crate::telemetry::Telemetry;

pub const WIDTH: f32 = 860.0;
pub const HEIGHT: f32 = 520.0;

const MARGIN: f32 = 40.0;
const LEFT_WIDTH: f32 = 480.0;

/// The halo: its centre sits on the springline, the line the apse's half-dome rises from.
const HALO_X: f32 = MARGIN + LEFT_WIDTH / 2.0;
const SPRINGLINE: f32 = 236.0;
const HALO_R: f32 = 194.0;
const HALO_INNER_R: f32 = 180.0;
/// Worn gilding: lengths along each arc, drawn then bare, in turn.
const HALO_WEAR: [f32; 9] = [120.0, 9.0, 46.0, 4.0, 88.0, 14.0, 160.0, 6.0, 200.0];
const HALO_INNER_WEAR: [f32; 5] = [70.0, 18.0, 140.0, 30.0, 300.0];
const RAYS: usize = 13;
const RAY_FROM: f32 = 120.0;
const RAY_TO: f32 = 176.0;
/// How many rays are lit at the shortest Time; the rest light up as it grows.
const RAYS_ALWAYS: f32 = 5.0;

const NUMERALS: Rect = Rect { x: MARGIN, y: 70.0, w: LEFT_WIDTH, h: 130.0 };
const NUMERAL_BASELINE: f32 = 182.0;
const TIME_ROW: Rect = Rect { x: MARGIN, y: 244.0, w: LEFT_WIDTH, h: 28.0 };
const TIME_LABEL: f32 = 60.0;
const MODE_ROW: Rect = Rect { x: MARGIN, y: 278.0, w: LEFT_WIDTH, h: 40.0 };
const SLIDERS_Y: f32 = 330.0;
const SLIDER_ROW: f32 = 32.0;
const SLIDER_ROWS: usize = 5;
const SLIDER_GAP: f32 = 28.0;
const SLIDER_WIDTH: f32 = (LEFT_WIDTH + 20.0 - SLIDER_GAP) / 2.0;

const RIGHT_X: f32 = 600.0;
const RIGHT_WIDTH: f32 = 236.0;
const LAMP_X: f32 = 708.0;
const CHAIN_LENGTH: f32 = 38.0;
const LAMP: Rect = Rect { x: 628.0, y: 28.0, w: 160.0, h: 160.0 };
const TAIL: Rect = Rect { x: RIGHT_X, y: 214.0, w: RIGHT_WIDTH, h: 58.0 };
const CHOIR_HEADER: f32 = 300.0;
const CHOIR_ROWS_Y: f32 = 312.0;
const CHOIR_ROW: f32 = 30.0;
const LIMIT_METER: Rect = Rect { x: RIGHT_X, y: 496.0, w: RIGHT_WIDTH, h: 2.0 };
const LIMIT_FULL_DB: f32 = 12.0;

const TICK: Duration = Duration::from_millis(33);
const FINE: f32 = 0.12;
const NUMERALS_TRAVEL: f32 = 400.0;
const SCROLL_STEP: f32 = 0.01;
/// Limiting dies away on screen at this much per second, so short peaks can be seen.
const LIMIT_FALL_DB: f32 = 40.0;
/// The tail graph: one point per tick, about eight seconds across, this deep.
const TAIL_POINTS: usize = 242;
const TAIL_FLOOR_DB: f32 = -72.0;

/// Cracks in the plaster, kept to the bare wall between the controls.
const CRACKS: [&[(f32, f32)]; 5] = [
    &[(560.0, 0.0), (552.0, 46.0), (566.0, 90.0), (548.0, 150.0), (556.0, 214.0)],
    &[(552.0, 46.0), (528.0, 70.0)],
    &[(572.0, 520.0), (566.0, 470.0), (578.0, 430.0), (562.0, 380.0), (570.0, 340.0)],
    &[(566.0, 470.0), (590.0, 488.0)],
    &[(0.0, 150.0), (34.0, 138.0), (52.0, 160.0), (70.0, 152.0)],
];

const ROMAN: [&str; 6] = ["I", "II", "III", "IV", "V", "VI"];

const WORDMARK: TextStyle = TextStyle::new(Face::CinzelSemibold, 14.0, DIM, 5.6);
const NUMERAL: TextStyle = TextStyle::new(Face::Cinzel, 108.0, INK, 0.0);
const CAPTION: TextStyle = TextStyle::new(Face::Cinzel, 11.0, DIM, 3.5);
const LABEL: TextStyle = TextStyle::new(Face::Cinzel, 11.0, INK, 1.3);
const STRONG_LABEL: TextStyle = TextStyle::new(Face::CinzelSemibold, 12.0, INK, 2.2);
const MODE_NUMERAL: TextStyle = TextStyle::new(Face::Cinzel, 10.0, FAINT, 2.0);
const MODE_NAME: TextStyle = TextStyle::new(Face::Cinzel, 12.0, FAINT, 0.6);
const NUMBER: TextStyle = TextStyle::new(Face::Mono, 11.0, VALUE, 0.0);
const NOTE: TextStyle = TextStyle::new(Face::Italic, 16.0, DIM, 0.0);
const SMALL_NOTE: TextStyle = TextStyle::new(Face::Italic, 14.0, DIM, 0.0);

fn mode_cell(i: usize) -> Rect {
    let w = MODE_ROW.w / MODE_NAMES.len() as f32;
    Rect { x: MODE_ROW.x + i as f32 * w, w, ..MODE_ROW }
}

fn slider_row(i: usize) -> Rect {
    let (column, row) = (i / SLIDER_ROWS, i % SLIDER_ROWS);
    Rect { x: MARGIN + column as f32 * (SLIDER_WIDTH + SLIDER_GAP), y: SLIDERS_Y + row as f32 * SLIDER_ROW, w: SLIDER_WIDTH, h: SLIDER_ROW }
}

fn choir_row(i: usize) -> Rect {
    Rect { x: RIGHT_X, y: CHOIR_ROWS_Y + i as f32 * CHOIR_ROW, w: RIGHT_WIDTH, h: CHOIR_ROW }
}

/// The part of a slider row you can grab: right of the label, left of the value.
fn slider_track(row: Rect) -> Rect {
    const LABEL_WIDTH: f32 = 86.0;
    const VALUE_WIDTH: f32 = 74.0;
    Rect { x: row.x + LABEL_WIDTH, y: row.y, w: row.w - LABEL_WIDTH - VALUE_WIDTH, h: row.h }
}

fn time_track() -> Rect {
    Rect { x: TIME_ROW.x + TIME_LABEL, w: TIME_ROW.w - TIME_LABEL, ..TIME_ROW }
}

/// Time as the numerals show it: two decimals while it's short, one after.
fn numerals_text(seconds: f32) -> String {
    if seconds < 10.0 {
        format!("{seconds:.2}")
    } else {
        format!("{seconds:.1}")
    }
}

fn to_db(x: f32) -> f32 {
    20.0 * x.max(1e-9).log10()
}

/// How many of the halo's rays are lit for a Time knob position (0–1).
fn lit_rays(time_knob: f32) -> usize {
    (RAYS_ALWAYS + (RAYS as f32 - RAYS_ALWAYS) * time_knob).round() as usize
}

struct Slider {
    label: &'static str,
    param: ParamPtr,
    row: Rect,
    /// One of the Choir mode's own.
    choir: bool,
}

#[derive(Clone, Copy, PartialEq)]
enum Target {
    Slider(usize),
    Time,
    Numerals,
    Mode(usize),
    Freeze,
}

/// Sent to the panel by its ticker thread.
enum PanelEvent {
    Tick,
}

struct Drag {
    param: ParamPtr,
    vertical: bool,
    per_pixel: f32,
    anchor: f32,
    anchor_value: f32,
    fine: bool,
}

pub struct Panel {
    params: Arc<ApseParams>,
    telemetry: Arc<Telemetry>,
    sliders: Vec<Slider>,
    drag: Option<Drag>,
    limit_db: f32,
    /// The wet level, newest last, in dB.
    tail: VecDeque<f32>,
    stains: Vec<Stain>,
    /// Keeps the ticker thread running; cleared when the panel goes away.
    ticking: Arc<AtomicBool>,
}

impl Drop for Panel {
    fn drop(&mut self) {
        self.ticking.store(false, Ordering::Relaxed);
    }
}

impl Panel {
    pub fn new(cx: &mut Context, params: Arc<ApseParams>, telemetry: Arc<Telemetry>) -> Handle<'_, Self> {
        let ticking = Arc::new(AtomicBool::new(true));
        let ticker = ticking.clone();
        let list: [(&'static str, ParamPtr); 9] = [
            ("Size", params.size.as_ptr()),
            ("Pre Delay", params.predelay.as_ptr()),
            ("Diffusion", params.diffusion.as_ptr()),
            ("Mod", params.modulation.as_ptr()),
            ("Damp", params.damp.as_ptr()),
            ("Color", params.color.as_ptr()),
            ("Width", params.width.as_ptr()),
            ("Mix", params.mix.as_ptr()),
            ("Level", params.level.as_ptr()),
        ];
        let choir: [(&'static str, ParamPtr); 5] = [
            ("Vowel", params.vowel.as_ptr()),
            ("Drift", params.drift.as_ptr()),
            ("Resonance", params.resonance.as_ptr()),
            ("Ensemble", params.ensemble.as_ptr()),
            ("Breath", params.breath.as_ptr()),
        ];
        let general = list.iter().enumerate().map(|(i, (label, param))| Slider { label, param: *param, row: slider_row(i), choir: false });
        let singing = choir.iter().enumerate().map(|(i, (label, param))| Slider { label, param: *param, row: choir_row(i), choir: true });
        let sliders = general.chain(singing).collect();
        let tail = VecDeque::from(vec![TAIL_FLOOR_DB; TAIL_POINTS]);
        let stains = stains(WIDTH, HEIGHT);
        Self { params, telemetry, sliders, drag: None, limit_db: 0.0, tail, stains, ticking }
            .build(cx, move |cx| {
                // vizia's timers never fire under baseview, but events sent through a
                // context proxy are delivered every frame. The proxy targets this panel.
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

    fn transform(bounds: BoundingBox) -> (f32, f32, f32) {
        let scale = (bounds.w / WIDTH).min(bounds.h / HEIGHT);
        (scale, bounds.x + (bounds.w - WIDTH * scale) / 2.0, bounds.y + (bounds.h - HEIGHT * scale) / 2.0)
    }

    fn to_panel(cx: &EventContext, x: f32, y: f32) -> (f32, f32) {
        let (scale, ox, oy) = Self::transform(cx.bounds());
        ((x - ox) / scale, (y - oy) / scale)
    }

    fn hit(&self, x: f32, y: f32) -> Option<Target> {
        if time_track().contains(x, y) {
            return Some(Target::Time);
        }
        if LAMP.contains(x, y) {
            return Some(Target::Freeze);
        }
        if let Some(i) = (0..MODE_NAMES.len()).find(|i| mode_cell(*i).contains(x, y)) {
            return Some(Target::Mode(i));
        }
        if let Some(i) = self.sliders.iter().position(|s| slider_track(s.row).contains(x, y)) {
            return Some(Target::Slider(i));
        }
        NUMERALS.contains(x, y).then_some(Target::Numerals)
    }

    fn param_of(&self, target: Target) -> ParamPtr {
        match target {
            Target::Slider(i) => self.sliders[i].param,
            Target::Time | Target::Numerals => self.params.time.as_ptr(),
            Target::Mode(_) => self.params.mode.as_ptr(),
            Target::Freeze => self.params.freeze.as_ptr(),
        }
    }

    /// Works around a vizia bug: when the pointer goes past the window's right or bottom
    /// edge (during a drag, say), vizia marks the window as "not under the pointer" and
    /// only undoes that when the platform reports the cursor re-entering, which baseview
    /// on Windows never does. So, whenever the pointer is inside the window, say it entered.
    fn keep_hover_alive(cx: &mut EventContext) {
        let bounds = cx.bounds();
        let (x, y) = (cx.mouse().cursorx, cx.mouse().cursory);
        if x >= bounds.x && y >= bounds.y && x < bounds.x + bounds.w && y < bounds.y + bounds.h {
            cx.emit_custom(Event::new(WindowEvent::MouseEnter).origin(Entity::root()).direct(Entity::root()));
        }
    }

    fn tick(&mut self, cx: &mut EventContext) {
        Self::keep_hover_alive(cx);
        let fallen = (self.limit_db - LIMIT_FALL_DB * TICK.as_secs_f32()).max(0.0);
        self.limit_db = fallen.max(self.telemetry.take_reduction_db());
        self.tail.pop_front();
        self.tail.push_back(to_db(self.telemetry.take_wet_peak()).max(TAIL_FLOOR_DB));
        cx.needs_redraw();
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

    fn value_of(p: ParamPtr) -> f32 {
        unsafe { p.unmodulated_normalized_value() }
    }

    fn set_once(cx: &mut EventContext, p: ParamPtr, v: f32) {
        Self::begin(cx, p);
        Self::set(cx, p, v);
        Self::end(cx, p);
    }

    fn start_slider(&mut self, cx: &mut EventContext, p: ParamPtr, track: Rect, x: f32, fine: bool) {
        Self::begin(cx, p);
        let mut anchor_value = Self::value_of(p);
        if !fine {
            anchor_value = ((x - track.x) / track.w).clamp(0.0, 1.0);
            Self::set(cx, p, anchor_value);
        }
        self.drag = Some(Drag { param: p, vertical: false, per_pixel: 1.0 / track.w, anchor: x, anchor_value, fine });
        cx.capture();
    }

    fn mouse_down(&mut self, cx: &mut EventContext, x: f32, y: f32) {
        // A drag whose release never arrived: finish it before starting anything new.
        self.mouse_up(cx);
        let fine = cx.modifiers().contains(Modifiers::SHIFT);
        let Some(target) = self.hit(x, y) else { return };
        let p = self.param_of(target);
        match target {
            Target::Slider(i) => self.start_slider(cx, p, slider_track(self.sliders[i].row), x, fine),
            Target::Time => self.start_slider(cx, p, time_track(), x, fine),
            Target::Numerals => {
                Self::begin(cx, p);
                self.drag = Some(Drag { param: p, vertical: true, per_pixel: 1.0 / NUMERALS_TRAVEL, anchor: y, anchor_value: Self::value_of(p), fine });
                cx.capture();
            }
            Target::Mode(i) => Self::set_once(cx, p, i as f32 / (MODE_NAMES.len() - 1) as f32),
            Target::Freeze => Self::set_once(cx, p, if Self::value_of(p) > 0.5 { 0.0 } else { 1.0 }),
        }
    }

    fn mouse_move(&mut self, cx: &mut EventContext, x: f32, y: f32) {
        if self.drag.is_some() && cx.mouse().left.state != MouseButtonState::Pressed {
            self.mouse_up(cx); // the button is up but we never heard the release
        }
        let fine = cx.modifiers().contains(Modifiers::SHIFT);
        if let Some(drag) = &mut self.drag {
            let at = if drag.vertical { y } else { x };
            if fine != drag.fine {
                // Re-anchor, so changing speed never makes the value jump.
                drag.anchor_value = Self::value_of(drag.param);
                drag.anchor = at;
                drag.fine = fine;
            }
            let moved = if drag.vertical { drag.anchor - at } else { at - drag.anchor };
            let v = drag.anchor_value + moved * drag.per_pixel * if fine { FINE } else { 1.0 };
            Self::set(cx, drag.param, v);
            cx.needs_redraw();
        }
    }

    fn mouse_up(&mut self, cx: &mut EventContext) {
        if let Some(drag) = self.drag.take() {
            Self::end(cx, drag.param);
            cx.release();
        }
    }

    fn reset(&mut self, cx: &mut EventContext, x: f32, y: f32) {
        self.mouse_up(cx);
        match self.hit(x, y) {
            // The buttons already changed on the first press of the double-click.
            None | Some(Target::Freeze | Target::Mode(_)) => {}
            Some(target) => {
                let p = self.param_of(target);
                Self::set_once(cx, p, unsafe { p.default_normalized_value() });
            }
        }
    }

    fn scroll(&mut self, cx: &mut EventContext, x: f32, y: f32, notches: f32) {
        if let Some(target @ (Target::Slider(_) | Target::Time | Target::Numerals)) = self.hit(x, y) {
            let p = self.param_of(target);
            let step = SCROLL_STEP * if cx.modifiers().contains(Modifiers::SHIFT) { FINE } else { 1.0 };
            Self::set_once(cx, p, Self::value_of(p) + notches * step);
        }
    }

    fn paint_wall(&self, p: &mut Painter) {
        p.rect(Rect { x: 0.0, y: 0.0, w: WIDTH, h: HEIGHT }, PLASTER);
        self.stains.iter().for_each(|stain| p.stain(stain));
        for crack in CRACKS {
            // A lit lower lip, then the crack itself.
            let lip: Vec<_> = crack.iter().map(|(x, y)| (*x, y + 1.5)).collect();
            p.line(&lip, color(CRACK_EDGE), 0.8);
            p.line(crack, color(CRACK), 1.3);
        }
    }

    fn paint_halo(&self, p: &mut Painter) {
        let mut disc = Path::new();
        disc.move_to(HALO_X - HALO_R, SPRINGLINE);
        disc.arc(HALO_X, SPRINGLINE, HALO_R, PI, 2.0 * PI, vg::Solidity::Hole);
        disc.close();
        p.fill(&disc, faded(GOLD, 0.045));
        p.worn_arc(HALO_X, SPRINGLINE, HALO_R, &HALO_WEAR, faded(GILT, 0.85), 3.0);
        p.worn_arc(HALO_X, SPRINGLINE, HALO_INNER_R, &HALO_INNER_WEAR, faded(GILT, 0.45), 1.0);
        let lit = lit_rays(self.params.time.unmodulated_normalized_value());
        for i in 0..RAYS {
            let a = PI * (i + 1) as f32 / (RAYS + 1) as f32;
            let (dx, dy) = (-a.cos(), -a.sin());
            let ray = [(HALO_X + RAY_FROM * dx, SPRINGLINE + RAY_FROM * dy), (HALO_X + RAY_TO * dx, SPRINGLINE + RAY_TO * dy)];
            p.line(&ray, faded(GILT, if i < lit { 0.32 } else { 0.06 }), 1.0);
        }
        p.rect(Rect { x: MARGIN, y: SPRINGLINE, w: LEFT_WIDTH, h: 1.0 }, GROOVE_EDGE);
    }

    fn paint_time(&self, p: &mut Painter) {
        p.text(MARGIN, 31.0, "APSE", WORDMARK, Align::Left);
        p.text_on_baseline(HALO_X, NUMERAL_BASELINE, &numerals_text(self.params.time.unmodulated_plain_value()), NUMERAL, Align::Center);
        p.text(HALO_X, 214.0, "SECONDS TO SILENCE", CAPTION, Align::Center);
        p.text(TIME_ROW.x, TIME_ROW.middle(), "TIME", STRONG_LABEL, Align::Left);
        p.groove(time_track(), self.params.time.unmodulated_normalized_value(), true);
    }

    fn paint_modes(&self, p: &mut Painter) {
        let current = self.params.mode.unmodulated_plain_value() as usize;
        for (i, name) in MODE_NAMES.iter().enumerate() {
            let cell = mode_cell(i);
            let centre = cell.x + cell.w / 2.0;
            let ink = if i == current { GOLD_BRIGHT } else { FAINT };
            p.text(centre, cell.y + 9.0, ROMAN[i], MODE_NUMERAL.ink(ink), Align::Center);
            p.text(centre, cell.y + 25.0, &name.to_uppercase(), MODE_NAME.ink(ink), Align::Center);
            if i == current {
                p.rect(Rect { x: cell.x + 14.0, y: cell.y + cell.h - 2.0, w: cell.w - 28.0, h: 1.0 }, GOLD);
            }
        }
    }

    fn choir_chosen(&self) -> bool {
        MODES[self.params.mode.unmodulated_plain_value() as usize].choir
    }

    fn paint_sliders(&self, p: &mut Painter) {
        let singing = self.choir_chosen();
        for slider in &self.sliders {
            let row = slider.row;
            let lit = singing || !slider.choir;
            let (label, number) = if lit { (LABEL, NUMBER) } else { (LABEL.ink(DORMANT), NUMBER.ink(DORMANT)) };
            p.text(row.x, row.middle(), &slider.label.to_uppercase(), label, Align::Left);
            let value = Self::value_of(slider.param);
            p.groove(slider_track(row), value, lit);
            let shown = unsafe { slider.param.normalized_value_to_string(value, true) };
            p.text(row.x + row.w, row.middle(), &shown, number, Align::Right);
        }
    }

    fn paint_choir_header(&self, p: &mut Painter) {
        let singing = self.choir_chosen();
        p.text(RIGHT_X, CHOIR_HEADER, "CHOIR", STRONG_LABEL.ink(if singing { INK } else { DORMANT }), Align::Left);
        let note = if singing { "the tail is sung" } else { "sung in mode VI" };
        p.text(RIGHT_X + RIGHT_WIDTH, CHOIR_HEADER, note, SMALL_NOTE.ink(if singing { DIM } else { DORMANT }), Align::Right);
    }

    /// The sanctuary lamp: a red glass cup in a brass collar, hung on a chain.
    fn paint_lamp(&self, p: &mut Painter) {
        let lit = self.params.freeze.value();
        let (x, top) = (LAMP_X - 45.0, CHAIN_LENGTH);
        for y in (0..CHAIN_LENGTH as usize).step_by(5) {
            p.rect(Rect { x: LAMP_X - 0.75, y: y as f32, w: 1.5, h: 3.0 }, CHAIN);
        }
        if lit {
            p.glow(LAMP_X, top + 56.0, 70.0, 70.0, faded(EMBER, 0.5));
        }
        let shape = |points: &[(f32, f32)]| {
            let mut path = Path::new();
            for (i, (px, py)) in points.iter().enumerate() {
                if i == 0 {
                    path.move_to(x + px, top + py);
                } else {
                    path.line_to(x + px, top + py);
                }
            }
            path.close();
            path
        };
        for end in [18.0, 45.0, 72.0] {
            p.line(&[(LAMP_X, top), (x + end, top + 22.0)], color(CHAIN), 1.0);
        }
        p.fill(&shape(&[(14.0, 22.0), (76.0, 22.0), (70.0, 34.0), (20.0, 34.0)]), color(BRASS));
        let mut cup = Path::new();
        cup.move_to(x + 24.0, top + 34.0);
        cup.line_to(x + 66.0, top + 34.0);
        cup.line_to(x + 60.0, top + 74.0);
        cup.quad_to(x + 45.0, top + 86.0, x + 30.0, top + 74.0);
        cup.close();
        p.fill(&cup, color(if lit { LAMP_LIT } else { LAMP_DARK }));
        p.stroke(&cup, color(BRASS), 1.5);
        let mut flame = Path::new();
        flame.move_to(x + 45.0, top + 46.0);
        flame.quad_to(x + 52.0, top + 56.0, x + 45.0, top + 66.0);
        flame.quad_to(x + 38.0, top + 56.0, x + 45.0, top + 46.0);
        p.fill(&flame, color(if lit { FLAME } else { FLAME_DARK }));
        p.fill(&shape(&[(40.0, 84.0), (50.0, 84.0), (45.0, 96.0)]), color(BRASS));

        p.text(LAMP_X, top + 110.0, "FREEZE", STRONG_LABEL.ink(INK), Align::Center);
        let note = if lit { "the lamp is lit; nothing enters" } else { "light the lamp to hold the tail" };
        p.text(LAMP_X, top + 131.0, note, NOTE, Align::Center);
    }

    fn paint_tail(&self, p: &mut Painter) {
        let r = TAIL;
        p.text(r.x, r.y - 14.0, "TAIL", LABEL.ink(DIM), Align::Left);
        p.text(r.x + r.w, r.y - 14.0, "VIII S", LABEL.ink(DIM), Align::Right);
        for third in [1.0, 2.0] {
            p.rect(Rect { y: r.y + r.h * third / 3.0, h: 1.0, ..r }, RULE);
        }
        p.rect(Rect { y: r.y + r.h - 1.0, h: 1.0, ..r }, GROOVE_EDGE);
        let y_of = |db: f32| r.y + r.h * (db / TAIL_FLOOR_DB).clamp(0.0, 1.0);
        let step = r.w / (TAIL_POINTS - 1) as f32;
        let mut path = Path::new();
        path.move_to(r.x, r.y + r.h);
        for (i, db) in self.tail.iter().enumerate() {
            path.line_to(r.x + i as f32 * step, y_of(*db));
        }
        path.line_to(r.x + r.w, r.y + r.h);
        path.close();
        p.fill(&path, faded(GOLD, 0.5));
    }

    fn paint_limit(&self, p: &mut Painter) {
        let meter = LIMIT_METER;
        p.text(meter.x, meter.y - 14.0, "LIMIT", LABEL.ink(DIM), Align::Left);
        p.text(meter.x + meter.w, meter.y - 14.0, "-0.5 dBFS", NUMBER.ink(DIM), Align::Right);
        p.rect(meter, RULE);
        let depth = (self.limit_db / LIMIT_FULL_DB).min(1.0);
        if depth > 0.0 {
            p.rect(Rect { w: meter.w * depth, ..meter }, LIMIT);
        }
    }
}

impl View for Panel {
    fn element(&self) -> Option<&'static str> {
        Some("apse-panel")
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
            let (x, y) = Self::to_panel(cx, cx.mouse().cursorx, cx.mouse().cursory);
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
                    self.reset(cx, x, y);
                    meta.consume();
                }
                WindowEvent::MouseUp(MouseButton::Left) => self.mouse_up(cx),
                WindowEvent::MouseScroll(_, notches) => {
                    self.scroll(cx, x, y, notches);
                    meta.consume();
                }
                _ => return,
            }
            cx.needs_redraw();
        });
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        let (scale, ox, oy) = Self::transform(cx.bounds());
        canvas.save();
        canvas.translate(ox, oy);
        canvas.scale(scale, scale);
        let fonts = fonts(canvas);
        let mut p = Painter { canvas, fonts };
        self.paint_wall(&mut p);
        self.paint_halo(&mut p);
        self.paint_time(&mut p);
        self.paint_modes(&mut p);
        self.paint_sliders(&mut p);
        self.paint_choir_header(&mut p);
        self.paint_lamp(&mut p);
        self.paint_tail(&mut p);
        self.paint_limit(&mut p);
        canvas.restore();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_layout_fits() {
        let last = slider_row(8); // the ninth slider
        assert!(last.y + last.h <= HEIGHT);
        assert!(slider_row(SLIDER_ROWS).x + SLIDER_WIDTH <= RIGHT_X);
        assert!(MODE_ROW.y + MODE_ROW.h <= SLIDERS_Y);
        assert!(LAMP.y + LAMP.h < TAIL.y - 20.0);
        assert!(TAIL.y + TAIL.h < CHOIR_HEADER - 14.0);
        let last_choir = choir_row(4);
        assert!(last_choir.y + last_choir.h < LIMIT_METER.y - 20.0);
        assert_eq!(ROMAN.len(), MODE_NAMES.len());
    }

    #[test]
    fn rays_light_up_with_time() {
        assert_eq!(lit_rays(0.0), 5);
        assert_eq!(lit_rays(1.0), RAYS);
    }

    #[test]
    fn numerals_stay_short() {
        assert_eq!(numerals_text(2.5), "2.50");
        assert_eq!(numerals_text(30.0), "30.0");
    }
}
