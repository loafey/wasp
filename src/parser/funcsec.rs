use std::io::Read;

use crate::parser::error::SectionError;

use super::{Parsable, TypeIdX};

#[derive(Debug)]
#[allow(unused)]
pub struct FunctionSection {
    pub size: u32,
    pub functions: Vec<TypeIdX>,
}
impl Parsable for FunctionSection {
    fn parse(data: &mut std::io::Cursor<&[u8]>) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let mut n = [0u8];
        data.read_exact(&mut n)?;
        if !matches!(n, [3]) {
            Err(SectionError::InvalidHeader(3, n[0]))?;
        }

        let size = u32::parse(data)?;
        let functions = Vec::parse(data)?;
        Ok(Self { size, functions })
    }
}
