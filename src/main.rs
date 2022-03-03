use std::{fs::{File, remove_file}, thread, time::Duration, io::ErrorKind::WouldBlock};
use rdev::{listen, Event, Button::Left, EventType::ButtonPress};
use device_query::{DeviceQuery, DeviceState, MouseState};
use scrap::{Capturer, Display};
use image::{ImageBuffer, Rgb};

fn convert_to_hex(r: u8, g: u8, b: u8) -> String {
    format!("{:02X}{:02X}{:02X}", r, g, b) 
}

fn get_screen() {
    let device_state = DeviceState::new();
    let mouse: MouseState = device_state.get_mouse();
    let one_second = Duration::new(1, 0);
    let one_frame = one_second / 60;
    let display = Display::primary().expect("Couldn't find primary display.");
    let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");
    let (w, h) = (capturer.width(), capturer.height());
    loop {

        let buffer = match capturer.frame() {
            Ok(buffer) => buffer,
            Err(error) => {
                if error.kind() == WouldBlock {
                    thread::sleep(one_frame);
                    continue;
                } else {
                    panic!("Error: {}", error);
                }
            }
        };

        println!("Captured! Please wait...");
        let stride = buffer.len() / h;
        let index = stride as i32 * mouse.coords.1 + 4 * mouse.coords.0;
        let i = index as usize;
        println!("Index: {}", i);
        let r = buffer[i+2];
        let g = buffer[i+1];
        let b = buffer[i];

        println!("Pixel color RGB: ({}, {}, {})", r, g, b);
        println!("Pixel color Hex: #{}", convert_to_hex(r, g, b));

        break;
    }
    std::process::exit(0);
}

fn callback(event: Event) {
    match event.event_type {
        ButtonPress(Left) => get_screen(),
        _ => ()
    }
}

fn main() {
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }
}
