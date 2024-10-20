use super::{error::ParseError, GlobalType, MemType, Parsable, TableType, TypeIdX};
use crate::hex::Hex;
use std::io::Read;

#[derive(Debug)]
#[allow(unused)]
pub enum ExportDesc {
    Func(TypeIdX),
    Table(TableType),
    Mem(MemType),
    Global(GlobalType),
}
impl Parsable for ExportDesc {
    fn parse(data: &mut std::io::Cursor<&[u8]>) -> Result<Self, ParseError>
    where
        Self: std::marker::Sized,
    {
        let mut b = [0];
        data.read_exact(&mut b)?;
        Ok(match b[0] {
            0x00 => Self::Func(TypeIdX::parse(data)?),
            0x01 => Self::Table(TableType::parse(data)?),
            0x02 => Self::Mem(MemType::parse(data)?),
            0x03 => Self::Global(GlobalType::parse(data)?),
            _ => Err(ParseError::InvalidImportDesc(Hex(b)))?,
        })
    }
}
