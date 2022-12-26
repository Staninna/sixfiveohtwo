use crate::{
    device::Device,
    device_mapper::DeviceMapper,
    opcodes::*,
    registers::{Registers, Status},
};

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
        let offset = 0x0800;
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

    // Push a byte on the stack
    fn push(&mut self, byte: u8) {
        // Decrement the stack pointer
        self.registers.sp = self.registers.sp.wrapping_sub(1);

        // Write the byte to the stack
        let address = 0x0100 + self.registers.sp as u16;
        self.write(address, byte);
    }

    // Pull a byte from the stack
    fn pull(&mut self) -> u8 {
        // Read the byte from the stack
        let address = 0x0100 + self.registers.sp as u16;
        let byte = self.read(address);

        // Increment the stack pointer
        self.registers.sp = self.registers.sp.wrapping_add(1);

        byte
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

            // Stack

            // Transfer stack pointer to X register
            TSX => self.tsx(),

            // Transfer X register to stack pointer
            TXS => self.txs(),

            // Push accumulator on stack
            PHA => self.pha(),

            // Push processor status on stack
            PHP => self.php(),

            // Pull accumulator from stack
            PLA => self.pla(),

            // Pull processor status from stack
            PLP => self.plp(),

            // Logical

            // Logical AND
            AND_IM => self.and_immediate(),
            AND_ZP => self.and_zero_page(),
            AND_ZPX => self.and_zero_page_x(),
            AND_ABS => self.and_absolute(),
            AND_ABSX => self.and_absolute_x(),
            AND_ABSY => self.and_absolute_y(),
            AND_INDX => self.and_indirect_x(),
            AND_INDY => self.and_indirect_y(),

            // Logical EOR
            EOR_IM => self.eor_immediate(),
            EOR_ZP => self.eor_zero_page(),
            EOR_ZPX => self.eor_zero_page_x(),
            EOR_ABS => self.eor_absolute(),
            EOR_ABSX => self.eor_absolute_x(),
            EOR_ABSY => self.eor_absolute_y(),
            EOR_INDX => self.eor_indirect_x(),
            EOR_INDY => self.eor_indirect_y(),

            // Logical ORA
            ORA_IM => self.ora_immediate(),
            ORA_ZP => self.ora_zero_page(),
            ORA_ZPX => self.ora_zero_page_x(),
            ORA_ABS => self.ora_absolute(),
            ORA_ABSX => self.ora_absolute_x(),
            ORA_ABSY => self.ora_absolute_y(),
            ORA_INDX => self.ora_indirect_x(),
            ORA_INDY => self.ora_indirect_y(),

            // Bit test
            BIT_ZP => self.bit_zero_page(),
            BIT_ABS => self.bit_absolute(),

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

    // Opcode implementations

    // Load/store

    // Load accumulator
    fn lda(&mut self, value: u8) {
        self.registers.acc = value;

        self.registers.status.set(Status::ZERO, value == 0x00);
        self.registers
            .status
            .set(Status::NEGATIVE, value & 0b1000_0000 != 0);
    }

    // Load accumulator immediate
    fn lda_immediate(&mut self) {
        let value = self.immediate();
        self.lda(value);
    }

    // Load accumulator zero page
    fn lda_zero_page(&mut self) {
        let value = self.zero_page_read();
        self.lda(value);
    }

    // Load accumulator zero page, X
    fn lda_zero_page_x(&mut self) {
        let value = self.zero_page_x_read();
        self.lda(value);
    }

    // Load accumulator absolute
    fn lda_absolute(&mut self) {
        let value = self.absolute_read();
        self.lda(value);
    }

    // Load accumulator absolute, X
    fn lda_absolute_x(&mut self) {
        let value = self.absolute_x_read();
        self.lda(value);
    }

    // Load accumulator absolute, Y
    fn lda_absolute_y(&mut self) {
        let value = self.absolute_y_read();
        self.lda(value);
    }

    // Load accumulator indirect, X
    fn lda_indirect_x(&mut self) {
        let value = self.indirect_x_read();
        self.lda(value);
    }

    // Load accumulator indirect, Y
    fn lda_indirect_y(&mut self) {
        let value = self.indirect_y_read();
        self.lda(value);
    }

    // Load X register
    fn ldx(&mut self, value: u8) {
        self.registers.x = value;

        self.registers.status.set(Status::ZERO, value == 0x00);
        self.registers
            .status
            .set(Status::NEGATIVE, value & 0b1000_0000 != 0);
    }

    // Load X register immediate
    fn ldx_immediate(&mut self) {
        let value = self.immediate();
        self.ldx(value);
    }

    // Load X register zero page
    fn ldx_zero_page(&mut self) {
        let value = self.zero_page_read();
        self.ldx(value);
    }

    // Load X register zero page, Y
    fn ldx_zero_page_y(&mut self) {
        let value = self.zero_page_y_read();
        self.ldx(value);
    }

    // Load X register absolute
    fn ldx_absolute(&mut self) {
        let value = self.absolute_read();
        self.ldx(value);
    }

    // Load X register absolute, Y
    fn ldx_absolute_y(&mut self) {
        let value = self.absolute_y_read();
        self.ldx(value);
    }

    // Load Y register
    fn ldy(&mut self, value: u8) {
        self.registers.y = value;

        self.registers.status.set(Status::ZERO, value == 0x00);
        self.registers
            .status
            .set(Status::NEGATIVE, value & 0b1000_0000 != 0);
    }

    // Load Y register immediate
    fn ldy_immediate(&mut self) {
        let value = self.immediate();
        self.ldy(value);
    }

    // Load Y register zero page
    fn ldy_zero_page(&mut self) {
        let value = self.zero_page_read();
        self.ldy(value);
    }

    // Load Y register zero page, X
    fn ldy_zero_page_x(&mut self) {
        let value = self.zero_page_x_read();
        self.ldy(value);
    }

    // Load Y register absolute
    fn ldy_absolute(&mut self) {
        let value = self.absolute_read();
        self.ldy(value);
    }

    // Load Y register absolute, X
    fn ldy_absolute_x(&mut self) {
        let value = self.absolute_x_read();
        self.ldy(value);
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

        self.registers.status.set(Status::ZERO, value == 0x00);
        self.registers
            .status
            .set(Status::NEGATIVE, value & 0b1000_0000 != 0);
    }

    // Transfer accumulator to Y register
    fn tay(&mut self) {
        let value = self.registers.acc;
        self.registers.y = value;

        self.registers.status.set(Status::ZERO, value == 0x00);
        self.registers
            .status
            .set(Status::NEGATIVE, value & 0b1000_0000 != 0);
    }

    // Transfer X register to accumulator
    fn txa(&mut self) {
        let value = self.registers.x;
        self.registers.acc = value;

        self.registers.status.set(Status::ZERO, value == 0x00);
        self.registers
            .status
            .set(Status::NEGATIVE, value & 0b1000_0000 != 0);
    }

    // Transfer Y register to accumulator
    fn tya(&mut self) {
        let value = self.registers.y;
        self.registers.acc = value;

        self.registers.status.set(Status::ZERO, value == 0x00);
        self.registers
            .status
            .set(Status::NEGATIVE, value & 0b1000_0000 != 0);
    }

    // Transfer stack pointer to X register
    fn tsx(&mut self) {
        let value = self.registers.sp;
        self.registers.x = value;

        self.registers.status.set(Status::ZERO, value == 0x00);
        self.registers
            .status
            .set(Status::NEGATIVE, value & 0b1000_0000 != 0);
    }

    // Transfer X register to stack pointer
    fn txs(&mut self) {
        let value = self.registers.x;
        self.registers.sp = value;
    }

    // Stack

    // Push accumulator
    fn pha(&mut self) {
        let value = self.registers.acc;
        self.push(value);
    }

    // Push processor status
    fn php(&mut self) {
        let value = self.registers.status.bits();
        self.push(value);
    }

    // Pull accumulator
    fn pla(&mut self) {
        let value = self.pull();
        self.registers.acc = value;

        self.registers.status.set(Status::ZERO, value == 0x00);
        self.registers
            .status
            .set(Status::NEGATIVE, value & 0b1000_0000 != 0);
    }

    // Pull processor status
    fn plp(&mut self) {
        let value = self.pull();
        self.registers.status = Status::from_bits_truncate(value);
    }

    // Logical

    // Logical AND
    fn and(&mut self, value: u8) {
        let result = self.registers.acc & value;
        self.registers.acc = result;

        self.registers.status.set(Status::ZERO, result == 0x00);
        self.registers
            .status
            .set(Status::NEGATIVE, result & 0x80 != 0);
    }

    // Logical AND immediate
    fn and_immediate(&mut self) {
        let value = self.immediate();
        self.and(value)
    }

    // Logical AND zero page
    fn and_zero_page(&mut self) {
        let value = self.zero_page_read();
        self.and(value)
    }

    // Logical AND zero page, X
    fn and_zero_page_x(&mut self) {
        let value = self.zero_page_x_read();
        self.and(value)
    }

    // Logical AND absolute
    fn and_absolute(&mut self) {
        let value = self.absolute_read();
        self.and(value)
    }

    // Logical AND absolute, X
    fn and_absolute_x(&mut self) {
        let value = self.absolute_x_read();
        self.and(value)
    }

    // Logical AND absolute, Y
    fn and_absolute_y(&mut self) {
        let value = self.absolute_y_read();
        self.and(value)
    }

    // Logical AND (indirect, X)
    fn and_indirect_x(&mut self) {
        let value = self.indirect_x_read();
        self.and(value)
    }

    // Logical AND (indirect), Y
    fn and_indirect_y(&mut self) {
        let value = self.indirect_y_read();
        self.and(value)
    }

    // Logical EOR
    fn eor(&mut self, value: u8) {
        self.registers.acc ^= value;

        self.registers
            .status
            .set(Status::ZERO, self.registers.acc == 0x00);
        self.registers
            .status
            .set(Status::NEGATIVE, self.registers.acc & 0b1000_0000 != 0);
    }

    // Logical EOR immediate
    fn eor_immediate(&mut self) {
        let value = self.immediate();
        self.eor(value)
    }

    // Logical EOR zero page
    fn eor_zero_page(&mut self) {
        let value = self.zero_page_read();
        self.eor(value)
    }

    // Logical EOR zero page, X
    fn eor_zero_page_x(&mut self) {
        let value = self.zero_page_x_read();
        self.eor(value)
    }

    // Logical EOR absolute
    fn eor_absolute(&mut self) {
        let value = self.absolute_read();
        self.eor(value)
    }

    // Logical EOR absolute, X
    fn eor_absolute_x(&mut self) {
        let value = self.absolute_x_read();
        self.eor(value)
    }

    // Logical EOR absolute, Y
    fn eor_absolute_y(&mut self) {
        let value = self.absolute_y_read();
        self.eor(value)
    }

    // Logical EOR (indirect, X)
    fn eor_indirect_x(&mut self) {
        let value = self.indirect_x_read();
        self.eor(value)
    }

    // Logical EOR (indirect), Y
    fn eor_indirect_y(&mut self) {
        let value = self.indirect_y_read();
        self.eor(value)
    }

    // Logical ORA
    fn ora(&mut self, value: u8) {
        self.registers.acc |= value;

        self.registers
            .status
            .set(Status::ZERO, self.registers.acc == 0x00);
        self.registers
            .status
            .set(Status::NEGATIVE, self.registers.acc & 0b1000_0000 != 0);
    }

    // Logical ORA immediate
    fn ora_immediate(&mut self) {
        let value = self.immediate();
        self.ora(value)
    }

    // Logical ORA zero page
    fn ora_zero_page(&mut self) {
        let value = self.zero_page_read();
        self.ora(value)
    }

    // Logical ORA zero page, X
    fn ora_zero_page_x(&mut self) {
        let value = self.zero_page_x_read();
        self.ora(value)
    }

    // Logical ORA absolute
    fn ora_absolute(&mut self) {
        let value = self.absolute_read();
        self.ora(value)
    }

    // Logical ORA absolute, X
    fn ora_absolute_x(&mut self) {
        let value = self.absolute_x_read();
        self.ora(value)
    }

    // Logical ORA absolute, Y
    fn ora_absolute_y(&mut self) {
        let value = self.absolute_y_read();
        self.ora(value)
    }

    // Logical ORA (indirect, X)
    fn ora_indirect_x(&mut self) {
        let value = self.indirect_x_read();
        self.ora(value)
    }

    // Logical ORA (indirect), Y
    fn ora_indirect_y(&mut self) {
        let value = self.indirect_y_read();
        self.ora(value)
    }

    // Logical BIT test
    fn bit(&mut self, value: u8) {
        let result = self.registers.acc & value;

        self.registers.status.set(Status::ZERO, result == 0x00);
        self.registers
            .status
            .set(Status::NEGATIVE, result & 0b1000_0000 != 0);
        self.registers
            .status
            .set(Status::OVERFLOW, result & 0b0100_0000 != 0);
    }

    // Logical BIT zero page
    fn bit_zero_page(&mut self) {
        let value = self.zero_page_read();
        self.bit(value)
    }

    // Logical BIT absolute
    fn bit_absolute(&mut self) {
        let value = self.absolute_read();
        self.bit(value)
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
