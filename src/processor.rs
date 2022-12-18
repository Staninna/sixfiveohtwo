// Imports
use crate::device::Device;
use crate::device_mapper::DeviceMapper;
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

// Struct for the processor
#[derive(Debug)]
pub struct Processor {
    registers: Registers,
    device_mapper: DeviceMapper,
}

impl Processor {
    pub fn new(program: Vec<u8>) -> Self {
        // Make the memory and registers
        let mut registers = Registers::new();
        let mut device_mapper = DeviceMapper::new();

        // Load the program into memory
        let offset = 0x8000;
        registers.pc = offset;
        for (i, byte) in program.iter().enumerate() {
            device_mapper.write((i + offset as usize) as u16, *byte);
        }

        // Return the processor
        Self {
            registers,
            device_mapper,
        }
    }

    pub fn new_offset(program: Vec<u8>, offset: u16) -> Self {
        // Make the memory and registers
        let mut registers = Registers::new();
        let mut device_mapper = DeviceMapper::new();

        // Load the program into memory
        registers.pc = offset;
        for (i, byte) in program.iter().enumerate() {
            device_mapper.write((i + offset as usize) as u16, *byte);
        }

        // Return the processor
        Self {
            registers,
            device_mapper,
        }
    }

    pub fn read_registers(&self) -> &Registers {
        &self.registers
    }

    pub fn map(&mut self, start: u16, end: u16, device: Box<dyn Device>) {
        self.device_mapper.map(start, end, device);
    }

    pub fn unmap(&mut self, start: u16, end: u16) {
        self.device_mapper.unmap(start, end);
    }

    fn read(&mut self, address: u16) -> u8 {
        self.device_mapper.read(address)
    }

    fn write(&mut self, address: u16, data: u8) {
        self.device_mapper.write(address, data);
    }

    pub fn run(&mut self) {
        // Fetch and execute opcodes
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
        // Read the byte at the program counter
        let byte = self.read(self.registers.pc);

        // Increment the program counter
        self.registers.pc += 1;

        // Return the byte
        byte
    }

