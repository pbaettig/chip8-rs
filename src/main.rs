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
    let pause: Arc<Mutex<u8>> = Arc::new(Mutex::new(0));
    let pause_2: Arc<Mutex<u8>> = Arc::clone(&pause);
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
                    0x6b, 0x0F, // Set VB (Y),
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
            if *pause_2.lock().unwrap() == 0 {
                let r = proc.execute();
                match r {
                    Ok(d) => dbg!(d),
                    Err(e) => {
                        println!("{e}");
                        break;
                    }
                };
            }

            ::std::thread::sleep(Duration::new(0, 200_000_000u32));
        }
    });

    let mut display = disp::Display::new(20, display_buffer);
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
                    if *p == 1 {
                        *p = 0;
                    } else {
                        *p = 1;
                    };
                }
                _ => {}
            }
        }
        display.update(*pause.lock().unwrap());
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    // handle.join().unwrap();

    // const pixel_size: u32 = 20;

    // let mut v = String::from("hello");

    // let v1 = &v;
    // let v2 = &v;
    // let v3 = &mut v;

    // println!("{v}: {v1}, {v2}, {v3}");

    // let mut screen: [bool; 32*64] = [false; 32*64];

    // let sdl_context = sdl2::init().unwrap();
    // let video_subsystem = sdl_context.video().unwrap();

    // let window = video_subsystem.window("rust-sdl2 demo", 64 * pixel_size, 32*pixel_size)
    //     .position_centered()
    //     .build()
    //     .unwrap();

    // let mut canvas = window.into_canvas().build().unwrap();
    // let mut pixels: Vec<Rect> = vec![];

    // canvas.set_draw_color(Color::RGB(0, 0, 0));
    // canvas.clear();

    // canvas.present();
    // let mut event_pump = sdl_context.event_pump().unwrap();
    // let mut i = 0;

    // let mut last_keypress = Instant::now();
    // 'running: loop {
    //     canvas.set_draw_color(Color::BLACK);
    //     canvas.clear();

    //     for event in event_pump.poll_iter() {
    //         match event {
    //             Event::Quit {..} |
    //             Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
    //                 break 'running
    //             },
    //             // Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
    //             //     let since_last_keypress = last_keypress.elapsed();
    //             //     last_keypress = Instant::now();
    //             //     if since_last_keypress < Duration::from_millis(100) {
    //             //         continue;
    //             //     }

    //             //     // let x = i % 64;
    //             //     // let y = i / 64;
    //             //     screen[i] = true;
    //             //     i += 1;
    //             // },
    //             _ => {}
    //         }
    //     }

    //     screen[i] = i%3==0;
    //     i = (i+1) % (32*64);

    //     canvas.set_draw_color(Color::WHITE);
    //     for x in 0..64 {
    //         for y in 0..32 {
    //             if screen[x + y*64] {
    //                 canvas.fill_rect(Rect::new((x as i32)*(pixel_size as i32), (y as i32)*(pixel_size as i32), pixel_size, pixel_size));
    //             }
    //         }
    //     }
    //     canvas.present();

    //     ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 100));
    // }
}
