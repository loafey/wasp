use super::{Func, Parsable, Pretty};

#[derive(Debug)]
#[allow(unused)]
pub struct Code {
    pub size: u32,
    pub code: Func,
}
impl Parsable for Code {
    fn parse_inner(
        data: &mut std::io::Cursor<&[u8]>,
        stack: super::DebugStack,
    ) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let size = u32::parse(data, stack)?;
        let code = Func::parse(data, stack)?;
        Ok(Self { size, code })
    }
}
impl Pretty for Code {
    fn pretty_indent(&self, indent: usize) -> String {
        format!("(code {})", self.code.pretty_indent(indent))
    }
}
