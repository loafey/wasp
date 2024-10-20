use std::ops::{Deref, DerefMut};

use super::{Limits, Parsable};

#[derive(Debug)]
pub struct MemType(Limits);
impl Deref for MemType {
    type Target = Limits;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for MemType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Parsable for MemType {
    fn parse(data: &mut std::io::Cursor<&[u8]>) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        Ok(Self(Limits::parse(data)?))
    }
}
