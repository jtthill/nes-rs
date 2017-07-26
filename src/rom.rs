use std::path::Path;
use std::fs;
use std::io::Read;

pub struct Rom {
    raw: Box<[u8]>
}

impl Rom {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        //TODO: Error Handling
        let mut file = fs::File::open(path).unwrap();
        let mut file_buf = Vec::new();
        file.read_to_end(&mut file_buf).unwrap();
        Rom {
            raw: file_buf.into_boxed_slice()
        }
        
    }
}