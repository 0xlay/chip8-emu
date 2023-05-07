mod chip8;
mod display;
mod instructions;
mod keyboard;
mod ram;
mod registers;

use chip8::Chip8;

// http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
// TODO: complete the implementation of the Chip8 emulator.
// TODO: Отрисовать окно или с помощью вулкан или opengl
// https://tobiasvl.github.io/blog/write-a-chip-8-emulator/

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<String>>();
    if args.len() == 1 {
        let mut chip8 = Chip8::new();
        if let Err(err) = chip8.load_rom(&args[0]) {
            eprintln!("CHIP8 load rom: {err}");
        } else if let Err(err) = chip8.run() {
            eprintln!("CHIP8 run: {err}");
        }
    } else {
        eprintln!("CHIP8: Invalid arguments!")
    }
}
