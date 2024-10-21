use std::ops::{Deref, DerefMut};

use super::Parsable;

#[derive(Debug, Clone, Copy)]
pub struct FuncIdx(pub u32);
impl Deref for FuncIdx {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for FuncIdx {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Parsable for FuncIdx {
    fn parse_inner(data: &mut std::io::Cursor<&[u8]>) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        Ok(Self(u32::parse_inner(data)?))
    }
}
