extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use std::sync::{Arc, Mutex};
use std::time::Duration;
pub struct Display {
    pixel_size: u32,
    sdl_context: sdl2::Sdl,
    // window: Window,
    canvas: Canvas<Window>,
    pub event_pump: EventPump,
    screen_buffer: Arc<Mutex<[u8; 2048]>>,
    pub draw_grid: bool
}

impl Display {
    pub fn new(pixel_size: u32, buffer: Arc<Mutex<[u8; 2048]>>) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("CHIP-8", 64 * pixel_size, 32 * pixel_size)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();
        Self {
            pixel_size: pixel_size,
            screen_buffer: buffer,
            sdl_context: sdl_context,
            // window: window,
            canvas: canvas,
            event_pump: event_pump, // canvas: canvas,
            draw_grid: false,
        }
    }

    // pub fn set_pixel(&mut self, x: u8, y: u8, value: u8) {
    //     self.screen_buffer[(x*64 + y) as usize] = value;
    // }

    pub fn update(&mut self, pause_state: u8) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        self.canvas.set_draw_color(Color::GRAY);
        if pause_state == 1 {
            self.canvas
                .fill_rect(Rect::new(
                    self.pixel_size as i32 * 60,
                    self.pixel_size as i32 * 1,
                    self.pixel_size,
                    self.pixel_size * 4,
                ))
                .unwrap();
            self.canvas
                .fill_rect(Rect::new(
                    self.pixel_size as i32 * 62,
                    self.pixel_size as i32 * 1,
                    self.pixel_size,
                    self.pixel_size * 4,
                ))
                .unwrap();
        }

        {
            let screen_buffer = self.screen_buffer.lock().unwrap();
            for x in 0..64 {
                if self.draw_grid {
                    self.canvas
                        .draw_line(
                            Point::new(self.pixel_size as i32 * x, 0),
                            Point::new(self.pixel_size as i32 * x, self.pixel_size as i32 * 32),
                        )
                        .unwrap();
                }
                
                for y in 0..32 {
                    if self.draw_grid {
                        self.canvas
                            .draw_line(
                                Point::new(0, self.pixel_size as i32 * y),
                                Point::new(self.pixel_size as i32 * 64, self.pixel_size as i32 * y),
                            )
                            .unwrap();

                    }
                    
                    if screen_buffer[(x + y * 64) as usize] == 1 {
                        self.canvas.set_draw_color(Color::WHITE);

                        self.canvas
                            .fill_rect(Rect::new(
                                (x as i32) * (self.pixel_size as i32),
                                (y as i32) * (self.pixel_size as i32),
                                self.pixel_size,
                                self.pixel_size,
                            ))
                            .unwrap();
                        self.canvas.set_draw_color(Color::GRAY);
                    }
                }
            }
        }
        self.canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 100));
    }
}
