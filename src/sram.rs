use std::path::Path;
use std::io::{self, Read};
use std::fs::File;

use mem_map::*;

pub const MAX_SRAM_SIZE: usize = SRAM_SIZE as usize;

pub struct Sram {
	bytes: Box<[u8]>,
}

impl Sram {
	pub fn new() -> Self {
		let mut sram_bytes = vec![0xff; MAX_SRAM_SIZE].into_boxed_slice();
		Sram {
			bytes: sram_bytes,
		}
	}

	pub fn load<P: AsRef<Path>>(path: P) -> io::Result<Sram> {
		let mut file = File::open(path)?;
		let mut vec = Vec::new();
		file.read_to_end(&mut vec)?;

		let size = vec.len();
		if size > MAX_SRAM_SIZE {
			return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid SRAM size."));
		}
		let mut bytes = vec.into_boxed_slice();
		Ok(Sram{
			bytes,
		})
	}
}