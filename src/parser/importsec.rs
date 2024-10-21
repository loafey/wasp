use super::{Import, Parsable, Pretty};

#[derive(Debug, Default)]
#[allow(unused)]
pub struct ImportSection {
    pub size: u32,
    pub imports: Vec<Import>,
}
impl ImportSection {
    pub fn concat(&mut self, mut other: Self) {
        self.size += other.size;
        self.imports.append(&mut other.imports);
    }
}
impl Parsable for ImportSection {
    fn parse_inner(data: &mut std::io::Cursor<&[u8]>) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let size = u32::parse_inner(data)?;
        let imports = Vec::parse_inner(data)?;
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
