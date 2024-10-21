use super::{Parsable, Pretty, Table};

#[derive(Debug, Default)]
#[allow(unused)]
pub struct TableSection {
    pub size: u32,
    pub tables: Vec<Table>,
}
impl TableSection {
    pub fn concat(&mut self, mut other: Self) {
        self.size += other.size;
        self.tables.append(&mut other.tables);
    }
}
impl Parsable for TableSection {
    fn parse(data: &mut std::io::Cursor<&[u8]>) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let size = u32::parse(data)?;
        let tables = Vec::parse(data)?;
        Ok(Self { size, tables })
    }
}
impl Pretty for TableSection {
    fn pretty_indent(&self, _: usize) -> String {
        todo!()
    }
}
