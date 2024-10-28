use super::{Expr, Locals, Parsable};

#[derive(Debug)]
#[allow(unused)]
pub struct Func {
    pub t: Vec<Locals>,
    pub e: Expr,
}
impl Parsable for Func {
    fn parse_inner(
        data: &mut std::io::Cursor<&[u8]>,
        stack: super::DebugStack,
    ) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let t = Vec::parse(data, stack)?;
        let e = Expr::parse(data, stack)?;
        Ok(Self { t, e })
    }
}
