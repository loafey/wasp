use crate::hex::Hex;
use std::io;

#[derive(Debug)]
#[allow(unused)]
pub enum ParseError {
    InvalidModule(ModuleError),
    InvalidSection(SectionError),
    NotImplemented(&'static str),
    IOError(io::Error),
    Leb128(leb128::read::Error),
    UnknownType(Hex<1>),
    InvalidFuncType(Hex<1>),
    InvalidImportDesc(Hex<1>),
    InvalidLimit(Hex<1>),
    UnknownInstruction(Hex<1>),
    EndOfInstructions,
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
impl From<leb128::read::Error> for ParseError {
    fn from(value: leb128::read::Error) -> Self {
        Self::Leb128(value)
    }
}

#[derive(Debug)]
#[allow(unused)]
pub enum ModuleError {
    InvalidHeader(Hex<4>),
    InvalidVersion(Hex<4>),
}

#[derive(Debug)]
#[allow(unused)]
pub enum SectionError {
    UnknownHeader(Hex<1>),
}
