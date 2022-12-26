// NOTE: I only test 1 address mode for each opcode
// Because the implementation is the same for all address modes. The only difference is getting the value from memory.

pub use processor::*;
mod processor {
    use crate::{processor::Processor, registers::Status};
    // Get status register
    fn get_status(processor: &Processor) -> (bool, bool, bool, bool, bool, bool, bool, bool) {
        let st = processor.get_registers().status;
        let has = |s: Status| -> bool { st.contains(s) };
        (
            has(Status::ZERO),
            has(Status::NEGATIVE),
            has(Status::CARRY),
            has(Status::OVERFLOW),
            has(Status::DECIMAL),
            has(Status::INTERRUPT),
            has(Status::BREAK),
            has(Status::UNUSED),
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
        // LDA - Load Accumulator
        fn test_lda() {
            // Create a new processor
            let mut processor = Processor::new(vec![
                LDA_IM, 0x42, // LDA #$42 ; No flags
                LDA_IM, 0x00, // LDA #$00 ; Zero flag
                LDA_IM, 0x80, // LDA #$FF ; Negative flag
            ]);

            // Execute instruction
            processor.step();

            // Check processor state
            let acc = processor.get_registers().acc;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(acc, 0x42);
            assert!(!zero);
            assert!(!negative);

            // Execute instruction
            processor.step();

            // Check processor state
            let acc = processor.get_registers().acc;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(acc, 0x00);
            assert!(zero);
            assert!(!negative);

            // Execute instruction
            processor.step();

            // Check processor state
            let acc = processor.get_registers().acc;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(acc, 0x80);
            assert!(!zero);
            assert!(negative);
        }

        #[test]
        // LDX - Load X Register
        fn test_ldx() {
            // Create a new processor
            let mut processor = Processor::new(vec![
                LDX_IM, 0x42, // LDX #$42 ; No flags
                LDX_IM, 0x00, // LDX #$00 ; Zero flag
                LDX_IM, 0x80, // LDX #$FF ; Negative flag
            ]);

            // Execute instruction
            processor.step();

            // Check processor state
            let x = processor.get_registers().x;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(x, 0x42);
            assert!(!zero);
            assert!(!negative);

            // Execute instruction
            processor.step();

            // Check processor state
            let x = processor.get_registers().x;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(x, 0x00);
            assert!(zero);
            assert!(!negative);

            // Execute instruction
            processor.step();

            // Check processor state
            let x = processor.get_registers().x;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(x, 0x80);
            assert!(!zero);
            assert!(negative);
        }

        #[test]
        // LDY - Load Y Register
        fn test_ldy() {
            // Create a new processor
            let mut processor = Processor::new(vec![
                LDY_IM, 0x42, // LDY #$42 ; No flags
                LDY_IM, 0x00, // LDY #$00 ; Zero flag
                LDY_IM, 0x80, // LDY #$FF ; Negative flag
            ]);

            // Execute instruction
            processor.step();

            // Check processor state
            let y = processor.get_registers().y;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(y, 0x42);
            assert!(!zero);
            assert!(!negative);

            // Execute instruction
            processor.step();

            // Check processor state
            let y = processor.get_registers().y;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(y, 0x00);
            assert!(zero);
            assert!(!negative);

            // Execute instruction
            processor.step();

            // Check processor state
            let y = processor.get_registers().y;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(y, 0x80);
            assert!(!zero);
            assert!(negative);
        }

        #[test]
        // STA - Store Accumulator
        fn test_sta() {
            // Create a new processor
            let mut processor = Processor::new(vec![
                STA_ZP, 0x42, // STA $42
            ]);

            // Set state of processor
            processor.set_register().acc = 0x42;

            // Execute instruction
            processor.step();

            // Check processor state
            let acc = processor.get_registers().acc;
            assert_eq!(acc, 0x42);
        }

        #[test]
        // STX - Store X Register
        fn test_stx() {
            // Create a new processor
            let mut processor = Processor::new(vec![
                STX_ZP, 0x42, // STX $42
            ]);

            // Set state of processor
            processor.set_register().x = 0x42;

            // Execute instruction
            processor.step();

            // Check processor state
            let x = processor.get_registers().x;
            assert_eq!(x, 0x42);
        }

        #[test]
        // STY - Store Y Register
        fn test_sty() {
            // Create a new processor
            let mut processor = Processor::new(vec![
                STY_ZP, 0x42, // STY $42
            ]);

            // Set state of processor
            processor.set_register().y = 0x42;

            // Execute instruction
            processor.step();

            // Check processor state
            let y = processor.get_registers().y;
            assert_eq!(y, 0x42);
        }
    }

    // Transfer
    pub use transfer::*;
    mod transfer {
        use super::*;
        use crate::{
            opcodes::{TAX, TAY, TSX, TXA, TXS, TYA},
            processor::Processor,
        };

        #[test]
        // TAX - Transfer Accumulator to X
        fn test_tax() {
            // Create a new processor
            let mut processor = Processor::new(vec![
                TAX, // TAX ; No flags
                TAX, // TAX ; Zero flag
                TAX, // TAX ; Negative flag
            ]);

            // Set state of processor
            processor.set_register().acc = 0x42;

            // Execute instruction
            processor.step();

            // Check processor state
            let acc = processor.get_registers().acc;
            let x = processor.get_registers().x;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(acc, 0x42);
            assert_eq!(x, 0x42);
            assert!(!zero);
            assert!(!negative);

            // Set state of processor
            processor.set_register().acc = 0x00;

            // Execute instruction
            processor.step();

            // Check processor state
            let acc = processor.get_registers().acc;
            let x = processor.get_registers().x;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(acc, 0x00);
            assert_eq!(x, 0x00);
            assert!(zero);
            assert!(!negative);

            // Set state of processor
            processor.set_register().acc = 0x80;

            // Execute instruction
            processor.step();

            // Check processor state
            let acc = processor.get_registers().acc;
            let x = processor.get_registers().x;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(acc, 0x80);
            assert_eq!(x, 0x80);
            assert!(!zero);
            assert!(negative);
        }

        #[test]
        // TAY - Transfer Accumulator to Y
        fn test_tay() {
            // Create a new processor
            let mut processor = Processor::new(vec![
                TAY, // TAY ; No flags
                TAY, // TAY ; Zero flag
                TAY, // TAY ; Negative flag
            ]);

            // Set state of processor
            processor.set_register().acc = 0x42;

            // Execute instruction
            processor.step();

            // Check processor state
            let acc = processor.get_registers().acc;
            let y = processor.get_registers().y;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(acc, 0x42);
            assert_eq!(y, 0x42);
            assert!(!zero);
            assert!(!negative);

            // Set state of processor
            processor.set_register().acc = 0x00;

            // Execute instruction
            processor.step();

            // Check processor state
            let acc = processor.get_registers().acc;
            let y = processor.get_registers().y;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(acc, 0x00);
            assert_eq!(y, 0x00);
            assert!(zero);
            assert!(!negative);

            // Set state of processor
            processor.set_register().acc = 0x80;

            // Execute instruction
            processor.step();

            // Check processor state
            let acc = processor.get_registers().acc;
            let y = processor.get_registers().y;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(acc, 0x80);
            assert_eq!(y, 0x80);
            assert!(!zero);
            assert!(negative);
        }

        #[test]
        // TXA - Transfer X to Accumulator
        fn test_txa() {
            // Create a new processor
            let mut processor = Processor::new(vec![
                TXA, // TXA ; No flags
                TXA, // TXA ; Zero flag
                TXA, // TXA ; Negative flag
            ]);

            // Set state of processor
            processor.set_register().x = 0x42;

            // Execute instruction
            processor.step();

            // Check processor state
            let acc = processor.get_registers().acc;
            let x = processor.get_registers().x;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(acc, 0x42);
            assert_eq!(x, 0x42);
            assert!(!zero);
            assert!(!negative);

            // Set state of processor
            processor.set_register().x = 0x00;

            // Execute instruction
            processor.step();

            // Check processor state
            let acc = processor.get_registers().acc;
            let x = processor.get_registers().x;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(acc, 0x00);
            assert_eq!(x, 0x00);
            assert!(zero);
            assert!(!negative);

            // Set state of processor
            processor.set_register().x = 0x80;

            // Execute instruction
            processor.step();

            // Check processor state
            let acc = processor.get_registers().acc;
            let x = processor.get_registers().x;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(acc, 0x80);
            assert_eq!(x, 0x80);
            assert!(!zero);
            assert!(negative);
        }

        #[test]
        // TYA - Transfer Y to Accumulator
        fn test_tya() {
            // Create a new processor
            let mut processor = Processor::new(vec![
                TYA, // TYA ; No flags
                TYA, // TYA ; Zero flag
                TYA, // TYA ; Negative flag
            ]);

            // Set state of processor
            processor.set_register().y = 0x42;

            // Execute instruction
            processor.step();

            // Check processor state
            let acc = processor.get_registers().acc;
            let y = processor.get_registers().y;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(acc, 0x42);
            assert_eq!(y, 0x42);
            assert!(!zero);
            assert!(!negative);

            // Set state of processor
            processor.set_register().y = 0x00;

            // Execute instruction
            processor.step();

            // Check processor state
            let acc = processor.get_registers().acc;
            let y = processor.get_registers().y;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(acc, 0x00);
            assert_eq!(y, 0x00);
            assert!(zero);
            assert!(!negative);

            // Set state of processor
            processor.set_register().y = 0x80;

            // Execute instruction
            processor.step();

            // Check processor state
            let acc = processor.get_registers().acc;
            let y = processor.get_registers().y;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(acc, 0x80);
            assert_eq!(y, 0x80);
            assert!(!zero);
            assert!(negative);
        }

        #[test]
        // TSX - Transfer Stack Pointer to X
        fn test_tsx() {
            // Create a new processor
            let mut processor = Processor::new(vec![
                TSX, // TSX ; No flags
                TSX, // TSX ; Zero flag
                TSX, // TSX ; Negative flag
            ]);

            // Set state of processor
            processor.set_register().sp = 0x42;

            // Execute instruction
            processor.step();

            // Check processor state
            let sp = processor.get_registers().sp;
            let x = processor.get_registers().x;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(sp, 0x42);
            assert_eq!(x, 0x42);
            assert!(!zero);
            assert!(!negative);

            // Set state of processor
            processor.set_register().sp = 0x00;

            // Execute instruction
            processor.step();

            // Check processor state
            let sp = processor.get_registers().sp;
            let x = processor.get_registers().x;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(sp, 0x00);
            assert_eq!(x, 0x00);
            assert!(zero);
            assert!(!negative);

            // Set state of processor
            processor.set_register().sp = 0x80;

            // Execute instruction
            processor.step();

            // Check processor state
            let sp = processor.get_registers().sp;
            let x = processor.get_registers().x;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(sp, 0x80);
            assert_eq!(x, 0x80);
            assert!(!zero);
            assert!(negative);
        }

        #[test]
        // TXS - Transfer X to Stack Pointer
        fn test_txs() {
            // Create a new processor
            let mut processor = Processor::new(vec![
                TXS, // TXS
            ]);

            // Set state of processor
            processor.set_register().x = 0x42;

            // Execute instruction
            processor.step();

            // Check processor state
            let sp = processor.get_registers().sp;
            let x = processor.get_registers().x;
            assert_eq!(sp, 0x42);
            assert_eq!(x, 0x42);
        }
    }

    pub use stack::*;
    mod stack {
        use super::*;
        use crate::{
            opcodes::{PHA, PHP, PLA, PLP},
            processor::Processor,
        };

        #[test]
        // PHA - Push Accumulator to Stack
        fn test_pha() {
            // Create a new processor
            let mut processor = Processor::new(vec![
                PHA, // PHA ; Not initialized
                PHA, // PHA ; Initialized
            ]);

            // Set state of processor
            processor.set_register().acc = 0x42;

            // Execute instruction
            processor.step();

            // Check processor state
            let acc = processor.get_registers().acc;
            let sp = processor.get_registers().sp;
            assert_eq!(acc, 0x42);
            assert_eq!(sp, 0xFF);

            // Set state of processor
            processor.set_register().acc = 0x43;
            processor.set_register().sp = 0xFE;

            // Execute instruction
            processor.step();

            // Check processor state
            let acc = processor.get_registers().acc;
            let sp = processor.get_registers().sp;
            assert_eq!(acc, 0x43);
            assert_eq!(sp, 0xFD);
        }

        #[test]
        // PHP - Push Processor Status to Stack
        fn test_php() {
            // Create a new processor
            let mut processor = Processor::new(vec![
                PHP, // PHP ; Not initialized
                PHP, // PHP ; Initialized
            ]);

            // Set state of processor
            processor.set_register().status = Status::from_bits_truncate(0x42);

            // Execute instruction
            processor.step();

            // Check processor state
            let status = processor.get_registers().status.bits();
            let sp = processor.get_registers().sp;
            assert_eq!(status, 0x42);
            assert_eq!(sp, 0xFF);

            // Set state of processor
            processor.set_register().status = Status::from_bits_truncate(0x43);
            processor.set_register().sp = 0xFE;

            // Execute instruction
            processor.step();

            // Check processor state
            let status = processor.get_registers().status.bits();
            let sp = processor.get_registers().sp;
            assert_eq!(status, 0x43);
            assert_eq!(sp, 0xFD);
        }

        #[test]
        // PLA - Pull Accumulator from Stack
        fn test_pla() {
            // Create a new processor
            let mut processor = Processor::new(vec![
                PLA, // PLA ; Not initialized
                PLA, // PLA ; Initialized
            ]);

            // Set state of processor
            processor.set_mem(0x0100, 0x42);

            // Execute instruction
            processor.step();

            // Check processor state
            let acc = processor.get_registers().acc;
            let sp = processor.get_registers().sp;
            assert_eq!(acc, 0x42);
            assert_eq!(sp, 0x01);

            // Set state of processor
            processor.set_register().sp = 0xFE;
            processor.set_mem(0x01FE, 0x43);

            // Execute instruction
            processor.step();

            // Check processor state
            let acc = processor.get_registers().acc;
            let sp = processor.get_registers().sp;
            assert_eq!(acc, 0x43);
            assert_eq!(sp, 0xFF);
        }

        #[test]
        // PLP - Pull Processor Status from Stack
        fn test_plp() {
            // Create a new processor
            let mut processor = Processor::new(vec![
                PLP, // PLP ; Not initialized
                PLP, // PLP ; Initialized
            ]);

            // Set state of processor
            processor.set_mem(0x0100, 0x42);

            // Execute instruction
            processor.step();

            // Check processor state
            let status = processor.get_registers().status.bits();
            let sp = processor.get_registers().sp;
            assert_eq!(status, 0x42);
            assert_eq!(sp, 0x01);

            // Set state of processor
            processor.set_register().sp = 0xFE;
            processor.set_mem(0x01FE, 0x43);

            // Execute instruction
            processor.step();

            // Check processor state
            let status = processor.get_registers().status.bits();
            let sp = processor.get_registers().sp;
            assert_eq!(status, 0x43);
            assert_eq!(sp, 0xFF);
        }
    }

    // Logical
    pub use logical::*;
    mod logical {
        use super::*;
        use crate::{
            opcodes::{AND_IM, BIT_ZP, EOR_IM, ORA_IM},
            processor::Processor,
        };

        #[test]
        // AND - Logical AND
        fn test_and() {
            // Create a new processor
            let mut processor = Processor::new(vec![
                AND_IM, 0x42, // AND #$42 ; No flags
                AND_IM, 0x00, // AND #$43 ; Zero
                AND_IM, 0x80, // AND #$44 ; Negative
            ]);

            // Set state of processor
            processor.set_register().acc = 0x42;

            // Execute instruction
            processor.step();

            // Check processor state
            let acc = processor.get_registers().acc;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(acc, 0x42);
            assert!(!zero);
            assert!(!negative);

            // Set state of processor
            processor.set_register().acc = 0x00;

            // Execute instruction
            processor.step();

            // Check processor state
            let acc = processor.get_registers().acc;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(acc, 0x00);
            assert!(zero);
            assert!(!negative);

            // Set state of processor
            processor.set_register().acc = 0x80;

            // Execute instruction
            processor.step();

            // Check processor state
            let acc = processor.get_registers().acc;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(acc, 0x80);
            assert!(!zero);
            assert!(negative);
        }

        #[test]
        // EOR - Logical Exclusive OR
        fn test_eor() {
            // Create a new processor
            let mut processor = Processor::new(vec![
                EOR_IM, 0x42, // EOR #$42 ; No flags
                EOR_IM, 0x00, // EOR #$43 ; Zero
                EOR_IM, 0x80, // EOR #$44 ; Negative
            ]);

            // Set state of processor
            processor.set_register().acc = 0x43;

            // Execute instruction
            processor.step();

            // Check processor state
            let acc = processor.get_registers().acc;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(acc, 0x01);
            assert!(!zero);
            assert!(!negative);

            // Set state of processor
            processor.set_register().acc = 0x00;

            // Execute instruction
            processor.step();

            // Check processor state
            let acc = processor.get_registers().acc;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(acc, 0x00);
            assert!(zero);
            assert!(!negative);

            // Set state of processor
            processor.set_register().acc = 0x00;

            // Execute instruction
            processor.step();

            // Check processor state
            let acc = processor.get_registers().acc;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(acc, 0x80);
            assert!(!zero);
            assert!(negative);
        }

        #[test]
        // ORA - Logical Inclusive OR
        fn test_ora() {
            // Create a new processor
            let mut processor = Processor::new(vec![
                ORA_IM, 0x42, // ORA #$42 ; No flags
                ORA_IM, 0x00, // ORA #$43 ; Zero
                ORA_IM, 0x80, // ORA #$44 ; Negative
            ]);

            // Set state of processor
            processor.set_register().acc = 0x42;

            // Execute instruction
            processor.step();

            // Check processor state
            let acc = processor.get_registers().acc;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(acc, 0x42);
            assert!(!zero);
            assert!(!negative);

            // Set state of processor
            processor.set_register().acc = 0x00;

            // Execute instruction
            processor.step();

            // Check processor state
            let acc = processor.get_registers().acc;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(acc, 0x00);
            assert!(zero);
            assert!(!negative);

            // Set state of processor
            processor.set_register().acc = 0x00;

            // Execute instruction
            processor.step();

            // Check processor state
            let acc = processor.get_registers().acc;
            let (zero, negative, ..) = get_status(&processor);
            assert_eq!(acc, 0x80);
            assert!(!zero);
            assert!(negative);
        }

        #[test]
        // BIT - Bit Test
        fn test_bit() {
            // Create a new processor
            let mut processor = Processor::new(vec![
                BIT_ZP, 0x00, // BIT $00 ; No flags
                BIT_ZP, 0x01, // BIT $01 ; Zero
                BIT_ZP, 0x02, // BIT $02 ; Negative
                BIT_ZP, 0x03, // BIT $03 ; Overflow
            ]);

            // Set state of processor
            processor.set_register().acc = 0x24;
            processor.set_mem(0x0000, 0x42);
            processor.set_mem(0x0001, 0x00);
            processor.set_mem(0x0002, 0x80);
            processor.set_mem(0x0003, 0x40);

            // Set state of processor
            processor.set_register().acc = 0x00;

            // Execute instruction
            processor.step();

            // Check processor state
            let (zero, negative, _carry, overflow, ..) = get_status(&processor);
            assert!(zero);
            assert!(!negative);
            assert!(!overflow);

            // Set state of processor
            processor.set_register().acc = 0x00;

            // Execute instruction
            processor.step();

            // Check processor state
            let (zero, negative, _carry, overflow, ..) = get_status(&processor);
            assert!(zero);
            assert!(!negative);
            assert!(!overflow);

            // Set state of processor
            processor.set_register().acc = 0x80;

            // Execute instruction
            processor.step();

            // Check processor state
            let (zero, negative, _carry, overflow, ..) = get_status(&processor);
            assert!(!zero);
            assert!(negative);
            assert!(!overflow);

            // Set state of processor
            processor.set_register().acc = 0x40;

            // Execute instruction
            processor.step();

            // Check processor state
            let (zero, negative, _carry, overflow, ..) = get_status(&processor);
            assert!(!zero);
            assert!(!negative);
            assert!(overflow);
        }
    }

    // Arithmetic
    pub use arithmetic::*;
    mod arithmetic {
        use super::*;
        use crate::{opcodes::ADC_IM, processor::Processor};

        #[test]
        // ADC - Add with Carry
        fn adc_test() {
            let mut processor = Processor::new(vec![
                ADC_IM, 0x42, // ADC #$42 ; No flags
                ADC_IM, 0x00, // ADC #$00 ; Zero
                ADC_IM, 0x80, // ADC #$80 ; Negative
                ADC_IM, 0x20, // ADC #$20 ; Overflow
                ADC_IM, 0xFF, // ADC #$FF ; Carry
            ]);

            // Set state of processor
            processor.set_register().acc = 0x27;

            // Execute instruction
            processor.step();

            // Check processor state
            let acc = processor.get_registers().acc;
            let (zero, negative, carry, overflow, ..) = get_status(&processor);
            // 0x42 + 0x27 = 0x69
            assert_eq!(acc, 0x69);
            assert!(!zero);
            assert!(!negative);
            assert!(!carry);
            assert!(!overflow);

            // Set state of processor
            processor.set_register().acc = 0x00;

            // Execute instruction
            processor.step();

            // Check processor state
            let acc = processor.get_registers().acc;
            let (zero, negative, carry, overflow, ..) = get_status(&processor);
            // 0x00 + 0x00 = 0x00
            assert_eq!(acc, 0x00);
            assert!(zero);
            assert!(!negative);
            assert!(!carry);
            assert!(!overflow);

            // Set state of processor
            processor.set_register().acc = 0x05;

            // Execute instruction
            processor.step();

            // Check processor state
            let acc = processor.get_registers().acc;
            let (zero, negative, carry, overflow, ..) = get_status(&processor);
            // 0x80 + 0x05 = 0x85
            assert_eq!(acc, 0x85);
            assert!(!zero);
            assert!(negative);
            assert!(!carry);
            assert!(!overflow);

            // Set state of processor
            processor.set_register().acc = 0x7F;

            // Execute instruction
            processor.step();

            // Check processor state
            let acc = processor.get_registers().acc;
            let (zero, negative, carry, overflow, ..) = get_status(&processor);
            // 0x20 + 0x7F = 0x9F
            assert_eq!(acc, 0x9F);
            assert!(!zero);
            assert!(negative);
            assert!(!carry);
            assert!(overflow);

            // Set state of processor
            processor.set_register().acc = 0x7F;

            // Execute instruction
            processor.step();

            // Check processor state
            let acc = processor.get_registers().acc;
            let (zero, negative, carry, overflow, ..) = get_status(&processor);
            // 0xFF + 0x7F = 0x7E
            assert_eq!(acc, 0x7E);
            assert!(!zero);
            assert!(!negative);
            assert!(carry);
            assert!(!overflow);
        }
    }
}
