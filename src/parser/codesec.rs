use super::{Code, Parsable, Pretty};

#[derive(Debug, Default)]
#[allow(unused)]
pub struct CodeSection {
    pub size: u32,
    pub code: Vec<Code>,
}
impl CodeSection {
    pub fn concat(&mut self, mut other: Self) {
        self.size += other.size;
        self.code.append(&mut other.code);
    }
}

impl Parsable for CodeSection {
    fn parse_inner(data: &mut std::io::Cursor<&[u8]>) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let size = u32::parse_inner(data)?;
        let code = Vec::parse_inner(data)?;
        Ok(Self { size, code })
    }
}
impl Pretty for CodeSection {
    fn pretty_indent(&self, indent: usize) -> String {
        format!(
            "{i}(code // b_size={}\n{i}{}\n{i})\n",
            self.size,
            self.code.pretty_indent(indent),
            i = self.get_indent(indent),
        )
    }
}
