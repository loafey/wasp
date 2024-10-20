use super::Parsable;

#[derive(Debug)]
pub struct TableType;
impl Parsable for TableType {
    fn parse(_data: &mut std::io::Cursor<&[u8]>) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}
