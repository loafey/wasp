use super::{Expr, GlobalType, Parsable};

#[derive(Debug)]
#[allow(unused)]
pub struct Global {
    pub gt: GlobalType,
    pub e: Expr,
}
impl Parsable for Global {
    fn parse_inner(
        data: &mut std::io::Cursor<&[u8]>,
        stack: super::DebugStack,
    ) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let gt = GlobalType::parse(data, stack)?;
        let e = Expr::parse(data, stack)?;
        Ok(Self { gt, e })
    }
}
