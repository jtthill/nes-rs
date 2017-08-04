mod cpu;
mod rom;
mod bit_utils;
mod mem_map;
mod interconnect;

use std::env;

fn main() {
    println!("Hello, world!");
    let inter: interconnect::Interconnect;
    if let Some(rom_file_name) = env::args().nth(1) {
        if let Ok(rom) = rom::Rom::new(rom_file_name) {
            // Create memory with Rom object that was created
        } else {
            panic!("Not given proper iNES file.");
        }
    } else {
        panic!("No arguments given.");
    }

    let cpu = cpu::Cpu::new();
    println!("Cpu: {:#?}", cpu);
}
