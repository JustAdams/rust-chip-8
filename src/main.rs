use std::fs;
use std::fs::File;
use std::io::prelude::*;

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

struct Chip8 {
    memory: [u8; 4096], // RAM
    stack : [u16; 16],
    stack_ptr: u8, // tracks position of most recent value
    index_reg: u16, // index register
    delay_timer: u8, // delay timer
    sound_timer: u8, // sound timer
    var_registers: [u8; 16], // variable registers
    program_counter: usize,
}
impl Chip8 {

    fn new() -> Self {
        // RAM
        let mut memory: [u8; 4096] = [0; 4096];

        let fonts: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];
        for i in 0..fonts.len() {
            memory[0x50 + i] = fonts[i];
        }

        Chip8 {
            memory,
            stack: [0; 16],
            stack_ptr: 0,
            index_reg: 0,
            delay_timer: 0,
            sound_timer: 0,
            var_registers: [0; 16],
            program_counter: PROGRAM_START,
        }
    }
    pub fn load_rom(&mut self, rom: ROM) {
        self.memory[PROGRAM_START..].copy_from_slice(&rom.memory);
        println!("ROM loaded");
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
        let nibbles = ((opcode & 0xF000) >> 12, (opcode & 0x0F00) >> 8, (opcode & 0x00F0) >> 4, (opcode & 0x000F) >> 0);

        // execute
        match nibbles {
            (0x0, 0x0, 0xE, 0x0) => { /* clear screen */ },
            (0x1, _, _, _) => { /* jump */  }
            (0x6, _, _, _) => { /* set */ }
            (0x7, _, _, _) => { /* add */ },
            _ => {}
        }

        break;
    }
}
