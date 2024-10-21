use std::io::Read;

use crate::hex::Hex;

use super::{Parsable, Pretty};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(unused)]
pub enum ValType {
    Num(NumType),
    Vec,
    Ref(RefTyp),
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum NumType {
    I32,
    I64,
    F32,
    F64,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum RefTyp {
    FuncRef,
    ExternRef,
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
            0x7B => ValType::Vec,
            0x70 => ValType::Ref(RefTyp::FuncRef),
            0x6F => ValType::Ref(RefTyp::ExternRef),
            _ => Err(super::error::ParseError::UnknownType(Hex(b)))?,
        })
    }
}

impl Pretty for ValType {
    fn pretty_indent(&self, _: usize) -> String {
        match self {
            ValType::Num(num_type) => num_type.pretty(),
            ValType::Vec => "vec".to_string(),
            ValType::Ref(ref_typ) => ref_typ.pretty(),
        }
    }
}
impl Pretty for NumType {
    fn pretty_indent(&self, _: usize) -> String {
        match self {
            NumType::I32 => "i32",
            NumType::I64 => "i64",
            NumType::F32 => "f32",
            NumType::F64 => "f64",
        }
        .to_string()
    }
}
impl Pretty for RefTyp {
    fn pretty_indent(&self, _: usize) -> String {
        match self {
            RefTyp::FuncRef => "func_ref",
            RefTyp::ExternRef => "extern_ref",
        }
        .to_string()
    }
}
