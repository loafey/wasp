use super::{Data, Parsable};

#[derive(Debug)]
#[allow(unused)]
pub struct DataSection {
    pub size: u32,
    pub data: Vec<Data>,
}
impl Parsable for DataSection {
    fn parse(data: &mut std::io::Cursor<&[u8]>) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let size = u32::parse(data)?;
        let data = Vec::parse(data)?;
        Ok(Self { size, data })
    }
}
