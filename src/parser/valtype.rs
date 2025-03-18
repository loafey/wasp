use std::io::Read;

use crate::{hex::Hex, runtime::Value};

use super::Parsable;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[allow(unused)]
pub enum ValType {
    Poly,
    Num(NumType),
    Vec128,
    Ref(RefTyp),
}
impl ValType {
    pub fn default_value(&self) -> Value {
        match self {
            ValType::Poly => todo!(),
            ValType::Num(num_type) => num_type.default_value(),
            ValType::Vec128 => todo!(),
            ValType::Ref(_) => todo!(),
        }
    }
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum NumType {
    I32,
    I64,
    F32,
    F64,
}
impl NumType {
    pub fn default_value(&self) -> Value {
        match self {
            NumType::I32 => Value::I32(0),
            NumType::I64 => Value::I64(0),
            NumType::F32 => Value::F32(0.0),
            NumType::F64 => Value::F64(0.0),
        }
    }
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum RefTyp {
    FuncRef,
    ExternRef,
}
impl Parsable for RefTyp {
    fn parse_inner(
        data: &mut std::io::Cursor<&[u8]>,
        _: super::DebugStack,
    ) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let mut h = [0];
        data.read_exact(&mut h)?;
        match h[0] {
            0x00 | 0x70 => Ok(RefTyp::FuncRef),
            0x01 | 0x6f => Ok(RefTyp::ExternRef),
            _ => Err(super::error::ParseError::InvalidRefType(Hex(h))),
        }
    }
}
impl Parsable for ValType {
    fn parse_inner(
        data: &mut std::io::Cursor<&[u8]>,
        _: super::DebugStack,
    ) -> Result<Self, super::error::ParseError>
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
            0x7B => ValType::Vec128,
            0x70 => ValType::Ref(RefTyp::FuncRef),
            0x6F => ValType::Ref(RefTyp::ExternRef),
            _ => Err(super::error::ParseError::UnknownType(Hex(b)))?,
        })
    }
}
