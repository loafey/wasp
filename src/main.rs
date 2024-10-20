use std::{
    fmt::Debug,
    io::{self, Cursor, Read},
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
};

#[derive(Debug)]
enum ParseError {
    InvalidModule(ModuleError),
    NotImplemented(&'static str),
    IOError(io::Error),
}
impl From<io::Error> for ParseError {
    fn from(value: io::Error) -> Self {
        Self::IOError(value)
    }
}
impl From<ModuleError> for ParseError {
    fn from(value: ModuleError) -> Self {
        Self::InvalidModule(value)
    }
}

#[derive(Clone, Copy)]
struct Hex<const N: usize>(pub [u8; N]);
impl<const N: usize> Debug for Hex<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<")?;
        for (i, b) in self.0.iter().enumerate() {
            if i + 1 != self.0.len() {
                write!(f, "{b:02x} ")?;
            } else {
                write!(f, "{b:02x}")?;
            }
        }
        write!(f, ">")?;
        Ok(())
    }
}
impl<const N: usize> Deref for Hex<N> {
    type Target = [u8; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<const N: usize> DerefMut for Hex<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug)]
enum ModuleError {
    InvalidHeader(Hex<4>),
    InvalidVersion(Hex<4>),
}

fn alloc<const N: usize>() -> Hex<N> {
    #[allow(clippy::uninit_assumed_init)]
    #[allow(invalid_value)]
    unsafe {
        MaybeUninit::uninit().assume_init()
    }
}

/// https://webassembly.github.io/spec/core/binary/modules.html#binary-module
#[derive(Debug)]
struct Module {
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
fn module(data: &mut Cursor<&[u8]>) -> Result<Module, ParseError> {
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

fn main() {
    let bin: &[u8] = include_bytes!("../examples/hello_world.wasm");
    let mut cursor = Cursor::new(bin);

    println!("{:#?}", module(&mut cursor))
}
