use std::{
    io::{self, Cursor, Read},
    mem::MaybeUninit,
};

#[derive(Debug)]
enum ParseError {
    InvalidModule,
    NotImplemented(&'static str),
    IOError(io::Error),
}
impl From<io::Error> for ParseError {
    fn from(value: io::Error) -> Self {
        Self::IOError(value)
    }
}

#[derive(Debug)]
struct Module {}

fn module(data: &mut Cursor<&[u8]>) -> Result<Module, ParseError> {
    #[allow(clippy::uninit_assumed_init)]
    let mut bytes: [u8; 4] = unsafe { MaybeUninit::uninit().assume_init() };
    data.read_exact(&mut bytes)?;
    for b in bytes {
        print!("{b:0x} ");
    }
    println!();
    todo!()
}

fn main() {
    let bin: &[u8] = include_bytes!("../examples/hello_world.wasm");
    let mut cursor = Cursor::new(bin);

    println!("{:?}", module(&mut cursor))
}
