//! The delay memory and everything soldered to it.
//!
//! 32K x 8 of pseudo-static RAM holding 16,384 16-bit words, two bus cycles to a word.
//! Sixteen switches can join lines of the address and data buses to one common wire.
//! Nothing happens with a single switch on, as there's nothing for it to meet. With two
//! or more, the lines are shorted and fight: low usually wins.

pub const WORDS: usize = 16_384;
const BYTES: usize = WORDS * 2;
const BYTE_MASK: u16 = (BYTES - 1) as u16;

/// Bytes in one row of cells: they lose their refresh together.
const ROW_STRIDE: usize = 128;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Line {
    Address(u8),
    Data(u8),
}
use Line::{Address as A, Data as D};

pub const SWITCHES: usize = 16;

/// Top row: the fastest address lines, tones and ring modulation, and the sign bit,
/// which is the one that takes off.
/// Second: middling address lines, fast tremolo and waveshaping. Third: slow address
/// lines, rhythms that follow the clock. Bottom: data lines, crunch.
pub const MATRIX: [Line; SWITCHES] = [
    A(1), A(2), A(3), D(7),
    A(5), A(6), A(7), A(8),
    A(10), A(11), A(12), A(13),
    D(4), D(3), D(2), D(1),
];

/// A small, fast generator of unimportant decisions.
#[derive(Clone)]
pub struct Rng(u32);

impl Rng {
    pub fn new(seed: u32) -> Self {
        Self(seed | 1)
    }

    #[inline(always)]
    pub fn bits(&mut self) -> u32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 17;
        self.0 ^= self.0 << 5;
        self.0
    }

    /// True with this probability.
    #[inline(always)]
    pub fn chance(&mut self, probability: f32) -> bool {
        (self.bits() as f32) < probability * u32::MAX as f32
    }
}

pub struct Bus {
    cells: Box<[u8]>,
    /// The lines on the common wire. Both empty unless two or more switches are on.
    address_net: u16,
    data_net: u8,
    /// How often a line fighting a low line wins anyway.
    contention: f32,
    write_enable: bool,
    /// What the data bus last carried: it stays there until something drives it.
    held: u8,
    rng: Rng,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            cells: vec![0; BYTES].into_boxed_slice(),
            address_net: 0,
            data_net: 0,
            contention: 0.0,
            write_enable: true,
            held: 0,
            rng: Rng::new(0x51832),
        }
    }

    /// Solder: which switches are closed, in `MATRIX` order.
    pub fn set_matrix(&mut self, closed: &[bool; SWITCHES]) {
        let (mut address, mut data, mut count) = (0u16, 0u8, 0);
        for line in MATRIX.iter().zip(closed).filter(|(_, on)| **on).map(|(line, _)| *line) {
            count += 1;
            match line {
                Line::Address(bit) => address |= 1 << bit,
                Line::Data(bit) => data |= 1 << bit,
            }
        }
        (self.address_net, self.data_net) = if count >= 2 { (address, data) } else { (0, 0) };
    }

    pub fn set_contention(&mut self, contention: f32) {
        self.contention = contention;
    }

    pub fn set_write_enable(&mut self, enabled: bool) {
        self.write_enable = enabled;
    }

    #[inline(always)]
    fn tied(&self) -> bool {
        self.address_net != 0 || self.data_net != 0
    }

    /// Where the common wire settles with these lines driving it.
    #[inline(always)]
    fn wire(&mut self, address: u16, data: u8) -> bool {
        let (a, d) = (address & self.address_net, data & self.data_net);
        if a == self.address_net && d == self.data_net {
            true
        } else if a == 0 && d == 0 {
            false
        } else {
            self.contention > 0.0 && self.rng.chance(self.contention)
        }
    }

    #[inline(always)]
    fn on_wire_address(&self, address: u16, high: bool) -> u16 {
        if high { address | self.address_net } else { address & !self.address_net }
    }

    #[inline(always)]
    fn on_wire_data(&self, data: u8, high: bool) -> u8 {
        if high { data | self.data_net } else { data & !self.data_net }
    }

    #[inline(always)]
    fn read_byte(&mut self, address: u16) -> u8 {
        let data = if self.tied() {
            // The address settles first, against whatever the bus still holds.
            let wire = self.wire(address, self.held);
            let cell = self.on_wire_address(address, wire) & BYTE_MASK;
            let data = self.cells[cell as usize];
            let wire = self.wire(address, data);
            self.on_wire_data(data, wire)
        } else {
            self.cells[address as usize]
        };
        self.held = data;
        data
    }

    #[inline(always)]
    fn write_byte(&mut self, address: u16, data: u8) {
        let (cell, data) = if self.tied() {
            let wire = self.wire(address, data);
            (self.on_wire_address(address, wire) & BYTE_MASK, self.on_wire_data(data, wire))
        } else {
            (address, data)
        };
        self.held = data;
        if self.write_enable {
            self.cells[cell as usize] = data;
        }
    }

    /// `word` must already be wrapped to `WORDS`.
    #[inline(always)]
    pub fn read(&mut self, word: u16) -> i16 {
        let low = self.read_byte(word << 1);
        let high = self.read_byte((word << 1) | 1);
        i16::from_le_bytes([low, high])
    }

    #[inline(always)]
    pub fn write(&mut self, word: u16, value: i16) {
        let [low, high] = value.to_le_bytes();
        self.write_byte(word << 1, low);
        self.write_byte((word << 1) | 1, high);
    }

    /// A row of cells misses its refresh: some of their charge is gone, so ones become
    /// zeros. It runs across the whole memory, so it comes back as a pitched crackle.
    pub fn rot_row(&mut self) {
        let row = self.rng.bits() as usize % ROW_STRIDE;
        let leak = self.rng.bits() as u8 & self.rng.bits() as u8;
        for cell in self.cells.iter_mut().skip(row).step_by(ROW_STRIDE) {
            *cell &= !leak;
        }
    }

    pub fn rng(&mut self) -> &mut Rng {
        &mut self.rng
    }
}

