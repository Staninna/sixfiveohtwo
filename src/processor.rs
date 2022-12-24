use crate::device::Device;
use crate::device_mapper::DeviceMapper;
use crate::opcodes::*;
use crate::registers::{Registers, Status};

pub struct Processor {
    registers: Registers,
    device_mapper: DeviceMapper,
}

impl Processor {
    // Public functions

    // Create a new processor with the given program loaded into RAM
    pub fn new(program: Vec<u8>) -> Self {
        // Create RAM (divice_mapper) and registers
        let mut registers = Registers::new();
        let mut device_mapper = DeviceMapper::new();

        // Load the program
        let offset = 0x0000;
        registers.pc = offset;
        for (i, byte) in program.iter().enumerate() {
            device_mapper.write((i + offset as usize) as u16, *byte);
        }

        Self {
            registers,
            device_mapper,
        }
    }

    // Map a device to the given address range
    pub fn map(&mut self, start: u16, end: u16, device: Box<dyn Device>) {
        self.device_mapper.map(start, end, device);
    }

    // Unmap a device from the given address range
    pub fn unmap(&mut self, start: u16, end: u16) {
        self.device_mapper.unmap(start, end);
    }

    // Run the processor until the program crashes
    pub fn run(&mut self) {
        loop {
            let instruction = self.fetch8();
            self.execute(instruction);
        }
    }

    // Run the processor for one instruction
    pub fn step(&mut self) {
        let instruction = self.fetch8();
        self.execute(instruction);
    }

    // Private functions

    // Read a byte from the given address
    fn read(&mut self, address: u16) -> u8 {
        self.device_mapper.read(address)
    }

    // Write a byte to the given address
    fn write(&mut self, address: u16, byte: u8) {
        self.device_mapper.write(address, byte);
    }

    // Fetch an 8-bit value from the program counter
    fn fetch8(&mut self) -> u8 {
        // Read the byte at the PC
        let byte = self.read(self.registers.pc);

        // Increment the PC
        self.registers.pc += 1;

        byte
    }

    // Fetch a 16-bit value from the program counter
    fn fetch16(&mut self) -> u16 {
        // Read the two bytes at the PC
        let high = self.fetch8();
        let low = self.fetch8();

        // Return as u16
        u16::from_le_bytes([low, high])
    }

    // Execute the given opcode
    fn execute(&mut self, opcode: u8) {
        match opcode {
            // Load/Store

            // Load accumulator
            LDA_IM => self.lda_immediate(),
            LDA_ZP => self.lda_zero_page(),
            LDA_ZPX => self.lda_zero_page_x(),
            LDA_ABS => self.lda_absolute(),
            LDA_ABSX => self.lda_absolute_x(),
            LDA_ABSY => self.lda_absolute_y(),
            LDA_INDX => self.lda_indirect_x(),
            LDA_INDY => self.lda_indirect_y(),

            // Load X register
            LDX_IM => self.ldx_immediate(),
            LDX_ZP => self.ldx_zero_page(),
            LDX_ZPY => self.ldx_zero_page_y(),
            LDX_ABS => self.ldx_absolute(),
            LDX_ABSY => self.ldx_absolute_y(),

            // Load Y register
            LDY_IM => self.ldy_immediate(),
            LDY_ZP => self.ldy_zero_page(),
            LDY_ZPX => self.ldy_zero_page_x(),
            LDY_ABS => self.ldy_absolute(),
            LDY_ABSX => self.ldy_absolute_x(),

            // Store accumulator
            STA_ZP => self.sta_zero_page(),
            STA_ZPX => self.sta_zero_page_x(),
            STA_ABS => self.sta_absolute(),
            STA_ABSX => self.sta_absolute_x(),
            STA_ABSY => self.sta_absolute_y(),
            STA_INDX => self.sta_indirect_x(),
            STA_INDY => self.sta_indirect_y(),

            // Store X register
            STX_ZP => self.stx_zero_page(),
            STX_ZPY => self.stx_zero_page_y(),
            STX_ABS => self.stx_absolute(),

            // Store Y register
            STY_ZP => self.sty_zero_page(),
            STY_ZPX => self.sty_zero_page_x(),
            STY_ABS => self.sty_absolute(),

            // Transfer

            // Transfer accumulator to X register
            TAX => self.tax(),

            // Transfer accumulator to Y register
            TAY => self.tay(),

            // Transfer X register to accumulator
            TXA => self.txa(),

            // Transfer Y register to accumulator
            TYA => self.tya(),

            // Unknow opcode
            _ => {
                panic!("Unknown opcode: {:#X}", opcode);
            }
        }
    }

