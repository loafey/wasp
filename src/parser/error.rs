use crate::hex::Hex;
use std::io;

#[derive(Debug)]
pub enum ParseError {
    InvalidModule(ModuleError),
    NotImplemented(&'static str),
    IOError(io::Error),
}
impl From<io::Error> for ParseError {
    fn from(value: io::Error) -> Self {
        Self::IOError(value)
    }
}
impl From<ModuleError> for ParseError {
    fn from(value: ModuleError) -> Self {
        Self::InvalidModule(value)
    }
}

#[derive(Debug)]
pub enum ModuleError {
    InvalidHeader(Hex<4>),
    InvalidVersion(Hex<4>),
}
