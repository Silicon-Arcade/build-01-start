use crate::Bus;

pub struct FlatBus {
    pub memory: [u8; 65536],
}

impl FlatBus {
    pub fn new() -> Self {
        FlatBus { memory: [0; 65536] }
    }
}

impl Default for FlatBus {
    fn default() -> Self {
        Self::new()
    }
}

impl Bus for FlatBus {
    fn memory_read(&mut self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn memory_write(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize] = value;
    }
}
