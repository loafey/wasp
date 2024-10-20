use std::io::Cursor;

use super::error::ParseError;

pub trait Parsable {
    fn parse(data: &mut Cursor<&[u8]>) -> Result<Self, ParseError>
    where
        Self: std::marker::Sized;
}
