use chip::Chip8;
use rom::ROM;

mod chip;
mod chip_tests;
mod rom;

const PROGRAM_START: usize = 0x200; // starting position for ROM instructions

fn main() {
    let mut chip8 = Chip8::new();
  //  let mut display_buffer: [[bool; SCREEN_WIDTH]; SCREEN_HEIGHT] = [[false; SCREEN_WIDTH]; SCREEN_HEIGHT];
    let rom: ROM = ROM::new("roms/IBMLogo.ch8");
    chip8.load_rom(rom);

    println!("starting pc: {}", chip8.program_counter);
    // cycle
    loop {

        // fetch
        let opcode: u16 = (chip8.memory[chip8.program_counter] as u16) << 8 | (chip8.memory[chip8.program_counter + 1] as u16);

        // decode
        let nibbles: (u8, u8, u8, u8) = (((opcode & 0xF000) >> 12) as u8, ((opcode & 0x0F00) >> 8) as u8, ((opcode & 0x00F0) >> 4) as u8, ((opcode & 0x000F) >> 0) as u8);
    //    println!("nibbles: {:?}", nibbles);
        // execute
        match nibbles {
            (0x0, 0x0, 0xE, 0x0) => { chip8.clear_display() },
            (0x1, _, _, _) => { chip8.op_1nnn(opcode & 0xFFF); },
            (0x2, _, _, _) => { /* call */ },
            (0x3, _, _, _) => { /* skip */ },
            (0x4, _, _, _) => { /* skip */ },
            (0x5, _, _, 0x0) => { /* skip next if vx == vy */ },
            (0x6, _, _, _) => { chip8.op_6xnn(nibbles.1 as usize, (nibbles.2 << 4) | nibbles.3); },
            (0x7, _, _, _) => { chip8.op_7xnn(nibbles.1 as usize, (nibbles.2 << 4) | nibbles.3); },
            (0x8, _, _, 0x0) => { chip8.set(nibbles.1 as usize, chip8.var_registers[nibbles.2 as usize]); },
            (0x8, _, _, 0x1) => { chip8.or(nibbles.1 as usize, nibbles.2 as usize); },
            (0x8, _, _, 0x2) => { chip8.and(nibbles.1 as usize, nibbles.2 as usize); },
            (0x8, _, _, 0x3) => { /* XOR */ },
            (0x8, _, _, 0x4) => { chip8.add(nibbles.1 as usize, nibbles.2); },
            (0x8, _, _, 0x5) => { /* set VX to (VX - VY) */ },
            (0x8, _, _, 0x6) => { /* shift */ },
            (0x8, _, _, 0x7) => { /* set VX to (VX - VY) */ },
            (0x8, _, _, 0xE) => { /* shift */ },
            (0x9, _, _, 0x0) => { /* skips if vx != vy */ },
            (0xA, _, _, _) => { chip8.op_annn(opcode & 0x0FFF); },
            (0xB, _, _, _) => { /* jump with offset */ },
            (0xC, _, _, _) => { /* random */ },
            (0xD, _, _, _) => { chip8.draw(nibbles.1 as usize, nibbles.2 as usize, nibbles.3 as usize); },
            (0xE, _, 0x9, 0xE) => { /* skip if key */ },
            (0xE, _, 0xA, 0x1) => { /* skip if key */ },
            (0xF, _, 0x1, 0xE) => { chip8.add_i_index(nibbles.1 as usize); }
            _ => { chip8.program_counter += 2; }
        }


        // draw
       draw_screen(&chip8);
    }
}

fn draw_screen(chip8: &Chip8) {
    let black_square = '■';
    let white_square = '□';
    let width = 64;
    let height = 32;

    for row in 0..height - 1 {
        for col in 0..width - 1 {
           if chip8.display[row][col] == 0x1 {
               print!("{} ", black_square);
           } else {
               print!("{} ", white_square);
           }

        }
        println!();
    }

    println!();
}
