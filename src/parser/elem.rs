use super::{error::ParseError, Expr, FuncIdx, Parsable, RefTyp, TableIdX};
use Elem::*;
pub type ElemKind = RefTyp;

#[derive(Debug)]
#[allow(unused)]
pub enum Elem {
    E0(Expr, Vec<FuncIdx>),
    E1(ElemKind, Vec<FuncIdx>),
    E2(TableIdX, Expr, ElemKind, Vec<FuncIdx>),
    E3(ElemKind, Vec<FuncIdx>),
    E4(Expr, Vec<Expr>),
    E5(RefTyp, Vec<Expr>),
    E6(TableIdX, Expr, RefTyp, Vec<Expr>),
    E7(RefTyp, Vec<Expr>),
}
impl Parsable for Elem {
    fn parse_inner(
        data: &mut std::io::Cursor<&[u8]>,
        stack: super::DebugStack,
    ) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let header = u32::parse(data, stack)?;
        Ok(match header {
            0 => E0(Parsable::parse(data, stack)?, Parsable::parse(data, stack)?),
            1 => E1(Parsable::parse(data, stack)?, Parsable::parse(data, stack)?),
            2 => E2(
                Parsable::parse(data, stack)?,
                Parsable::parse(data, stack)?,
                Parsable::parse(data, stack)?,
                Parsable::parse(data, stack)?,
            ),
            3 => E3(Parsable::parse(data, stack)?, Parsable::parse(data, stack)?),
            4 => E4(Parsable::parse(data, stack)?, Parsable::parse(data, stack)?),
            5 => E5(Parsable::parse(data, stack)?, Parsable::parse(data, stack)?),
            6 => E6(
                Parsable::parse(data, stack)?,
                Parsable::parse(data, stack)?,
                Parsable::parse(data, stack)?,
                Parsable::parse(data, stack)?,
            ),
            7 => E7(Parsable::parse(data, stack)?, Parsable::parse(data, stack)?),
            _ => Err(ParseError::InvalidElem(header))?,
        })
    }
}
