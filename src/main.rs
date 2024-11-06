#![feature(const_type_name)]
#![forbid(clippy::unwrap_used)]
#![deny(clippy::print_stdout)]
#![deny(clippy::print_stderr)]
use gui::gui;
use hex::Hex;
use parser::{Module, Parsable};
use runtime::{Runtime, RuntimeError};
use std::{
    env::args,
    fs::File,
    io::{Cursor, Read},
    mem::MaybeUninit,
    path::PathBuf,
};
mod gui;
mod hex;
mod parser;
mod runtime;
mod testsuite;

#[allow(unused_imports)]
#[macro_use]
extern crate log;

fn alloc<const N: usize>() -> Hex<N> {
    #[allow(clippy::uninit_assumed_init)]
    #[allow(invalid_value)]
    unsafe {
        MaybeUninit::uninit().assume_init()
    }
}

fn create_runtime(path: PathBuf) -> Result<Runtime, RuntimeError> {
    let mut buf = Vec::new();

    let mut f = File::open(&path).expect("Failed to open file");
    f.read_to_end(&mut buf).expect("Failed to read file");

    let mut cursor = Cursor::new(&buf[..]);
    let mut stack = Vec::new();
    let module = match Module::parse(&mut cursor, &mut stack) {
        Ok(o) => o,
        Err(e) => {
            stack.reverse();
            return Err(RuntimeError::ParseError(format!(
                "File: {path:?}\n{e:?}, bin pos: {}, stack: {stack:#?}",
                cursor.position()
            )));
        }
    };
    Runtime::new(module)
}

fn main() {
    pretty_env_logger::init();
    let path = args()
        .skip(1)
        .find(|p| !p.starts_with("-"))
        .unwrap_or("examples/c_addition.wasm".to_string());

    if path.ends_with(".wast") {
        testsuite::test(path);
    } else if args().any(|s| s == "--gui") {
        gui(path)
    } else {
        let mut runtime = create_runtime(path.into()).expect("Failed to load runtime");
        loop {
            if let Err(e) = runtime.step() {
                match e {
                    RuntimeError::Exit(x) => std::process::exit(x),
                    _ => {
                        error!("{e:?}");
                        std::process::exit(1)
                    }
                }
            }
        }
    }
}
