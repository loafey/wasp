use std::io::Cursor;

use super::error::ParseError;

#[derive(Debug)]
pub struct CustomSection {}
impl CustomSection {
    pub fn parse(data: &mut Cursor<&[u8]>) -> Result<CustomSection, ParseError> {
        todo!()
    }
}
