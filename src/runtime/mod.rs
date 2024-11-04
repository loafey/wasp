use std::{collections::HashMap, fmt::Debug, mem};
pub mod clean_model;
mod memory;
use clean_model::{Function, Model};
use memory::Memory;
use RuntimeError::*;

use crate::parser::{
    self, ExportDesc, FuncIdx, Global, GlobalIdX,
    Instr::{self, *},
    LabelIdX, LocalIdX, MemArg, Module, TableIdX, TypeIdX, BT,
};

#[derive(Clone, Copy)]
#[allow(unused)]
pub enum Value {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f32),
    BlockLock,
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::I32(arg0) => write!(f, "i32({arg0})"),
            Self::I64(arg0) => write!(f, "i64({arg0})"),
            Self::F32(arg0) => write!(f, "f32({arg0})"),
            Self::F64(arg0) => write!(f, "f64({arg0})"),
            Self::BlockLock => write!(f, "--- BLOCK ---"),
        }
    }
}

pub struct DepthValue {
    bt: BT,
    pos: usize,
}

impl Debug for DepthValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} - {}", self.bt, self.pos)
    }
}

#[derive(Debug)]
pub struct Frame {
    pub func_id: u32,
    pub pc: usize,
    pub stack: Vec<Value>,
    pub locals: HashMap<u32, Value>,
    // pub labels: HashMap<u32, u32>,
    pub depth_stack: Vec<DepthValue>,
}

