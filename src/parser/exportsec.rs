use super::{error::ParseError, Export, Parsable};
use std::collections::BTreeSet as Set;

#[derive(Debug, Default)]
#[allow(unused)]
pub struct ExportSection {
    pub size: u32,
    pub exports: Set<Export>,
}
impl ExportSection {
    pub fn concat(&mut self, mut other: Self) {
        self.size += other.size;
        self.exports.append(&mut other.exports);
    }
}
impl Parsable for ExportSection {
    fn parse_inner(
        data: &mut std::io::Cursor<&[u8]>,
        stack: super::DebugStack,
    ) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let size = u32::parse(data, stack)?;
        let expected = data.position() + size as u64;
        let exports = Set::from_iter(Vec::parse(data, stack)?);
        if data.position() != expected {
            return Err(ParseError::SectionSizeMismatch);
        }
        Ok(ExportSection { size, exports })
    }
}
