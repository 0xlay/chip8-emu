use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufReader, Read};
use std::{thread, time};

use rand::{rngs, Rng};
use sdl2::{event::Event, keyboard::Keycode};

use super::io::{Display, Keyboard, GRID_HEIGHT, GRID_WIDTH};
use super::memory::{Ram, Registers};

///
/// The `WORD_SIZE` constant is the chip8's word size.
///
pub const WORD_SIZE: u16 = 2;

///
/// The `INSTRUCTIONS_PER_SECOND` value need for emulate the COSMAC VIP CPU's frequency.
///
const INSTRUCTIONS_PER_SECOND: u32 = 450;

///
/// The `Chip8Error` enum represents the possible errors that can occur when running the CHIP-8 emulator.
///
#[derive(Debug)]
pub enum Chip8Error {
    FailedToDecodeOpcode,
}

impl Error for Chip8Error {}

impl fmt::Display for Chip8Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::FailedToDecodeOpcode => write!(f, "Failed to decode opcode!"),
        }
    }
}

///
/// The `Instruction` enum represents the set of instructions supported by the Chip8 emulator.
///
#[allow(clippy::upper_case_acronyms)]
#[derive(PartialEq, Eq)]
pub enum Instruction {
    CLS,
    RET,
    JMP,
    JMPV0,
    CALL,
    LD,
    LDR,
    LDRI,
    LDRDT,
    LDDTR,
    LDRST,
    LDK,
    LDSR,
    LDB,
    LDRIR,
    LDRRI,
    SE,
    SER,
    SNE,
    SNER,
    ADD,
    ADDR,
    ADDRI,
    SUB,
    SUBN,
    AND,
    OR,
    XOR,
    SHR,
    SHL,
    RND,
    DRW,
    SKP,
    SKNP,
}

///
/// The `Chip8` structure represents the interface for using the chip8 emulator.
///
pub struct Chip8 {
    display: Display,
    keyboard: Keyboard,
    ram: Ram,
    registers: Registers,
    rnd_engine: rngs::ThreadRng,
    delay_timer: time::Instant,
}

