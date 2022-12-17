// Imports
use crate::memory::Memory;
use crate::opcodes::*;
use crate::registers::{Flag, Registers, Status};

// Constants
const NEGATIVE: Flag = Flag::Negative;
const OVERFLOW: Flag = Flag::Overflow;
const UNUSED: Flag = Flag::Unused;
const BREAK_COMMAND: Flag = Flag::BreakCommand;
const DECIMAL_MODE: Flag = Flag::DecimalMode;
const INTERRUPT: Flag = Flag::Interrupt;
const ZERO: Flag = Flag::Zero;
const CARRY: Flag = Flag::Carry;

// Enums

enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
}

// Structs

///////////////
// Processor //
///////////////

#[derive(Debug)]
pub struct Processor {
    registers: Registers,
    memory: Memory,
}

impl Processor {
    pub fn new(memory_size: u16, program: Option<Vec<u8>>) -> Self {
        // Make the memory and registers
        let registers = Registers::new();
        let mut memory = Memory::new(memory_size);

        // Load the program into memory
        if let Some(program) = program {
            memory.load(program, 0x0000); // DEBUG
        }

        // Return the processor
        Self { registers, memory }
    }

    pub fn run(&mut self) {
        // Fetch and execute instructions
        loop {
            let instruction = self.fetch8();
            self.execute(instruction);
        }
    }

    pub fn step(&mut self) {
        // Fetch and execute one instruction
        let instruction = self.fetch8();
        self.execute(instruction);
    }

    fn fetch8(&mut self) -> u8 {
        // Get the byte at the program counter
        let byte = self.memory.read(self.registers.pc);

        // Increment the program counter
        self.registers.pc += 1;

        // Return the byte
        byte
    }

    fn fetch16(&mut self) -> u16 {
        // Get the two bytes at the program counter least significant bit name low and high
        let high = self.fetch8();
        let low = self.fetch8();

        // Return the bytes combined
        u16::from_le_bytes([low, high])
    }

    fn write_flag(&mut self, flag: Flag, status: bool) {
        match flag {
            Flag::Negative => self.registers.status.set(Status::NEGATIVE, status),
            Flag::Overflow => self.registers.status.set(Status::OVERFLOW, status),
            Flag::Unused => self.registers.status.set(Status::UNUSED, status),
            Flag::BreakCommand => self.registers.status.set(Status::BREAK_COMMAND, status),
            Flag::DecimalMode => self.registers.status.set(Status::DECIMAL_MODE, status),
            Flag::Interrupt => self.registers.status.set(Status::INTERRUPT, status),
            Flag::Zero => self.registers.status.set(Status::ZERO, status),
            Flag::Carry => self.registers.status.set(Status::CARRY, status),
        }
    }

    fn execute(&mut self, opcode: u8) {
        // Execute the instruction
        match opcode {
            // Load/store instructions

            // Load accumulator
            LDA_IM => self.lda(AddressingMode::Immediate),
            LDA_ZP => self.lda(AddressingMode::ZeroPage),
            LDA_ZPX => self.lda(AddressingMode::ZeroPageX),
            LDA_ABS => self.lda(AddressingMode::Absolute),
            LDA_ABSX => self.lda(AddressingMode::AbsoluteX),
            LDA_ABSY => self.lda(AddressingMode::AbsoluteY),
            LDA_INDX => self.lda(AddressingMode::IndirectX),
            LDA_INDY => self.lda(AddressingMode::IndirectY),

            // Load X
            LDX_IM => self.ldx(AddressingMode::Immediate),
            LDX_ZP => self.ldx(AddressingMode::ZeroPage),
            LDX_ZPY => self.ldx(AddressingMode::ZeroPageY),
            LDX_ABS => self.ldx(AddressingMode::Absolute),
            LDX_ABSY => self.ldx(AddressingMode::AbsoluteY),

            // Load y
            LDY_IM => self.ldy(AddressingMode::Immediate),
            LDY_ZP => self.ldy(AddressingMode::ZeroPage),
            LDY_ZPX => self.ldy(AddressingMode::ZeroPageX),
            LDY_ABS => self.ldy(AddressingMode::Absolute),
            LDY_ABSX => self.ldy(AddressingMode::AbsoluteX),

            // Store accumulator
            STA_ZP => self.sta(AddressingMode::ZeroPage),
            STA_ZPX => self.sta(AddressingMode::ZeroPageX),
            STA_ABS => self.sta(AddressingMode::Absolute),
            STA_ABSX => self.sta(AddressingMode::AbsoluteX),
            STA_ABSY => self.sta(AddressingMode::AbsoluteY),
            STA_INDX => self.sta(AddressingMode::IndirectX),
            STA_INDY => self.sta(AddressingMode::IndirectY),

            // Store X
            STX_ZP => self.stx(AddressingMode::ZeroPage),
            STX_ZPY => self.stx(AddressingMode::ZeroPageY),
            STX_ABS => self.stx(AddressingMode::Absolute),

            // Unknow opcode
            _ => {
                panic!("Unknown opcode: {:#X}", opcode);
            }
        }
    }

