use dvi::opcodes::OpCode;

pub fn disassemble(bytes: Vec<u8>) -> Vec<OpCode> {
    let mut disassembler = Disassembler::new(bytes);
    disassembler.disassemble()
}

struct Disassembler {
    bytes: Vec<u8>,
    position: usize,
}

impl Disassembler {
    fn new(bytes: Vec<u8>) -> Disassembler {
        Disassembler {
            bytes: bytes,
            position: 0,
        }
    }

    fn disassemble(&mut self) -> Vec<OpCode> {
        self.position = 0;
        let mut opcodes = Vec::new();

        while self.has_more() {
            let opcode = self.disassemble_next();
            opcodes.push(opcode);
        }

        opcodes
    }

    fn disassemble_next(&mut self) -> OpCode {
        let byte = self.consume_one_byte() as u8;
        match byte {
            0...127 => self.handle_set_char(byte),
            128 => self.handle_set1(),
            129 => self.handle_set2(),
            130 => self.handle_set3(),
            131 => self.handle_set4(),
            132 => self.handle_set_rule(),
            133 => self.handle_put1(),
            134 => self.handle_put2(),
            135 => self.handle_put3(),
            136 => self.handle_put4(),
            137 => self.handle_put_rule(),
            138 => self.handle_nop(),
            139 => self.handle_bop(),
            140 => self.handle_eop(),
            141 => self.handle_push(),
            142 => self.handle_pop(),
            143 => self.handle_right1(),
            144 => self.handle_right2(),
            145 => self.handle_right3(),
            146 => self.handle_right4(),
            147 => self.handle_w0(),
            148 => self.handle_w1(),
            149 => self.handle_w2(),
            150 => self.handle_w3(),
            151 => self.handle_w4(),
            152 => self.handle_x0(),
            153 => self.handle_x1(),
            154 => self.handle_x2(),
            155 => self.handle_x3(),
            156 => self.handle_x4(),
            157 => self.handle_down1(),
            158 => self.handle_down2(),
            159 => self.handle_down3(),
            160 => self.handle_down4(),
            161 => self.handle_y0(),
            162 => self.handle_y1(),
            163 => self.handle_y2(),
            164 => self.handle_y3(),
            165 => self.handle_y4(),
            166 => self.handle_z0(),
            167 => self.handle_z1(),
            168 => self.handle_z2(),
            169 => self.handle_z3(),
            170 => self.handle_z4(),
            171...234 => self.handle_fnt_num(byte),
            _ => panic!("Unknown opcode: {}", byte),
        }
    }

    // Set

    fn handle_set_char(&mut self, byte: u8) -> OpCode {
        OpCode::SetChar { c: byte as u32}
    }

    fn handle_set1(&mut self) -> OpCode {
        OpCode::Set1 {
            c: self.consume_one_byte(),
        }
    }

    fn handle_set2(&mut self) -> OpCode {
        OpCode::Set2 {
            c: self.consume_two_bytes(),
        }
    }

    fn handle_set3(&mut self) -> OpCode {
        OpCode::Set3 {
            c: self.consume_three_bytes(),
        }
    }

    fn handle_set4(&mut self) -> OpCode {
        OpCode::Set4 {
            c: self.consume_four_bytes(),
        }
    }

    fn handle_set_rule(&mut self) -> OpCode {
        OpCode::SetRule {
            a: self.consume_four_bytes(),
            b: self.consume_four_bytes(),
        }
    }

    // Put

    fn handle_put1(&mut self) -> OpCode {
        OpCode::Put1 {
            c: self.consume_one_byte(),
        }
    }

    fn handle_put2(&mut self) -> OpCode {
        OpCode::Put2 {
            c: self.consume_two_bytes(),
        }
    }

    fn handle_put3(&mut self) -> OpCode {
        OpCode::Put3 {
            c: self.consume_three_bytes(),
        }
    }

    fn handle_put4(&mut self) -> OpCode {
        OpCode::Put4 {
            c: self.consume_four_bytes(),
        }
    }

    fn handle_put_rule(&mut self) -> OpCode {
        OpCode::PutRule {
            a: self.consume_four_bytes(),
            b: self.consume_four_bytes(),
        }
    }

    fn handle_nop(&mut self) -> OpCode {
        OpCode::Nop
    }

