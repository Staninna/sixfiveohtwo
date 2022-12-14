// Imports
use crate::memory::Memory;
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
    pub fn new(memory_size: u16) -> Self {
        // Make the memory and registers
        let mut memory = Memory::new(memory_size);
        let mut registers = Registers::new();

        // Return the processor
        Self { registers, memory }
    }
}
