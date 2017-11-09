use std::fmt;

#[derive(Debug, PartialEq)]
#[cfg_attr(rustfmt, rustfmt_skip)]
pub enum OpCode {
    SetChar { c: u32 },
    Set1 { c: u32 },
    Set2 { c: u32 },
    Set3 { c: u32 },
    Set4 { c: i32 },
    SetRule { a: i32, b: i32 },
    Put1 { c: u32 },
    Put2 { c: u32 },
    Put3 { c: u32 },
    Put4 { c: i32 },
    PutRule { a: i32, b: i32 },
    Nop,
    Bop {c0: i32, c1: i32, c2: i32, c3: i32, c4: i32, c5: i32, c6: i32, c7: i32, c8: i32, c9: i32, p: i32 },
    Eop,
    Push,
    Pop,
    Right1 { b: i32 },
    Right2 { b: i32 },
    Right3 { b: i32 },
    Right4 { b: i32 },
    W0,
    W1 { b: i32 },
    W2 { b: i32 },
    W3 { b: i32 },
    W4 { b: i32 },
    X0,
    X1 { b: i32 },
    X2 { b: i32 },
    X3 { b: i32 },
    X4 { b: i32 },
    Down1 { a: i32 },
    Down2 { a: i32 },
    Down3 { a: i32 },
    Down4 { a: i32 },
    Y0,
    Y1 { a: i32 },
    Y2 { a: i32 },
    Y3 { a: i32 },
    Y4 { a: i32 },
    Z0,
    Z1 { a: i32 },
    Z2 { a: i32 },
    Z3 { a: i32 },
    Z4 { a: i32 },
    FntNum { k: u32},
    Fnt1 { k: u32 },
    Fnt2 { k: u32 },
    Fnt3 { k: u32 },
    Fnt4 { k: i32 },
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
