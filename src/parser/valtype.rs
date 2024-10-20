use std::io::Read;

use crate::hex::Hex;

use super::Parsable;

#[derive(Debug)]
pub enum ValType {
    NumType(NumType),
    VecType,
    RefType(RefTyp),
}
#[derive(Debug)]
pub enum NumType {
    I32,
    I64,
    F32,
    F64,
}
#[derive(Debug)]
pub enum RefTyp {
    FuncRef,
    ExternRef,
}
impl Parsable for ValType {
    fn parse(data: &mut std::io::Cursor<&[u8]>) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let mut b = [0];
        data.read_exact(&mut b)?;
        Ok(match b[0] {
            0x7F => ValType::NumType(NumType::I32),
            0x7E => ValType::NumType(NumType::I64),
            0x7D => ValType::NumType(NumType::F32),
            0x7C => ValType::NumType(NumType::F64),
            0x7B => ValType::VecType,
            0x70 => ValType::RefType(RefTyp::FuncRef),
            0x6F => ValType::RefType(RefTyp::ExternRef),
            _ => Err(super::error::ParseError::UnknownType(Hex(b)))?,
        })
    }
}
