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

    fn read_flag(&self, flag: Flag) -> bool {
        match flag {
            Flag::Negative => self.registers.status.contains(Status::NEGATIVE),
            Flag::Overflow => self.registers.status.contains(Status::OVERFLOW),
            Flag::Unused => self.registers.status.contains(Status::UNUSED),
            Flag::BreakCommand => self.registers.status.contains(Status::BREAK_COMMAND),
            Flag::DecimalMode => self.registers.status.contains(Status::DECIMAL_MODE),
            Flag::Interrupt => self.registers.status.contains(Status::INTERRUPT),
            Flag::Zero => self.registers.status.contains(Status::ZERO),
            Flag::Carry => self.registers.status.contains(Status::CARRY),
        }
    }

    fn execute(&mut self, opcode: u8) {
        // Execute the instruction
        match opcode {
            // Load/Store
            LDA => self.lda(), // Load Accumulator
            LDX => self.ldx(), // Load X
            LDY => self.ldy(), // Load Y
            STA => self.sta(), // Store Accumulator
            STX => self.stx(), // Store X
            STY => self.sty(), // Store Y

            // Transfer
            TAX => self.tax(), // Transfer Accumulator to X
            TAY => self.tay(), // Transfer Accumulator to Y
            TXA => self.txa(), // Transfer X to Accumulator
            TYA => self.tya(), // Transfer Y to Accumulator

            // Unknown opcode
            _ => {
                panic!("Unknown opcode: {:#X}", opcode);
            }
        }
    }

    // Instructions

    // Load/Store

    // Load Accumulator
    fn lda(&mut self) {
        // Get the address
        let address = self.fetch16();
        let byte = self.memory.read(address);
        self.registers.acc = byte;

        // Set flags
        if byte == 0x00 {
            self.write_flag(ZERO, true)
        } else {
            self.write_flag(ZERO, false)
        }

        if byte & 0x80 == 0x80 {
            self.write_flag(NEGATIVE, true)
        } else {
            self.write_flag(NEGATIVE, false)
        }
    }

    // Load X
    fn ldx(&mut self) {
        // Get the address
        let address = self.fetch16();
        let byte = self.memory.read(address);
        self.registers.x = byte;

        // Set flags
        if byte == 0x00 {
            self.write_flag(ZERO, true)
        } else {
            self.write_flag(ZERO, false)
        }

        if byte & 0x80 == 0x80 {
            self.write_flag(NEGATIVE, true)
        } else {
            self.write_flag(NEGATIVE, false)
        }
    }

    // Load Y
    fn ldy(&mut self) {
        let address = self.fetch16();
        let byte = self.memory.read(address);
        self.registers.y = byte;

        // Set flags
        if byte == 0x00 {
            self.write_flag(ZERO, true)
        } else {
            self.write_flag(ZERO, false)
        }

        if byte & 0x80 == 0x80 {
            self.write_flag(NEGATIVE, true)
        } else {
            self.write_flag(NEGATIVE, false)
        }
    }

    // Store Accumulator
    fn sta(&mut self) {
        let address = self.fetch16();
        let acc = self.registers.acc;
        self.memory.write(address, acc)
    }

    // Store X
    fn stx(&mut self) {
        let address = self.fetch16();
        let x = self.registers.x;
        self.memory.write(address, x)
    }

    // Store Y
    fn sty(&mut self) {
        let address = self.fetch16();
        let y = self.registers.y;
        self.memory.write(address, y)
    }

    // Transfer

    // Transfer Accumulator to X
    fn tax(&mut self) {
        self.registers.x = self.registers.acc;
    }

    // Transfer Accumulator to Y
    fn tay(&mut self) {
        self.registers.y = self.registers.acc;
    }

    // Transfer X to Accumulator
    fn txa(&mut self) {
        self.registers.acc = self.registers.x;
    }

    // Transfer Y to Accumulator
    fn tya(&mut self) {
        self.registers.acc = self.registers.y;
    }
}

// Unit tests

#[cfg(test)]
mod tests {
    use super::*;

    // Flags tests

    #[test]
    // Test the Negative flag true
    fn test_negative_flag_true() {
        let mut processor = Processor::new(
            0x00FF,
            Some(vec![
                LDA, 0x00, 0x03, // Load value from memory address $0003 into A (0xFF)
                0xFF, // ---------- Hardcoded value at $0003
            ]),
        );

        processor.step();
        assert_eq!(processor.registers.status.contains(Status::NEGATIVE), true);
    }

    #[test]
    // Test the Negative flag false
    fn test_negative_flag_false() {
        let mut processor = Processor::new(
            0x00FF,
            Some(vec![
                LDA, 0x00, 0x03, // Load value from memory address $0003 into A (0x0F)
                0x0F, // ---------- Hardcoded value at $0003
            ]),
        );

        processor.step();
        assert_eq!(processor.registers.status.contains(Status::NEGATIVE), false);
    }

    #[test]
    // Test the Zero flag true
    fn test_zero_flag_true() {
        let mut processor = Processor::new(
            0x00FF,
            Some(vec![
                LDA, 0x00, 0x03, // Load value from memory address $0003 into A (0x00)
                0x00, // ---------- Hardcoded value at $0003
            ]),
        );

        processor.step();
        assert_eq!(processor.registers.status.contains(Status::ZERO), true);
    }

