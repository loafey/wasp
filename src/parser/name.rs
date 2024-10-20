use std::ops::{Deref, DerefMut};

use super::Parsable;

#[derive(Debug)]
pub struct Name(String);
impl Deref for Name {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Name {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Parsable for Name {
    fn parse(data: &mut std::io::Cursor<&[u8]>) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let bytes = Vec::parse(data)?;
        let s = String::from_utf8_lossy(&bytes);
        Ok(Name(s.to_string()))
    }
}
