use super::{ExportDesc, Name, Parsable, Pretty};

#[derive(Debug)]
#[allow(unused)]
pub struct Export {
    pub nm: Name,
    pub d: ExportDesc,
}
impl PartialEq for Export {
    fn eq(&self, other: &Self) -> bool {
        self.d.eq(&other.d)
    }
}
impl Eq for Export {}
impl PartialOrd for Export {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Export {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.d.cmp(&other.d)
    }
}
impl Parsable for Export {
    fn parse_inner(
        data: &mut std::io::Cursor<&[u8]>,
        stack: super::DebugStack,
    ) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let nm = Name::parse(data, stack)?;
        let d = ExportDesc::parse(data, stack)?;
        Ok(Self { nm, d })
    }
}
impl Pretty for Export {
    fn pretty_indent(&self, _: usize) -> String {
        format!("(func {} {})", self.nm.pretty(), self.d.pretty())
    }
}
