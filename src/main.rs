#![allow(dead_code)]
#![allow(unused)]

use image::{Rgb, RgbImage};
use lazy_static::lazy_static;
use nalgebra_glm::*;
use std::time::Instant;
use minifb::{Window, WindowOptions};


const WIDTH: usize = 800;
const HEIGHT: usize = 800;
lazy_static! {
    static ref LIGHT: Vec3 = normalize(&vec3(0.5, 0.5, 0.1));
}
lazy_static! {
    static ref BG_COL: Vec3 = vec3(0.2, 0.4, 0.8);
}

struct Screen {
    width: usize,
    height: usize,
    window: Window,
    buf: Vec<Vec3>
}
impl Screen {
    fn new(name: &str, width: usize, height: usize) -> Screen {
        Screen {
            width: width,
            height: height,
            window: Window::new(name, width, height,
                WindowOptions {
                    resize: true,
                    ..WindowOptions::default()
                }
            ).expect("Unable to open Window"),
            buf: vec![vec3(0.0, 0.0, 0.0); WIDTH*HEIGHT]//Vec::new()
        }
    }
    fn update(&mut self) {
        self.window.update_with_buffer(
            &self.buf_to_u32(),
            self.width,
            self.height
        );
    }
    fn buf_to_u32(&self) -> Vec<u32> {
        let mut vec = Vec::new();
        for i in self.buf.iter() {
            let c = clamp(i, 0.0, 1.0);
            let r = (c.x * 255.0) as u32;
            let g = (c.y * 255.0) as u32;
            let b = (c.z * 255.0) as u32;
            vec.push((r<< 16) | (g << 8) | (b));
        }
        vec
    }
    fn set_at(&mut self, pos: &U32Vec2, color: Vec3) {
        self.buf[(pos.x as usize) * self.width + (pos.y as usize)] = color;
    }
    fn is_open(&self) -> bool {
        self.window.is_open()
    }
}

struct App {
    screen: Screen,

}
impl App {
    
}


fn main() {
    let mut screen = Screen::new(
        "RayTraccer",
        WIDTH,
        HEIGHT
    );
    while screen.is_open() {
        screen.update()
    }
}