extern crate minifb;

mod bit_utils;
mod cpu;
mod interconnect;
mod mem_map;
mod nes;
mod ram;
mod rom;
mod sram;

use std::env;
use minifb::{Key, KeyRepeat, WindowOptions, Window, MouseMode};

const WIDTH: usize = 256;
const HEIGHT: usize = 240;

fn main() {

	let rom_file_name = match env::args().nth(1){
		Some(filename) => {
			filename
		}
		None => {
			panic!("No arguments given.");
		}
	};
	let rom = match rom::Rom::load(rom_file_name) {
		Ok(rom) => {
			println!("ROM loaded successfully.");
			rom
		}
		Err(err) => {
			panic!("Couldn't load ROM.");
		}
	};

	let mut nes = nes::Nes::new(rom, sram::Sram::new());
	println!("Cpu: {:#?}", nes.cpu);

	let mut window_buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

	let mut window_handle = Window::new("Test Window",
										WIDTH,
										HEIGHT,
										WindowOptions::default()).unwrap();
	let mut last_mouse_pos = (0.0, 0.0);
	while window_handle.is_open() && !window_handle.is_key_down(Key::Escape) {
		for i in window_buffer.iter_mut() {
			*i = 100;
		}
		window_handle.update_with_buffer(&window_buffer).unwrap();
		window_handle.get_mouse_pos(MouseMode::Clamp).map(|mouse| {
			if mouse != last_mouse_pos {
				last_mouse_pos = mouse;
				println!("x: {} y: {}", mouse.0, mouse.1);
			}
		});
	}
}
