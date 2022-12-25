pub use processor::*;
mod processor {
    use crate::{processor::Processor, registers::Status};
    // Get status register
    fn get_status(processor: &Processor) -> (bool, bool, bool, bool, bool, bool, bool, bool) {
        let status = processor.get_registers().status;
        let zero = status.contains(Status::ZERO);
        let negative = status.contains(Status::NEGATIVE);
        let carry = status.contains(Status::CARRY);
        let overflow = status.contains(Status::OVERFLOW);
        let decimal = status.contains(Status::DECIMAL);
        let interrupt = status.contains(Status::INTERRUPT);
        let break_ = status.contains(Status::BREAK);
        let unused = status.contains(Status::UNUSED);
        (
            zero, negative, carry, overflow, decimal, interrupt, break_, unused,
        )
    }

    // Load/store
    pub use load_store::*;
    mod load_store {
        use super::*;
        use crate::{
            opcodes::{LDA_IM, LDX_IM, LDY_IM, STA_ZP, STX_ZP, STY_ZP},
            processor::Processor,
        };

        #[test]
        // Load accumulator
        fn test_lda() {
            // Create processor with instruction
            let mut processor = Processor::new(vec![
                LDA_IM, 0x42, // LDA #$42 (No flags set)
                LDA_IM, 0x80, // LDA #$80 (Negative flag set)
                LDA_IM, 0x00, // LDA #$00 (Zero flag set)
            ]);

            // Execute instruction
            processor.step();

            // Check state of processor
            let acc = processor.get_registers().acc;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(acc, 0x42);
            assert!(!zero);
            assert!(!negative);

            // Execute instruction
            processor.step();

            // Check state of processor
            let acc = processor.get_registers().acc;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(acc, 0x80);
            assert!(!zero);
            assert!(negative);

            // Execute instruction
            processor.step();

            // Check state of processor
            let acc = processor.get_registers().acc;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(acc, 0x00);
            assert!(zero);
            assert!(!negative);
        }

        #[test]
        // Load X register
        fn test_ldx() {
            // Create processor with instruction
            let mut processor = Processor::new(vec![
                LDX_IM, 0x42, // LDX #$42 (No flags set)
                LDX_IM, 0x80, // LDX #$80 (Negative flag set)
                LDX_IM, 0x00, // LDX #$00 (Zero flag set)
            ]);

            // Execute instruction
            processor.step();

            // Check state of processor
            let x = processor.get_registers().x;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(x, 0x42);
            assert!(!zero);
            assert!(!negative);

            // Execute instruction
            processor.step();

            // Check state of processor
            let x = processor.get_registers().x;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(x, 0x80);
            assert!(!zero);
            assert!(negative);

            // Execute instruction
            processor.step();

            // Check state of processor
            let x = processor.get_registers().x;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(x, 0x00);
            assert!(zero);
            assert!(!negative);
        }

        #[test]
        // Load Y register
        fn test_ldy() {
            // Create processor with instruction
            let mut processor = Processor::new(vec![
                LDY_IM, 0x42, // LDY #$42 (No flags set)
                LDY_IM, 0x80, // LDY #$80 (Negative flag set)
                LDY_IM, 0x00, // LDY #$00 (Zero flag set)
            ]);

            // Execute instruction
            processor.step();

            // Check state of processor
            let y = processor.get_registers().y;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(y, 0x42);
            assert!(!zero);
            assert!(!negative);

            // Execute instruction
            processor.step();

            // Check state of processor
            let y = processor.get_registers().y;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(y, 0x80);
            assert!(!zero);
            assert!(negative);

            // Execute instruction
            processor.step();

            // Check state of processor
            let y = processor.get_registers().y;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(y, 0x00);
            assert!(zero);
            assert!(!negative);
        }

        #[test]
        // Store accumulator
        fn test_sta() {
            // Create processor with instruction
            let mut processor = Processor::new(vec![
                STA_ZP, 0x00, // STA $00
            ]);

            // Set processor state
            processor.set_register().acc = 0x42;

            // Execute instruction
            processor.step();

            // Check state of processor
            let acc = processor.get_registers().acc;
            let acc_in_mem = processor.get_mem(0x00);
            assert_eq!(acc, 0x42);
            assert_eq!(acc_in_mem, 0x42);
        }

