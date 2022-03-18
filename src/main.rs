use std::{env, thread, time::Duration, io::ErrorKind::WouldBlock};
use rdev::{listen, Event, Button::{Left, Right}, EventType::ButtonPress};
use device_query::{DeviceQuery, DeviceState, MouseState};
use scrap::{Capturer, Display};

// Reformats the rgb values to a hex string
fn convert_to_hex(r: u8, g: u8, b: u8) -> String {
    format!("{:02X}{:02X}{:02X}", r, g, b) 
}

fn get_screen() {
    // Get mouse coords
    let device_state = DeviceState::new();
    let mouse: MouseState = device_state.get_mouse();
    
    // Define one frame of a screen record, in order to get a screenshot
    let one_second = Duration::new(1, 0);
    let one_frame = one_second / 60;
    
    // Create a capturer
    let display = Display::primary().expect("Couldn't find primary display.");
    let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");
    let h = capturer.height();
    
    loop {
        // Capture a screenshot
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

        // Get the pixel at the mouse coords
        let stride = buffer.len() / h;
        let index = stride as i32 * mouse.coords.1 + 4 * mouse.coords.0;
        let i = index as usize;
        let r = buffer[i+2];
        let g = buffer[i+1];
        let b = buffer[i];

        // Print out color info
        println!("Pixel color RGB: ({}, {}, {})", r, g, b);
        println!("Pixel color Hex: #{}", convert_to_hex(r, g, b));
        break;
    }
}

fn callback(event: Event) {
    match event.event_type {
        ButtonPress(Left) => get_screen(),
        ButtonPress(Right) => std::process::exit(0),
        _ => ()
    }
}

fn callback_live(event: Event) {
    loop {
        if let ButtonPress(Right) = event.event_type {
            std::process::exit(0)
        } 
        get_screen();
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Press left mouse button to get screen color, or right mouse button to exit.");
        if let Err(error) = listen(callback) {
            println!("Error: {:?}", error)
        }
    } else {
        if args[1] == "--live" || args[1] == "-l" {
            println!("Use ctrl+c to exit after you're done.");
        if let Err(error) = listen(callback_live) {
            println!("Error: {:?}", error)
        }
        }
    }
}
