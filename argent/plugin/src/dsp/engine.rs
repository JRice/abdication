//! Argent's sound: four latched notes, each the note and up to three harmonics, every one
//! of them an oscillator reading the SEED's bank; summed into a dark body with a little
//! of the raw top left over it, all of it moved by one set of slow and fast motions.

use std::sync::Arc;

use super::filters::Svf;
use super::limiter::Limiter;
use super::modulation::{Bloom, Flare, Tremor, Wander};
use super::notes::{Notes, CUT_S, NOTES};
use super::osc::{Osc, Reading};
use super::random::Rng;
use super::tables::{Bank, CYCLE, TABLES};

/// The note and its octave, octave and fifth, and two octaves.
pub const VOICES: usize = 4;
const HARMONIC: [f32; VOICES] = [1.0, 2.0, 3.0, 4.0];
/// Each oscillator's level in the sum, before Volume.
const VOICE_GAIN: f32 = 0.75;
/// How long the old bank fades under the new one when SEED changes.
const BANK_FADE_S: f32 = 0.15;
/// The body: a 4-pole lowpass, resonant in its first half.
const BODY_Q: [f32; 2] = [1.4, 0.6];
/// Fizz is what lies above this, taken from the oscillators unfiltered.
const FIZZ_HZ: f32 = 2_000.0;
const FIZZ_Q: f32 = 0.707;
/// How far blooms move the tone and the wave at full Bloom.
const BLOOM_TONE_OCT: f32 = 1.6;
const BLOOM_WAVE: f32 = 0.3;
/// How far the tide moves SCAN (in tables) and MORPH at full Tide, and how often.
const TIDE_TABLES: f32 = 3.0;
const TIDE_WAVE: f32 = 0.25;
const TIDE_SCAN_HZ: f32 = 1.0 / 120.0;
const TIDE_WAVE_HZ: f32 = 1.0 / 97.0;
const WANDER_HZ: f32 = 0.05;
/// What SEED spreads between voices.
const WAVE_SPREAD: f32 = 0.25;
const TABLE_SPREAD: f32 = 0.5;
const NOTE_CENTS: f32 = 4.0;
const HARMONIC_CENTS: f32 = 7.0;
const PAN_SPREAD: f32 = 0.7;
const MAX_TONE_HZ: f32 = 18_000.0;
/// dB the body drops for each octave the motion opens it, so a flare brightens without
/// getting louder, as the recording's do.
const OPENING_COMPENSATION_DB: f32 = 2.5;

/// Everything the knobs set, once a sample.
#[derive(Clone, Copy, Debug)]
pub struct Controls {
    /// Position inside a table, 0..1.
    pub morph: f32,
    /// Position in the bank, 0..TABLES-1.
    pub scan: f32,
    /// Harmonic voices, 0..1: the three come in one after another.
    pub span: f32,
    pub tone_hz: f32,
    /// Linear gain of the fizz layer; 0 is off.
    pub fizz: f32,
    pub tide: f32,
    pub flare_s: f32,
    pub depth_oct: f32,
    pub bloom: f32,
    pub tremor: f32,
    pub wander_cents: f32,
    pub fade_s: f32,
    pub curve: f32,
    pub reading: Reading,
    pub volume: f32,
}

impl Default for Controls {
    fn default() -> Self {
        Self {
            morph: 0.3,
            scan: 0.0,
            span: 0.62,
            tone_hz: 520.0,
            fizz: 10f32.powf(-30.0 / 20.0),
            tide: 0.4,
            flare_s: 9.0,
            depth_oct: 3.0,
            bloom: 0.5,
            tremor: 0.13,
            wander_cents: 15.0,
            fade_s: 20.0,
            curve: 2.0,
            reading: Reading { stepped: true, coarse: false },
            volume: 0.5,
        }
    }
}

/// How much of harmonic voice `k` SPAN lets in.
fn span_level(span: f32, k: usize) -> f32 {
    if k == 0 {
        1.0
    } else {
        ((span - (k - 1) as f32 / 3.0) * 3.0).clamp(0.0, 1.0)
    }
}

/// Folds any value into 0..1 and back, so a wave position ping-pongs inside its table.
fn reflect(x: f32) -> f32 {
    1.0 - ((x.rem_euclid(2.0)) - 1.0).abs()
}

