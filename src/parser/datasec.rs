use super::Parsable;

#[derive(Debug)]
pub struct DataSection;
impl Parsable for DataSection {
    fn parse(_data: &mut std::io::Cursor<&[u8]>) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}
