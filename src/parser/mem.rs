use super::{Limits, Parsable};

#[derive(Debug)]
#[allow(unused)]
pub struct Mem {
    pub limits: Limits,
}
impl Parsable for Mem {
    fn parse_inner(data: &mut std::io::Cursor<&[u8]>) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        Ok(Self {
            limits: Limits::parse_inner(data)?,
        })
    }
}
