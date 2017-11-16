use std::io;

#[derive(Debug)]
pub enum DviousError {
    IoError(io::Error),
    KpsewhichError(String),
    IndexOutOfBoundsError,
    UnknownOpcodeError(u8),
}

pub type DviousResult<T> = Result<T, DviousError>;

impl From<io::Error> for DviousError {
    fn from(error: io::Error) -> Self {
        DviousError::IoError(error)
    }
}
