use super::{Parsable, Table};

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
    fn parse_inner(
        data: &mut std::io::Cursor<&[u8]>,
        stack: super::DebugStack,
    ) -> Result<Self, super::error::ParseError> {
        let size = u32::parse(data, stack)?;
        let tables = Vec::parse(data, stack)?;
        Ok(Self { size, tables })
    }
}