#[derive(Debug)]
#[allow(unused)]
pub enum RuntimeError {
    NoMain,
    Exit(i32),
    GlobalWithoutOffset,
    ActiveDataWithoutOffset,
    NoFrame(&'static str, u32, u32),
    EmptyStack(&'static str, u32, u32),
    WrongType(&'static str, u32, u32),
    MissingLocal(&'static str, u32, u32),
    UnknownFunction(String, String),
    Impossible(&'static str, u32, u32),
    MissingJumpLabel(&'static str, u32, u32),
    MissingFunction(&'static str, u32, u32),
    MissingTableIndex(&'static str, u32, u32),
}

pub struct Runtime {
    pub module: Model,
    pub stack: Vec<Frame>,
    pub globals: HashMap<u32, Value>,
    pub memory: Memory<1024>,
    pub datas: HashMap<u32, Vec<u8>>,
}
impl Runtime {
    pub fn new(module: Module) -> Result<Self, RuntimeError> {
        let mut memory = Memory::new();
        let mut datas = HashMap::new();
        for (i, d) in module.datas.data.iter().enumerate() {
            match d {
                parser::Data::Active(e, vec) => {
                    let [Instr::x41_i32_const(p)] = &e.instrs[..] else {
                        return Err(ActiveDataWithoutOffset);
                    };
                    for (i, v) in vec.iter().enumerate() {
                        memory.set(
                            *p as usize + i,
                            MemArg {
                                align: 0,
                                offset: 0,
                            },
                            *v,
                        );
                    }
                }
                parser::Data::Passive(v) => {
                    datas.insert(i as u32, v.clone());
                }
                parser::Data::ActiveX(_, _, _) => todo!(),
            }
        }

        let Some(ExportDesc::Func(TypeIdX(main_id))) = module
            .exports
            .exports
            .iter()
            .find(|s| matches!(&*s.nm.0, "main" | "_start"))
            .map(|f| f.d)
        else {
            return Err(NoMain);
        };
        let mut globals = HashMap::new();
        for (i, Global { e, .. }) in module.globals.globals.iter().enumerate() {
            let x41_i32_const(x) = e.instrs[0] else {
                return Err(GlobalWithoutOffset);
            };
            globals.insert(i as u32, Value::I32(x));
        }

        let module = Model::from(module);
        Ok(Self {
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
            datas,
        })
    }
    pub fn step(&mut self) -> Result<(), RuntimeError> {
        macro_rules! unwrap {
            ($expr:expr, $err:expr) => {
                $expr.ok_or($err(file!(), line!(), column!()))?
            };
        }
        let f = unwrap!(self.stack.last_mut(), NoFrame);

        macro_rules! local {
            ($index:expr) => {
                unwrap!(f.locals.get($index), MissingLocal)
            };
        }

        macro_rules! pop {
            () => {
                unwrap!(f.stack.pop(), EmptyStack)
            };
        }

        macro_rules! throw {
            ($expr:expr) => {
                unwrap!(None, $expr)
            };
        }

        match &self.module.functions[&f.func_id] {
            Function::Import { module, name, .. } => {
                println!("calling {module:?}::{name:?}");
                match (&*module.0, &*name.0) {
                    ("console", "log") => {
                        let y = *local!(&0);
                        let x = *local!(&1);
                        let (x, y) = if let (Value::I32(x), Value::I32(y)) = (x, y) {
                            (x as usize, y as usize)
                        } else {
                            throw!(WrongType)
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
                        f.stack.push(Value::I32(std::env::args().count() as i32));
                        let s = std::env::args_os().map(|s| s.len() + 1).sum::<usize>() as i32;
                        f.stack.push(Value::I32(s));
                        f.stack.push(Value::I32(0));
                    }
                    ("wasi_snapshot_preview1", "proc_exit") => {
                        let Value::I32(x) = *local!(&0) else {
                            throw!(WrongType)
                        };
                        return Err(Exit(x));
                    }
                    ("wasi_snapshot_preview1", "args_get") => {
                        let Value::I32(argv) = *local!(&0) else {
                            throw!(WrongType)
                        };
                        let Value::I32(argv_buf) = *local!(&0) else {
                            throw!(WrongType)
                        };
                        println!("{argv} {argv_buf}");
                        f.stack.push(Value::I32(0));
                    }
                    (module, function) => {
                        return Err(UnknownFunction(module.to_string(), function.to_string()))
                    }
                }
                let mut frame = unwrap!(self.stack.pop(), NoFrame);
                let last = unwrap!(self.stack.last_mut(), NoFrame);
                last.stack.append(&mut frame.stack);
                Ok(())
            }
            Function::Local { code, ty, .. } => {
                if f.pc >= code.len() {
                    let mut frame = unwrap!(self.stack.pop(), NoFrame);
                    let last = unwrap!(self.stack.last_mut(), NoFrame);
                    for _ in 0..ty.output.types.len() {
                        last.stack.push(unwrap!(frame.stack.pop(), EmptyStack));
                    }
                    return Ok(());
                }
                let mut instr = &code[f.pc];
                instr = if let comment(_, r) = instr { r } else { instr };
                f.pc += 1;
                match instr {
                    x02_block(_, _) => throw!(Impossible),
                    x03_loop(_, _) => throw!(Impossible),
                    x0c_br(LabelIdX(label)) => {
                        let mut last = None;
                        for _ in 0..=*label {
                            last = f.depth_stack.pop();
                        }
                        let bt = unwrap!(last, MissingJumpLabel);
                        match bt.bt {
                            BT::Loop => {
                                f.pc = bt.pos;
                            }
                            BT::Block => {
                                f.pc = bt.pos + 1;
                            }
                        }
                        for _ in 0..=*label {
                            loop {
                                if matches!(pop!(), Value::BlockLock) {
                                    break;
                                }
                            }
                        }
                    }
                    x0d_br_if(LabelIdX(label)) => {
                        let Value::I32(val) = pop!() else {
                            throw!(WrongType)
                        };

                        if val != 0 {
                            let mut last = None;
                            for _ in 0..=*label {
                                last = f.depth_stack.pop();
                            }
                            let bt = unwrap!(last, MissingJumpLabel);
                            match bt.bt {
                                BT::Loop => {
                                    f.pc = bt.pos;
                                }
                                BT::Block => {
                                    f.pc = bt.pos + 1;
                                }
                            }
                            for _ in 0..=*label {
                                loop {
                                    if matches!(pop!(), Value::BlockLock) {
                                        break;
                                    }
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
                        let mut last_f = unwrap!(self.stack.pop(), NoFrame);
                        let ty =
                            unwrap!(self.module.functions.get(&last_f.func_id), MissingFunction);
                        let ty = match ty {
                            Function::Import { ty, .. } => ty,
                            Function::Local { ty, .. } => ty,
                        };
                        let mut res = Vec::new();
                        for _ in ty.output.types.iter() {
                            res.push(unwrap!(last_f.stack.pop(), EmptyStack))
                        }
                        res.reverse();
                        unwrap!(self.stack.last_mut(), NoFrame)
                            .stack
                            .append(&mut res);
                    }
                    x10_call(FuncIdx(id)) => {
                        let fun = &self.module.functions[id];
                        let (ty, _) = match fun {
                            Function::Import { ty, .. } => {
                                (ty, (0..=ty.input.types.len()).collect::<Vec<_>>())
                            }
                            Function::Local { ty, locals, .. } => (ty, locals.clone()),
                        };

                        let mut locals = HashMap::new();
                        for (i, _) in ty.input.types.iter().enumerate() {
                            locals.insert(i as u32, pop!());
                        }

                        self.stack.push(Frame {
                            func_id: *id,
                            pc: 0,
                            stack: Vec::new(),
                            locals,
                            depth_stack: Vec::new(),
                        });
                    }
                    x11_call_indirect(TypeIdX(type_index), TableIdX(table_index)) => {
                        let Value::I32(function_index) = pop!() else {
                            throw!(WrongType)
                        };

                        let ty =
                            unwrap!(self.module.function_types.get(type_index), MissingFunction);

                        let mut locals = HashMap::new();
                        for (i, _) in ty.input.types.iter().enumerate() {
                            locals.insert(i as u32, pop!());
                        }

                        let table = &self.module.tables[*table_index as usize];
                        println!(
                            "Call info ({}): \n\tinputs: {locals:?}\n\tfunction_index: {function_index}",
                            f.func_id
                        );
                        println!("\ttable: {table:?}");

                        let FuncIdx(id) =
                            unwrap!(table.get(&(function_index as u32)), MissingTableIndex);

                        self.stack.push(Frame {
                            func_id: *id,
                            pc: 0,
                            stack: Vec::new(),
                            locals,
                            depth_stack: Vec::new(),
                        });
                    }
                    x1a_drop => {
                        pop!();
                    }
                    x1b_select => {
                        let Value::I32(cond) = pop!() else {
                            throw!(WrongType)
                        }; // const 0
                        let y = pop!(); // const 20
                        let x = pop!(); // const 10
                        match cond == 1 {
                            true => f.stack.push(x),
                            false => f.stack.push(y),
                        }
                    }
                    x20_local_get(LocalIdX(id)) => f.stack.push(*local!(id)),
                    x21_local_set(LocalIdX(id)) => {
                        let val = pop!();
                        f.locals.insert(*id, val);
                    }
                    x22_local_tee(LocalIdX(id)) => {
                        let last = pop!();
                        f.locals.insert(*id, last);
                        f.stack.push(last);
                    }
                    x23_global_get(GlobalIdX(id)) => f
                        .stack
                        .push(self.globals.get(id).copied().unwrap_or(Value::I32(0))),
                    x24_global_set(GlobalIdX(id)) => {
                        let pop = pop!();
                        self.globals.insert(*id, pop);
                    }
                    x28_i32_load(mem) => {
                        let Value::I32(addr) = pop!() else {
                            throw!(WrongType)
                        };
                        f.stack
                            .push(Value::I32(self.memory.get(addr as usize, *mem)));
                    }
                    x2d_i32_load8_u(mem) => {
                        let Value::I32(addr) = pop!() else {
                            throw!(WrongType)
                        };
                        f.stack.push(Value::I32(unsafe {
                            mem::transmute::<u32, i32>(
                                self.memory.get::<u8>(addr as usize, *mem) as u32
                            )
                        }));
                    }
                    x36_i32_store(mem) => {
                        let Value::I32(v) = pop!() else {
                            throw!(WrongType)
                        };
                        let Value::I32(addr) = pop!() else {
                            throw!(WrongType)
                        };
                        self.memory.set(addr as usize, *mem, v);
                    }
                    x37_i64_store(mem) => {
                        let Value::I64(v) = pop!() else {
                            throw!(WrongType)
                        };
                        let Value::I32(addr) = pop!() else {
                            throw!(WrongType)
                        };
                        self.memory.set(addr as usize, *mem, v);
                    }
                    x3a_i32_store8(mem) => {
                        let Value::I32(v) = pop!() else {
                            throw!(WrongType)
                        };
                        let Value::I32(addr) = pop!() else {
                            throw!(WrongType)
                        };
                        self.memory.set(addr as usize, *mem, v as u8);
                    }
                    x41_i32_const(i) => f.stack.push(Value::I32(*i)),
                    x42_i64_const(val) => f.stack.push(Value::I64(*val)),
                    x45_i32_eqz => {
                        let Value::I32(val) = pop!() else {
                            throw!(WrongType)
                        };
                        f.stack.push(Value::I32((val == 0) as i32));
                    }
                    x46_i32_eq => {
                        let y = pop!();
                        let x = pop!();
                        match (x, y) {
                            (Value::I32(x), Value::I32(y)) => {
                                let r = (x == y) as i32;
                                f.stack.push(Value::I32(r))
                            }
                            _ => throw!(WrongType),
                        }
                    }
                    x47_i32_ne => {
                        let y = pop!();
                        let x = pop!();
                        match (x, y) {
                            (Value::I32(x), Value::I32(y)) => {
                                let r = (x != y) as i32;
                                f.stack.push(Value::I32(r))
                            }
                            _ => throw!(WrongType),
                        }
                    }
                    x48_i32_lt_s => {
                        let y = pop!();
                        let x = pop!();
                        match (x, y) {
                            (Value::I32(x), Value::I32(y)) => {
                                let r = (x < y) as i32;
                                f.stack.push(Value::I32(r))
                            }
                            _ => throw!(WrongType),
                        }
                    }
                    x49_i32_lt_u => {
                        let y = pop!();
                        let x = pop!();
                        match (x, y) {
                            (Value::I32(x), Value::I32(y)) => {
                                let x = unsafe { mem::transmute::<i32, u32>(x) };
                                let y = unsafe { mem::transmute::<i32, u32>(y) };
                                let r = (x < y) as i32;
                                f.stack.push(Value::I32(r))
                            }
                            _ => throw!(WrongType),
                        }
                    }
                    x4a_i32_gt_s => {
                        let y = pop!();
                        let x = pop!();
                        match (x, y) {
                            (Value::I32(x), Value::I32(y)) => {
                                let r = (x > y) as i32;
                                f.stack.push(Value::I32(r))
                            }
                            _ => throw!(WrongType),
                        }
                    }
                    x4b_i32_gt_u => {
                        let y = pop!();
                        let x = pop!();
                        match (x, y) {
                            (Value::I32(x), Value::I32(y)) => {
                                let x = unsafe { mem::transmute::<i32, u32>(x) };
                                let y = unsafe { mem::transmute::<i32, u32>(y) };
                                let r = (y > x) as i32;
                                f.stack.push(Value::I32(r))
                            }
                            _ => throw!(WrongType),
                        }
                    }
                    x4d_i32_le_u => {
                        let y = pop!();
                        let x = pop!();
                        match (x, y) {
                            (Value::I32(x), Value::I32(y)) => {
                                let x = unsafe { mem::transmute::<i32, u32>(x) };
                                let y = unsafe { mem::transmute::<i32, u32>(y) };
                                let r = (x <= y) as i32;
                                f.stack.push(Value::I32(r))
                            }
                            _ => throw!(WrongType),
                        }
                    }
                    x4e_i32_ge_s => {
                        let y = pop!();
                        let x = pop!();
                        match (x, y) {
                            (Value::I32(x), Value::I32(y)) => {
                                f.stack.push(Value::I32((x >= y) as i32))
                            }
                            _ => throw!(WrongType),
                        }
                    }
                    x4f_i32_ge_u => {
                        let y = pop!();
                        let x = pop!();
                        match (x, y) {
                            (Value::I32(x), Value::I32(y)) => {
                                let x = unsafe { mem::transmute::<i32, u32>(x) };
                                let y = unsafe { mem::transmute::<i32, u32>(y) };
                                f.stack.push(Value::I32((x >= y) as i32))
                            }
                            _ => throw!(WrongType),
                        }
                    }
                    x52_i64_ne => {
                        let y = pop!();
                        let x = pop!();
                        match (x, y) {
                            (Value::I64(x), Value::I64(y)) => {
                                f.stack.push(Value::I64((x != y) as i64))
                            }
                            _ => throw!(WrongType),
                        }
                    }
                    x6a_i32_add => {
                        let y = pop!();
                        let x = pop!();
                        match (x, y) {
                            (Value::I32(x), Value::I32(y)) => f.stack.push(Value::I32(x + y)),
                            _ => throw!(WrongType),
                        }
                    }
                    x6b_i32_sub => {
                        let y = pop!();
                        let x = pop!();
                        match (x, y) {
                            (Value::I32(x), Value::I32(y)) => f.stack.push(Value::I32(x - y)),
                            _ => throw!(WrongType),
                        }
                    }
                    x6c_i32_mul => {
                        let y = pop!();
                        let x = pop!();
                        match (x, y) {
                            (Value::I32(x), Value::I32(y)) => f.stack.push(Value::I32(x * y)),
                            _ => throw!(WrongType),
                        }
                    }
                    x71_i32_and => {
                        let y = pop!();
                        let x = pop!();
                        match (x, y) {
                            (Value::I32(x), Value::I32(y)) => f.stack.push(Value::I32(x & y)),
                            _ => throw!(WrongType),
                        }
                    }
                    x72_i32_or => {
                        let y = pop!();
                        let x = pop!();
                        match (x, y) {
                            (Value::I32(x), Value::I32(y)) => f.stack.push(Value::I32(x | y)),
                            _ => throw!(WrongType),
                        }
                    }
                    x73_i32_xor => {
                        let y = pop!();
                        let x = pop!();
                        match (x, y) {
                            (Value::I32(x), Value::I32(y)) => f.stack.push(Value::I32(x ^ y)),
                            _ => throw!(WrongType),
                        }
                    }
                    x74_i32_shl => {
                        let y = pop!();
                        let x = pop!();
                        match (x, y) {
                            (Value::I32(x), Value::I32(y)) => f.stack.push(Value::I32(x << y)),
                            _ => throw!(WrongType),
                        }
                    }
                    x7e_i64_mul => {
                        let y = pop!();
                        let x = pop!();
                        match (x, y) {
                            (Value::I64(x), Value::I64(y)) => f.stack.push(Value::I64(x * y)),
                            _ => throw!(WrongType),
                        }
                    }
                    xad_i64_extend_i32_u => {
                        let x = pop!();
                        match x {
                            Value::I32(x) => f.stack.push(Value::I64(x as i64)),
                            _ => throw!(WrongType),
                        }
                    }
                    block_start(bt, be) => {
                        f.stack.push(Value::BlockLock);
                        match bt {
                            BT::Block => f.depth_stack.push(DepthValue { bt: *bt, pos: *be }),
                            BT::Loop => f.depth_stack.push(DepthValue {
                                bt: *bt,
                                pos: f.pc - 1,
                            }),
                        }
                    }
                    block_end(_, _) => {
                        loop {
                            if let Some(Value::BlockLock) = f.stack.last() {
                                f.stack.pop();
                                break;
                            }
                            f.stack.pop();
                        }
                        f.depth_stack.pop();
                    }
                    f => {
                        unimplemented!("instruction not supported : {f:?}")
                    }
                };
                Ok(())
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