        #[test]
        // Store X register
        fn test_stx() {
            // Create processor with instruction
            let mut processor = Processor::new(vec![
                STX_ZP, 0x00, // STX $00
            ]);

            // Set processor state
            processor.set_register().x = 0x42;

            // Execute instruction
            processor.step();

            // Check state of processor
            let x = processor.get_registers().x;
            let x_in_mem = processor.get_mem(0x00);
            assert_eq!(x, 0x42);
            assert_eq!(x_in_mem, 0x42);
        }

        #[test]
        // Store Y register
        fn test_sty() {
            // Create processor with instruction
            let mut processor = Processor::new(vec![
                STY_ZP, 0x00, // STY $00
            ]);

            // Set processor state
            processor.set_register().y = 0x42;

            // Execute instruction
            processor.step();

            // Check state of processor
            let y = processor.get_registers().y;
            let y_in_mem = processor.get_mem(0x00);
            assert_eq!(y, 0x42);
            assert_eq!(y_in_mem, 0x42);
        }
    }

    // Transfers
    pub use transfer::*;
    mod transfer {
        use super::*;
        use crate::{
            opcodes::{TAX, TAY, TSX, TXA, TXS, TYA},
            processor::Processor,
        };

        #[test]
        // Transfer accumulator to X register
        fn test_tax() {
            // Create processor with instruction
            let mut processor = Processor::new(vec![
                TAX, // TAX (No flags set)
                TAX, // TAX (Negative flag set)
                TAX, // TAX (Zero flag set)
            ]);

            // Set processor state
            processor.set_register().acc = 0x42;

            // Execute instruction
            processor.step();

            // Check state of processor
            let acc = processor.get_registers().acc;
            let x = processor.get_registers().x;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(acc, 0x42);
            assert_eq!(x, 0x42);
            assert!(!zero);
            assert!(!negative);

            // Set processor state
            processor.set_register().acc = 0x80;

            // Execute instruction
            processor.step();

            // Check state of processor
            let acc = processor.get_registers().acc;
            let x = processor.get_registers().x;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(acc, 0x80);
            assert_eq!(x, 0x80);
            assert!(!zero);
            assert!(negative);

            // Set processor state
            processor.set_register().acc = 0x00;

            // Execute instruction
            processor.step();

            // Check state of processor
            let acc = processor.get_registers().acc;
            let x = processor.get_registers().x;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(acc, 0x00);
            assert_eq!(x, 0x00);
            assert!(zero);
            assert!(!negative);
        }

        #[test]
        // Transfer accumulator to Y register
        fn test_tay() {
            // Create processor with instruction
            let mut processor = Processor::new(vec![
                TAY, // TAY (No flags set)
                TAY, // TAY (Negative flag set)
                TAY, // TAY (Zero flag set)
            ]);

            // Set processor state
            processor.set_register().acc = 0x42;

            // Execute instruction
            processor.step();

            // Check state of processor
            let acc = processor.get_registers().acc;
            let y = processor.get_registers().y;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(acc, 0x42);
            assert_eq!(y, 0x42);
            assert!(!zero);
            assert!(!negative);

            // Set processor state
            processor.set_register().acc = 0x80;

            // Execute instruction
            processor.step();

            // Check state of processor
            let acc = processor.get_registers().acc;
            let y = processor.get_registers().y;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(acc, 0x80);
            assert_eq!(y, 0x80);
            assert!(!zero);
            assert!(negative);

            // Set processor state
            processor.set_register().acc = 0x00;

            // Execute instruction
            processor.step();

            // Check state of processor
            let acc = processor.get_registers().acc;
            let y = processor.get_registers().y;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(acc, 0x00);
            assert_eq!(y, 0x00);
            assert!(zero);
            assert!(!negative);
        }

        #[test]
        // Transfer X register to accumulator
        fn test_txa() {
            // Create processor with instruction
            let mut processor = Processor::new(vec![
                TXA, // TXA (No flags set)
                TXA, // TXA (Negative flag set)
                TXA, // TXA (Zero flag set)
            ]);

            // Set processor state
            processor.set_register().x = 0x42;

            // Execute instruction
            processor.step();

            // Check state of processor
            let acc = processor.get_registers().acc;
            let x = processor.get_registers().x;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(acc, 0x42);
            assert_eq!(x, 0x42);
            assert!(!zero);
            assert!(!negative);

            // Set processor state
            processor.set_register().x = 0x80;

            // Execute instruction
            processor.step();

            // Check state of processor
            let acc = processor.get_registers().acc;
            let x = processor.get_registers().x;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(acc, 0x80);
            assert_eq!(x, 0x80);
            assert!(!zero);
            assert!(negative);

            // Set processor state
            processor.set_register().x = 0x00;

            // Execute instruction
            processor.step();

            // Check state of processor
            let acc = processor.get_registers().acc;
            let x = processor.get_registers().x;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(acc, 0x00);
            assert_eq!(x, 0x00);
            assert!(zero);
            assert!(!negative);
        }

