use super::{Import, Parsable, Pretty};

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
        let size = u32::parse(data)?;
        let imports = Vec::parse(data)?;
        Ok(Self { imports, size })
    }
}
impl Pretty for ImportSection {
    fn pretty_indent(&self, indent: usize) -> String {
        format!(
            "{i}(import // b_size={}\n{i}{}\n{i})\n",
            self.size,
            self.imports.pretty_indent(indent),
            i = self.get_indent(indent),
        )
    }
}
