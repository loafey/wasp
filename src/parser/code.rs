use super::{Func, Parsable};

#[derive(Debug)]
#[allow(unused)]
pub struct Code {
    pub size: u32,
    pub code: Func,
}
impl Parsable for Code {
    fn parse(data: &mut std::io::Cursor<&[u8]>) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let size = u32::parse(data)?;
        let code = Func::parse(data)?;
        Ok(Self { size, code })
    }
}
