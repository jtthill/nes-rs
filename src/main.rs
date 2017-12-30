extern crate minifb;

mod cpu;
mod rom;
mod bit_utils;
mod mem_map;
mod interconnect;

use std::env;
use minifb::{Key, WindowOptions, Window};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
	println!("Hello, world!");
	let inter: interconnect::Interconnect;
	if let Some(rom_file_name) = env::args().nth(1) {
		if let Ok(rom) = rom::Rom::new(rom_file_name) {
			// Create memory with Rom object that was created
		} else {
			panic!("Not given proper iNES file.");
		}
	} else {
		panic!("No arguments given.");
	}

	let cpu = cpu::Cpu::new();
	println!("Cpu: {:#?}", cpu);

	let mut window_buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

	let mut window_handle = Window::new("Test Window",
										WIDTH,
										HEIGHT,
										WindowOptions::default()).unwrap();
	while window_handle.is_open() && !window.is_key_down(Key::Escape) {
		for i in window_buffer.iter_mut() {
			*i = 0;
		}
		window.update_with_buffer(&window_buffer).unwrap();
	}
}
