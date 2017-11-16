use std::io;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum DviousError {
    IoError(io::Error),
    KpsewhichError(String),
    IndexOutOfBoundsError,
    TfmParseError(String),
    UnknownOpcodeError(u8),
    Utf8Error(FromUtf8Error),
}

pub type DviousResult<T> = Result<T, DviousError>;

impl From<io::Error> for DviousError {
    fn from(error: io::Error) -> Self {
        DviousError::IoError(error)
    }
}

impl From<FromUtf8Error> for DviousError {
    fn from(error: FromUtf8Error) -> Self {
        DviousError::Utf8Error(error)
    }
}
