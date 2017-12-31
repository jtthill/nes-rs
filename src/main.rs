extern crate minifb;

mod bit_utils;
mod cpu;
mod interconnect;
mod mem_map;
mod nes;
mod rom;
mod sram;

use std::env;
use minifb::{Key, WindowOptions, Window};

const WIDTH: usize = 256;
const HEIGHT: usize = 240;

fn main() {

//	if let Some(rom_file_name) = env::args().nth(1) {
//		if let Ok(rom) = rom::Rom::new(rom_file_name) {
//			// Create memory with Rom object that was created
//
//		} else {
//			panic!("Not given proper iNES file.");
//		}
//	} else {
//		panic!("No arguments given.");
//	}

	let rom_file_name = match env::args().nth(1){
		Some(filename) => {
			filename
		}
		None => {
			panic!("No arguments given.");
		}
	};
	let rom = match rom::Rom::new(rom_file_name) {
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
	while window_handle.is_open() && !window_handle.is_key_down(Key::Escape) {
		for i in window_buffer.iter_mut() {
			*i = 100;
		}
		window_handle.update_with_buffer(&window_buffer).unwrap();
	}
}
