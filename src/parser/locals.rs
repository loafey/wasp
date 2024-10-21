use super::Parsable;

#[derive(Debug)]
pub struct Locals;
impl Parsable for Locals {
    fn parse_inner(_data: &mut std::io::Cursor<&[u8]>) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}
