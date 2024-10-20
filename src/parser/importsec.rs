use std::io::Read;

use crate::parser::error::SectionError;

use super::{Import, Parsable};

#[derive(Debug)]
#[allow(unused)]
pub struct ImportSection {
    pub size: u32,
    pub imports: Vec<Import>,
}
impl Parsable for ImportSection {
    fn parse(data: &mut std::io::Cursor<&[u8]>) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let mut n = [0u8];
        data.read_exact(&mut n)?;
        if !matches!(n, [2]) {
            Err(SectionError::InvalidHeader(2, n[0]))?;
        }

        let size = u32::parse(data)?;

        let imports = Vec::parse(data)?;

        Ok(Self { imports, size })
    }
}
