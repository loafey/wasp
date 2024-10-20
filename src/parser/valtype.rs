use std::io::Read;

use crate::hex::Hex;

use super::Parsable;

#[derive(Debug)]
#[allow(unused)]
pub enum ValType {
    Num(NumType),
    Vec,
    Ref(RefTyp),
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
            0x7F => ValType::Num(NumType::I32),
            0x7E => ValType::Num(NumType::I64),
            0x7D => ValType::Num(NumType::F32),
            0x7C => ValType::Num(NumType::F64),
            0x7B => ValType::Vec,
            0x70 => ValType::Ref(RefTyp::FuncRef),
            0x6F => ValType::Ref(RefTyp::ExternRef),
            _ => Err(super::error::ParseError::UnknownType(Hex(b)))?,
        })
    }
}
