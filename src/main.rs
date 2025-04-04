use std::fs::File;
use std::io::prelude::*;
use chip::Chip8;

mod chip;

const PROGRAM_START: usize = 0x200; // starting position for ROM instructions
const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;

struct ROM {
    memory: [u8; 3584],
}
impl ROM {
    fn new(file_path: &str) -> Self {
        let mut file = File::open(file_path).expect("Unable to open ROM");
        let mut buffer: [u8; 3584] = [0; 3584];
        file.read(&mut buffer).expect("Unable to read ROM file");

        ROM {
            memory: buffer
        }
    }
}

fn main() {
    let mut chip8 = Chip8::new();
  //  let mut display_buffer: [[bool; SCREEN_WIDTH]; SCREEN_HEIGHT] = [[false; SCREEN_WIDTH]; SCREEN_HEIGHT];
    let rom: ROM = ROM::new("roms/IBMLogo.ch8");
    chip8.load_rom(rom);


    // cycle
    loop {

        // fetch
        let opcode: u16 = (chip8.memory[chip8.program_counter] as u16) << 8 | (chip8.memory[chip8.program_counter as usize + 1] as u16);
        println!("{:?}", opcode);


        // decode
        let nibbles: (u8, u8, u8, u8) = (((opcode & 0xF000) >> 12) as u8, ((opcode & 0x0F00) >> 8) as u8, ((opcode & 0x00F0) >> 4) as u8, ((opcode & 0x000F) >> 0) as u8);

        // execute
        match nibbles {
            (0x0, 0x0, 0xE, 0x0) => { /* clear screen */ },
            (0x1, _, _, _) => { /* jump */  },
            (0x3, _, _, _) => { /* skip */ },
            (0x4, _, _, _) => { /* skip */ },
            (0x6, _, _, _) => { /* set */ },
            (0x7, _, _, _) => { /* add */ },
            (0x8, _, _, 0x0) => { chip8.set(nibbles.1 as usize, nibbles.2); },
            (0x8, _, _, 0x1) => { chip8.or(nibbles.1 as usize, nibbles.2 as usize); },
            (0x8, _, _, 0x2) => { chip8.and(nibbles.1 as usize, nibbles.2 as usize); },
            (0x8, _, _, 0x3) => { /* XOR */ },
            (0x8, _, _, 0x4) => { chip8.add(nibbles.1 as usize, nibbles.2); },
            _ => {}
        }

        break;
    }
}
