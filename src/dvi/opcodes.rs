use std::fmt;

#[derive(Debug, PartialEq)]
#[cfg_attr(rustfmt, rustfmt_skip)]
pub enum OpCode {
    SetChar { c: u32 },
    Set { c: i32 },
    SetRule { a: i32, b: i32 },
    Put { c: i32 },
    PutRule { a: i32, b: i32 },
    Nop,
    Bop {c0: i32, c1: i32, c2: i32, c3: i32, c4: i32, c5: i32, c6: i32, c7: i32, c8: i32, c9: i32, p: i32 },
    Eop,
    Push,
    Pop,
    Right { b: i32 },
    W0,
    W { b: i32 },
    X0,
    X { b: i32 },
    Down { a: i32 },
    Y0,
    Y { a: i32 },
    Z0,
    Z { a: i32 },
    FntNum { k: u32},
    Fnt { k: i32 },
    Xxx { k: i32, x: Vec<u8> },
    FntDef { k: i32, c: u32, s: u32, d: u32, a: u8, l: u8, n: Vec<u8> },
    Pre { i: u8, num: u32, den: u32, mag: u32, k: u8, x: Vec<u8> },
    Post { p: Option<usize>, num: u32, den: u32, mag: u32, l: u32, u: u32, s: u16, t: u16 },
    PostPost { q: Option<usize>, i: u8 }
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
