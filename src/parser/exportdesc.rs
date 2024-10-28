use super::{error::ParseError, GlobalIdX, MemIdX, Parsable, TableIdX, TypeIdX};
use crate::hex::Hex;
use std::io::Read;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[allow(unused)]
pub enum ExportDesc {
    Func(TypeIdX),
    Table(TableIdX),
    Mem(MemIdX),
    Global(GlobalIdX),
}
impl Parsable for ExportDesc {
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
            0x01 => Self::Table(TableIdX::parse(data, stack)?),
            0x02 => Self::Mem(MemIdX::parse(data, stack)?),
            0x03 => Self::Global(GlobalIdX::parse(data, stack)?),
            _ => Err(ParseError::InvalidExportDesc(Hex(b)))?,
        })
    }
}
