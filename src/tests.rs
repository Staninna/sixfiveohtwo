pub use processor::*;
mod processor {
    // Load/store
    pub use load_store::*;
    mod load_store {
        // Load accumulator
        pub use load_acc::*;
        mod load_acc {
            use crate::opcodes::{
                LDA_ABS, LDA_ABSX, LDA_ABSY, LDA_IM, LDA_INDX, LDA_INDY, LDA_ZP, LDA_ZPX,
            };
            use crate::processor::Processor;

            #[test]
            // LDA #42 (Immediate)
            fn test_lda_im() {
                // Create processor with instruction
                let mut processor = Processor::new(vec![
                    LDA_IM, 0x42, // Load accumulator with 0x42
                ]);

                // Execute instruction
                processor.step();

                // Check state of processor
                let acc = processor.get_registers().acc;
                assert_eq!(acc, 0x42);
            }

            #[test]
            // LDA $02 (Zero Page)
            fn test_lda_zp() {
                // Create processor with instruction
                let mut processor = Processor::new(vec![
                    LDA_ZP, 0x02, // Load accumulator with 0x42
                ]);

                // Set state of processor
                processor.set_mem(0x0002, 0x42);

                // Execute instruction
                processor.step();

                // Check state of processor
                let acc = processor.get_registers().acc;
                assert_eq!(acc, 0x42);
            }

            #[test]
            // LDA $02,X (Zero Page, X)
            fn test_lda_zpx() {
                // Create processor with instruction
                let mut processor = Processor::new(vec![
                    LDA_ZPX, 0x02, // Load accumulator with 0x42
                ]);

                // Set state of processor
                processor.set_register().x = 0x01;
                processor.set_mem(0x0003, 0x42);

                // Execute instruction
                processor.step();

                // Check state of processor
                let acc = processor.get_registers().acc;
                assert_eq!(acc, 0x42);
            }

            #[test]
            // LDA $0002 (Absolute)
            fn test_lda_abs() {
                // Create processor with instruction
                let mut processor = Processor::new(vec![
                    LDA_ABS, 0x42, 0x00, // Load accumulator with 0x42
                ]);

                // Set state of processor
                processor.set_mem(0x4200, 0x42);

                // Execute instruction
                processor.step();

                // Check state of processor
                let acc = processor.get_registers().acc;
                assert_eq!(acc, 0x42);
            }

            #[test]
            // LDA $0002,X (Absolute, X)
            fn test_lda_absx() {
                // Create processor with instruction
                let mut processor = Processor::new(vec![
                    LDA_ABSX, 0x42, 0x00, // Load accumulator with 0x42
                ]);

                // Set state of processor
                processor.set_register().x = 0x01;
                processor.set_mem(0x4201, 0x42);

                // Execute instruction
                processor.step();

                // Check state of processor
                let acc = processor.get_registers().acc;
                assert_eq!(acc, 0x42);
            }

            #[test]
            // LDA $0002,Y (Absolute, Y)
            fn test_lda_absy() {
                // Create processor with instruction
                let mut processor = Processor::new(vec![
                    LDA_ABSY, 0x42, 0x00, // Load accumulator with 0x42
                ]);

                // Set state of processor
                processor.set_register().y = 0x01;
                processor.set_mem(0x4201, 0x42);

                // Execute instruction
                processor.step();

                // Check state of processor
                let acc = processor.get_registers().acc;
                assert_eq!(acc, 0x42);
            }

            #[test]
            // LDA ($02,X) (Indirect, X)
            fn test_lda_indx() {
                // Create processor with instruction
                let mut processor = Processor::new(vec![
                    LDA_INDX, 0x02, // Load accumulator with 0x42
                ]);

                // Set state of processor
                processor.set_register().x = 0x01;
                processor.set_mem(0x0003, 0x00);
                processor.set_mem(0x0004, 0x42);
                processor.set_mem(0x4200, 0x42);

                // Execute instruction
                processor.step();

                // Check state of processor
                let acc = processor.get_registers().acc;
                assert_eq!(acc, 0x42);
            }

            #[test]
            // LDA ($02),Y (Indirect, Y)
            fn test_lda_indy() {
                // Create processor with instruction
                let mut processor = Processor::new(vec![
                    LDA_INDY, 0x02, // Load accumulator with 0x42
                ]);

                // Set state of processor
                processor.set_register().y = 0x01;
                processor.set_mem(0x0002, 0x00);
                processor.set_mem(0x0003, 0x42);
                processor.set_mem(0x4201, 0x42);

                // Execute instruction
                processor.step();

                // Check state of processor
                let acc = processor.get_registers().acc;
                assert_eq!(acc, 0x42);
            }
        }

