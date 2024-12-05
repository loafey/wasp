use super::{Mutable, Parsable, ValType};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GlobalType {
    pub t: ValType,
    pub mutable: Mutable,
}
impl Parsable for GlobalType {
    fn parse_inner(
        data: &mut std::io::Cursor<&[u8]>,
        stack: super::DebugStack,
    ) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let t = ValType::parse(data, stack)?;
        let mutable = Mutable::parse(data, stack)?;
        Ok(Self { t, mutable })
    }
}
