use std::io::Read;

use crate::hex::Hex;

use super::{Parsable, Pretty};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(unused)]
pub enum Limits {
    Min(u32),
    MinMax(u32, u32),
}
impl Parsable for Limits {
    fn parse_inner(data: &mut std::io::Cursor<&[u8]>) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let mut b = [0];
        data.read_exact(&mut b)?;
        Ok(match b[0] {
            0x00 => {
                let min = u32::parse_inner(data)?;
                Self::Min(min)
            }
            0x01 => {
                let min = u32::parse_inner(data)?;
                let max = u32::parse_inner(data)?;
                Self::MinMax(min, max)
            }
            _ => Err(super::error::ParseError::InvalidLimit(Hex(b)))?,
        })
    }
}
impl Pretty for Limits {
    fn pretty_indent(&self, _: usize) -> String {
        match self {
            Limits::Min(n) => format!("{n} ≤ N ≤ ε"),
            Limits::MinMax(n, m) => format!("{n} ≤ N ≤ {m}"),
        }
    }
}
