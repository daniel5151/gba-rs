use std::default::Default;
use std::ops::{Index, IndexMut};

use bit_util::extract;

pub type Reg = u8;

const NUM_RGSR: Reg = 37;

pub const SP: Reg = 13;
pub const LR: Reg = 14;
pub const PC: Reg = 15;
pub const CPSR: Reg = 16;
pub const SPSR: Reg = 17;

const REG_MAP: [[usize; 18]; 6] = [
    [0, 1, 2, 3, 4, 5, 6, 7,  8,  9, 10, 11, 12, 13, 14, 15, 16, 16],
    [0, 1, 2, 3, 4, 5, 6, 7, 17, 18, 19, 20, 21, 22, 23, 15, 16, 24],
    [0, 1, 2, 3, 4, 5, 6, 7,  8,  9, 10, 11, 12, 25, 26, 15, 16, 27],
    [0, 1, 2, 3, 4, 5, 6, 7,  8,  9, 10, 11, 12, 28, 29, 15, 16, 30],
    [0, 1, 2, 3, 4, 5, 6, 7,  8,  9, 10, 11, 12, 31, 32, 15, 16, 33],
    [0, 1, 2, 3, 4, 5, 6, 7,  8,  9, 10, 11, 12, 34, 35, 15, 16, 36],
];

pub struct RegFile {
    reg: [u32; NUM_RGSR as usize],
}

impl RegFile {
    pub fn mode(&self) -> u32 {
        extract(self.reg[CPSR as usize], 0, 5)
    }

    pub fn bank(&self) -> usize {
        match self.mode() {
            0x10 => 0, // user
            0x11 => 1, // FIQ
            0x12 => 2, // IRQ
            0x13 => 3, // Supervisor
            0x17 => 4, // Abort
            0x1B => 5, // Undefined
            0x1F => 0, // privileged user
            val => { warn!("Invalid mode: {:#010x}", val); 0 }
        }
    }

    pub fn set(&mut self, bank: usize, reg: Reg, val: u32) {
        self.reg[REG_MAP[bank][reg as usize]] = val;
    }

    pub fn get(&self, bank: usize, reg: Reg) -> u32 {
        self.reg[REG_MAP[bank][reg as usize]]
    }
}

impl Default for RegFile {
    fn default() -> RegFile {
        RegFile { reg: [0; NUM_RGSR as usize] }
    }
}

impl Index<Reg> for RegFile {
    type Output = u32;
    #[inline]
    fn index(&self, idx: Reg) -> &u32 {
        &self.reg[REG_MAP[self.bank()][idx as usize]]
    }
}

impl IndexMut<Reg> for RegFile {
    #[inline]
    fn index_mut(&mut self, idx: Reg) -> &mut u32 {
        &mut self.reg[REG_MAP[self.bank()][idx as usize]]
    }
}

pub mod cpsr {
    use super::Reg;

    pub const N: Reg = 31;
    pub const Z: Reg = 30;
    pub const C: Reg = 29;
    pub const V: Reg = 28;

    pub const T: Reg = 5;
}
