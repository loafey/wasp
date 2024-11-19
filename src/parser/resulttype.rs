use super::{Parsable, ValType};

#[derive(Clone, PartialEq, Eq)]
#[allow(unused)]
pub struct ResultType {
    pub types: Vec<ValType>,
}

impl std::fmt::Debug for ResultType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.types)
    }
}
impl Parsable for ResultType {
    fn parse_inner(
        data: &mut std::io::Cursor<&[u8]>,
        stack: super::DebugStack,
    ) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        Ok(ResultType {
            types: Vec::parse(data, stack)?,
        })
    }
}
