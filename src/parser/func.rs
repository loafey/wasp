use super::{Expr, Locals, Parsable, Pretty};

#[derive(Debug)]
#[allow(unused)]
pub struct Func {
    pub t: Vec<Locals>,
    pub e: Expr,
}
impl Parsable for Func {
    fn parse_inner(data: &mut std::io::Cursor<&[u8]>) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let t = Vec::parse_inner(data)?;
        let e = Expr::parse_inner(data)?;
        Ok(Self { t, e })
    }
}
impl Pretty for Func {
    fn pretty_indent(&self, _: usize) -> String {
        todo!()
    }
}
