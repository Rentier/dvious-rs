use dvi::opcodes::OpCode;

pub struct Disassembler {
    bytes: Vec<u8>,
    opcodes: Vec<OpCode>,
    position: usize,
}

impl Disassembler {
    pub fn new(bytes: Vec<u8>) -> Disassembler {
        Disassembler {
            bytes: bytes,
            opcodes: Vec::new(),
            position: 0,
        }
    }

    pub fn disassemble<'a>(&'a mut self) -> &'a [OpCode] {
        while self.has_more() {
            let opcode = self.disassemble_next();
            self.opcodes.push(opcode);
        }

        &self.opcodes
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
    use Disassembler;
    use dvi::opcodes::OpCode;

    #[test]
    fn test_disassemble_set_char() {
        for i in 0..127 {
            let bytes = vec![i];
            let mut disassembler = Disassembler::new(bytes);
            let result = disassembler.disassemble();

            assert_eq!(result.len(), 1);
            assert_eq!(result[0], OpCode::SetN(i));
        }
    }
}
