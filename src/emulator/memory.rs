use crate::emulator::bus::Xlen;

pub const MEMORY_SIZE: u32 = 1024 * 1024 * 1024;
pub const MEMORY_BASE_ADDRESS: u32 = 0x8000_0000;

pub struct Memory {
    pub memory: Vec<u8>,
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            memory: vec![0; MEMORY_SIZE as usize],
        }
    }
}

impl Memory {
    pub fn size(&self) -> u32 {
        MEMORY_BASE_ADDRESS + MEMORY_SIZE
    }

    pub fn load(&self, address: u32, xlen: Xlen) -> u32 {
        (0..xlen as usize).fold(0, |acc, i| {
            acc | (self.memory[(address - MEMORY_BASE_ADDRESS) as usize + i] as u32) << (8 * i)
        })
    }

    pub fn store(&mut self, address: u32, value: u32, xlen: Xlen) {
        for i in 0..xlen as usize {
            self.memory[(address - MEMORY_BASE_ADDRESS) as usize + i] = (value >> (i * 8)) as u8;
        }
    }
}