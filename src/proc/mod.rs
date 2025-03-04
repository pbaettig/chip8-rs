use crate::{mem, ops, reg};


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
            PC: 512,
            SP: 0,
            I: 0,
            stack: [0; 128],
        }
    }

    fn fetch(&mut self) -> [u8; 2] {
        let bs = self.memory.get_word(self.PC);
        self.PC += 2;

        bs
    }

    fn push_stack(&mut self, val: u16) {
        self.stack[self.SP] = val;
        self.SP += 1;
    }
    fn pop_stack(&mut self) -> u16 {
        self.SP -= 1;

        self.stack[self.SP]
    }

    pub fn decode_and_execute(&mut self) {
        let op = ops::Opcode::parse(self.fetch()).unwrap();
        // println!("(PC:{}, SP:{}) | {op:?}", self.PC, self.SP);
        match op {
            ops::Opcode::CallSubroutine { addr } => {
                self.push_stack(self.PC as u16);
                self.PC = addr as usize;
            }
            ops::Opcode::Goto { addr } => self.PC = addr as usize,
            ops::Opcode::Return => {
                self.PC = self.pop_stack() as usize;
            }
            ops::Opcode::SetRegister { register, value } => self.registers.set(register, value),
            ops::Opcode::AddToRegister { register, value } => self
                .registers
                .set(register, self.registers.get(register).unwrap() + value),
            ops::Opcode::AddRegisters {
                value_register,
                operand_register,
            } => {
                let a = self.registers.get(value_register).unwrap();
                let b = self.registers.get(operand_register).unwrap();

                let (v, of) = a.overflowing_add(b);
                self.registers.VF = if of { 1 } else { 0 };

                self.registers.set(value_register, v);
            }
            ops::Opcode::SubtractRegisters {
                value_register,
                operand_register,
            } => {
                let a = self.registers.get(value_register).unwrap();
                let b = self.registers.get(operand_register).unwrap();

                let (v, of) = a.overflowing_sub(b);
                self.registers.VF = if of { 1 } else { 0 };
                self.registers.set(value_register, v);
            }
            ops::Opcode::SetI { addr } => {
                self.I = addr;
            }
            ops::Opcode::DumpRegisters { end_register } => {
                for r_v in self.registers.as_array()[0..(end_register + 1) as usize].iter() {
                    self.memory.set_byte(self.I as usize, **r_v);
                    self.I += 1;
                }
            }
            ops::Opcode::SkipIfRegisterEquals { register, value } => {
                if self.registers.get(register).unwrap() == value {
                    self.PC += 2;
                }
            }
            _ => return,
        }
    }
}

mod tests {
    use crate::mem::Memory;

    use super::*;

    fn load_test_program(mem: &mut Memory) {
        // ** Construct a test program in Memory
        // Goto 0x4d2 (1234)
        let jmp: [u8;2] =[0x14, 0xd2]; 
        
        let subroutine: [u8;10] = [
            0x68, 0x32, // Set V8 to 50
            0x69, 0x2a, // Set V9 to 42
            0x88, 0x95, // V8 =- V9
            0x88, 0x95, // V8 =- V9
            0x00, 0xee, // return
        ];
        
        let main: [u8;18] = [
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
        mem.load_array(
            1234,
            &main
        );
    }

    #[test]
    fn test_run_program() {
        let mut mem = Memory::new();
        load_test_program(&mut mem);

        let mut proc = Processor::new(mem);
        proc.registers.from_array(&[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16]);
        
        // Execute jmp to 1234 (0x4d2)
        proc.decode_and_execute();
        assert_eq!(proc.PC, 1234);

        // Set VA = 250
        proc.decode_and_execute();
        assert_eq!(proc.registers.VA, 250);

        // VA += 255
        proc.decode_and_execute();
        assert_eq!(proc.registers.VA, 255);

        // Set I = 255
        proc.decode_and_execute();
        assert_eq!(proc.I, 255);

        // Dump registers VA-V8 to memory starting at 255 (0xff)
        proc.decode_and_execute();
        for i in 255..264 {
            let v = proc.memory.get_byte(i);
            let e = (i - 254) as u8;
            assert_eq!(v, e);
        }
        assert_eq!(proc.memory.get_byte(264), 0);

        // Set VE = 42
        proc.decode_and_execute();
        assert_eq!(proc.registers.VE, 42);

        // VE += VA (overflows)
        proc.decode_and_execute();
        assert_eq!(proc.registers.VA, 41);
        assert_eq!(proc.registers.VF, 1);

        // Call Subroutine at 0x400 (1024)
        proc.decode_and_execute();
        assert_eq!(proc.PC, 1024);
        assert_eq!(proc.SP, 1);
        assert_eq!(proc.stack[0], 1248);

        // Set V8 = 50
        proc.decode_and_execute();
        assert_eq!(proc.registers.V8, 50);

        // Set V9 = 42
        proc.decode_and_execute();
        assert_eq!(proc.registers.V9, 42);

        // V8 -= V9
        proc.decode_and_execute();
        assert_eq!(proc.registers.V8, 8);
        assert_eq!(proc.registers.VF, 0);


        // V8 -= V9 (again, overflows)
        proc.decode_and_execute();
        assert_eq!(proc.registers.V8, 222);
        assert_eq!(proc.registers.VF, 1);

        // return from subroutine
        proc.decode_and_execute();
        assert_eq!(proc.SP, 0);
        assert_eq!(proc.PC, proc.stack[proc.SP] as usize);

        let pre_skip_pc = proc.PC;
        // skip if VA == 41 (true)
        proc.decode_and_execute();
        assert_eq!(proc.PC, pre_skip_pc + 4);

    }
}