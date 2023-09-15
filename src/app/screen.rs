use minifb::{Window, WindowOptions};

use crate::constants::*;


pub struct Screen {
    width: usize,
    height: usize,
    window: Window,
    pub buf: Vec<Vec3>
}
impl Screen {
    pub fn new(name: &str, width: usize, height: usize) -> Screen {
        Screen {
            width: width,
            height: height,
            window: Window::new(name, width, height,
                WindowOptions {
                    resize: true,
                    ..WindowOptions::default()
                }
            ).expect("Unable to open Window"),
            buf: vec![vec3(0.0, 0.0, 0.0); WIDTH*HEIGHT]
        }
    }
    #[allow(unused_must_use)]
    pub fn update(&mut self) {
        self.window.update_with_buffer(
            &self.buf_to_u32(),
            self.width,
            self.height
        );
    }
    pub fn set_at(&mut self, pos: &U32Vec2, color: Vec3) {
        self.buf[(pos.x as usize) * self.width + (pos.y as usize)] = color;
    }
    pub fn is_open(&self) -> bool {
        self.window.is_open()
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
}
