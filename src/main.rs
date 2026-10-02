use chip6502_core::{Chip6502, ConsoleBus};

fn main() {
    let mut bus = ConsoleBus::new();
    let mut cpu = Chip6502::new();

    let program = [
        0xA9, 0x48, // LDA #$48      'H'
        0x8D, 0x01, 0xF0, // STA $F001     write to the console port
        0x4C, 0x05, 0x02, // JMP $0205     halt: jump to self
    ];
    bus.ram.memory[0x0200..0x0200 + program.len()].copy_from_slice(&program);

    bus.ram.memory[0xFFFC] = 0x00; // reset vector -> $0200
    bus.ram.memory[0xFFFD] = 0x02;

    cpu.reset(&mut bus);
    loop {
        let pc_before = cpu.pc;
        cpu.step(&mut bus);
        if cpu.pc == pc_before {
            break; // JMP to itself: the program has halted
        }
    }

    println!("{}", String::from_utf8_lossy(&bus.output_log));
}
