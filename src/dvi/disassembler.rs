use dvi::opcodes::OpCode;

pub fn disassemble(bytes: Vec<u8>) -> Result<Vec<OpCode>, String> {
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

    fn disassemble(&mut self) -> Result<Vec<OpCode>, String> {
        self.position = 0;
        let mut opcodes = Vec::new();

        while self.has_more() {
            let opcode = match self.disassemble_next() {
                Err(why) => return Err(why),
                Ok(opcode) => opcode
            };
            opcodes.push(opcode);
        }

        Ok(opcodes)
    }

    fn disassemble_next(&mut self) -> Result<OpCode, String> {
        let byte = self.consume_one_byte_as_scalar() as u8;
        let opcode = match byte {
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
            235 => self.handle_fnt1(),
            236 => self.handle_fnt2(),
            237 => self.handle_fnt3(),
            238 => self.handle_fnt4(),
            239 => self.handle_xxx1(),
            240 => self.handle_xxx2(),
            241 => self.handle_xxx3(),
            242 => self.handle_xxx4(),
            243 => self.handle_fnt_def1(),
            244 => self.handle_fnt_def2(),
            245 => self.handle_fnt_def3(),
            246 => self.handle_fnt_def4(),
            _ => return Err(format!("Unknown opcode: {}", byte)),
        };
        Ok(opcode)
    }

    // Set

    fn handle_set_char(&mut self, byte: u8) -> OpCode {
        OpCode::SetChar { c: byte as u32}
    }

    fn handle_set1(&mut self) -> OpCode {
        OpCode::Set {
            c: self.consume_one_byte_as_scalar() as i32,
        }
    }

    fn handle_set2(&mut self) -> OpCode {
        OpCode::Set {
            c: self.consume_two_bytes_as_scalar() as i32,
        }
    }

    fn handle_set3(&mut self) -> OpCode {
        OpCode::Set {
            c: self.consume_three_bytes_as_scalar() as i32,
        }
    }

    fn handle_set4(&mut self) -> OpCode {
        OpCode::Set {
            c: self.consume_four_bytes_as_scalar(),
        }
    }

    fn handle_set_rule(&mut self) -> OpCode {
        OpCode::SetRule {
            a: self.consume_four_bytes_as_scalar(),
            b: self.consume_four_bytes_as_scalar(),
        }
    }

    // Put

    fn handle_put1(&mut self) -> OpCode {
        OpCode::Put {
            c: self.consume_one_byte_as_scalar() as i32,
        }
    }

    fn handle_put2(&mut self) -> OpCode {
        OpCode::Put {
            c: self.consume_two_bytes_as_scalar() as i32,
        }
    }

    fn handle_put3(&mut self) -> OpCode {
        OpCode::Put {
            c: self.consume_three_bytes_as_scalar() as i32,
        }
    }

    fn handle_put4(&mut self) -> OpCode {
        OpCode::Put {
            c: self.consume_four_bytes_as_scalar() as i32,
        }
    }

    fn handle_put_rule(&mut self) -> OpCode {
        OpCode::PutRule {
            a: self.consume_four_bytes_as_scalar(),
            b: self.consume_four_bytes_as_scalar(),
        }
    }

    fn handle_nop(&mut self) -> OpCode {
        OpCode::Nop
    }

    fn handle_bop(&mut self) -> OpCode {
        OpCode::Bop {
            c0: self.consume_four_bytes_as_scalar(),
            c1: self.consume_four_bytes_as_scalar(),
            c2: self.consume_four_bytes_as_scalar(),
            c3: self.consume_four_bytes_as_scalar(),
            c4: self.consume_four_bytes_as_scalar(),
            c5: self.consume_four_bytes_as_scalar(),
            c6: self.consume_four_bytes_as_scalar(),
            c7: self.consume_four_bytes_as_scalar(),
            c8: self.consume_four_bytes_as_scalar(),
            c9: self.consume_four_bytes_as_scalar(),
            p: self.consume_four_bytes_as_scalar(),
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
        OpCode::Right {
            b: self.consume_one_byte_as_scalar() as i32,
        }
    }

    fn handle_right2(&mut self) -> OpCode {
        OpCode::Right {
            b: self.consume_two_bytes_as_scalar() as i32,
        }
    }

    fn handle_right3(&mut self) -> OpCode {
        OpCode::Right {
            b: self.consume_three_bytes_as_scalar() as i32,
        }
    }

    fn handle_right4(&mut self) -> OpCode {
        OpCode::Right {
            b: self.consume_four_bytes_as_scalar(),
        }
    }

    // W

    fn handle_w0(&mut self) -> OpCode {
        OpCode::W0
    }    

    fn handle_w1(&mut self) -> OpCode {
        OpCode::W {
            b: self.consume_one_byte_as_scalar() as i32,
        }
    }

    fn handle_w2(&mut self) -> OpCode {
        OpCode::W {
            b: self.consume_two_bytes_as_scalar() as i32,
        }
    }

    fn handle_w3(&mut self) -> OpCode {
        OpCode::W {
            b: self.consume_three_bytes_as_scalar() as i32,
        }
    }

    fn handle_w4(&mut self) -> OpCode {
        OpCode::W {
            b: self.consume_four_bytes_as_scalar(),
        }
    }        

    // X

    fn handle_x0(&mut self) -> OpCode {
        OpCode::X0
    }    

    fn handle_x1(&mut self) -> OpCode {
        OpCode::X {
            b: self.consume_one_byte_as_scalar() as i32,
        }
    }

    fn handle_x2(&mut self) -> OpCode {
        OpCode::X {
            b: self.consume_two_bytes_as_scalar() as i32,
        }
    }

    fn handle_x3(&mut self) -> OpCode {
        OpCode::X {
            b: self.consume_three_bytes_as_scalar() as i32,
        }
    }

    fn handle_x4(&mut self) -> OpCode {
        OpCode::X {
            b: self.consume_four_bytes_as_scalar(),
        }
    }

    // Down

    fn handle_down1(&mut self) -> OpCode {
        OpCode::Down {
            a: self.consume_one_byte_as_scalar() as i32,
        }
    }

    fn handle_down2(&mut self) -> OpCode {
        OpCode::Down {
            a: self.consume_two_bytes_as_scalar() as i32,
        }
    }

    fn handle_down3(&mut self) -> OpCode {
        OpCode::Down {
            a: self.consume_three_bytes_as_scalar() as i32,
        }
    }

    fn handle_down4(&mut self) -> OpCode {
        OpCode::Down {
            a: self.consume_four_bytes_as_scalar(),
        }
    }

    // Y

    fn handle_y0(&mut self) -> OpCode {
        OpCode::Y0
    }    

    fn handle_y1(&mut self) -> OpCode {
        OpCode::Y {
            a: self.consume_one_byte_as_scalar() as i32,
        }
    }

    fn handle_y2(&mut self) -> OpCode {
        OpCode::Y {
            a: self.consume_two_bytes_as_scalar() as i32,
        }
    }

    fn handle_y3(&mut self) -> OpCode {
        OpCode::Y {
            a: self.consume_three_bytes_as_scalar() as i32,
        }
    }

    fn handle_y4(&mut self) -> OpCode {
        OpCode::Y {
            a: self.consume_four_bytes_as_scalar(),
        }
    }

    // Y

    fn handle_z0(&mut self) -> OpCode {
        OpCode::Z0
    }

    fn handle_z1(&mut self) -> OpCode {
        OpCode::Z {
            a: self.consume_one_byte_as_scalar() as i32,
        }
    }

    fn handle_z2(&mut self) -> OpCode {
        OpCode::Z {
            a: self.consume_two_bytes_as_scalar() as i32,
        }
    }

    fn handle_z3(&mut self) -> OpCode {
        OpCode::Z {
            a: self.consume_three_bytes_as_scalar() as i32,
        }
    }

    fn handle_z4(&mut self) -> OpCode {
        OpCode::Z {
            a: self.consume_four_bytes_as_scalar(),
        }
    }

    // Fonts

    fn handle_fnt_num(&mut self, byte: u8) -> OpCode {
        OpCode::FntNum { k: byte as u32}
    }

    fn handle_fnt1(&mut self) -> OpCode {
        OpCode::Fnt {
            k: self.consume_one_byte_as_scalar() as i32,
        }
    }

    fn handle_fnt2(&mut self) -> OpCode {
        OpCode::Fnt {
            k: self.consume_two_bytes_as_scalar() as i32,
        }
    }

    fn handle_fnt3(&mut self) -> OpCode {
        OpCode::Fnt {
            k: self.consume_three_bytes_as_scalar() as i32,
        }
    }

    fn handle_fnt4(&mut self) -> OpCode {
        OpCode::Fnt {
            k: self.consume_four_bytes_as_scalar(),
        }
    }

    // Xxx

    fn handle_xxx1(&mut self) -> OpCode {
        let k = self.consume_one_byte_as_scalar() as i32;
        OpCode::Xxx {
            k: k,
            x: self.consume_n_bytes_as_vec(k as u32)
        }
    }

    fn handle_xxx2(&mut self) -> OpCode {
        let k = self.consume_two_bytes_as_scalar() as i32;
        OpCode::Xxx {
            k: k,
            x: self.consume_n_bytes_as_vec(k as u32)
        }
    }

    fn handle_xxx3(&mut self) -> OpCode {
        let k = self.consume_three_bytes_as_scalar() as i32;
        OpCode::Xxx {
            k: k,
            x: self.consume_n_bytes_as_vec(k as u32)
        }
    }        

    fn handle_xxx4(&mut self) -> OpCode {
        let k = self.consume_four_bytes_as_scalar()  as i32;
        OpCode::Xxx {
            k: k,
            x: self.consume_n_bytes_as_vec(k as u32)
        }
    }
    
    // fnt_def

    fn handle_fnt_def1(&mut self) -> OpCode {
        let k = self.consume_one_byte_as_scalar();
        self.handle_fnt_def(k as i32)
    }

    fn handle_fnt_def2(&mut self) -> OpCode {
        let k = self.consume_two_bytes_as_scalar();
        self.handle_fnt_def(k as i32)
    }

    fn handle_fnt_def3(&mut self) -> OpCode {
        let k = self.consume_three_bytes_as_scalar();
        self.handle_fnt_def(k as i32)
    }

    fn handle_fnt_def4(&mut self) -> OpCode {
        let k = self.consume_four_bytes_as_scalar();
        self.handle_fnt_def(k)
    }

    fn handle_fnt_def(&mut self, k: i32) -> OpCode {
        let a = self.consume_one_byte_as_scalar();
        let l = self.consume_one_byte_as_scalar();
    
        OpCode::FntDef {
            k: k,
            c: self.consume_four_bytes_as_scalar(), 
            s: self.consume_four_bytes_as_scalar(), 
            d: self.consume_four_bytes_as_scalar(), 
            a: a, 
            l: l, 
            n: self.consume_n_bytes_as_vec((a + l) as u32),
        }
    }

    // Read bytes

    fn consume_one_byte_as_scalar(&mut self) -> u8 {
        self.consume_n_bytes_as_scalar(1) as u8
    }

    fn consume_two_bytes_as_scalar(&mut self) -> u32 {
        self.consume_n_bytes_as_scalar(2)
    }

    fn consume_three_bytes_as_scalar(&mut self) -> u32 {
        self.consume_n_bytes_as_scalar(3)
    }

    fn consume_four_bytes_as_scalar(&mut self) -> i32 {
        self.consume_n_bytes_as_scalar(4) as i32
    }

    fn consume_n_bytes_as_scalar(&mut self, n: u32) -> u32 {
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

    fn consume_n_bytes_as_vec(&mut self, k: u32) -> Vec<u8> {
        let mut result = Vec::with_capacity(k as usize);

        for _ in 0..k {
            result.push(self.consume_one_byte_as_scalar() as u8);
        }

        assert_eq!(result.len(), k as usize);

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

        assert_that_opcode_was_generated(result, OpCode::Set { c: 0xAB })
    }

    #[test]
    fn test_disassemble_set2() {
        let result = disassemble(vec![129, 0xAB, 0xCD]);

        assert_that_opcode_was_generated(result, OpCode::Set { c: 0xABCD })
    }

    #[test]
    fn test_disassemble_set3() {
        let result = disassemble(vec![130, 0xAB, 0xCD, 0xEF]);

        assert_that_opcode_was_generated(result, OpCode::Set { c: 0xABCDEF })
    }

    #[test]
    fn test_disassemble_set4() {
        let result = disassemble(vec![131, 0x00, 0x11, 0x22, 0x33]);

        assert_that_opcode_was_generated(result, OpCode::Set { c: 0x112233 })
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

        assert_that_opcode_was_generated(result, OpCode::Put { c: 0xAB })
    }

    #[test]
    fn test_disassemble_put2() {
        let result = disassemble(vec![134, 0xAB, 0xCD]);

        assert_that_opcode_was_generated(result, OpCode::Put { c: 0xABCD })
    }

    #[test]
    fn test_disassemble_put3() {
        let result = disassemble(vec![135, 0xAB, 0xCD, 0xEF]);

        assert_that_opcode_was_generated(result, OpCode::Put { c: 0xABCDEF })
    }

    #[test]
    fn test_disassemble_put4() {
        let result = disassemble(vec![136, 0x00, 0x11, 0x22, 0x33]);

        assert_that_opcode_was_generated(result, OpCode::Put { c: 0x112233 })
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

        assert_that_opcode_was_generated(result, OpCode::Right { b: 0xAB })
    }

    #[test]
    fn test_disassemble_right2() {
        let result = disassemble(vec![144, 0xAB, 0xCD]);

        assert_that_opcode_was_generated(result, OpCode::Right { b: 0xABCD })
    }

    #[test]
    fn test_disassemble_right3() {
        let result = disassemble(vec![145, 0xAB, 0xCD, 0xEF]);

        assert_that_opcode_was_generated(result, OpCode::Right { b: 0xABCDEF })
    }

    #[test]
    fn test_disassemble_right4() {
        let result = disassemble(vec![146, 0x00, 0x11, 0x22, 0x33]);

        assert_that_opcode_was_generated(result, OpCode::Right { b: 0x112233 })
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

        assert_that_opcode_was_generated(result, OpCode::W { b: 0xAB })
    }

    #[test]
    fn test_disassemble_w2() {
        let result = disassemble(vec![149, 0xAB, 0xCD]);

        assert_that_opcode_was_generated(result, OpCode::W { b: 0xABCD })
    }

    #[test]
    fn test_disassemble_w3() {
        let result = disassemble(vec![150, 0xAB, 0xCD, 0xEF]);

        assert_that_opcode_was_generated(result, OpCode::W { b: 0xABCDEF })
    }

    #[test]
    fn test_disassemble_w4() {
        let result = disassemble(vec![151, 0x00, 0x11, 0x22, 0x33]);

        assert_that_opcode_was_generated(result, OpCode::W { b: 0x112233 })
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

        assert_that_opcode_was_generated(result, OpCode::X { b: 0xAB })
    }

    #[test]
    fn test_disassemble_x2() {
        let result = disassemble(vec![154, 0xAB, 0xCD]);

        assert_that_opcode_was_generated(result, OpCode::X { b: 0xABCD })
    }

    #[test]
    fn test_disassemble_x3() {
        let result = disassemble(vec![155, 0xAB, 0xCD, 0xEF]);

        assert_that_opcode_was_generated(result, OpCode::X { b: 0xABCDEF })
    }

    #[test]
    fn test_disassemble_x4() {
        let result = disassemble(vec![156, 0x00, 0x11, 0x22, 0x33]);

        assert_that_opcode_was_generated(result, OpCode::X { b: 0x112233 })
    }   

    // Down

   #[test]
    fn test_disassemble_down1() {
        let result = disassemble(vec![157, 0xAB]);

        assert_that_opcode_was_generated(result, OpCode::Down { a: 0xAB })
    }

    #[test]
    fn test_disassemble_down2() {
        let result = disassemble(vec![158, 0xAB, 0xCD]);

        assert_that_opcode_was_generated(result, OpCode::Down { a: 0xABCD })
    }

    #[test]
    fn test_disassemble_down3() {
        let result = disassemble(vec![159, 0xAB, 0xCD, 0xEF]);

        assert_that_opcode_was_generated(result, OpCode::Down { a: 0xABCDEF })
    }

    #[test]
    fn test_disassemble_down4() {
        let result = disassemble(vec![160, 0x00, 0x11, 0x22, 0x33]);

        assert_that_opcode_was_generated(result, OpCode::Down { a: 0x112233 })
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

        assert_that_opcode_was_generated(result, OpCode::Y { a: 0xAB })
    }

    #[test]
    fn test_disassemble_y2() {
        let result = disassemble(vec![163, 0xAB, 0xCD]);

        assert_that_opcode_was_generated(result, OpCode::Y { a: 0xABCD })
    }

    #[test]
    fn test_disassemble_y3() {
        let result = disassemble(vec![164, 0xAB, 0xCD, 0xEF]);

        assert_that_opcode_was_generated(result, OpCode::Y { a: 0xABCDEF })
    }

    #[test]
    fn test_disassemble_y4() {
        let result = disassemble(vec![165, 0x00, 0x11, 0x22, 0x33]);

        assert_that_opcode_was_generated(result, OpCode::Y { a: 0x112233 })
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

        assert_that_opcode_was_generated(result, OpCode::Z { a: 0xAB })
    }

    #[test]
    fn test_disassemble_z2() {
        let result = disassemble(vec![168, 0xAB, 0xCD]);

        assert_that_opcode_was_generated(result, OpCode::Z { a: 0xABCD })
    }

    #[test]
    fn test_disassemble_z3() {
        let result = disassemble(vec![169, 0xAB, 0xCD, 0xEF]);

        assert_that_opcode_was_generated(result, OpCode::Z { a: 0xABCDEF })
    }

    #[test]
    fn test_disassemble_z4() {
        let result = disassemble(vec![170, 0x00, 0x11, 0x22, 0x33]);

        assert_that_opcode_was_generated(result, OpCode::Z { a: 0x112233 })
    }

    // Font

    #[test]
    fn test_disassemble_fnt_num() {
        for i in 172..234 + 1 {
            let result = disassemble(vec![i]);

            assert_that_opcode_was_generated(result, OpCode::FntNum { k: i as u32 })
        }
    }

    #[test]
    fn test_disassemble_fnt1() {
        let result = disassemble(vec![235, 0xAB]);

        assert_that_opcode_was_generated(result, OpCode::Fnt {k: 0xAB })
    }

    #[test]
    fn test_disassemble_fnt2() {
        let result = disassemble(vec![236, 0xAB, 0xCD]);

        assert_that_opcode_was_generated(result, OpCode::Fnt {k: 0xABCD })
    }

    #[test]
    fn test_disassemble_fnt3() {
        let result = disassemble(vec![237, 0xAB, 0xCD, 0xEF]);

        assert_that_opcode_was_generated(result, OpCode::Fnt {k: 0xABCDEF })
    }

    #[test]
    fn test_disassemble_fnt4() {
        let result = disassemble(vec![238, 0x11, 0x22, 0x33, 0x44]);

        assert_that_opcode_was_generated(result, OpCode::Fnt {k: 0x11223344 })
    }

    // Xxx

    #[test]
    fn test_disassemble_xxx1() {
        let result = disassemble(vec![239, 0x5, 0x1, 0x2, 0x3, 0x4, 0x5]);

        assert_that_opcode_was_generated(
            result, 
            OpCode::Xxx {
                k: 0x5,
                x: vec![1, 2, 3, 4, 5] 
            }
        )
    }

    #[test]
    fn test_disassemble_xxx2() {
        let result = disassemble(vec![240, 0x0, 0x5, 0x1, 0x2, 0x3, 0x4, 0x5]);

        assert_that_opcode_was_generated(
            result, 
            OpCode::Xxx {
                k: 0x5,
                x: vec![1, 2, 3, 4, 5] 
            }
        )
    }

    #[test]
    fn test_disassemble_xxx3() {
        let result = disassemble(vec![241, 0x0, 0x0, 0x5, 0x1, 0x2, 0x3, 0x4, 0x5]);

        assert_that_opcode_was_generated(
            result, 
            OpCode::Xxx {
                k: 0x5,
                x: vec![1, 2, 3, 4, 5] 
            }
        )
    }

    #[test]
    fn test_disassemble_xxx4() {
        let result = disassemble(vec![242, 0x0, 0x0, 0x0, 0x5, 0x1, 0x2, 0x3, 0x4, 0x5]);

        assert_that_opcode_was_generated(
            result, 
            OpCode::Xxx {
                k: 0x5,
                x: vec![1, 2, 3, 4, 5] 
            }
        )
    }

    // Helper

    fn assert_that_opcode_was_generated(result: Result<Vec<OpCode>, String>, opcode: OpCode) {
        let opcodes = result.unwrap();
        assert_eq!(opcodes.len(), 1);
        assert_eq!(opcodes[0], opcode);
    }
}
