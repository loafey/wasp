#![feature(const_type_name)]

use hex::Hex;
use parser::{ExportDesc, FuncIdx, GlobalIdX, ImportDesc, Instr::*, Module, Parsable, TypeIdX};
use std::{io::Cursor, mem::MaybeUninit};
mod hex;
mod parser;

#[macro_use]
extern crate log;

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
    stack: Vec<Vec<StackValue>>,
    data: Vec<u8>,
}
impl Runtime {
    pub fn new(module: Module) -> Self {
        let mut data = Vec::new();
        for d in &module.datas.data {
            match d {
                parser::Data::Mem0(_, vec) => data.append(&mut vec.clone()),
                parser::Data::MemB(_) => todo!(),
                parser::Data::MemX(_, _, _) => todo!(),
            }
        }
        Self {
            module,
            stack: vec![Vec::new()],
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
                        let st = self.stack.last_mut().unwrap();
                        let Some((StackValue::I32(y), StackValue::I32(x))) = st.pop().zip(st.pop())
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
            let index = if import_funcs == 0 {
                id as usize
            } else {
                import_funcs - id as usize
            };
            let instrs = self.module.code.code[index].code.e.instrs.clone();
            #[allow(clippy::needless_range_loop)]
            for pc in 0..instrs.len() {
                let instr = &instrs[pc];
                println!("---- Stack -----\n{:#?}", self.stack);
                match instr {
                    x10_i32_const(i) => {
                        let st = self.stack.last_mut().unwrap();
                        st.push(StackValue::I32(*i))
                    }
                    f @ x23_global_get(_) => {
                        error!("not implemented : {f:?}")
                    }
                    x6b_i32_sub => {
                        let st = self.stack.last_mut().unwrap();
                        let y = st.pop().unwrap();
                        let x = st.pop().unwrap();
                        match (x, y) {
                            (StackValue::I32(x), StackValue::I32(y)) => {
                                st.push(StackValue::I32(y - x))
                            }
                        }
                    }
                    x41_call(FuncIdx(id)) => self.call_by_id(*id),
                    f => {
                        unimplemented!("instruction not supported : {f:?}")
                    }
                }
            }
        }
    }
}

fn main() {
    pretty_env_logger::init();

    let bin: &[u8] = include_bytes!("../examples/rust_addition.wasm");
    let mut cursor = Cursor::new(bin);
    let mut stack = Vec::new();
    let module = match Module::parse(&mut cursor, &mut stack) {
        Ok(o) => o,
        Err(e) => {
            stack.reverse();
            eprintln!(
                "Error: {e:?}, bin pos: {}, stack: {stack:#?}",
                cursor.position()
            );
            return;
        }
    };
    let ExportDesc::Func(TypeIdX(main_id)) = module
        .exports
        .exports
        .iter()
        .find(|s| s.nm.0 == "main")
        .map(|f| f.d)
        .unwrap()
    else {
        panic!("no main :(")
    };
    // println!("{module:#?}");
    Runtime::new(module).call_by_id(main_id);
}
