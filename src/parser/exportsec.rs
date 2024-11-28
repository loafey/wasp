use std::collections::HashMap;

use super::{error::ParseError, Export, ExportDesc, Parsable};

#[derive(Debug, Default)]
#[allow(unused)]
pub struct ExportSection {
    pub size: u32,
    pub exports: HashMap<String, ExportDesc>,
}
impl ExportSection {
    pub fn concat(&mut self, other: Self) {
        self.size += other.size;
        for (k, v) in other.exports {
            self.exports.insert(k, v);
        }
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
        let exports: Vec<Export> = Vec::parse(data, stack)?;
        let exports = HashMap::from_iter(exports.into_iter().map(|e| (e.nm.0, e.d)));
        if data.position() != expected {
            return Err(ParseError::SectionSizeMismatch(expected, data.position()));
        }
        Ok(ExportSection { size, exports })
    }
}
