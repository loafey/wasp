use super::{error::ParseError, GlobalType, MemType, Parsable, Pretty, TableType, TypeIdX};
use crate::hex::Hex;
use std::io::Read;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(unused)]
pub enum ExportDesc {
    Func(TypeIdX),
    Table(TableType),
    Mem(MemType),
    Global(GlobalType),
}
impl Parsable for ExportDesc {
    fn parse_inner(data: &mut std::io::Cursor<&[u8]>) -> Result<Self, ParseError>
    where
        Self: std::marker::Sized,
    {
        let mut b = [0];
        data.read_exact(&mut b)?;
        Ok(match b[0] {
            0x00 => Self::Func(TypeIdX::parse_inner(data)?),
            0x01 => Self::Table(TableType::parse_inner(data)?),
            0x02 => Self::Mem(MemType::parse_inner(data)?),
            0x03 => Self::Global(GlobalType::parse_inner(data)?),
            _ => Err(ParseError::InvalidExportDesc(Hex(b)))?,
        })
    }
}
impl Pretty for ExportDesc {
    fn pretty_indent(&self, _: usize) -> String {
        match self {
            ExportDesc::Func(type_id_x) => format!("(type {})", type_id_x.pretty()),
            ExportDesc::Table(table_type) => format!("(table {})", table_type.pretty()),
            ExportDesc::Mem(mem_type) => format!("(mem {})", mem_type.pretty()),
            ExportDesc::Global(global_type) => format!("(global {})", global_type.pretty()),
        }
    }
}
