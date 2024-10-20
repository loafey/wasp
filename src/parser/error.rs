use crate::hex::Hex;
use std::io;

#[derive(Debug)]
pub enum ParseError {
    #[allow(unused)]
    InvalidModule(ModuleError),
    #[allow(unused)]
    InvalidSection(SectionError),
    #[allow(unused)]
    NotImplemented(&'static str),
    #[allow(unused)]
    IOError(io::Error),
    #[allow(unused)]
    Leb128(leb128::read::Error),
    #[allow(unused)]
    UnknownType(Hex<1>),
    #[allow(unused)]
    InvalidFuncType(Hex<1>),
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
pub enum ModuleError {
    #[allow(unused)]
    InvalidHeader(Hex<4>),
    #[allow(unused)]
    InvalidVersion(Hex<4>),
}

#[derive(Debug)]
pub enum SectionError {
    #[allow(unused)]
    NotTypeSec(u8),
}
