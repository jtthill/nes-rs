use apu::Apu;
use gamepak::GamePak;
use ppu::Ppu;
use ram::Ram;
use mem_map::*;

pub struct Interconnect {
	pub apu: Apu,
	pub gamepak: GamePak,
	pub ppu: Ppu,
	pub ram: Ram,
}

impl Interconnect {
	pub fn new(gamepak: GamePak) -> Self {
		Interconnect {
			apu: Apu::new(),
			gamepak,
			ppu: Ppu::new(),
			ram: Ram::new(),
		}
	}

	pub fn read_byte(&self, addr: u16) -> u8 {
		match addr {
			INTERNAL_RAM_START ... INTERNAL_RAM_END => self.ram.read_byte(addr - INTERNAL_RAM_START),
			PPU_CTRL_REG ... PPU_MIRROR_END => unimplemented!("PPU register access unimplemented."),
			APU_SQ1_VOL ... APU_DMC_LEN => unimplemented!("APU register access unimplemented."),
			APU_SND_CHN => unimplemented!("APU sound enable unimplemented."),
			OAM_DMA_REG => unimplemented!("DMA register access unimplemented."),
			GAMEPAD_JOY_1 ... GAMEPAD_JOY_2 => unimplemented!("Gamepad registers unimplemented."),
			GAMEPAK_MEM_START ... GAMEPAK_MEM_END => self.gamepak.read_byte(addr - GAMEPAK_MEM_START),
			_ => panic!("Unrecognized address: {}", addr)
		}
	}

	pub fn write_byte(&mut self, addr: u16, data: u8) {
		match addr {
			INTERNAL_RAM_START ... INTERNAL_RAM_END => self.ram.write_byte(addr - INTERNAL_RAM_START, data),
			PPU_CTRL_REG ... PPU_MIRROR_END => unimplemented!("PPU register access unimplemented."),
			APU_SQ1_VOL ... APU_DMC_LEN => unimplemented!("APU register access unimplemented."),
			APU_SND_CHN => unimplemented!("APU sound enable unimplemented."),
			OAM_DMA_REG => unimplemented!("DMA register access unimplemented."),
			GAMEPAD_JOY_1 ... GAMEPAD_JOY_2 => unimplemented!("Gamepad registers unimplemented."),
			GAMEPAK_MEM_START ... GAMEPAK_MEM_END => self.gamepak.write_byte(addr - GAMEPAK_MEM_START, data),
			_ => panic!("Unrecognized address: {}", addr)
		}
	}
}