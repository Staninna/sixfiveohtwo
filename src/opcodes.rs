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

// Load X
pub const LDX_IM: u8 = 0xA2;
pub const LDX_ZP: u8 = 0xA6;
pub const LDX_ZPY: u8 = 0xB6;
pub const LDX_ABS: u8 = 0xAE;
pub const LDX_ABSY: u8 = 0xBE;

// Load Y
pub const LDY_IM: u8 = 0xA0;
pub const LDY_ZP: u8 = 0xA4;
pub const LDY_ZPX: u8 = 0xB4;
pub const LDY_ABS: u8 = 0xAC;
pub const LDY_ABSX: u8 = 0xBC;
