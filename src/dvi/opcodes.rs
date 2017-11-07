#[derive(Debug, PartialEq)]
pub enum OpCode {
    SetN { n: u64 },
    Nop(),
}
