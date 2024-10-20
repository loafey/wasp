use std::io::Read;

use crate::hex::Hex;

use super::Parsable;

#[derive(Debug)]
#[allow(unused)]
pub enum Limits {
    Min(u32),
    MinMax(u32, u32),
}
impl Parsable for Limits {
    fn parse(data: &mut std::io::Cursor<&[u8]>) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let mut b = [0];
        data.read_exact(&mut b)?;
        Ok(match b[0] {
            0x00 => {
                let min = u32::parse(data)?;
                Self::Min(min)
            }
            0x01 => {
                let min = u32::parse(data)?;
                let max = u32::parse(data)?;
                Self::MinMax(min, max)
            }
            _ => Err(super::error::ParseError::InvalidLimit(Hex(b)))?,
        })
    }
}
