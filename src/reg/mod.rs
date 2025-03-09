#![allow(dead_code)]

use std::error::Error;
use std::fmt;

pub const V0: u8 = 0;
pub const V1: u8 = 1;
pub const V2: u8 = 2;
pub const V3: u8 = 3;
pub const V4: u8 = 4;
pub const V5: u8 = 5;
pub const V6: u8 = 6;
pub const V7: u8 = 7;
pub const V8: u8 = 8;
pub const V9: u8 = 9;
pub const VA: u8 = 10;
pub const VB: u8 = 11;
pub const VC: u8 = 12;
pub const VD: u8 = 13;
pub const VE: u8 = 14;
pub const VF: u8 = 15;

#[derive(Debug, Clone)]
pub struct RegisterError(u8);

impl fmt::Display for RegisterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "register error")
    }
}

impl Error for RegisterError {}

#[derive(Debug)]
pub struct Registers {
    pub v0: u8,
    pub v1: u8,
    pub v2: u8,
    pub v3: u8,
    pub v4: u8,
    pub v5: u8,
    pub v6: u8,
    pub v7: u8,
    pub v8: u8,
    pub v9: u8,
    pub va: u8,
    pub vb: u8,
    pub vc: u8,
    pub vd: u8,
    pub ve: u8,
    pub vf: u8,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            v0: 0,
            v1: 0,
            v2: 0,
            v3: 0,
            v4: 0,
            v5: 0,
            v6: 0,
            v7: 0,
            v8: 0,
            v9: 0,
            va: 0,
            vb: 0,
            vc: 0,
            vd: 0,
            ve: 0,
            vf: 0,
        }
    }

    pub fn from_array(&mut self, vs: &[u8; 16]) {
        self.v0 = vs[0];
        self.v1 = vs[1];
        self.v2 = vs[2];
        self.v3 = vs[3];
        self.v4 = vs[4];
        self.v5 = vs[5];
        self.v6 = vs[6];
        self.v7 = vs[7];
        self.v8 = vs[8];
        self.v9 = vs[9];
        self.va = vs[10];
        self.vb = vs[11];
        self.vc = vs[12];
        self.vd = vs[13];
        self.ve = vs[14];
        self.vf = vs[15];
    }

    pub fn as_array(&self) -> [u8; 16] {
        [
            self.v0, self.v1, self.v2, self.v3, self.v4, self.v5, self.v6, self.v7, self.v8, self.v9, self.va, self.vb,
            self.vc, self.vd, self.ve, self.vf,
        ]
    }

    pub fn get(&self, index: u8) -> Result<u8, RegisterError> {
        match index {
            0 => Ok(self.v0),
            1 => Ok(self.v1),
            2 => Ok(self.v2),
            3 => Ok(self.v3),
            4 => Ok(self.v4),
            5 => Ok(self.v5),
            6 => Ok(self.v6),
            7 => Ok(self.v7),
            8 => Ok(self.v8),
            9 => Ok(self.v9),
            10 => Ok(self.va),
            11 => Ok(self.vb),
            12 => Ok(self.vc),
            13 => Ok(self.vd),
            14 => Ok(self.ve),
            15 => Ok(self.vf),
            _ => Err(RegisterError(index)),
        }
    }

    pub fn set(&mut self, index: u8, value: u8) -> Result<(), RegisterError> {
        match index {
            0 => self.v0 = value,
            1 => self.v1 = value,
            2 => self.v2 = value,
            3 => self.v3 = value,
            4 => self.v4 = value,
            5 => self.v5 = value,
            6 => self.v6 = value,
            7 => self.v7 = value,
            8 => self.v8 = value,
            9 => self.v9 = value,
            10 => self.va = value,
            11 => self.vb = value,
            12 => self.vc = value,
            13 => self.vd = value,
            14 => self.ve = value,
            15 => self.vf = value,
            _ => return Err(RegisterError(index)),
        };
        Ok(())
    }
}

mod tests {
    #[test]
    fn test_from_as_array() {
        let mut reg = super::Registers::new();
        reg.from_array(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
        assert_eq!(reg.v0, 1);
        assert_eq!(reg.v1, 2);
        assert_eq!(reg.v2, 3);
        assert_eq!(reg.v3, 4);
        assert_eq!(reg.v4, 5);
        assert_eq!(reg.v5, 6);
        assert_eq!(reg.v6, 7);
        assert_eq!(reg.v7, 8);
        assert_eq!(reg.v8, 9);
        assert_eq!(reg.v9, 10);
        assert_eq!(reg.va, 11);
        assert_eq!(reg.vb, 12);
        assert_eq!(reg.vc, 13);
        assert_eq!(reg.vd, 14);
        assert_eq!(reg.ve, 15);
        assert_eq!(reg.vf, 16);

        assert_eq!(reg.as_array(), [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    }

    #[test]
    fn test_get_set() {
        let mut reg = super::Registers::new();
        for i in 0..15 {
            let r = reg.set(i, i + 1);
            assert!(matches!(r, Ok(())));
        }
        for i in 0..15 {
            let r = reg.get(i);
            assert!(matches!(r, Ok(_)));
            assert_eq!(r.unwrap(), i + 1);
        }
    }
}
