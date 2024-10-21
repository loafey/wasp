use super::{Parsable, Pretty, TypeIdX};

#[derive(Debug)]
#[allow(unused)]
pub struct FunctionSection {
    pub size: u32,
    pub functions: Vec<TypeIdX>,
}
impl Parsable for FunctionSection {
    fn parse(data: &mut std::io::Cursor<&[u8]>) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let size = u32::parse(data)?;
        let functions = Vec::parse(data)?;
        Ok(Self { size, functions })
    }
}

impl Pretty for FunctionSection {
    fn pretty_indent(&self, indent: usize) -> String {
        format!(
            "{i}(func // b_size={}\n{i}{}\n{i})\n",
            self.size,
            self.functions.pretty_indent(indent),
            i = self.get_indent(indent),
        )
    }
}
