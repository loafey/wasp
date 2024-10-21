use std::ops::{Deref, DerefMut};

use super::{Parsable, Pretty};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TypeIdX(pub u32);
impl Deref for TypeIdX {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for TypeIdX {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Parsable for TypeIdX {
    fn parse(data: &mut std::io::Cursor<&[u8]>) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        Ok(Self(u32::parse(data)?))
    }
}
impl Pretty for TypeIdX {
    fn pretty_indent(&self, _: usize) -> String {
        format!("TypeIdX({})", self.0)
    }
}
