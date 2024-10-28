use super::{Export, Parsable};
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
        let exports = Set::from_iter(Vec::parse(data, stack)?);
        Ok(ExportSection { size, exports })
    }
}
