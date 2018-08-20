mod mem_map;

use self::mem_map::*;
use bit_utils;

use std::path::Path;
use std::io::{self, Read};
use std::fs::File;

pub struct GamePak {
	//Sizes in number of bytes
	prg_rom_size: u32,
	chr_rom_size: u32,

	//Put these values elsewhere?
	pub contains_prg_ram: bool,
	pub has_trainer: bool,
	pub mapper_num: u8,

	prg_rom_bytes: Box<[u8]>,
	chr_rom_bytes: Box<[u8]>,
	prg_ram_bytes: Option<Box<[u8]>>,
}

#[derive(Debug)]
pub enum GamePakError {
	InvalidHeader,
}

impl GamePak {
	pub fn load_rom<P: AsRef<Path>>(path: P) -> Result<GamePak, GamePakError> {
		//TODO: Add iNES 2.0 support
		//TODO: Error handling
		let mut file = File::open(path).unwrap();
		let mut header_data: [u8; 16] = [0; 16];
		file.read_exact(&mut header_data).unwrap();
		if header_data[0] != 0x4E || header_data[1] != 0x45 || header_data[2] != 0x53 ||
			header_data[3] != 0x1A
			{
				//Header doesn't start with "NES\r" so file isn't a iNES ROM
				return Err(GamePakError::InvalidHeader);
			}
		let prg_rom_size = (header_data[4] as u32) * 16 * 1024;
		let chr_rom_size = (header_data[5] as u32) * 8 * 1024;
		let flag_6 = header_data[6];
		let flag_7 = header_data[7];
		let has_trainer = bit_utils::test_bit_u8(&flag_6, 2);
		if has_trainer {
			// DO trainer data
		}
		let mut prg_rom: Vec<u8> = vec![0; prg_rom_size as usize];
		let mut chr_rom: Vec<u8> = vec![0; chr_rom_size as usize];
		file.read_exact(prg_rom.as_mut_slice()).unwrap();
		debug!("PRG ROM size: {}", prg_rom.len()); //REMOVE

		if chr_rom_size > 0 {
			file.read_exact(chr_rom.as_mut_slice()).unwrap();
			debug!("CHR ROM size: {}", chr_rom.len());
		}

		// TODO: proper initializing of SRAM with file loading
		let mut prg_ram_bytes = vec![0xff; SRAM_SIZE as usize].into_boxed_slice();
		let contains_prg_ram = bit_utils::test_bit_u8(&flag_6, 1);
		let mut prg_ram = None;
		if contains_prg_ram {
			prg_ram = Some(prg_ram_bytes);
		}

		Ok(GamePak {
			prg_rom_size,
			chr_rom_size,
			contains_prg_ram,
			has_trainer,
			mapper_num: (flag_7 & 0xF0) | (flag_6 >> 4),
			prg_rom_bytes: prg_rom.into_boxed_slice(),
			chr_rom_bytes: chr_rom.into_boxed_slice(),
			prg_ram_bytes: prg_ram,
		})
	}

	// TODO: Implement SRAM loading for save files
//	pub fn load_sram<P: AsRef<Path>>(path: P) -> io::Result<Sram> {
//		let mut file = File::open(path)?;
//		let mut vec = Vec::new();
//		file.read_to_end(&mut vec)?;
//
//		let size = vec.len();
//		if size > MAX_SRAM_SIZE {
//			return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid SRAM size."));
//		}
//		let mut bytes = vec.into_boxed_slice();
//		Ok(Sram{
//			bytes,
//		})
//	}

	pub fn read_byte(&self, addr: u16) -> u8 {
		match self.mapper_num {
			0 => 0,
			_ => unimplemented!("Unimplemented mapper detected!")
		}
	}

	pub fn write_byte(&mut self, addr: u16, data: u8) {

	}
}
