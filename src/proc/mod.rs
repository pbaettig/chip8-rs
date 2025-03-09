#![allow(dead_code)]
use crate::{inst, mem, reg};
use getch_rs::{Getch, Key};
use rand::prelude::*;
use std::fmt;
use std::sync::{Arc, Mutex};
use std::{error::Error, time::Duration, time::Instant};

static RESET_VECTOR: usize = 512;

#[derive(Debug, Clone)]
pub enum ErrorKind {
    InvalidRegister(u8),
    InstructionNotImplemented(inst::Instruction),
    InstructionInvalid([u8; 2]),
    InvalidMemoryAccess(usize),
    KeyboardError,
}

#[derive(Debug, Clone)]
pub struct ProcError {
    pub kind: ErrorKind,
}

impl fmt::Display for ProcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.kind)
    }
}

impl Error for ProcError {}

pub struct Processor {
    pub memory: mem::Memory,
    pub registers: reg::Registers,
    pub pc: usize,
    pub sp: usize,
    pub i: u16,
    display: Arc<Mutex<[u8; 2048]>>,
    keys: Getch,
    rng: ThreadRng,
    stack: [u16; 128],
}

impl Processor {
    pub fn new(mem: mem::Memory, display: Arc<Mutex<[u8; 2048]>>) -> Self {
        Processor {
            memory: mem,
            registers: reg::Registers::new(),
            pc: RESET_VECTOR,
            sp: 0,
            i: 0,
            display: display,
            keys: Getch::new(),
            rng: rand::rng(),
            stack: [0; 128],
        }
    }

    fn fetch(&mut self) -> Result<[u8; 2], ProcError> {
        let bs = self.memory.get_word(self.pc).map_err(|_| ProcError {
            kind: ErrorKind::InvalidMemoryAccess(self.pc),
        })?;
        self.pc += 2;

        Ok(bs)
    }

    fn push_stack(&mut self, val: u16) {
        self.stack[self.sp] = val;
        self.sp += 1;
    }
    fn pop_stack(&mut self) -> u16 {
        self.sp -= 1;

        self.stack[self.sp]
    }

    pub fn reset(&mut self) {
        self.pc = RESET_VECTOR;
        self.sp = 0;
    }

    fn get_register(&self, index: u8) -> Result<u8, ProcError> {
        self.registers.get(index).map_err(|_| ProcError {
            kind: ErrorKind::InvalidRegister(index),
        })
    }

    fn set_register(&mut self, index: u8, value: u8) -> Result<(), ProcError> {
        self.registers.set(index, value).map_err(|_| ProcError {
            kind: ErrorKind::InvalidRegister(index),
        })
    }

    fn fetch_and_decode(&mut self) -> Result<inst::Instruction, ProcError> {
        let bs = self.fetch()?;
        inst::Instruction::parse(bs).map_err(|_| ProcError {
            kind: ErrorKind::InstructionInvalid(bs),
        })
    }

    fn read_key(&self) -> Result<u8, ProcError> {
        loop {
            match self.keys.getch() {
                Ok(Key::Char(c)) => {
                    // let ord: u8 = c as u8 - 49; // '1' -> 0, '2' ->1
                    match c {
                        '1' => return Ok(0),
                        '2' => return Ok(1),
                        '3' => return Ok(2),
                        '4' => return Ok(3),
                        'q' => return Ok(4),
                        'w' => return Ok(5),
                        'e' => return Ok(6),
                        'r' => return Ok(7),
                        'a' => return Ok(8),
                        's' => return Ok(9),
                        'd' => return Ok(10),
                        'f' => return Ok(11),
                        'z' => return Ok(12),
                        'x' => return Ok(13),
                        'c' => return Ok(14),
                        'v' => return Ok(15),
                        _ => {}
                    }
                }
                _ => {
                    return Err(ProcError {
                        kind: ErrorKind::KeyboardError,
                    })
                }
            }
        }
    }

    // Call { addr: u16 },
    // Display,
    // Return,
    // Goto { addr: u16 },
    // CallSubroutine { addr: u16 },
    // SkipIfRegisterEquals { register: u8, value: u8 },
    // SkipIfRegisterNotEquals { register: u8, value: u8 },
    // SkipIfRegistersEqual { register_1: u8, register_2: u8 },
    // SetRegister { register: u8, value: u8 },
    // AddToRegister { register: u8, value: u8 },
    // CopyRegister { src_register: u8, dst_register: u8 },
    // ApplyBitwiseOr { value_register: u8, operand_register: u8 },
    // ApplyBitwiseAnd { value_register: u8, operand_register: u8 },
    // ApplyBitwiseXor { value_register: u8, operand_register: u8 },
    // AddRegisters { value_register: u8, operand_register: u8 },
    // SubtractRegisters { value_register: u8, operand_register: u8 },
    // SetI { addr: u16 },
    // DumpRegisters { end_register: u8 },

