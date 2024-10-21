use super::{error::ParseError, Instr, Parsable};

#[derive(Debug)]
#[allow(unused)]
pub struct Expr {
    pub instrs: Vec<Instr>,
}
impl Parsable for Expr {
    fn parse_inner(data: &mut std::io::Cursor<&[u8]>) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let mut instrs = Vec::new();
        loop {
            match Instr::parse_inner(data) {
                Ok(i) => instrs.push(i),
                Err(ParseError::EndOfInstructions) => break,
                Err(e) => Err(e)?,
            }
        }
        Ok(Self { instrs })
    }
}
