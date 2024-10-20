use super::{Export, Parsable};
use crate::parser::error::SectionError;
use std::io::Read;

#[derive(Debug)]
#[allow(unused)]
pub struct ExportSection {
    pub size: u32,
    pub exports: Vec<Export>,
}
impl Parsable for ExportSection {
    fn parse(data: &mut std::io::Cursor<&[u8]>) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let mut n = [0u8];
        data.read_exact(&mut n)?;
        if !matches!(n, [7]) {
            Err(SectionError::InvalidHeader(7, n[0]))?;
        }

        let size = u32::parse(data)?;
        let exports = Vec::parse(data)?;
        Ok(ExportSection { size, exports })
    }
}
