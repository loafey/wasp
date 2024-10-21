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
    fn parse(data: &mut std::io::Cursor<&[u8]>) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let module = Name::parse(data)?;
        let name = Name::parse(data)?;
        let desc = ImportDesc::parse(data)?;
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
