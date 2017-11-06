#[derive(Debug, PartialEq)]
pub enum OpCode {
    SetN(u8),
    Nop(),
}
