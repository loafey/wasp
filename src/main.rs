#![feature(const_type_name)]
#![forbid(clippy::unwrap_used)]
#![deny(clippy::print_stdout)]
#![deny(clippy::print_stderr)]
use hex::Hex;
use parser::MemArg;
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
        let memory = unsafe { runtime.modules["_$_main_$_"].as_ws() }
            .memory
            .clone();
        runtime.modules.insert(
            "wasi_snapshot_preview1".to_string(),
            Import::IO {
                functions: {
                    let map: Vec<(&'static str, runtime::Function)> = vec![
                        ("args_sizes_get", &|_, _| Ok(vec![Value::I32(0)])),
                        #[allow(clippy::print_stdout)]
                        ("fd_write", &|locals, memory| {
                            let Some(Value::I32(fd)) = locals.get(&0) else {
                                panic!()
                            };
                            let Some(Value::I32(iovs_base)) = locals.get(&1) else {
                                panic!()
                            };
                            let Some(Value::I32(iovs_length)) = locals.get(&2) else {
                                panic!()
                            };
                            let Some(Value::I32(bytes_written)) = locals.get(&2) else {
                                panic!()
                            };
                            // println!("fd: {fd}, base: {iovs_base}, len: {iovs_length}");
                            let bytes_written = *bytes_written as usize;
                            let mut written = 0i32;
                            for i in 0..*iovs_length as usize {
                                let ptr = *iovs_base as usize + i * 8;
                                let buf = memory.get::<u32>(ptr, MemArg::default())? as usize;
                                let buf_len =
                                    memory.get::<u32>(ptr + 4, MemArg::default())? as usize;

                                let mut b = Vec::new();
                                for i in 0..buf_len {
                                    b.push(memory.get::<u8>(buf + i, MemArg::default())?);
                                    written += 1;
                                }
                                // print!("{b:?}");
                                match fd {
                                    1 => print!("{}", String::from_utf8_lossy(&b)),
                                    _ => panic!(),
                                }
                            }
                            memory.set(bytes_written, MemArg::default(), written)?;
                            Ok(vec![Value::I32(0)])
                        }),
                    ];

                    let mut res = HashMap::new();
                    for (k, v) in map {
                        res.insert(k, v);
                    }
                    res
                },
                globals: HashMap::new(),
                memory,
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
