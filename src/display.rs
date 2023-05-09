use std::error::Error;
use std::fmt;

use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::{Canvas, RenderTarget, WindowCanvas},
    video::{Window, WindowBuilder},
};

const WINDOW_NAME: &str = "The CHIP8 Emulator";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

///
/// The DisplayError enum represents the possible errors that can occur when working with the display.
///
#[derive(Debug)]
pub enum DisplayError {
    FailedToCreateContext,
    FailedToCreateVideoSubsystem,
    FailedToCreateWindow,
    FailedToCreateCanvas,
}

impl Error for DisplayError {}

impl fmt::Display for DisplayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            DisplayError::FailedToCreateContext => write!(f, "Failed to create context!"),
            DisplayError::FailedToCreateVideoSubsystem => {
                write!(f, "Failed to create video subsystem!")
            }
            DisplayError::FailedToCreateWindow => write!(f, "Failed to create window!"),
            DisplayError::FailedToCreateCanvas => write!(f, "Failed to create canvas!"),
        }
    }
}

pub struct Display {
    width: u32,
    height: u32,
    sdl_context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    canvas: WindowCanvas,
}

impl Display {
    pub fn new() -> Result<Self, DisplayError> {
        let sdl_context = match sdl2::init() {
            Ok(sdl_context) => sdl_context,
            Err(_) => return Err(DisplayError::FailedToCreateContext),
        };

        let video_subsystem = match sdl_context.video() {
            Ok(video_subsystem) => video_subsystem,
            Err(_) => return Err(DisplayError::FailedToCreateVideoSubsystem),
        };

        let window =
            match WindowBuilder::new(&video_subsystem, WINDOW_NAME, WINDOW_WIDTH, WINDOW_HEIGHT)
                .position_centered()
                .build()
                .map_err(|e| e.to_string())
            {
                Ok(window) => window,
                Err(_) => return Err(DisplayError::FailedToCreateWindow),
            };

        let canvas = match window.into_canvas().build().map_err(|e| e.to_string()) {
            Ok(canvas) => canvas,
            Err(_) => return Err(DisplayError::FailedToCreateCanvas),
        };

        let display = Self {
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            sdl_context,
            video_subsystem,
            canvas,
        };

        Ok(display)
    }

    pub fn create(&mut self) -> Result<(), String> {
        self.clear();

        let mut event_pump = self.sdl_context.event_pump()?;
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }
        }

        Ok(())
    }

    pub fn clear(&mut self) {
        self.update_color(Color::RGB(0, 0, 0));
    }

    pub fn update_color(&mut self, rgb: Color) {
        self.canvas.set_draw_color(rgb);
        self.canvas.clear();
        self.canvas.present();
    }
}
