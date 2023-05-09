use std::error::Error;
use std::fmt::{Display, Formatter};

///
/// The RAM_SIZE constant is the size of the RAM in bytes for the CHIP-8.
///
const RAM_SIZE: usize = 4_096;

///
/// The RESERVED_SIZE constant is the size of the reserved memory in the RAM for sprites.
///
const RESERVED_SIZE: usize = 80;

///
/// The DEFAULT_PROGRAM_START_OFFSET constant is the default offset for the start of the program in the RAM.
///
const DEFAULT_PROGRAM_START_OFFSET: usize = 0x200usize;

///
/// The RamError enum represents the possible errors that can occur when loading a program into the RAM.
///
#[derive(Debug)]
pub enum RamError {
    NotEnoughSpace,
    OutOfBound,
}

impl Error for RamError {}

impl Display for RamError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            RamError::NotEnoughSpace => write!(f, "Not enough space to load program!"),
            RamError::OutOfBound => write!(f, "Out of bound memory!"),
        }
    }
}

///
/// The Ram struct represents the RAM of the CHIP-8.
///
pub struct Ram {
    data: [u8; RAM_SIZE],
}

impl Ram {
    pub fn new() -> Self {
        let mut ram = Self {
            data: [0; RAM_SIZE],
        };

        let sprites: [u8; RESERVED_SIZE] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];

        for (i, byte) in sprites.iter().enumerate() {
            ram.data[i] = *byte;
        }

        ram
    }

    #[allow(dead_code)]
    pub fn load(&mut self, data: &[u8]) -> Result<(), RamError> {
        if data.len() <= RAM_SIZE - DEFAULT_PROGRAM_START_OFFSET {
            for (i, byte) in data.iter().enumerate() {
                self.data[i + DEFAULT_PROGRAM_START_OFFSET] = *byte;
            }
            Ok(())
        } else {
            Err(RamError::NotEnoughSpace)
        }
    }

    #[allow(dead_code)]
    pub fn read_byte(&self, address: usize) -> Result<u8, RamError> {
        if address < RAM_SIZE {
            Ok(self.data[address])
        } else {
            Err(RamError::OutOfBound)
        }
    }

    #[allow(dead_code)]
    pub fn write_byte(&mut self, address: usize, value: u8) -> Result<(), RamError> {
        if address < RAM_SIZE {
            self.data[address] = value;
            Ok(())
        } else {
            Err(RamError::OutOfBound)
        }
    }

    #[allow(dead_code)]
    pub fn read_word(&self, address: usize) -> Result<u16, RamError> {
        if address < RAM_SIZE - 1 {
            Ok((self.data[address] as u16) << 8 | self.data[address + 1] as u16)
        } else {
            Err(RamError::OutOfBound)
        }
    }

    #[allow(dead_code)]
    pub fn write_word(&mut self, address: usize, value: u16) -> Result<(), RamError> {
        if address < RAM_SIZE - 1 {
            self.data[address] = (value >> 8) as u8;
            self.data[address + 1] = value as u8;
            Ok(())
        } else {
            Err(RamError::OutOfBound)
        }
    }
}

mod ram_tests {
    #[test]
    fn read_byte() {
        let ram = super::Ram::new();
        assert_eq!(ram.read_byte(0usize).unwrap(), 0xF0);
    }

    #[test]
    fn write_byte() {
        let mut ram = super::Ram::new();
        ram.write_byte(0usize, 1).unwrap();
        assert_eq!(ram.read_byte(0usize).unwrap(), 1);
    }

    #[test]
    fn read_word() {
        let ram = super::Ram::new();
        assert_eq!(ram.read_word(0usize).unwrap(), 0xF090);
    }

    #[test]
    fn write_word() {
        let mut ram = super::Ram::new();
        ram.write_word(0usize, 1024u16).unwrap();
        assert_eq!(ram.read_word(0usize).unwrap(), 1024u16);
    }
}
