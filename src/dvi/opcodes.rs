#[derive(Debug, PartialEq)]
pub enum OpCode {
    SetN(u32),
    Nop(),
}
