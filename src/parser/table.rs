use std::io::Read;

use crate::hex::Hex;

use super::{error::ParseError, Limits, Parsable, RefTyp};

#[derive(Debug)]
#[allow(unused)]
pub struct Table {
    et: RefTyp,
    lim: Limits,
}
impl Parsable for Table {
    fn parse_inner(
        data: &mut std::io::Cursor<&[u8]>,
        stack: super::DebugStack,
    ) -> Result<Self, ParseError>
    where
        Self: std::marker::Sized,
    {
        let mut et = [0];
        data.read_exact(&mut et)?;
        let et = match et[0] {
            0x70 => RefTyp::FuncRef,
            0x6F => RefTyp::ExternRef,
            _ => Err(ParseError::UnknownType(Hex(et)))?,
        };
        let lim = Limits::parse(data, stack)?;
        Ok(Self { et, lim })
    }
}
