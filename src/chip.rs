use crate::{PROGRAM_START, ROM};

pub struct Chip8 {
    pub memory: [u8; 4096], // RAM
    pub stack : [u16; 16],
    pub stack_ptr: u8, // tracks position of most recent value
    pub index_reg: u16, // index register
    pub delay_timer: u8, // delay timer
    pub sound_timer: u8, // sound timer
    pub var_registers: [u8; 16], // variable registers
    pub program_counter: usize,
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
        }
    }
    pub fn load_rom(&mut self, rom: ROM) {
        self.memory[PROGRAM_START..].copy_from_slice(&rom.memory);
        println!("ROM loaded");
    }

    pub fn clear_screen(&mut self) {
        panic!("Clear screen not implemented");
    }

    pub fn jump(&mut self, addr: u16) {
        if addr >= 0x1000 { panic!("Attempting to jump to an out of bounds location"); }
        self.program_counter = addr as usize;
    }


    /** 6XNN / 8XY0 - The value val is assigned to register v_reg */
    pub fn set(&mut self, v_reg: usize, val: u8) {
        if v_reg > 0xF { panic!("Invalid register accessed"); }
        self.var_registers[v_reg] = val;
    }

    /** 8XY1 - */
    pub fn or(&mut self, v_reg_one: usize, v_reg_two: usize) {
        if v_reg_one > 0xF || v_reg_two > 0xF { panic!("Invalid register accessed"); }
        let or_val: u8 = self.var_registers[v_reg_one] | self.var_registers[v_reg_two];
        self.var_registers[v_reg_one] = or_val;
    }

    /** 8XY2 - */
    pub fn and(&mut self, v_reg_one: usize, v_reg_two: usize) {
        if v_reg_one > 0xF || v_reg_two > 0xF { panic!("Invalid register accessed"); }
        let and_val: u8 = self.var_registers[v_reg_one] & self.var_registers[v_reg_two];
        self.var_registers[v_reg_one] = and_val;
    }

    /** 8XY4 - The value val is added to the current value at register v_reg */
    pub fn add(&mut self, v_reg: usize, val: u8) {
        if v_reg > 0xF { panic!("Invalid register accessed"); }
        self.var_registers[v_reg] += val;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jump_valid() {
        let mut chip = Chip8::new();
        chip.jump(0x343);
        assert_eq!(chip.program_counter, 0x343);
    }
    #[test]
    #[should_panic]
    fn jump_out_of_bounds() {
        let mut chip = Chip8::new();
        chip.jump(0x1001);
    }

    #[test]
    fn add_valid() {
        let mut chip = Chip8::new();
        chip.var_registers[5] = 0x5;

        chip.add(0x5, 25);
        assert_eq!(chip.var_registers[5], 0x1E);
    }
    #[test]
    #[should_panic]
    fn add_invalid_register() {
        let mut chip = Chip8::new();
        chip.add(0x10, 25);
    }

    #[test]
    fn set_valid() {
        let mut chip = Chip8::new();
        chip.var_registers[5] = 0x5;
        chip.set(0x5, 25);
        assert_eq!(chip.var_registers[5], 0x19);
    }

    #[test]
    fn or_valid() {
        let mut chip = Chip8::new();
        chip.var_registers[0x5] = 0x5;
        chip.var_registers[0x1] = 0x3;
        chip.or(0x5, 0x1);
        let expected = 0x5 | 0x3;
        assert_eq!(expected, chip.var_registers[0x5]);
    }
    #[test]
    fn and_valid() {
        let mut chip = Chip8::new();
        chip.var_registers[0x5] = 0x5;
        chip.var_registers[0x1] = 0x3;
        chip.and(0x5, 0x1);
        let expected = 0x5 & 0x3;
        assert_eq!(expected, chip.var_registers[0x5]);
    }
}