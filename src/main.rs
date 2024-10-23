#![feature(const_type_name)]

use hex::Hex;
use parser::{
    ExportDesc, FuncIdx, GlobalIdX, ImportDesc, Instr::*, LocalIdX, MemArg, Module, Parsable,
    TypeIdX,
};
use std::{collections::HashMap, fmt::format, io::Cursor, mem::MaybeUninit};
mod hex;
mod parser;

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

#[derive(Clone, Copy)]
enum Value {
    I32(i32),
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::I32(arg0) => write!(f, "i32({arg0})"),
        }
    }
}

#[derive(Debug, Default)]
struct Frame {
    pub stack: Vec<Value>,
    pub locals: HashMap<u32, Value>,
}

struct Runtime {
    module: Module,
    stack: Vec<Frame>,
    globals: HashMap<u32, Value>,
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
            stack: vec![Frame {
                stack: Vec::new(),
                locals: [(0, Value::I32(0)), (1, Value::I32(0))].into(),
            }],
            data,
            globals: HashMap::new(),
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
                        let Some((Value::I32(y), Value::I32(x))) =
                            st.stack.pop().zip(st.stack.pop())
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
                std::thread::sleep(std::time::Duration::from_secs_f32(0.5));
                let instr = &instrs[pc];

                let mut fs = "┌────┄┄┄┈┈\n".to_string();
                for line in format!(
                    "════  Stack  ════\n{:#?}\n════ Globals ════\n{:#?}",
                    self.stack, self.globals
                )
                .lines()
                {
                    fs += &format!("│ {line}\n");
                }
                fs += "├────┄┄┄┈┈┈┈\n";
                fs += &format!("│ Executing: {instr:?}\n└─┄┈");
                println!("{fs}");
                let f = self.stack.last_mut().unwrap();

                match instr {
                    x10_i32_const(i) => f.stack.push(Value::I32(*i)),
                    x20_local_get(LocalIdX(id)) => f.stack.push(*f.locals.get(id).unwrap()),
                    x22_local_tee(LocalIdX(id)) => {
                        let last = f.stack.last().unwrap();
                        f.locals.insert(*id, *last);
                    }
                    x23_global_get(GlobalIdX(id)) => f
                        .stack
                        .push(self.globals.get(id).copied().unwrap_or(Value::I32(0))),
                    x24_global_set(GlobalIdX(id)) => {
                        let pop = f.stack.pop().unwrap();
                        self.globals.insert(*id, pop);
                    }
                    x36_i32_store(MemArg { align, offset }) => {
                        let addr = (align * offset) as usize;
                        let end_pos = addr + size_of::<i32>();
                        if self.module.mems.len() < end_pos {
                            self.module
                                .mems
                                .append(&mut vec![0; end_pos - self.module.mems.len()]);
                        }
                        #[allow(irrefutable_let_patterns)]
                        let Value::I32(v) = f.stack.pop().unwrap() else {
                            panic!()
                        };
                        let bytes = v.to_le_bytes();
                        for (i, b) in bytes.into_iter().enumerate() {
                            self.module.mems[addr + i] = b;
                        }
                    }
                    x6a_i32_add => {
                        let y = f.stack.pop().unwrap();
                        let x = f.stack.pop().unwrap();
                        match (x, y) {
                            (Value::I32(x), Value::I32(y)) => f.stack.push(Value::I32(y + x)),
                        }
                    }
                    x6b_i32_sub => {
                        let y = f.stack.pop().unwrap();
                        let x = f.stack.pop().unwrap();
                        match (x, y) {
                            (Value::I32(x), Value::I32(y)) => f.stack.push(Value::I32(y - x)),
                        }
                    }
                    x41_call(FuncIdx(id)) => {
                        self.stack.push(Frame::default());
                        self.call_by_id(*id);
                        self.stack.pop();
                    }
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
    // println!("{module:#?}");
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
