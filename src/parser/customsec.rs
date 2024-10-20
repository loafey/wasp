use super::{error::ParseError, Parsable};
use std::io::{Cursor, Read};

#[derive(Debug)]
pub struct CustomSection {}
impl Parsable for CustomSection {
    fn parse(data: &mut Cursor<&[u8]>) -> Result<CustomSection, ParseError> {
        let mut n = [0u8];
        data.read_exact(&mut n)?;

        todo!()
    }
}
