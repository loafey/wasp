use super::error::ParseError;
use std::{
    fmt::Debug,
    io::{Cursor, Read},
};

pub trait Parsable: Debug {
    fn parse_inner(data: &mut Cursor<&[u8]>) -> Result<Self, ParseError>
    where
        Self: std::marker::Sized;
}

impl<T: Parsable> Parsable for Vec<T> {
    fn parse_inner(data: &mut Cursor<&[u8]>) -> Result<Self, ParseError>
    where
        Self: std::marker::Sized,
    {
        let len = u32::parse_inner(data)?;
        let mut v = Vec::new();
        for _ in 0..len {
            v.push(T::parse_inner(data)?);
        }
        Ok(v)
    }
}
impl Parsable for u8 {
    fn parse_inner(data: &mut Cursor<&[u8]>) -> Result<Self, ParseError>
    where
        Self: std::marker::Sized,
    {
        let mut b = [0];
        data.read_exact(&mut b)?;
        Ok(b[0])
    }
}

impl Parsable for u32 {
    fn parse_inner(data: &mut Cursor<&[u8]>) -> Result<Self, ParseError>
    where
        Self: std::marker::Sized,
    {
        Ok(leb128::read::unsigned(data)? as u32)
    }
}

impl Parsable for i32 {
    fn parse_inner(data: &mut Cursor<&[u8]>) -> Result<Self, ParseError>
    where
        Self: std::marker::Sized,
    {
        Ok(leb128::read::signed(data)? as i32)
    }
}
