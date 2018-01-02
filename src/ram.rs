use mem_map::*;

pub struct Ram {
	bytes: Box<[u8]>,
}

impl Ram {
	pub fn new() -> Self {
		let mut bytes: Box<[u8]> = vec![0; INTERNAL_RAM_SIZE as usize].into_boxed_slice();
		Ram {
			bytes
		}
	}

	pub fn read_byte(&self, addr: u16) -> u8 {
		0
	}

	pub fn write_byte(&mut self, addr: u16, data: u8) {

	}
}