        #[test]
        // Transfer Y register to accumulator
        fn test_tya() {
            // Create processor with instruction
            let mut processor = Processor::new(vec![
                TYA, // TYA (No flags set)
                TYA, // TYA (Negative flag set)
                TYA, // TYA (Zero flag set)
            ]);

            // Set processor state
            processor.set_register().y = 0x42;

            // Execute instruction
            processor.step();

            // Check state of processor
            let acc = processor.get_registers().acc;
            let y = processor.get_registers().y;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(acc, 0x42);
            assert_eq!(y, 0x42);
            assert!(!zero);
            assert!(!negative);

            // Set processor state
            processor.set_register().y = 0x80;

            // Execute instruction
            processor.step();

            // Check state of processor
            let acc = processor.get_registers().acc;
            let y = processor.get_registers().y;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(acc, 0x80);
            assert_eq!(y, 0x80);
            assert!(!zero);
            assert!(negative);

            // Set processor state
            processor.set_register().y = 0x00;

            // Execute instruction
            processor.step();

            // Check state of processor
            let acc = processor.get_registers().acc;
            let y = processor.get_registers().y;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(acc, 0x00);
            assert_eq!(y, 0x00);
            assert!(zero);
            assert!(!negative);
        }

        #[test]
        // Transfer X register to stack pointer
        fn test_txs() {
            // Create processor with instruction
            let mut processor = Processor::new(vec![TXS]);

            // Set processor state
            processor.set_register().x = 0x42;

            // Execute instruction
            processor.step();

            // Check state of processor
            let x = processor.get_registers().x;
            let sp = processor.get_registers().sp;
            assert_eq!(x, 0x42);
            assert_eq!(sp, 0x42);
        }

        #[test]
        // Transfer stack pointer to X register
        fn test_tsx() {
            // Create processor with instruction
            let mut processor = Processor::new(vec![
                TSX, // TSX (No flags set)
                TSX, // TSX (Negative flag set)
                TSX, // TSX (Zero flag set)
            ]);

            // Set processor state
            processor.set_register().sp = 0x42;

            // Execute instruction
            processor.step();

            // Check state of processor
            let x = processor.get_registers().x;
            let sp = processor.get_registers().sp;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(x, 0x42);
            assert_eq!(sp, 0x42);
            assert!(!zero);
            assert!(!negative);

            // Set processor state
            processor.set_register().sp = 0x80;

            // Execute instruction
            processor.step();

            // Check state of processor
            let x = processor.get_registers().x;
            let sp = processor.get_registers().sp;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(x, 0x80);
            assert_eq!(sp, 0x80);
            assert!(!zero);
            assert!(negative);

            // Set processor state
            processor.set_register().sp = 0x00;

            // Execute instruction
            processor.step();

            // Check state of processor
            let x = processor.get_registers().x;
            let sp = processor.get_registers().sp;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(x, 0x00);
            assert_eq!(sp, 0x00);
            assert!(zero);
            assert!(!negative);
        }
    }

    pub use stack::*;
    mod stack {
        use crate::{
            opcodes::{PHA, PHP, PLA, PLP},
            processor::Processor,
            registers::Status,
        };

        #[test]
        // Push accumulator to stack
        fn test_pha() {
            // Create processor with instruction
            let mut processor = Processor::new(vec![
                PHA, // PHA (SP not initialized)
                PHA, // PHA (SP initialized)
            ]);

            // Set processor state
            processor.set_register().acc = 0x42;

            // Execute instruction
            processor.step();

            // Check state of processor
            let acc = processor.get_registers().acc;
            let sp = processor.get_registers().sp;
            let acc_on_stack = processor.get_mem(0x01FF);
            assert_eq!(acc, 0x42);
            assert_eq!(sp, 0xFF);
            assert_eq!(acc_on_stack, 0x42);

            // Set processor state
            processor.set_register().sp = 0xFF;

            // Execute instruction
            processor.step();

            // Check state of processor
            let acc = processor.get_registers().acc;
            let sp = processor.get_registers().sp;
            let acc_on_stack = processor.get_mem(0x01FE);
            assert_eq!(acc, 0x42);
            assert_eq!(sp, 0xFE);
            assert_eq!(acc_on_stack, 0x42);
        }

