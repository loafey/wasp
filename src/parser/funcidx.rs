use std::ops::{Deref, DerefMut};

use super::Parsable;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    fn parse_inner(
        data: &mut std::io::Cursor<&[u8]>,
        stack: super::DebugStack,
    ) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        Ok(Self(u32::parse(data, stack)?))
    }
}
