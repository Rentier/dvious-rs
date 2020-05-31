use crate::dvi::opcodes::OpCode;
use crate::errors::{DviousError, DviousResult};
use crate::util::byte_reader::ByteReader;
use crate::util::num::{i24, u24};

pub fn disassemble(bytes: Vec<u8>) -> DviousResult<Vec<OpCode>> {
    let mut disassembler = Disassembler::new(bytes);
    disassembler.disassemble()
}

struct Disassembler {
    reader: ByteReader,
    last_bop: Option<usize>,
    last_post: Option<usize>,
    number_of_instructions: usize,
}

impl Disassembler {
    fn new(bytes: Vec<u8>) -> Disassembler {
        Disassembler {
            reader: ByteReader::new(bytes),
            last_bop: Option::None,
            last_post: Option::None,
            number_of_instructions: 0,
        }
    }

    fn disassemble(&mut self) -> DviousResult<Vec<OpCode>> {
        let mut opcodes = Vec::new();
        self.last_bop = Option::None;

        while self.reader.has_more() {
            let opcode = self.disassemble_next()?;
            opcodes.push(opcode);
        }

        Ok(opcodes)
    }

    fn disassemble_next(&mut self) -> DviousResult<OpCode> {
        let byte = self.reader.read_be::<u8>()?;
        let opcode = match byte {
            0..=127 => self.handle_set_char(i32::from(byte))?,
            128 => self.handle_set1()?,
            129 => self.handle_set2()?,
            130 => self.handle_set3()?,
            131 => self.handle_set4()?,
            132 => self.handle_set_rule()?,
            133 => self.handle_put1()?,
            134 => self.handle_put2()?,
            135 => self.handle_put3()?,
            136 => self.handle_put4()?,
            137 => self.handle_put_rule()?,
            138 => self.handle_nop()?,
            139 => self.handle_bop()?,
            140 => self.handle_eop()?,
            141 => self.handle_push()?,
            142 => self.handle_pop()?,
            143 => self.handle_right1()?,
            144 => self.handle_right2()?,
            145 => self.handle_right3()?,
            146 => self.handle_right4()?,
            147 => self.handle_w0()?,
            148 => self.handle_w1()?,
            149 => self.handle_w2()?,
            150 => self.handle_w3()?,
            151 => self.handle_w4()?,
            152 => self.handle_x0()?,
            153 => self.handle_x1()?,
            154 => self.handle_x2()?,
            155 => self.handle_x3()?,
            156 => self.handle_x4()?,
            157 => self.handle_down1()?,
            158 => self.handle_down2()?,
            159 => self.handle_down3()?,
            160 => self.handle_down4()?,
            161 => self.handle_y0()?,
            162 => self.handle_y1()?,
            163 => self.handle_y2()?,
            164 => self.handle_y3()?,
            165 => self.handle_y4()?,
            166 => self.handle_z0()?,
            167 => self.handle_z1()?,
            168 => self.handle_z2()?,
            169 => self.handle_z3()?,
            170 => self.handle_z4()?,
            171..=234 => self.handle_fnt_num(byte)?,
            235 => self.handle_fnt1()?,
            236 => self.handle_fnt2()?,
            237 => self.handle_fnt3()?,
            238 => self.handle_fnt4()?,
            239 => self.handle_xxx1()?,
            240 => self.handle_xxx2()?,
            241 => self.handle_xxx3()?,
            242 => self.handle_xxx4()?,
            243 => self.handle_fnt_def1()?,
            244 => self.handle_fnt_def2()?,
            245 => self.handle_fnt_def3()?,
            246 => self.handle_fnt_def4()?,
            247 => self.handle_pre()?,
            248 => self.handle_post()?,
            249 => self.handle_post_post()?,
            _ => return Err(DviousError::UnknownOpcodeError(byte)),
        };

        self.number_of_instructions += 1;
        Ok(opcode)
    }

    // Set

    fn handle_set_char<T: Into<i32>>(&mut self, byte: T) -> DviousResult<OpCode> {
        Ok(OpCode::Set { c: byte.into() })
    }

