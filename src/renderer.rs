#![allow(dead_code)]
use minifb::{Key, Scale, Window, WindowOptions};

pub struct Renderer {
    screen_width: usize,
    screen_height: usize,
    window_ref: Window,
}

impl Renderer {
    pub fn new(title: &str, screen_width: usize, screen_height: usize) -> Self {
        // build window!
        let window_ref = Window::new(
            title,
            screen_width,
            screen_height,
            WindowOptions {
                scale: Scale::X1,
                ..WindowOptions::default()
            },
        ).expect("Unable to Create Window");
        Self {
            screen_width,
            screen_height,
            window_ref,
        }
    }

    pub fn run(self) {
        // build buffer
        let buffer: Vec<u32> = vec![0; self.screen_width * self.screen_height];
        // run window
        let mut window = self.window_ref;
        while window.is_open() && !window.is_key_down(Key::Escape) {
            window.update_with_buffer(&buffer).unwrap();
        }
    }
}
