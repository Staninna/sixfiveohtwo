#[cfg(test)]
// Load and store instructions
pub use load_store::*;
mod load_store {
    pub use load_acc::*;
    mod load_acc {
        use crate::opcodes::*;
        use crate::processor::Processor;

        #[test]
        // Test the accumulator load immediate instruction
        fn test_lda_immediate() {
            let mut processor = Processor::new_offset(
                vec![
                    LDA_IM, 0x42, // Load 0x42 into the accumulator
                ],
                0x0000,
            );
            processor.step();

            let acc = processor.read_registers().acc;
            assert_eq!(acc, 0x42);
        }

        #[test]
        // Test the accumulator load zero page instruction
        fn test_lda_zero_page() {
            let mut processor = Processor::new_offset(
                vec![
                    LDA_ZP, 0x02, // Load 0x42 into acc
                    0x42,
                ],
                0x0000,
            );
            processor.step();

            let acc = processor.read_registers().acc;
            assert_eq!(acc, 0x42);
        }

        #[test]
        // Test the accumulator load zero page x instruction
        fn test_lda_zero_page_x() {
            let mut processor = Processor::new_offset(
                vec![
                    LDX_IM, 0x03, //- Load 0x03 into X
                    LDA_ZPX, 0x01, // Load 0x01 + X (0x03) = 0x04 (0x42) into acc
                    0x42, //--------- Hardcoded value
                ],
                0x0000,
            );
            processor.step();
            processor.step();

            let acc = processor.read_registers().acc;
            assert_eq!(acc, 0x42);
        }

        #[test]
        // Test the accumulator load absolute instruction
        fn test_lda_absolute() {
            let mut processor = Processor::new_offset(
                vec![
                    LDA_ABS, 0x00, 0x03, // Load 0x03 (0x42) into acc
                    0x42, // -------------- Hardcoded value
                ],
                0x0000,
            );
            processor.step();

            let acc = processor.read_registers().acc;
            assert_eq!(acc, 0x42);
        }

        #[test]
        // Test the accumulator load absolute x instruction
        fn test_lda_absolute_x() {
            let mut processor = Processor::new_offset(
                vec![
                    LDX_IM, 0x04, //-------- Load 0x04 into X
                    LDA_ABSX, 0x00, 0x01, // Load 0x01 + X (0x04) = 0x05 (0x42) into acc
                    0x42, //---------------- Hardcoded value
                ],
                0x0000,
            );
            processor.step();
            processor.step();

            let acc = processor.read_registers().acc;
            assert_eq!(acc, 0x42);
        }

        #[test]
        // Test the accumulator load absolute y instruction
        fn test_lda_absolute_y() {
            let mut processor = Processor::new_offset(
                vec![
                    LDY_IM, 0x04, //-------- Load 0x04 into Y
                    LDA_ABSY, 0x00, 0x01, // Load 0x01 + Y (0x04) = 0x05 (0x42) into acc
                    0x42, //---------------- Hardcoded value
                ],
                0x0000,
            );
            processor.step();
            processor.step();

            let acc = processor.read_registers().acc;
            assert_eq!(acc, 0x42);
        }

        #[test]
        fn test_lda_indirect_x() {
            let mut processor = Processor::new_offset(
                vec![
                    LDX_IM, 0x02, //-- Load 0x02 into X
                    LDA_INDX, 0x02, // Load 0x02 + X (0x02) = pointer to 0x0006 (0x42)
                    0x06, 0x00, //---- Hardcoded value (pointer address)
                    0x42, //---------- Hardcoded value (value at pointer address)
                ],
                0x0000,
            );

            processor.step();
            processor.step();

            let acc = processor.read_registers().acc;
            assert_eq!(acc, 0x42);
        }

