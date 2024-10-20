use crate::hex::Hex;
use std::io;

#[derive(Debug)]
pub enum ParseError {
    InvalidModule(ModuleError),
    InvalidSection(SectionError),
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
impl From<SectionError> for ParseError {
    fn from(value: SectionError) -> Self {
        Self::InvalidSection(value)
    }
}

#[derive(Debug)]
pub enum ModuleError {
    InvalidHeader(Hex<4>),
    InvalidVersion(Hex<4>),
}

#[derive(Debug)]
pub enum SectionError {
    NotTypeSec(u8),
}
