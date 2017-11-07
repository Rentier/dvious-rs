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
            _ => panic!("Unknown opcode: {}", byte),
        }
    }

    fn handle_set1(&mut self) -> OpCode {
        let payload = self.consume_n_bytes(1);
        OpCode::SetN { n: payload }
    }

    fn handle_set2(&mut self) -> OpCode {
        let payload = self.consume_n_bytes(2);
        OpCode::SetN { n: payload }
    }

    fn handle_set3(&mut self) -> OpCode {
        let payload = self.consume_n_bytes(3);
        OpCode::SetN { n: payload }
    }

    fn handle_set4(&mut self) -> OpCode {
        let payload = self.consume_n_bytes(4);
        OpCode::SetN { n: payload }
    }

    fn consume_byte(&mut self) -> u64 {
        self.consume_n_bytes(1)
    }

    fn consume_n_bytes(&mut self, n: u64) -> u64 {
        debug_assert!(n <= 4, "Can at most read u64 with n == 4");

        let mut result: u64 = 0;
        for i in (0..n).rev() {
            // Bytes are in big endian
            let byte = self.bytes[self.position] as u64;
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

            assert_eq!(result.len(), 1);
            assert_eq!(result[0], OpCode::SetN { n: i as u64 });
        }
    }

    #[test]
    fn test_disassemble_set1() {
        let result = disassemble(vec![128, 0xAB]);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], OpCode::SetN { n: 0xAB });
    }

    #[test]
    fn test_disassemble_set2() {
        let result = disassemble(vec![129, 0xAB, 0xCD]);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], OpCode::SetN { n: 0xABCD });
    }

    #[test]
    fn test_disassemble_set3() {
        let result = disassemble(vec![130, 0xAB, 0xCD, 0xEF]);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], OpCode::SetN { n: 0xABCDEF });
    }

    #[test]
    fn test_disassemble_set4() {
        let result = disassemble(vec![131, 0xDE, 0xAD, 0xBE, 0xEF]);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], OpCode::SetN { n: 0xDEADBEEF });
    }
}
