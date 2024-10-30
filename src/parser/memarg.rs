use super::Parsable;

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
        Ok(Self {
            align: Parsable::parse(data, stack)?,
            offset: Parsable::parse(data, stack)?,
        })
    }
}
