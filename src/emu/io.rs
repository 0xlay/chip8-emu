use std::error::Error;
use std::fmt;

use sdl2::{
    keyboard::Keycode, pixels::Color, rect::Rect, render::WindowCanvas, video::WindowBuilder,
    EventPump,
};

///
/// The title of the window for the Chip8 emulator.
///
const WINDOW_NAME: &str = "The CHIP8 Emulator";

/// The width of the display grid for the Chip8 emulator in pixels.
pub const GRID_WIDTH: usize = 64;

/// The height of the display grid for the Chip8 emulator in pixels.
pub const GRID_HEIGHT: usize = 32;

///
/// The `DisplayError` enum represents the possible errors that can occur when working with the display.
///
#[derive(Debug)]
pub enum DisplayError {
    FailedToCreateContext,
    FailedToCreateVideoSubsystem,
    FailedToCreateWindow,
    FailedToCreateCanvas,
    FailedToGetEventPump,
}

impl Error for DisplayError {}

impl fmt::Display for DisplayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::FailedToCreateContext => write!(f, "Failed to create context!"),
            Self::FailedToCreateVideoSubsystem => {
                write!(f, "Failed to create video subsystem!")
            }
            Self::FailedToCreateWindow => write!(f, "Failed to create window!"),
            Self::FailedToCreateCanvas => write!(f, "Failed to create canvas!"),
            Self::FailedToGetEventPump => write!(f, "Failed to get event pump!"),
        }
    }
}

///
/// The `Display` structure represents display for the chip8 emulator.
///
pub struct Display {
    width: u32,
    height: u32,
    sdl_context: sdl2::Sdl,
    canvas: WindowCanvas,
    grid: [u8; GRID_WIDTH * GRID_HEIGHT],
}

impl Display {
    pub fn new(width: u32, height: u32) -> Result<Self, DisplayError> {
        let sdl_context = match sdl2::init() {
            Ok(sdl_context) => sdl_context,
            Err(_) => return Err(DisplayError::FailedToCreateContext),
        };

        let video_subsystem = match sdl_context.video() {
            Ok(video_subsystem) => video_subsystem,
            Err(_) => return Err(DisplayError::FailedToCreateVideoSubsystem),
        };

        let Ok(window) = WindowBuilder::new(&video_subsystem, WINDOW_NAME, width, height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())
        else {
            return Err(DisplayError::FailedToCreateWindow);
        };

        let canvas = match window.into_canvas().build().map_err(|e| e.to_string()) {
            Ok(canvas) => canvas,
            Err(_) => return Err(DisplayError::FailedToCreateCanvas),
        };

        let display = Self {
            width: width,
            height: height,
            sdl_context,
            canvas,
            grid: [0; GRID_WIDTH * GRID_HEIGHT],
        };

        Ok(display)
    }

    pub fn get_event_pump(&mut self) -> Result<EventPump, DisplayError> {
        match self.sdl_context.event_pump() {
            Ok(event_pump) => Ok(event_pump),
            Err(_) => Err(DisplayError::FailedToGetEventPump),
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, new_pixel: u8) {
        let index = y * GRID_WIDTH + x;
        self.grid[index] = new_pixel;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> u8 {
        let index = y * GRID_WIDTH + x;
        self.grid[index]
    }

    pub fn draw(&mut self) {
        // Set the background color to black
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        // Calculate the size of each pixel to fit the display size
        let pixel_width = self.width / GRID_WIDTH as u32;
        let pixel_height = self.height / GRID_HEIGHT as u32;

        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                // Calculate the index for the 1D array
                let index = y * GRID_WIDTH + x;

                // Access the pixel value using the calculated index
                let pixel_value = self.grid[index];
                // Set draw color based on the pixel's state
                if pixel_value == 1 {
                    self.canvas.set_draw_color(Color::RGB(0, 255, 0)); // Green for 'on' pixels
                } else {
                    continue; // Skip drawing 'off' pixels (background is already set)
                }

                let pixel_rect = Rect::new(
                    (x as u32 * pixel_width) as i32,
                    (y as u32 * pixel_height) as i32,
                    pixel_width,
                    pixel_height,
                );

                if let Err(e) = self.canvas.fill_rect(pixel_rect) {
                    eprintln!("Failed to draw pixel at ({x}, {y}): {e}");
                }
            }
        }

        // Present the updated canvas
        self.canvas.present();
    }

    pub fn clear(&mut self) {
        self.grid.fill(0);
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.present();
    }
}

///
/// The `Keyboard` struct represents keyboard for the chip8 emulator.
///
pub struct Keyboard {
    key: u8,
}

impl Keyboard {
    pub fn new() -> Self {
        Self { key: 0x0 }
    }

    pub fn press_key(&mut self, key: Keycode) {
        match key {
            Keycode::Num1 => self.key = 0x1,
            Keycode::Num2 => self.key = 0x2,
            Keycode::Num3 => self.key = 0x3,
            Keycode::Num4 => self.key = 0xC,
            Keycode::Q => self.key = 0x4,
            Keycode::W => self.key = 0x5,
            Keycode::E => self.key = 0x6,
            Keycode::R => self.key = 0xD,
            Keycode::A => self.key = 0x7,
            Keycode::S => self.key = 0x8,
            Keycode::D => self.key = 0x9,
            Keycode::F => self.key = 0xE,
            Keycode::Z => self.key = 0xA,
            Keycode::X => self.key = 0x0,
            Keycode::C => self.key = 0xB,
            Keycode::V => self.key = 0xF,
            _ => (),
        }
    }

    pub fn release_key(&mut self) {
        self.key = 0x0;
    }

    pub const fn get_pressed_key(&self) -> Option<u8> {
        if self.key != 0x0 {
            Some(self.key)
        } else {
            None
        }
    }

    pub const fn is_key_pressed(&self, key: u8) -> bool {
        self.key == key
    }
}
