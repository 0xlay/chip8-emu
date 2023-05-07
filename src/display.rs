use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::{Canvas, RenderTarget},
    video::{Window, WindowBuilder},
};

const DISPLAY_WIDTH: u32 = 64;
const DISPLAY_HEIGHT: u32 = 32;

pub struct Display {
    width: u32,
    height: u32,
}

impl Display {
    pub fn new() -> Self {
        Self {
            width: DISPLAY_WIDTH,
            height: DISPLAY_HEIGHT,
        }
    }

    pub fn clear(&mut self) {}
}
