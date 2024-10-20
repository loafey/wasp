use super::{Parsable, ValType};

#[derive(Debug)]
pub struct ResultType {
    pub types: Vec<ValType>,
}
impl Parsable for ResultType {
    fn parse(data: &mut std::io::Cursor<&[u8]>) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        Ok(ResultType {
            types: Vec::parse(data)?,
        })
    }
}
