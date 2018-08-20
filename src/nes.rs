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

	pub fn power_on_reset(&mut self) {
		self.cpu.power_on_reset(&self.interconnect);
	}

	pub fn step(&mut self) -> u32 {
		self.cpu.step(&mut self.interconnect)
	}
}