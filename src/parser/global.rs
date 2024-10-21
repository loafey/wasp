use super::{Expr, GlobalType, Parsable};

#[derive(Debug)]
#[allow(unused)]
pub struct Global {
    gt: GlobalType,
    e: Expr,
}
impl Parsable for Global {
    fn parse(data: &mut std::io::Cursor<&[u8]>) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        Ok(Self {
            gt: GlobalType::parse(data)?,
            e: Expr::parse(data)?,
        })
    }
}
