use super::{error::ParseError, Data, Parsable};

#[derive(Debug, Default)]
#[allow(unused)]
pub struct DataSection {
    pub size: u32,
    pub data: Vec<Data>,
}
impl DataSection {
    pub fn concat(&mut self, mut other: Self) {
        self.size += other.size;
        self.data.append(&mut other.data);
    }
}
impl Parsable for DataSection {
    fn parse_inner(
        data: &mut std::io::Cursor<&[u8]>,
        stack: super::DebugStack,
    ) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let size = u32::parse(data, stack)?;
        let expected = data.position() + size as u64;
        let data_ = Vec::parse(data, stack)?;
        if data.position() != expected {
            return Err(ParseError::SectionSizeMismatch);
        }
        Ok(Self { size, data: data_ })
    }
}
