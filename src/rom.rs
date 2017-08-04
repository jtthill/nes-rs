use bit_utils;

use std::path::Path;
use std::fs;
use std::io::Read;

pub struct Rom {
    //Sizes in number of bytes
    prg_rom_size: u32,
    chr_rom_size: u32,

    //Put these values elsewhere?
    contains_prg_ram: bool,
    has_trainer: bool,
    mapper_num: u8,

    prg_rom: Box<[u8]>,
    chr_rom: Box<[u8]>,
}

pub enum RomError {
    InvalidHeader,
}

impl Rom {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Rom, RomError> {
        //TODO: Add iNES 2.0 support
        //TODO: Error handling
        let mut file = fs::File::open(path).unwrap();
        let mut header_data: [u8; 16] = [0; 16];
        file.read_exact(&mut header_data).unwrap();
        if header_data[0] != 0x4E || header_data[1] != 0x45 || header_data[2] != 0x53 ||
            header_data[3] != 0x1A
        {
            //Header doesn't start with "NES\r" so file isn't a iNES ROM
            return Err(RomError::InvalidHeader);
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
        println!("PRG size: {}", prg_rom.len()); //REMOVE

        if chr_rom_size > 0 {
            file.read_exact(chr_rom.as_mut_slice()).unwrap();
            println!("CHR size: {}", chr_rom.len());
        }

        Ok(Rom {
            prg_rom_size: prg_rom_size,
            chr_rom_size: chr_rom_size,
            contains_prg_ram: bit_utils::test_bit_u8(&flag_6, 1),
            has_trainer: has_trainer,
            mapper_num: (flag_7 & 0xF0) | (flag_6 >> 4),
            prg_rom: prg_rom.into_boxed_slice(),
            chr_rom: chr_rom.into_boxed_slice(),
        })

    }
}
