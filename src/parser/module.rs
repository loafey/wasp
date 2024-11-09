use super::{
    error::{ModuleError, ParseError, SectionError},
    CustomSection, ElementSection, FuncIdx, GlobalSection, ImportSection, MemorySection, Parsable,
    TableSection, TypeSection,
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
    pub magic: Hex<4>,
    pub version: Hex<4>,
    pub types: TypeSection,
    pub imports: ImportSection,
    pub funcs: FunctionSection,
    pub exports: ExportSection,
    pub tables: TableSection,   //tablesec
    pub mems: MemorySection,    // memsec
    pub globals: GlobalSection, // globalsec
    pub start: Option<u32>,     // startsec
    pub elems: ElementSection,  //elemsec
    pub data_count: Vec<()>,    //datacountsec
    pub code: CodeSection,
    pub datas: DataSection,
    pub customs: CustomSection,
}
impl Parsable for Module {
    fn parse_inner(
        data: &mut Cursor<&[u8]>,
        stack: super::DebugStack,
    ) -> Result<Module, ParseError> {
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
        let mut tables = TableSection::default();
        let mut mems = MemorySection::default();
        let mut globals = GlobalSection::default();
        let mut elements = ElementSection::default();
        let customs = CustomSection::default();

        let mut section_header = [0];
        let mut found_data_count = false;
        let mut start = None;
        loop {
            if let Err(e) = data.read_exact(&mut section_header) {
                match e.kind() {
                    ErrorKind::UnexpectedEof => break,
                    _ => Err(e)?,
                }
            }
            match section_header[0] {
                0 => drop(CustomSection::parse(data, stack)?), // customs.concat(CustomSection::parse(data, stack)?),
                1 => functype.concat(TypeSection::parse(data, stack)?),
                2 => import.concat(ImportSection::parse(data, stack)?),
                3 => typeidx.concat(FunctionSection::parse(data, stack)?),
                4 => tables.concat(TableSection::parse(data, stack)?),
                5 => mems.concat(MemorySection::parse(data, stack)?),
                6 => globals.concat(GlobalSection::parse(data, stack)?),
                7 => export.concat(ExportSection::parse(data, stack)?),
                8 => {
                    let _size = u32::parse(data, stack)? as usize;
                    start = Some(u32::parse(data, stack)?)
                }
                9 => elements.concat(ElementSection::parse(data, stack)?),
                10 => code.concat(CodeSection::parse(data, stack)?),
                11 => datasec.concat(DataSection::parse(data, stack)?),
                12 => {
                    let _size = u32::parse(data, stack)? as usize;
                    found_data_count = true;
                    if (u32::parse(data, stack)? as usize) != datasec.data.len() {
                        return Err(ParseError::InvalidDataCount);
                    }
                }
                _ => Err(SectionError::UnknownHeader(Hex(section_header)))?,
            }
        }
        if typeidx.functions.len() != code.code.len() {
            return Err(ParseError::InconsistentFunctionAndCodeSectionLength);
        }
        if !found_data_count
            && code.code.iter().any(|c| {
                c.code.e.instrs.iter().any(|i| {
                    matches!(
                        i,
                        super::Instr::xfc_8_memory_init(_) | super::Instr::xfc_9_data_drop(_)
                    )
                })
            })
        {
            return Err(ParseError::NoDataCountSection);
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
            tables,
            mems,
            globals,
            start,
            elems: elements,
            data_count: Vec::new(),
            customs,
        })
    }
}
