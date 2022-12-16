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
            memory.load(program, 0x8000);
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
        let opcode = self.memory.read(self.registers.pc);

        // Increment the program counter
        self.registers.pc += 1;

        // Return the byte
        opcode
    }

    fn fetch16(&mut self) -> u16 {
        // Get the two bytes at the program counter
        let low = self.memory.read(self.registers.pc) as u16;
        self.registers.pc += 1;
        let high = self.memory.read(self.registers.pc) as u16;
        self.registers.pc += 1;

        // Return the bytes combined
        (high << 8) | low
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

        // TODO: Set the zero and negative flags as appropriate
    }

    // Load X
    fn ldx(&mut self) {
        // Get the address
        let address = self.fetch16();
        let byte = self.memory.read(address);
        self.registers.x = byte;

        // TODO: Set the zero and negative flags as appropriate
    }

    // Load Y
    fn ldy(&mut self) {
        let address = self.fetch16();
        let byte = self.memory.read(address);
        self.registers.y = byte;

        // TODO: Set the zero and negative flags as appropriate
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
