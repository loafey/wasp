use super::Parsable;

#[derive(Debug)]
pub struct GlobalType;
impl Parsable for GlobalType {
    fn parse(_data: &mut std::io::Cursor<&[u8]>) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}