    // Addressing modes

    fn immediate(&mut self) -> u8 {
        self.fetch8()
    }

    fn zero_page_read(&mut self) -> u8 {
        let address = self.fetch8();
        self.read(address as u16)
    }

    fn zero_page_addr(&mut self) -> u16 {
        self.fetch8() as u16
    }

    fn zero_page_x_read(&mut self) -> u8 {
        let x = self.registers.x;
        let address = self.fetch8().wrapping_add(x);
        self.read(address as u16)
    }

    fn zero_page_x_addr(&mut self) -> u16 {
        let x = self.registers.x;
        let address = self.fetch8().wrapping_add(x);
        address as u16
    }

    fn zero_page_y_read(&mut self) -> u8 {
        let y = self.registers.y;
        let address = self.fetch8().wrapping_add(y);
        self.read(address as u16)
    }

    fn zero_page_y_addr(&mut self) -> u16 {
        let y = self.registers.y;
        let address = self.fetch8().wrapping_add(y);
        address as u16
    }

    fn absolute_read(&mut self) -> u8 {
        let address = self.fetch16();
        self.read(address)
    }

    fn absolute_addr(&mut self) -> u16 {
        self.fetch16()
    }

    fn absolute_x_read(&mut self) -> u8 {
        let x = self.registers.x;
        let address = self.fetch16().wrapping_add(x as u16);
        self.read(address)
    }

    fn absolute_x_addr(&mut self) -> u16 {
        let x = self.registers.x;
        let address = self.fetch16().wrapping_add(x as u16);
        address
    }

    fn absolute_y_read(&mut self) -> u8 {
        let y = self.registers.y;
        let address = self.fetch16().wrapping_add(y as u16);
        self.read(address)
    }

    fn absolute_y_addr(&mut self) -> u16 {
        let y = self.registers.y;
        let address = self.fetch16().wrapping_add(y as u16);
        address
    }

    fn indirect_x_read(&mut self) -> u8 {
        let x = self.registers.x;
        let pointer = self.fetch8().wrapping_add(x);
        let low = self.read(pointer as u16);
        let high = self.read(pointer.wrapping_add(1) as u16);
        let address = u16::from_le_bytes([low, high]);
        self.read(address)
    }

    fn indirect_x_addr(&mut self) -> u16 {
        let x = self.registers.x;
        let pointer = self.fetch8().wrapping_add(x);
        let low = self.read(pointer as u16);
        let high = self.read(pointer.wrapping_add(1) as u16);
        u16::from_le_bytes([low, high])
    }

    fn indirect_y_read(&mut self) -> u8 {
        let y = self.registers.y;
        let pointer = self.fetch8();
        let low = self.read(pointer as u16);
        let high = self.read(pointer.wrapping_add(1) as u16);
        let address = u16::from_le_bytes([low, high]).wrapping_add(y as u16);
        self.read(address)
    }

    fn indirect_y_addr(&mut self) -> u16 {
        let y = self.registers.y;
        let pointer = self.fetch8();
        let low = self.read(pointer as u16);
        let high = self.read(pointer.wrapping_add(1) as u16);
        let address = u16::from_le_bytes([low, high]).wrapping_add(y as u16);
        address
    }

    // Set/unset flags

    fn set_negative(&mut self, value: u8) {
        self.registers
            .status
            .set(Status::NEGATIVE, value & 0x80 != 0x00);
    }

    fn set_overflow(&mut self, value: u8) {
        self.registers
            .status
            .set(Status::OVERFLOW, value & 0x40 != 0x00);
    }

