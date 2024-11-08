use crate::hex::Hex;
use std::io;

#[derive(Debug)]
#[allow(unused)]
pub enum ParseError {
    ExponentTooLarge(u32),
    InvalidModule(ModuleError),
    InvalidSection(SectionError),
    NotImplemented(&'static str),
    IOError(io::Error),
    Leb128(wasabi_leb128::ParseLeb128Error),
    SignedIntegerTooLarge(i64),
    UnsignedIntegerTooLarge(u64),
    UnknownType(Hex<1>),
    InvalidFuncType(Hex<1>),
    InvalidImportDesc(Hex<1>),
    InvalidExportDesc(Hex<1>),
    InvalidLimit(Hex<1>),
    UnknownInstruction(Hex<1>),
    Unknown0x40(Hex<1>),
    InvalidData(Hex<1>),
    InvalidRefType(Hex<1>),
    InvalidElem(u32),
    EndOfInstructions,
    AlignmentError,
    SectionSizeMismatch,
    TooManyLocals(u32),
    InconsistentFunctionAndCodeSectionLength,
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
impl From<wasabi_leb128::ParseLeb128Error> for ParseError {
    fn from(value: wasabi_leb128::ParseLeb128Error) -> Self {
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
