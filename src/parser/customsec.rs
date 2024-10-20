use std::io::{Cursor, Read};

use crate::parser::error::SectionError;

use super::error::ParseError;

#[derive(Debug)]
pub struct CustomSection {}
impl CustomSection {
    pub fn parse(data: &mut Cursor<&[u8]>) -> Result<CustomSection, ParseError> {
        let mut n = [0u8];
        data.read_exact(&mut n)?;

        todo!()
    }
}