    pub fn execute(&mut self) -> Result<Duration, ProcError> {
        let start = Instant::now();
        let instruction = self.fetch_and_decode()?;
        println!("(PC:{}, SP:{}) | {:?}", self.pc, self.sp, instruction);
        match instruction {
            inst::Instruction::ClearDisplay => {
                let mut display = self.display.lock().unwrap();
                display.fill(0);
                Ok(start.elapsed())
            }
            inst::Instruction::Draw {
                reg_x,
                reg_y,
                sprite_height,
            } => {
                let x = self.get_register(reg_x)?;
                let y = self.get_register(reg_y)?;
                let mut display = self.display.lock().unwrap();
                for y_offset in 0..sprite_height {
                    let row_addr = (self.i + (y_offset as u16)) as usize;
                    let row = self.memory.get_byte(row_addr).map_err(|_| ProcError {
                        kind: ErrorKind::InvalidMemoryAccess(row_addr),
                    })?;
                    for x_offset in 0..8 {
                        let pixel_addr = (x + x_offset) as usize + (((y + y_offset) as usize) * 64);
                        println!("Coords:({}, {}), Addr:{}", x + x_offset, y + y_offset, pixel_addr);

                        if row & (1 << x_offset) > 0 {
                            display[pixel_addr] = 1;
                        } else {
                            display[pixel_addr] = 0;
                        }
                    }
                }
                // let row = self.memory.get_byte(self.i as usize);
                Ok(start.elapsed())
            }
            inst::Instruction::Return => {
                self.pc = self.pop_stack() as usize;
                Ok(start.elapsed())
            }
            inst::Instruction::CallSubroutine { addr } => {
                self.push_stack(self.pc as u16);
                self.pc = addr as usize;

                Ok(start.elapsed())
            }
            inst::Instruction::Goto { addr } => {
                self.pc = addr as usize;
                Ok(start.elapsed())
            }
            inst::Instruction::GotoPlusV0 { addr } => {
                let v0 = self.get_register(reg::V0)? as u16;
                self.pc = (addr + v0) as usize;
                Ok(start.elapsed())
            }
            inst::Instruction::Call { addr: _ } => Ok(start.elapsed()),
            inst::Instruction::SetRegister { register, value } => {
                self.set_register(register, value)?;
                Ok(start.elapsed())
            }
            inst::Instruction::SetRegisterRandomBitwiseAnd { register, and_operand } => {
                let r = self.rng.random::<u8>();
                self.set_register(register, r & and_operand)?;
                Ok(start.elapsed())
            }
            inst::Instruction::AddToRegister { register, value } => {
                let r_v = self.get_register(register)?;
                self.set_register(register, r_v + value)?;
                Ok(start.elapsed())
            }
            inst::Instruction::CopyRegister {
                src_register,
                dst_register,
            } => {
                let v = self.get_register(src_register)?;
                self.set_register(dst_register, v)?;
                Ok(start.elapsed())
            }
            inst::Instruction::ApplyBitwiseOr {
                value_register,
                operand_register,
            } => {
                let a = self.get_register(value_register)?;
                let b = self.get_register(operand_register)?;

                self.set_register(value_register, a | b)?;
                Ok(start.elapsed())
            }
            inst::Instruction::ApplyBitwiseAnd {
                value_register,
                operand_register,
            } => {
                let a = self.get_register(value_register)?;
                let b = self.get_register(operand_register)?;

                self.set_register(value_register, a & b)?;
                Ok(start.elapsed())
            }
            inst::Instruction::ApplyBitwiseXor {
                value_register,
                operand_register,
            } => {
                let a = self.get_register(value_register)?;
                let b = self.get_register(operand_register)?;

                self.set_register(value_register, a ^ b)?;
                Ok(start.elapsed())
            }
            inst::Instruction::AddRegisters {
                value_register,
                operand_register,
            } => {
                let a = self.get_register(value_register)?;
                let b = self.get_register(operand_register)?;

                let (v, of) = a.overflowing_add(b);
                self.registers.vf = if of { 1 } else { 0 };

                self.set_register(value_register, v)?;
                Ok(start.elapsed())
            }
            inst::Instruction::SubtractRegisters {
                value_register,
                operand_register,
            } => {
                let a = self.get_register(value_register)?;
                let b = self.get_register(operand_register)?;

                let (v, of) = a.overflowing_sub(b);
                self.registers.vf = if of { 1 } else { 0 };
                self.set_register(value_register, v)?;
                Ok(start.elapsed())
            }
            inst::Instruction::SetI { addr } => {
                self.i = addr;
                Ok(start.elapsed())
            }
            inst::Instruction::DumpRegisters { end_register } => {
                for r_v in self.registers.as_array()[0..(end_register + 1) as usize].iter() {
                    self.memory.set_byte(self.i as usize, *r_v).map_err(|_| ProcError {
                        kind: ErrorKind::InvalidMemoryAccess(self.i as usize),
                    })?;
                    self.i += 1;
                }
                Ok(start.elapsed())
            }
            inst::Instruction::SkipIfRegisterEquals { register, value } => {
                let r_v = self.get_register(register)?;
                if r_v == value {
                    self.pc += 2;
                }
                Ok(start.elapsed())
            }
            inst::Instruction::SkipIfRegisterNotEquals { register, value } => {
                let r_v = self.get_register(register)?;
                if r_v != value {
                    self.pc += 2;
                }
                Ok(start.elapsed())
            }
            inst::Instruction::SkipIfRegistersEqual { register_1, register_2 } => {
                let r1_value = self.get_register(register_1)?;
                let r2_value = self.get_register(register_2)?;
                if r1_value == r2_value {
                    self.pc += 2;
                }
                Ok(start.elapsed())
            }
            inst::Instruction::GetKey { register: reg } => {
                let k = self.read_key()?;
                self.set_register(reg, k)?;

                Ok(start.elapsed())
            }
            _ => Err(ProcError {
                kind: ErrorKind::InstructionNotImplemented(instruction),
            }),
        }
    }
}

