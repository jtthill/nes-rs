#[macro_use] 
extern crate log;
extern crate env_logger;

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

const WIDTH: usize = 256;
const HEIGHT: usize = 240;

fn main() {
	env_logger::init();
	info!("Logging begun.");
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
			info!("ROM loaded successfully.");
			gamepak
		}
		Err(err) => {
			panic!("Couldn't load ROM {:#?}.", err);
		}
	};

	let mut nes = nes::Nes::new(gamepak);
	nes.power_on_reset();
	debug!("Cpu: {:#?}", nes.cpu);
	debug!("Mapper: {}", nes.interconnect.gamepak.mapper_num);
	debug!("Contains PRG Ram: {}", nes.interconnect.gamepak.contains_prg_ram);
	loop {
		nes.step();
	}
}
