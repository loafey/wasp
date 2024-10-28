use std::io::Read;

use crate::hex::Hex;

use super::{error::ParseError, GlobalType, MemType, Parsable, TableType, TypeIdX};

#[derive(Debug)]
#[allow(unused)]
pub enum ImportDesc {
    Func(TypeIdX),
    Table(TableType),
    Mem(MemType),
    Global(GlobalType),
}
impl Parsable for ImportDesc {
    fn parse_inner(
        data: &mut std::io::Cursor<&[u8]>,
        stack: super::DebugStack,
    ) -> Result<Self, ParseError>
    where
        Self: std::marker::Sized,
    {
        let mut b = [0];
        data.read_exact(&mut b)?;
        Ok(match b[0] {
            0x00 => Self::Func(TypeIdX::parse(data, stack)?),
            0x01 => Self::Table(TableType::parse(data, stack)?),
            0x02 => Self::Mem(MemType::parse(data, stack)?),
            0x03 => Self::Global(GlobalType::parse(data, stack)?),
            _ => Err(ParseError::InvalidImportDesc(Hex(b)))?,
        })
    }
}
