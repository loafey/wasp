use super::{Parsable, Pretty, ResultType};
use crate::{hex::Hex, parser::error::ParseError};
use std::io::Read;

#[derive(Debug)]
pub struct FuncType {
    #[allow(unused)]
    pub input: ResultType,
    #[allow(unused)]
    pub output: ResultType,
}
impl Parsable for FuncType {
    fn parse_inner(data: &mut std::io::Cursor<&[u8]>) -> Result<Self, ParseError>
    where
        Self: std::marker::Sized,
    {
        let mut b = [0];
        data.read_exact(&mut b)?;
        if !matches!(b, [0x60]) {
            Err(ParseError::InvalidFuncType(Hex(b)))?;
        }

        let input = ResultType::parse_inner(data)?;
        let output = ResultType::parse_inner(data)?;
        Ok(Self { input, output })
    }
}
impl Pretty for FuncType {
    fn pretty_indent(&self, _: usize) -> String {
        format!("{} -> {}", self.input.pretty(), self.output.pretty())
    }
}
