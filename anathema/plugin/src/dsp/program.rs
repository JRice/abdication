//! The programs the DSP runs once per clock tick. Everything lives in one ring of
//! memory, each line at a fixed offset from a pointer that moves one word per tick,
//! which is why a stuck address line makes one line read another's audio.
//!
//! Yamaha's firmware isn't available, so these are our own, in the same style.

use super::ram::WORDS;

pub const REGISTERS: usize = 4;
/// The register that parks the input between lines.
pub const PARK: u8 = 0xFE;
/// The most the Pre Delay knob can reach, in ticks.
pub const PREDELAY_WORDS: u16 = 1_600;

#[derive(Clone, Copy)]
pub struct Coef(u8);

impl Coef {
    #[inline(always)]
    pub fn index(self) -> usize {
        self.0 as usize
    }
}

#[derive(Clone, Copy)]
pub enum Op {
    Clear,
    /// acc += average of the two inputs * gain
    InputMono(Coef),
    Load(u8),
    Store(u8),
    Mix(u8, Coef),
    Gain(Coef),
    /// acc += memory[from ticks ago] * gain
    Read(u16, Coef),
    /// memory[to] = acc
    Write(u16),
    /// acc += memory[from + the Pre Delay knob] * gain
    ReadPredelay(u16, Coef),
    Allpass { to: u16, length: u16, gain: Coef },
    /// One-pole lowpass in a register. The coefficient is how much of the old value stays.
    Lowpass(u8, Coef),
    Output(u8, Coef),
    /// Two overlapping windows reading backwards through a line.
    Reverse { to: u16, length: u16, channel: u8, gain: Coef },
}

/// How a coefficient follows the knobs.
#[derive(Clone, Copy)]
enum Follows {
    Fixed,
    /// The Decay knob, scaled.
    Decay(f32),
    /// The Damp knob.
    Damp,
}

#[derive(Default)]
pub struct Program {
    pub name: &'static str,
    pub ops: Vec<Op>,
    pub coefs: Vec<i32>,
    follows: Vec<(f32, Follows)>,
}

fn q15(value: f32) -> i32 {
    (value * 32_768.0) as i32
}

impl Program {
    /// `decay` and `damp` are 0..1.
    pub fn set_knobs(&mut self, decay: f32, damp: f32) {
        for (coef, (fixed, follows)) in self.coefs.iter_mut().zip(&self.follows) {
            *coef = q15(match follows {
                Follows::Fixed => *fixed,
                Follows::Decay(scale) => decay.sqrt() * 0.97 * scale,
                Follows::Damp => damp * 0.92,
            });
        }
    }
}

/// Lays out lines in the ring and collects ops.
struct Builder {
    name: &'static str,
    ops: Vec<Op>,
    follows: Vec<(f32, Follows)>,
    next_free: usize,
}

impl Builder {
    fn new(name: &'static str) -> Self {
        Self { name, ops: Vec::new(), follows: Vec::new(), next_free: 0 }
    }

    fn fixed(&mut self, value: f32) -> Coef {
        self.coefficient(value, Follows::Fixed)
    }

    fn decay(&mut self, scale: f32) -> Coef {
        self.coefficient(0.0, Follows::Decay(scale))
    }

    fn damp(&mut self) -> Coef {
        self.coefficient(0.0, Follows::Damp)
    }

    fn coefficient(&mut self, value: f32, follows: Follows) -> Coef {
        self.follows.push((value, follows));
        Coef((self.follows.len() - 1) as u8)
    }

    /// Room in the ring for a line this long, plus the tick its tail is read on.
    fn alloc(&mut self, length: usize) -> u16 {
        let at = self.next_free;
        self.next_free += length + 1;
        assert!(self.next_free <= WORDS, "{} runs out of memory at {}", self.name, self.next_free);
        at as u16
    }

    fn op(&mut self, op: Op) {
        self.ops.push(op);
    }

    fn allpass(&mut self, length: usize, gain: f32) {
        let (to, gain) = (self.alloc(length), self.fixed(gain));
        self.op(Op::Allpass { to, length: length as u16, gain });
    }

    /// A plain delay: acc goes in, the tail comes back out.
    fn delay(&mut self, length: usize) -> u16 {
        let to = self.alloc(length);
        self.op(Op::Write(to));
        let unity = self.fixed(0.999);
        self.op(Op::Clear);
        self.op(Op::Read(to + length as u16, unity));
        to
    }

    /// Input through the predelay into acc.
    fn predelayed_input(&mut self, gain: f32) {
        let line = self.alloc(PREDELAY_WORDS as usize);
        let (gain, unity) = (self.fixed(gain), self.fixed(0.999));
        self.op(Op::Clear);
        self.op(Op::InputMono(gain));
        self.op(Op::Write(line));
        self.op(Op::Clear);
        self.op(Op::ReadPredelay(line, unity));
    }

    fn tap(&mut self, from: u16, channel: u8, gain: f32) {
        let gain = self.fixed(gain);
        let unity = self.fixed(1.0);
        self.op(Op::Clear);
        self.op(Op::Read(from, gain));
        self.op(Op::Output(channel, unity));
    }

    fn finish(self) -> Program {
        let count = self.follows.len();
        assert!(count <= u8::MAX as usize, "{} has too many coefficients", self.name);
        let mut program = Program { name: self.name, ops: self.ops, coefs: vec![0; count], follows: self.follows };
        program.set_knobs(0.5, 0.5);
        program
    }
}

