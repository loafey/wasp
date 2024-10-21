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
    fn parse(data: &mut std::io::Cursor<&[u8]>) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let size = u32::parse(data)?;
        let mems = Vec::parse(data)?;
        Ok(Self { size, mems })
    }
}
