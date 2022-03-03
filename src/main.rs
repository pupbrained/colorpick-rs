use std::{fs::{File, remove_file}, thread, time::Duration, io::ErrorKind::WouldBlock};
use rdev::{listen, Event, Button::Left, EventType::ButtonPress};
use device_query::{DeviceQuery, DeviceState, MouseState};
use scrap::{Capturer, Display};
use image::{ImageBuffer, Rgb};

fn convert_to_hex(r: i32, g: i32, b: i32) -> String {
    format!("{:02X}{:02X}{:02X}", 
    r as f32 as u8, 
    g as f32 as u8, 
    b as f32 as u8) 
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

        let mut bitflipped = Vec::with_capacity(w * h * 4);
        let stride = buffer.len() / h;

        for y in 0..h {
            for x in 0..w {
                let i = stride * y + 4 * x;
                bitflipped.extend_from_slice(&[
                    buffer[i + 2],
                    buffer[i + 1],
                    buffer[i],
                    255,
                ]);
            }
        }

        repng::encode(
            File::create("/tmp/screenshot.png").unwrap(),
            w as u32,
            h as u32,
            &bitflipped,
        ).unwrap();
        break;
    }
    let image = image::open("/tmp/screenshot.png").unwrap();
    let img: ImageBuffer<Rgb<u8>, Vec<u8>> = image.to_rgb8();
    let pix = img.get_pixel(mouse.coords.0 as u32, mouse.coords.1 as u32);
    println!("Pixel color RGB: ({}, {}, {})", pix[0], pix[1], pix[2]);
    println!("Pixel color Hex: #{}", convert_to_hex(pix[0] as i32, pix[1] as i32, pix[2] as i32));
    remove_file("/tmp/screenshot.png").expect("Failed to delete file!");
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