/// Two allpass-and-delay arms feeding each other: the shape every reverb of the time
/// was some variation on. `scale` sets the size of the room, `density` its diffusion.
fn figure_eight(name: &'static str, scale: f32, density: f32, tail: f32) -> Program {
    const RETURN_L: u8 = 1;
    const RETURN_R: u8 = 2;
    const LOW_L: u8 = 0;
    const LOW_R: u8 = 3;
    let sized = |length: f32| (length * scale) as usize;

    let mut b = Builder::new(name);
    b.predelayed_input(0.6);
    for (length, gain) in [(142.0, 0.75), (107.0, 0.75), (379.0, 0.625), (277.0, 0.625)] {
        b.allpass(sized(length), gain * density);
    }
    b.op(Op::Store(PARK));

    let arms = [(672.0, 4453.0, 1800.0, 3720.0, RETURN_R, RETURN_L, LOW_L), (908.0, 4217.0, 2656.0, 3163.0, RETURN_L, RETURN_R, LOW_R)];
    let mut taps = Vec::new();
    for (first, long, second, last, feed, store, low) in arms {
        let (feedback, unity) = (b.decay(tail), b.fixed(0.999));
        b.op(Op::Clear);
        b.op(Op::Mix(PARK, unity));
        b.op(Op::Mix(feed, feedback));
        b.allpass(sized(first), -0.7 * density);
        let long_at = b.delay(sized(long));
        let damp = b.damp();
        b.op(Op::Lowpass(low, damp));
        b.allpass(sized(second), 0.5 * density);
        let last_at = b.delay(sized(last));
        b.op(Op::Store(store));
        taps.push((long_at, last_at));
    }

    let at = |base: u16, offset: f32| base + (offset * scale) as u16;
    let [(left_long, left_last), (right_long, right_last)] = [taps[0], taps[1]];
    for (from, channel, gain) in [
        (at(right_long, 266.0), 0, 0.6),
        (at(right_long, 2974.0), 0, 0.6),
        (at(right_last, 1996.0), 0, -0.5),
        (at(left_long, 353.0), 1, 0.6),
        (at(left_long, 3627.0), 1, 0.6),
        (at(left_last, 1990.0), 1, -0.5),
    ] {
        b.tap(from, channel, gain);
    }
    b.finish()
}

/// Reflections only, with a little smear: short and metallic at the stock clock.
fn early_reflections() -> Program {
    let mut b = Builder::new("Early");
    b.predelayed_input(0.8);
    let line = b.alloc(6_000);
    b.op(Op::Write(line));
    for (ticks, channel, gain) in [
        (431, 0, 0.8), (797, 1, 0.75), (1_303, 0, 0.6), (1_709, 1, 0.6), (2_311, 0, 0.5), (2_917, 1, 0.45),
        (3_571, 0, -0.4), (4_211, 1, -0.35), (4_889, 0, 0.3), (5_503, 1, 0.25), (5_903, 0, 0.2), (5_987, 1, 0.2),
    ] {
        b.tap(line + ticks, channel, gain);
    }
    b.finish()
}

/// Two long lines crossing over, the darkening repeats the Damp knob asks for.
fn stereo_echo() -> Program {
    const RETURN_L: u8 = 1;
    const RETURN_R: u8 = 2;
    const LOW_L: u8 = 0;
    const LOW_R: u8 = 3;
    const LENGTHS: [usize; 2] = [8_191, 6_143];

    let mut b = Builder::new("Echo");
    b.predelayed_input(0.8);
    b.op(Op::Store(PARK));
    for (channel, (length, (feed, store, low))) in LENGTHS.iter().zip([(RETURN_R, RETURN_L, LOW_L), (RETURN_L, RETURN_R, LOW_R)]).enumerate() {
        let (feedback, unity, wet) = (b.decay(1.0), b.fixed(0.999), b.fixed(0.9));
        b.op(Op::Clear);
        if channel == 0 {
            b.op(Op::Mix(PARK, unity));
        }
        b.op(Op::Mix(feed, feedback));
        b.delay(*length);
        let damp = b.damp();
        b.op(Op::Lowpass(low, damp));
        b.op(Op::Store(store));
        b.op(Op::Output(channel as u8, wet));
    }
    b.finish()
}

/// The room heard backwards: everything in the window plays in reverse.
fn reverse() -> Program {
    const LENGTH: usize = 7_000;
    let mut b = Builder::new("Reverse");
    b.predelayed_input(0.8);
    for (channel, length) in [LENGTH, LENGTH * 4 / 5].into_iter().enumerate() {
        let to = b.alloc(length);
        let gain = b.fixed(0.9);
        // Reverse leaves acc alone, so the next line gets the same input.
        b.op(Op::Write(to));
        b.op(Op::Reverse { to, length: length as u16, channel: channel as u8, gain });
    }
    b.finish()
}

pub const PROGRAM_COUNT: usize = 6;
pub const PROGRAM_NAMES: [&str; PROGRAM_COUNT] = ["Hall", "Club", "Plate", "Early", "Echo", "Reverse"];

pub fn all_programs() -> Vec<Program> {
    vec![
        figure_eight("Hall", 0.62, 1.0, 1.0),
        figure_eight("Club", 0.2, 0.9, 0.8),
        figure_eight("Plate", 0.42, 1.15, 1.0),
        early_reflections(),
        stereo_echo(),
        reverse(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_program_fits_the_ring_and_matches_the_names() {
        let programs = all_programs();
        assert_eq!(programs.len(), PROGRAM_COUNT);
        for (program, name) in programs.iter().zip(PROGRAM_NAMES) {
            assert_eq!(program.name, name);
            assert!(!program.ops.is_empty());
        }
    }
}