mod tests {
    use crate::mem::Memory;

    // use super::*;

    fn load_test_program(mem: &mut Memory) {
        // ** Construct a test program in Memory
        let jmp: [u8; 2] = [0x14, 0xd2]; // Goto 0x4d2 (1234)

        let subroutine: [u8; 10] = [
            0x68, 0x32, // Set V8 to 50
            0x69, 0x2a, // Set V9 to 42
            0x88, 0x95, // V8 =- V9
            0x88, 0x95, // V8 =- V9
            0x00, 0xee, // return
        ];

        let main: [u8; 18] = [
            0x6a, 0xFA, // Set VA to 250
            0x7a, 0x05, // Add 5 to VA
            0xA0, 0xFF, // Set I = 255
            0xF8, 0x55, // reg dump V0 - V8 (inclusive) to 0xFF
            0x6e, 0x2a, // Set VE to 42
            0x8A, 0xE4, // Add register VE to VA
            0x24, 0x00, // Call Subroutine at 1024
            0x3a, 0x29, // skip if VA == 41 (true)
            0xff, 0xff, // Illegal instruction
        ];
        let _ = mem.set_word(512, jmp);
        let _ = mem.load_array(1024, &subroutine);
        let _ = mem.load_array(1234, &main);
    }

    #[test]
    fn test_run_program() {
        let mut mem = Memory::new();
        load_test_program(&mut mem);

        let mut proc = super::Processor::new(mem);
        proc.registers
            .from_array(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);

        // Execute jmp to 1234 (0x4d2)
        let r = proc.execute();
        assert!(matches!(r, Ok(_)));
        assert_eq!(proc.pc, 1234);

        // Set VA = 250
        let r = proc.execute();
        assert!(matches!(r, Ok(_)));

        assert_eq!(proc.registers.va, 250);

        // VA += 255
        let r = proc.execute();
        assert!(matches!(r, Ok(_)));
        assert_eq!(proc.registers.va, 255);

        // Set I = 255
        let r = proc.execute();
        assert!(matches!(r, Ok(_)));
        assert_eq!(proc.i, 255);

        // Dump registers VA-V8 to memory starting at 255 (0xff)
        let r = proc.execute();
        for i in 255..264 {
            let v = proc.memory.get_byte(i).unwrap();
            let e = (i - 254) as u8;
            assert_eq!(v, e);
        }
        assert!(matches!(r, Ok(_)));
        assert_eq!(proc.memory.get_byte(264).unwrap(), 0);

        // Set VE = 42
        let r = proc.execute();
        assert!(matches!(r, Ok(_)));
        assert_eq!(proc.registers.ve, 42);

        // VE += VA (overflows)
        let r = proc.execute();
        assert!(matches!(r, Ok(_)));
        assert_eq!(proc.registers.va, 41);
        assert_eq!(proc.registers.vf, 1);

        // Call Subroutine at 0x400 (1024)
        let r = proc.execute();
        assert!(matches!(r, Ok(_)));
        assert_eq!(proc.pc, 1024);
        assert_eq!(proc.sp, 1);
        assert_eq!(proc.stack[0], 1248);

        // Set V8 = 50
        let r = proc.execute();
        assert!(matches!(r, Ok(_)));
        assert_eq!(proc.registers.v8, 50);

        // Set V9 = 42
        let r = proc.execute();
        assert!(matches!(r, Ok(_)));
        assert_eq!(proc.registers.v9, 42);

        // V8 -= V9
        let r = proc.execute();
        assert!(matches!(r, Ok(_)));
        assert_eq!(proc.registers.v8, 8);
        assert_eq!(proc.registers.vf, 0);

        // V8 -= V9 (again, overflows)
        let r = proc.execute();
        assert!(matches!(r, Ok(_)));
        assert_eq!(proc.registers.v8, 222);
        assert_eq!(proc.registers.vf, 1);

        // return from subroutine
        let r = proc.execute();
        assert!(matches!(r, Ok(_)));
        assert_eq!(proc.sp, 0);
        assert_eq!(proc.pc, proc.stack[proc.sp] as usize);

        let pre_skip_pc = proc.pc;
        // skip if VA == 41 (true)
        let r = proc.execute();
        assert!(matches!(r, Ok(_)));
        assert_eq!(proc.pc, pre_skip_pc + 4);
    }
}
