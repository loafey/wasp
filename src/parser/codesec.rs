use super::{Code, Parsable};

#[derive(Debug)]
#[allow(unused)]
pub struct CodeSection {
    pub size: u32,
    pub code: Vec<Code>,
}

impl Parsable for CodeSection {
    fn parse(data: &mut std::io::Cursor<&[u8]>) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let size = u32::parse(data)?;
        let code = Vec::parse(data)?;
        Ok(Self { size, code })
    }
}
