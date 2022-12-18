#[cfg(test)]
mod processor {
    use super::super::opcodes::*;
    use super::super::processor::Processor;

    // Test instructions

    // LDA - Load Accumulator

    #[test]
    // Test LDA_IM instruction
    fn test_lda_im() {
        let mut processor = Processor::new_offset(
            vec![
                LDA_IM, 0x42, // Load accumulator with 0x42
            ],
            0x0000,
        );

        // Set program counter

        // Execute instruction
        processor.step();

        // Check accumulator
        let acc = processor.get_registers().acc;
        assert_eq!(acc, 0x42);
    }

    #[test]
    // Test LDA_ZP instruction
    fn test_lda_zp() {
        let mut processor = Processor::new_offset(
            vec![
                LDA_ZP, 0x02, // Load accumulator from 0x0002
                0x42, //-------- Hardcoded value
            ],
            0x0000,
        );

        // Set program counter

        // Execute instruction
        processor.step();

        // Check accumulator
        let acc = processor.get_registers().acc;
        assert_eq!(acc, 0x42);
    }

    #[test]
    // Test LDA_ZPX instruction
    fn test_lda_zpx() {
        let mut processor = Processor::new_offset(
            vec![
                LDX_IM, 0x01, //- Load X with 0x01
                LDA_ZPX, 0x03, // Load accumulator from 0x0003 + X (0x0004)
                0x42, //--------- Hardcoded value
            ],
            0x0000,
        );

        // Set program counter

        // Execute instructions
        processor.step();
        processor.step();

        // Check accumulator
        let acc = processor.get_registers().acc;
        assert_eq!(acc, 0x42);
    }

    #[test]
    // Test LDA_ABS instruction
    fn test_lda_abs() {
        let mut processor = Processor::new_offset(
            vec![
                LDA_ABS, 0x00, 0x03, // Load accumulator from 0x0004
                0x42, //--------------- Hardcoded value
            ],
            0x0000,
        );

        // Set program counter

        // Execute instruction
        processor.step();

        // Check accumulator
        let acc = processor.get_registers().acc;
        assert_eq!(acc, 0x42);
    }

    #[test]
    // Test LDA_ABSX instruction
    fn test_lda_absx() {
        let mut processor = Processor::new_offset(
            vec![
                LDX_IM, 0x01, //-------- Load X with 0x01
                LDA_ABSX, 0x00, 0x04, // Load accumulator from 0x0004 + X (0x0005)
                0x42, //---------------- Hardcoded value
            ],
            0x0000,
        );

        // Set program counter

        // Execute instructions
        processor.step();
        processor.step();

        // Check accumulator
        let acc = processor.get_registers().acc;
        assert_eq!(acc, 0x42);
    }

    #[test]
    // Test LDA_ABSY instruction
    fn test_lda_absy() {
        let mut processor = Processor::new_offset(
            vec![
                LDY_IM, 0x01, //-------- Load Y with 0x01
                LDA_ABSY, 0x00, 0x04, // Load accumulator from 0x0004 + Y (0x0005)
                0x42, //---------------- Hardcoded value
            ],
            0x0000,
        );

        // Set program counter

        // Execute instructions
        processor.step();
        processor.step();

        // Check accumulator
        let acc = processor.get_registers().acc;
        assert_eq!(acc, 0x42);
    }

    #[test]
    // Test LDA_INDX instruction
    fn test_lda_indx() {
        let mut processor = Processor::new_offset(
            vec![
                LDX_IM, 0x01, //-- Load X with 0x01
                LDA_INDX, 0x03, // Load accumulator from 0x0003 + X (0x0004)
                0x06, 0x00, //---- Hardcoded address
                0x42, //---------- Hardcoded value
            ],
            0x0000,
        );

        // Set program counter

        // Execute instructions
        processor.step();
        processor.step();

        // Check accumulator
        let acc = processor.get_registers().acc;
        assert_eq!(acc, 0x42);
    }

    #[test]
    // Test LDA_INDY instruction
    fn test_lda_indy() {
        let mut processor = Processor::new_offset(
            vec![
                LDY_IM, 0x01, //-- Load Y with 0x01
                LDA_INDY, 0x04, // Load accumulator from 0x0004
                0x05, 0x00, //---- Hardcoded address + Y (0x0006)
                0x42, //---------- Hardcoded value
            ],
            0x0000,
        );

        // Set program counter

        // Execute instructions
        processor.step();
        processor.step();

        // Check accumulator
        let acc = processor.get_registers().acc;
        assert_eq!(acc, 0x42);
    }

    // LDX - Load X

    #[test]
    // Test LDX_IM instruction
    fn test_ldx_im() {
        let mut processor = Processor::new_offset(
            vec![
                LDX_IM, 0x42, // Load X with 0x03
            ],
            0x0000,
        );

        // Set program counter

        // Execute instruction
        processor.step();

        // Check X
        let x = processor.get_registers().x;
        assert_eq!(x, 0x42);
    }

    #[test]
    // Test LDX_ZP instruction
    fn test_ldx_zp() {
        let mut processor = Processor::new_offset(
            vec![
                LDX_ZP, 0x02, // Load X from 0x0002
                0x42, //-------- Hardcoded value
            ],
            0x0000,
        );

        // Set program counter

        // Execute instruction
        processor.step();

        // Check X
        let x = processor.get_registers().x;
        assert_eq!(x, 0x42);
    }

