use crate::parser::Name;

use super::{ImportDesc, Parsable, Pretty};

#[derive(Debug)]
#[allow(unused)]
pub struct Import {
    pub module: Name,
    pub name: Name,
    pub desc: ImportDesc,
}
impl Parsable for Import {
    fn parse_inner(
        data: &mut std::io::Cursor<&[u8]>,
        stack: super::DebugStack,
    ) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let module = Name::parse(data, stack)?;
        let name = Name::parse(data, stack)?;
        let desc = ImportDesc::parse(data, stack)?;
        Ok(Self { module, name, desc })
    }
}
impl Pretty for Import {
    fn pretty_indent(&self, _: usize) -> String {
        format!(
            "(import {} {} {})",
            self.module.pretty(),
            self.name.pretty(),
            self.desc.pretty()
        )
    }
}
