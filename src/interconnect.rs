use rom::Rom;
use sram::Sram;

pub struct Interconnect {
	rom: Rom,
	sram: Sram
}

impl Interconnect {
	pub fn new(rom: Rom, sram: Sram) -> Self {
		Interconnect {
			rom,
			sram
		}
	}

	pub fn read_byte(&self, addr: u16) -> u8 {
		0
	}

	pub fn write_byte(&mut self, addr: u16, data: u8) {

	}
}