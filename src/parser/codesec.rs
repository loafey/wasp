use super::{Code, Parsable};

#[derive(Debug, Default)]
#[allow(unused)]
pub struct CodeSection {
    pub size: u32,
    pub code: Vec<Code>,
}
impl CodeSection {
    pub fn concat(&mut self, mut other: Self) {
        self.size += other.size;
        self.code.append(&mut other.code);
    }
}

impl Parsable for CodeSection {
    fn parse_inner(
        data: &mut std::io::Cursor<&[u8]>,
        stack: super::DebugStack,
    ) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let size = u32::parse(data, stack)?;
        let code = Vec::parse(data, stack)?;
        Ok(Self { size, code })
    }
}
