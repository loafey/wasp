use super::{Parsable, ResultType};
use crate::{hex::Hex, parser::error::ParseError};
use std::io::Read;

#[derive(Debug, Clone)]
pub struct FuncType {
    #[allow(unused)]
    pub input: ResultType,
    #[allow(unused)]
    pub output: ResultType,
}
impl Parsable for FuncType {
    fn parse_inner(
        data: &mut std::io::Cursor<&[u8]>,
        stack: super::DebugStack,
    ) -> Result<Self, ParseError>
    where
        Self: std::marker::Sized,
    {
        let mut b = [0];
        data.read_exact(&mut b)?;
        if !matches!(b, [0x60]) {
            Err(ParseError::InvalidFuncType(Hex(b)))?;
        }

        let input = ResultType::parse(data, stack)?;
        let output = ResultType::parse(data, stack)?;
        Ok(Self { input, output })
    }
}
