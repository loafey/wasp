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
    fn parse(data: &mut std::io::Cursor<&[u8]>) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let size = u32::parse(data)?;
        let globals = Vec::parse(data)?;
        Ok(Self { size, globals })
    }
}
