use super::{error::ParseError, Parsable};

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(unused)]
pub struct MemArg {
    pub align: u32,
    pub offset: u32,
}

impl Parsable for MemArg {
    fn parse_inner(
        data: &mut std::io::Cursor<&[u8]>,
        stack: super::DebugStack,
    ) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let exp = u32::parse(data, stack)?;
        if exp >= 32 {
            return Err(ParseError::ExponentTooLarge(exp));
        }
        Ok(Self {
            align: 2u32.pow(exp),
            offset: Parsable::parse(data, stack)?,
        })
    }
}
