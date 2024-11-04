use crate::alloc;

use super::error::ParseError;
use std::{
    fmt::Debug,
    io::{Cursor, Read},
};

pub type DebugStack<'t> = &'t mut Vec<&'static str>;

pub trait Parsable: Debug {
    const STACK_NAME: &'static str = std::any::type_name::<Self>();
    fn parse(data: &mut Cursor<&[u8]>, stack: DebugStack) -> Result<Self, ParseError>
    where
        Self: std::marker::Sized,
    {
        #[cfg(target_os = "windows")] // ugly fix to disable this on all important platforms
        {
            stack.push(Self::STACK_NAME);
            println!(
                "{}┌─Parsing {} 💅",
                "│ ".repeat(stack.len()),
                Self::STACK_NAME
            );
        }
        #[allow(deprecated)]
        let res = Self::parse_inner(data, stack)?;
        #[cfg(target_os = "windows")] // ugly fix to disable this on all important platforms
        {
            let format = format!("{res:?}");
            let formatted = &format[0..(format.len().min(128))];
            let eps = if formatted.len() != format.len() {
                "..."
            } else {
                ""
            };
            println!("{}├─Result: {formatted}{eps} 🏳️‍🌈", "│ ".repeat(stack.len()),);
            stack.pop();
        }
        Ok(res)
    }
    #[deprecated]
    fn parse_inner(data: &mut Cursor<&[u8]>, stack: DebugStack) -> Result<Self, ParseError>
    where
        Self: std::marker::Sized;
}

impl<T: Parsable> Parsable for Vec<T> {
    fn parse_inner(data: &mut Cursor<&[u8]>, stack: DebugStack) -> Result<Self, ParseError>
    where
        Self: std::marker::Sized,
    {
        let len = u32::parse(data, stack)?;
        let mut v = Vec::new();
        for _ in 0..len {
            let value = T::parse(data, stack)?;
            v.push(value);
        }
        Ok(v)
    }
}
impl Parsable for u8 {
    fn parse_inner(data: &mut Cursor<&[u8]>, _: DebugStack) -> Result<Self, ParseError>
    where
        Self: std::marker::Sized,
    {
        let mut b = [0];
        data.read_exact(&mut b)?;
        Ok(b[0])
    }
}

impl Parsable for u32 {
    fn parse_inner(data: &mut Cursor<&[u8]>, _: DebugStack) -> Result<Self, ParseError>
    where
        Self: std::marker::Sized,
    {
        Ok(leb128::read::unsigned(data)? as u32)
    }
}

impl Parsable for f32 {
    fn parse_inner(data: &mut Cursor<&[u8]>, _: DebugStack) -> Result<Self, ParseError>
    where
        Self: std::marker::Sized,
    {
        let mut buf = alloc::<4>();
        data.read_exact(&mut buf.0)?;
        Ok(f32::from_le_bytes(buf.0))
    }
}

impl Parsable for f64 {
    fn parse_inner(data: &mut Cursor<&[u8]>, _: DebugStack) -> Result<Self, ParseError>
    where
        Self: std::marker::Sized,
    {
        let mut buf = alloc::<8>();
        data.read_exact(&mut buf.0)?;
        Ok(f64::from_le_bytes(buf.0))
    }
}

impl Parsable for i32 {
    fn parse_inner(data: &mut Cursor<&[u8]>, _: DebugStack) -> Result<Self, ParseError>
    where
        Self: std::marker::Sized,
    {
        Ok(leb128::read::signed(data)? as i32)
    }
}
impl Parsable for i64 {
    fn parse_inner(data: &mut Cursor<&[u8]>, _: DebugStack) -> Result<Self, ParseError>
    where
        Self: std::marker::Sized,
    {
        Ok(leb128::read::signed(data)?)
    }
}