impl Default for Bus {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn closed(which: &[usize]) -> [bool; SWITCHES] {
        std::array::from_fn(|i| which.contains(&i))
    }

    #[test]
    fn stores_every_word_faithfully_when_nothing_is_soldered() {
        let mut bus = Bus::new();
        for w in 0..WORDS as u16 {
            bus.write(w, (w as i32 * 7 - 30_000) as i16);
        }
        for w in 0..WORDS as u16 {
            assert_eq!(bus.read(w), (w as i32 * 7 - 30_000) as i16);
        }
    }

    #[test]
    fn one_switch_alone_does_nothing() {
        for s in 0..SWITCHES {
            let mut bus = Bus::new();
            bus.set_matrix(&closed(&[s]));
            bus.write(0x1234, 0x5A5A);
            assert_eq!(bus.read(0x1234), 0x5A5A);
        }
    }

    #[test]
    fn two_address_lines_fold_the_memory() {
        // A10 and A11 are word bits 9 and 10. Low wins, so these words land together.
        let mut bus = Bus::new();
        bus.set_matrix(&closed(&[8, 9]));
        bus.write(1 << 9, 111);
        assert_eq!(bus.read(0), 111);
        bus.write(1 << 10, 222);
        assert_eq!(bus.read(0), 222);
        // Both high stays put.
        bus.write(3 << 9, 333);
        assert_eq!(bus.read(0), 222);
        assert_eq!(bus.read(3 << 9), 333);
    }

    #[test]
    fn two_data_lines_hit_both_bytes_of_a_word() {
        // D4 and D3: in each byte, bits 4 and 3 become their AND.
        let mut bus = Bus::new();
        bus.set_matrix(&closed(&[12, 13]));
        bus.write(9, 0x1008);
        assert_eq!(bus.read(9) as u16, 0x0000);
        bus.write(9, 0x1818);
        assert_eq!(bus.read(9) as u16, 0x1818);
    }

    #[test]
    fn an_address_line_on_a_data_line_writes_the_counter_into_the_audio() {
        // With silence going in, a high address line and a high data line can only come
        // from each other. Contention lets the address line win now and then.
        let mut bus = Bus::new();
        bus.set_matrix(&closed(&[3, 8]));
        bus.set_contention(1.0);
        for w in 0..WORDS as u16 {
            bus.write(w, 0);
        }
        bus.set_matrix(&closed(&[]));
        assert!((0..WORDS as u16).any(|w| bus.read(w) != 0));
    }

    #[test]
    fn a_disabled_write_leaves_the_memory_alone() {
        let mut bus = Bus::new();
        bus.write(5, 100);
        bus.set_write_enable(false);
        bus.write(5, 200);
        assert_eq!(bus.read(5), 100);
    }

    #[test]
    fn rot_only_ever_clears_bits() {
        let mut bus = Bus::new();
        for w in 0..WORDS as u16 {
            bus.write(w, -1);
        }
        for _ in 0..50 {
            bus.rot_row();
        }
        let values: Vec<i16> = (0..WORDS as u16).map(|w| bus.read(w)).collect();
        assert!(values.iter().any(|v| *v != -1));
        assert!(values.iter().any(|v| *v == -1));
    }
}
