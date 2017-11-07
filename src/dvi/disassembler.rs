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
        let byte = self.consume_byte();
        match byte {
            0...127 => OpCode::SetN { n: byte },
            128 => self.handle_set1(),
            129 => self.handle_set2(),
            130 => self.handle_set3(),
            131 => self.handle_set4(),
            132 => self.handle_set_rule(),
            _ => panic!("Unknown opcode: {}", byte),
        }
    }

    fn handle_set1(&mut self) -> OpCode {
        OpCode::SetN {
            n: self.consume_n_bytes(1),
        }
    }

    fn handle_set2(&mut self) -> OpCode {
        OpCode::SetN {
            n: self.consume_n_bytes(2),
        }
    }

    fn handle_set3(&mut self) -> OpCode {
        OpCode::SetN {
            n: self.consume_n_bytes(3),
        }
    }

    fn handle_set4(&mut self) -> OpCode {
        OpCode::SetN {
            n: self.consume_n_bytes(4),
        }
    }

    fn handle_set_rule(&mut self) -> OpCode {
        OpCode::SetRule {
            a: self.consume_n_bytes(4) as i32,
            b: self.consume_n_bytes(4) as i32,
        }
    }

    fn consume_byte(&mut self) -> u32 {
        self.consume_n_bytes(1)
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

            assert_that_opcode_was_generated(result, OpCode::SetN { n: i as u32 })
        }
    }

    #[test]
    fn test_disassemble_set1() {
        let result = disassemble(vec![128, 0xAB]);

        assert_that_opcode_was_generated(result, OpCode::SetN { n: 0xAB })
    }

    #[test]
    fn test_disassemble_set2() {
        let result = disassemble(vec![129, 0xAB, 0xCD]);

        assert_that_opcode_was_generated(result, OpCode::SetN { n: 0xABCD })
    }

    #[test]
    fn test_disassemble_set3() {
        let result = disassemble(vec![130, 0xAB, 0xCD, 0xEF]);

        assert_that_opcode_was_generated(result, OpCode::SetN { n: 0xABCDEF })
    }

    #[test]
    fn test_disassemble_set4() {
        let result = disassemble(vec![131, 0xDE, 0xAD, 0xBE, 0xEF]);

        assert_that_opcode_was_generated(result, OpCode::SetN { n: 0xDEADBEEF })
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

    fn assert_that_opcode_was_generated(result: Vec<OpCode>, opcode: OpCode) {
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], opcode);
    }
}
