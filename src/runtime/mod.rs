use std::collections::HashMap;
pub mod clean_model;
mod memory;

use clean_model::{Function, Model};
use memory::Memory;

use crate::parser::{
    self, ExportDesc, FuncIdx, GlobalIdX, ImportDesc, Instr::*, LabelIdX, LocalIdX, MemArg, Module,
    NumType, TypeIdX, ValType,
};

#[derive(Clone, Copy)]
pub enum Value {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f32),
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::I32(arg0) => write!(f, "i32({arg0})"),
            Self::I64(arg0) => write!(f, "i64({arg0})"),
            Self::F32(arg0) => write!(f, "f32({arg0})"),
            Self::F64(arg0) => write!(f, "f64({arg0})"),
        }
    }
}

#[derive(Debug)]
pub struct Frame {
    pub func_id: u32,
    pub pc: usize,
    pub stack: Vec<Value>,
    pub locals: HashMap<u32, Value>,
    pub labels: HashMap<u32, u32>,
    pub block_count: Vec<usize>,
}

pub struct Runtime {
    pub module: Model,
    pub stack: Vec<Frame>,
    pub globals: HashMap<u32, Value>,
    pub memory: Memory<1024>,
}
impl Runtime {
    pub fn new(module: Module) -> Self {
        let mut memory = Memory::new();
        for d in &module.datas.data {
            match d {
                parser::Data::Mem0(_, vec) => {
                    for (i, v) in vec.iter().enumerate() {
                        memory.set_u8(i, *v);
                    }
                }
                parser::Data::MemB(_) => todo!(),
                parser::Data::MemX(_, _, _) => todo!(),
            }
        }

        let iter = &module.exports.exports;

        let Some(ExportDesc::Func(TypeIdX(main_id))) = iter
            .iter()
            .find(|s| matches!(&*s.nm.0, "main" | "_start"))
            .map(|f| f.d)
        else {
            panic!("no main :(")
        };

        let module = Model::from(module);
        Self {
            module,
            stack: vec![Frame {
                func_id: main_id,
                pc: 0,
                stack: Vec::new(),
                locals: HashMap::new(),
                labels: HashMap::new(),
                block_count: Vec::new(),
            }],
            memory,
            globals: HashMap::new(),
        }
    }
    pub fn step(&mut self) {
        let f = self.stack.last_mut().unwrap();

        match &self.module.functions[&f.func_id] {
            Function::Import { module, name, .. } => {
                println!("calling \"{}::{}\"", &*module.0, &*name.0);
                match (&*module.0, &*name.0) {
                    ("console", "log") => {
                        let y = *f.locals.get(&0).unwrap();
                        let x = *f.locals.get(&1).unwrap();
                        let (x, y) = if let (Value::I32(x), Value::I32(y)) = (x, y) {
                            (x as usize, y as usize)
                        } else {
                            panic!()
                        };
                        let mut b = Vec::new();
                        for i in x..y {
                            let s = self.memory.get_u8(i);
                            b.push(s);
                        }
                        let s = String::from_utf8_lossy(&b);
                        println!("{s}")
                    }
                    ("wasi_snapshot_preview1", "args_sizes_get") => {
                        let args = std::env::args().count();
                        f.stack.push(Value::I32((args - 1) as i32)); // maybe i shouldn't do this?
                    }
                    ("wasi_snapshot_preview1", "proc_exit") => {
                        let Value::I32(x) = *f.locals.get(&0).unwrap() else {
                            panic!()
                        };
                        std::process::exit(x);
                    }
                    (module, function) => panic!("unknown function {module}::{function}"),
                }
                let mut frame = self.stack.pop().unwrap();
                let last = self.stack.last_mut().unwrap();
                last.stack.append(&mut frame.stack);
            }
            Function::Local { code, .. } => {
                if f.pc >= code.len() {
                    let mut frame = self.stack.pop().unwrap();
                    let last = self.stack.last_mut().unwrap();
                    last.stack.append(&mut frame.stack);
                    return;
                }
                let mut instr = &code[f.pc];
                instr = if let comment(_, r) = instr { r } else { instr };
                f.pc += 1;

                match instr {
                    x02_block(bt, ins) => {
                        f.block_count.push(f.stack.len());
                        match bt {
                            parser::BlockType::Eps => {}
                            parser::BlockType::T(_) => todo!(),
                            parser::BlockType::X(_) => todo!(),
                        }

                        let c = ins.clone();
                        let func = self.module.functions.get_mut(&f.func_id).unwrap();
                        let Function::Local { ty, locals, code } = func else {
                            unreachable!()
                        };

                        code.remove(f.pc - 1);
                        let pos_before = f.pc;
                        let mut modified = 0;
                        code.insert(f.pc - 1, block_end);
                        for (c, i) in c.into_iter().enumerate().rev() {
                            modified += 1;
                            code.insert(f.pc - 1, comment(format!("line: {c:?}"), Box::new(i)));
                        }
                        f.labels
                            .iter_mut()
                            .for_each(|(_, r)| *r += modified as u32 + 1);
                        f.labels
                            .insert(f.labels.len() as u32, (pos_before + modified) as u32);
                        code.insert(f.pc - 1, block_start);

                        // self.module.code.code[index].code.e.instrs.remove(f.pc - 1);
                        // for i in c.into_iter().rev() {
                        //     self.module.code.code[index]
                        //         .code
                        //         .e
                        //         .instrs
                        //         .insert(f.pc - 1, i);
                        // }
                        // self.module.code.code[index]
                        //     .code
                        //     .e
                        //     .instrs
                        //     .insert(f.pc - 1, block_start);
                    }
                    x0c_br(LabelIdX(label)) => {
                        (0..*label).for_each(|_| {
                            f.stack.pop();
                        });
                        let pc = f.labels.get(label).unwrap();
                        f.pc = *pc as usize;
                    }
                    x0d_br_if(LabelIdX(label)) => {
                        let Value::I32(val) = f.stack.pop().unwrap() else {
                            unreachable!()
                        };

                        if val != 0 {
                            (0..*label).for_each(|_| {
                                f.stack.pop();
                            });
                            f.pc = *f.labels.get(label).unwrap() as usize;
                        }
                    }
                    x10_call(FuncIdx(id)) => {
                        let fun = &self.module.functions[id];
                        let (ty, num) = match fun {
                            Function::Import { ty, .. } => {
                                (ty, (0..=ty.input.types.len()).collect::<Vec<_>>())
                            }
                            Function::Local { ty, locals, .. } => (ty, locals.clone()),
                        };

                        // if !f.stack.is_empty() {
                        let locals = ty
                            .input
                            .types
                            .iter()
                            .enumerate()
                            .map(|(i, _)| (i as u32, f.stack.pop().unwrap()))
                            .collect();
                        // } else {
                        // ty.input.types.iter().zip(num).for_each(|(v, n)| {
                        // let v = match v {
                        // ValType::Num(num_type) => match num_type {
                        // NumType::I32 => Value::I32(0),
                        // NumType::I64 => Value::I64(0),
                        // NumType::F32 => Value::F32(0.0),
                        // NumType::F64 => Value::F64(0.0),
                        // },
                        // ValType::Vec => todo!(),
                        // ValType::Ref(ref_typ) => todo!(),
                        // };
                        // locals.insert(n as u32, v);
                        // });
                        // }

                        self.stack.push(Frame {
                            func_id: *id,
                            pc: 0,
                            stack: Vec::new(),
                            locals,
                            labels: HashMap::new(),
                            block_count: Vec::new(),
                        });
                    }
                    x20_local_get(LocalIdX(id)) => f.stack.push(*f.locals.get(id).unwrap()),
                    x21_local_set(LocalIdX(id)) => {
                        let val = f.stack.pop().unwrap();
                        f.locals.insert(*id, val);
                    }
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
                    x28_i32_load(MemArg { align, offset }) => {
                        let Value::I32(addr) = f.stack.pop().unwrap() else {
                            panic!()
                        };
                        let offset = (align * offset) as usize;
                        f.stack
                            .push(Value::I32(self.memory.get(addr as usize + offset)));
                    }
                    x36_i32_store(MemArg { align, offset }) => {
                        let Value::I32(v) = f.stack.pop().unwrap() else {
                            panic!()
                        };
                        let Value::I32(addr) = f.stack.pop().unwrap() else {
                            panic!()
                        };
                        let offset = (align * offset) as usize;
                        let bytes = v.to_le_bytes();
                        for (i, b) in bytes.into_iter().enumerate() {
                            self.memory.set_u8(addr as usize + offset + i, b);
                        }
                    }
                    x41_i32_const(i) => f.stack.push(Value::I32(*i)),
                    x42_i64_const(val) => {
                        f.stack.push(Value::I64(*val));
                    }
                    x45_i32_eqz => {
                        let Value::I32(val) = f.stack.pop().unwrap() else {
                            unreachable!()
                        };
                        f.stack.push(Value::I32((val == 0) as i32));
                    }
                    x52_i64_ne => {
                        let y = f.stack.pop().unwrap();
                        let x = f.stack.pop().unwrap();
                        match (x, y) {
                            (Value::I64(x), Value::I64(y)) => {
                                f.stack.push(Value::I64((y != x) as i64))
                            }
                            _ => unreachable!(),
                        }
                    }
                    x6a_i32_add => {
                        let y = f.stack.pop().unwrap();
                        let x = f.stack.pop().unwrap();
                        match (x, y) {
                            (Value::I32(x), Value::I32(y)) => f.stack.push(Value::I32(y + x)),
                            _ => unreachable!(),
                        }
                    }
                    x6b_i32_sub => {
                        let y = f.stack.pop().unwrap();
                        let x = f.stack.pop().unwrap();
                        match (x, y) {
                            (Value::I32(x), Value::I32(y)) => f.stack.push(Value::I32(y - x)),
                            _ => unreachable!(),
                        }
                    }
                    x71_i32_and => {
                        let y = f.stack.pop().unwrap();
                        let x = f.stack.pop().unwrap();
                        match (x, y) {
                            (Value::I32(x), Value::I32(y)) => f.stack.push(Value::I32(y & x)),
                            _ => unreachable!(),
                        }
                    }
                    block_start | block_end => {}
                    f => {
                        unimplemented!("instruction not supported : {f:?}")
                    }
                }
            }
        }

        // if f.func_id < self.import_count {
        //     let func = &self.module.imports.imports[f.func_id];
        //     match &*func.module.0 {
        //         m @ "console" => match &*func.name.0 {
        //             "log" => {
        //                 let st = self.stack.last_mut().unwrap();
        //                 let Some((Value::I32(y), Value::I32(x))) =
        //                     st.stack.pop().zip(st.stack.pop())
        //                 else {
        //                     unimplemented!()
        //                 };
        //                 let (x, y) = (x as usize, y as usize);

        //                 let str = String::from_utf8_lossy(&self.data[x..y]);
        //                 println!("{str}");
        //             }
        //             n => panic!("no function named {n:?} in module {m:?}"),
        //         },
        //         m => panic!("unknown module {m:?}"),
        //     }
        // } else {
    }
}
