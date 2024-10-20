use crate::parser::error::SectionError;

use super::error::ParseError;
use std::io::{Cursor, Read};

#[derive(Debug)]
pub struct TypeSection {}
impl TypeSection {
    pub fn parse(data: &mut Cursor<&[u8]>) -> Result<TypeSection, ParseError> {
        let mut n = [0u8];
        data.read_exact(&mut n)?;
        if !matches!(n, [1]) {
            Err(SectionError::NotTypeSec(n[0]))?;
        }
        todo!()
    }
}
