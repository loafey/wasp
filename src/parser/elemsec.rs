use super::{Elem, Parsable};

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
        let elems = Vec::parse(data, stack)?;
        Ok(Self { size, elems })
    }
}