        #[test]
        fn test_lda_indirect_y() {
            let mut processor = Processor::new_offset(
                vec![
                    LDY_IM, 0x02, //-- Load 0x02 into Y
                    LDA_INDY, 0x04, // Load 0x0000 + 0x04 (0x0004) = \
                    //                          pointer to 0x0006 + Y (0x02) = 0x0008 (0x42)
                    0x06, 0x00, //---- Hardcoded value (pointer address)
                    0x00, 0x00, //---- Padding
                    0x42, //---------- Hardcoded value (value at pointer address)
                ],
                0x0000,
            );

            processor.step();
            processor.step();

            let acc = processor.read_registers().acc;
            assert_eq!(acc, 0x42);
        }
    }

    pub use load_x::*;
    mod load_x {
        use crate::opcodes::*;
        use crate::processor::Processor;

        #[test]
        // Test the X register load immediate instruction
        fn test_ldx_immediate() {
            let mut processor = Processor::new_offset(
                vec![
                    LDX_IM, 0x42, // Load 0x42 into the X register
                ],
                0x0000,
            );
            processor.step();

            let x = processor.read_registers().x;
            assert_eq!(x, 0x42);
        }

        #[test]
        // Test the X register load zero page instruction
        fn test_ldx_zero_page() {
            let mut processor = Processor::new_offset(
                vec![
                    LDX_ZP, 0x02, // Load 0x42 into X
                    0x42,
                ],
                0x0000,
            );
            processor.step();

            let x = processor.read_registers().x;
            assert_eq!(x, 0x42);
        }

        #[test]
        // Test the X register load zero page y instruction
        fn test_ldx_zero_page_y() {
            let mut processor = Processor::new_offset(
                vec![
                    LDY_IM, 0x03, //- Load 0x03 into Y
                    LDX_ZPY, 0x01, // Load 0x01 + Y (0x03) = 0x04 (0x42) into X
                    0x42, //--------- Hardcoded value
                ],
                0x0000,
            );
            processor.step();
            processor.step();

            let x = processor.read_registers().x;
            assert_eq!(x, 0x42);
        }

        #[test]
        // Test the X register load absolute instruction
        fn test_ldx_absolute() {
            let mut processor = Processor::new_offset(
                vec![
                    LDX_ABS, 0x00, 0x03, // Load 0x03 (0x42) into X
                    0x42, // -------------- Hardcoded value
                ],
                0x0000,
            );
            processor.step();

            let x = processor.read_registers().x;
            assert_eq!(x, 0x42);
        }

        #[test]
        // Test the X register load absolute y instruction
        fn test_ldx_absolute_y() {
            let mut processor = Processor::new_offset(
                vec![
                    LDY_IM, 0x04, //-------- Load 0x04 into Y
                    LDX_ABSY, 0x00, 0x01, // Load 0x01 + Y (0x04) = 0x05 (0x42) into X
                    0x42, //---------------- Hardcoded value
                ],
                0x0000,
            );
            processor.step();
            processor.step();

            let x = processor.read_registers().x;
            assert_eq!(x, 0x42);
        }
    }

    pub use load_y::*;
    mod load_y {
        use crate::opcodes::*;
        use crate::processor::Processor;

        #[test]
        // Test the Y register load immediate instruction
        fn test_ldy_immediate() {
            let mut processor = Processor::new_offset(
                vec![
                    LDY_IM, 0x42, // Load 0x42 into the Y register
                ],
                0x0000,
            );
            processor.step();

            let y = processor.read_registers().y;
            assert_eq!(y, 0x42);
        }

        #[test]
        // Test the Y register load zero page instruction
        fn test_ldy_zero_page() {
            let mut processor = Processor::new_offset(
                vec![
                    LDY_ZP, 0x02, // Load 0x42 into Y
                    0x42,
                ],
                0x0000,
            );
            processor.step();

            let y = processor.read_registers().y;
            assert_eq!(y, 0x42);
        }

        #[test]
        // Test the Y register load zero page x instruction
        fn test_ldy_zero_page_x() {
            let mut processor = Processor::new_offset(
                vec![
                    LDX_IM, 0x03, //- Load 0x03 into X
                    LDY_ZPX, 0x01, // Load 0x01 + X (0x03) = 0x04 (0x42) into Y
                    0x42, //--------- Hardcoded value
                ],
                0x0000,
            );
            processor.step();
            processor.step();

            let y = processor.read_registers().y;
            assert_eq!(y, 0x42);
        }