    fn handle_bop(&mut self) -> OpCode {
        OpCode::Bop {
            c0: self.consume_four_bytes(),
            c1: self.consume_four_bytes(),
            c2: self.consume_four_bytes(),
            c3: self.consume_four_bytes(),
            c4: self.consume_four_bytes(),
            c5: self.consume_four_bytes(),
            c6: self.consume_four_bytes(),
            c7: self.consume_four_bytes(),
            c8: self.consume_four_bytes(),
            c9: self.consume_four_bytes(),
            p: self.consume_four_bytes(),
        }
    }

    fn handle_eop(&mut self) -> OpCode {
        OpCode::Eop
    }

    fn handle_push(&mut self) -> OpCode {
        OpCode::Push
    }

    fn handle_pop(&mut self) -> OpCode {
        OpCode::Pop
    }

    // Right
    
    fn handle_right1(&mut self) -> OpCode {
        OpCode::Right1 {
            b: self.consume_one_byte() as i32,
        }
    }

    fn handle_right2(&mut self) -> OpCode {
        OpCode::Right2 {
            b: self.consume_two_bytes() as i32,
        }
    }

    fn handle_right3(&mut self) -> OpCode {
        OpCode::Right3 {
            b: self.consume_three_bytes() as i32,
        }
    }

    fn handle_right4(&mut self) -> OpCode {
        OpCode::Right4 {
            b: self.consume_four_bytes(),
        }
    }

    // W

    fn handle_w0(&mut self) -> OpCode {
        OpCode::W0
    }    

    fn handle_w1(&mut self) -> OpCode {
        OpCode::W1 {
            b: self.consume_one_byte() as i32,
        }
    }

    fn handle_w2(&mut self) -> OpCode {
        OpCode::W2 {
            b: self.consume_two_bytes() as i32,
        }
    }

    fn handle_w3(&mut self) -> OpCode {
        OpCode::W3 {
            b: self.consume_three_bytes() as i32,
        }
    }

    fn handle_w4(&mut self) -> OpCode {
        OpCode::W4 {
            b: self.consume_four_bytes(),
        }
    }        

    // X

    fn handle_x0(&mut self) -> OpCode {
        OpCode::X0
    }    

    fn handle_x1(&mut self) -> OpCode {
        OpCode::X1 {
            b: self.consume_one_byte() as i32,
        }
    }

    fn handle_x2(&mut self) -> OpCode {
        OpCode::X2 {
            b: self.consume_two_bytes() as i32,
        }
    }

    fn handle_x3(&mut self) -> OpCode {
        OpCode::X3 {
            b: self.consume_three_bytes() as i32,
        }
    }

    fn handle_x4(&mut self) -> OpCode {
        OpCode::X4 {
            b: self.consume_four_bytes(),
        }
    }

    // Down

    fn handle_down1(&mut self) -> OpCode {
        OpCode::Down1 {
            a: self.consume_one_byte() as i32,
        }
    }

    fn handle_down2(&mut self) -> OpCode {
        OpCode::Down2 {
            a: self.consume_two_bytes() as i32,
        }
    }

    fn handle_down3(&mut self) -> OpCode {
        OpCode::Down3 {
            a: self.consume_three_bytes() as i32,
        }
    }

    fn handle_down4(&mut self) -> OpCode {
        OpCode::Down4 {
            a: self.consume_four_bytes(),
        }
    }

    // Y

    fn handle_y0(&mut self) -> OpCode {
        OpCode::Y0
    }    

    fn handle_y1(&mut self) -> OpCode {
        OpCode::Y1 {
            a: self.consume_one_byte() as i32,
        }
    }

    fn handle_y2(&mut self) -> OpCode {
        OpCode::Y2 {
            a: self.consume_two_bytes() as i32,
        }
    }

    fn handle_y3(&mut self) -> OpCode {
        OpCode::Y3 {
            a: self.consume_three_bytes() as i32,
        }
    }

    fn handle_y4(&mut self) -> OpCode {
        OpCode::Y4 {
            a: self.consume_four_bytes(),
        }
    }

    // Y

    fn handle_z0(&mut self) -> OpCode {
        OpCode::Z0
    }

    fn handle_z1(&mut self) -> OpCode {
        OpCode::Z1 {
            a: self.consume_one_byte() as i32,
        }
    }

