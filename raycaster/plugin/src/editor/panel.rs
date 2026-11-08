//! The panel's layout, drawing and mouse handling. Knobs drag up and down (Shift is fine),
//! double-click resets, scroll nudges. Clicking any modulatable knob makes it the one the
//! dynamic modulator along the bottom shows and edits; its face then fills with the current
//! oscillator's backdrop in miniature. Arcs round a modulated knob show its LFO range
//! (amber) and envelope (teal), and a dot where the newest voice has it now. Each envelope
//! segment that can bend has a small button at its knob's top right: hold it and drag up
//! or down to bow the curve, which shows large while held.

use std::cell::RefCell;
use std::f32::consts::{FRAC_PI_2, PI};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use nih_plug::prelude::{Param, ParamPtr};
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::vizia::vg::{Align, Paint, Path};
use nih_plug_vizia::widgets::RawParamEvent;

use super::backdrops::{self, accent, Area};
use super::painter::{color, fonts, Face, Painter};
use crate::dsp::engine::{FilterKind, Shown};
use crate::dsp::geometry::{Bend, Bent, Outline};
use crate::dsp::envelope::along;
use crate::dsp::modulator::{sketch, LfoShape, SHAPES};
use crate::dsp::shapes::{chaos, Element, ALL, MAX_SEED};
use crate::dsp::targets::{global_target, osc_target, range_of, target_name, GlobalTarget, OscTarget, OSCS, OSC_TARGETS, ROMAN};
use crate::params::RaycasterParams;
use crate::telemetry::Telemetry;

pub const WIDTH: f32 = 944.0;
pub const HEIGHT: f32 = 700.0;

const GROUND: u32 = 0x131211;
const PANEL: u32 = 0x1b1a17;
const EDGE: u32 = 0x302d28;
const INK: u32 = 0xece5d7;
const SOFT: u32 = 0xb9b0a1;
const DIM: u32 = 0x887f72;
const FAINT: u32 = 0x4f4a43;
const TRACK: u32 = 0x37332e;
const HOLLOW: u32 = 0x0e0d0c;
const LFO: u32 = 0xe8b44e;
const ENV: u32 = 0x78c3d2;
/// The mod wheel and aftertouch.
const HAND: u32 = 0xd99ac0;

/// A knob sweeps 270 degrees, from bottom left to bottom right.
const SWEEP: f32 = 1.5 * PI;
const START: f32 = FRAC_PI_2 + 0.25 * PI;
const GRAB: f32 = 8.0;
const DRAG_PIXELS: f32 = 200.0;
const FINE: f32 = 0.12;
const SCROLL_STEP: f32 = 0.01;
const TICK: Duration = Duration::from_millis(33);

#[derive(Clone, Copy)]
struct Rect {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

impl Rect {
    const fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }

    fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && y >= self.y && x < self.x + self.w && y < self.y + self.h
    }
}

// The four panels.
const OSC_PANEL: Rect = Rect::new(16.0, 68.0, 504.0, 404.0);
const FILTER_PANEL: Rect = Rect::new(532.0, 68.0, 396.0, 212.0);
const AMP_PANEL: Rect = Rect::new(532.0, 292.0, 396.0, 180.0);
const MOD_PANEL: Rect = Rect::new(16.0, 484.0, 912.0, 200.0);

// The oscillator page.
const WAVE: Rect = Rect::new(32.0, 152.0, 196.0, 196.0);
const GRID_X: [f32; 3] = [290.0, 374.0, 458.0];
const GRID_Y: [f32; 3] = [186.0, 290.0, 394.0];
const UNDER_X: [f32; 3] = [66.0, 130.0, 194.0];
const UNDER_Y: f32 = 400.0;
/// In the grid's empty corner.
const FREE: Rect = Rect::new(430.0, 382.0, 56.0, 24.0);

// The dynamic modulator.
const LFO_SCOPE: Rect = Rect::new(230.0, 550.0, 294.0, 44.0);
const AD_SCOPE: Rect = Rect::new(706.0, 516.0, 206.0, 40.0);
const SYNC: Rect = Rect::new(536.0, 626.0, 58.0, 22.0);
const RETRIG: Rect = Rect::new(620.0, 626.0, 62.0, 22.0);
const HAND_Y: f32 = 630.0;
const MOD_ENV_Y: f32 = 616.0;
const FILTER_Y: f32 = 146.0;
const FILTER_X: [f32; 6] = [568.0, 632.0, 696.0, 760.0, 824.0, 888.0];
/// The curve popup, while a curve button is held.
const POPUP_W: f32 = 150.0;
const POPUP_H: f32 = 112.0;

const FILTER_PILLS: [(FilterKind, Rect); 2] =
    [(FilterKind::Svf, Rect::new(800.0, 80.0, 54.0, 24.0)), (FilterKind::Ladder, Rect::new(858.0, 80.0, 62.0, 24.0))];
const MONO_PILLS: [(bool, Rect); 2] = [(false, Rect::new(412.0, 18.0, 56.0, 24.0)), (true, Rect::new(472.0, 18.0, 56.0, 24.0))];
const TOP_X: [f32; 6] = [572.0, 636.0, 700.0, 764.0, 828.0, 892.0];

fn page_rect(i: usize) -> Rect {
    Rect::new(32.0 + i as f32 * 50.0, 82.0, 44.0, 56.0)
}

fn element_rect(i: usize) -> Rect {
    let (row, col) = (i / 4, i % 4);
    Rect::new(140.0 + col as f32 * 92.0, 82.0 + row as f32 * 30.0, 88.0, 26.0)
}

