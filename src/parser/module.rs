use super::error::{ModuleError, ParseError};
use crate::{alloc, hex::Hex};
use std::io::{Cursor, Read};

/// https://webassembly.github.io/spec/core/binary/modules.html#binary-module
#[derive(Debug)]
pub struct Module {
    magic: Hex<4>,
    version: Hex<4>,
    // customsec
    // functpye: typesec
    // customsec
    // import: importsec
    // customsec
    // typeidx: funcsec
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
    // code: codesec
    // customsec
    // data: datasec
    // customsec
}

/// https://webassembly.github.io/spec/core/binary/modules.html#binary-module
pub fn parse_module(data: &mut Cursor<&[u8]>) -> Result<Module, ParseError> {
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

    Ok(Module { magic, version })
}