    fn handle_z2(&mut self) -> OpCode {
        OpCode::Z2 {
            a: self.consume_two_bytes() as i32,
        }
    }

    fn handle_z3(&mut self) -> OpCode {
        OpCode::Z3 {
            a: self.consume_three_bytes() as i32,
        }
    }

    fn handle_z4(&mut self) -> OpCode {
        OpCode::Z4 {
            a: self.consume_four_bytes(),
        }
    }

    // Fonts

    fn handle_fnt_num(&mut self, byte: u8) -> OpCode {
        OpCode::FntNum { k: byte as u32}
    }

    // Read bytes

    fn consume_one_byte(&mut self) -> u32 {
        self.consume_n_bytes(1)
    }

    fn consume_two_bytes(&mut self) -> u32 {
        self.consume_n_bytes(2)
    }

    fn consume_three_bytes(&mut self) -> u32 {
        self.consume_n_bytes(3)
    }

    fn consume_four_bytes(&mut self) -> i32 {
        self.consume_n_bytes(4) as i32
    }

    fn consume_n_bytes(&mut self, n: u32) -> u32 {
        debug_assert!(n <= 4, "Can at most read u32 with n == 4");

        let mut result: u32 = 0;
        for i in (0..n).rev() {
            // Bytes are in big endian
            let byte = self.bytes[self.position] as u32;
            self.position += 1;
            result |= byte << (8 * i);
        }

        result
    }

    fn has_more(&self) -> bool {
        self.position < self.bytes.len()
    }
}

#[cfg(test)]
mod tests {
    use dvi::disassembler::disassemble;
    use dvi::opcodes::OpCode;

    #[test]
    fn test_disassemble_set_char() {
        for i in 0..127 + 1 {
            let result = disassemble(vec![i]);

            assert_that_opcode_was_generated(result, OpCode::SetChar { c: i as u32 })
        }
    }

    #[test]
    fn test_disassemble_set1() {
        let result = disassemble(vec![128, 0xAB]);

        assert_that_opcode_was_generated(result, OpCode::Set1 { c: 0xAB })
    }

    #[test]
    fn test_disassemble_set2() {
        let result = disassemble(vec![129, 0xAB, 0xCD]);

        assert_that_opcode_was_generated(result, OpCode::Set2 { c: 0xABCD })
    }

    #[test]
    fn test_disassemble_set3() {
        let result = disassemble(vec![130, 0xAB, 0xCD, 0xEF]);

        assert_that_opcode_was_generated(result, OpCode::Set3 { c: 0xABCDEF })
    }

    #[test]
    fn test_disassemble_set4() {
        let result = disassemble(vec![131, 0x00, 0x11, 0x22, 0x33]);

        assert_that_opcode_was_generated(result, OpCode::Set4 { c: 0x112233 })
    }

    #[test]
    fn test_disassemble_set_rule() {
        let result = disassemble(vec![132, 0x0, 0xAB, 0xCD, 0xEF, 0x0, 0xFE, 0xDC, 0xBA]);

        assert_that_opcode_was_generated(
            result,
            OpCode::SetRule {
                a: 0xABCDEF,
                b: 0xFEDCBA,
            },
        )
    }

    #[test]
    fn test_disassemble_put1() {
        let result = disassemble(vec![133, 0xAB]);

        assert_that_opcode_was_generated(result, OpCode::Put1 { c: 0xAB })
    }

    #[test]
    fn test_disassemble_put2() {
        let result = disassemble(vec![134, 0xAB, 0xCD]);

        assert_that_opcode_was_generated(result, OpCode::Put2 { c: 0xABCD })
    }

    #[test]
    fn test_disassemble_put3() {
        let result = disassemble(vec![135, 0xAB, 0xCD, 0xEF]);

        assert_that_opcode_was_generated(result, OpCode::Put3 { c: 0xABCDEF })
    }

    #[test]
    fn test_disassemble_put4() {
        let result = disassemble(vec![136, 0x00, 0x11, 0x22, 0x33]);

        assert_that_opcode_was_generated(result, OpCode::Put4 { c: 0x112233 })
    }

    #[test]
    fn test_disassemble_put_rule() {
        let result = disassemble(vec![137, 0x0, 0xAB, 0xCD, 0xEF, 0x0, 0xFE, 0xDC, 0xBA]);

        assert_that_opcode_was_generated(
            result,
            OpCode::PutRule {
                a: 0xABCDEF,
                b: 0xFEDCBA,
            },
        )
    }

