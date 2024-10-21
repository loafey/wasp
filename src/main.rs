#![feature(const_type_name)]

use hex::Hex;
use parser::{FuncIdx, ImportDesc, Module, Parsable};
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

#[derive(Debug)]
enum StackValue {
    I32(i32),
}
struct Runtime {
    module: Module,
    stack: Vec<StackValue>,
    data: Vec<u8>,
}
impl Runtime {
    pub fn new(module: Module) -> Self {
        let mut data = Vec::new();
        for d in &module.datas.data {
            match d {
                parser::Data::Mem0(expr, vec) => data.append(&mut vec.clone()),
                parser::Data::MemB(vec) => todo!(),
                parser::Data::MemX(mem_id_x, expr, vec) => todo!(),
            }
        }
        Self {
            module,
            stack: Vec::new(),
            data,
        }
    }
    pub fn call_by_id(&mut self, id: u32) {
        let import_funcs: usize = self
            .module
            .imports
            .imports
            .iter()
            .map(|i| matches!(i.desc, ImportDesc::Func(_)) as usize)
            .sum();

        if (id as usize) < import_funcs {
            let func = &self.module.imports.imports[id as usize];
            match &*func.module.0 {
                m @ "console" => match &*func.name.0 {
                    "log" => {
                        let Some((StackValue::I32(y), StackValue::I32(x))) =
                            self.stack.pop().zip(self.stack.pop())
                        else {
                            unimplemented!()
                        };
                        let (x, y) = (x as usize, y as usize);

                        let str = String::from_utf8_lossy(&self.data[x..y]);
                        println!("{str}");
                    }
                    n => panic!("no function named {n:?} in module {m:?}"),
                },
                m => panic!("unknown module {m:?}"),
            }
        } else {
            let instrs = self.module.code.code[import_funcs - id as usize]
                .code
                .e
                .instrs
                .clone();
            #[allow(clippy::needless_range_loop)]
            for pc in 0..instrs.len() {
                match instrs[pc] {
                    parser::Instr::i32_const(i) => self.stack.push(StackValue::I32(i)),
                    parser::Instr::call(FuncIdx(id)) => self.call_by_id(id),
                }
            }
        }
    }
}

fn main() {
    let bin: &[u8] = include_bytes!("../examples/rust_addition.wasm");
    let mut cursor = Cursor::new(bin);
    let mut stack = Vec::new();
    let module = match Module::parse(&mut cursor, &mut stack) {
        Ok(o) => o,
        Err(e) => {
            stack.reverse();
            eprintln!("Error: {e:?}, stack: {stack:#?}");
            return;
        }
    };
    println!("{module:#?}");
    Runtime::new(module).call_by_id(1);
}
