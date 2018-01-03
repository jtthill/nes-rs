use cpu::Cpu;
use interconnect::Interconnect;
use gamepak::GamePak;

pub struct Nes {
	pub cpu: Cpu,
	pub interconnect: Interconnect,
}

impl Nes {
	pub fn new(gamepak: GamePak) -> Self {
		Nes {
			cpu: Cpu::new(),
			interconnect: Interconnect::new(gamepak),
		}
	}

	pub fn step(&mut self) -> u32 {
		// Step with the CPU, passing in Interconnect
		0
	}
}