fn shape_rect(i: usize) -> Rect {
    Rect::new(230.0 + i as f32 * 42.0, 516.0, 38.0, 28.0)
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Hit {
    Knob(usize),
    Page(usize),
    Element(Element),
    Filter(FilterKind),
    Mono(bool),
    Shape(LfoShape),
    Sync,
    Retrig,
    Free,
    Curve(usize),
}

#[derive(Clone, Copy)]
struct Knob {
    label: &'static str,
    param: ParamPtr,
    x: f32,
    y: f32,
    r: f32,
    bipolar: bool,
    target: Option<usize>,
    /// Small knobs show their label, and their value only while touched.
    small: bool,
    hue: u32,
    /// Shown dimmed when it does nothing (the SVF's mode with the ladder chosen).
    enabled: bool,
}

/// A segment's curve button: `param` is its bow; `rising` is which way the segment runs.
#[derive(Clone, Copy)]
struct CurveButton {
    param: ParamPtr,
    name: &'static str,
    x: f32,
    y: f32,
    rising: bool,
    hue: u32,
}

const CURVE_SIZE: f32 = 13.0;

enum PanelEvent {
    Tick,
}

struct Drag {
    param: ParamPtr,
    anchor_y: f32,
    anchor_value: f32,
    fine: bool,
    /// Dragging a curve button: which one, for the popup.
    curve: Option<usize>,
}

fn clock_seed() -> u32 {
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.subsec_nanos() ^ d.as_secs() as u32).unwrap_or(1);
    nanos.wrapping_mul(0x9E37_79B1) ^ std::process::id()
}

pub struct Panel {
    params: Arc<RaycasterParams>,
    telemetry: Arc<Telemetry>,
    page: usize,
    selected: usize,
    shown: Shown,
    elements: Vec<Outline>,
    chaos: RefCell<[(u32, Outline); OSCS]>,
    bent: RefCell<Box<Bent>>,
    opened: Instant,
    drag: Option<Drag>,
    hover: Option<Hit>,
    ticking: Arc<AtomicBool>,
}

impl Drop for Panel {
    fn drop(&mut self) {
        self.ticking.store(false, Ordering::Relaxed);
    }
}