        // Load X
        pub use load_x::*;
        mod load_x {
            use crate::opcodes::{LDX_ABS, LDX_ABSY, LDX_IM, LDX_ZP, LDX_ZPY};
            use crate::processor::Processor;

            #[test]
            // LDX #42 (Immediate)
            fn test_ldx_im() {
                // Create processor with instruction
                let mut processor = Processor::new(vec![
                    LDX_IM, 0x42, // Load X with 0x42
                ]);

                // Execute instruction
                processor.step();

                // Check state of processor
                let x = processor.get_registers().x;
                assert_eq!(x, 0x42);
            }

            #[test]
            // LDX $02 (Zero Page)
            fn test_ldx_zp() {
                // Create processor with instruction
                let mut processor = Processor::new(vec![
                    LDX_ZP, 0x02, // Load X with 0x42
                ]);

                // Set state of processor
                processor.set_mem(0x0002, 0x42);

                // Execute instruction
                processor.step();

                // Check state of processor
                let x = processor.get_registers().x;
                assert_eq!(x, 0x42);
            }

            #[test]
            // LDX $02,Y (Zero Page, Y)
            fn test_ldx_zpy() {
                // Create processor with instruction
                let mut processor = Processor::new(vec![
                    LDX_ZPY, 0x02, // Load X with 0x42
                ]);

                // Set state of processor
                processor.set_register().y = 0x01;
                processor.set_mem(0x0003, 0x42);

                // Execute instruction
                processor.step();

                // Check state of processor
                let x = processor.get_registers().x;
                assert_eq!(x, 0x42);
            }

            #[test]
            // LDX $0002 (Absolute)
            fn test_ldx_abs() {
                // Create processor with instruction
                let mut processor = Processor::new(vec![
                    LDX_ABS, 0x42, 0x00, // Load X with 0x42
                ]);

                // Set state of processor
                processor.set_mem(0x4200, 0x42);

                // Execute instruction
                processor.step();

                // Check state of processor
                let x = processor.get_registers().x;
                assert_eq!(x, 0x42);
            }

            #[test]
            // LDX $0002,Y (Absolute, Y)
            fn test_ldx_absy() {
                // Create processor with instruction
                let mut processor = Processor::new(vec![
                    LDX_ABSY, 0x42, 0x00, // Load X with 0x42
                ]);

                // Set state of processor
                processor.set_register().y = 0x01;
                processor.set_mem(0x4201, 0x42);

                // Execute instruction
                processor.step();

                // Check state of processor
                let x = processor.get_registers().x;
                assert_eq!(x, 0x42);
            }
        }

        // Load Y
        pub use load_y::*;
        mod load_y {
            use crate::opcodes::{LDY_ABS, LDY_ABSX, LDY_IM, LDY_ZP, LDY_ZPX};
            use crate::processor::Processor;

            #[test]
            // LDY #42 (Immediate)
            fn test_ldy_im() {
                // Create processor with instruction
                let mut processor = Processor::new(vec![
                    LDY_IM, 0x42, // Load Y with 0x42
                ]);

                // Execute instruction
                processor.step();

                // Check state of processor
                let y = processor.get_registers().y;
                assert_eq!(y, 0x42);
            }

            #[test]
            // LDY $02 (Zero Page)
            fn test_ldy_zp() {
                // Create processor with instruction
                let mut processor = Processor::new(vec![
                    LDY_ZP, 0x02, // Load Y with 0x42
                ]);

                // Set state of processor
                processor.set_mem(0x0002, 0x42);

                // Execute instruction
                processor.step();

                // Check state of processor
                let y = processor.get_registers().y;
                assert_eq!(y, 0x42);
            }

            #[test]
            // LDY $02,X (Zero Page, X)
            fn test_ldy_zpx() {
                // Create processor with instruction
                let mut processor = Processor::new(vec![
                    LDY_ZPX, 0x02, // Load Y with 0x42
                ]);

                // Set state of processor
                processor.set_register().x = 0x01;
                processor.set_mem(0x0003, 0x42);

                // Execute instruction
                processor.step();

                // Check state of processor
                let y = processor.get_registers().y;
                assert_eq!(y, 0x42);
            }

            #[test]
            // LDY $0002 (Absolute)
            fn test_ldy_abs() {
                // Create processor with instruction
                let mut processor = Processor::new(vec![
                    LDY_ABS, 0x42, 0x00, // Load Y with 0x42
                ]);

                // Set state of processor
                processor.set_mem(0x4200, 0x42);

                // Execute instruction
                processor.step();

                // Check state of processor
                let y = processor.get_registers().y;
                assert_eq!(y, 0x42);
            }

