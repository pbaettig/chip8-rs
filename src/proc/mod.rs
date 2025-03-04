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
        let bs = self.memory.get_bytes(self.PC);
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
        println!("(PC:{}, SP:{}) | {op:?}", self.PC, self.SP);
        match op {
            ops::Opcode::CallSubroutine { addr } => {
                self.push_stack(self.PC as u16);
                self.PC = addr as usize;
            }
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
                if of {
                    self.registers.VF = 1;
                }
                self.registers.set(value_register, v);
            }
            ops::Opcode::SetI { addr } => {
                self.I = addr;
            }
            ops::Opcode::DumpRegisters { end_register } => {
                for r_v in self.registers.as_array()[0..(end_register + 1) as usize].iter() {
                    println!("set mem[{}] -> {}", self.I, **r_v);
                    self.memory.set_byte(self.I as usize, **r_v);
                    self.I += 1;
                }
            }
            _ => return,
        }
    }
}
