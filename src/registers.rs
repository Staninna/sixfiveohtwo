// Imports
use bitflags::bitflags;

// Structs

// Struct for the flags
pub enum Flag {
    Negative,
    Overflow,
    Unused,
    BreakCommand,
    DecimalMode,
    Interrupt,
    Zero,
    Carry,
}

// Obivous way to set flasgs in Status
struct StatusFlags {
    negative: bool,
    overflow: bool,
    unused: bool,
    break_command: bool,
    decimal_mode: bool,
    interrupt: bool,
    zero: bool,
    carry: bool,
}

// Create a bitflag struct for the Status
bitflags! {
    pub struct Status: u8 {
        const NEGATIVE =      0b1000_0000;
        const OVERFLOW =      0b0100_0000;
        const UNUSED =        0b0010_0000;
        const BREAK_COMMAND = 0b0001_0000;
        const DECIMAL_MODE =  0b0000_1000;
        const INTERRUPT =     0b0000_0100;
        const ZERO =          0b0000_0010;
        const CARRY =         0b0000_0001;
    }
}

// Implement the Status struct
impl Status {
    fn default() -> Self {
        Self::new(StatusFlags {
            negative: false,
            overflow: false,
            unused: true,
            break_command: false,
            decimal_mode: false,
            interrupt: false,
            zero: false,
            carry: false,
        })
    }

    // Create a new Status from flags given a StatusFlags
    fn new(flags: StatusFlags) -> Self {
        // Get a empty status
        let mut status = Status::empty();

        // Set the status flags
        if flags.negative {
            status |= Status::NEGATIVE;
        }
        if flags.overflow {
            status |= Status::OVERFLOW;
        }
        if flags.unused {
            status |= Status::UNUSED;
        }
        if flags.break_command {
            status |= Status::BREAK_COMMAND;
        }
        if flags.decimal_mode {
            status |= Status::DECIMAL_MODE;
        }
        if flags.interrupt {
            status |= Status::INTERRUPT;
        }
        if flags.zero {
            status |= Status::ZERO;
        }
        if flags.carry {
            status |= Status::CARRY;
        }

        // Return the status
        status
    }
}

// Struct for the registers
#[derive(Debug)]
pub struct Registers {
    pub pc: u16,        // Program counter
    pub sp: u8,         // Stack pointer
    pub acc: u8,        // Accumulator
    pub x: u8,          // X register
    pub y: u8,          // Y register
    pub status: Status, // Status register
}

// Implement the Registers struct
impl Registers {
    pub fn new() -> Self {
        Self {
            pc: 0x8000,
            sp: 0x00,
            acc: 0x00,
            x: 0x00,
            y: 0x00,
            status: Status::default(),
        }
    }
}