fn midi_hz(key: u8) -> f32 {
    440.0 * 2f32.powf((key as f32 - 69.0) / 12.0)
}

/// What SEED gives one voice of one slot.
#[derive(Clone, Copy, Default)]
struct Offsets {
    wave: f32,
    table: f32,
    ratio: f32,
    pan: [f32; 2],
}

fn offsets(seed: u32) -> [[Offsets; VOICES]; NOTES] {
    std::array::from_fn(|slot| {
        std::array::from_fn(|k| {
            let mut rng = Rng::new(seed.wrapping_mul(1009).wrapping_add((slot * VOICES + k) as u32 * 7_877 + 3));
            let cents = rng.gaussian() * if k == 0 { NOTE_CENTS } else { HARMONIC_CENTS };
            let angle = (1.0 + PAN_SPREAD * rng.signed()) * std::f32::consts::FRAC_PI_4;
            Offsets {
                wave: WAVE_SPREAD * rng.signed(),
                table: TABLE_SPREAD * rng.signed(),
                ratio: 2f32.powf(cents / 1200.0),
                pan: [angle.cos(), angle.sin()],
            }
        })
    })
}

/// Where the panel's trace is reading, and what.
#[derive(Clone, Copy, Debug, Default)]
pub struct Shown {
    pub table: f32,
    pub wave: f32,
    pub frame: usize,
    pub frames: usize,
    pub sounding: bool,
}

pub struct Engine {
    sample_rate: f32,
    bank: Arc<Bank>,
    /// The bank being faded out after a SEED change, and how much of it is left.
    previous: Option<(Arc<Bank>, f32)>,
    retired: Option<Arc<Bank>>,
    notes: Notes,
    oscs: [[Osc; VOICES]; NOTES],
    offsets: [[Offsets; VOICES]; NOTES],
    wanders: [[Wander; VOICES]; NOTES],
    tide_scan: Wander,
    tide_wave: Wander,
    flare: Flare,
    bloom: Bloom,
    tremor: Tremor,
    body: [[Svf; 2]; 2],
    fizz: [[Svf; 2]; 2],
    limiter: Limiter,
    start_phase: Rng,
    shown: Shown,
}

impl Engine {
    pub fn new(sample_rate: f32, bank: Arc<Bank>) -> Self {
        let mut fizz = [[Svf::default(); 2]; 2];
        fizz.iter_mut().flatten().for_each(|f| f.set(FIZZ_HZ, FIZZ_Q, sample_rate));
        Self {
            sample_rate,
            offsets: offsets(bank.seed),
            bank,
            previous: None,
            retired: None,
            notes: Notes::default(),
            oscs: [[Osc::default(); VOICES]; NOTES],
            wanders: std::array::from_fn(|i| std::array::from_fn(|k| Wander::new(9_001 + (i * VOICES + k) as u32 * 31))),
            tide_scan: Wander::new(41),
            tide_wave: Wander::new(43),
            flare: Flare::new(47),
            bloom: Bloom::new(53),
            tremor: Tremor::default(),
            body: [[Svf::default(); 2]; 2],
            fizz,
            limiter: Limiter::new(sample_rate),
            start_phase: Rng::new(59),
            shown: Shown::default(),
        }
    }

    pub fn seed(&self) -> u32 {
        self.bank.seed
    }

    /// Whether a new bank can come in now (not while the last change is still fading).
    pub fn ready_for_bank(&self) -> bool {
        self.previous.is_none() && self.retired.is_none()
    }

    /// A new SEED's bank: the old one fades out under it.
    pub fn take_bank(&mut self, bank: Arc<Bank>) {
        self.offsets = offsets(bank.seed);
        let old = std::mem::replace(&mut self.bank, bank);
        self.previous = Some((old, 1.0));
    }

    /// A bank finished with, for the caller to drop somewhere other than here.
    pub fn take_retired(&mut self) -> Option<Arc<Bank>> {
        self.retired.take()
    }

    pub fn keep_retired(&mut self, bank: Arc<Bank>) {
        self.retired = Some(bank);
    }

    pub fn press(&mut self, key: u8) {
        if let Some(slot) = self.notes.press(key) {
            self.restart(slot);
        }
    }

