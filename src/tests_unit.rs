use crate::{Chip6502, FlatBus, StatusFlag};

#[test]
fn ldx_immediate_loads_x_and_sets_flags() {
    let mut bus = FlatBus::new();
    let mut cpu = Chip6502::new();
    bus.memory[0x0200] = 0xA2; // LDX #$44
    bus.memory[0x0201] = 0x44;
    cpu.pc = 0x0200;
    cpu.p |= StatusFlag::Carry as u8; // set C — LDX must leave it alone

    cpu.step(&mut bus);

    assert_eq!(cpu.x, 0x44);
    assert_eq!(cpu.pc, 0x0202); // stepped past both bytes
    assert_eq!(cpu.p & StatusFlag::Zero as u8, 0); // Z clear: $44 is not zero
    assert_eq!(cpu.p & StatusFlag::Negative as u8, 0); // N clear: bit 7 of $44 is 0
    assert_eq!(
        cpu.p & StatusFlag::Carry as u8, // C survived —
        StatusFlag::Carry as u8
    ); //   "others unchanged"
}

#[test]
fn lda_absolute_x_indexes_from_base() {
    let mut bus = FlatBus::new();
    let mut cpu = Chip6502::new();
    bus.memory[0x0200] = 0xBD; // LDA $0300,X
    bus.memory[0x0201] = 0x00;
    bus.memory[0x0202] = 0x03;
    bus.memory[0x0300] = 0xEE; // decoy — the answer if X is ignored
    bus.memory[0x0305] = 0x42; // the answer if X is added
    cpu.pc = 0x0200;
    cpu.x = 0x05;

    cpu.step(&mut bus);

    assert_eq!(cpu.a, 0x42);
    assert_eq!(cpu.pc, 0x0203); // stepped past all three bytes
}

#[test]
fn ldx_immediate_zero_sets_z() {
    let mut bus = FlatBus::new();
    let mut cpu = Chip6502::new();
    bus.memory[0x0200] = 0xA2; // LDX #$00
    bus.memory[0x0201] = 0x00;
    cpu.pc = 0x0200;

    cpu.step(&mut bus);

    assert_eq!(cpu.x, 0x00);
    assert_eq!(
        cpu.p & StatusFlag::Zero as u8, // Z set: the result is zero
        StatusFlag::Zero as u8
    );
    assert_eq!(cpu.p & StatusFlag::Negative as u8, 0); // N clear: bit 7 of $00 is 0
}

#[test]
fn ldx_immediate_negative_sets_n() {
    let mut bus = FlatBus::new();
    let mut cpu = Chip6502::new();
    bus.memory[0x0200] = 0xA2; // LDX #$80
    bus.memory[0x0201] = 0x80;
    cpu.pc = 0x0200;

    cpu.step(&mut bus);

    assert_eq!(cpu.x, 0x80);
    assert_eq!(cpu.p & StatusFlag::Zero as u8, 0); // Z clear: $80 is not zero
    assert_eq!(
        cpu.p & StatusFlag::Negative as u8, // N set: bit 7 of $80 is 1
        StatusFlag::Negative as u8
    );
}
