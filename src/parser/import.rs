use crate::parser::Name;

use super::{ImportDesc, Parsable};

#[derive(Debug)]
#[allow(unused)]
pub struct Import {
    module: Name,
    name: Name,
    desc: ImportDesc,
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
