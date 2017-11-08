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
        let byte = self.consume_one_byte();
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
            _ => panic!("Unknown opcode: {}", byte),
        }
    }

    fn handle_set_char(&mut self, byte: u32) -> OpCode {
        OpCode::SetChar { n: byte }
    }

    fn handle_set1(&mut self) -> OpCode {
        OpCode::Set1 {
            n: self.consume_one_byte(),
        }
    }

    fn handle_set2(&mut self) -> OpCode {
        OpCode::Set2 {
            n: self.consume_two_bytes(),
        }
    }

    fn handle_set3(&mut self) -> OpCode {
        OpCode::Set3 {
            n: self.consume_three_bytes(),
        }
    }

    fn handle_set4(&mut self) -> OpCode {
        OpCode::Set4 {
            n: self.consume_four_bytes(),
        }
    }

    fn handle_set_rule(&mut self) -> OpCode {
        OpCode::SetRule {
            a: self.consume_four_bytes(),
            b: self.consume_four_bytes(),
        }
    }

    fn handle_put1(&mut self) -> OpCode {
        OpCode::Put1 {
            n: self.consume_one_byte(),
        }
    }

    fn handle_put2(&mut self) -> OpCode {
        OpCode::Put2 {
            n: self.consume_two_bytes(),
        }
    }

    fn handle_put3(&mut self) -> OpCode {
        OpCode::Put3 {
            n: self.consume_three_bytes(),
        }
    }

    fn handle_put4(&mut self) -> OpCode {
        OpCode::Put4 {
            n: self.consume_four_bytes(),
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
        for i in 0..127 {
            let result = disassemble(vec![i]);

            assert_that_opcode_was_generated(result, OpCode::SetChar { n: i as u32 })
        }
    }

    #[test]
    fn test_disassemble_set1() {
        let result = disassemble(vec![128, 0xAB]);

        assert_that_opcode_was_generated(result, OpCode::Set1 { n: 0xAB })
    }

    #[test]
    fn test_disassemble_set2() {
        let result = disassemble(vec![129, 0xAB, 0xCD]);

        assert_that_opcode_was_generated(result, OpCode::Set2 { n: 0xABCD })
    }

    #[test]
    fn test_disassemble_set3() {
        let result = disassemble(vec![130, 0xAB, 0xCD, 0xEF]);

        assert_that_opcode_was_generated(result, OpCode::Set3 { n: 0xABCDEF })
    }

    #[test]
    fn test_disassemble_set4() {
        let result = disassemble(vec![131, 0x00, 0x11, 0x22, 0x33]);

        assert_that_opcode_was_generated(result, OpCode::Set4 { n: 0x112233 })
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

        assert_that_opcode_was_generated(result, OpCode::Put1 { n: 0xAB })
    }

    #[test]
    fn test_disassemble_put2() {
        let result = disassemble(vec![134, 0xAB, 0xCD]);

        assert_that_opcode_was_generated(result, OpCode::Put2 { n: 0xABCD })
    }

    #[test]
    fn test_disassemble_put3() {
        let result = disassemble(vec![135, 0xAB, 0xCD, 0xEF]);

        assert_that_opcode_was_generated(result, OpCode::Put3 { n: 0xABCDEF })
    }

    #[test]
    fn test_disassemble_put4() {
        let result = disassemble(vec![136, 0x00, 0x11, 0x22, 0x33]);

        assert_that_opcode_was_generated(result, OpCode::Put4 { n: 0x112233 })
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

    fn assert_that_opcode_was_generated(result: Vec<OpCode>, opcode: OpCode) {
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], opcode);
    }
}
