use super::{
    error::{ModuleError, ParseError, SectionError},
    ImportSection, Parsable, TypeSection,
};
use crate::{
    alloc,
    hex::Hex,
    parser::{CodeSection, DataSection, ExportSection, FunctionSection},
};
use std::io::{Cursor, Read};

/// https://webassembly.github.io/spec/core/binary/modules.html#binary-module
#[derive(Debug)]
#[allow(unused)]
pub struct Module {
    magic: Hex<4>,
    version: Hex<4>,
    functype: Vec<TypeSection>,
    // customsec
    import: Vec<ImportSection>,
    // customsec
    typeidx: Vec<FunctionSection>,
    export: Vec<ExportSection>,
    // customsec
    // table: tablesec
    // customsec
    // mem: memsec
    // customsec
    // global: globalsec
    // customsec
    // export:exportsec
    // customsec
    // start: startsec
    // customsec
    // elem: elemsec
    // customsec
    // m: datacountsec
    // customsec
    code: Vec<CodeSection>,
    // customsec
    data: Vec<DataSection>,
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
        loop {
            let mut section_header = [0];
            data.read_exact(&mut section_header)?;
            // let functype = TypeSection::parse(data)?;
            // let import = ImportSection::parse(data)?;
            // let function = FunctionSection::parse(data)?;
            // let export = ExportSection::parse(data)?;
            // let code = CodeSection::parse(data)?;
            // let datasec = DataSection::parse(data)?;
            match section_header[0] {
                0 => unimplemented!("\"custom\" sections (0)"),
                1 => unimplemented!("\"type\" sections (1)"),
                2 => unimplemented!("\"import\" sections (2)"),
                3 => unimplemented!("\"function\" sections (3)"),
                4 => unimplemented!("\"table\" sections (4)"),
                5 => unimplemented!("\"memory\" sections (5)"),
                6 => unimplemented!("\"global\" sections (6)"),
                7 => unimplemented!("\"export\" sections (7)"),
                8 => unimplemented!("\"start\" sections (8)"),
                9 => unimplemented!("\"element\" sections (9)"),
                10 => unimplemented!("\"code\" sections (10)"),
                11 => unimplemented!("\"data\" sections (11)"),
                12 => unimplemented!("\"data count\" sections (12)"),
                _ => Err(SectionError::UnknownHeader(Hex(section_header)))?,
            }
        }

        Ok(Module {
            magic,
            version,
            functype,
            import,
            typeidx,
            export,
            code,
            data: datasec,
        })
    }
}
