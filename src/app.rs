mod screen;
mod per_pixel;

use crate::constants::*;
use screen::Screen;

pub struct App {
    screen: Screen,
}

pub fn new() -> App {
    App {
        screen: Screen::new(
            "RayTraccer",
            WIDTH,
            HEIGHT,
        )
    }
}

impl App {
    pub fn run(&mut self) {
        while self.screen.is_open() {
            self.screen.update();
        }
    }
    fn pixel_iter(&self) {
        for i in self.screen.buf.iter() {
            
        }
    }
}