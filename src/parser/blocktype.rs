use super::{Parsable, ValType};

#[derive(Debug, Clone, Copy)]
#[allow(unused)]
pub enum BlockType {
    Eps,
    T(ValType),
    X(i64),
}
impl Parsable for BlockType {
    fn parse_inner(
        data: &mut std::io::Cursor<&[u8]>,
        stack: super::DebugStack,
    ) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let pos = data.position();
        let i = u8::parse(data, stack)?;
        if i == 0x40 {
            return Ok(BlockType::Eps);
        } else {
            data.set_position(pos);
        }

        todo!()
    }
}