    #[test]
    // Test the Zero flag false
    fn test_zero_flag_false() {
        let mut processor = Processor::new(
            0x00FF,
            Some(vec![
                LDA, 0x00, 0x03, // Load value from memory address $0003 into A (0x0F)
                0x0F, // ---------- Hardcoded value at $0003
            ]),
        );

        processor.step();
        assert_eq!(processor.registers.status.contains(Status::ZERO), false);
    }

    // Instruction tests

    #[test]
    // Test the Load Accumulator (LDA) opcode
    fn test_lda() {
        let mut processor = Processor::new(
            0x00FF,
            Some(vec![
                LDA, 0x00, 0x03, // Load value from memory address $0003 into A (0xFF)
                0xFF, // ---------- Hardcoded value at $0004
            ]),
        );

        processor.step();
        assert_eq!(processor.registers.acc, 0xFF);
    }

    #[test]
    // Test the Load X (LDX) opcode
    fn test_ldx() {
        let mut processor = Processor::new(
            0x00FF,
            Some(vec![
                LDX, 0x00, 0x03, // Load value from memory address $0003 into X (0xFF)
                0xFF, // ---------- Hardcoded value at $0004
            ]),
        );

        processor.step();
        assert_eq!(processor.registers.x, 0xFF);
    }

    #[test]
    // Test the Load Y (LDY) opcode
    fn test_ldy() {
        let mut processor = Processor::new(
            0x00FF,
            Some(vec![
                LDY, 0x00, 0x03, // Load value from memory address $0003 into Y (0xFF)
                0xFF, // ---------- Hardcoded value at $0004
            ]),
        );

        processor.step();
        assert_eq!(processor.registers.y, 0xFF);
    }

    #[test]
    // Test the Store Accumulator (STA) opcode
    fn test_sta() {
        let mut processor = Processor::new(
            0x00FF,
            Some(vec![
                LDA, 0x00, 0x06, // Load value from memory address $0006 into A (0xFF)
                STA, 0x00, 0x07, // Store value in A at memory address $0007
                0xFF, // ---------- Hardcoded value at $0006
            ]),
        );

        processor.step();
        processor.step();
        assert_eq!(processor.memory.read(0x0007), 0xFF);
    }

    #[test]
    // Test the Store X (STX) opcode
    fn test_stx() {
        let mut processor = Processor::new(
            0x00FF,
            Some(vec![
                LDX, 0x00, 0x06, // Load value from memory address $0006 into X (0xFF)
                STX, 0x00, 0x07, // Store value in X at memory address $0007
                0xFF, // ---------- Hardcoded value at $0006
            ]),
        );

        processor.step();
        processor.step();
        assert_eq!(processor.memory.read(0x0007), 0xFF);
    }

    #[test]
    // Test the Store Y (STY) opcode
    fn test_sty() {
        let mut processor = Processor::new(
            0x00FF,
            Some(vec![
                LDY, 0x00, 0x06, // Load value from memory address $0006 into Y (0xFF)
                STY, 0x00, 0x07, // Store value in Y at memory address $0007
                0xFF, // ---------- Hardcoded value at $0006
            ]),
        );

        processor.step();
        processor.step();
        assert_eq!(processor.memory.read(0x0007), 0xFF);
    }

    #[test]
    // Test the Transfer Accumulator to X (TAX) opcode
    fn test_tax() {
        let mut processor = Processor::new(
            0x00FF,
            Some(vec![
                LDA, 0x00, 0x04, // Load value from memory address $0004 into A (0xFF)
                TAX,  // ---------- Transfer value in A to X
                0xFF, // ---------- Hardcoded value at $0006
            ]),
        );

        processor.step();
        processor.step();
        assert_eq!(processor.registers.x, 0xFF);
    }

    #[test]
    // Test the Transfer Accumulator to Y (TAY) opcode
    fn test_tay() {
        let mut processor = Processor::new(
            0x00FF,
            Some(vec![
                LDA, 0x00, 0x04, // Load value from memory address $0004 into A (0xFF)
                TAY,  // ---------- Transfer value in A to Y
                0xFF, // ---------- Hardcoded value at $0006
            ]),
        );

        processor.step();
        processor.step();
        assert_eq!(processor.registers.y, 0xFF);
    }

    #[test]
    // Test the Transfer X to Accumulator (TXA) opcode
    fn test_txa() {
        let mut processor = Processor::new(
            0x00FF,
            Some(vec![
                LDX, 0x00, 0x04, // Load value from memory address $0004 into X (0xFF)
                TXA,  // ---------- Transfer value in X to A
                0xFF, // ---------- Hardcoded value at $0006
            ]),
        );

        processor.step();
        processor.step();
        assert_eq!(processor.registers.acc, 0xFF);
    }

    #[test]
    // Test the Transfer Y to Accumulator (TYA) opcode
    fn test_tya() {
        let mut processor = Processor::new(
            0x00FF,
            Some(vec![
                LDY, 0x00, 0x04, // Load value from memory address $0004 into Y (0xFF)
                TYA,  // ---------- Transfer value in Y to A
                0xFF, // ---------- Hardcoded value at $0006
            ]),
        );

        processor.step();
        processor.step();
        assert_eq!(processor.registers.acc, 0xFF);
    }
}
