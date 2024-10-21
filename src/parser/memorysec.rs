use super::{Mem, Parsable};

#[derive(Debug, Default)]
pub struct MemorySection {
    pub size: u32,
    pub mems: Vec<Mem>,
}
impl MemorySection {
    pub fn concat(&mut self, mut other: Self) {
        self.size += other.size;
        self.mems.append(&mut other.mems);
    }
}
impl Parsable for MemorySection {
    fn parse_inner(
        data: &mut std::io::Cursor<&[u8]>,
        stack: super::DebugStack,
    ) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let size = u32::parse(data, stack)?;
        let mems = Vec::parse(data, stack)?;
        Ok(Self { size, mems })
    }
}
