use super::{Import, Parsable};

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
