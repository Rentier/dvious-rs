#[derive(Debug, PartialEq)]
pub enum OpCode {
    SetN { n: u32 },
    SetRule { a: i32, b: i32 },
}
