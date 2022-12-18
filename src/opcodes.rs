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

// Store accumulator
pub const STA_ZP: u8 = 0x85;
pub const STA_ZPX: u8 = 0x95;
pub const STA_ABS: u8 = 0x8D;
pub const STA_ABSX: u8 = 0x9D;
pub const STA_ABSY: u8 = 0x99;
pub const STA_INDX: u8 = 0x81;
pub const STA_INDY: u8 = 0x91;

// Store X
pub const STX_ZP: u8 = 0x86;
pub const STX_ZPY: u8 = 0x96;
pub const STX_ABS: u8 = 0x8E;

// Store Y
pub const STY_ZP: u8 = 0x84;
pub const STY_ZPX: u8 = 0x94;
pub const STY_ABS: u8 = 0x8C;

// Register transfers

// Transfer accumulator to X
pub const TAX: u8 = 0xAA;

// Transfer accumulator to Y
pub const TAY: u8 = 0xA8;

// Transfer X to accumulator
pub const TXA: u8 = 0x8A;

// Transfer Y to accumulator
pub const TYA: u8 = 0x98;
