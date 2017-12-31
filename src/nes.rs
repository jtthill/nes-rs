use cpu::Cpu;
use interconnect::Interconnect;
use rom::Rom;
use sram::Sram;
use minifb::Window;

pub struct Nes {
	pub cpu: Cpu,
	pub interconnect: Interconnect,
}

impl Nes {
	pub fn new(rom: Rom, sram: Sram) -> Self {
		Nes {
			cpu: Cpu::new(),
			interconnect: Interconnect::new(rom, sram),
		}
	}

	pub fn step(&mut self) -> u32 {
		// Step with the CPU, passing in Interconnect
		0
	}
}