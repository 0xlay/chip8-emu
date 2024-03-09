#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
// #![warn(clippy::restriction)]

mod emu;
mod utl;

use clap::Parser;

use emu::chip8::Chip8;
use utl::config::Args;

fn main() {
    let args = Args::parse();
    match Chip8::new(args.width, args.height) {
        Ok(mut chip8) => {
            if let Err(err) = chip8.load_rom(args.rom_path.as_str()) {
                eprintln!("[-] Failed to load the ROM. Error => `{err}`");
            } else if let Err(err) = chip8.run() {
                eprintln!("[-] Failed to run the app. Error => `{err}`");
            }
        }
        Err(err) => {
            eprintln!("[-] Failed to run the CHIP8 emulator. Error => `{err}`");
        }
    }
}
