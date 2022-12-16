// Opcodes

// Load/store instructions
pub const LDA: u8 = 0xA9; // Load Accumulator
pub const LDX: u8 = 0xA2; // Load X
pub const LDY: u8 = 0xA0; // Load Y
pub const STA: u8 = 0x8D; // Store Accumulator
pub const STX: u8 = 0x8E; // Store X
pub const STY: u8 = 0x8C; // Store Y

// Register transfer instructions
pub const TAX: u8 = 0xAA; // Transfer Accumulator to X
pub const TAY: u8 = 0xA8; // Transfer Accumulator to Y
pub const TXA: u8 = 0x8A; // Transfer X to Accumulator
pub const TYA: u8 = 0x98; // Transfer Y to Accumulator