    // Instructions

    // Load/store instructions

    // Load accumulator
    fn lda(&mut self, mode: AddressingMode) {
        let value = match mode {
            // Immediate
            AddressingMode::Immediate => self.fetch8(),

            // Zero page
            AddressingMode::ZeroPage => {
                let offset = self.fetch8();
                let address = 0x0000 + offset as u16;
                self.memory.read(address)
            }

            // Zero page X
            AddressingMode::ZeroPageX => {
                let x = self.registers.x;
                let offset = self.fetch8();
                let address = 0x0000 + offset as u16 + x as u16;
                self.memory.read(address)
            }

            // Absolute
            AddressingMode::Absolute => {
                let address = self.fetch16();
                self.memory.read(address)
            }

            // Absolute X
            AddressingMode::AbsoluteX => {
                let x = self.registers.x;
                let address = self.fetch16() + x as u16;
                self.memory.read(address)
            }

            // Absolute Y
            AddressingMode::AbsoluteY => {
                let y = self.registers.y;
                let address = self.fetch16() + y as u16;
                self.memory.read(address)
            }

            // Indirect X
            AddressingMode::IndirectX => {
                // Get the offset
                let x = self.registers.x;
                let offset = self.fetch8();

                // Get the pointer address
                let pointer_address = 0x0000 + offset as u16 + x as u16;
                let low = self.memory.read(pointer_address);
                let high = self.memory.read(pointer_address + 1);

                // Get the value
                let address = u16::from_le_bytes([low, high]);
                self.memory.read(address)
            }

            // Indirect Y
            AddressingMode::IndirectY => {
                // Get the offset
                let y = self.registers.y;
                let offset = self.fetch8();

                // Get the pointer address
                let address = 0x0000 + offset as u16;
                let low = self.memory.read(address);
                let high = self.memory.read(address + 1);

                // Get the value
                let address = u16::from_le_bytes([low, high]) + y as u16;
                self.memory.read(address)
            }

            // Unknow addressing mode
            _ => {
                unreachable!("Unreachable addressing mode")
            }
        };

        // Write accumulator
        self.registers.acc = value;

        // Write zero flag
        if value == 0x00 {
            self.write_flag(ZERO, true)
        } else {
            self.write_flag(ZERO, false)
        }

        // Write negative flag
        if value & 0x80 == 0x80 {
            self.write_flag(NEGATIVE, true)
        } else {
            self.write_flag(NEGATIVE, false)
        }
    }

    // Load X
    fn ldx(&mut self, mode: AddressingMode) {
        let value = match mode {
            // Immediate
            AddressingMode::Immediate => self.fetch8(),

            // Zero page
            AddressingMode::ZeroPage => {
                let offset = self.fetch8();
                let address = 0x0000 + offset as u16;
                self.memory.read(address)
            }

            // Zero page y
            AddressingMode::ZeroPageY => {
                let y = self.registers.y;
                let offset = self.fetch8();
                let address = 0x0000 + offset as u16 + y as u16;
                self.memory.read(address)
            }

            // Absolute
            AddressingMode::Absolute => {
                let address = self.fetch16();
                self.memory.read(address)
            }

            // Absolute Y
            AddressingMode::AbsoluteY => {
                let y = self.registers.y;
                let address = self.fetch16() + y as u16;
                self.memory.read(address)
            }

            // Unknow addressing mode
            _ => {
                unreachable!("Unreachable addressing mode")
            }
        };

        // Write X
        self.registers.x = value;

        // Write zero flag
        if value == 0x00 {
            self.write_flag(ZERO, true)
        } else {
            self.write_flag(ZERO, false)
        }

        // Write negative flag
        if value & 0x80 == 0x80 {
            self.write_flag(NEGATIVE, true)
        } else {
            self.write_flag(NEGATIVE, false)
        }
    }

