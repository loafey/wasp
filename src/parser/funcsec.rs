use super::{Parsable, TypeIdX};

#[derive(Debug)]
#[allow(unused)]
pub struct FunctionSection {
    pub size: u32,
    pub functions: Vec<TypeIdX>,
}
impl Parsable for FunctionSection {
    fn parse(data: &mut std::io::Cursor<&[u8]>) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let size = u32::parse(data)?;
        let functions = Vec::parse(data)?;
        Ok(Self { size, functions })
    }
}
