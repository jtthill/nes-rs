mod cpu;
mod rom;
mod bit_utils;
mod mem_map;

use std::env;

fn main() {
    println!("Hello, world!");
    let rom_file_name = env::args().nth(1).unwrap();

    let rom = rom::Rom::new(rom_file_name);

    let cpu = cpu::Cpu::new();
    println!("Cpu: {:#?}", cpu);
}