    fn handle_set1(&mut self) -> DviousResult<OpCode> {
        let c = self.reader.read_be::<u8>()?;
        self.handle_set_char(c)
    }

    fn handle_set2(&mut self) -> DviousResult<OpCode> {
        let c = self.reader.read_be::<u16>()?;
        self.handle_set_char(c)
    }

    fn handle_set3(&mut self) -> DviousResult<OpCode> {
        let c = self.reader.read_be::<u24>()?;
        self.handle_set_char(c)
    }

    fn handle_set4(&mut self) -> DviousResult<OpCode> {
        let c = self.reader.read_be::<i32>()?;
        self.handle_set_char(c)
    }

    fn handle_set_rule(&mut self) -> DviousResult<OpCode> {
        Ok(OpCode::SetRule {
            a: self.reader.read_be::<i32>()?,
            b: self.reader.read_be::<i32>()?,
        })
    }

    // Put

    fn handle_put<T: Into<i32>>(&mut self, c: T) -> DviousResult<OpCode> {
        Ok(OpCode::Put { c: c.into() })
    }

    fn handle_put1(&mut self) -> DviousResult<OpCode> {
        let c = self.reader.read_be::<u8>()?;
        self.handle_put(c)
    }

    fn handle_put2(&mut self) -> DviousResult<OpCode> {
        let c = self.reader.read_be::<u16>()?;
        self.handle_put(c)
    }

    fn handle_put3(&mut self) -> DviousResult<OpCode> {
        let c = self.reader.read_be::<u24>()?;
        self.handle_put(c)
    }

    fn handle_put4(&mut self) -> DviousResult<OpCode> {
        let c = self.reader.read_be::<i32>()?;
        self.handle_put(c)
    }

    fn handle_put_rule(&mut self) -> DviousResult<OpCode> {
        Ok(OpCode::PutRule {
            a: self.reader.read_be::<i32>()?,
            b: self.reader.read_be::<i32>()?,
        })
    }

    fn handle_nop(&mut self) -> DviousResult<OpCode> {
        Ok(OpCode::Nop)
    }

    fn handle_bop(&mut self) -> DviousResult<OpCode> {
        self.last_bop = Some(self.number_of_instructions);

        Ok(OpCode::Bop {
            c0: self.reader.read_be::<i32>()?,
            c1: self.reader.read_be::<i32>()?,
            c2: self.reader.read_be::<i32>()?,
            c3: self.reader.read_be::<i32>()?,
            c4: self.reader.read_be::<i32>()?,
            c5: self.reader.read_be::<i32>()?,
            c6: self.reader.read_be::<i32>()?,
            c7: self.reader.read_be::<i32>()?,
            c8: self.reader.read_be::<i32>()?,
            c9: self.reader.read_be::<i32>()?,
            p: self.reader.read_be::<i32>()?,
        })
    }

    fn handle_eop(&mut self) -> DviousResult<OpCode> {
        Ok(OpCode::Eop)
    }

    fn handle_push(&mut self) -> DviousResult<OpCode> {
        Ok(OpCode::Push)
    }

    fn handle_pop(&mut self) -> DviousResult<OpCode> {
        Ok(OpCode::Pop)
    }

    // Right

    fn handle_right<T: Into<i32>>(&mut self, b: T) -> DviousResult<OpCode> {
        Ok(OpCode::Right { b: b.into() })
    }

    fn handle_right1(&mut self) -> DviousResult<OpCode> {
        let b = self.reader.read_be::<i8>()?;
        self.handle_right(b)
    }

    fn handle_right2(&mut self) -> DviousResult<OpCode> {
        let b = self.reader.read_be::<i16>()?;
        self.handle_right(b)
    }

    fn handle_right3(&mut self) -> DviousResult<OpCode> {
        let b = self.reader.read_be::<i24>()?;
        self.handle_right(b)
    }

