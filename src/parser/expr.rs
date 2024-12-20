use super::{error::ParseError, Instr, Parsable};

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct Expr {
    pub instrs: Vec<Instr>,
}
impl Parsable for Expr {
    fn parse_inner(
        data: &mut std::io::Cursor<&[u8]>,
        stack: super::DebugStack,
    ) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let mut instrs = Vec::new();
        loop {
            match Instr::parse(data, stack) {
                Ok(i) => instrs.push(i),
                Err(ParseError::EndOfInstructions) => {
                    stack.pop();
                    break;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(Self { instrs })
    }
}
