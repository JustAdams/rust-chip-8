use crate::PROGRAM_START;
use crate::rom::ROM;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

pub struct Chip8 {
    pub memory: [u8; 4096], // 4kb of RAM
    pub stack: [u16; 16],
    pub stack_ptr: u8, // tracks position of most recent value
    pub index_reg: u16, // index register
    pub delay_timer: u8, // delay timer
    pub sound_timer: u8, // sound timer
    pub var_registers: [u8; 16], // variable registers
    pub program_counter: usize,
    pub display: [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT],
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
            display: [[0x00; SCREEN_WIDTH]; SCREEN_HEIGHT],
        }
    }
    pub fn load_rom(&mut self, rom: ROM) {
        self.memory[PROGRAM_START..].copy_from_slice(&rom.memory);
    }

    /** 00E0 - Clears the screen */
    pub fn clear_display(&mut self) {
        self.display = [[0x00; SCREEN_WIDTH]; SCREEN_HEIGHT];
    }

    /** 1NNN - Jumps the PC to NNN */
    pub fn op_1nnn(&mut self, addr: u16) {
        if addr >= 0x1000 { panic!("Attempting to jump to an out of bounds location"); }
        self.program_counter = addr as usize;
    }

    /** 2NNN - Call */


    /** 3XNN - Skip instruction if the value in var_registers[VX] == NN */
    pub fn op_3xnn(&mut self, vx: usize, nn: u8) {
        if vx > 0xF { panic!("Invalid register accessed"); }
        if self.var_registers[vx] == nn {
            self.program_counter += 2;
        }
    }

    /** 4XNN - Skip instruction if the value in var_registers[VX] != NN */
    pub fn op_4xnn(&mut self, vx: usize, nn: u8) {
        if vx > 0xF { panic!("Invalid register accessed"); }
        if self.var_registers[vx] != nn {
            self.program_counter += 2;
        }
    }

    /** 5XY0 - Skip if var_registers[VX] == var_registers[VY] */
    pub fn op_5xy0(&mut self, vx: usize, vy: usize) {
        if vx > 0xF || vy > 0xF { panic!("Invalid register accessed"); }
        if self.var_registers[vx] == self.var_registers[vy] {
            self.program_counter += 2;
        }
    }

    /** 6XNN - Sets VX to the value given */
    pub fn op_6xnn(&mut self, vx: usize, val: u8) {
        self.var_registers[vx] = val;
    }

    /** Adds a value to VX */
    pub fn op_7xnn(&mut self, vx: usize, val: u8) {
        self.var_registers[vx] = self.var_registers[vx].wrapping_add(val);
    }

    /** 8XY0 - Sets VX to the value of VY */
    pub fn op_8xy0(&mut self, vx: usize, vy: usize) {
        if vx > 0xF || vy > 0xF { panic!("Invalid register accessed"); }
        self.var_registers[vx] = self.var_registers[vy];
    }

    /** 8XY1 - Sets VX to VX ^ VY */
    pub fn op_8xy1(&mut self, vx: usize, vy: usize) {
        if vx > 0xF || vy > 0xF { panic!("Invalid register accessed"); }
        let or_val: u8 = self.var_registers[vx] | self.var_registers[vy];
        self.var_registers[vx] = or_val;
    }

    /** 8XY2 - VX is set to VX & VY */
    pub fn op_8xy2(&mut self, vx: usize, vy: usize) {
        if vx > 0xF || vy > 0xF { panic!("Invalid register accessed"); }
        let and_val: u8 = self.var_registers[vx] & self.var_registers[vy];
        self.var_registers[vx] = and_val;
    }

    /** 8XY3 - VX is set to VX ^ VY */
    pub fn op_8xy3(&mut self, vx: usize, vy: usize) {
        self.var_registers[vx] ^= self.var_registers[vy];
    }

    /** 8XY4 - The value val is added to the current value at register v_reg */
    pub fn op_8xy4(&mut self, vx: usize, vy: usize) {
        if vx > 0xF || vy > 0xF { panic!("Invalid register accessed"); }
        self.var_registers[vx] += self.var_registers[vy];
    }

    /** 8XY5 - Sets VX to [VX] - [VY] */
    pub fn op_8xy5(&mut self, vx: usize, vy: usize) {
        if vx > 0xF || vy > 0xF { panic!("Invalid register accessed"); }
        self.var_registers[vx] -= self.var_registers[vy];
    }

    /** 8XY5 - Sets VX to [VY] - [VX] */
    pub fn op_8xy7(&mut self, vx: usize, vy: usize) {
        if vx > 0xF || vy > 0xF { panic!("Invalid register accessed"); }
        self.var_registers[vx] = self.var_registers[vy] - self.var_registers[vx];
    }

    /** 9XY0 - Skip if var_register[VX] == var_register[VY] */
    pub fn op_9xy0(&mut self, vx: usize, vy: usize) {
        if vx > 0xF || vy > 0xF { panic!("Attempting to jump to an out of bounds location"); }
        if self.var_registers[vx] != self.var_registers[vy] {
            self.program_counter += 2;
        }
    }

    /** ANNN - Sets the index register I to value NNN */
    pub fn op_annn(&mut self, nnn: u16) {
        self.index_reg = nnn;
    }

    /** BNNN - Jumps to address NNN plus value in var_registers[V0] */
    pub fn op_bnnn(&mut self, nnn: u16) {
        let next_addr = (nnn + self.var_registers[0x0] as u16) as usize;
        self.program_counter = next_addr;
    }

    /** CXNN - Generate a random number, & with NN, and set to VX */
    pub fn op_cxnn(&mut self, x: usize, nn: u8) {
        let rand_num = rand::random::<u8>();
        let new_val = rand_num & nn;
        self.var_registers[x] = new_val;
    }

    /** DXYN - Sets a sprite from memory to the VX and VY coordinates */
    pub fn draw(&mut self, vx: usize, vy: usize, n: usize) {
        self.var_registers[0xF] = 0x0;
        for row in 0..n {
            let curr_idx = self.index_reg as usize + row;
            let sprite_byte = self.memory[curr_idx];

            let y_coord = (self.var_registers[vy] as usize + row) % SCREEN_HEIGHT;
            for col in 0..8 {
                let x_coord = (self.var_registers[vx] as usize + col) % SCREEN_WIDTH;
                let bit = sprite_byte >> (7 - col) & 1;
                self.var_registers[0xF] |= bit & self.display[y_coord][x_coord];
                self.display[y_coord][x_coord] ^= bit;
            }
        }
    }

    /** FX0A - Get key loops until a key is pressed */
    pub fn op_fx0a(&mut self) {
        // check for key input
        self.program_counter -= 2;
    }

    /** FX1E - Add to I index */
    pub fn add_i_index(&mut self, v_reg: usize) {
        panic!("not implemented");
    }
}