        #[test]
        // Push processor status to stack
        fn test_php() {
            // Create processor with instruction
            let mut processor = Processor::new(vec![
                PHP, // PHP (SP not initialized)
                PHP, // PHP (SP initialized)
            ]);

            // Set processor state
            processor.set_register().status = Status::from_bits_truncate(0x42);

            // Execute instruction
            processor.step();

            // Check state of processor
            let status = processor.get_registers().status.bits();
            let sp = processor.get_registers().sp;
            let status_on_stack = processor.get_mem(0x01FF);
            assert_eq!(status, 0x42);
            assert_eq!(sp, 0xFF);
            assert_eq!(status_on_stack, 0x42);

            // Set processor state
            processor.set_register().sp = 0xFF;

            // Execute instruction
            processor.step();

            // Check state of processor
            let status = processor.get_registers().status.bits();
            let sp = processor.get_registers().sp;
            let status_on_stack = processor.get_mem(0x01FE);
            assert_eq!(status, 0x42);
            assert_eq!(sp, 0xFE);
            assert_eq!(status_on_stack, 0x42);
        }

        #[test]
        // Pull accumulator from stack
        fn test_pla() {
            // Create processor with instruction
            let mut processor = Processor::new(vec![
                PLA, // PLA (SP not initialized)
                PLA, // PLA (SP initialized)
            ]);

            // Set processor state
            processor.set_mem(0x0100, 0x42);

            // Execute instruction
            processor.step();

            // Check state of processor
            let acc = processor.get_registers().acc;
            let sp = processor.get_registers().sp;
            assert_eq!(acc, 0x42);
            assert_eq!(sp, 0x01);

            // Set processor state
            processor.set_register().sp = 0xFE;
            processor.set_mem(0x01FE, 0x42);

            // Execute instruction
            processor.step();

            // Check state of processor
            let acc = processor.get_registers().acc;
            let sp = processor.get_registers().sp;
            assert_eq!(acc, 0x42);
            assert_eq!(sp, 0xFF);
        }

        #[test]
        // Pull processor status from stack
        fn test_plp() {
            // Create processor with instruction
            let mut processor = Processor::new(vec![
                PLP, // PLP (SP not initialized)
                PLP, // PLP (SP initialized)
            ]);

            // Set processor state
            processor.set_mem(0x0100, 0x42);

            // Execute instruction
            processor.step();

            // Check state of processor
            let status = processor.get_registers().status.bits();
            let sp = processor.get_registers().sp;
            assert_eq!(status, 0x42);
            assert_eq!(sp, 0x01);

            // Set processor state
            processor.set_register().sp = 0xFE;
            processor.set_mem(0x01FE, 0x42);

            // Execute instruction
            processor.step();

            // Check state of processor
            let status = processor.get_registers().status.bits();
            let sp = processor.get_registers().sp;
            assert_eq!(status, 0x42);
            assert_eq!(sp, 0xFF);
        }
    }

    pub use logical::*;
    mod logical {
        use super::get_status;
        use crate::{
            opcodes::{AND_IM, EOR_IM, ORA_IM},
            processor::Processor,
        };

        #[test]
        // Logical AND
        fn test_and() {
            // Create processor with instruction
            let mut processor = Processor::new(vec![
                AND_IM, 0x42, // AND (No flags set)
                AND_IM, 0x00, // AND (Zero flag set)
                AND_IM, 0x80, // AND (Negative flag set)
            ]);

            // Set processor state
            processor.set_register().acc = 0x42;

            // Execute instruction
            processor.step();

            // Check state of processor
            let acc = processor.get_registers().acc;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(acc, 0x42);
            assert!(!zero);
            assert!(!negative);

            // Set processor state
            processor.set_register().acc = 0x00;

            // Execute instruction
            processor.step();

            // Check state of processor
            let acc = processor.get_registers().acc;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(acc, 0x00);
            assert!(zero);
            assert!(!negative);

            // Set processor state
            processor.set_register().acc = 0x80;

            // Execute instruction
            processor.step();

            // Check state of processor
            let acc = processor.get_registers().acc;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(acc, 0x80);
            assert!(!zero);
            assert!(negative);
        }

