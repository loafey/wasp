use crate::parser::{error::SectionError, FuncType};

use super::{error::ParseError, Parsable};
use std::io::{Cursor, Read};

#[derive(Debug)]
pub struct TypeSection {
    pub size: u32,
    pub function_types: Vec<FuncType>,
}
impl Parsable for TypeSection {
    fn parse(data: &mut Cursor<&[u8]>) -> Result<TypeSection, ParseError> {
        let mut n = [0u8];
        data.read_exact(&mut n)?;
        if !matches!(n, [1]) {
            Err(SectionError::NotTypeSec(n[0]))?;
        }

        let size = u32::parse(data)?;

        let function_types: Vec<FuncType> = Vec::parse(data)?;
        Ok(Self {
            size,
            function_types,
        })
    }
}