    fn handle_right4(&mut self) -> DviousResult<OpCode> {
        let b = self.reader.read_be::<i32>()?;
        self.handle_right(b)
    }

    // W

    fn handle_w<T: Into<i32>>(&mut self, b: T) -> DviousResult<OpCode> {
        Ok(OpCode::W { b: b.into() })
    }

    fn handle_w0(&mut self) -> DviousResult<OpCode> {
        Ok(OpCode::W0)
    }

    fn handle_w1(&mut self) -> DviousResult<OpCode> {
        let b = self.reader.read_be::<i8>()?;
        self.handle_w(b)
    }

    fn handle_w2(&mut self) -> DviousResult<OpCode> {
        let b = self.reader.read_be::<i16>()?;
        self.handle_w(b)
    }

    fn handle_w3(&mut self) -> DviousResult<OpCode> {
        let b = self.reader.read_be::<i24>()?;
        self.handle_w(b)
    }

    fn handle_w4(&mut self) -> DviousResult<OpCode> {
        let b = self.reader.read_be::<i32>()?;
        self.handle_w(b)
    }

    // X

    fn handle_x<T: Into<i32>>(&mut self, b: T) -> DviousResult<OpCode> {
        Ok(OpCode::X { b: b.into() })
    }

    fn handle_x0(&mut self) -> DviousResult<OpCode> {
        Ok(OpCode::X0)
    }

    fn handle_x1(&mut self) -> DviousResult<OpCode> {
        let b = self.reader.read_be::<i8>()?;
        self.handle_x(b)
    }

    fn handle_x2(&mut self) -> DviousResult<OpCode> {
        let b = self.reader.read_be::<i16>()?;
        self.handle_x(b)
    }

    fn handle_x3(&mut self) -> DviousResult<OpCode> {
        let b = self.reader.read_be::<i24>()?;
        self.handle_x(b)
    }

    fn handle_x4(&mut self) -> DviousResult<OpCode> {
        let b = self.reader.read_be::<i32>()?;
        self.handle_x(b)
    }

    // Down

    fn handle_down<T: Into<i32>>(&mut self, a: T) -> DviousResult<OpCode> {
        Ok(OpCode::Down { a: a.into() })
    }

    fn handle_down1(&mut self) -> DviousResult<OpCode> {
        let a = self.reader.read_be::<i8>()?;
        self.handle_down(a)
    }

    fn handle_down2(&mut self) -> DviousResult<OpCode> {
        let a = self.reader.read_be::<i16>()?;
        self.handle_down(a)
    }

    fn handle_down3(&mut self) -> DviousResult<OpCode> {
        let a = self.reader.read_be::<i24>()?;
        self.handle_down(a)
    }

    fn handle_down4(&mut self) -> DviousResult<OpCode> {
        let a = self.reader.read_be::<i32>()?;
        self.handle_down(a)
    }

    // Y

    fn handle_y<T: Into<i32>>(&mut self, a: T) -> DviousResult<OpCode> {
        Ok(OpCode::Y { a: a.into() })
    }

    fn handle_y0(&mut self) -> DviousResult<OpCode> {
        Ok(OpCode::Y0)
    }

    fn handle_y1(&mut self) -> DviousResult<OpCode> {
        let a = self.reader.read_be::<i8>()?;
        self.handle_y(a)
    }

    fn handle_y2(&mut self) -> DviousResult<OpCode> {
        let a = self.reader.read_be::<i16>()?;
        self.handle_y(a)
    }

    fn handle_y3(&mut self) -> DviousResult<OpCode> {
        let a = self.reader.read_be::<i24>()?;
        self.handle_y(a)
    }

    fn handle_y4(&mut self) -> DviousResult<OpCode> {
        let a = self.reader.read_be::<i32>()?;
        self.handle_y(a)
    }

    // Z

    fn handle_z<T: Into<i32>>(&mut self, a: T) -> DviousResult<OpCode> {
        Ok(OpCode::Z { a: a.into() })
    }

    fn handle_z0(&mut self) -> DviousResult<OpCode> {
        Ok(OpCode::Z0)
    }

