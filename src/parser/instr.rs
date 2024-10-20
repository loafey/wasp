use std::io::Read;

use super::Parsable;

#[derive(Debug)]
pub enum Instr {}
impl Parsable for Instr {
    fn parse(data: &mut std::io::Cursor<&[u8]>) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let mut typ = [0];
        data.read_exact(&mut typ)?;
        println!("{typ:?}");
        todo!()
    }
}