    fn fetch16(&mut self) -> u16 {
        // Read the two bytes at the program counter least significant bit name low and high
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
            // Load/store opcodes

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

            // Store Y
            STY_ZP => self.sty(AddressingMode::ZeroPage),
            STY_ZPX => self.sty(AddressingMode::ZeroPageX),
            STY_ABS => self.sty(AddressingMode::Absolute),

            // Register transfers opcodes

            // Transfer accumulator to X
            TAX => self.tax(),

            // Transfer accumulator to Y
            TAY => self.tay(),

            // Transfer X to accumulator
            TXA => self.txa(),

            // Transfer Y to accumulator
            TYA => self.tya(),

            // Stack opcodes

            // Transfer stack pointer to X
            TSX => self.tsx(),

            // Transfer X to stack pointer
            TXS => self.txs(),

            // Push accumulator on stack
            PHA => self.pha(),

            // Push processor status on stack
            PHP => self.php(),

            // Pull accumulator from stack
            PLA => self.pla(),

            // Pull processor status from stack
            PLP => self.plp(),

            // Unknow opcode
            _ => {
                panic!("Unknown opcode: {:#X}", opcode);
            }
        }
    }

    // Opcodes

    // Load/store opcodes

    // Load accumulator
    fn lda(&mut self, mode: AddressingMode) {
        let value = match mode {
            // Immediate
            AddressingMode::Immediate => self.fetch8(),

            // Zero page
            AddressingMode::ZeroPage => {
                let offset = self.fetch8();
                let address = 0x0000 + offset as u16;
                self.read(address)
            }

            // Zero page X
            AddressingMode::ZeroPageX => {
                let x = self.registers.x;
                let offset = self.fetch8();
                let address = 0x0000 + offset as u16 + x as u16;
                self.read(address)
            }

            // Absolute
            AddressingMode::Absolute => {
                let address = self.fetch16();
                self.read(address)
            }

            // Absolute X
            AddressingMode::AbsoluteX => {
                let x = self.registers.x;
                let address = self.fetch16() + x as u16;
                self.read(address)
            }

            // Absolute Y
            AddressingMode::AbsoluteY => {
                let y = self.registers.y;
                let address = self.fetch16() + y as u16;
                self.read(address)
            }

            // Indirect X
            AddressingMode::IndirectX => {
                // Read the offset
                let x = self.registers.x;
                let offset = self.fetch8();

                // Read the pointer address
                let pointer_address = 0x0000 + offset as u16 + x as u16;
                let low = self.read(pointer_address);
                let high = self.read(pointer_address + 1);

                // Read the value
                let address = u16::from_le_bytes([low, high]);
                self.read(address)
            }

            // Indirect Y
            AddressingMode::IndirectY => {
                // Read the offset
                let y = self.registers.y;
                let offset = self.fetch8();

                // Read the pointer address
                let address = 0x0000 + offset as u16;
                let low = self.read(address);
                let high = self.read(address + 1);

                // Read the value
                let address = u16::from_le_bytes([low, high]) + y as u16;
                self.read(address)
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
                self.read(address)
            }

            // Zero page y
            AddressingMode::ZeroPageY => {
                let y = self.registers.y;
                let offset = self.fetch8();
                let address = 0x0000 + offset as u16 + y as u16;
                self.read(address)
            }

            // Absolute
            AddressingMode::Absolute => {
                let address = self.fetch16();
                self.read(address)
            }

            // Absolute Y
            AddressingMode::AbsoluteY => {
                let y = self.registers.y;
                let address = self.fetch16() + y as u16;
                self.read(address)
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
                self.read(address)
            }

            // Zero page X
            AddressingMode::ZeroPageX => {
                let x = self.registers.x;
                let offset = self.fetch8();
                let address = 0x0000 + offset as u16 + x as u16;
                self.read(address)
            }

            // Absolute
            AddressingMode::Absolute => {
                let address = self.fetch16();
                self.read(address)
            }

            // Absolute X
            AddressingMode::AbsoluteX => {
                let x = self.registers.x;
                let address = self.fetch16() + x as u16;
                self.read(address)
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
                // Read the offset
                let x = self.registers.x;
                let offset = self.fetch8();

                // Read the pointer address
                let pointer_address = 0x0000 + offset as u16 + x as u16;
                let low = self.read(pointer_address);
                let high = self.read(pointer_address + 1);

                // Read the value
                u16::from_le_bytes([low, high])
            }

            // Indirect Y
            AddressingMode::IndirectY => {
                // Read the offset
                let y = self.registers.y;
                let offset = self.fetch8();

                // Read the pointer address
                let address = 0x0000 + offset as u16;
                let low = self.read(address);
                let high = self.read(address + 1);

                // Read the value
                u16::from_le_bytes([low, high]) + y as u16
            }

            // Unknow addressing mode
            _ => {
                unreachable!("Unreachable addressing mode")
            }
        };

        // Write value
        self.write(address, self.registers.acc);
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
        self.write(address, self.registers.x);
    }

    // Store Y
    fn sty(&mut self, mode: AddressingMode) {
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

            // Unknow addressing mode
            _ => {
                unreachable!("Unreachable addressing mode")
            }
        };

        // Write value
        self.write(address, self.registers.y);
    }

    // Register transfers opcodes

    // Transfer accumulator to X
    fn tax(&mut self) {
        // Write X
        self.registers.x = self.registers.acc;

        // Write zero flag
        if self.registers.x == 0x00 {
            self.write_flag(ZERO, true)
        } else {
            self.write_flag(ZERO, false)
        }

        // Write negative flag
        if self.registers.x & 0x80 == 0x80 {
            self.write_flag(NEGATIVE, true)
        } else {
            self.write_flag(NEGATIVE, false)
        }
    }

    // Transfer accumulator to Y
    fn tay(&mut self) {
        // Write Y
        self.registers.y = self.registers.acc;

        // Write zero flag
        if self.registers.y == 0x00 {
            self.write_flag(ZERO, true)
        } else {
            self.write_flag(ZERO, false)
        }

        // Write negative flag
        if self.registers.y & 0x80 == 0x80 {
            self.write_flag(NEGATIVE, true)
        } else {
            self.write_flag(NEGATIVE, false)
        }
    }

    // Transfer X to accumulator
    fn txa(&mut self) {
        // Write accumulator
        self.registers.acc = self.registers.x;

        // Write zero flag
        if self.registers.acc == 0x00 {
            self.write_flag(ZERO, true)
        } else {
            self.write_flag(ZERO, false)
        }

        // Write negative flag
        if self.registers.acc & 0x80 == 0x80 {
            self.write_flag(NEGATIVE, true)
        } else {
            self.write_flag(NEGATIVE, false)
        }
    }

    // Transfer Y to accumulator
    fn tya(&mut self) {
        // Write accumulator
        self.registers.acc = self.registers.y;

        // Write zero flag
        if self.registers.acc == 0x00 {
            self.write_flag(ZERO, true)
        } else {
            self.write_flag(ZERO, false)
        }

        // Write negative flag
        if self.registers.acc & 0x80 == 0x80 {
            self.write_flag(NEGATIVE, true)
        } else {
            self.write_flag(NEGATIVE, false)
        }
    }

    // Stack opcodes

    // Transfer stack pointer to X
    fn tsx(&mut self) {
        // Write X
        self.registers.x = self.registers.sp;

        // Write zero falg
        if self.registers.x == 0x00 {
            self.write_flag(ZERO, true)
        } else {
            self.write_flag(ZERO, false)
        }

        // Write negative flag
        if self.registers.x & 0x80 == 0x80 {
            self.write_flag(NEGATIVE, true)
        } else {
            self.write_flag(NEGATIVE, false)
        }
    }

    // Transfer X to stack pointer
    fn txs(&mut self) {
        // Write Sp
        self.registers.sp = self.registers.x;
    }

    // Push accumulator on stack
    fn pha(&mut self) {
        // Read accumulator and sp
        let acc = self.registers.acc;
        let sp = self.registers.sp;

        // Push acc
        self.write(0x0100 + sp as u16, acc);

        // Decrement the sp
        let _ = self.registers.sp - 1;
    }

    // Push processor status on stack
    fn php(&mut self) {
        // Read status and sp
        let status = self.registers.status.bits();
        let sp = self.registers.sp;

        // Push status
        self.write(0x0100 + sp as u16, status);

        // Decrement the sp
        let _ = self.registers.sp - 1;
    }

    // Pull accumulator from stack
    fn pla(&mut self) {
        // Read sp
        let sp = self.registers.sp;

        // Pull accumulator
        let acc = self.read(0x0100 + sp as u16);

        // Write accumulator
        self.registers.acc = acc;

        // Increment sp
        let _ = self.registers.sp + 1;

        // Write zero falg
        if self.registers.x == 0x00 {
            self.write_flag(ZERO, true)
        } else {
            self.write_flag(ZERO, false)
        }

        // Write negative flag
        if self.registers.x & 0x80 == 0x80 {
            self.write_flag(NEGATIVE, true)
        } else {
            self.write_flag(NEGATIVE, false)
        }
    }

    // Pull processor status from stack
    fn plp(&mut self) {
        // Read sp
        let sp = self.registers.sp;

        // Pull status
        let raw_status = self.read(0x0100 + sp as u16);

        // Write status
        self.registers.status = match Status::from_bits(raw_status) {
            Some(status) => status,
            None => unreachable!("Unreachable status"),
        };
    }
}