    fn set_break(&mut self, value: u8) {
        self.registers
            .status
            .set(Status::BREAK, value & 0x10 != 0x00);
    }

    fn set_decimal(&mut self, value: u8) {
        self.registers
            .status
            .set(Status::DECIMAL, value & 0x08 != 0x00);
    }

    fn set_interrupt(&mut self, value: u8) {
        self.registers
            .status
            .set(Status::INTERRUPT, value & 0x04 != 0x00);
    }

    fn set_zero(&mut self, value: u8) {
        self.registers
            .status
            .set(Status::ZERO, value & 0x02 != 0x00);
    }

    fn set_carry(&mut self, value: u8) {
        self.registers
            .status
            .set(Status::CARRY, value & 0x01 != 0x00);
    }

    // Opcode implementations

    // Load/store

    // Load accumulator

    // Load accumulator immediate
    fn lda_immediate(&mut self) {
        let value = self.immediate();
        self.registers.acc = value;

        self.set_negative(value);
        self.set_zero(value);
    }

    // Load accumulator zero page
    fn lda_zero_page(&mut self) {
        let value = self.zero_page_read();
        self.registers.acc = value;

        self.set_negative(value);
        self.set_zero(value);
    }

    // Load accumulator zero page, X
    fn lda_zero_page_x(&mut self) {
        let value = self.zero_page_x_read();
        self.registers.acc = value;

        self.set_negative(value);
        self.set_zero(value);
    }

    // Load accumulator absolute
    fn lda_absolute(&mut self) {
        let value = self.absolute_read();
        self.registers.acc = value;

        self.set_negative(value);
        self.set_zero(value);
    }

    // Load accumulator absolute, X
    fn lda_absolute_x(&mut self) {
        let value = self.absolute_x_read();
        self.registers.acc = value;

        self.set_negative(value);
        self.set_zero(value);
    }

    // Load accumulator absolute, Y
    fn lda_absolute_y(&mut self) {
        let value = self.absolute_y_read();
        self.registers.acc = value;

        self.set_negative(value);
        self.set_zero(value);
    }

    // Load accumulator indirect, X
    fn lda_indirect_x(&mut self) {
        let value = self.indirect_x_read();
        self.registers.acc = value;

        self.set_negative(value);
        self.set_zero(value);
    }

    // Load accumulator indirect, Y
    fn lda_indirect_y(&mut self) {
        let value = self.indirect_y_read();
        self.registers.acc = value;

        self.set_negative(value);
        self.set_zero(value);
    }

    // Load X register

    // Load X register immediate
    fn ldx_immediate(&mut self) {
        let value = self.immediate();
        self.registers.x = value;

        self.set_negative(value);
        self.set_zero(value);
    }

    // Load X register zero page
    fn ldx_zero_page(&mut self) {
        let value = self.zero_page_read();
        self.registers.x = value;

        self.set_negative(value);
        self.set_zero(value);
    }

    // Load X register zero page, Y
    fn ldx_zero_page_y(&mut self) {
        let value = self.zero_page_y_read();
        self.registers.x = value;

        self.set_negative(value);
        self.set_zero(value);
    }

    // Load X register absolute
    fn ldx_absolute(&mut self) {
        let value = self.absolute_read();
        self.registers.x = value;

        self.set_negative(value);
        self.set_zero(value);
    }

    // Load X register absolute, Y
    fn ldx_absolute_y(&mut self) {
        let value = self.absolute_y_read();
        self.registers.x = value;

        self.set_negative(value);
        self.set_zero(value);
    }

    // Load Y register

    // Load Y register immediate
    fn ldy_immediate(&mut self) {
        let value = self.immediate();
        self.registers.y = value;

        self.set_negative(value);
        self.set_zero(value);
    }

    // Load Y register zero page
    fn ldy_zero_page(&mut self) {
        let value = self.zero_page_read();
        self.registers.y = value;

        self.set_negative(value);
        self.set_zero(value);
    }

    // Load Y register zero page, X
    fn ldy_zero_page_x(&mut self) {
        let value = self.zero_page_x_read();
        self.registers.y = value;

        self.set_negative(value);
        self.set_zero(value);
    }

