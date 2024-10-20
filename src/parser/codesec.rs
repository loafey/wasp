use std::io::Read;

use super::{error::SectionError, Code, Parsable};

#[derive(Debug)]
#[allow(unused)]
pub struct CodeSection {
    pub size: u32,
    pub code: Vec<Code>,
}

impl Parsable for CodeSection {
    fn parse(data: &mut std::io::Cursor<&[u8]>) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let mut n = [0u8];
        data.read_exact(&mut n)?;
        if !matches!(n, [10]) {
            Err(SectionError::InvalidHeader(10, n[0]))?;
        }

        let size = u32::parse(data)?;
        let code = Vec::parse(data)?;
        Ok(Self { size, code })
    }
}
