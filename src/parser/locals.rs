use super::{error::ParseError, Parsable, ValType};

#[derive(Debug, Clone, Copy)]
#[allow(unused)]
pub struct Locals {
    pub n: u32,
    pub t: ValType,
}
impl Parsable for Locals {
    fn parse_inner(
        data: &mut std::io::Cursor<&[u8]>,
        stack: super::DebugStack,
    ) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let n = Parsable::parse(data, stack)?;
        if n >= 0x10000000 {
            return Err(ParseError::TooManyLocals(n));
        }
        let t = Parsable::parse(data, stack)?;

        Ok(Self { n, t })
    }
}
