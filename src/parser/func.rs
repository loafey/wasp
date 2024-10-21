use super::{Expr, Locals, Parsable, Pretty};

#[derive(Debug)]
#[allow(unused)]
pub struct Func {
    pub t: Vec<Locals>,
    pub e: Expr,
}
impl Parsable for Func {
    fn parse(data: &mut std::io::Cursor<&[u8]>) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let t = Vec::parse(data)?;
        let e = Expr::parse(data)?;
        Ok(Self { t, e })
    }
}
impl Pretty for Func {
    fn pretty_indent(&self, _: usize) -> String {
        todo!()
    }
}
