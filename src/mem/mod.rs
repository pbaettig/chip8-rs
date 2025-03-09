#![allow(unused)]
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{self, ErrorKind, Read};
use std::path::Path;

static MAX_SIZE: usize = 4096;

#[derive(Debug, Clone)]
pub struct MemoryError (usize);

impl fmt::Display for MemoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "memory error")
    }
}

impl Error for MemoryError {}

pub struct MemorySlice {}

pub struct Memory {
    pub mem: [u8; MAX_SIZE],
}

impl fmt::Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let chunk = 32;
        let mut i = 0;
        while i < self.mem.len() {
            _ = write!(f, "{:#06x}:  ", i);
            for j in i..(i + chunk) {
                _ = write!(f, "{:02x} ", self.mem[j]);
            }
            i += chunk;
            _ = write!(f, "\n");
        }

        write!(f, "")
    }
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

    pub fn load_rom(&mut self, p: &Path) -> io::Result<()> {
        let mut file = File::open(p)?;
        let mut instruction: [u8; 2] = [0; 2];
        let mut i: usize = 512;
        loop {
            match file
                .read(&mut instruction)
                .map_err(|e| std::io::Error::new(ErrorKind::Other, e))
            {
                Ok(0) => {
                    return Ok(());
                }
                Ok(_) => {
                    self.mem[i] = instruction[0];
                    i += 1;
                    self.mem[i] = instruction[1];
                    i += 1;
                }
                Err(e) => return Err(e),
            }
        }
    }

    fn in_bounds(i: usize) -> Result<(), MemoryError> {
        if i > MAX_SIZE - 1 {
            Err(MemoryError(i))
        } else {
            Ok(())
        }
    }

    pub fn load_array(&mut self, loc: usize, b: &[u8]) -> Result<(), MemoryError> {
        let end = loc + b.len();
        Memory::in_bounds(end)?;
        self.mem[loc..end].copy_from_slice(b);
        Ok(())
    }

    pub fn set_word(&mut self, index: usize, bs: [u8; 2]) -> Result<(), MemoryError> {
        Memory::in_bounds(index + 1)?;

        self.mem[index] = bs[0];
        self.mem[index + 1] = bs[1];
        Ok(())
    }
    pub fn set_byte(&mut self, index: usize, b: u8) -> Result<(), MemoryError> {
        Memory::in_bounds(index)?;
        self.mem[index] = b;
        Ok(())
    }

    pub fn get_byte(&self, index: usize) -> Result<u8, MemoryError> {
        Memory::in_bounds(index)?;

        Ok(self.mem[index])
    }

    pub fn get_word(&self, index: usize) -> Result<[u8; 2], MemoryError> {
        Memory::in_bounds(index + 1)?;
        let mut bs: [u8; 2] = [0; 2];
        bs[0] = self.mem[index];
        bs[1] = self.mem[index + 1];
        Ok(bs)
    }
}

mod tests {
    use super::{Memory, MAX_SIZE, MemoryError};

    #[test]
    fn test_get_set() {
        let mut mem = Memory::new();

        for addr in 0..MAX_SIZE {
            let v = (addr % 255) as u8;
            assert!(matches!(mem.set_byte(addr, v), Ok(())));
            assert_eq!(mem.get_byte(addr).unwrap(), v);
        }

        for addr in (0..MAX_SIZE - 2).step_by(2) {
            let v = (addr % 255) as u8;
            let bs: [u8; 2] = [0, v];
            assert!(matches!(mem.set_word(addr, bs), Ok(())));
            assert_eq!(mem.get_word(addr).unwrap(), [0, v]);
        }

        assert!(matches!(mem.get_byte(5123), Err(MemoryError(5123))));
        assert!(matches!(mem.set_byte(4097, 42), Err(MemoryError(4097))));
        assert!(matches!(mem.get_word(4095), Err(MemoryError(4096))));
        assert!(matches!(mem.set_word(4095, [0, 1]), Err(MemoryError(4096))));
    }
}