    pub fn lift(&mut self, key: u8) {
        self.notes.lift(key);
    }

    pub fn release(&mut self) {
        self.notes.release();
    }

    fn restart(&mut self, slot: usize) {
        for osc in &mut self.oscs[slot] {
            let phase = self.start_phase.unit();
            osc.restart(phase, 0.0, Reading { stepped: true, coarse: false });
        }
    }

    pub fn shown(&self) -> Shown {
        self.shown
    }

    /// One cycle of the frame the panel shows, decimated into `out`.
    pub fn trace(&self, out: &mut [f32]) {
        let table = (self.shown.table.round() as usize).min(TABLES - 1);
        let cycle = self.bank.cycle(table, self.shown.frame);
        let n = out.len();
        for (i, o) in out.iter_mut().enumerate() {
            *o = cycle[i * CYCLE / n];
        }
    }

    pub fn reduction_db(&self) -> f32 {
        self.limiter.reduction_db()
    }

    #[inline]
    pub fn process(&mut self, c: &Controls) -> [f32; 2] {
        let sr = self.sample_rate;
        let tide_scan = c.tide * TIDE_TABLES * self.tide_scan.next(TIDE_SCAN_HZ / sr);
        let tide_wave = c.tide * TIDE_WAVE * self.tide_wave.next(TIDE_WAVE_HZ / sr);
        let flare = self.flare.next(c.flare_s, sr);
        let bloom = self.bloom.next(sr) * flare;
        let wave_base = c.morph + tide_wave + BLOOM_WAVE * c.bloom * bloom;
        let table_base = c.scan + tide_scan;
        let last_table = (TABLES - 1) as f32;

        let started = self.notes.step(1.0 / (c.fade_s.max(0.01) * sr), 1.0 / (CUT_S * sr));
        for (slot, began) in started.iter().enumerate() {
            if *began {
                self.restart(slot);
            }
        }

        let fading = self.previous.as_ref().map(|(bank, left)| (bank, *left));
        let mut sum = [0.0f32; 2];
        for slot in 0..NOTES {
            let note = *self.notes.get(slot);
            if !note.sounding {
                continue;
            }
            let base_hz = midi_hz(note.key);
            for k in 0..VOICES {
                let off = self.offsets[slot][k];
                let level = self.notes.level(slot, c.curve, k) * span_level(c.span, k);
                let cents = c.wander_cents * self.wanders[slot][k].next(WANDER_HZ / sr);
                let hz = base_hz * HARMONIC[k] * off.ratio * 2f32.powf(cents / 1200.0);
                let table = (table_base + off.table).clamp(0.0, last_table);
                let wave = reflect(wave_base + off.wave);
                let osc = &mut self.oscs[slot][k];
                if level > 1e-6 {
                    let mut v = osc.read(&self.bank, table, wave, c.reading);
                    if let Some((old, left)) = &fading {
                        v += (osc.read(old, table, wave, c.reading) - v) * left;
                    }
                    let v = v * level * VOICE_GAIN;
                    sum[0] += v * off.pan[0];
                    sum[1] += v * off.pan[1];
                }
                osc.advance(hz / sr, wave, c.reading);
            }
        }
        self.fade_banks();
        self.track_shown(table_base, wave_base, c.reading);

        let opening = c.depth_oct * flare + BLOOM_TONE_OCT * c.bloom * bloom;
        let cutoff = (c.tone_hz * 2f32.powf(opening)).min(MAX_TONE_HZ);
        // Flares change the colour, not the loudness: opening the body is paid for in level.
        let body_gain = 10f32.powf(-OPENING_COMPENSATION_DB * opening / 20.0);
        let gain = self.tremor.next(c.tremor, sr) * c.volume;
        let mut out = [0.0f32; 2];
        for ch in 0..2 {
            let x = sum[ch];
            let mut body = x;
            for (stage, q) in self.body[ch].iter_mut().zip(BODY_Q) {
                stage.set(cutoff, q, sr);
                body = stage.process(body).low;
            }
            let mut low = x;
            for stage in &mut self.fizz[ch] {
                low = stage.process(low).low;
            }
            out[ch] = (body * body_gain + c.fizz * (x - low)) * gain;
        }
        self.limiter.process(out)
    }

