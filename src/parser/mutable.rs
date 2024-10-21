use std::io::Read;

use crate::hex::Hex;

use super::{error::ParseError, Parsable};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Mutable {
    Const,
    Var,
}
impl Parsable for Mutable {
    fn parse_inner(data: &mut std::io::Cursor<&[u8]>) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let mut b = [0];
        data.read_exact(&mut b)?;
        Ok(match b[0] {
            0x00 => Mutable::Const,
            0x01 => Mutable::Var,
            _ => Err(ParseError::InvalidData(Hex(b)))?,
        })
    }
}
