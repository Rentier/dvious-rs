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
        let position = 0;
        let mut opcodes = Vec::new();

        while self.has_more() {
            let opcode = self.disassemble_next();
            opcodes.push(opcode);
        }

        opcodes
    }

    fn disassemble_next(&mut self) -> OpCode {
        let byte = self.consume();
        match byte {
            0...127 => OpCode::SetN(byte),
            _ => panic!("Unknown opcode"),
        }
    }

    fn consume(&mut self) -> u8 {
        self.position += 1;
        self.bytes[self.position - 1]
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
            assert_eq!(result[0], OpCode::SetN(i));
        }
    }
}
