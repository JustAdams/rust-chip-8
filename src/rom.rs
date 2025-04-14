use std::fs::File;
use std::io::Read;

pub struct ROM {
    pub memory: [u8; 3584],
}

impl ROM {
    pub fn new(file_path: &str) -> Self {
        let mut file = File::open(file_path).expect("Unable to open ROM");
        let mut buffer: [u8; 3584] = [0; 3584];
        file.read(&mut buffer).expect("Unable to read ROM file");

        ROM {
            memory: buffer
        }
    }
}