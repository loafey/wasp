use super::error::ParseError;
use std::{fmt::Debug, io::Cursor};

pub trait Parsable: Debug {
    fn parse(data: &mut Cursor<&[u8]>) -> Result<Self, ParseError>
    where
        Self: std::marker::Sized;
}

impl<T: Parsable> Parsable for Vec<T> {
    fn parse(data: &mut Cursor<&[u8]>) -> Result<Self, ParseError>
    where
        Self: std::marker::Sized,
    {
        let len = u32::parse(data)?;
        let mut v = Vec::new();
        for _ in 0..len {
            v.push(T::parse(data)?);
        }
        Ok(v)
    }
}

impl Parsable for u32 {
    fn parse(data: &mut Cursor<&[u8]>) -> Result<Self, ParseError>
    where
        Self: std::marker::Sized,
    {
        Ok(leb128::read::unsigned(data)? as u32)
    }
}
