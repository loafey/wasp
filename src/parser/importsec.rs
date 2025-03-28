use super::{error::ParseError, Import, Parsable};

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
    fn parse_inner(
        data: &mut std::io::Cursor<&[u8]>,
        stack: super::DebugStack,
    ) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let size = u32::parse(data, stack)?;
        let expected = data.position() + size as u64;
        let imports = Vec::parse(data, stack)?;
        if data.position() != expected {
            return Err(ParseError::SectionSizeMismatch(expected, data.position()));
        }
        Ok(Self { imports, size })
    }
}