        #[test]
        // Logical Exclusive OR
        fn test_eor() {
            // Create processor with instruction
            let mut processor = Processor::new(vec![
                EOR_IM, 0x42, // EOR (No flags set)
                EOR_IM, 0x00, // EOR (Zero flag set)
                EOR_IM, 0x80, // EOR (Negative flag set)
            ]);

            // Set processor state
            processor.set_register().acc = 0x3F;

            // Execute instruction
            processor.step();

            // Check state of processor
            let acc = processor.get_registers().acc;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(acc, 0x7D);
            assert!(!zero);
            assert!(!negative);

            // Set processor state
            processor.set_register().acc = 0x00;

            // Execute instruction
            processor.step();

            // Check state of processor
            let acc = processor.get_registers().acc;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(acc, 0x00);
            assert!(zero);
            assert!(!negative);

            // Set processor state
            processor.set_register().acc = 0x01;

            // Execute instruction
            processor.step();

            // Check state of processor
            let acc = processor.get_registers().acc;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(acc, 0x81);
            assert!(!zero);
            assert!(negative);
        }

        #[test]
        // Logical OR
        fn test_or() {
            let mut processor = Processor::new(vec![
                ORA_IM, 0x42, // ORA (No flags set)
                ORA_IM, 0x00, // ORA (Zero flag set)
                ORA_IM, 0x80, // ORA (Negative flag set)
            ]);

            // Set processor state
            processor.set_register().acc = 0x3F;

            // Execute instruction
            processor.step();

            // Check state of processor
            let acc = processor.get_registers().acc;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(acc, 0x7F);
            assert!(!zero);
            assert!(!negative);

            // Set processor state
            processor.set_register().acc = 0x00;

            // Execute instruction
            processor.step();

            // Check state of processor
            let acc = processor.get_registers().acc;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(acc, 0x00);
            assert!(zero);
            assert!(!negative);

            // Set processor state
            processor.set_register().acc = 0x01;

            // Execute instruction
            processor.step();

            // Check state of processor
            let acc = processor.get_registers().acc;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(acc, 0x81);
            assert!(!zero);
            assert!(negative);
        }
    }

    pub use arithmetic::*;
    mod arithmetic {
        use super::get_status;
        use crate::{opcodes::ADC_IM, processor::Processor};

        #[test]
        // Add with carry
        fn test_adc() {
            // Create processor with instruction
            let mut processor = Processor::new(vec![
                ADC_IM, 0x42, // ADC (No flags set)
                ADC_IM, 0x00, // ADC (Zero flag set)
                ADC_IM, 0x80, // ADC (Negative, Overflow flag set)
                ADC_IM, 0x10, // ADC (Overflow, Carry flag set)
            ]);

            // Set processor state
            processor.set_register().acc = 0x00;

            // Execute instruction
            processor.step();

            // Check state of processor
            let acc = processor.get_registers().acc;
            let (zero, negative, carry, overflow, ..) = get_status(&processor);
            assert_eq!(acc, 0x42);
            assert!(!zero);
            assert!(!negative);
            assert!(!carry);
            assert!(!overflow);

            // Set processor state
            processor.set_register().acc = 0x00;

            // Execute instruction
            processor.step();

            // Check state of processor
            let acc = processor.get_registers().acc;
            let (zero, negative, carry, overflow, ..) = get_status(&processor);
            assert_eq!(acc, 0x00);
            assert!(zero);
            assert!(!negative);
            assert!(!carry);
            assert!(!overflow);

            // Set processor state
            processor.set_register().acc = 0x00;

            // Execute instruction
            processor.step();

            // Check state of processor
            let acc = processor.get_registers().acc;
            let (zero, negative, carry, overflow, ..) = get_status(&processor);
            assert_eq!(acc, 0x80);
            assert!(!zero);
            assert!(negative);
            assert!(!carry);
            assert!(overflow);

            // Set processor state
            processor.set_register().acc = 0xFF;

            // Execute instruction
            processor.step();

            // Check state of processor
            let acc = processor.get_registers().acc;
            let (zero, negative, carry, overflow, ..) = get_status(&processor);
            assert_eq!(acc, 0x0F);
            assert!(!zero);
            assert!(!negative);
            assert!(carry);
            assert!(overflow);
        }
    }
}
