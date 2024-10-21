use super::Parsable;

#[derive(Debug, Clone, Copy)]
#[allow(unused)]
pub struct MemArg {
    align: u32,
    offset: u32,
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
