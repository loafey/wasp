use super::{error::ParseError, Parsable, Pretty};
use crate::parser::FuncType;
use std::io::Cursor;

#[derive(Debug, Default)]
#[allow(unused)]
pub struct TypeSection {
    pub size: u32,
    pub function_types: Vec<FuncType>,
}
impl TypeSection {
    pub fn concat(&mut self, mut other: Self) {
        self.size += other.size;
        self.function_types.append(&mut other.function_types);
    }
}
impl Parsable for TypeSection {
    fn parse_inner(data: &mut Cursor<&[u8]>) -> Result<TypeSection, ParseError> {
        let size = u32::parse_inner(data)?;
        let function_types: Vec<FuncType> = Vec::parse_inner(data)?;
        Ok(Self {
            size,
            function_types,
        })
    }
}
impl Pretty for TypeSection {
    fn pretty_indent(&self, indent: usize) -> String {
        format!(
            "{i}(types // b_size={}\n{i}{}\n{i})\n",
            self.size,
            self.function_types.pretty_indent(indent),
            i = self.get_indent(indent),
        )
    }
}
