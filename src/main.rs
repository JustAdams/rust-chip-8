use std::thread::sleep;
use std::time::Duration;

// display in pixels
const HEIGHT: u8 = 32;
const WIDTH: u8 = 64;
const PROGRAM_START: u16 = 0x200;

fn main() {
    let display_height = HEIGHT;
    let display_width = WIDTH;

    // component setup
    // chip-8 program loaded starting at address 0x200 (512)
    let mut memory: [u8; 4096] = [0; 4096];

    // program counter - probably making it too large
    let mut pc: u32 = 0x200; // starting at address 0x200 (512)

    // index register - points to locations in memory
    let mut idx_reg: u32;

    let mut delay_timer: u8 = 0; // decremented by one 60 times per second until it reaches 0
    let mut sound_timer: u8 = 0; // beeps if its not zero

    // general purpose variable registers V0 through VF
    // VF is also often used as a flag register
    let mut var_regs: [u8; 16] = [0; 16];

    // fonts get stored in memory
    let font: [u8; 80] = [
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

    // todo: eventually make this use emulated memory
    let mut instruction_stack: Vec<u8> = Vec::new();



    // program loop should run at 60fps
    loop {
        // fetch opcode from memory at the current PC
        if pc >= 0x1000 {
            panic!("Out of memory!");
        }

        // an instruction is two successive bytes
        let curr_opcode = memory[pc as usize];
        pc += 1;
        let curr_instr = memory[pc as usize];
        pc += 1; // always increment by two

        // decode the instruction
        println!("num: {}", curr_opcode & 0);

        // execute the instruction
        match curr_opcode {
            0x00E0 => {}, // clear screen
            _ => {}
        }

        // slow down the program
        sleep(Duration::from_millis(1000));
    }
}
