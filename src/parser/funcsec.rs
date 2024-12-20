use super::{Parsable, TypeIdX};

#[derive(Debug, Default)]
#[allow(unused)]
pub struct FunctionSection {
    pub size: u32,
    pub functions: Vec<TypeIdX>,
}
impl FunctionSection {
    pub fn concat(&mut self, mut other: Self) {
        self.size += other.size;
        self.functions.append(&mut other.functions);
    }
}
impl Parsable for FunctionSection {
    fn parse_inner(
        data: &mut std::io::Cursor<&[u8]>,
        stack: super::DebugStack,
    ) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let size = u32::parse(data, stack)?;
        let functions = Vec::parse(data, stack)?;
        Ok(Self { size, functions })
    }
}
