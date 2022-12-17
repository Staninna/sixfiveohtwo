// Opcodes

// Load/store instructions

// Load accumulator
pub const LDA_IM: u8 = 0xA9;
pub const LDA_ZP: u8 = 0xA5;
pub const LDA_ZPX: u8 = 0xB5;
pub const LDA_ABS: u8 = 0xAD;
pub const LDA_ABSX: u8 = 0xBD;
pub const LDA_ABSY: u8 = 0xB9;
pub const LDA_INDX: u8 = 0xA1;
pub const LDA_INDY: u8 = 0xB1;
