use std::io;

#[derive(Debug)]
pub enum MgError {
    Io(io::Error),
    Protocol(String),
}

impl From<io::Error> for MgError {
    fn from(e: io::Error) -> Self {
        MgError::Io(e)
    }
}