            #[test]
            // LDY $0002,X (Absolute, X)
            fn test_ldy_absx() {
                // Create processor with instruction
                let mut processor = Processor::new(vec![
                    LDY_ABSX, 0x42, 0x00, // Load Y with 0x42
                ]);

                // Set state of processor
                processor.set_register().x = 0x01;
                processor.set_mem(0x4201, 0x42);

                // Execute instruction
                processor.step();

                // Check state of processor
                let y = processor.get_registers().y;
                assert_eq!(y, 0x42);
            }
        }

        // Store Accumulator
        pub use store_acc::*;
        mod store_acc {
            use crate::opcodes::{
                STA_ABS, STA_ABSX, STA_ABSY, STA_INDX, STA_INDY, STA_ZP, STA_ZPX,
            };
            use crate::processor::Processor;

            #[test]
            // STA $02 (Zero Page)
            fn test_sta_zp() {
                // Create processor with instruction
                let mut processor = Processor::new(vec![
                    STA_ZP, 0x02, // Store A with 0x42
                ]);

                // Set state of processor
                processor.set_register().acc = 0x42;

                // Execute instruction
                processor.step();

                // Check state of processor
                let mem = processor.get_mem(0x0002);
                assert_eq!(mem, 0x42);
            }

            #[test]
            // STA $02,X (Zero Page, X)
            fn test_sta_zpx() {
                // Create processor with instruction
                let mut processor = Processor::new(vec![
                    STA_ZPX, 0x02, // Store A with 0x42
                ]);

                // Set state of processor
                processor.set_register().acc = 0x42;
                processor.set_register().x = 0x01;

                // Execute instruction
                processor.step();

                // Check state of processor
                let mem = processor.get_mem(0x0003);
                assert_eq!(mem, 0x42);
            }

            #[test]
            // STA $4200 (Absolute)
            fn test_sta_abs() {
                // Create processor with instruction
                let mut processor = Processor::new(vec![
                    STA_ABS, 0x42, 0x00, // Store A with 0x42
                ]);

                // Set state of processor
                processor.set_register().acc = 0x42;

                // Execute instruction
                processor.step();

                // Check state of processor
                let mem = processor.get_mem(0x4200);
                assert_eq!(mem, 0x42);
            }

            #[test]
            // STA $4200,X (Absolute, X)
            fn test_sta_absx() {
                // Create processor with instruction
                let mut processor = Processor::new(vec![
                    STA_ABSX, 0x42, 0x00, // Store A with 0x42
                ]);

                // Set state of processor
                processor.set_register().acc = 0x42;
                processor.set_register().x = 0x01;

                // Execute instruction
                processor.step();

                // Check state of processor
                let mem = processor.get_mem(0x4201);
                assert_eq!(mem, 0x42);
            }

            #[test]
            // STA $4200,Y (Absolute, Y)
            fn test_sta_absy() {
                // Create processor with instruction
                let mut processor = Processor::new(vec![
                    STA_ABSY, 0x42, 0x00, // Store A with 0x42
                ]);

                // Set state of processor
                processor.set_register().acc = 0x42;
                processor.set_register().y = 0x01;

                // Execute instruction
                processor.step();

                // Check state of processor
                let mem = processor.get_mem(0x4201);
                assert_eq!(mem, 0x42);
            }

            #[test]
            // STA ($02,X) (Indirect, X)
            fn test_sta_indx() {
                // Create processor with instruction
                let mut processor = Processor::new(vec![
                    STA_INDX, 0x02, // Store A with 0x42
                ]);

                // Set state of processor
                processor.set_register().acc = 0x42;
                processor.set_register().x = 0x01;
                processor.set_mem(0x0003, 0x00);
                processor.set_mem(0x0004, 0x42);

                // Execute instruction
                processor.step();

                // Check state of processor
                let mem = processor.get_mem(0x4200);
                assert_eq!(mem, 0x42);
            }

            #[test]
            // STA ($02),Y (Indirect, Y)
            fn test_sta_indy() {
                // Create processor with instruction
                let mut processor = Processor::new(vec![
                    STA_INDY, 0x02, // Store A with 0x42
                ]);

                // Set state of processor
                processor.set_register().acc = 0x42;
                processor.set_register().y = 0x01;
                processor.set_mem(0x0002, 0x00);
                processor.set_mem(0x0003, 0x42);

                // Execute instruction
                processor.step();

                // Check state of processor
                let mem = processor.get_mem(0x4201);
                assert_eq!(mem, 0x42);
            }
        }

