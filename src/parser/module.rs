use super::{
    error::{ModuleError, ParseError, SectionError},
    CustomSection, ElementSection, GlobalSection, ImportSection, MemorySection, Parsable,
    TableSection, TypeSection,
};
use crate::{
    alloc,
    hex::Hex,
    parser::{CodeSection, DataSection, ExportSection, FunctionSection},
};
use std::{
    collections::HashSet,
    io::{Cursor, ErrorKind, Read},
};

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

        let mut types = TypeSection::default();
        let mut import = ImportSection::default();
        let mut functions = FunctionSection::default();
        let mut export = ExportSection::default();
        let mut code = CodeSection::default();
        let mut datasec = DataSection::default();
        let mut tables = TableSection::default();
        let mut mems = MemorySection::default();
        let mut globals = GlobalSection::default();
        let mut elements = ElementSection::default();
        let customs = CustomSection::default();

        let mut section_header = [0];

        let mut found_data_count = None;
        let mut found_data_size = None;

        let mut start = None;
        let mut parsed_sections = HashSet::new();
        let mut last_section = 0;

        loop {
            if let Err(e) = data.read_exact(&mut section_header) {
                match e.kind() {
                    ErrorKind::UnexpectedEof => break,
                    _ => Err(e)?,
                }
            }
            if section_header[0] != 12 && parsed_sections.contains(&section_header[0]) {
                return Err(ParseError::DuplicateSection(section_header[0] as u32));
            }
            parsed_sections.insert(section_header[0]);
            if last_section > section_header[0] && last_section != 12 {
                return Err(ParseError::OutOfOrderSection);
            }
            last_section = section_header[0];
            match section_header[0] {
                0 => drop(CustomSection::parse(data, stack)?), // customs.concat(CustomSection::parse(data, stack)?),
                1 => types.concat(TypeSection::parse(data, stack)?),
                2 => import.concat(ImportSection::parse(data, stack)?),
                3 => functions.concat(FunctionSection::parse(data, stack)?),
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
                11 => {
                    let parse = DataSection::parse(data, stack)?;
                    found_data_size = Some(parse.data.len());
                    datasec.concat(parse);
                }
                12 => {
                    let _size = u32::parse(data, stack)? as usize;
                    let size = u32::parse(data, stack)? as usize;
                    found_data_count = Some(size);
                }
                _ => Err(SectionError::UnknownHeader(Hex(section_header)))?,
            }
        }
        if let Some((found_data_count, found_data_size)) = found_data_count.zip(found_data_size) {
            if found_data_count != found_data_size {
                return Err(ParseError::InvalidDataCount);
            }
        } else if found_data_size.is_none() {
            if let Some(found_data_count) = found_data_count {
                if found_data_count != 0 {
                    return Err(ParseError::InvalidDataCount);
                }
            }
        }

        if functions.functions.len() != code.code.len() {
            return Err(ParseError::InconsistentFunctionAndCodeSectionLength);
        }
        if found_data_count.is_none()
            && code.code.iter().any(|c| {
                c.code.e.instrs.iter().any(|i| {
                    matches!(
                        i,
                        super::Instr::xfc_8_memory_init(_, _) | super::Instr::xfc_9_data_drop(_)
                    )
                })
            })
        {
            return Err(ParseError::NoDataCountSection);
        }

        Ok(Module {
            magic,
            version,
            types,
            imports: import,
            funcs: functions,
            exports: export,
            code,
            datas: datasec,
            tables,
            mems,
            globals,
            start,
            elems: elements,
            customs,
        })
    }
}
