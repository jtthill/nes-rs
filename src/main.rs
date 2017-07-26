use std::path::Path;
use std::fs;
use std::io::Read;
use std::env;


fn main() {
    println!("Hello, world!");
    let rom_file_name = env::args().nth(1).unwrap();

    let rom = read_bin(rom_file_name);
    println!("Goodbye!");
}

fn read_bin<P: AsRef<Path>>(path: P) -> Box<[u8]> {
    //TODO: Error Handling
    let mut file = fs::File::open(path).unwrap();
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();
    file_buf.into_boxed_slice()
}