        // Store X Register
        pub use store_x::*;
        mod store_x {
            use crate::opcodes::{STX_ABS, STX_ZP, STX_ZPY};
            use crate::processor::Processor;

            #[test]
            // STX $02 (Zero Page)
            fn test_stx_zp() {
                // Create processor with instruction
                let mut processor = Processor::new(vec![
                    STX_ZP, 0x02, // Store X with 0x42
                ]);

                // Set state of processor
                processor.set_register().x = 0x42;

                // Execute instruction
                processor.step();

                // Check state of processor
                let mem = processor.get_mem(0x0002);
                assert_eq!(mem, 0x42);
            }

            #[test]
            // STX $02,Y (Zero Page, Y)
            fn test_stx_zpy() {
                // Create processor with instruction
                let mut processor = Processor::new(vec![
                    STX_ZPY, 0x02, // Store X with 0x42
                ]);

                // Set state of processor
                processor.set_register().x = 0x42;
                processor.set_register().y = 0x01;

                // Execute instruction
                processor.step();

                // Check state of processor
                let mem = processor.get_mem(0x0003);
                assert_eq!(mem, 0x42);
            }

            #[test]
            // STX $4200 (Absolute)
            fn test_stx_abs() {
                // Create processor with instruction
                let mut processor = Processor::new(vec![
                    STX_ABS, 0x42, 0x00, // Store X with 0x42
                ]);

                // Set state of processor
                processor.set_register().x = 0x42;

                // Execute instruction
                processor.step();

                // Check state of processor
                let mem = processor.get_mem(0x4200);
                assert_eq!(mem, 0x42);
            }
        }
    }

    // Transfer
    pub use transfer::*;
    mod transfer {
        // Transfer Accumulator to X Register
        pub use transfer_acc_to_x::*;
        mod transfer_acc_to_x {
            use crate::opcodes::TAX;
            use crate::processor::Processor;

            #[test]
            // TAX (Transfer Accumulator to X)
            fn test_tax() {
                // Create processor with instruction
                let mut processor = Processor::new(vec![
                    TAX, // Transfer Accumulator to X
                ]);

                // Set state of processor
                processor.set_register().acc = 0x42;

                // Execute instruction
                processor.step();

                // Check state of processor
                let reg = processor.get_registers();
                assert_eq!(reg.acc, 0x42);
                assert_eq!(reg.x, 0x42);
            }
        }

        // Transfer Accumulator to Y Register
        pub use transfer_acc_to_y::*;
        mod transfer_acc_to_y {
            use crate::opcodes::TAY;
            use crate::processor::Processor;

            #[test]
            // TAY (Transfer Accumulator to Y)
            fn test_tay() {
                // Create processor with instruction
                let mut processor = Processor::new(vec![
                    TAY, // Transfer Accumulator to Y
                ]);

                // Set state of processor
                processor.set_register().acc = 0x42;

                // Execute instruction
                processor.step();

                // Check state of processor
                let reg = processor.get_registers();
                assert_eq!(reg.acc, 0x42);
                assert_eq!(reg.y, 0x42);
            }
        }

        // Transfer Stack Pointer to X Register
        pub use transfer_x_to_acc::*;
        mod transfer_x_to_acc {
            use crate::opcodes::TXA;
            use crate::processor::Processor;

            #[test]
            // TXA (Transfer X to Accumulator)
            fn test_txa() {
                // Create processor with instruction
                let mut processor = Processor::new(vec![
                    TXA, // Transfer X to Accumulator
                ]);

                // Set state of processor
                processor.set_register().x = 0x42;

                // Execute instruction
                processor.step();

                // Check state of processor
                let reg = processor.get_registers();
                assert_eq!(reg.acc, 0x42);
                assert_eq!(reg.x, 0x42);
            }
        }

        // Transfer Y to Accumulator
        pub use transfer_y_to_acc::*;
        mod transfer_y_to_acc {
            use crate::opcodes::TYA;
            use crate::processor::Processor;

            #[test]
            // TYA (Transfer Y to Accumulator)
            fn test_tya() {
                // Create processor with instruction
                let mut processor = Processor::new(vec![
                    TYA, // Transfer Y to Accumulator
                ]);

                // Set state of processor
                processor.set_register().y = 0x42;

                // Execute instruction
                processor.step();

                // Check state of processor
                let reg = processor.get_registers();
                assert_eq!(reg.acc, 0x42);
                assert_eq!(reg.y, 0x42);
            }
        }
    }
}
