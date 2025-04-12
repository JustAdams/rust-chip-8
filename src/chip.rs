use crate::{PROGRAM_START, ROM};

const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;

pub struct Chip8 {
    pub memory: [u8; 4096], // RAM
    pub stack : [u16; 16],
    pub stack_ptr: u8, // tracks position of most recent value
    pub index_reg: u16, // index register
    pub delay_timer: u8, // delay timer
    pub sound_timer: u8, // sound timer
    pub var_registers: [u8; 16], // variable registers
    pub program_counter: usize,
    pub display: [[u8; SCREEN_HEIGHT]; SCREEN_WIDTH],
}

impl Chip8 {

    pub fn new() -> Self {
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
            display: [[0; SCREEN_HEIGHT];  SCREEN_WIDTH]
        }
    }
    pub fn load_rom(&mut self, rom: ROM) {
        self.memory[PROGRAM_START..].copy_from_slice(&rom.memory);
        println!("ROM loaded");
    }

    /** 00E0 - Clears the screen */
    pub fn clear_display(&mut self) {
        self.display = [[0; SCREEN_HEIGHT];  SCREEN_WIDTH];
        self.program_counter += 2;
    }

    /** 1NNN - Jumps the PC to NNN */
    pub fn op_1NNN(&mut self, addr: u16) {
        if addr >= 0x1000 { panic!("Attempting to jump to an out of bounds location"); }
        self.program_counter = addr as usize;
    }


    /** 6XNN / 8XY0 - The value val is assigned to register v_reg */
    pub fn set(&mut self, v_reg: usize, val: u8) {
        if v_reg > 0xF { panic!("Invalid register accessed"); }
        self.var_registers[v_reg] = val;
        self.program_counter += 2;
    }

    /** Sets VX to the value given */
    pub fn op_6XNN(&mut self, vx: usize, val: u8) {
        self.var_registers[vx] = val;
        self.program_counter += 2;
    }

    /** Adds the value NN to VX */
    pub fn op_7XNN(&mut self, vx: usize, val: u8) {
        self.var_registers[vx] = self.var_registers[vx].wrapping_add(val);
        self.program_counter += 2;
    }

    /** 8XY1 - */
    pub fn or(&mut self, v_reg_one: usize, v_reg_two: usize) {
        if v_reg_one > 0xF || v_reg_two > 0xF { panic!("Invalid register accessed"); }
        let or_val: u8 = self.var_registers[v_reg_one] | self.var_registers[v_reg_two];
        self.var_registers[v_reg_one] = or_val;
        self.program_counter += 2;
    }

    /** 8XY2 - */
    pub fn and(&mut self, v_reg_one: usize, v_reg_two: usize) {
        if v_reg_one > 0xF || v_reg_two > 0xF { panic!("Invalid register accessed"); }
        let and_val: u8 = self.var_registers[v_reg_one] & self.var_registers[v_reg_two];
        self.var_registers[v_reg_one] = and_val;
        self.program_counter += 2;
    }

    /** 8XY4 - The value val is added to the current value at register v_reg */
    pub fn add(&mut self, v_reg: usize, val: u8) {
        if v_reg > 0xF { panic!("Invalid register accessed"); }
        self.var_registers[v_reg] += val;
        self.program_counter += 2;
    }

    /** ANNN - Sets the index register I to value NNN */
    pub fn set_index(&mut self, opcode: u16) {
        self.index_reg = opcode & 0x0FFF;
        self.program_counter += 2;
    }

    /** DXYN - Draws a sprite from memory to the VX and VY coordinates */
    pub fn draw(&mut self, vx: usize, vy: usize, n: usize) {
        println!("drawing");
        let x_coord = self.var_registers[vx] % SCREEN_WIDTH as u8;
        let y_coord = self.var_registers[vy] % SCREEN_HEIGHT as u8;

        for row in 0..n {
            let sprite_row = self.memory[self.index_reg as usize + row];
            for col in 0..8 {
                let pixel = sprite_row & (0x80 >> col);
                self.display[x_coord as usize][y_coord as usize] = pixel;
            }
        }
        self.program_counter += 2;
    }

    /** FX1E - Add to I index */
    pub fn add_i_index(&mut self, v_reg: usize) {
        panic!("not implemented");
    }
}
