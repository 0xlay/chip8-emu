use std::collections::LinkedList;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufReader, Read};
use std::{thread, time};

use crate::display::Display;
use crate::instructions::Instructions;
use crate::keyboard::Keyboard;
use crate::ram::{Ram, RamError};
use crate::registers::Registers;

/// Emulate the COSMAC VIP CPU's frequency
const MAX_INSTRUCTION_PER_SECOND: u64 = 890;

///
/// The RamError enum represents the possible errors that can occur when running the CHIP-8 emulator.
///
#[derive(Debug)]
pub enum Chip8Error {
    FailedToDecodeOpcode,
    FailedToExecuteInstruction,
}

impl Error for Chip8Error {}

impl fmt::Display for Chip8Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Chip8Error::FailedToDecodeOpcode => write!(f, "Failed to decode opcode!"),
            Chip8Error::FailedToExecuteInstruction => write!(f, "Failed to execute instruction!"),
        }
    }
}

pub struct Chip8 {
    display: Display,
    keyboard: Keyboard,
    ram: Ram,
    stack: LinkedList<u16>,
    registers: Registers,
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            display: Display::new(),
            keyboard: Keyboard::new(),
            ram: Ram::new(),
            stack: LinkedList::new(),
            registers: Registers::new(),
        }
    }

    pub fn load_rom(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(path)?;

        let mut buf = Vec::new();
        let mut reader = BufReader::new(file);

        reader.read_to_end(&mut buf)?;
        self.ram.load(buf.as_slice())?;
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut num_of_instruction = 0;
        let mut end_time = time::Instant::now() + time::Duration::from_secs(1);
        loop {
            let opcode = self.fetch()?;
            let instruction = self.decode(opcode)?;
            self.execute(instruction, opcode)?;
            self.emulate_speed(&mut num_of_instruction, &mut end_time);
        }
    }

    fn fetch(&mut self) -> Result<u16, RamError> {
        let opcode = self.ram.read_word(self.registers.PC as usize)?;
        self.registers.PC += 2;
        Ok(opcode)
    }

    fn decode(&self, opcode: u16) -> Result<Instructions, Chip8Error> {
        if (opcode & 0x0111u16) == 1 {
            Ok(Instructions::SYS)
        } else if opcode == 0x00E0 {
            Ok(Instructions::CLS)
        } else if opcode == 0x00EE {
            Ok(Instructions::RET)
        } else {
            Err(Chip8Error::FailedToDecodeOpcode)
        }
    }

    fn execute(&mut self, instruction: Instructions, opcode: u16) -> Result<(), Chip8Error> {
        match instruction {
            Instructions::SYS => unimplemented!(),
            Instructions::CLS => unimplemented!(),
            Instructions::RET => unimplemented!(),
            _ => Err(Chip8Error::FailedToExecuteInstruction),
        }
    }

    fn emulate_speed(&mut self, num_of_instruction: &mut u64, end_time: &mut time::Instant) {
        *num_of_instruction += 1;
        if *num_of_instruction == MAX_INSTRUCTION_PER_SECOND {
            thread::sleep(*end_time - time::Instant::now());
            *end_time = time::Instant::now() + time::Duration::from_secs(1);
            *num_of_instruction = 0;
        }
    }
}
