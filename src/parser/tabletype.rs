use super::{Parsable, Pretty};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TableType;
impl Parsable for TableType {
    fn parse_inner(
        _data: &mut std::io::Cursor<&[u8]>,
        _: super::DebugStack,
    ) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}
impl Pretty for TableType {
    fn pretty_indent(&self, _: usize) -> String {
        todo!()
    }
}
