fn combine_nibbles(n1: u8, n2: u8, n3: u8) -> u16 {
    (n1 as u16).wrapping_shl(8) + (n2 as u16).wrapping_shl(4) + (n3 as u16)
}


#[derive(Debug)]
pub enum Opcode {
    Call {
        addr: u16,
    },
    Display,
    Return,
    Goto {
        addr: u16,
    },
    CallSubroutine {
        addr: u16,
    },
    SkipIfRegisterEquals {
        register: u8,
        value: u8,
    },
    SkipIfRegisterNotEquals {
        register: u8,
        value: u8,
    },
    SkipIfRegistersEqual {
        register_1: u8,
        register_2: u8,
    },
    SetRegister {
        register: u8,
        value: u8,
    },
    AddToRegister {
        register: u8,
        value: u8,
    },
    CopyRegister {
        src_register: u8,
        dst_register: u8,
    },
    ApplyBitwiseOr {
        value_register: u8,
        operand_register: u8,
    },
    ApplyBitwiseAnd {
        value_register: u8,
        operand_register: u8,
    },
    ApplyBitwiseXor {
        value_register: u8,
        operand_register: u8,
    },
    AddRegisters {
        value_register: u8,
        operand_register: u8,
    },
    SubtractRegisters {
        value_register: u8,
        operand_register: u8,
    },
    SetI {
        addr: u16,
    },
    DumpRegisters {
        end_register: u8,
    },
}

impl Opcode {
    pub fn parse(opcode_bytes: [u8; 2]) -> Option<Self> {
        let mut opcode_nibbles: [u8; 4] = [255; 4];
        opcode_nibbles[0] = (opcode_bytes[0] & 0xf0) >> 4;
        opcode_nibbles[1] = opcode_bytes[0] & 0x0f;

        opcode_nibbles[2] = (opcode_bytes[1] & 0xf0) >> 4;
        opcode_nibbles[3] = opcode_bytes[1] & 0x0f;

        let addr_12bit: u16 = combine_nibbles(opcode_nibbles[1], opcode_nibbles[2], opcode_nibbles[3]);
        
       
        // println!("{:#x} {:#x} | {:04b} {:04b} {:04b} ({:012b})", opcode_bytes[0], opcode_bytes[1], opcode_nibbles[1], opcode_nibbles[2], opcode_nibbles[3], addr_12bit);
        match opcode_nibbles {
            [0x0, 0x0, 0xe, 0x0] => Some(Self::Display),
            [0x0, 0x0, 0xe, 0xe] => Some(Self::Return),
            [0x0, _, _, _] => Some(Self::Call { addr: addr_12bit }),
            [0x1, _, _, _] => Some(Self::Goto { addr: addr_12bit }),
            [0x2, _, _, _] => Some(Self::CallSubroutine { addr: addr_12bit }),
            [0x3, r, _, _] => Some(Self::SkipIfRegisterEquals {
                register: r,
                value: opcode_bytes[1],
            }),
            [0x4, r, _, _] => Some(Self::SkipIfRegisterNotEquals {
                register: r,
                value: opcode_bytes[1],
            }),
            [0x5, r1, r2, 0x0] => Some(Self::SkipIfRegistersEqual {
                register_1: r1,
                register_2: r2,
            }),
            [0x6, r, _, _] => Some(Self::SetRegister {
                register: r,
                value: opcode_bytes[1],
            }),
            [0x7, r, _, _] => Some(Self::AddToRegister {
                register: r,
                value: opcode_bytes[1],
            }),
            [0x8, r_dst, r_src, 0x0] => Some(Self::CopyRegister {
                src_register: r_src,
                dst_register: r_dst,
            }),
            [0x8, r_value, r_op, 0x1] => Some(Self::ApplyBitwiseOr {
                value_register: r_value,
                operand_register: r_op,
            }),
            [0x8, r_value, r_op, 0x2] => Some(Self::ApplyBitwiseAnd {
                value_register: r_value,
                operand_register: r_op,
            }),
            [0x8, r_value, r_op, 0x3] => Some(Self::ApplyBitwiseXor {
                value_register: r_value,
                operand_register: r_op,
            }),
            [0x8, r_value, r_op, 0x4] => Some(Self::AddRegisters {
                value_register: r_value,
                operand_register: r_op,
            }),
            [0x8, r_value, r_op, 0x5] => Some(Self::SubtractRegisters {
                value_register: r_value,
                operand_register: r_op,
            }),
            [0xa, _, _, _] => Some(Self::SetI { addr: addr_12bit }),
            [0xf, r_end, 0x5, 0x5] => Some(Self::DumpRegisters {
                end_register: r_end,
            }),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_parse() {
        assert!(matches!(Opcode::parse([0xab, 0xcd]), Some(Opcode::SetI { addr: 0xbcd })));
        assert!(matches!(Opcode::parse([0x62, 0xfe]), Some(Opcode::SetRegister {register: 0x2, value: 0xfe})));
    }
}