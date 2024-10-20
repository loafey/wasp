use super::{
    error::{ModuleError, ParseError},
    ImportSection, Parsable, TypeSection,
};
use crate::{
    alloc,
    hex::Hex,
    parser::{CodeSection, ExportSection, FunctionSection},
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
    // data: datasec
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

        let functype = TypeSection::parse(data)?;
        let import = ImportSection::parse(data)?;
        let function = FunctionSection::parse(data)?;
        let export = ExportSection::parse(data)?;
        let code = CodeSection::parse(data)?;

        let mut b = [0];
        data.read_exact(&mut b)?;
        println!("{b:?}");

        Ok(Module {
            magic,
            version,
            functype: vec![functype],
            import: vec![import],
            typeidx: vec![function],
            export: vec![export],
            code: vec![code],
        })
    }
}
