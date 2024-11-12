use super::{error::ParseError, Elem, Parsable};

#[derive(Debug, Default)]
pub struct ElementSection {
    pub size: u32,
    pub elems: Vec<Elem>,
}
impl ElementSection {
    pub fn concat(&mut self, mut other: Self) {
        self.size += other.size;
        self.elems.append(&mut other.elems);
    }
}
impl Parsable for ElementSection {
    fn parse_inner(
        data: &mut std::io::Cursor<&[u8]>,
        stack: super::DebugStack,
    ) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let size = u32::parse(data, stack)?;
        let expected = data.position() + size as u64;
        let elems = Vec::parse(data, stack)?;
        if data.position() != expected {
            return Err(ParseError::SectionSizeMismatch(expected, data.position()));
        }
        Ok(Self { size, elems })
    }
}
