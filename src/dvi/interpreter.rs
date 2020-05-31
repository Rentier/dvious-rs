use crate::dvi::opcodes::OpCode;
use crate::errors::DviousResult;
use crate::fonts::tfm::*;


/// The units used internally by the interpreter are TeX scaled points (sp).
pub struct Interpreter {
    registers: RegisterFrame,
    f: Option<u32>,
    stack: Vec<RegisterFrame>,
    fonts: Vec<FontInformation>,
}

#[derive(Default, Debug)]
struct RegisterFrame {
    h: i64,
    v: i64,
    w: i64,
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug)]
struct FontInformation {
    s: i32,
    d: i32,
    tfm: TexFontMetric,
}

impl Interpreter {
    fn new() -> Interpreter {
        Interpreter {
            registers: Default::default(),
            f: Option::None,
            stack: Vec::new(),
            fonts: Vec::new(),
        }
    }

    pub fn execute(&mut self, instructions: Vec<OpCode>) -> DviousResult<()> {
        for instruction in instructions {
            match instruction {
                OpCode::Set { c } => self.handle_set(c),
                _ => unimplemented!(),
            }
        }
        Ok(())
    }

    fn handle_set(&mut self, _c: i32) {
        // self.registers.h += self.get_character_width(c);
    }

    // fn get_char_info(&self, c: i32) {
    //     let tfm = &self.fonts[self.f as usize];
    //     let char_info = &tfm.char_info_table;
    // }

    // fn get_character_width(&self, c: i32) -> Fixword {
    //     let char_info = self.get_char_info(c);
    //     0.
    // }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::dvi::interpreter::*;
    use crate::dvi::opcodes::*;
    

    #[test]
    fn test_execute_set() {
        let instructions = vec![OpCode::Set { c: 0x42 }];
        let mut interpreter = interpreter_fixture();

        interpreter.execute(instructions).unwrap();

        // assert_eq!(interpreter.registers.h, 1);
    }

    fn interpreter_fixture() -> Interpreter {
        // Create font

        let font_header = TfmMetricHeader {
            checksum: 0xDEAD,
            design_size: 10.0,
            encoding: Option::None,
            font_identifier: Option::None,
            face: Option::None,
            misc: Vec::new(),
        };

        let char_info = TfmCharInfo {
            character: 0x42,
            width_index: 1,
            height_index: 1,
            depth_index: 1,
            italic_index: 1,
            tag: TfmCharInfoTag::None,
        };

        let mut char_infos = HashMap::new();
        char_infos.insert(0x42, char_info);

        let tfm = TexFontMetric {
            header: font_header,
            char_info_table: char_infos,
            width_table: vec![0.0, 1.0],
            heigth_table: vec![0.0, 2.0],
            depth_table: vec![0.0, 3.0],
            italic_table: vec![0.0, 4.0],
            lig_kern_table: vec![],
            kern_table: vec![0.0, 5.0],
            extension_table: vec![],
            param_table: vec![0.0, 6.0],
        };

        let font_information = FontInformation {
            tfm: tfm,
            s: 655360,
            d: 655360,
        };
        // Build interpreter


        let mut interpreter = Interpreter::new();
        interpreter.fonts.push(font_information);
        interpreter
    }

}