impl Chip8 {
    pub fn new(window_width: u32, window_height: u32) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            display: Display::new(window_width, window_height)?,
            keyboard: Keyboard::new(),
            ram: Ram::new(),
            registers: Registers::new(),
            rnd_engine: rand::thread_rng(),
            delay_timer: time::Instant::now(),
        })
    }

    pub fn load_rom(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        let file = File::open(path)?;
        let mut buf = Vec::new();
        BufReader::new(file).read_to_end(&mut buf)?;
        self.ram.load(buf.as_slice())?;
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let mut event_pump = self.display.get_event_pump()?;
        'exit_from_loop: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'exit_from_loop,
                    Event::KeyDown { keycode, .. } => {
                        if let Some(key) = keycode {
                            self.keyboard.press_key(key);
                        } else {
                            self.keyboard.release_key();
                        }
                    }
                    _ => {}
                }
            }

            let opcode = self.fetch()?;
            let instruction = Self::decode(opcode)?;

            self.execute(&instruction, opcode)?;
            Self::emulate_speed();
        }

        Ok(())
    }

    fn fetch(&mut self) -> Result<u16, Box<dyn Error>> {
        let opcode = self.ram.read_word(self.registers.pc as usize)?;
        Ok(opcode)
    }

    fn decode(opcode: u16) -> Result<Instruction, Box<dyn Error>> {
        if opcode == 0x00E0 {
            Ok(Instruction::CLS)
        } else if opcode == 0x00EE {
            Ok(Instruction::RET)
        } else if (opcode & 0xF000) == 0x1000 {
            Ok(Instruction::JMP)
        } else if (opcode & 0xF000) == 0x2000 {
            Ok(Instruction::CALL)
        } else if (opcode & 0xF000) == 0x3000 {
            Ok(Instruction::SE)
        } else if (opcode & 0xF000) == 0x4000 {
            Ok(Instruction::SNE)
        } else if (opcode & 0xF000) == 0x5000 {
            Ok(Instruction::SER)
        } else if (opcode & 0xF000) == 0x6000 {
            Ok(Instruction::LD)
        } else if (opcode & 0xF000) == 0x7000 {
            Ok(Instruction::ADD)
        } else if (opcode & 0xF000) == 0x8000 {
            match (opcode & 0x000F) as u8 {
                0x0 => Ok(Instruction::LDR),
                0x1 => Ok(Instruction::OR),
                0x2 => Ok(Instruction::AND),
                0x3 => Ok(Instruction::XOR),
                0x4 => Ok(Instruction::ADDR),
                0x5 => Ok(Instruction::SUB),
                0x6 => Ok(Instruction::SHR),
                0x7 => Ok(Instruction::SUBN),
                0xE => Ok(Instruction::SHL),
                _ => Err(Chip8Error::FailedToDecodeOpcode.into()),
            }
        } else if (opcode & 0xF000) == 0x9000 {
            Ok(Instruction::SNER)
        } else if (opcode & 0xF000) == 0xA000 {
            Ok(Instruction::LDRI)
        } else if (opcode & 0xF000) == 0xB000 {
            Ok(Instruction::JMPV0)
        } else if (opcode & 0xF000) == 0xC000 {
            Ok(Instruction::RND)
        } else if (opcode & 0xF000) == 0xD000 {
            Ok(Instruction::DRW)
        } else if (opcode & 0xF000) == 0xE000 {
            match (opcode & 0x00FF) as u8 {
                0x9E => Ok(Instruction::SKP),
                0xA1 => Ok(Instruction::SKNP),
                _ => Err(Chip8Error::FailedToDecodeOpcode.into()),
            }
        } else if (opcode & 0xF000) == 0xF000 {
            match (opcode & 0x00FF) as u8 {
                0x07 => Ok(Instruction::LDRDT),
                0x0A => Ok(Instruction::LDK),
                0x15 => Ok(Instruction::LDDTR),
                0x18 => Ok(Instruction::LDRST),
                0x1E => Ok(Instruction::ADDRI),
                0x29 => Ok(Instruction::LDSR),
                0x33 => Ok(Instruction::LDB),
                0x55 => Ok(Instruction::LDRIR),
                0x65 => Ok(Instruction::LDRRI),
                _ => Err(Chip8Error::FailedToDecodeOpcode.into()),
            }
        } else {
            Err(Chip8Error::FailedToDecodeOpcode.into())
        }
    }

    fn execute(&mut self, instruction: &Instruction, opcode: u16) -> Result<(), Box<dyn Error>> {
        match instruction {
            Instruction::CLS => {
                self.cls();
                Ok(())
            }
            Instruction::RET => {
                self.ret();
                Ok(())
            }
            Instruction::JMP => {
                self.jmp(opcode);
                Ok(())
            }
            Instruction::CALL => {
                self.call(opcode);
                Ok(())
            }
            Instruction::SE => {
                self.se(opcode);
                Ok(())
            }
            Instruction::SNE => {
                self.sne(opcode);
                Ok(())
            }
            Instruction::SER => {
                self.ser(opcode);
                Ok(())
            }
            Instruction::LD => {
                self.ld(opcode);
                Ok(())
            }
            Instruction::ADD => {
                self.add(opcode);
                Ok(())
            }
            Instruction::LDR => {
                self.ldr(opcode);
                Ok(())
            }
            Instruction::OR => {
                self.or(opcode);
                Ok(())
            }
            Instruction::AND => {
                self.and(opcode);
                Ok(())
            }
            Instruction::XOR => {
                self.xor(opcode);
                Ok(())
            }
            Instruction::ADDR => {
                self.addr(opcode);
                Ok(())
            }
            Instruction::SUB => {
                self.sub(opcode);
                Ok(())
            }
            Instruction::SHR => {
                self.shr(opcode);
                Ok(())
            }
            Instruction::SUBN => {
                self.subn(opcode);
                Ok(())
            }
            Instruction::SHL => {
                self.shl(opcode);
                Ok(())
            }
            Instruction::SNER => {
                self.sner(opcode);
                Ok(())
            }
            Instruction::LDRI => {
                self.ldri(opcode);
                Ok(())
            }
            Instruction::JMPV0 => {
                self.jmpv0(opcode);
                Ok(())
            }
            Instruction::RND => {
                self.rnd(opcode);
                Ok(())
            }
            Instruction::DRW => self.drw(opcode),
            Instruction::SKP => {
                self.skp(opcode);
                Ok(())
            }
            Instruction::SKNP => {
                self.sknp(opcode);
                Ok(())
            }
            Instruction::LDRDT => {
                self.ldrdt(opcode);
                Ok(())
            }
            Instruction::LDK => {
                self.ldk(opcode);
                Ok(())
            }
            Instruction::LDDTR => {
                self.lddtr(opcode);
                Ok(())
            }
            Instruction::LDRST => {
                self.ldrst(opcode);
                Ok(())
            }
            Instruction::ADDRI => {
                self.addri(opcode);
                Ok(())
            }
            Instruction::LDSR => {
                self.ldsr(opcode);
                Ok(())
            }
            Instruction::LDB => self.ldb(opcode),
            Instruction::LDRIR => self.ldrir(opcode),
            Instruction::LDRRI => self.ldrri(opcode),
        }
    }

    fn emulate_speed() {
        thread::sleep(time::Duration::from_secs(1) / INSTRUCTIONS_PER_SECOND);
    }

    fn cls(&mut self) {
        self.display.clear();
        self.registers.pc += WORD_SIZE;
    }

    fn ret(&mut self) {
        self.registers.pc = self.registers.sp.pop().unwrap();
    }

    fn jmp(&mut self, opcode: u16) {
        self.registers.pc = opcode & 0x0FFF;
    }

    fn call(&mut self, opcode: u16) {
        self.registers.sp.push(self.registers.pc + WORD_SIZE);
        self.registers.pc = opcode & 0x0FFF;
    }

    fn se(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let val = (opcode & 0x00FF) as u8;

        if self.registers.v[x] == val {
            self.registers.pc += WORD_SIZE * 2;
        } else {
            self.registers.pc += WORD_SIZE;
        }
    }

    fn sne(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let val = (opcode & 0x00FF) as u8;

        if self.registers.v[x] != val {
            self.registers.pc += WORD_SIZE * 2;
        } else {
            self.registers.pc += WORD_SIZE;
        }
    }

    fn ser(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;

        if self.registers.v[x] == self.registers.v[y] {
            self.registers.pc += WORD_SIZE * 2;
        } else {
            self.registers.pc += WORD_SIZE;
        }
    }

    fn ld(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;

        self.registers.v[x] = (opcode & 0x00FF) as u8;

        self.registers.pc += WORD_SIZE;
    }

    fn add(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let val = (opcode & 0x00FF) as u8;

        self.registers.v[x] = self.registers.v[x].wrapping_add(val);

        self.registers.pc += WORD_SIZE;
    }

    fn ldr(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;

        self.registers.v[x] = self.registers.v[y];

        self.registers.pc += WORD_SIZE;
    }

    fn or(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;

        self.registers.v[x] |= self.registers.v[y];

        self.registers.pc += WORD_SIZE;
    }

    fn and(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;

        self.registers.v[x] &= self.registers.v[y];

        self.registers.pc += WORD_SIZE;
    }

    fn xor(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;

        self.registers.v[x] ^= self.registers.v[y];

        self.registers.pc += WORD_SIZE;
    }

    fn addr(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;

        let sum = self.registers.v[x] as u16 + self.registers.v[y] as u16;
        self.registers.v[x] = sum as u8;

        if sum > 0xFF {
            self.registers.v[0xF] = 1;
        } else {
            self.registers.v[0xF] = 0;
        }

        self.registers.pc += WORD_SIZE;
    }

    fn sub(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;

        let diff = self.registers.v[x].wrapping_sub(self.registers.v[y]) as i8;
        self.registers.v[x] = diff as u8;

        if diff < 0 {
            self.registers.v[0xF] = 1;
        } else {
            self.registers.v[0xF] = 0;
        }

        self.registers.pc += WORD_SIZE;
    }

    fn shr(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;

        self.registers.v[0xF] = self.registers.v[x] & 0x1;
        self.registers.v[x] >>= 1;

        self.registers.pc += WORD_SIZE;
    }

    fn subn(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;

        let diff = self.registers.v[y] as i8 - self.registers.v[x] as i8;
        self.registers.v[x] = diff as u8;

        if diff < 0 {
            self.registers.v[0xF] = 1;
        } else {
            self.registers.v[0xF] = 0;
        }

        self.registers.pc += WORD_SIZE;
    }

    fn shl(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;

        self.registers.v[0xF] = (self.registers.v[x] & 0x80) >> 7;
        self.registers.v[x] <<= 1;

        self.registers.pc += WORD_SIZE;
    }

    fn sner(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;

        if self.registers.v[x] != self.registers.v[y] {
            self.registers.pc += WORD_SIZE * 2;
        } else {
            self.registers.pc += WORD_SIZE;
        }
    }

    fn ldri(&mut self, opcode: u16) {
        self.registers.i = opcode & 0x0FFF;
        self.registers.pc += WORD_SIZE;
    }

    fn jmpv0(&mut self, opcode: u16) {
        self.registers.pc = self.registers.v[0] as u16 + (opcode & 0x0FFF);
    }

    fn rnd(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let val = (opcode & 0x0FF) as u8;

        let num = self.rnd_engine.gen_range(0..0xFF) as u8;
        self.registers.v[x] = num & val;

        self.registers.pc += WORD_SIZE;
    }

    fn drw(&mut self, opcode: u16) -> Result<(), Box<dyn Error>> {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let n = (opcode & 0x000F) as usize;

        let x_pos = self.registers.v[x] as usize;
        let y_pos = self.registers.v[y] as usize;

        self.registers.v[0xF] = 0;

        for byte in 0..n {
            let sprite_byte = self.ram.read_byte(self.registers.i as usize + byte)?;

            for bit in 0..8usize {
                let sprite_pixel = (sprite_byte >> (7 - bit)) & 1;
                let screen_x = (x_pos + bit) % GRID_WIDTH;
                let screen_y = (y_pos + byte) % GRID_HEIGHT;

                let screen_pixel = self.display.get_pixel(screen_x, screen_y);

                // XOR sprite pixel and screen pixel, then update the display
                let new_pixel = sprite_pixel ^ screen_pixel;
                self.display.set_pixel(screen_x, screen_y, new_pixel);

                // If screen pixel was on and now is off, set VF to 1
                if sprite_pixel == 1 && screen_pixel == 1 {
                    self.registers.v[0xF] = 1;
                }
            }
        }

        self.display.draw();
        self.registers.pc += WORD_SIZE;
        Ok(())
    }

    fn skp(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let key = self.registers.v[x];

        if self.keyboard.is_key_pressed(key) {
            self.keyboard.release_key();
            self.registers.pc += WORD_SIZE * 2;
        } else {
            self.registers.pc += WORD_SIZE;
        }
    }

    fn sknp(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let key = self.registers.v[x];

        if !self.keyboard.is_key_pressed(key) {
            self.registers.pc += WORD_SIZE * 2;
        } else {
            self.keyboard.release_key();
            self.registers.pc += WORD_SIZE;
        }
    }

    fn ldrdt(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;

        self.registers.v[x] = self.get_delay_timer();
        self.registers.pc += WORD_SIZE;
    }

    fn ldk(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;

        if let Some(val) = self.keyboard.get_pressed_key() {
            self.registers.v[x] = val;
        }

        self.registers.pc += WORD_SIZE;
    }

    fn lddtr(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;

        self.set_delay_timer(self.registers.v[x]);

        self.registers.pc += WORD_SIZE;
    }

    fn ldrst(&mut self, _opcode: u16) {
        self.registers.pc += WORD_SIZE;
    }

    fn addri(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;

        self.registers.i += self.registers.v[x] as u16;

        self.registers.pc += WORD_SIZE;
    }

    fn ldsr(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;

        // 5 because each sprite has 5 lines
        self.registers.i = self.registers.v[x] as u16 * 5;

        self.registers.pc += WORD_SIZE;
    }

    fn ldb(&mut self, opcode: u16) -> Result<(), Box<dyn Error>> {
        let x = ((opcode & 0x0F00) >> 8) as usize;

        self.ram
            .write_byte(self.registers.i as usize, self.registers.v[x] / 100)?;
        self.ram
            .write_byte(self.registers.i as usize + 1, self.registers.v[x] % 100)?;
        self.ram
            .write_byte(self.registers.i as usize + 2, self.registers.v[x] % 10)?;

        self.registers.pc += WORD_SIZE;
        Ok(())
    }

    fn ldrir(&mut self, opcode: u16) -> Result<(), Box<dyn Error>> {
        let x = ((opcode & 0x0F00) >> 8) as usize;

        for i in 0..=x {
            self.ram
                .write_byte(self.registers.i as usize + i, self.registers.v[i])?;
        }

        self.registers.i += x as u16 + 1;
        self.registers.pc += WORD_SIZE;
        Ok(())
    }

    fn ldrri(&mut self, opcode: u16) -> Result<(), Box<dyn Error>> {
        let x = ((opcode & 0x0F00) >> 8) as usize;

        for i in 0..=x {
            self.registers.v[i] = self.ram.read_byte(self.registers.i as usize + i)?;
        }

        self.registers.i += x as u16 + 1;
        self.registers.pc += WORD_SIZE;
        Ok(())
    }

    fn set_delay_timer(&mut self, value: u8) {
        self.delay_timer = time::Instant::now();
        self.registers.dt = value;
    }

    fn get_delay_timer(&self) -> u8 {
        let ms = self.delay_timer.elapsed().as_millis();
        let ticks = ms / 16;
        if ticks >= self.registers.dt as u128 {
            0
        } else {
            self.registers.dt - ticks as u8
        }
    }
}
