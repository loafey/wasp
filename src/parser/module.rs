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
    types: Vec<TypeSection>,
    // customsec
    imports: Vec<ImportSection>,
    // customsec
    funcs: Vec<FunctionSection>,
    exports: Vec<ExportSection>,
    // customsec
    tables: Vec<()>, //tablesec
    // customsec
    mems: Vec<()>, // memsec
    // customsec
    globals: Vec<()>, // globalsec
    // customsec
    start: Vec<()>, // startsec
    // customsec
    elems: Vec<()>, //elemsec
    // customsec
    data_count: Vec<()>, //datacountsec
    // customsec
    code: Vec<CodeSection>,
    // customsec
    datas: Vec<DataSection>,
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

        let mut functype = Vec::new();
        let mut import = Vec::new();
        let mut typeidx = Vec::new();
        let mut export = Vec::new();
        let mut code = Vec::new();
        let mut datasec = Vec::new();
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
                1 => functype.push(TypeSection::parse(data)?),
                2 => import.push(ImportSection::parse(data)?),
                3 => typeidx.push(FunctionSection::parse(data)?),
                4 => unimplemented!("\"table\" sections (4)"),
                5 => unimplemented!("\"memory\" sections (5)"),
                6 => unimplemented!("\"global\" sections (6)"),
                7 => export.push(ExportSection::parse(data)?),
                8 => unimplemented!("\"start\" sections (8)"),
                9 => unimplemented!("\"element\" sections (9)"),
                10 => code.push(CodeSection::parse(data)?),
                11 => datasec.push(DataSection::parse(data)?),
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
            start: Vec::new(),
            elems: Vec::new(),
            data_count: Vec::new(),
        })
    }
}
impl Pretty for Module {
    fn pretty_indent(&self, indent: usize) -> String {
        let mut s = format!("{}(module\n", self.get_indent(indent));
        // magic
        // version
        // functype
        // import
        // typeidx
        // table
        // mem
        // global
        // export
        // start
        // elem
        // m
        // code
        // data
        s += ")";
        s
    }
}
