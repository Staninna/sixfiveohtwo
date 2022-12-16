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

// Unit tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Test the Memory struct write and read functions
    fn test_memory_write_read() {
        let mut memory = Memory::new(0xFFFF);

        memory.write(0x0000, 0x01);
        memory.write(0x0001, 0x02);
        memory.write(0x0002, 0x03);
        memory.write(0x0003, 0x04);
        memory.write(0x0004, 0x05);
        memory.write(0x0005, 0x06);

        assert_eq!(memory.read(0x0000), 0x01);
        assert_eq!(memory.read(0x0001), 0x02);
        assert_eq!(memory.read(0x0002), 0x03);
        assert_eq!(memory.read(0x0003), 0x04);
        assert_eq!(memory.read(0x0004), 0x05);
        assert_eq!(memory.read(0x0005), 0x06);
    }

    #[test]
    // Test the Memory struct load function
    fn test_memory_load() {
        let mut memory = Memory::new(0xFFFF);

        memory.load(vec![0x01, 0x02, 0x03], 0x0000);

        assert_eq!(memory.read(0x0000), 0x01);
        assert_eq!(memory.read(0x0001), 0x02);
        assert_eq!(memory.read(0x0002), 0x03);
    }

    #[test]
    #[should_panic]
    // Test the Memory struct load function with an out of bounds addresses
    fn test_memory_load_out_of_bounds() {
        let mut memory = Memory::new(0xFFFF);

        memory.load(vec![0x01, 0x02, 0x03], 0xFFFD);
    }

    #[test]
    // Test the Memory struct Debug trait
    fn test_memory_debug() {
        let mut memory = Memory::new(0x0020);

        memory.load(
            vec![
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B,
            ],
            0x0000,
        );

        let output = format!("{:?}", memory);

        assert_eq!(
            output,
            "\n0x0000: 01 02 03 04 05 06 07 08 09 0A 0B 00 00 00 00 00 \n0x0010: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 \n"
        );
    }
}
