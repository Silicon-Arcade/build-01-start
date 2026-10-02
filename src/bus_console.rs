use crate::bus_flat::FlatBus;
use crate::Bus;

/// MMIO address the emulated program writes to emit a byte of output.
pub const OUTPUT_PORT: u16 = 0xF001;

pub struct ConsoleBus {
    /// The underlying memory. Load programs through `bus.ram.memory`.
    pub ram: FlatBus,
    /// Every byte written to [`OUTPUT_PORT`], in order.
    pub output_log: Vec<u8>,
}

impl ConsoleBus {
    pub fn new() -> Self {
        ConsoleBus { ram: FlatBus::new(), output_log: Vec::new() }
    }
}

impl Default for ConsoleBus {
    fn default() -> Self {
        Self::new()
    }
}

impl Bus for ConsoleBus {
    fn memory_read(&mut self, addr: u16) -> u8 {
        self.ram.memory_read(addr)
    }

    fn memory_write(&mut self, addr: u16, value: u8) {
        match addr {
            OUTPUT_PORT => self.output_log.push(value),
            _ => self.ram.memory_write(addr, value),
        }
    }
}
