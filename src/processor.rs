// Imports
use crate::registers::Registers;

// Structs

///////////////
// Processor //
///////////////

#[derive(Debug)]
pub struct Processor {
    registers: Registers,
}

impl Processor {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
        }
    }
}
