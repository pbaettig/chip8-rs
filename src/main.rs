use std::io;
mod reg;
mod ops;
mod mem;
mod proc;

fn main() -> io::Result<()> {
    let m = mem::Memory::new();
    // m.load_rom(Path::new("rom/keypad_test.ch8"))?;

    let mut proc = proc::Processor::new(m);

    proc.registers.V0 = 1;
    proc.registers.V1 = 2;
    proc.registers.V2 = 3;
    proc.registers.V3 = 4;
    proc.registers.V4 = 5;
    proc.registers.V5 = 6;
    proc.registers.V6 = 7;
    proc.registers.V7 = 8;
    proc.registers.V8 = 9;
    proc.registers.V9 = 10;
    proc.registers.VA = 11;
    proc.registers.VB = 12;
    proc.registers.VC = 13;
    proc.registers.VD = 14;
    proc.registers.VE = 15;
    proc.registers.VF = 16;

    let program: [u8; 12] = [
        0x6a, 0xFA, // Set VA to 250
        0x7a, 0x05, // Add 5 to VA
        0xA0, 0xFF, // Set I = 255
        0xF5, 0x55, // reg dump
        0x8A, 0xE4, // Add register VE to VA
        0x24, 0x00, // Call Subroutine at 1024
    ];

    let subroutine: [u8; 4] = [0x00, 0xE0, 0x00, 0xEE];
    proc.memory.load_array(512, &program);
    proc.memory.load_array(1024, &subroutine);
    println!("{}", proc.memory);
    for _ in 0..((program.len() / 2) + (subroutine.len() / 2) + 1) {
        proc.decode_and_execute();
        println!("{:?}", proc.registers);
        println!();
    }


    Ok(())
}
