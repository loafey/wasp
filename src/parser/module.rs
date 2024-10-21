use super::{
    error::{ModuleError, ParseError, SectionError},
    ImportSection, Parsable, Pretty, TypeSection,
};
use crate::{
    alloc,
    hex::Hex,
    parser::{CodeSection, DataSection, ExportSection, FunctionSection},
};
use std::io::{Cursor, ErrorKind, Read};

/// https://webassembly.github.io/spec/core/binary/modules.html#binary-module
#[derive(Debug)]
#[allow(unused)]
pub struct Module {
    magic: Hex<4>,
    version: Hex<4>,
    types: TypeSection,
    // customsec
    imports: ImportSection,
    // customsec
    funcs: FunctionSection,
    exports: ExportSection,
    // customsec
    tables: Vec<()>, //tablesec
    // customsec
    mems: Vec<()>, // memsec
    // customsec
    globals: Vec<()>, // globalsec
    // customsec
    start: Option<()>, // startsec
    // customsec
    elems: Vec<()>, //elemsec
    // customsec
    data_count: Vec<()>, //datacountsec
    // customsec
    code: CodeSection,
    // customsec
    datas: DataSection,
    // customsec
}
impl Parsable for Module {
    fn parse(data: &mut Cursor<&[u8]>) -> Result<Module, ParseError> {
        // parse magic
        let mut magic = alloc::<4>();
        data.read_exact(&mut *magic)?;
        if !matches!(&*magic, b"\0asm") {
            Err(ModuleError::InvalidHeader(magic))?;
        }

        // parse version
        let mut version = alloc::<4>();
        data.read_exact(&mut *version)?;
        if !matches!(&*version, [0x01, 0x00, 0x00, 0x00]) {
            Err(ModuleError::InvalidVersion(version))?;
        }

        let mut functype = TypeSection::default();
        let mut import = ImportSection::default();
        let mut typeidx = FunctionSection::default();
        let mut export = ExportSection::default();
        let mut code = CodeSection::default();
        let mut datasec = DataSection::default();
        let mut section_header = [0];
        loop {
            if let Err(e) = data.read_exact(&mut section_header) {
                match e.kind() {
                    ErrorKind::UnexpectedEof => break,
                    _ => Err(e)?,
                }
            }
            match section_header[0] {
                0 => unimplemented!("\"custom\" sections (0)"),
                1 => functype.concat(TypeSection::parse(data)?),
                2 => import.concat(ImportSection::parse(data)?),
                3 => typeidx.concat(FunctionSection::parse(data)?),
                4 => unimplemented!("\"table\" sections (4)"),
                5 => unimplemented!("\"memory\" sections (5)"),
                6 => unimplemented!("\"global\" sections (6)"),
                7 => export.concat(ExportSection::parse(data)?),
                8 => unimplemented!("\"start\" sections (8)"),
                9 => unimplemented!("\"element\" sections (9)"),
                10 => code.concat(CodeSection::parse(data)?),
                11 => datasec.concat(DataSection::parse(data)?),
                12 => unimplemented!("\"data count\" sections (12)"),
                _ => Err(SectionError::UnknownHeader(Hex(section_header)))?,
            }
        }

        Ok(Module {
            magic,
            version,
            types: functype,
            imports: import,
            funcs: typeidx,
            exports: export,
            code,
            datas: datasec,
            tables: Vec::new(),
            mems: Vec::new(),
            globals: Vec::new(),
            start: None,
            elems: Vec::new(),
            data_count: Vec::new(),
        })
    }
}
impl Pretty for Module {
    fn pretty_indent(&self, indent: usize) -> String {
        let mut s = format!("{}(module\n", self.get_indent(indent));
        // magic
        s += &format!(
            "{}// {:?}\n",
            self.get_indent(indent + 1),
            self.magic.pretty()
        );
        // version
        s += &format!(
            "{}// {:?}\n",
            self.get_indent(indent + 1),
            self.version.pretty()
        );
        // functype
        s += &self.funcs.pretty_indent(indent);
        // import
        s += &self.imports.pretty_indent(indent);
        // typeidx
        s += &self.types.pretty_indent(indent);
        // table
        s += &self.tables.pretty_indent(indent);
        // mem
        s += &self.mems.pretty_indent(indent);
        // global
        s += &self.globals.pretty_indent(indent);
        // export
        s += &self.exports.pretty_indent(indent);
        // start
        s += &self.start.pretty_indent(indent);
        // elem
        s += &self.elems.pretty_indent(indent);
        // data_count
        s += &self.data_count.pretty_indent(indent);
        // code
        s += &self.code.pretty_indent(indent);
        // data
        s += &self.datas.pretty_indent(indent);
        s += ")";
        s
    }
}
