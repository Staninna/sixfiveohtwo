// Imports
use std::fmt;

// Structs

////////////
// Memory //
////////////

// Struct for the memory
pub struct Memory {
    memory: Vec<u8>,
}

// Implement the Memory struct
impl Memory {
    pub fn new(size: u16) -> Self {
        Self {
            memory: vec![0; size as usize],
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn write(&mut self, address: u16, data: u8) {
        self.memory[address as usize] = data;
    }

    pub fn load(&mut self, program: Vec<u8>, start_address: u16) {
        if start_address as usize + program.len() > self.memory.len() {
            panic!("Program does not fit in memory");
        }

        // Load the program into memory
        for (i, byte) in program.iter().enumerate() {
            self.memory[start_address as usize + i] = *byte;
        }
    }
}

// Implement the Debug trait for the Memory struct
impl fmt::Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();

        // Add new line
        output.push_str(&format!("\n"));

        // Iterate through the memory
        for (i, byte) in self.memory.iter().enumerate() {
            // Add the address
            if i % 16 == 0 {
                output.push_str(&format!("{:#06X}: ", i));
            }

            // Add the byte
            output.push_str(&format!("{:02X} ", byte));

            // Add a new line
            if i % 16 == 15 {
                output.push_str(&format!("\n"));
            }
        }

        // Return the output
        write!(f, "{}", output)
    }
}
