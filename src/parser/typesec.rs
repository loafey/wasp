use super::{error::ParseError, Parsable};
use crate::parser::FuncType;
use std::io::Cursor;

#[derive(Debug, Default)]
#[allow(unused)]
pub struct TypeSection {
    pub size: u32,
    pub function_types: Vec<FuncType>,
}
impl TypeSection {
    pub fn concat(&mut self, mut other: Self) {
        self.size += other.size;
        self.function_types.append(&mut other.function_types);
    }
}
impl Parsable for TypeSection {
    fn parse_inner(
        data: &mut Cursor<&[u8]>,
        stack: super::DebugStack,
    ) -> Result<TypeSection, ParseError> {
        let size = u32::parse(data, stack)?;
        let expected = data.position() + size as u64;
        let function_types: Vec<FuncType> = Vec::parse(data, stack)?;
        if data.position() != expected {
            return Err(ParseError::SectionSizeMismatch);
        }
        Ok(Self {
            size,
            function_types,
        })
    }
}
