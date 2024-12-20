use super::{error::ParseError, Global, Parsable};

#[derive(Debug, Default)]
pub struct GlobalSection {
    pub size: u32,
    pub globals: Vec<Global>,
}
impl GlobalSection {
    pub fn concat(&mut self, mut other: Self) {
        self.size += other.size;
        self.globals.append(&mut other.globals);
    }
}
impl Parsable for GlobalSection {
    fn parse_inner(
        data: &mut std::io::Cursor<&[u8]>,
        stack: super::DebugStack,
    ) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let size = u32::parse(data, stack)?;
        let expected = data.position() + size as u64;
        let globals = Vec::parse(data, stack)?;
        if data.position() != expected {
            return Err(ParseError::SectionSizeMismatch(expected, data.position()));
        }
        Ok(Self { size, globals })
    }
}
