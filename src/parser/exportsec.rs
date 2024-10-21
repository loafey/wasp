use super::{Export, Parsable, Pretty};

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
        let size = u32::parse(data)?;
        let exports = Vec::parse(data)?;
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
