#[derive(Debug, PartialEq)]
#[cfg_attr(rustfmt, rustfmt_skip)]
pub enum OpCode {
    SetChar { n: u32 },
    Set1 { n: u32 },
    Set2 { n: u32 },
    Set3 { n: u32 },
    Set4 { n: i32 },
    SetRule { a: i32, b: i32 },
    Put1 { n: u32 },
    Put2 { n: u32 },
    Put3 { n: u32 },
    Put4 { n: i32 },
    PutRule { a: i32, b: i32 },
    Nop,
    Bop {c0: i32, c1: i32, c2: i32, c3: i32, c4: i32, c5: i32, c6: i32, c7: i32, c8: i32, c9: i32, p: i32 },
    Eop,
    Push,
    Pop,
}
