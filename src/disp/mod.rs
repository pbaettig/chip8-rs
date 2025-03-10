extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::sync::{Arc, Mutex};
use std::time::Duration;
pub struct Display {
    pixel_size: u32,
    sdl_context: sdl2::Sdl,
    screen_buffer: Arc<Mutex<[u8; 2048]>>,
}

impl Display {
    pub fn new(pixel_size: u32, buffer: Arc<Mutex<[u8; 2048]>>) -> Self {
        let sdl_context = sdl2::init().unwrap();

        Self {
            pixel_size: pixel_size,
            screen_buffer: buffer,
            sdl_context: sdl_context,
            // canvas: canvas,
        }
    }

    // pub fn set_pixel(&mut self, x: u8, y: u8, value: u8) {
    //     self.screen_buffer[(x*64 + y) as usize] = value;
    // }

    pub fn run(&mut self) {
        let video_subsystem = self.sdl_context.video().unwrap();

        let window = video_subsystem
            .window("CHIP-8", 64 * self.pixel_size, 32 * self.pixel_size)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        'running: loop {
            canvas.set_draw_color(Color::BLACK);
            canvas.clear();

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

            canvas.set_draw_color(Color::GRAY);

            {
                let screen_buffer = self.screen_buffer.lock().unwrap();
                for x in 0..64 {
                    canvas.draw_line(
                        Point::new(self.pixel_size as i32 * x, 0),
                        Point::new(self.pixel_size as i32 * x, self.pixel_size as i32 * 32),
                    );

                    for y in 0..32 {
                        canvas.draw_line(
                            Point::new(0, self.pixel_size as i32 * y),
                            Point::new(self.pixel_size as i32 * 64, self.pixel_size as i32 * y),
                        );

                        if screen_buffer[(x + y * 64) as usize] == 1 {
                            canvas.set_draw_color(Color::WHITE);

                            canvas
                                .fill_rect(Rect::new(
                                    (x as i32) * (self.pixel_size as i32),
                                    (y as i32) * (self.pixel_size as i32),
                                    self.pixel_size,
                                    self.pixel_size,
                                ))
                                .unwrap();
                            canvas.set_draw_color(Color::GRAY);
                        }
                    }
                }
            }
            canvas.present();

            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 100));
        }
    }
}
