use super::{Mutable, Parsable, Pretty, ValType};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GlobalType {
    pub t: ValType,
    pub mutable: Mutable,
}
impl Parsable for GlobalType {
    fn parse_inner(data: &mut std::io::Cursor<&[u8]>) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        Ok(Self {
            t: ValType::parse_inner(data)?,
            mutable: Mutable::parse_inner(data)?,
        })
    }
}
impl Pretty for GlobalType {
    fn pretty_indent(&self, _: usize) -> String {
        todo!()
    }
}