        #[test]
        // Test the Y register load absolute instruction
        fn test_ldy_absolute() {
            let mut processor = Processor::new_offset(
                vec![
                    LDY_ABS, 0x00, 0x03, // Load 0x03 (0x42) into Y
                    0x42, // -------------- Hardcoded value
                ],
                0x0000,
            );
            processor.step();

            let y = processor.read_registers().y;
            assert_eq!(y, 0x42);
        }

        #[test]
        // Test the Y register load absolute x instruction
        fn test_ldy_absolute_x() {
            let mut processor = Processor::new_offset(
                vec![
                    LDX_IM, 0x04, //-------- Load 0x04 into X
                    LDY_ABSX, 0x00, 0x01, // Load 0x01 + X (0x04) = 0x05 (0x42) into Y
                    0x42, //---------------- Hardcoded value
                ],
                0x0000,
            );
            processor.step();
            processor.step();

            let y = processor.read_registers().y;
            assert_eq!(y, 0x42);
        }
    }

    pub use store_acc::*;
    mod store_acc {
        use super::super::super::processor::Processor;
        use crate::opcodes::*;

        #[test]
        // Test the accumulator store zero page instruction
        fn test_sta_zero_page() {
            let mut processor = Processor::new_offset(
                vec![
                    LDA_IM, 0x42, // Load 0x42 into the accumulator
                    STA_ZP, 0x02, // Store the accumulator into 0x02
                ],
                0x0000,
            );
            processor.step();
            processor.step();

            let acc = processor.read_memory(0x02);
            assert_eq!(acc, 0x42);
        }

        #[test]
        // Test the accumulator store zero page x instruction
        fn test_sta_zero_page_x() {
            let mut processor = Processor::new_offset(
                vec![
                    LDX_IM, 0x03, //- Load 0x03 into X
                    LDA_IM, 0x42, //- Load 0x42 into the accumulator
                    STA_ZPX, 0x01, // Store the accumulator into 0x01 + X (0x03) = 0x04
                ],
                0x0000,
            );
            processor.step();
            processor.step();
            processor.step();

            let acc = processor.read_memory(0x04);
            assert_eq!(acc, 0x42);
        }

        #[test]
        // Test the accumulator store absolute instruction
        fn test_sta_absolute() {
            let mut processor = Processor::new_offset(
                vec![
                    LDA_IM, 0x42, //------- Load 0x42 into the accumulator
                    STA_ABS, 0x00, 0x03, // Store the accumulator into 0x03
                ],
                0x0000,
            );
            processor.step();
            processor.step();

            let acc = processor.read_memory(0x03);
            assert_eq!(acc, 0x42);
        }

        #[test]
        // Test the accumulator store absolute x instruction
        fn test_sta_absolute_x() {
            let mut processor = Processor::new_offset(
                vec![
                    LDX_IM, 0x04, //-------- Load 0x04 into X
                    LDA_IM, 0x42, //-------- Load 0x42 into the accumulator
                    STA_ABSX, 0x00, 0x01, // Store the accumulator into 0x01 + X (0x04) = 0x05
                ],
                0x0000,
            );
            processor.step();
            processor.step();
            processor.step();

            let acc = processor.read_memory(0x05);
            assert_eq!(acc, 0x42);
        }

        #[test]
        // Test the accumulator store absolute y instruction
        fn test_sta_absolute_y() {
            let mut processor = Processor::new_offset(
                vec![
                    LDY_IM, 0x04, //-------- Load 0x04 into Y
                    LDA_IM, 0x42, //-------- Load 0x42 into the accumulator
                    STA_ABSY, 0x00, 0x01, // Store the accumulator into 0x01 + Y (0x04) = 0x05
                ],
                0x0000,
            );
            processor.step();
            processor.step();
            processor.step();

            let acc = processor.read_memory(0x05);
            assert_eq!(acc, 0x42);
        }
    }
}
