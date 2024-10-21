use super::{Global, Parsable};

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
    fn parse_inner(data: &mut std::io::Cursor<&[u8]>) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let size = u32::parse_inner(data)?;
        let globals = Vec::parse_inner(data)?;
        Ok(Self { size, globals })
    }
}