impl Panel {
    pub fn new(cx: &mut Context, params: Arc<RaycasterParams>, telemetry: Arc<Telemetry>) -> Handle<'_, Self> {
        let ticking = Arc::new(AtomicBool::new(true));
        let ticker = ticking.clone();
        Self {
            elements: ALL[..6].iter().map(|e| e.outline()).collect(),
            chaos: RefCell::new(std::array::from_fn(|_| (u32::MAX, Outline::default()))),
            bent: RefCell::new(Box::default()),
            shown: telemetry.snapshot(),
            params,
            telemetry,
            page: 0,
            selected: osc_target(0, OscTarget::Squish),
            opened: Instant::now(),
            drag: None,
            hover: None,
            ticking,
        }
        .build(cx, move |cx| {
            // vizia's timers never fire under baseview, but events sent through a context
            // proxy are delivered every frame. The proxy targets this panel.
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

    fn osc(&self) -> &crate::params::OscParams {
        &self.params.osc[self.page]
    }

    fn page_hue(&self) -> u32 {
        accent(self.osc().element.value())
    }

    fn knobs(&self) -> Vec<Knob> {
        let o = self.page;
        let p = self.osc();
        let hue = self.page_hue();
        let knob = |label, param: ParamPtr, x, y, r, bipolar, target, small, hue| Knob { label, param, x, y, r, bipolar, target, small, hue, enabled: true };
        let t = |k| Some(osc_target(o, k));
        let g = |k| Some(global_target(k));
        let mods = &self.params.mods[self.selected];
        let rate = if mods.sync.value() { mods.division.as_ptr() } else { mods.rate.as_ptr() };
        let params = &self.params;
        vec![
            // Oscillator.
            knob("Octave", p.octave.as_ptr(), UNDER_X[0], UNDER_Y, 18.0, true, None, false, hue),
            knob("Semi", p.semi.as_ptr(), UNDER_X[1], UNDER_Y, 18.0, true, None, false, hue),
            knob("Fine", p.fine.as_ptr(), UNDER_X[2], UNDER_Y, 18.0, true, t(OscTarget::Fine), false, hue),
            knob("Squish", p.squish.as_ptr(), GRID_X[0], GRID_Y[0], 22.0, true, t(OscTarget::Squish), false, hue),
            knob("Skew", p.skew.as_ptr(), GRID_X[1], GRID_Y[0], 22.0, true, t(OscTarget::Skew), false, hue),
            knob("Tilt", p.tilt.as_ptr(), GRID_X[2], GRID_Y[0], 22.0, true, t(OscTarget::Tilt), false, hue),
            knob("Center", p.center.as_ptr(), GRID_X[0], GRID_Y[1], 22.0, true, t(OscTarget::Center), false, hue),
            knob("Ray", p.ray.as_ptr(), GRID_X[1], GRID_Y[1], 22.0, false, t(OscTarget::Ray), false, hue),
            knob("Level", p.level.as_ptr(), GRID_X[2], GRID_Y[1], 22.0, false, t(OscTarget::Level), false, hue),
            knob("Orbit", p.orbit.as_ptr(), GRID_X[0], GRID_Y[2], 22.0, false, t(OscTarget::Orbit), false, hue),
            knob("Rate", p.orbit_hz.as_ptr(), GRID_X[1], GRID_Y[2], 22.0, false, t(OscTarget::OrbitRate), false, hue),
            // Filter.
            knob("Drive", params.drive.as_ptr(), FILTER_X[0], FILTER_Y, 18.0, false, g(GlobalTarget::Drive), false, INK),
            knob("Cutoff", params.cutoff.as_ptr(), FILTER_X[1], FILTER_Y, 18.0, false, g(GlobalTarget::Cutoff), false, INK),
            knob("Res", params.resonance.as_ptr(), FILTER_X[2], FILTER_Y, 18.0, false, g(GlobalTarget::Resonance), false, INK),
            Knob {
                enabled: params.filter.value() == FilterKind::Svf,
                ..knob("Mode", params.filter_mode.as_ptr(), FILTER_X[3], FILTER_Y, 18.0, false, g(GlobalTarget::FilterMode), false, INK)
            },
            knob("Env", params.filter_env.as_ptr(), FILTER_X[4], FILTER_Y, 18.0, true, g(GlobalTarget::FilterEnv), false, INK),
            knob("Key", params.keytrack.as_ptr(), FILTER_X[5], FILTER_Y, 18.0, false, None, false, INK),
            knob("A", params.filter_attack.as_ptr(), 654.0, 240.0, 14.0, false, None, true, INK),
            knob("D", params.filter_decay.as_ptr(), 716.0, 240.0, 14.0, false, None, true, INK),
            knob("S", params.filter_sustain.as_ptr(), 778.0, 240.0, 14.0, false, None, true, INK),
            knob("R", params.filter_release.as_ptr(), 840.0, 240.0, 14.0, false, None, true, INK),
            // Amp.
            knob("Attack", params.amp_attack.as_ptr(), 576.0, 376.0, 22.0, false, None, false, INK),
            knob("Decay", params.amp_decay.as_ptr(), 654.0, 376.0, 22.0, false, None, false, INK),
            knob("Sustain", params.amp_sustain.as_ptr(), 732.0, 376.0, 22.0, false, None, false, INK),
            knob("Release", params.amp_release.as_ptr(), 810.0, 376.0, 22.0, false, None, false, INK),
            knob("Velocity", params.velocity.as_ptr(), 888.0, 376.0, 22.0, false, None, false, INK),
            // Top bar.
            knob("Unison", params.unison.as_ptr(), TOP_X[0], 30.0, 14.0, false, None, true, INK),
            knob("Detune", params.detune.as_ptr(), TOP_X[1], 30.0, 14.0, false, None, true, INK),
            knob("Glide", params.glide.as_ptr(), TOP_X[2], 30.0, 14.0, false, None, true, INK),
            knob("Width", params.width.as_ptr(), TOP_X[3], 30.0, 14.0, false, g(GlobalTarget::Width), true, INK),
            knob("Chorus", params.chorus.as_ptr(), TOP_X[4], 30.0, 14.0, false, g(GlobalTarget::Chorus), true, INK),
            knob("Volume", params.volume.as_ptr(), TOP_X[5], 30.0, 14.0, false, None, true, INK),
            // The selected control's modulator.
            knob("Rate", rate, 566.0, 556.0, 22.0, false, None, false, LFO),
            knob("LFO", mods.lfo.as_ptr(), 650.0, 556.0, 22.0, true, None, false, LFO),
            knob("Wheel", mods.wheel.as_ptr(), 268.0, HAND_Y, 16.0, true, None, false, HAND),
            knob("Touch", mods.touch.as_ptr(), 344.0, HAND_Y, 16.0, true, None, false, HAND),
            knob("Attack", mods.attack.as_ptr(), 736.0, MOD_ENV_Y, 22.0, false, None, false, ENV),
            knob("Decay", mods.decay.as_ptr(), 812.0, MOD_ENV_Y, 22.0, false, None, false, ENV),
            knob("Env", mods.env.as_ptr(), 888.0, MOD_ENV_Y, 22.0, true, None, false, ENV),
        ]
    }

    /// The curve buttons, each at its segment's knob's top right.
    fn curves(&self) -> Vec<CurveButton> {
        let params = &self.params;
        let mods = &self.params.mods[self.selected];
        let at = |param: ParamPtr, name, x: f32, y: f32, r: f32, rising, hue| CurveButton {
            param,
            name,
            x: x + 0.62 * r + 2.0,
            y: y - r - CURVE_SIZE + 4.0,
            rising,
            hue,
        };
        vec![
            at(params.filter_attack_curve.as_ptr(), "Filter attack", 654.0, 240.0, 14.0, true, INK),
            at(params.filter_decay_curve.as_ptr(), "Filter decay", 716.0, 240.0, 14.0, false, INK),
            at(params.filter_release_curve.as_ptr(), "Filter release", 840.0, 240.0, 14.0, false, INK),
            at(params.amp_attack_curve.as_ptr(), "Amp attack", 576.0, 376.0, 22.0, true, INK),
            at(params.amp_decay_curve.as_ptr(), "Amp decay", 654.0, 376.0, 22.0, false, INK),
            at(params.amp_release_curve.as_ptr(), "Amp release", 810.0, 376.0, 22.0, false, INK),
            at(mods.attack_curve.as_ptr(), "Env attack", 736.0, MOD_ENV_Y, 22.0, true, ENV),
            at(mods.decay_curve.as_ptr(), "Env decay", 812.0, MOD_ENV_Y, 22.0, false, ENV),
        ]
    }

    fn hit(&self, x: f32, y: f32) -> Option<Hit> {
        let rects = (0..OSCS)
            .map(|i| (page_rect(i), Hit::Page(i)))
            .chain(ALL.iter().enumerate().map(|(i, e)| (element_rect(i), Hit::Element(*e))))
            .chain(FILTER_PILLS.iter().map(|(k, r)| (*r, Hit::Filter(*k))))
            .chain(MONO_PILLS.iter().map(|(m, r)| (*r, Hit::Mono(*m))))
            .chain(SHAPES.iter().enumerate().map(|(i, s)| (shape_rect(i), Hit::Shape(*s))))
            .chain([(SYNC, Hit::Sync), (RETRIG, Hit::Retrig), (FREE, Hit::Free)])
            .chain(self.curves().into_iter().enumerate().map(|(i, c)| (Rect::new(c.x - 2.0, c.y - 2.0, CURVE_SIZE + 4.0, CURVE_SIZE + 4.0), Hit::Curve(i))));
        for (rect, hit) in rects {
            if rect.contains(x, y) {
                return Some(hit);
            }
        }
        self.knobs().iter().position(|k| (x - k.x).hypot(y - k.y) <= k.r + GRAB).map(Hit::Knob)
    }

    /// Works around a vizia bug (see Argent): whenever the pointer is inside the window,
    /// say it entered, so hover keeps working after a drag leaves the window.
    fn keep_hover_alive(cx: &mut EventContext) {
        let bounds = cx.bounds();
        let (x, y) = (cx.mouse().cursorx, cx.mouse().cursory);
        if x >= bounds.x && y >= bounds.y && x < bounds.x + bounds.w && y < bounds.y + bounds.h {
            cx.emit_custom(Event::new(WindowEvent::MouseEnter).origin(Entity::root()).direct(Entity::root()));
        }
    }

    fn tick(&mut self, cx: &mut EventContext) {
        Self::keep_hover_alive(cx);
        self.shown = self.telemetry.snapshot();
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

    fn toggle(cx: &mut EventContext, p: ParamPtr) {
        let v = Self::value_of(p);
        Self::set_once(cx, p, if v > 0.5 { 0.0 } else { 1.0 });
    }

    fn mouse_down(&mut self, cx: &mut EventContext, x: f32, y: f32) {
        self.mouse_up(cx);
        let Some(hit) = self.hit(x, y) else { return };
        let mods = &self.params.mods[self.selected];
        match hit {
            Hit::Page(i) => {
                if self.selected < OSCS * OSC_TARGETS.len() {
                    self.selected = (self.selected % OSC_TARGETS.len()) + i * OSC_TARGETS.len();
                }
                self.page = i;
            }
            Hit::Element(e) => {
                let p = self.osc();
                Self::set_once(cx, p.element.as_ptr(), p.element.preview_normalized(e));
                if e == Element::Chaos {
                    // Every click on Chaos is a new one.
                    let seed = 1 + clock_seed() % MAX_SEED as u32;
                    Self::set_once(cx, p.seed.as_ptr(), p.seed.preview_normalized(seed as i32));
                }
            }
            Hit::Filter(kind) => Self::set_once(cx, self.params.filter.as_ptr(), self.params.filter.preview_normalized(kind)),
            Hit::Mono(mono) => Self::set_once(cx, self.params.mono.as_ptr(), if mono { 1.0 } else { 0.0 }),
            Hit::Shape(shape) => Self::set_once(cx, mods.shape.as_ptr(), mods.shape.preview_normalized(shape)),
            Hit::Sync => Self::toggle(cx, mods.sync.as_ptr()),
            Hit::Retrig => Self::toggle(cx, mods.retrig.as_ptr()),
            Hit::Free => Self::toggle(cx, self.osc().free.as_ptr()),
            Hit::Knob(i) => {
                let knob = self.knobs()[i];
                if let Some(t) = knob.target {
                    self.selected = t;
                }
                self.grab(cx, knob.param, y, None);
            }
            Hit::Curve(i) => self.grab(cx, self.curves()[i].param, y, Some(i)),
        }
        cx.needs_redraw();
    }

    fn grab(&mut self, cx: &mut EventContext, param: ParamPtr, y: f32, curve: Option<usize>) {
        Self::begin(cx, param);
        let fine = cx.modifiers().contains(Modifiers::SHIFT);
        self.drag = Some(Drag { param, anchor_y: y, anchor_value: Self::value_of(param), fine, curve });
        cx.capture();
    }

    fn mouse_move(&mut self, cx: &mut EventContext, x: f32, y: f32) {
        if self.drag.is_some() && cx.mouse().left.state != MouseButtonState::Pressed {
            self.mouse_up(cx);
        }
        let hover = self.hit(x, y);
        if hover != self.hover {
            self.hover = hover;
            cx.needs_redraw();
        }
        let fine = cx.modifiers().contains(Modifiers::SHIFT);
        let Some(drag) = &mut self.drag else { return };
        if fine != drag.fine {
            drag.anchor_value = Self::value_of(drag.param);
            drag.anchor_y = y;
            drag.fine = fine;
        }
        let v = drag.anchor_value + (drag.anchor_y - y) / DRAG_PIXELS * if fine { FINE } else { 1.0 };
        Self::set(cx, drag.param, v);
        cx.needs_redraw();
    }

    fn mouse_up(&mut self, cx: &mut EventContext) {
        let Some(drag) = self.drag.take() else { return };
        Self::end(cx, drag.param);
        cx.release();
        cx.needs_redraw();
    }

    fn reset(&mut self, cx: &mut EventContext, x: f32, y: f32) {
        self.mouse_up(cx);
        let p = match self.hit(x, y) {
            Some(Hit::Knob(i)) => self.knobs()[i].param,
            Some(Hit::Curve(i)) => self.curves()[i].param,
            _ => return,
        };
        Self::set_once(cx, p, unsafe { p.default_normalized_value() });
    }

    fn scroll(&mut self, cx: &mut EventContext, x: f32, y: f32, notches: f32) {
        let p = match self.hit(x, y) {
            Some(Hit::Knob(i)) => self.knobs()[i].param,
            Some(Hit::Curve(i)) => self.curves()[i].param,
            _ => return,
        };
        let step = match unsafe { p.step_count() } {
            Some(steps) => 1.0 / steps as f32,
            None => SCROLL_STEP * if cx.modifiers().contains(Modifiers::SHIFT) { FINE } else { 1.0 },
        };
        Self::set_once(cx, p, Self::value_of(p) + notches * step);
    }

    // ---------------------------------------------------------------- drawing

    /// Runs `f` on oscillator `osc`'s outline: its element's, or its Chaos shape (rebuilt
    /// here when the seed changes, exactly as the plugin builds it).
    fn with_outline<R>(&self, osc: usize, f: impl FnOnce(&Outline) -> R) -> R {
        let element = self.params.osc[osc].element.value();
        if element != Element::Chaos {
            return f(&self.elements[element.index()]);
        }
        let seed = self.params.osc[osc].seed.value() as u32;
        let mut cache = self.chaos.borrow_mut();
        if cache[osc].0 != seed {
            cache[osc] = (seed, chaos(seed).0);
        }
        f(&cache[osc].1)
    }

    fn chaos_loops(&self) -> Vec<Vec<(f32, f32)>> {
        if self.osc().element.value() != Element::Chaos {
            return Vec::new();
        }
        self.with_outline(self.page, |o| o.loops().into_iter().map(|l| l.into_iter().map(|[x, y]| (x, y)).collect()).collect())
    }

    fn backdrop(&self, p: &mut Painter, area: Area, t: f32) {
        let loops = self.chaos_loops();
        backdrops::paint(p, self.osc().element.value(), area, t, &loops);
    }

    fn panel_box(p: &mut Painter, r: Rect) {
        p.rounded(r.x, r.y, r.w, r.h, 6.0, Some(color(PANEL, 1.0)), Some((color(EDGE, 1.0), 1.0)));
    }

    fn pill(p: &mut Painter, r: Rect, label: &str, on: bool, hovered: bool, hue: u32) {
        let fill = if on { Some(color(hue, 0.18)) } else { None };
        let edge = if on { color(hue, 0.9) } else if hovered { color(SOFT, 0.8) } else { color(FAINT, 1.0) };
        p.rounded(r.x + 0.5, r.y + 0.5, r.w - 1.0, r.h - 1.0, 4.0, fill, Some((edge, 1.0)));
        let ink = if on { color(INK, 1.0) } else { color(DIM, 1.0) };
        p.text(r.x + r.w / 2.0, r.y + r.h / 2.0 + 0.5, label, Face::Label, 12.0, ink, Align::Center);
    }

    fn knob_angle(v: f32) -> f32 {
        START + SWEEP * v.clamp(0.0, 1.0)
    }

    fn paint_knob(&self, p: &mut Painter, k: &Knob, hovered: bool, t: f32) {
        let v = Self::value_of(k.param);
        let (x, y, r) = (k.x, k.y, k.r);
        let selected = k.target == Some(self.selected);
        let fade = if k.enabled { 1.0 } else { 0.35 };
        p.fill_circle(x, y, r - 3.0, color(HOLLOW, 0.75));
        if selected {
            let face = r - 3.0;
            self.backdrop(p, Area { x: x - face, y: y - face, w: 2.0 * face, h: 2.0 * face, round: true }, t);
        }
        p.arc(x, y, r, START, START + SWEEP, color(TRACK, 1.0), 2.5);
        let from: f32 = if k.bipolar { 0.5 } else { 0.0 };
        let (a0, a1) = (Self::knob_angle(from.min(v)), Self::knob_angle(from.max(v)));
        if (a1 - a0).abs() > 0.01 {
            p.arc(x, y, r, a0, a1, color(k.hue, fade * if hovered { 1.0 } else { 0.9 }), 2.5);
        }
        let a = Self::knob_angle(v);
        p.line(x + 0.35 * r * a.cos(), y + 0.35 * r * a.sin(), x + (r - 4.0) * a.cos(), y + (r - 4.0) * a.sin(), color(INK, fade), 2.0);

        if let Some(t) = k.target {
            let mods = &self.params.mods[t];
            let (lfo, env) = (mods.lfo.value(), mods.env.value());
            if lfo.abs() > 1e-4 {
                p.arc(x, y, r + 5.0, Self::knob_angle(v - lfo.abs()), Self::knob_angle(v + lfo.abs()), color(LFO, 0.85), 2.0);
            }
            if env.abs() > 1e-4 {
                let (e0, e1) = (Self::knob_angle(v.min(v + env)), Self::knob_angle(v.max(v + env)));
                p.arc(x, y, r + 9.0, e0, e1, color(ENV, 0.85), 2.0);
            }
            if (mods.active() || mods.held()) && self.shown.sounding {
                let live = Self::knob_angle(self.shown.values[t]);
                p.fill_circle(x + (r + 5.0) * live.cos(), y + (r + 5.0) * live.sin(), 3.0, color(INK, 1.0));
            }
        }

        let touched = hovered || self.drag.as_ref().is_some_and(|d| d.param == k.param);
        if k.small {
            let text = if touched { unsafe { k.param.normalized_value_to_string(v, true) } } else { k.label.to_string() };
            let face = if touched { Face::Mono } else { Face::Label };
            p.text(x, y + r + 12.0, &text, face, if touched { 10.0 } else { 11.0 }, color(if touched { INK } else { SOFT }, 1.0), Align::Center);
        } else {
            p.text(x, y + r + 14.0, k.label, Face::Label, 12.5, color(if selected { INK } else { SOFT }, fade), Align::Center);
            let shown = unsafe { k.param.normalized_value_to_string(v, true) };
            p.text(x, y + r + 29.0, &shown, Face::Mono, 10.5, color(if touched { INK } else { DIM }, fade), Align::Center);
        }
    }

    /// This page's controls as the newest voice has them, or as the knobs set them.
    fn live(&self, k: OscTarget) -> f32 {
        let t = osc_target(self.page, k);
        if self.shown.sounding {
            range_of(t).unnormalize(self.shown.values[t])
        } else {
            self.params.target(t).value()
        }
    }

    /// One turn of the wave this page's oscillator is making, centred and fitted.
    fn paint_wave(&self, p: &mut Painter, hue: u32) {
        let bend = Bend { squish: self.live(OscTarget::Squish), skew: self.live(OscTarget::Skew), tilt: self.live(OscTarget::Tilt) };
        let mut bent = self.bent.borrow_mut();
        self.with_outline(self.page, |outline| bent.bend(outline, bend));
        let depth = self.live(OscTarget::Orbit);
        let orbit = if self.shown.sounding { self.shown.orbit[self.page] } else { 0.0 };
        let (os, oc) = (std::f32::consts::TAU * orbit).sin_cos();
        let center = [bent.centroid[0] + self.live(OscTarget::Center) + depth * oc, bent.centroid[1] + depth * os];
        let scale = 2.0 / bent.reach(center);
        let probe = self.live(OscTarget::Ray);

        let w = WAVE;
        p.rounded(w.x, w.y, w.w, w.h, 4.0, Some(color(HOLLOW, 0.72)), Some((color(FAINT, 0.9), 1.0)));
        let (mid_x, mid_y) = (w.x + w.w / 2.0, w.y + w.h / 2.0);
        p.line(w.x + 6.0, mid_y, w.x + w.w - 6.0, mid_y, color(FAINT, 0.7), 1.0);
        p.line(mid_x, w.y + 6.0, mid_x, w.y + w.h - 6.0, color(FAINT, 0.35), 1.0);
        let points = 240;
        let values: Vec<f32> = (0..=points).map(|i| bent.read(i as f32 / points as f32, center, scale, probe)).collect();
        let (low, high) = values.iter().fold((f32::MAX, f32::MIN), |(lo, hi), v| (lo.min(*v), hi.max(*v)));
        let (middle, half) = (0.5 * (low + high), (0.5 * (high - low)).max(0.02));
        let fit = (w.h / 2.0 - 14.0) / half;
        let inner = w.w - 16.0;
        let alpha = if self.shown.sounding { 1.0 } else { 0.85 };
        p.polyline(values.iter().enumerate().map(|(i, v)| (w.x + 8.0 + inner * i as f32 / points as f32, mid_y - (v - middle) * fit)), color(hue, alpha), 1.6);
    }

    fn paint_osc_page(&self, p: &mut Painter, t: f32) {
        let element = self.osc().element.value();
        let hue = accent(element);
        let r = OSC_PANEL;
        Self::panel_box(p, r);
        self.backdrop(p, Area { x: r.x + 1.0, y: r.y + 1.0, w: r.w - 2.0, h: r.h - 2.0, round: false }, t);
        for i in 0..OSCS {
            let rect = page_rect(i);
            let on = i == self.page;
            let hovered = self.hover == Some(Hit::Page(i));
            let edge = if on { INK } else if hovered { SOFT } else { FAINT };
            p.rounded(rect.x, rect.y, rect.w, rect.h, 4.0, on.then(|| color(INK, 0.08)), Some((color(edge, 1.0), 1.0)));
            let osc_hue = accent(self.params.osc[i].element.value());
            p.text(rect.x + rect.w / 2.0, rect.y + rect.h / 2.0 + 1.0, ROMAN[i], Face::Display, 26.0, color(if on { osc_hue } else { DIM }, 1.0), Align::Center);
        }
        for (i, e) in ALL.iter().enumerate() {
            Self::pill(p, element_rect(i), e.name(), *e == element, self.hover == Some(Hit::Element(*e)), accent(*e));
        }
        self.paint_wave(p, hue);
        Self::pill(p, FREE, "Free", self.osc().free.value(), self.hover == Some(Hit::Free), hue);
    }

    fn paint_filter_and_amp(&self, p: &mut Painter) {
        Self::panel_box(p, FILTER_PANEL);
        Self::panel_box(p, AMP_PANEL);
        p.text(FILTER_PANEL.x + 18.0, FILTER_PANEL.y + 24.0, "Filter", Face::Label, 14.0, color(INK, 1.0), Align::Left);
        let kind = self.params.filter.value();
        for (k, rect) in FILTER_PILLS {
            let label = if k == FilterKind::Svf { "SVF 2" } else { "Ladder 4" };
            Self::pill(p, rect, label, kind == k, self.hover == Some(Hit::Filter(k)), INK);
        }
        p.text(FILTER_PANEL.x + 18.0, 240.0, "Env", Face::Label, 11.0, color(SOFT, 1.0), Align::Left);
        p.text(AMP_PANEL.x + 18.0, AMP_PANEL.y + 24.0, "Amp", Face::Label, 14.0, color(INK, 1.0), Align::Left);
    }

    fn paint_top_bar(&self, p: &mut Painter) {
        p.text(26.0, 34.0, "Raycaster", Face::Label, 26.0, color(INK, 1.0), Align::Left);
        let mono = self.params.mono.value();
        for (m, rect) in MONO_PILLS {
            Self::pill(p, rect, if m { "Mono" } else { "Poly" }, mono == m, self.hover == Some(Hit::Mono(m)), INK);
        }
    }

    fn paint_modulator(&self, p: &mut Painter) {
        let r = MOD_PANEL;
        Self::panel_box(p, r);
        let t = self.selected;
        let mods = &self.params.mods[t];
        let osc_owned = t < OSCS * OSC_TARGETS.len();
        let hue = if osc_owned { accent(self.params.osc[t / OSC_TARGETS.len()].element.value()) } else { INK };
        p.text(r.x + 20.0, r.y + 22.0, "Modulating", Face::Label, 11.0, color(DIM, 1.0), Align::Left);
        let name = if osc_owned { target_name(t).replacen(' ', "  ·  ", 1) } else { target_name(t) };
        p.text(r.x + 20.0, r.y + 56.0, &name, Face::Display, 30.0, color(hue, 1.0), Align::Left);
        let (lfo, env) = (mods.lfo.value(), mods.env.value());
        p.text(r.x + 20.0, r.y + 94.0, &format!("LFO {:+.0} %", lfo * 100.0), Face::Mono, 11.0, color(if lfo.abs() > 1e-4 { LFO } else { DIM }, 1.0), Align::Left);
        p.text(r.x + 20.0, r.y + 112.0, &format!("ENV {:+.0} %", env * 100.0), Face::Mono, 11.0, color(if env.abs() > 1e-4 { ENV } else { DIM }, 1.0), Align::Left);
        let (wheel, touch) = (mods.wheel.value(), mods.touch.value());
        p.text(r.x + 20.0, r.y + 130.0, &format!("WHL {:+.0} %", wheel * 100.0), Face::Mono, 11.0, color(if wheel.abs() > 1e-4 { HAND } else { DIM }, 1.0), Align::Left);
        p.text(r.x + 20.0, r.y + 148.0, &format!("AFT {:+.0} %", touch * 100.0), Face::Mono, 11.0, color(if touch.abs() > 1e-4 { HAND } else { DIM }, 1.0), Align::Left);
        p.text(380.0, HAND_Y - 8.0, "Mod wheel", Face::Label, 10.5, color(HAND, 0.9), Align::Left);
        p.text(380.0, HAND_Y + 8.0, "Aftertouch", Face::Label, 10.5, color(HAND, 0.9), Align::Left);

        // LFO.
        p.text(230.0, 504.0, "LFO", Face::Label, 12.0, color(LFO, 1.0), Align::Left);
        let shape = mods.shape.value();
        for (i, s) in SHAPES.iter().enumerate() {
            let rect = shape_rect(i);
            let on = *s == shape;
            let hovered = self.hover == Some(Hit::Shape(*s));
            let edge = if on { LFO } else if hovered { SOFT } else { FAINT };
            p.rounded(rect.x + 0.5, rect.y + 0.5, rect.w - 1.0, rect.h - 1.0, 4.0, on.then(|| color(LFO, 0.16)), Some((color(edge, 1.0), 1.0)));
            let wave = sketch(*s, 24);
            p.polyline(wave.iter().enumerate().map(|(k, v)| (rect.x + 6.0 + (rect.w - 12.0) * k as f32 / 23.0, rect.y + rect.h / 2.0 - v * 8.0)), color(if on { INK } else { SOFT }, 1.0), 1.2);
        }
        let s = LFO_SCOPE;
        p.rounded(s.x, s.y, s.w, s.h, 3.0, Some(color(HOLLOW, 0.6)), Some((color(FAINT, 0.7), 1.0)));
        let mid = s.y + s.h / 2.0;
        p.line(s.x, mid, s.x + s.w, mid, color(FAINT, 0.6), 1.0);
        let wave = sketch(shape, 120);
        let direction = if lfo < 0.0 { -1.0 } else { 1.0 };
        let amp = lfo.abs().max(0.08) * (s.h / 2.0 - 6.0) * direction;
        let points: Vec<(f32, f32)> = (0..=240).map(|i| (s.x + s.w * i as f32 / 240.0, mid - wave[i % 120] * amp)).collect();
        p.polyline(points, color(LFO, if lfo.abs() > 1e-4 { 1.0 } else { 0.35 }), 1.5);
        Self::pill(p, SYNC, "Sync", mods.sync.value(), self.hover == Some(Hit::Sync), LFO);
        Self::pill(p, RETRIG, "Retrig", mods.retrig.value(), self.hover == Some(Hit::Retrig), LFO);

        // Envelope.
        p.text(706.0, 504.0, "Envelope", Face::Label, 12.0, color(ENV, 1.0), Align::Left);
        let a = AD_SCOPE;
        p.rounded(a.x, a.y, a.w, a.h, 3.0, Some(color(HOLLOW, 0.6)), Some((color(FAINT, 0.7), 1.0)));
        let (attack, decay) = (mods.attack.value(), mods.decay.value());
        let (start, end) = (a.x + 4.0, a.x + a.w - 4.0);
        let peak = start + (end - start) * (attack / (attack + decay)).clamp(0.02, 0.9);
        let base = a.y + a.h - 5.0;
        let height = (a.h - 10.0) * env.abs().max(0.15);
        let (rise, fall) = (mods.attack_curve.value(), mods.decay_curve.value());
        let mut pts: Vec<(f32, f32)> = (0..=20).map(|i| i as f32 / 20.0).map(|f| (start + (peak - start) * f, base - height * along(0.0, 1.0, f, rise))).collect();
        pts.extend((1..=40).map(|i| i as f32 / 40.0).map(|f| (peak + (end - peak) * f, base - height * along(1.0, 0.0, f, fall))));
        p.polyline(pts, color(ENV, if env.abs() > 1e-4 { 1.0 } else { 0.35 }), 1.5);
    }

    fn paint(&self, p: &mut Painter, t: f32) {
        let mut ground = Path::new();
        ground.rect(0.0, 0.0, WIDTH, HEIGHT);
        p.canvas.fill_path(&ground, &Paint::color(color(GROUND, 1.0)));
        self.paint_top_bar(p);
        self.paint_osc_page(p, t);
        self.paint_filter_and_amp(p);
        self.paint_modulator(p);
        for (i, k) in self.knobs().iter().enumerate() {
            self.paint_knob(p, k, self.hover == Some(Hit::Knob(i)), t);
        }
        let held = self.drag.as_ref().and_then(|d| d.curve);
        for (i, c) in self.curves().iter().enumerate() {
            Self::paint_curve_button(p, c, self.hover == Some(Hit::Curve(i)) || held == Some(i));
        }
        if let Some(i) = held {
            Self::paint_curve_popup(p, &self.curves()[i]);
        }
    }

    fn bow_of(c: &CurveButton) -> f32 {
        unsafe { c.param.preview_plain(Self::value_of(c.param)) }
    }

    /// `points` along a segment's curve, in a box, running the segment's way.
    fn curve_points(c: &CurveButton, x: f32, y: f32, w: f32, h: f32, points: usize) -> Vec<(f32, f32)> {
        let bow = Self::bow_of(c);
        let (from, to) = if c.rising { (0.0, 1.0) } else { (1.0, 0.0) };
        (0..=points).map(|i| i as f32 / points as f32).map(|f| (x + w * f, y + h - h * along(from, to, f, bow))).collect()
    }

    fn paint_curve_button(p: &mut Painter, c: &CurveButton, lit: bool) {
        let edge = if lit { color(c.hue, 1.0) } else { color(FAINT, 1.0) };
        p.rounded(c.x, c.y, CURVE_SIZE, CURVE_SIZE, 2.5, Some(color(HOLLOW, 0.85)), Some((edge, 1.0)));
        let inset = 3.0;
        let side = CURVE_SIZE - 2.0 * inset;
        let ink = color(if lit { INK } else { SOFT }, 1.0);
        p.polyline(Self::curve_points(c, c.x + inset, c.y + inset, side, side, 10), ink, 1.3);
    }

    /// The held curve, large, beside its button.
    fn paint_curve_popup(p: &mut Painter, c: &CurveButton) {
        let x = (c.x + CURVE_SIZE - POPUP_W).clamp(8.0, WIDTH - POPUP_W - 8.0);
        let above = c.y - POPUP_H - 6.0;
        let y = if above > 8.0 { above } else { c.y + CURVE_SIZE + 6.0 };
        p.rounded(x, y, POPUP_W, POPUP_H, 6.0, Some(color(PANEL, 0.97)), Some((color(c.hue, 0.9), 1.0)));
        p.text(x + 12.0, y + 16.0, c.name, Face::Label, 10.5, color(SOFT, 1.0), Align::Left);
        let text = unsafe { c.param.normalized_value_to_string(Self::value_of(c.param), true) };
        p.text(x + POPUP_W - 12.0, y + 16.0, &text, Face::Mono, 10.0, color(INK, 1.0), Align::Right);
        let (gx, gy, gw, gh) = (x + 12.0, y + 30.0, POPUP_W - 24.0, POPUP_H - 42.0);
        p.rounded(gx - 4.0, gy - 4.0, gw + 8.0, gh + 8.0, 3.0, Some(color(HOLLOW, 0.8)), None);
        let (y0, y1) = if c.rising { (gy + gh, gy) } else { (gy, gy + gh) };
        p.line(gx, y0, gx + gw, y1, color(FAINT, 0.9), 1.0);
        p.polyline(Self::curve_points(c, gx, gy, gw, gh, 60), color(c.hue, 1.0), 2.0);
    }
}

impl View for Panel {
    fn element(&self) -> Option<&'static str> {
        Some("raycaster-panel")
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
                _ => {}
            }
        });
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        let (scale, ox, oy) = Self::transform(cx.bounds());
        let t = self.opened.elapsed().as_secs_f32();
        canvas.save();
        canvas.translate(ox, oy);
        canvas.scale(scale, scale);
        let fonts = fonts(canvas);
        let mut p = Painter { canvas, fonts };
        self.paint(&mut p, t);
        canvas.restore();
    }
}
