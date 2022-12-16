// Opcodes

// Load/store instructions
// LDA - Load Accumulator
// LDX - Load X Register
// LDY - Load Y Register
// STA - Store Accumulator
// STX - Store X Register
// STY - Store Y Register

#![allow(dead_code)] // TODO: Remove this when all unused opcodes are implemented
pub const LDA: u8 = 0xA9;
pub const LDX: u8 = 0xA2;
pub const LDY: u8 = 0xA0;
pub const STA: u8 = 0x8D;
pub const STX: u8 = 0x8E;
pub const STY: u8 = 0x8C;

// Register transfer instructions
// TAX - Transfer Accumulator to X
// TAY - Transfer Accumulator to Y
// TXA - Transfer X to Accumulator
// TYA - Transfer Y to Accumulator

pub const TAX: u8 = 0xAA;
pub const TAY: u8 = 0xA8;
pub const TXA: u8 = 0x8A;
pub const TYA: u8 = 0x98;