    // Load Y register absolute
    fn ldy_absolute(&mut self) {
        let value = self.absolute_read();
        self.registers.y = value;

        self.set_negative(value);
        self.set_zero(value);
    }

    // Load Y register absolute, X
    fn ldy_absolute_x(&mut self) {
        let value = self.absolute_x_read();
        self.registers.y = value;

        self.set_negative(value);
        self.set_zero(value);
    }

    // Store accumulator

    // Store accumulator zero page
    fn sta_zero_page(&mut self) {
        let address = self.zero_page_addr();
        self.write(address, self.registers.acc);
    }

    // Store accumulator zero page, X
    fn sta_zero_page_x(&mut self) {
        let address = self.zero_page_x_addr();
        self.write(address, self.registers.acc);
    }

    // Store accumulator absolute
    fn sta_absolute(&mut self) {
        let address = self.absolute_addr();
        self.write(address, self.registers.acc);
    }

    // Store accumulator absolute, X
    fn sta_absolute_x(&mut self) {
        let address = self.absolute_x_addr();
        self.write(address, self.registers.acc);
    }

    // Store accumulator absolute, Y
    fn sta_absolute_y(&mut self) {
        let address = self.absolute_y_addr();
        self.write(address, self.registers.acc);
    }

    // Store accumulator indirect, X
    fn sta_indirect_x(&mut self) {
        let address = self.indirect_x_addr();
        self.write(address, self.registers.acc);
    }

    // Store accumulator indirect, Y
    fn sta_indirect_y(&mut self) {
        let address = self.indirect_y_addr();
        self.write(address, self.registers.acc);
    }

    // Store X register

    // Store X register zero page
    fn stx_zero_page(&mut self) {
        let address = self.zero_page_addr();
        self.write(address, self.registers.x);
    }

    // Store X register zero page, Y
    fn stx_zero_page_y(&mut self) {
        let address = self.zero_page_y_addr();
        self.write(address, self.registers.x);
    }

    // Store X register absolute
    fn stx_absolute(&mut self) {
        let address = self.absolute_addr();
        self.write(address, self.registers.x);
    }

    // Store Y register

    // Store Y register zero page
    fn sty_zero_page(&mut self) {
        let address = self.zero_page_addr();
        self.write(address, self.registers.y);
    }

    // Store Y register zero page, X
    fn sty_zero_page_x(&mut self) {
        let address = self.zero_page_x_addr();
        self.write(address, self.registers.y);
    }

    // Store Y register absolute
    fn sty_absolute(&mut self) {
        let address = self.absolute_addr();
        self.write(address, self.registers.y);
    }

    // Transfer

    // Transfer accumulator to X register
    fn tax(&mut self) {
        let value = self.registers.acc;
        self.registers.x = value;

        self.set_negative(value);
        self.set_zero(value);
    }

    // Transfer accumulator to Y register
    fn tay(&mut self) {
        let value = self.registers.acc;
        self.registers.y = value;

        self.set_negative(value);
        self.set_zero(value);
    }

    // Transfer X register to accumulator
    fn txa(&mut self) {
        let value = self.registers.x;
        self.registers.acc = value;

        self.set_negative(value);
        self.set_zero(value);
    }

    // Transfer Y register to accumulator
    fn tya(&mut self) {
        let value = self.registers.y;
        self.registers.acc = value;

        self.set_negative(value);
        self.set_zero(value);
    }
}

#[cfg(test)]
// Functions for unit testing
impl Processor {
    // Get a mutable reference to the registers
    pub fn set_register(&mut self) -> &mut Registers {
        &mut self.registers
    }

    // Get an immutable reference to the registers
    pub fn get_registers(&self) -> &Registers {
        &self.registers
    }

    // Write a byte to the given address
    pub fn set_mem(&mut self, address: u16, byte: u8) {
        self.device_mapper.write(address, byte);
    }

    // Read a byte from the given address
    pub fn get_mem(&mut self, address: u16) -> u8 {
        self.device_mapper.read(address)
    }
}
