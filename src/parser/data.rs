use super::{Expr, MemIdX, Parsable, Pretty};
use crate::{hex::Hex, parser::error::ParseError};
use std::io::Read;

#[derive(Debug)]
#[allow(unused)]
pub enum Data {
    Mem0(Expr, Vec<u8>),
    MemB(Vec<u8>),
    MemX(MemIdX, Expr, Vec<u8>),
}
impl Parsable for Data {
    fn parse(data: &mut std::io::Cursor<&[u8]>) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let mut header = [0];
        data.read_exact(&mut header)?;
        Ok(match header[0] {
            0 => Data::Mem0(Expr::parse(data)?, Vec::parse(data)?),
            1 => Data::MemB(Vec::parse(data)?),
            2 => Data::MemX(MemIdX::parse(data)?, Expr::parse(data)?, Vec::parse(data)?),
            _ => Err(ParseError::InvalidData(Hex(header)))?,
        })
    }
}
impl Pretty for Data {
    fn pretty_indent(&self, _: usize) -> String {
        todo!()
    }
}
