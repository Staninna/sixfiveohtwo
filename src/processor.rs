// Imports
use crate::memory::Memory;
use crate::opcodes::*;
use crate::registers::Registers;

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

    fn fetch(&mut self) -> u8 {
        // Get the byte at the program counter
        let opcode = self.memory.read(self.registers.pc);

        // Increment the program counter
        self.registers.pc += 1;

        // Return the byte
        opcode
    }

    fn execute(&mut self, opcode: u8) {
        // Execute the instruction
        match opcode {
            // Loadstore
            // LDA => self.lda(), TODO: Add this instruction
            // LDX => self.ldx(), TODO: Add this instruction
            // LDY => self.ldy(), TODO: Add this instruction
            // STA => self.sta(), TODO: Add this instruction
            // STX => self.stx(), TODO: Add this instruction
            // STY => self.sty(), TODO: Add this instruction

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

    pub fn run(&mut self) {
        // Fetch and execute instructions
        loop {
            let instruction = self.fetch();
            self.execute(instruction);
        }
    }

    pub fn step(&mut self) {
        // Fetch and execute one instruction
        let instruction = self.fetch();
        self.execute(instruction);
    }

    // Instructions ----------------------------------------------------------------

    // Transfer

    // Transfer Accumulator to X
    pub fn tax(&mut self) {
        self.registers.x = self.registers.acc;
    }

    // Transfer Accumulator to Y
    pub fn tay(&mut self) {
        self.registers.y = self.registers.acc;
    }

    // Transfer X to Accumulator
    pub fn txa(&mut self) {
        self.registers.acc = self.registers.x;
    }

    // Transfer Y to Accumulator
    pub fn tya(&mut self) {
        self.registers.acc = self.registers.y;
    }
}
