//! The digital half: runs a program once per tick against the memory bus, in 16-bit
//! integers, rounding down as it goes. The dry signal goes through here too.

use super::program::{all_programs, Coef, Op, Program, PARK, PREDELAY_WORDS, REGISTERS};
use super::ram::{Bus, SWITCHES, WORDS};

const WRAP: u16 = (WORDS - 1) as u16;
const ACC_LIMIT: i32 = 1 << 17;
/// Rows that miss their refresh per second, flat out at the stock clock.
const ROT_ROWS_PER_SECOND: f32 = 60.0;
const STOCK_HZ: f32 = 31_250.0;

#[derive(Clone, Copy)]
pub struct Knobs {
    pub program: usize,
    pub decay: f32,
    pub damp: f32,
    /// 0..1 of the predelay line.
    pub predelay: f32,
    /// 0 dry, 1 wet, both through the converters.
    pub mix: f32,
    pub matrix: [bool; SWITCHES],
    pub contention: f32,
    pub loop_ram: bool,
    pub rot: f32,
}

impl Default for Knobs {
    fn default() -> Self {
        Self { program: 0, decay: 0.6, damp: 0.4, predelay: 0.1, mix: 0.4, matrix: [false; SWITCHES], contention: 0.0, loop_ram: false, rot: 0.0 }
    }
}

#[inline(always)]
fn scale(value: i32, coef: i32) -> i32 {
    ((value as i64 * coef as i64) >> 15) as i32
}

#[inline(always)]
fn word(value: i32) -> i16 {
    value.clamp(i16::MIN as i32, i16::MAX as i32) as i16
}

pub struct Machine {
    bus: Bus,
    programs: Vec<Program>,
    current: usize,
    /// The ring's moving pointer.
    base: u16,
    /// Counts ticks for the backwards windows.
    ticks: u32,
    registers: [i32; REGISTERS],
    park: i32,
    predelay: u16,
    wet_gain: i32,
    dry_gain: i32,
    rot_per_tick: f32,
    rot: f32,
}

impl Machine {
    pub fn new() -> Self {
        Self {
            bus: Bus::new(),
            programs: all_programs(),
            current: 0,
            base: 0,
            ticks: 0,
            registers: [0; REGISTERS],
            park: 0,
            predelay: 0,
            wet_gain: 0,
            dry_gain: 0,
            rot_per_tick: 0.0,
            rot: 0.0,
        }
    }

    /// Once per block. Changing program keeps the memory: the new program plays through
    /// the old one's leftovers, as the real thing does.
    pub fn set_knobs(&mut self, knobs: &Knobs) {
        if knobs.program != self.current && knobs.program < self.programs.len() {
            self.current = knobs.program;
            self.registers = [0; REGISTERS];
        }
        self.programs[self.current].set_knobs(knobs.decay, knobs.damp);
        self.predelay = (knobs.predelay * PREDELAY_WORDS as f32) as u16;
        self.wet_gain = (knobs.mix.sqrt() * 32_767.0) as i32;
        self.dry_gain = ((1.0 - knobs.mix).sqrt() * 32_767.0) as i32;
        self.bus.set_matrix(&knobs.matrix);
        self.bus.set_contention(knobs.contention);
        self.bus.set_write_enable(!knobs.loop_ram);
        self.rot = knobs.rot;
    }

    /// Rot gets worse the slower the clock: refresh comes from the same clock.
    pub fn set_clock(&mut self, hz: f32) {
        let rows = self.rot * self.rot * ROT_ROWS_PER_SECOND * (STOCK_HZ / hz).sqrt();
        self.rot_per_tick = rows / hz;
    }

    #[inline(always)]
    fn at(&self, offset: u16) -> u16 {
        self.base.wrapping_add(offset) & WRAP
    }

    /// One clock tick: takes the converted input, gives back what goes to the DAC.
    pub fn tick(&mut self, input: [i16; 2]) -> [i16; 2] {
        self.base = self.base.wrapping_sub(1) & WRAP;
        self.ticks = self.ticks.wrapping_add(1);
        if self.rot_per_tick > 0.0 && self.bus.rng().chance(self.rot_per_tick) {
            self.bus.rot_row();
        }

        let mono = (input[0] as i32 + input[1] as i32) >> 1;
        let mut acc = 0i32;
        let mut wet = [0i32; 2];
        // Taken out for the tick, so the ops can read it while the rest of us changes.
        let program = std::mem::take(&mut self.programs[self.current]);
        let coef = |c: Coef| program.coefs[c.index()];
        for op in &program.ops {
            match *op {
                Op::Clear => acc = 0,
                Op::InputMono(gain) => acc += scale(mono, coef(gain)),
                Op::Load(r) => acc = self.register(r),
                Op::Store(r) => self.set_register(r, acc),
                Op::Mix(r, gain) => acc += scale(self.register(r), coef(gain)),
                Op::Gain(gain) => acc = scale(acc, coef(gain)),
                Op::Read(from, gain) => {
                    let value = self.bus.read(self.at(from)) as i32;
                    acc += scale(value, coef(gain));
                }
                Op::Write(to) => {
                    let address = self.at(to);
                    self.bus.write(address, word(acc));
                }
                Op::ReadPredelay(from, gain) => {
                    let value = self.bus.read(self.at(from + self.predelay)) as i32;
                    acc += scale(value, coef(gain));
                }
                Op::Allpass { to, length, gain } => {
                    let g = coef(gain);
                    let tail = self.bus.read(self.at(to + length)) as i32;
                    let head = acc + scale(tail, g);
                    let address = self.at(to);
                    self.bus.write(address, word(head));
                    acc = tail - scale(head, g);
                }
                Op::Lowpass(r, gain) => {
                    let g = coef(gain);
                    let state = self.register(r);
                    acc = scale(acc, 32_767 - g) + scale(state, g);
                    self.set_register(r, acc);
                }
                Op::Output(channel, gain) => wet[channel as usize] += scale(acc, coef(gain)),
                Op::Reverse { to, length, channel, gain } => {
                    let grain = |phase: u32| {
                        let position = phase % length as u32;
                        let half = length as u32 / 2;
                        let window = if position < half { position } else { length as u32 - position };
                        ((position * 2) % length as u32, (window << 15) as i64 / half as i64)
                    };
                    let g = coef(gain) as i64;
                    let mut sum = 0i64;
                    for phase in [self.ticks, self.ticks.wrapping_add(length as u32 / 2)] {
                        let (reach, window) = grain(phase);
                        let value = self.bus.read(self.at(to + reach as u16)) as i64;
                        sum += (value * window) >> 15;
                    }
                    wet[channel as usize] += ((sum * g) >> 15) as i32;
                }
            }
            acc = acc.clamp(-ACC_LIMIT, ACC_LIMIT);
        }
        self.programs[self.current] = program;
        std::array::from_fn(|c| word(scale(input[c] as i32, self.dry_gain) + scale(wet[c], self.wet_gain)))
    }

    #[inline(always)]
    fn register(&self, r: u8) -> i32 {
        if r == PARK { self.park } else { self.registers[r as usize] }
    }

    #[inline(always)]
    fn set_register(&mut self, r: u8, value: i32) {
        if r == PARK {
            self.park = value;
        } else {
            self.registers[r as usize] = value;
        }
    }
}

impl Default for Machine {
    fn default() -> Self {
        Self::new()
    }
}
