use super::*;

#[test]
fn initialized_correctly() {
    let chip = Chip8::new();
    assert_eq!(chip.stack, [0; 16]);
    assert_eq!(chip.stack_ptr, 0);
    assert_eq!(chip.index_reg, 0);

    // validate font start and finish
    assert_eq!(chip.memory[0x50], 0xF0, "Font starting position isn't correct");
    assert_eq!(chip.memory[0x9F], 0x80, "Font ending position isn't correct");
}

#[test]
fn op_1nnn_valid() {
    let mut chip = Chip8::new();
    assert_ne!(chip.program_counter, 0x343, "starting position should not equal the expected position prior to the jump");
    chip.op_1nnn(0x343);
    assert_eq!(chip.program_counter, 0x343);
}
#[test]
#[should_panic]
fn op_1nnn_out_of_range() {
    let mut chip = Chip8::new();
    chip.op_1nnn(0x1001);
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

#[test]
fn set_index_valid() {
    let mut chip = Chip8::new();
    assert_ne!(chip.index_reg, 0xABC);
    chip.op_annn(0xABC);
    assert_eq!(chip.index_reg, 0xABC);
}

/** Test that the VX register can have a value assigned to it */
#[test]
fn op_6xnn_valid() {
    let mut chip = Chip8::new();
    let expected = 0x3F;
    chip.op_6xnn(0x5, 0x3F);
    assert_eq!(expected, chip.var_registers[0x5]);
}

#[test]
fn op_7xnn_valid() {
    let mut chip = Chip8::new();
    chip.var_registers[0x5] = 0x5;
    let expected: u8 = 0x19;
    chip.op_7xnn(0x5, 0x14);
    assert_eq!(expected, chip.var_registers[5]);
}
#[test]
fn op_7xnn_valid_overflow() {
    let mut chip = Chip8::new();
    chip.var_registers[0x5] = 0x5;
    let expected: u8 = 0x4;
    chip.op_7xnn(0x5, 0xFF);
    assert_eq!(expected, chip.var_registers[5]);
}