    #[test]
    fn test_disassemble_nop() {
        let result = disassemble(vec![138]);

        assert_that_opcode_was_generated(result, OpCode::Nop)
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn test_disassemble_bop() {
        let result = disassemble(vec![
            139, 
            0x01, 0x11, 0x11, 0x11,
            0x02, 0x22, 0x22, 0x22,
            0x03, 0x33, 0x33, 0x33,
            0x04, 0x44, 0x44, 0x44,
            0x05, 0x55, 0x55, 0x55,
            0x06, 0x66, 0x66, 0x66,
            0x07, 0x77, 0x77, 0x77,
            0x08, 0x88, 0x88, 0x88,
            0x09, 0x99, 0x99, 0x99,
            0x0A, 0xAA, 0xAA, 0xAA,
            0x0C, 0xAF, 0xEB, 0xAE,
        ]);

        assert_that_opcode_was_generated(
            result,
            OpCode::Bop {
                c0: 0x1111111,
                c1: 0x2222222,
                c2: 0x3333333,
                c3: 0x4444444,
                c4: 0x5555555,
                c5: 0x6666666,
                c6: 0x7777777,
                c7: 0x8888888,
                c8: 0x9999999,
                c9: 0xAAAAAAA,
                p:  0xCAFEBAE,
            }
        )
    }

    #[test]
    fn test_disassemble_eop() {
        let result = disassemble(vec![140]);

        assert_that_opcode_was_generated(result, OpCode::Eop)
    }

    #[test]
    fn test_disassemble_push() {
        let result = disassemble(vec![141]);

        assert_that_opcode_was_generated(result, OpCode::Push)
    }

    #[test]
    fn test_disassemble_pop() {
        let result = disassemble(vec![142]);

        assert_that_opcode_was_generated(result, OpCode::Pop)
    }

    #[test]
    fn test_disassemble_right1() {
        let result = disassemble(vec![143, 0xAB]);

        assert_that_opcode_was_generated(result, OpCode::Right1 { b: 0xAB })
    }

    #[test]
    fn test_disassemble_right2() {
        let result = disassemble(vec![144, 0xAB, 0xCD]);

        assert_that_opcode_was_generated(result, OpCode::Right2 { b: 0xABCD })
    }

    #[test]
    fn test_disassemble_right3() {
        let result = disassemble(vec![145, 0xAB, 0xCD, 0xEF]);

        assert_that_opcode_was_generated(result, OpCode::Right3 { b: 0xABCDEF })
    }

    #[test]
    fn test_disassemble_right4() {
        let result = disassemble(vec![146, 0x00, 0x11, 0x22, 0x33]);

        assert_that_opcode_was_generated(result, OpCode::Right4 { b: 0x112233 })
    }

    // W

   #[test]
    fn test_disassemble_w0() {
        let result = disassemble(vec![147]);

        assert_that_opcode_was_generated(result, OpCode::W0)
    }

   #[test]
    fn test_disassemble_w1() {
        let result = disassemble(vec![148, 0xAB]);

        assert_that_opcode_was_generated(result, OpCode::W1 { b: 0xAB })
    }

    #[test]
    fn test_disassemble_w2() {
        let result = disassemble(vec![149, 0xAB, 0xCD]);

        assert_that_opcode_was_generated(result, OpCode::W2 { b: 0xABCD })
    }

    #[test]
    fn test_disassemble_w3() {
        let result = disassemble(vec![150, 0xAB, 0xCD, 0xEF]);

        assert_that_opcode_was_generated(result, OpCode::W3 { b: 0xABCDEF })
    }

    #[test]
    fn test_disassemble_w4() {
        let result = disassemble(vec![151, 0x00, 0x11, 0x22, 0x33]);

        assert_that_opcode_was_generated(result, OpCode::W4 { b: 0x112233 })
    }

    // X

   #[test]
    fn test_disassemble_x0() {
        let result = disassemble(vec![152]);

        assert_that_opcode_was_generated(result, OpCode::X0)
    }

   #[test]
    fn test_disassemble_x1() {
        let result = disassemble(vec![153, 0xAB]);

        assert_that_opcode_was_generated(result, OpCode::X1 { b: 0xAB })
    }

    #[test]
    fn test_disassemble_x2() {
        let result = disassemble(vec![154, 0xAB, 0xCD]);

        assert_that_opcode_was_generated(result, OpCode::X2 { b: 0xABCD })
    }

    #[test]
    fn test_disassemble_x3() {
        let result = disassemble(vec![155, 0xAB, 0xCD, 0xEF]);

        assert_that_opcode_was_generated(result, OpCode::X3 { b: 0xABCDEF })
    }

    #[test]
    fn test_disassemble_x4() {
        let result = disassemble(vec![156, 0x00, 0x11, 0x22, 0x33]);

        assert_that_opcode_was_generated(result, OpCode::X4 { b: 0x112233 })
    }   

    // Down

   #[test]
    fn test_disassemble_down1() {
        let result = disassemble(vec![157, 0xAB]);

        assert_that_opcode_was_generated(result, OpCode::Down1 { a: 0xAB })
    }

    #[test]
    fn test_disassemble_down2() {
        let result = disassemble(vec![158, 0xAB, 0xCD]);

        assert_that_opcode_was_generated(result, OpCode::Down2 { a: 0xABCD })
    }

    #[test]
    fn test_disassemble_down3() {
        let result = disassemble(vec![159, 0xAB, 0xCD, 0xEF]);

        assert_that_opcode_was_generated(result, OpCode::Down3 { a: 0xABCDEF })
    }

    #[test]
    fn test_disassemble_down4() {
        let result = disassemble(vec![160, 0x00, 0x11, 0x22, 0x33]);

        assert_that_opcode_was_generated(result, OpCode::Down4 { a: 0x112233 })
    }

    // Y

   #[test]
    fn test_disassemble_y0() {
        let result = disassemble(vec![161]);

        assert_that_opcode_was_generated(result, OpCode::Y0)
    }

   #[test]
    fn test_disassemble_y1() {
        let result = disassemble(vec![162, 0xAB]);

        assert_that_opcode_was_generated(result, OpCode::Y1 { a: 0xAB })
    }

    #[test]
    fn test_disassemble_y2() {
        let result = disassemble(vec![163, 0xAB, 0xCD]);

        assert_that_opcode_was_generated(result, OpCode::Y2 { a: 0xABCD })
    }

    #[test]
    fn test_disassemble_y3() {
        let result = disassemble(vec![164, 0xAB, 0xCD, 0xEF]);

        assert_that_opcode_was_generated(result, OpCode::Y3 { a: 0xABCDEF })
    }

    #[test]
    fn test_disassemble_y4() {
        let result = disassemble(vec![165, 0x00, 0x11, 0x22, 0x33]);

        assert_that_opcode_was_generated(result, OpCode::Y4 { a: 0x112233 })
    }

    // Z

   #[test]
    fn test_disassemble_z0() {
        let result = disassemble(vec![166]);

        assert_that_opcode_was_generated(result, OpCode::Z0)
    }

   #[test]
    fn test_disassemble_z1() {
        let result = disassemble(vec![167, 0xAB]);

        assert_that_opcode_was_generated(result, OpCode::Z1 { a: 0xAB })
    }

    #[test]
    fn test_disassemble_z2() {
        let result = disassemble(vec![168, 0xAB, 0xCD]);

        assert_that_opcode_was_generated(result, OpCode::Z2 { a: 0xABCD })
    }

    #[test]
    fn test_disassemble_z3() {
        let result = disassemble(vec![169, 0xAB, 0xCD, 0xEF]);

        assert_that_opcode_was_generated(result, OpCode::Z3 { a: 0xABCDEF })
    }

    #[test]
    fn test_disassemble_z4() {
        let result = disassemble(vec![170, 0x00, 0x11, 0x22, 0x33]);

        assert_that_opcode_was_generated(result, OpCode::Z4 { a: 0x112233 })
    }

    // Font

    #[test]
    fn test_disassemble_fnt_num() {
        for i in 172..234 + 1 {
            let result = disassemble(vec![i]);

            assert_that_opcode_was_generated(result, OpCode::FntNum { k: i as u32 })
        }
    }

    // Helper

    fn assert_that_opcode_was_generated(result: Vec<OpCode>, opcode: OpCode) {
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], opcode);
    }
}
