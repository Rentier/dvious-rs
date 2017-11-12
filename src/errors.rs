use std::io;

impl From<io::Error> for DviousError {
    fn from(error: io::Error) -> Self {
        DviousError::IoError(error)
    }
}

pub enum DviousError {
    IoError(io::Error),
    KpsewhichError(String)
}
