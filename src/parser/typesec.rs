use super::{error::ParseError, Parsable};
use crate::parser::FuncType;
use std::io::Cursor;

#[derive(Debug)]
#[allow(unused)]
pub struct TypeSection {
    pub size: u32,
    pub function_types: Vec<FuncType>,
}
impl Parsable for TypeSection {
    fn parse(data: &mut Cursor<&[u8]>) -> Result<TypeSection, ParseError> {
        let size = u32::parse(data)?;
        let function_types: Vec<FuncType> = Vec::parse(data)?;
        Ok(Self {
            size,
            function_types,
        })
    }
}
