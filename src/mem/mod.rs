
use std::fs::File;
use std::io::{self, ErrorKind, Read};
use std::path::Path;

pub struct Memory {
    pub mem: [u8; 4096],
}

impl Memory {
    pub fn new() -> Self {
        let mut m = Self { mem: [0; 4096] };
        m.load_fonts();

        m
    }

    pub fn load_fonts(&mut self) {
        self.mem[0x50..0x55].copy_from_slice(&[0xF0, 0x90, 0x90, 0x90, 0xF0]); // 0
        self.mem[0x55..0x5a].copy_from_slice(&[0x20, 0x60, 0x20, 0x20, 0x70]); // 1
        self.mem[0x5a..0x5f].copy_from_slice(&[0xF0, 0x10, 0xF0, 0x80, 0xF0]); // 2
        self.mem[0x5f..0x64].copy_from_slice(&[0xF0, 0x10, 0xF0, 0x10, 0xF0]); // 3
        self.mem[0x64..0x69].copy_from_slice(&[0x90, 0x90, 0xF0, 0x10, 0x10]); // 4
        self.mem[0x69..0x6e].copy_from_slice(&[0xF0, 0x80, 0xF0, 0x10, 0xF0]); // 5
        self.mem[0x6e..0x73].copy_from_slice(&[0xF0, 0x80, 0xF0, 0x90, 0xF0]); // 6
        self.mem[0x73..0x78].copy_from_slice(&[0xF0, 0x10, 0x20, 0x40, 0x40]); // 7
        self.mem[0x78..0x7d].copy_from_slice(&[0xF0, 0x90, 0xF0, 0x90, 0xF0]); // 8
        self.mem[0x7d..0x82].copy_from_slice(&[0xF0, 0x90, 0xF0, 0x10, 0xF0]); // 9
        self.mem[0x82..0x87].copy_from_slice(&[0xF0, 0x90, 0xF0, 0x90, 0x90]); // A
        self.mem[0x87..0x8c].copy_from_slice(&[0xE0, 0x90, 0xE0, 0x90, 0xE0]); // B
        self.mem[0x8c..0x91].copy_from_slice(&[0xF0, 0x80, 0x80, 0x80, 0xF0]); // C
        self.mem[0x91..0x96].copy_from_slice(&[0xE0, 0x90, 0x90, 0x90, 0xE0]); // D
        self.mem[0x96..0x9b].copy_from_slice(&[0xF0, 0x80, 0xF0, 0x80, 0xF0]); // E
        self.mem[0x9b..0xa0].copy_from_slice(&[0xF0, 0x80, 0xF0, 0x80, 0x80]); // F
    }

    fn load_rom(&mut self, p: &Path) -> io::Result<()> {
        let mut file = File::open(p)?;
        let mut opcode: [u8; 2] = [0; 2];
        let mut i: usize = 512;
        loop {
            match file
                .read(&mut opcode)
                .map_err(|e| std::io::Error::new(ErrorKind::Other, e))
            {
                Ok(0) => {
                    println!("Finished reading file");
                    return Ok(());
                }
                Ok(_) => {
                    self.mem[i] = opcode[0];
                    i += 1;
                    self.mem[i] = opcode[1];
                    i += 1;
                }
                Err(e) => return Err(e),
            }
        }
    }

    pub fn set_word(&mut self, index: usize, bs: [u8; 2]) {
        self.mem[index] = bs[0];
        self.mem[index + 1] = bs[1]
    }
    pub fn set_byte(&mut self, index: usize, b: u8) {
        self.mem[index] = b
    }

    pub fn get_bytes(&self, i: usize) -> [u8; 2] {
        let mut bs: [u8; 2] = [0; 2];
        bs[0] = self.mem[i];
        bs[1] = self.mem[i + 1];
        bs
    }
}
