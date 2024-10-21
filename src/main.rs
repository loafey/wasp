use hex::Hex;
use parser::{Module, Parsable};
use std::{io::Cursor, mem::MaybeUninit};
mod hex;
mod parser;

fn alloc<const N: usize>() -> Hex<N> {
    #[allow(clippy::uninit_assumed_init)]
    #[allow(invalid_value)]
    unsafe {
        MaybeUninit::uninit().assume_init()
    }
}

fn main() {
    let bin: &[u8] = include_bytes!("../examples/hello_world.wasm");
    let mut cursor = Cursor::new(bin);
    println!("{:#?}", Module::parse(&mut cursor).unwrap())
}