    // Load Y
    fn ldy(&mut self, mode: AddressingMode) {
        let value = match mode {
            // Immediate
            AddressingMode::Immediate => self.fetch8(),

            // Zero page
            AddressingMode::ZeroPage => {
                let offset = self.fetch8();
                let address = 0x0000 + offset as u16;
                self.memory.read(address)
            }

            // Zero page X
            AddressingMode::ZeroPageX => {
                let x = self.registers.x;
                let offset = self.fetch8();
                let address = 0x0000 + offset as u16 + x as u16;
                self.memory.read(address)
            }

            // Absolute
            AddressingMode::Absolute => {
                let address = self.fetch16();
                self.memory.read(address)
            }

            // Absolute X
            AddressingMode::AbsoluteX => {
                let x = self.registers.x;
                let address = self.fetch16() + x as u16;
                self.memory.read(address)
            }

            // Unknow addressing mode
            _ => {
                unreachable!("Unreachable addressing mode")
            }
        };

        // Write Y
        self.registers.y = value;

        // Write zero flag
        if value == 0x00 {
            self.write_flag(ZERO, true)
        } else {
            self.write_flag(ZERO, false)
        }

        // Write negative flag
        if value & 0x80 == 0x80 {
            self.write_flag(NEGATIVE, true)
        } else {
            self.write_flag(NEGATIVE, false)
        }
    }

    // Store accumulator
    fn sta(&mut self, mode: AddressingMode) {
        let address = match mode {
            // Zero page
            AddressingMode::ZeroPage => {
                let offset = self.fetch8();
                0x0000 + offset as u16
            }

            // Zero page X
            AddressingMode::ZeroPageX => {
                let x = self.registers.x;
                let offset = self.fetch8();
                0x0000 + offset as u16 + x as u16
            }

            // Absolute
            AddressingMode::Absolute => self.fetch16(),

            // Absolute X
            AddressingMode::AbsoluteX => {
                let x = self.registers.x;
                self.fetch16() + x as u16
            }

            // Absolute Y
            AddressingMode::AbsoluteY => {
                let y = self.registers.y;
                self.fetch16() + y as u16
            }

            // Indirect X
            AddressingMode::IndirectX => {
                // Get the offset
                let x = self.registers.x;
                let offset = self.fetch8();

                // Get the pointer address
                let pointer_address = 0x0000 + offset as u16 + x as u16;
                let low = self.memory.read(pointer_address);
                let high = self.memory.read(pointer_address + 1);

                // Get the value
                u16::from_le_bytes([low, high])
            }

            // Indirect Y
            AddressingMode::IndirectY => {
                // Get the offset
                let y = self.registers.y;
                let offset = self.fetch8();

                // Get the pointer address
                let address = 0x0000 + offset as u16;
                let low = self.memory.read(address);
                let high = self.memory.read(address + 1);

                // Get the value
                u16::from_le_bytes([low, high]) + y as u16
            }

            // Unknow addressing mode
            _ => {
                unreachable!("Unreachable addressing mode")
            }
        };

        // Write value
        self.memory.write(address, self.registers.acc);
    }

    // Store X
    fn stx(&mut self, mode: AddressingMode) {
        let address = match mode {
            // Zero page
            AddressingMode::ZeroPage => {
                let offset = self.fetch8();
                0x0000 + offset as u16
            }

            // Zero page Y
            AddressingMode::ZeroPageY => {
                let y = self.registers.y;
                let offset = self.fetch8();
                0x0000 + offset as u16 + y as u16
            }

            // Absolute
            AddressingMode::Absolute => self.fetch16(),

            // Unknow addressing mode
            _ => {
                unreachable!("Unreachable addressing mode")
            }
        };

        // Write value
        self.memory.write(address, self.registers.x);
    }
}

// Unit tests: TODO
