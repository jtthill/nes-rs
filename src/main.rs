extern crate minifb;

mod apu;
mod bit_utils;
mod cpu;
mod gamepak;
mod interconnect;
mod mem_map;
mod nes;
mod ppu;
mod ram;

use std::env;
use minifb::{Key, WindowOptions, Window};

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
	let gamepak = match gamepak::GamePak::load_rom(rom_file_name) {
		Ok(gamepak) => {
			println!("ROM loaded successfully.");
			gamepak
		}
		Err(err) => {
			panic!("Couldn't load ROM.");
		}
	};

	let mut nes = nes::Nes::new(gamepak);
	println!("Cpu: {:#?}", nes.cpu);
	println!("Mapper: {}", nes.interconnect.gamepak.mapper_num);
	println!("Contains PRG Ram: {}", nes.interconnect.gamepak.contains_prg_ram);

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
	}
}
