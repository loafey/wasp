#![feature(const_type_name)]
#![forbid(clippy::unwrap_used)]
#![deny(clippy::print_stdout)]
#![deny(clippy::print_stderr)]
use hex::Hex;
use runtime::{Import, Runtime, RuntimeError, Value};
use std::{collections::HashMap, env::args, mem::MaybeUninit};
mod hex;
mod parser;
mod ptr;
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

fn main() {
    pretty_env_logger::init();
    let path = args()
        .skip(1)
        .find(|p| !p.starts_with("-"))
        .unwrap_or("examples/c_addition.wasm".to_string());

    if path.ends_with(".wast") {
        testsuite::test(path);
    } else {
        let mut runtime = Runtime::new(path).expect("Failed to load runtime");
        runtime.modules.insert(
            "wasi_snapshot_preview1".to_string(),
            Import::IO {
                functions: {
                    let map: Vec<(&'static str, runtime::Function)> = vec![
                        ("args_sizes_get", &|_| Ok(vec![Value::I32(0)])),
                        #[allow(clippy::print_stdout)]
                        ("fd_write", &|locals| {
                            let ptr = locals.get(&0);
                            let array = locals.get(&1);
                            println!("{ptr:?} {array:?}");
                            Ok(vec![])
                        }),
                    ];

                    let mut res = HashMap::new();
                    for (k, v) in map {
                        res.insert(k, v);
                    }
                    res
                },
                globals: HashMap::new(),
            },
        );
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
