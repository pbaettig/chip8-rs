#[derive(Debug)]
pub struct Registers {
    pub V0: u8,
    pub V1: u8,
    pub V2: u8,
    pub V3: u8,
    pub V4: u8,
    pub V5: u8,
    pub V6: u8,
    pub V7: u8,
    pub V8: u8,
    pub V9: u8,
    pub VA: u8,
    pub VB: u8,
    pub VC: u8,
    pub VD: u8,
    pub VE: u8,
    pub VF: u8,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            V0: 0,
            V1: 0,
            V2: 0,
            V3: 0,
            V4: 0,
            V5: 0,
            V6: 0,
            V7: 0,
            V8: 0,
            V9: 0,
            VA: 0,
            VB: 0,
            VC: 0,
            VD: 0,
            VE: 0,
            VF: 0,
        }
    }

    pub fn from_array(vs: [u8; 16]) -> Self {
        Self {
            V0: vs[0],
            V1: vs[1],
            V2: vs[2],
            V3: vs[3],
            V4: vs[4],
            V5: vs[5],
            V6: vs[6],
            V7: vs[7],
            V8: vs[8],
            V9: vs[9],
            VA: vs[10],
            VB: vs[11],
            VC: vs[12],
            VD: vs[13],
            VE: vs[14],
            VF: vs[15],
        }
    }

    pub fn as_array(&self) -> [&u8; 16] {
        [
            &self.V0, &self.V1, &self.V2, &self.V3, &self.V4, &self.V5, &self.V6, &self.V7,
            &self.V8, &self.V9, &self.VA, &self.VB, &self.VC, &self.VD, &self.VE, &self.VF,
        ]
    }

    pub fn get(&self, index: u8) -> Option<u8> {
        match index {
            0 => Some(self.V0),
            1 => Some(self.V1),
            2 => Some(self.V2),
            3 => Some(self.V3),
            4 => Some(self.V4),
            5 => Some(self.V5),
            6 => Some(self.V6),
            7 => Some(self.V7),
            8 => Some(self.V8),
            9 => Some(self.V9),
            10 => Some(self.VA),
            11 => Some(self.VB),
            12 => Some(self.VC),
            13 => Some(self.VD),
            14 => Some(self.VE),
            15 => Some(self.VF),
            _ => None,
        }
    }

    pub fn set(&mut self, index: u8, value: u8) {
        match index {
            0 => self.V0 = value,
            1 => self.V1 = value,
            2 => self.V2 = value,
            3 => self.V3 = value,
            4 => self.V4 = value,
            5 => self.V5 = value,
            6 => self.V6 = value,
            7 => self.V7 = value,
            8 => self.V8 = value,
            9 => self.V9 = value,
            10 => self.VA = value,
            11 => self.VB = value,
            12 => self.VC = value,
            13 => self.VD = value,
            14 => self.VE = value,
            15 => self.VF = value,
            _ => return,
        }
    }
}