    #[test]
    // Test LDX_ZPY instruction
    fn test_ldx_zpy() {
        let mut processor = Processor::new_offset(
            vec![
                LDY_IM, 0x01, //- Load Y with 0x01
                LDX_ZPY, 0x03, // Load X from 0x0003 + Y (0x0004)
                0x42, //--------- Hardcoded value
            ],
            0x0000,
        );

        // Set program counter

        // Execute instructions
        processor.step();
        processor.step();

        // Check X
        let x = processor.get_registers().x;
        assert_eq!(x, 0x42);
    }

    #[test]
    // Test LDX_ABS instruction
    fn test_ldx_abs() {
        let mut processor = Processor::new_offset(
            vec![
                LDX_ABS, 0x00, 0x03, // Load X from 0x0003
                0x42, //--------------- Hardcoded value
            ],
            0x0000,
        );

        // Set program counter

        // Execute instruction
        processor.step();

        // Check X
        let x = processor.get_registers().x;
        assert_eq!(x, 0x42);
    }

    #[test]
    // Test LDX_ABSY instruction
    fn test_ldx_absy() {
        let mut processor = Processor::new_offset(
            vec![
                LDY_IM, 0x01, //-------- Load Y with 0x01
                LDX_ABSY, 0x00, 0x04, // Load X from 0x0004 + Y (0x0005)
                0x42, //---------------- Hardcoded value
            ],
            0x0000,
        );

        // Set program counter

        // Execute instructions
        processor.step();
        processor.step();

        // Check X
        let x = processor.get_registers().x;
        assert_eq!(x, 0x42);
    }

    // LDY - Load Y

    #[test]
    // Test LDY_IM instruction
    fn test_ldy_im() {
        let mut processor = Processor::new_offset(
            vec![
                LDY_IM, 0x42, // Load Y with 0x03
            ],
            0x0000,
        );

        // Set program counter

        // Execute instruction
        processor.step();

        // Check Y
        let y = processor.get_registers().y;
        assert_eq!(y, 0x42);
    }

    #[test]
    // Test LDY_ZP instruction
    fn test_ldy_zp() {
        let mut processor = Processor::new_offset(
            vec![
                LDY_ZP, 0x02, // Load Y from 0x0002
                0x42, //-------- Hardcoded value
            ],
            0x0000,
        );

        // Set program counter

        // Execute instruction
        processor.step();

        // Check Y
        let y = processor.get_registers().y;
        assert_eq!(y, 0x42);
    }

    #[test]
    // Test LDY_ZPX instruction
    fn test_ldy_zpx() {
        let mut processor = Processor::new_offset(
            vec![
                LDX_IM, 0x01, //- Load X with 0x01
                LDY_ZPX, 0x03, // Load Y from 0x0003 + X (0x0004)
                0x42, //--------- Hardcoded value
            ],
            0x0000,
        );

        // Set program counter

        // Execute instructions
        processor.step();
        processor.step();

        // Check Y
        let y = processor.get_registers().y;
        assert_eq!(y, 0x42);
    }

    #[test]
    // Test LDY_ABS instruction
    fn test_ldy_abs() {
        let mut processor = Processor::new_offset(
            vec![
                LDY_ABS, 0x00, 0x03, // Load Y from 0x0003
                0x42, //--------------- Hardcoded value
            ],
            0x0000,
        );

        // Set program counter

        // Execute instruction
        processor.step();

        // Check Y
        let y = processor.get_registers().y;
        assert_eq!(y, 0x42);
    }

    #[test]
    // Test LDY_ABSX instruction
    fn test_ldy_absx() {
        let mut processor = Processor::new_offset(
            vec![
                LDX_IM, 0x01, //-------- Load X with 0x01
                LDY_ABSX, 0x00, 0x04, // Load Y from 0x0004 + X (0x0005)
                0x42, //---------------- Hardcoded value
            ],
            0x0000,
        );

        // Set program counter

        // Execute instructions
        processor.step();
        processor.step();

        // Check Y
        let y = processor.get_registers().y;
        assert_eq!(y, 0x42);
    }

    // Transfer instructions

    #[test]
    // Test TAX instruction
    fn test_tax() {
        let mut processor = Processor::new_offset(
            vec![
                LDA_IM, 0x42, // Load A with 0x42
                TAX,  //-------- Transfer A to X
            ],
            0x0000,
        );

        // Set program counter

        // Execute instructions
        processor.step();
        processor.step();

        // Check X
        let x = processor.get_registers().x;
        assert_eq!(x, 0x42);
    }

    #[test]
    // Test TAY instruction
    fn test_tay() {
        let mut processor = Processor::new_offset(
            vec![
                LDA_IM, 0x42, // Load A with 0x42
                TAY,  //-------- Transfer A to Y
            ],
            0x0000,
        );

        // Set program counter

        // Execute instructions
        processor.step();
        processor.step();

        // Check Y
        let y = processor.get_registers().y;
        assert_eq!(y, 0x42);
    }

    #[test]
    // Test TXA instruction
    fn test_txa() {
        let mut processor = Processor::new_offset(
            vec![
                LDX_IM, 0x42, // Load X with 0x42
                TXA,  //-------- Transfer X to A
            ],
            0x0000,
        );

        // Execute instructions
        processor.step();
        processor.step();

        // Check A
        let a = processor.get_registers().acc;
        assert_eq!(a, 0x42);
    }

    #[test]
    // Test TYA instruction
    fn test_tya() {
        let mut processor = Processor::new_offset(
            vec![
                LDY_IM, 0x42, // Load Y with 0x42
                TYA,  //-------- Transfer Y to A
            ],
            0x0000,
        );

        // Set program counter

        // Execute instructions
        processor.step();
        processor.step();

        // Check A
        let a = processor.get_registers().acc;
        assert_eq!(a, 0x42);
    }
}