    fn fade_banks(&mut self) {
        if let Some((_, left)) = &mut self.previous {
            *left -= 1.0 / (BANK_FADE_S * self.sample_rate);
            if *left <= 0.0 {
                self.retired = self.previous.take().map(|(bank, _)| bank);
            }
        }
    }

    fn track_shown(&mut self, table_base: f32, wave_base: f32, reading: Reading) {
        let last_table = (TABLES - 1) as f32;
        self.shown = match self.notes.newest() {
            Some(slot) => {
                let off = self.offsets[slot][0];
                Shown {
                    table: (table_base + off.table).clamp(0.0, last_table),
                    wave: reflect(wave_base + off.wave),
                    frame: self.oscs[slot][0].frame(),
                    frames: reading.frame_count(),
                    sounding: true,
                }
            }
            None => {
                let wave = reflect(wave_base);
                Shown {
                    table: table_base.clamp(0.0, last_table),
                    wave,
                    frame: reading.nearest(wave),
                    frames: reading.frame_count(),
                    sounding: false,
                }
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SR: f32 = 48_000.0;

    fn engine() -> Engine {
        Engine::new(SR, Bank::build(417))
    }

    fn run(engine: &mut Engine, c: &Controls, seconds: f32) -> Vec<[f32; 2]> {
        (0..(seconds * SR) as usize).map(|_| engine.process(c)).collect()
    }

    fn rms(samples: &[[f32; 2]]) -> f32 {
        (samples.iter().map(|[l, r]| l * l + r * r).sum::<f32>() / (2 * samples.len()) as f32).sqrt()
    }

    #[test]
    fn silent_with_no_notes() {
        let mut e = engine();
        assert!(run(&mut e, &Controls::default(), 0.5).iter().all(|[l, r]| *l == 0.0 && *r == 0.0));
    }

    #[test]
    fn a_chord_fades_in_stays_and_fades_out_on_release() {
        let mut e = engine();
        let c = Controls { fade_s: 1.0, ..Controls::default() };
        for key in [33, 38, 45, 52] {
            e.press(key);
            e.lift(key);
        }
        let rising = run(&mut e, &c, 0.25);
        let full = run(&mut e, &c, 3.0);
        assert!(rms(&rising) < 0.5 * rms(&full[(SR as usize)..]), "{} {}", rms(&rising), rms(&full));
        assert!(full.iter().all(|[l, r]| l.is_finite() && r.is_finite() && l.abs() < 1.0 && r.abs() < 1.0));
        e.release();
        run(&mut e, &c, 1.2);
        let after = run(&mut e, &c, 0.5);
        assert!(rms(&after) < 1e-4, "{}", rms(&after));
    }

    #[test]
    fn a_new_seed_fades_in_without_a_jump() {
        let mut e = engine();
        let c = Controls { fade_s: 0.1, ..Controls::default() };
        e.press(45);
        let before = run(&mut e, &c, 1.0);
        e.take_bank(Bank::build(9));
        let during = run(&mut e, &c, 0.3);
        assert!(e.take_retired().is_some());
        let biggest_step = during.windows(2).map(|w| (w[1][0] - w[0][0]).abs()).fold(0.0, f32::max);
        let usual_step = before.windows(2).map(|w| (w[1][0] - w[0][0]).abs()).fold(0.0, f32::max);
        assert!(biggest_step < 3.0 * usual_step + 1e-3, "{biggest_step} vs {usual_step}");
    }

    #[test]
    fn span_brings_harmonics_in_one_after_another() {
        let near = |a: f32, b: f32| (a - b).abs() < 1e-5;
        assert!(near(span_level(0.0, 1), 0.0));
        assert!(near(span_level(1.0 / 3.0, 1), 1.0));
        assert!(near(span_level(1.0 / 3.0, 2), 0.0));
        assert!(near(span_level(0.5, 2), 0.5));
        assert!(near(span_level(1.0, 3), 1.0));
        assert!(near(span_level(0.0, 0), 1.0));
    }

    #[test]
    fn reflect_ping_pongs() {
        assert_eq!(reflect(0.25), 0.25);
        assert!((reflect(1.25) - 0.75).abs() < 1e-6);
        assert!((reflect(-0.25) - 0.25).abs() < 1e-6);
    }
}