    fn handle_z1(&mut self) -> DviousResult<OpCode> {
        let a = self.reader.read_be::<i8>()?;
        self.handle_z(a)
    }

    fn handle_z2(&mut self) -> DviousResult<OpCode> {
        let a = self.reader.read_be::<i16>()?;
        self.handle_z(a)
    }

    fn handle_z3(&mut self) -> DviousResult<OpCode> {
        let a = self.reader.read_be::<i24>()?;
        self.handle_z(a)
    }

    fn handle_z4(&mut self) -> DviousResult<OpCode> {
        let a = self.reader.read_be::<i32>()?;
        self.handle_z(a)
    }

    // Fonts

    fn handle_fnt<T: Into<i32>>(&mut self, k: T) -> DviousResult<OpCode> {
        Ok(OpCode::Fnt { k: k.into() })
    }

    fn handle_fnt_num(&mut self, byte: u8) -> DviousResult<OpCode> {
        self.handle_fnt(byte)
    }

    fn handle_fnt1(&mut self) -> DviousResult<OpCode> {
        let k = self.reader.read_be::<u8>()?;
        self.handle_fnt(k)
    }

    fn handle_fnt2(&mut self) -> DviousResult<OpCode> {
        let k = self.reader.read_be::<u16>()?;
        self.handle_fnt(k)
    }

    fn handle_fnt3(&mut self) -> DviousResult<OpCode> {
        let k = self.reader.read_be::<u24>()?;
        self.handle_fnt(k)
    }

    fn handle_fnt4(&mut self) -> DviousResult<OpCode> {
        let k = self.reader.read_be::<i32>()?;
        self.handle_fnt(k)
    }

    // Xxx

    fn handle_xxx<T: Into<u32>>(&mut self, n: T) -> DviousResult<OpCode> {
        let k = n.into();
        let x = self.reader.read_vector_be::<u8>(k as usize)?;
        Ok(OpCode::Xxx { k: k, x: x })
    }

    fn handle_xxx1(&mut self) -> DviousResult<OpCode> {
        let k = self.reader.read_be::<u8>()?;
        self.handle_xxx(k)
    }

    fn handle_xxx2(&mut self) -> DviousResult<OpCode> {
        let k = self.reader.read_be::<u16>()?;
        self.handle_xxx(k)
    }

    fn handle_xxx3(&mut self) -> DviousResult<OpCode> {
        let k = self.reader.read_be::<u24>()?;
        self.handle_xxx(k)
    }

    fn handle_xxx4(&mut self) -> DviousResult<OpCode> {
        let k = self.reader.read_be::<u32>()?;
        self.handle_xxx(k)
    }

    // fnt_def

    fn handle_fnt_def<T: Into<i32>>(&mut self, k: T) -> DviousResult<OpCode> {
        let c = self.reader.read_be::<u32>()?;
        let s = self.reader.read_be::<u32>()?;
        let d = self.reader.read_be::<u32>()?;
        let a = self.reader.read_be::<u8>()?;
        let l = self.reader.read_be::<u8>()?;
        let n = self.reader.read_vector_be::<u8>(usize::from(a + l))?;

        Ok(OpCode::FntDef {
            k: k.into(),
            c: c,
            s: s,
            d: d,
            a: a,
            l: l,
            n: n,
        })
    }

    fn handle_fnt_def1(&mut self) -> DviousResult<OpCode> {
        let k = self.reader.read_be::<u8>()?;
        self.handle_fnt_def(k)
    }

    fn handle_fnt_def2(&mut self) -> DviousResult<OpCode> {
        let k = self.reader.read_be::<u16>()?;
        self.handle_fnt_def(k)
    }

    fn handle_fnt_def3(&mut self) -> DviousResult<OpCode> {
        let k = self.reader.read_be::<u24>()?;
        self.handle_fnt_def(k)
    }

    fn handle_fnt_def4(&mut self) -> DviousResult<OpCode> {
        let k = self.reader.read_be::<i32>()?;
        self.handle_fnt_def(k)
    }

