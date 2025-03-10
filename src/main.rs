// use std::{io, path::Path};
// use getch_rs::{Getch, Key};
mod inst;
mod mem;
mod proc;
mod reg;

extern crate sdl2;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod disp;
pub fn main() {
    let pause: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    let pause_2: Arc<Mutex<bool>> = Arc::clone(&pause);
    let display_buffer: Arc<Mutex<[u8; 2048]>> = Arc::new(Mutex::new([0; 2048]));
    let display_buffer_2 = Arc::clone(&display_buffer);

    let _ = thread::spawn(move || {
        let mut memory = mem::Memory::new();
        let _ = memory
            .load_array(
                512,
                &[
                    0xA0, 0x82, // Set I
                    0x6a, 0x12, // Set VA (X)
                    0x6b, 0x0e, // Set VB (Y),
                    0xda, 0xb5, // draw
                    0xA0, 0x87, // Set I = 0x91
                    0x6a, 0x17, // Set VA
                    0xda, 0xb5, // draw
                    0xA0, 0x8c, // Set I
                    0x6a, 0x1c, // Set VA
                    0xda, 0xb5, // draw
                    0xA0, 0x91, // Set I
                    0x6a, 0x21, // Set VA
                    0xda, 0xb5, // draw
                    0xA0, 0x96, // Set I
                    0x6a, 0x26, // Set VA
                    0xda, 0xb5, // draw
                    0xA0, 0x9b, // Set I
                    0x6a, 0x2b, // Set VA
                    0xda, 0xb5, // draw
                    0x00, 0xe0, // clear
                    0x12, 0x00, // goto 512
                ],
            )
            .unwrap();

        let mut proc = proc::Processor::new(memory, display_buffer_2);
        loop {
            if !*pause_2.lock().unwrap() {
                let r = proc.execute();
                match r {
                    Ok(d) => dbg!(d),
                    Err(e) => {
                        println!("{e}");
                        break;
                    }
                };
            }

            ::std::thread::sleep(Duration::new(0, 100_000_000u32));
        }
    });

    let mut display = disp::Display::new(23, display_buffer);
    let mut grid_state: bool = false;

    'running: loop {
        for event in display.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    let mut p = pause.lock().unwrap();
                    *p = !*p;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::G),
                    ..
                } => {
                    grid_state = !grid_state;
                }
                _ => {}
            }
        }
        display.set_pause(*pause.lock().unwrap());
        display.set_grid(grid_state);
        display.update();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
