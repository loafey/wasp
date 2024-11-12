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
        let stack_before = stack.clone();
        // I think this is correct
        let t: Vec<Locals> = match Vec::parse(data, stack) {
            Err(_) => {
                *stack = stack_before;
                Vec::new()
            }
            Ok(v) => v,
        };
        let e = Expr::parse(data, stack)?;
        Ok(Self { t, e })
    }
}
