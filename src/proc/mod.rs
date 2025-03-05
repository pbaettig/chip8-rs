use crate::{mem, ops, reg};
use std::fmt;
use std::{error::Error, time::Duration, time::Instant};

static RESET_VECTOR: usize = 512;

#[derive(Debug, Clone)]
pub enum ErrorKind {
    InvalidRegister(u8),
    OpcodeNotImplemented(ops::Opcode),
    OpcodeInvalid([u8; 2]),
    InvalidMemoryAccess,
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
    pub PC: usize,
    pub SP: usize,
    I: u16,
    stack: [u16; 128],
}

impl Processor {
    pub fn new(mem: mem::Memory) -> Self {
        Processor {
            memory: mem,
            registers: reg::Registers::new(),
            PC: RESET_VECTOR,
            SP: 0,
            I: 0,
            stack: [0; 128],
        }
    }

    fn fetch(&mut self) -> Result<[u8; 2], ProcError> {
        let bs = self.memory.get_word(self.PC).map_err(|_| ProcError {
            kind: ErrorKind::InvalidMemoryAccess,
        })?;
        self.PC += 2;

        Ok(bs)
    }

    fn push_stack(&mut self, val: u16) {
        self.stack[self.SP] = val;
        self.SP += 1;
    }
    fn pop_stack(&mut self) -> u16 {
        self.SP -= 1;

        self.stack[self.SP]
    }

    pub fn reset(&mut self) {
        self.PC = RESET_VECTOR;
        self.SP = 0;
    }

    fn get_register(&self, index: u8) -> Result<u8, ProcError> {
        self.registers.get(index).ok_or(ProcError {
            kind: ErrorKind::InvalidRegister(index),
        })
    }

    fn fetch_and_decode(&mut self) -> Result<ops::Opcode, ProcError> {
        let bs = self.fetch()?;
        ops::Opcode::parse(bs).map_err(|_| ProcError {
            kind: ErrorKind::OpcodeInvalid(bs),
        })
    }

    pub fn execute(&mut self) -> Result<Duration, ProcError> {
        let start = Instant::now();
        let opcode = self.fetch_and_decode()?;
        // println!("(PC:{}, SP:{}) | {op:?}", self.PC, self.SP);
        match opcode {
            ops::Opcode::CallSubroutine { addr } => {
                self.push_stack(self.PC as u16);
                self.PC = addr as usize;

                Ok(start.elapsed())
            }
            ops::Opcode::Goto { addr } => {
                self.PC = addr as usize;
                Ok(start.elapsed())
            }
            ops::Opcode::Return => {
                self.PC = self.pop_stack() as usize;
                Ok(start.elapsed())
            }
            ops::Opcode::SetRegister { register, value } => {
                self.registers.set(register, value);
                Ok(start.elapsed())
            }
            ops::Opcode::AddToRegister { register, value } => {
                let r_v = self.get_register(register)?;
                self.registers.set(register, r_v + value);
                Ok(start.elapsed())
            }
            ops::Opcode::AddRegisters {
                value_register,
                operand_register,
            } => {
                let a = self.get_register(value_register)?;
                let b = self.get_register(operand_register)?;

                let (v, of) = a.overflowing_add(b);
                self.registers.VF = if of { 1 } else { 0 };

                self.registers.set(value_register, v);
                Ok(start.elapsed())
            }
            ops::Opcode::SubtractRegisters {
                value_register,
                operand_register,
            } => {
                let a = self.get_register(value_register)?;
                let b = self.get_register(operand_register)?;

                let (v, of) = a.overflowing_sub(b);
                self.registers.VF = if of { 1 } else { 0 };
                self.registers.set(value_register, v);
                Ok(start.elapsed())
            }
            ops::Opcode::SetI { addr } => {
                self.I = addr;
                Ok(start.elapsed())
            }
            ops::Opcode::DumpRegisters { end_register } => {
                for r_v in self.registers.as_array()[0..(end_register + 1) as usize].iter() {
                    self.memory.set_byte(self.I as usize, **r_v);
                    self.I += 1;
                }
                Ok(start.elapsed())
            }
            ops::Opcode::SkipIfRegisterEquals { register, value } => {
                let r_v = self.get_register(register)?;
                if r_v == value {
                    self.PC += 2;
                }
                Ok(start.elapsed())
            }
            ops::Opcode::SkipIfRegisterNotEquals { register, value } => {
                let r_v = self.get_register(register)?;
                if r_v != value {
                    self.PC += 2;
                }
                Ok(start.elapsed())
            }
            ops::Opcode::SkipIfRegistersEqual { register_1, register_2 } => {
                let r1_value = self.get_register(register_1)?;
                let r2_value = self.get_register(register_2)?;
                if r1_value == r2_value {
                    self.PC += 2;
                }
                Ok(start.elapsed())
            }
            ops::Opcode::Display => Ok(start.elapsed()),

            _ => Err(ProcError {
                kind: ErrorKind::OpcodeNotImplemented(opcode),
            }),
        }
    }
}

mod tests {
    use crate::mem::Memory;

    use super::*;

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
        mem.set_word(512, jmp);
        mem.load_array(1024, &subroutine);
        mem.load_array(1234, &main);
    }

    #[test]
    fn test_run_program() {
        let mut mem = Memory::new();
        load_test_program(&mut mem);

        let mut proc = Processor::new(mem);
        proc.registers
            .from_array(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);

        // Execute jmp to 1234 (0x4d2)
        proc.execute();
        assert_eq!(proc.PC, 1234);

        // Set VA = 250
        proc.execute();
        assert_eq!(proc.registers.VA, 250);

        // VA += 255
        proc.execute();
        assert_eq!(proc.registers.VA, 255);

        // Set I = 255
        proc.execute();
        assert_eq!(proc.I, 255);

        // Dump registers VA-V8 to memory starting at 255 (0xff)
        proc.execute();
        for i in 255..264 {
            let v = proc.memory.get_byte(i).unwrap();
            let e = (i - 254) as u8;
            assert_eq!(v, e);
        }
        assert_eq!(proc.memory.get_byte(264).unwrap(), 0);

        // Set VE = 42
        proc.execute();
        assert_eq!(proc.registers.VE, 42);

        // VE += VA (overflows)
        proc.execute();
        assert_eq!(proc.registers.VA, 41);
        assert_eq!(proc.registers.VF, 1);

        // Call Subroutine at 0x400 (1024)
        proc.execute();
        assert_eq!(proc.PC, 1024);
        assert_eq!(proc.SP, 1);
        assert_eq!(proc.stack[0], 1248);

        // Set V8 = 50
        proc.execute();
        assert_eq!(proc.registers.V8, 50);

        // Set V9 = 42
        proc.execute();
        assert_eq!(proc.registers.V9, 42);

        // V8 -= V9
        proc.execute();
        assert_eq!(proc.registers.V8, 8);
        assert_eq!(proc.registers.VF, 0);

        // V8 -= V9 (again, overflows)
        proc.execute();
        assert_eq!(proc.registers.V8, 222);
        assert_eq!(proc.registers.VF, 1);

        // return from subroutine
        proc.execute();
        assert_eq!(proc.SP, 0);
        assert_eq!(proc.PC, proc.stack[proc.SP] as usize);

        let pre_skip_pc = proc.PC;
        // skip if VA == 41 (true)
        proc.execute();
        assert_eq!(proc.PC, pre_skip_pc + 4);
    }
}
