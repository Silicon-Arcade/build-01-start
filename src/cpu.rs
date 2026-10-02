use crate::{Bus, StatusFlag};

pub struct Chip6502 {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u8,
    pub pc: u16,
    pub p: u8,
}

impl Chip6502 {
    /// A fresh CPU in a deterministic zeroed state. Call `reset`
    /// afterwards to apply the NMOS startup defaults.
    pub fn new() -> Self {
        Chip6502 { a: 0, x: 0, y: 0, sp: 0, pc: 0, p: 0 }
    }

    /// Hardware reset: load PC from the vector at `$FFFC/$FFFD` (low
    /// byte first), set I and the always-on bit 5, and consume the
    /// three bytes of stack the silicon's dummy pushes would.
    pub fn reset<B: Bus>(&mut self, bus: &mut B) {
        let low = u16::from(bus.memory_read(0xFFFC));
        let high = u16::from(bus.memory_read(0xFFFD));
        self.pc = (high << 8) | low;
        self.sp = self.sp.wrapping_sub(3);
        self.p = 0b0010_0100; // I = 1, bit 5 = 1: the NMOS startup state, $24
    }

    /// Clear-then-set the two flags most instructions own: Z when the
    /// result is zero, N as a straight copy of the result's bit 7.
    fn set_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.p |= StatusFlag::Zero as u8;
        } else {
            self.p &= !(StatusFlag::Zero as u8);
        }
        if result & 0b1000_0000 != 0 {
            self.p |= StatusFlag::Negative as u8;
        } else {
            self.p &= !(StatusFlag::Negative as u8);
        }
    }

    /// Execute one instruction.
    pub fn step<B: Bus>(&mut self, bus: &mut B) {
        let opcode = bus.memory_read(self.pc);

        match opcode {
            0xA9 => {
                // LDA #imm
                self.a = bus.memory_read(self.pc.wrapping_add(1));
                self.set_zero_and_negative_flags(self.a);
                self.pc = self.pc.wrapping_add(2);
            }
            0xA2 => {
                // LDX #imm
                self.x = bus.memory_read(self.pc.wrapping_add(1));
                self.set_zero_and_negative_flags(self.x);
                self.pc = self.pc.wrapping_add(2);
            }
            0xBD => {
                // LDA abs,X
                let low = u16::from(bus.memory_read(self.pc.wrapping_add(1)));
                let high = u16::from(bus.memory_read(self.pc.wrapping_add(2)));
                let base = (high << 8) | low;
                let addr = base.wrapping_add(u16::from(self.x));
                self.a = bus.memory_read(addr);
                self.set_zero_and_negative_flags(self.a);
                self.pc = self.pc.wrapping_add(3);
            }
            0x8D => {
                // STA abs
                let low = u16::from(bus.memory_read(self.pc.wrapping_add(1)));
                let high = u16::from(bus.memory_read(self.pc.wrapping_add(2)));
                bus.memory_write((high << 8) | low, self.a);
                self.pc = self.pc.wrapping_add(3);
            }
            0x4C => {
                // JMP abs
                let low = u16::from(bus.memory_read(self.pc.wrapping_add(1)));
                let high = u16::from(bus.memory_read(self.pc.wrapping_add(2)));
                self.pc = (high << 8) | low;
            }
            _ => panic!("unimplemented opcode ${opcode:02X} at ${:04X}", self.pc),
        }
    }
}

impl Default for Chip6502 {
    fn default() -> Self {
        Self::new()
    }
}
