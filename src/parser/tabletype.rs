use super::{Limits, Parsable, RefTyp};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TableType {
    pub et: RefTyp,
    pub lim: Limits,
}
impl Parsable for TableType {
    fn parse_inner(
        data: &mut std::io::Cursor<&[u8]>,
        stack: super::DebugStack,
    ) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        Ok(TableType {
            et: Parsable::parse(data, stack)?,
            lim: Parsable::parse(data, stack)?,
        })
    }
}
