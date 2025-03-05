use std::error::Error;
use std::{clone, fmt};

#[derive(Debug, Clone)]
pub struct UnknownOpcodeError {
    pub bytes: [u8; 2],
}
impl fmt::Display for UnknownOpcodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unknown opcode: {:#02x}{:02x}", self.bytes[0], self.bytes[1])
    }
}

impl Error for UnknownOpcodeError {}

fn combine_nibbles(n1: u8, n2: u8, n3: u8) -> u16 {
    (n1 as u16).wrapping_shl(8) + (n2 as u16).wrapping_shl(4) + (n3 as u16)
}

#[derive(Debug, Clone)]
pub enum Opcode {
    Call { addr: u16 },
    Display,
    Return,
    Goto { addr: u16 },
    CallSubroutine { addr: u16 },
    SkipIfRegisterEquals { register: u8, value: u8 },
    SkipIfRegisterNotEquals { register: u8, value: u8 },
    SkipIfRegistersEqual { register_1: u8, register_2: u8 },
    SetRegister { register: u8, value: u8 },
    AddToRegister { register: u8, value: u8 },
    CopyRegister { src_register: u8, dst_register: u8 },
    ApplyBitwiseOr { value_register: u8, operand_register: u8 },
    ApplyBitwiseAnd { value_register: u8, operand_register: u8 },
    ApplyBitwiseXor { value_register: u8, operand_register: u8 },
    AddRegisters { value_register: u8, operand_register: u8 },
    SubtractRegisters { value_register: u8, operand_register: u8 },
    SetI { addr: u16 },
    DumpRegisters { end_register: u8 },
}

impl Opcode {
    pub fn parse(opcode_bytes: [u8; 2]) -> Result<Self, UnknownOpcodeError> {
        let mut opcode_nibbles: [u8; 4] = [255; 4];
        opcode_nibbles[0] = (opcode_bytes[0] & 0xf0) >> 4;
        opcode_nibbles[1] = opcode_bytes[0] & 0x0f;

        opcode_nibbles[2] = (opcode_bytes[1] & 0xf0) >> 4;
        opcode_nibbles[3] = opcode_bytes[1] & 0x0f;

        let addr_12bit: u16 = combine_nibbles(opcode_nibbles[1], opcode_nibbles[2], opcode_nibbles[3]);

        // println!("{:#x} {:#x} | {:04b} {:04b} {:04b} ({:012b})", opcode_bytes[0], opcode_bytes[1], opcode_nibbles[1], opcode_nibbles[2], opcode_nibbles[3], addr_12bit);
        match opcode_nibbles {
            [0x0, 0x0, 0xe, 0x0] => Ok(Self::Display),
            [0x0, 0x0, 0xe, 0xe] => Ok(Self::Return),
            [0x0, _, _, _] => Ok(Self::Call { addr: addr_12bit }),
            [0x1, _, _, _] => Ok(Self::Goto { addr: addr_12bit }),
            [0x2, _, _, _] => Ok(Self::CallSubroutine { addr: addr_12bit }),
            [0x3, r, _, _] => Ok(Self::SkipIfRegisterEquals {
                register: r,
                value: opcode_bytes[1],
            }),
            [0x4, r, _, _] => Ok(Self::SkipIfRegisterNotEquals {
                register: r,
                value: opcode_bytes[1],
            }),
            [0x5, r1, r2, 0x0] => Ok(Self::SkipIfRegistersEqual {
                register_1: r1,
                register_2: r2,
            }),
            [0x6, r, _, _] => Ok(Self::SetRegister {
                register: r,
                value: opcode_bytes[1],
            }),
            [0x7, r, _, _] => Ok(Self::AddToRegister {
                register: r,
                value: opcode_bytes[1],
            }),
            [0x8, r_dst, r_src, 0x0] => Ok(Self::CopyRegister {
                src_register: r_src,
                dst_register: r_dst,
            }),
            [0x8, r_value, r_op, 0x1] => Ok(Self::ApplyBitwiseOr {
                value_register: r_value,
                operand_register: r_op,
            }),
            [0x8, r_value, r_op, 0x2] => Ok(Self::ApplyBitwiseAnd {
                value_register: r_value,
                operand_register: r_op,
            }),
            [0x8, r_value, r_op, 0x3] => Ok(Self::ApplyBitwiseXor {
                value_register: r_value,
                operand_register: r_op,
            }),
            [0x8, r_value, r_op, 0x4] => Ok(Self::AddRegisters {
                value_register: r_value,
                operand_register: r_op,
            }),
            [0x8, r_value, r_op, 0x5] => Ok(Self::SubtractRegisters {
                value_register: r_value,
                operand_register: r_op,
            }),
            [0xa, _, _, _] => Ok(Self::SetI { addr: addr_12bit }),
            [0xf, r_end, 0x5, 0x5] => Ok(Self::DumpRegisters { end_register: r_end }),
            _ => Err(UnknownOpcodeError { bytes: opcode_bytes }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_parse() {
        assert!(matches!(Opcode::parse([0xab, 0xcd]), Ok(Opcode::SetI { addr: 0xbcd })));
        assert!(matches!(
            Opcode::parse([0x62, 0xfe]),
            Ok(Opcode::SetRegister {
                register: 0x2,
                value: 0xfe
            })
        ));
    }
}
