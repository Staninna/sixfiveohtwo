// Imports
use bitflags::bitflags;

// Structs

//////////////////
// Status flags //
//////////////////

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

///////////////
// Registers //
///////////////

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
            pc: 0x0000,
            sp: 0x00,
            acc: 0x00,
            x: 0x00,
            y: 0x00,
            status: Status::default(),
        }
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Test the Status struct
    fn test_status() {
        // Create a new status
        let status = Status::new(StatusFlags {
            negative: true,
            overflow: true,
            unused: true,
            break_command: true,
            decimal_mode: true,
            interrupt: true,
            zero: true,
            carry: true,
        });

        // Check the status
        assert_eq!(status.bits(), 0b1111_1111);
        assert!(status.contains(Status::NEGATIVE));
        assert!(status.contains(Status::OVERFLOW));
        assert!(status.contains(Status::UNUSED));
        assert!(status.contains(Status::BREAK_COMMAND));
        assert!(status.contains(Status::DECIMAL_MODE));
        assert!(status.contains(Status::INTERRUPT));
        assert!(status.contains(Status::ZERO));
        assert!(status.contains(Status::CARRY));
    }

    #[test]
    // Test the Registers struct creation
    fn test_registers() {
        // Create a new registers
        let registers = Registers::new();

        // Check the registers
        assert_eq!(registers.pc, 0x0000);
        assert_eq!(registers.sp, 0x00);
        assert_eq!(registers.acc, 0x00);
        assert_eq!(registers.x, 0x00);
        assert_eq!(registers.y, 0x00);
        assert_eq!(registers.status.bits(), 0b0010_0000);
        assert!(!registers.status.contains(Status::NEGATIVE));
        assert!(!registers.status.contains(Status::OVERFLOW));
        assert!(registers.status.contains(Status::UNUSED));
        assert!(!registers.status.contains(Status::BREAK_COMMAND));
        assert!(!registers.status.contains(Status::DECIMAL_MODE));
        assert!(!registers.status.contains(Status::INTERRUPT));
        assert!(!registers.status.contains(Status::ZERO));
        assert!(!registers.status.contains(Status::CARRY));
    }

    #[test]
    // Test the Registers struct with debug
    fn test_registers_debug() {
        // Create a new registers
        let mut registers = Registers::new();

        // Set the registers
        registers.pc = 0x1234;
        registers.sp = 0x56;
        registers.acc = 0x78;
        registers.x = 0x9A;
        registers.y = 0xBC;
        registers.status = Status::new(StatusFlags {
            negative: true,
            overflow: true,
            unused: true,
            break_command: true,
            decimal_mode: true,
            interrupt: true,
            zero: true,
            carry: true,
        });

        // Check the registers
        assert_eq!(
            format!("{:?}", registers),
            "Registers { pc: 4660, sp: 86, acc: 120, x: 154, y: 188, status: NEGATIVE | OVERFLOW | UNUSED | BREAK_COMMAND | DECIMAL_MODE | INTERRUPT | ZERO | CARRY }"
        );
    }
}
