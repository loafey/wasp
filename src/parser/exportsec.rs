use super::{Export, Parsable, Pretty};
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
    fn parse_inner(data: &mut std::io::Cursor<&[u8]>) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let size = u32::parse_inner(data)?;
        let exports = Set::from_iter(Vec::parse_inner(data)?);
        Ok(ExportSection { size, exports })
    }
}
impl Pretty for ExportSection {
    fn pretty_indent(&self, indent: usize) -> String {
        format!(
            "{i}(exports // b_size={}\n{i}{}\n{i})\n",
            self.size,
            self.exports.pretty_indent(indent),
            i = self.get_indent(indent),
        )
    }
}
