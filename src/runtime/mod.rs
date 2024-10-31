use std::{collections::HashMap, mem};
pub mod clean_model;
mod memory;

use clean_model::{Function, Model};
use memory::Memory;

use crate::parser::{
    self, ExportDesc, FuncIdx, Global, GlobalIdX,
    Instr::{self, *},
    LabelIdX, LocalIdX, MemArg, Module, RefTyp, Table, TableIdX, TypeIdX, BT,
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
    // pub labels: HashMap<u32, u32>,
    pub depth_stack: Vec<(BT, usize)>,
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
                parser::Data::Mem0(e, vec) => {
                    let [Instr::x41_i32_const(p)] = &e.instrs[..] else {
                        panic!()
                    };
                    for (i, v) in vec.iter().enumerate() {
                        memory.set_u8(
                            *p as usize + i,
                            MemArg {
                                align: 0,
                                offset: 0,
                            },
                            *v,
                        );
                    }
                }
                parser::Data::MemB(_) => todo!(),
                parser::Data::MemX(_, _, _) => todo!(),
            }
        }

        let Some(ExportDesc::Func(TypeIdX(main_id))) = module
            .exports
            .exports
            .iter()
            .find(|s| matches!(&*s.nm.0, "main" | "_start"))
            .map(|f| f.d)
        else {
            panic!("no main :(")
        };
        let mut globals = HashMap::new();
        for (i, Global { e, .. }) in module.globals.globals.iter().enumerate() {
            let x41_i32_const(x) = e.instrs[0] else {
                panic!()
            };
            globals.insert(i as u32, Value::I32(x));
        }

        let module = Model::from(module);
        Self {
            module,
            stack: vec![Frame {
                func_id: main_id,
                pc: 0,
                stack: Vec::new(),
                locals: HashMap::new(),
                // labels: HashMap::new(),
                depth_stack: Vec::new(),
            }],
            memory,
            globals,
        }
    }
    pub fn step(&mut self) {
        let f = self.stack.last_mut().unwrap();

        match &self.module.functions[&f.func_id] {
            Function::Import { module, name, .. } => {
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
                            let s = self.memory.get(
                                i,
                                MemArg {
                                    align: 0,
                                    offset: 0,
                                },
                            );
                            b.push(s);
                        }
                        let s = String::from_utf8_lossy(&b);
                        println!("{s}")
                    }
                    ("wasi_snapshot_preview1", "args_sizes_get") => {
                        f.stack.push(Value::I32(0));
                    }
                    ("wasi_snapshot_preview1", "proc_exit") => {
                        let Value::I32(x) = *f.locals.get(&0).unwrap() else {
                            panic!()
                        };
                        std::process::exit(x);
                    }
                    ("wasi_snapshot_preview1", "args_get") => {
                        let Value::I32(_) = *f.locals.get(&0).unwrap() else {
                            panic!()
                        };
                        let Value::I32(_) = *f.locals.get(&0).unwrap() else {
                            panic!()
                        };
                        f.stack.push(Value::I32(0));
                    }
                    (module, function) => panic!("unknown function {module}::{function}"),
                }
                let mut frame = self.stack.pop().unwrap();
                let last = self.stack.last_mut().unwrap();
                last.stack.append(&mut frame.stack);
            }
            Function::Local { code, labels, .. } => {
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
                    x02_block(_, _) => panic!("impossible"),
                    x03_loop(_, _) => panic!("impossible"),
                    x0c_br(LabelIdX(label)) => {
                        let mut last = None;
                        for _ in 0..=*label {
                            last = f.depth_stack.pop();
                        }
                        match last.unwrap() {
                            (BT::Loop, l) => {
                                f.pc = l;
                            }
                            (BT::Block, l) => {
                                f.pc = l + 1;
                            }
                        }
                        // let label = f.depth - 1 - *label as usize;
                        // (0..label).for_each(|_| {
                        //     f.stack.pop();
                        // });
                        // let pc = labels.get(&(label as u32)).unwrap();
                        // f.pc = *pc as usize + 1;
                        // f.depth = label;
                    }
                    x0d_br_if(LabelIdX(label)) => {
                        let Value::I32(val) = f.stack.pop().unwrap() else {
                            unreachable!()
                        };

                        if val != 0 {
                            let mut last = None;
                            for _ in 0..=*label {
                                last = f.depth_stack.pop();
                            }
                            match last.unwrap() {
                                (BT::Loop, l) => {
                                    f.pc = l;
                                }
                                (BT::Block, l) => {
                                    f.pc = l + 1;
                                }
                            }
                            // let label = f.depth - 1 - *label as usize;
                            // let old = f.depth;
                            // f.depth = label;
                            // (0..label).for_each(|_| {
                            // f.stack.pop();
                            // });
                            // if let Some(pc) = labels.get(&(label as u32)) {
                            // f.pc = *pc as usize + 1;
                        }
                    }
                    x0f_return => {
                        let mut last_f = self.stack.pop().unwrap();
                        let ty = self.module.functions.get(&last_f.func_id).unwrap();
                        let ty = match ty {
                            Function::Import { ty, .. } => ty,
                            Function::Local { ty, .. } => ty,
                        };
                        let mut res = Vec::new();
                        ty.output
                            .types
                            .iter()
                            .for_each(|_| res.push(last_f.stack.pop().unwrap()));
                        res.reverse();
                        self.stack.last_mut().unwrap().stack.append(&mut res);
                    }
                    x10_call(FuncIdx(id)) => {
                        let fun = &self.module.functions[id];
                        let (ty, _) = match fun {
                            Function::Import { ty, .. } => {
                                (ty, (0..=ty.input.types.len()).collect::<Vec<_>>())
                            }
                            Function::Local { ty, locals, .. } => (ty, locals.clone()),
                        };

                        let locals = ty
                            .input
                            .types
                            .iter()
                            .enumerate()
                            .map(|(i, _)| (i as u32, f.stack.pop().unwrap()))
                            .collect();

                        self.stack.push(Frame {
                            func_id: *id,
                            pc: 0,
                            stack: Vec::new(),
                            locals,
                            depth_stack: Vec::new(),
                        });
                    }
                    x11_call_indirect(TypeIdX(ti), TableIdX(tai)) => {
                        let Table { et, lim } = &self.module.tables[*tai as usize];
                        if !matches!(et, RefTyp::FuncRef) {
                            panic!()
                        }
                        panic!("call x11_call_indirect {ti} - {tai}")
                    }
                    x1a_drop => {
                        f.stack.pop().unwrap();
                    }
                    x1b_select => {
                        let Value::I32(cond) = f.stack.pop().unwrap() else {
                            unreachable!()
                        }; // const 0
                        let y = f.stack.pop().unwrap(); // const 20
                        let x = f.stack.pop().unwrap(); // const 10
                        if cond == 1 {
                            f.stack.push(x);
                        } else {
                            f.stack.push(y);
                        }
                    }
                    x20_local_get(LocalIdX(id)) => f.stack.push(*f.locals.get(id).unwrap()),
                    x21_local_set(LocalIdX(id)) => {
                        if f.stack.is_empty() {
                            f.pc -= 1;
                            println!("!LOCK!");
                            return;
                        }
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
                    x28_i32_load(mem) => {
                        let Value::I32(addr) = f.stack.pop().unwrap() else {
                            panic!()
                        };
                        f.stack
                            .push(Value::I32(self.memory.get(addr as usize, *mem)));
                    }
                    x2d_i32_load8_u(mem) => {
                        let Value::I32(addr) = f.stack.pop().unwrap() else {
                            panic!()
                        };
                        f.stack.push(Value::I32(unsafe {
                            mem::transmute::<u32, i32>(
                                self.memory.get::<u8>(addr as usize, *mem) as u32
                            )
                        }));
                    }
                    x36_i32_store(mem) => {
                        let Value::I32(v) = f.stack.pop().unwrap() else {
                            panic!()
                        };
                        let Value::I32(addr) = f.stack.pop().unwrap() else {
                            panic!()
                        };
                        let bytes = v.to_le_bytes();
                        for (i, b) in bytes.into_iter().enumerate() {
                            self.memory.set_u8(addr as usize, *mem, b);
                        }
                    }
                    x37_i64_store(mem) => {
                        let Value::I64(v) = f.stack.pop().unwrap() else {
                            panic!()
                        };
                        let Value::I32(addr) = f.stack.pop().unwrap() else {
                            panic!()
                        };
                        let bytes = v.to_le_bytes();
                        for (i, b) in bytes.into_iter().enumerate() {
                            self.memory.set_u8(addr as usize, *mem, b);
                        }
                    }
                    x3a_i32_store8(mem) => {
                        let Value::I32(v) = f.stack.pop().unwrap() else {
                            panic!()
                        };
                        let Value::I32(addr) = f.stack.pop().unwrap() else {
                            panic!()
                        };
                        self.memory.set_u8(addr as usize, *mem, v as u8);
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
                    x46_i32_eq => {
                        let y = f.stack.pop().unwrap();
                        let x = f.stack.pop().unwrap();
                        match (x, y) {
                            (Value::I32(x), Value::I32(y)) => {
                                let r = (x == y) as i32;
                                f.stack.push(Value::I32(r))
                            }
                            _ => unreachable!(),
                        }
                    }
                    x47_i32_ne => {
                        let y = f.stack.pop().unwrap();
                        let x = f.stack.pop().unwrap();
                        match (x, y) {
                            (Value::I32(x), Value::I32(y)) => {
                                let r = (x != y) as i32;
                                f.stack.push(Value::I32(r))
                            }
                            _ => unreachable!(),
                        }
                    }
                    x48_i32_lt_s => {
                        let y = f.stack.pop().unwrap();
                        let x = f.stack.pop().unwrap();
                        match (x, y) {
                            (Value::I32(x), Value::I32(y)) => {
                                let r = (x < y) as i32;
                                f.stack.push(Value::I32(r))
                            }
                            _ => unreachable!(),
                        }
                    }
                    x49_i32_lt_u => {
                        let y = f.stack.pop().unwrap();
                        let x = f.stack.pop().unwrap();
                        match (x, y) {
                            (Value::I32(x), Value::I32(y)) => {
                                let x = unsafe { mem::transmute::<i32, u32>(x) };
                                let y = unsafe { mem::transmute::<i32, u32>(y) };
                                let r = (x < y) as i32;
                                f.stack.push(Value::I32(r))
                            }
                            _ => unreachable!(),
                        }
                    }
                    x4a_i32_gt_s => {
                        let y = f.stack.pop().unwrap();
                        let x = f.stack.pop().unwrap();
                        match (x, y) {
                            (Value::I32(x), Value::I32(y)) => {
                                let r = (x > y) as i32;
                                f.stack.push(Value::I32(r))
                            }
                            _ => unreachable!(),
                        }
                    }
                    x4b_i32_gt_u => {
                        let y = f.stack.pop().unwrap();
                        let x = f.stack.pop().unwrap();
                        match (x, y) {
                            (Value::I32(x), Value::I32(y)) => {
                                let x = unsafe { mem::transmute::<i32, u32>(x) };
                                let y = unsafe { mem::transmute::<i32, u32>(y) };
                                let r = (y > x) as i32;
                                f.stack.push(Value::I32(r))
                            }
                            _ => unreachable!(),
                        }
                    }
                    x4d_i32_le_u => {
                        let y = f.stack.pop().unwrap();
                        let x = f.stack.pop().unwrap();
                        match (x, y) {
                            (Value::I32(x), Value::I32(y)) => {
                                let x = unsafe { mem::transmute::<i32, u32>(x) };
                                let y = unsafe { mem::transmute::<i32, u32>(y) };
                                let r = (x <= y) as i32;
                                f.stack.push(Value::I32(r))
                            }
                            _ => unreachable!(),
                        }
                    }
                    x4e_i32_ge_s => {
                        let y = f.stack.pop().unwrap();
                        let x = f.stack.pop().unwrap();
                        match (x, y) {
                            (Value::I32(x), Value::I32(y)) => {
                                f.stack.push(Value::I32((x >= y) as i32))
                            }
                            _ => unreachable!(),
                        }
                    }
                    x4f_i32_ge_u => {
                        let y = f.stack.pop().unwrap();
                        let x = f.stack.pop().unwrap();
                        match (x, y) {
                            (Value::I32(x), Value::I32(y)) => {
                                let x = unsafe { mem::transmute::<i32, u32>(x) };
                                let y = unsafe { mem::transmute::<i32, u32>(y) };
                                f.stack.push(Value::I32((x >= y) as i32))
                            }
                            _ => unreachable!(),
                        }
                    }
                    x52_i64_ne => {
                        let y = f.stack.pop().unwrap();
                        let x = f.stack.pop().unwrap();
                        match (x, y) {
                            (Value::I64(x), Value::I64(y)) => {
                                f.stack.push(Value::I64((x != y) as i64))
                            }
                            _ => unreachable!(),
                        }
                    }
                    x6a_i32_add => {
                        let y = f.stack.pop().unwrap();
                        let x = f.stack.pop().unwrap();
                        match (x, y) {
                            (Value::I32(x), Value::I32(y)) => f.stack.push(Value::I32(x + y)),
                            _ => unreachable!(),
                        }
                    }
                    x6b_i32_sub => {
                        let y = f.stack.pop().unwrap();
                        let x = f.stack.pop().unwrap();
                        match (x, y) {
                            (Value::I32(x), Value::I32(y)) => f.stack.push(Value::I32(x - y)),
                            _ => unreachable!(),
                        }
                    }
                    x6c_i32_mul => {
                        let y = f.stack.pop().unwrap();
                        let x = f.stack.pop().unwrap();
                        match (x, y) {
                            (Value::I32(x), Value::I32(y)) => f.stack.push(Value::I32(x * y)),
                            _ => unreachable!(),
                        }
                    }
                    x71_i32_and => {
                        let y = f.stack.pop().unwrap();
                        let x = f.stack.pop().unwrap();
                        match (x, y) {
                            (Value::I32(x), Value::I32(y)) => f.stack.push(Value::I32(x & y)),
                            _ => unreachable!(),
                        }
                    }
                    x72_i32_or => {
                        let y = f.stack.pop().unwrap();
                        let x = f.stack.pop().unwrap();
                        match (x, y) {
                            (Value::I32(x), Value::I32(y)) => f.stack.push(Value::I32(x | y)),
                            _ => unreachable!(),
                        }
                    }
                    x73_i32_xor => {
                        let y = f.stack.pop().unwrap();
                        let x = f.stack.pop().unwrap();
                        match (x, y) {
                            (Value::I32(x), Value::I32(y)) => f.stack.push(Value::I32(x ^ y)),
                            _ => unreachable!(),
                        }
                    }
                    x74_i32_shl => {
                        let y = f.stack.pop().unwrap();
                        let x = f.stack.pop().unwrap();
                        match (x, y) {
                            (Value::I32(x), Value::I32(y)) => f.stack.push(Value::I32(x << y)),
                            _ => unreachable!(),
                        }
                    }
                    x7e_i64_mul => {
                        let y = f.stack.pop().unwrap();
                        let x = f.stack.pop().unwrap();
                        match (x, y) {
                            (Value::I64(x), Value::I64(y)) => f.stack.push(Value::I64(x * y)),
                            _ => unreachable!(),
                        }
                    }
                    xad_i64_extend_i32_u => {
                        let x = f.stack.pop().unwrap();
                        match x {
                            Value::I32(x) => f.stack.push(Value::I64(x as i64)),
                            _ => unreachable!(),
                        }
                    }
                    block_start(bt, be) => match bt {
                        BT::Block => f.depth_stack.push((*bt, *be)),
                        BT::Loop => f.depth_stack.push((*bt, f.pc - 1)),
                    },
                    block_end(_, _) => {
                        f.depth_stack.pop();
                    }
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
