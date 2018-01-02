use rom::Rom;
use sram::Sram;
use ram::Ram;

pub struct Interconnect {
	rom: Rom,
	sram: Sram,
	ram: Ram,
}

impl Interconnect {
	pub fn new(rom: Rom, sram: Sram) -> Self {
		Interconnect {
			rom,
			sram,
			ram: Ram::new(),
		}
	}

	pub fn read_byte(&self, addr: u16) -> u8 {
		0
	}

	pub fn write_byte(&mut self, addr: u16, data: u8) {

	}
}