    // Pre and post

    fn handle_pre(&mut self) -> DviousResult<OpCode> {
        let i = self.reader.read_be::<u8>()?;
        let num = self.reader.read_be::<u32>()?;
        let den = self.reader.read_be::<u32>()?;
        let mag = self.reader.read_be::<u32>()?;
        let k = self.reader.read_be::<u8>()?;

        Ok(OpCode::Pre {
            i: i,
            num: num,
            den: den,
            mag: mag,
            k: k,
            x: self.reader.read_vector_be::<u8>(k as usize)?,
        })
    }

    fn handle_post(&mut self) -> DviousResult<OpCode> {
        self.reader.read_be::<i32>()?;
        self.last_post = Option::Some(self.number_of_instructions);

        Ok(OpCode::Post {
            p: self.last_bop,
            num: self.reader.read_be::<u32>()?,
            den: self.reader.read_be::<u32>()?,
            mag: self.reader.read_be::<u32>()?,
            l: self.reader.read_be::<u32>()?,
            u: self.reader.read_be::<u32>()?,
            s: self.reader.read_be::<u16>()?,
            t: self.reader.read_be::<u16>()?,
        })
    }

    fn handle_post_post(&mut self) -> DviousResult<OpCode> {
        // Consume unused pointer to post command
        self.reader.read_be::<i32>()?;

        let result = OpCode::PostPost {
            q: self.last_post,
            i: self.reader.read_be::<u8>()?,
        };

        // Consume padding
        while self.reader.has_more() && self.reader.peek_be::<u8>()? == 223 {
            self.reader.read_be::<u8>()?;
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::dvi::disassembler::disassemble;
    use crate::dvi::opcodes::OpCode;
    use crate::errors::DviousResult;

    #[test]
    fn test_disassemble_set_char() {
        for i in 0..127 + 1 {
            let result = disassemble(vec![i]);

            assert_that_opcode_was_generated(result, OpCode::Set { c: i as i32 })
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
        let result = disassemble(vec![143, 0x42]);

        assert_that_opcode_was_generated(result, OpCode::Right { b: 0x42 })
    }

    #[test]
    fn test_disassemble_right2() {
        let result = disassemble(vec![144, 0x0B, 0xCD]);

        assert_that_opcode_was_generated(result, OpCode::Right { b: 0x0BCD })
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
        let result = disassemble(vec![148, 0xD6]);

        assert_that_opcode_was_generated(result, OpCode::W { b: -42 })
    }

    #[test]
    fn test_disassemble_w2() {
        let result = disassemble(vec![149, 0xEF, 0x98]);

        assert_that_opcode_was_generated(result, OpCode::W { b: -4200 })
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
        let result = disassemble(vec![153, 0xD6]);

        assert_that_opcode_was_generated(result, OpCode::X { b: -42 })
    }

    #[test]
    fn test_disassemble_x2() {
        let result = disassemble(vec![154, 0xEF, 0x98]);

        assert_that_opcode_was_generated(result, OpCode::X { b: -4200 })
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
        let result = disassemble(vec![157, 0xD6]);

        assert_that_opcode_was_generated(result, OpCode::Down { a: -42 })
    }

    #[test]
    fn test_disassemble_down2() {
        let result = disassemble(vec![158, 0xEF, 0x98]);

        assert_that_opcode_was_generated(result, OpCode::Down { a: -4200 })
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
        let result = disassemble(vec![162, 0xD6]);

        assert_that_opcode_was_generated(result, OpCode::Y { a: -42 })
    }

    #[test]
    fn test_disassemble_y2() {
        let result = disassemble(vec![163, 0xEF, 0x98]);

        assert_that_opcode_was_generated(result, OpCode::Y { a: -4200 })
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
        let result = disassemble(vec![167, 0xD6]);

        assert_that_opcode_was_generated(result, OpCode::Z { a: -42 })
    }

    #[test]
    fn test_disassemble_z2() {
        let result = disassemble(vec![168, 0xEF, 0x98]);

        assert_that_opcode_was_generated(result, OpCode::Z { a: -4200 })
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

            assert_that_opcode_was_generated(result, OpCode::Fnt { k: i32::from(i) })
        }
    }

    #[test]
    fn test_disassemble_fnt1() {
        let result = disassemble(vec![235, 0xAB]);

        assert_that_opcode_was_generated(result, OpCode::Fnt { k: 0xAB })
    }

    #[test]
    fn test_disassemble_fnt2() {
        let result = disassemble(vec![236, 0xAB, 0xCD]);

        assert_that_opcode_was_generated(result, OpCode::Fnt { k: 0xABCD })
    }

    #[test]
    fn test_disassemble_fnt3() {
        let result = disassemble(vec![237, 0xAB, 0xCD, 0xEF]);

        assert_that_opcode_was_generated(result, OpCode::Fnt { k: 0xABCDEF })
    }

    #[test]
    fn test_disassemble_fnt4() {
        let result = disassemble(vec![238, 0x11, 0x22, 0x33, 0x44]);

        assert_that_opcode_was_generated(result, OpCode::Fnt { k: 0x11223344 })
    }

    // Xxx

    #[test]
    fn test_disassemble_xxx1() {
        let result = disassemble(vec![239, 0x5, 0x1, 0x2, 0x3, 0x4, 0x5]);

        assert_that_opcode_was_generated(
            result,
            OpCode::Xxx {
                k: 0x5,
                x: vec![1, 2, 3, 4, 5],
            },
        )
    }

    #[test]
    fn test_disassemble_xxx2() {
        let result = disassemble(vec![240, 0x0, 0x5, 0x1, 0x2, 0x3, 0x4, 0x5]);

        assert_that_opcode_was_generated(
            result,
            OpCode::Xxx {
                k: 0x5,
                x: vec![1, 2, 3, 4, 5],
            },
        )
    }

    #[test]
    fn test_disassemble_xxx3() {
        let result = disassemble(vec![241, 0x0, 0x0, 0x5, 0x1, 0x2, 0x3, 0x4, 0x5]);

        assert_that_opcode_was_generated(
            result,
            OpCode::Xxx {
                k: 0x5,
                x: vec![1, 2, 3, 4, 5],
            },
        )
    }

    #[test]
    fn test_disassemble_xxx4() {
        let result = disassemble(vec![242, 0x0, 0x0, 0x0, 0x5, 0x1, 0x2, 0x3, 0x4, 0x5]);

        assert_that_opcode_was_generated(
            result,
            OpCode::Xxx {
                k: 0x5,
                x: vec![1, 2, 3, 4, 5],
            },
        )
    }

    // fnt_def

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn test_disassemble_fnt_def1() {
        let result = disassemble(vec![
            243,
            0x42,
            0xDE, 0xAD, 0xBE, 0xEF,
            0xCA, 0xFE, 0xBA, 0xBE,
            0xBA, 0xAA, 0xAA, 0xAD,
            0x2,
            0x3,
            0x1, 0x2, 0x3, 0x4, 0x5
        ]);

        assert_that_opcode_was_generated(
            result,
            OpCode::FntDef {
                k: 0x42,
                c: 0xDEADBEEF,
                s: 0xCAFEBABE,
                d: 0xBAAAAAAD,
                a: 0x2,
                l: 0x3,
                n: vec![1, 2, 3, 4, 5]
            }
        )
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn test_disassemble_fnt_def2() {
        let result = disassemble(vec![
            244,
            0xAB, 0xCD,
            0xDE, 0xAD, 0xBE, 0xEF,
            0xCA, 0xFE, 0xBA, 0xBE,
            0xBA, 0xAA, 0xAA, 0xAD,
            0x2,
            0x3,
            0x1, 0x2, 0x3, 0x4, 0x5
        ]);

        assert_that_opcode_was_generated(
            result,
            OpCode::FntDef {
                k: 0xABCD,
                c: 0xDEADBEEF,
                s: 0xCAFEBABE,
                d: 0xBAAAAAAD,
                a: 0x2,
                l: 0x3,
                n: vec![1, 2, 3, 4, 5]
            }
        )
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn test_disassemble_fnt_def3() {
        let result = disassemble(vec![
            245,
            0xAB, 0xCD, 0xEF,
            0xDE, 0xAD, 0xBE, 0xEF,
            0xCA, 0xFE, 0xBA, 0xBE,
            0xBA, 0xAA, 0xAA, 0xAD,
            0x2,
            0x3,
            0x1, 0x2, 0x3, 0x4, 0x5
        ]);

        assert_that_opcode_was_generated(
            result,
            OpCode::FntDef {
                k: 0xABCDEF,
                c: 0xDEADBEEF,
                s: 0xCAFEBABE,
                d: 0xBAAAAAAD,
                a: 0x2,
                l: 0x3,
                n: vec![1, 2, 3, 4, 5]
            }
        )
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn test_disassemble_fnt_def4() {
        let result = disassemble(vec![
            246,
            0x0A, 0xBC, 0xDE, 0x12,
            0xDE, 0xAD, 0xBE, 0xEF,
            0xCA, 0xFE, 0xBA, 0xBE,
            0xBA, 0xAA, 0xAA, 0xAD,
            0x2,
            0x3,
            0x1, 0x2, 0x3, 0x4, 0x5
        ]);

        assert_that_opcode_was_generated(
            result,
            OpCode::FntDef {
                k: 0x0ABCDE12,
                c: 0xDEADBEEF,
                s: 0xCAFEBABE,
                d: 0xBAAAAAAD ,
                a: 0x2,
                l: 0x3,
                n: vec![1, 2, 3, 4, 5]
            }
        )
    }

    // Pre and post

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn test_disassemble_pre() {
        let result = disassemble(vec![
            247,
            0x42,
            0xDE, 0xAD, 0xBE, 0xEF,
            0xCA, 0xFE, 0xBA, 0xBE,
            0xBA, 0xAA, 0xAA, 0xAD,
            0x5,
            0x1, 0x2, 0x3, 0x4, 0x5
        ]);

        assert_that_opcode_was_generated(
            result,
            OpCode::Pre {
                i: 0x42,
                num: 0xDEADBEEF,
                den: 0xCAFEBABE,
                mag: 0xBAAAAAAD,
                k: 0x5,
                x: vec![1, 2, 3, 4, 5],
            }
        )
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn test_disassemble_post() {
        let result = disassemble(vec![
            248,
            0x00, 0x00, 0x00, 0x42,
            0xDE, 0xAD, 0xBE, 0xEF,
            0xCA, 0xFE, 0xBA, 0xBE,
            0xBA, 0xAA, 0xAA, 0xAD,
            0xDE, 0xAD, 0xC0, 0xDE,
            0xB1, 0x05, 0xF0, 0x0D,
            0xAB, 0xCD,
            0xDC, 0xBA,
        ]);

        assert_that_opcode_was_generated(
            result,
            OpCode::Post {
                p: Option::None,
                num: 0xDEADBEEF,
                den: 0xCAFEBABE,
                mag: 0xBAAAAAAD,
                l: 0xDEADC0DE,
                u: 0xB105F00D,
                s: 0xABCD,
                t: 0xDCBA,
            }
        )
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn test_disassemble_post_post() {
        let result = disassemble(vec![
            249,
            0xAB, 0xCD, 0xEF, 0xAA,
            0x42,
            0xDF, 0xDF
        ]);

        assert_that_opcode_was_generated(
            result,
            OpCode::PostPost {
                q: Option::None,
                i: 0x42,
            }
        )
    }

    // Helper

    fn assert_that_opcode_was_generated(result: DviousResult<Vec<OpCode>>, opcode: OpCode) {
        let opcodes = result.unwrap();
        assert_eq!(opcodes.len(), 1);
        assert_eq!(opcodes[0], opcode);
    }
}
