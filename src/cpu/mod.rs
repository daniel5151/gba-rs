use std;
use std::default::Default;
use std::ops::{Index, IndexMut};

use super::mmu::Mmu;

mod bit_util;
mod arm;

use self::arm::ArmIsaCpu;

type Reg = u8;

const NUM_RGSR: Reg = 37;

const PC_REG: Reg = 15;
const CPSR_REG: Reg = 36;

mod reg {
    use super::Reg;

    pub const PC: Reg = 15;
    pub const CPSR: Reg = 36;
}

mod cpsr {
    use super::Reg;

    pub const N: Reg = 31;
    pub const Z: Reg = 30;
    pub const C: Reg = 29;
    pub const V: Reg = 28;

    pub const T: Reg = 5;
}

enum IsaMode {
    Arm,
    Thumb,
}

struct RegFile {
    reg: [u32; NUM_RGSR as usize],
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
        &self.reg[idx as usize]
    }
}

impl IndexMut<Reg> for RegFile {
    #[inline]
    fn index_mut(&mut self, idx: Reg) -> &mut u32 {
        &mut self.reg[idx as usize]
    }
}

pub struct Cpu {
    reg: RegFile,
    mmu: Mmu,
    isa_mode: IsaMode,
}

impl Cpu {
    pub fn new(mmu: Mmu) -> Cpu {
        Cpu {
            reg: Default::default(),
            mmu: mmu,
            isa_mode: IsaMode::Arm,
        }
    }
}
