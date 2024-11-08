use std::{
    collections::HashMap,
    fmt::Debug,
    fs::File,
    io::{Cursor, Read},
    mem,
    path::{Path, PathBuf},
};
pub mod clean_model;
mod memory;
use clean_model::{Function, Model};
use memory::Memory;
mod error;
pub use error::RuntimeError;
use RuntimeError::*;

use crate::parser::{
    self, ExportDesc, FuncIdx, Global, GlobalIdX,
    Instr::{self, *},
    LabelIdX, LocalIdX, MemArg, MemIdX, Module, Parsable, TableIdX, TypeIdX, BT,
};

#[derive(Clone, Copy, PartialEq)]
#[allow(unused)]
pub enum Value {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
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

pub struct Runtime {
    pub path: PathBuf,
    pub module: Model,
    pub stack: Vec<Frame>,
    pub globals: HashMap<u32, Value>,
    pub memory: Memory<65536>,
    pub exports: HashMap<String, ExportDesc>,
    #[allow(unused)]
    pub datas: HashMap<u32, Vec<u8>>,
}
impl Runtime {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, RuntimeError> {
        let mut buf = Vec::new();

        let mut f = File::open(path.as_ref()).expect("Failed to open file");
        f.read_to_end(&mut buf).expect("Failed to read file");

        let mut cursor = Cursor::new(&buf[..]);
        let mut stack = Vec::new();
        let module = match Module::parse(&mut cursor, &mut stack) {
            Ok(o) => o,
            Err(e) => {
                stack.reverse();
                return Err(RuntimeError::ParseError(format!(
                    "File: {:?}\n{e:?}, bin pos: {}, stack: {stack:#?}",
                    path.as_ref(),
                    cursor.position()
                )));
            }
        };

        let (mem_cur, mem_max) = module
            .mems
            .mems
            .first()
            .map(|m| match m.limits {
                parser::Limits::Min(i) => (i as usize, usize::MAX),
                parser::Limits::MinMax(i, m) => (i as usize, m as usize),
            })
            .unwrap_or((1, usize::MAX));
        let mut memory = Memory::new(mem_cur, mem_max);
        let mut datas = HashMap::new();
        for (i, d) in module.datas.data.iter().enumerate() {
            match d {
                parser::Data::ActiveX(MemIdX(0), e, vec) | parser::Data::Active(e, vec) => {
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
                        )?;
                    }
                }
                parser::Data::Passive(v) => {
                    datas.insert(i as u32, v.clone());
                }
                parser::Data::ActiveX(_, _, _) => todo!(""),
            }
        }

        let stack = if let Some(ExportDesc::Func(TypeIdX(main_id))) = module
            .exports
            .exports
            .iter()
            .find(|s| matches!(&*s.nm.0, "main" | "_start"))
            .map(|f| f.d)
        {
            vec![Frame {
                func_id: main_id,
                pc: 0,
                stack: Vec::new(),
                locals: HashMap::new(),
                // labels: HashMap::new(),
                depth_stack: Vec::new(),
            }]
        } else {
            Vec::new()
        };
        let mut globals = HashMap::new();
        for (i, Global { e, .. }) in module.globals.globals.iter().enumerate() {
            let val = match e.instrs[0] {
                x41_i32_const(x) => Value::I32(x),
                x42_i64_const(x) => Value::I64(x),
                x43_f32_const(x) => Value::F32(x),
                x44_f64_const(x) => Value::F64(x),
                _ => return Err(GlobalWithoutValue),
            };
            globals.insert(i as u32, val);
        }

        let exports = module
            .exports
            .exports
            .iter()
            .map(|exp| (exp.nm.0.clone(), exp.d))
            .collect();

        let module = Model::from(module);
        Ok(Self {
            module,
            stack,
            memory,
            globals,
            datas,
            exports,
            path: path.as_ref().to_path_buf(),
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
            (i32) => {{
                let Value::I32(val) = unwrap!(f.stack.pop(), EmptyStack) else {
                    throw!(WrongType)
                };
                val
            }};
            (i64) => {{
                let Value::I64(val) = unwrap!(f.stack.pop(), EmptyStack) else {
                    throw!(WrongType)
                };
                val
            }};
            (u32) => {{
                let Value::I32(val) = unwrap!(f.stack.pop(), EmptyStack) else {
                    throw!(WrongType)
                };
                unsafe { std::mem::transmute::<i32, u32>(val) }
            }};
            (u64) => {{
                let Value::I64(val) = unwrap!(f.stack.pop(), EmptyStack) else {
                    throw!(WrongType)
                };
                unsafe { std::mem::transmute::<i64, u64>(val) }
            }};
            (f32) => {{
                let Value::F32(val) = unwrap!(f.stack.pop(), EmptyStack) else {
                    throw!(WrongType)
                };
                val
            }};
            (f64) => {{
                let Value::F64(val) = unwrap!(f.stack.pop(), EmptyStack) else {
                    throw!(WrongType)
                };
                val
            }};
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
                // println!("calling {module:?}::{name:?}");
                match (&*module.0, &*name.0) {
                    #[allow(clippy::print_stdout)]
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
                            )?;
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
                    x04_if_else(_, _, _) => throw!(Impossible),
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
                        let val = pop!(i32);

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
                        }
                    }
                    x0e_br_table(labels, def) => {
                        let index = pop!(u32) as usize;
                        let label = if index > labels.len() {
                            *def
                        } else {
                            labels[index]
                        };

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
                        let function_index = pop!(i32);

                        let ty =
                            unwrap!(self.module.function_types.get(type_index), MissingFunction);

                        let mut locals = HashMap::new();
                        for (i, _) in ty.input.types.iter().enumerate() {
                            locals.insert(i as u32, pop!());
                        }

                        let table = &self.module.tables[*table_index as usize];
                        // println!(
                        //     "Call info ({}): \n\tinputs: {locals:?}\n\tfunction_index: {function_index}",
                        //     f.func_id
                        // );
                        // println!("\ttable: {table:?}");

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
                        let cond = pop!(i32);
                        let y = pop!();
                        let x = pop!();
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
                        let addr = pop!(u32);
                        f.stack
                            .push(Value::I32(self.memory.get(addr as usize, *mem)?));
                    }
                    x29_i64_load(mem) => {
                        let addr = pop!(u32);
                        f.stack
                            .push(Value::I64(self.memory.get(addr as usize, *mem)?));
                    }
                    x2a_f32_load(mem) => {
                        let addr = pop!(u32);
                        f.stack
                            .push(Value::F32(self.memory.get(addr as usize, *mem)?));
                    }
                    x2b_f64_load(mem) => {
                        let addr = pop!(u32);
                        f.stack
                            .push(Value::F64(self.memory.get(addr as usize, *mem)?));
                    }
                    x2c_i32_load8_s(mem) => {
                        let addr = pop!(u32);
                        f.stack.push(Value::I32(
                            self.memory.get::<i8>(addr as usize, *mem)? as i32
                        ));
                    }
                    x2d_i32_load8_u(mem) => {
                        let addr = pop!(u32);
                        f.stack.push(Value::I32(unsafe {
                            mem::transmute::<u32, i32>(
                                self.memory.get::<u8>(addr as usize, *mem)? as u32
                            )
                        }));
                    }
                    x2e_i32_load16_s(mem) => {
                        let addr = pop!(u32);
                        f.stack.push(Value::I32(
                            self.memory.get::<u16>(addr as usize, *mem)? as i32
                        ));
                    }
                    x2f_i32_load16_u(mem) => {
                        let addr = pop!(u32);
                        f.stack.push(Value::I32(unsafe {
                            mem::transmute::<u32, i32>(
                                self.memory.get::<u16>(addr as usize, *mem)? as u32,
                            )
                        }));
                    }
                    x30_i64_load8_s(mem) => {
                        let addr = pop!(u32);
                        f.stack.push(Value::I64(
                            self.memory.get::<i8>(addr as usize, *mem)? as i64
                        ));
                    }
                    x31_i64_load8_u(mem) => {
                        let addr = pop!(u32);
                        f.stack.push(Value::I64(unsafe {
                            mem::transmute::<u64, i64>(
                                self.memory.get::<u8>(addr as usize, *mem)? as u64
                            )
                        }));
                    }
                    x32_i64_load16_s(mem) => {
                        let addr = pop!(u32);
                        f.stack.push(Value::I64(
                            self.memory.get::<i16>(addr as usize, *mem)? as i64
                        ));
                    }
                    x33_i64_load16_u(mem) => {
                        let addr = pop!(u32);
                        f.stack.push(Value::I64(unsafe {
                            mem::transmute::<u64, i64>(
                                self.memory.get::<u16>(addr as usize, *mem)? as u64,
                            )
                        }));
                    }
                    x34_i64_load32_s(mem) => {
                        let addr = pop!(u32);
                        f.stack.push(Value::I64(
                            self.memory.get::<i32>(addr as usize, *mem)? as i64
                        ));
                    }
                    x35_i64_load32_u(mem) => {
                        let addr = pop!(u32);
                        f.stack.push(Value::I64(unsafe {
                            mem::transmute::<u64, i64>(
                                self.memory.get::<u32>(addr as usize, *mem)? as u64,
                            )
                        }));
                    }
                    x36_i32_store(mem) => {
                        let v = pop!(i32);
                        let addr = pop!(u32);
                        self.memory.set(addr as usize, *mem, v)?;
                    }
                    x37_i64_store(mem) => {
                        let v = pop!(i64);
                        let addr = pop!(u32);
                        self.memory.set(addr as usize, *mem, v)?;
                    }
                    x38_f32_store(mem) => {
                        let v = pop!(f32);
                        let addr = pop!(u32);
                        self.memory.set(addr as usize, *mem, v)?;
                    }
                    x39_f64_store(mem) => {
                        let v = pop!(f64);
                        let addr = pop!(u32);
                        self.memory.set(addr as usize, *mem, v)?;
                    }
                    x3a_i32_store8(mem) => {
                        let v = pop!(i32);
                        let addr = pop!(u32);
                        self.memory.set(addr as usize, *mem, v as i8)?;
                    }
                    x3b_i32_store16(mem) => {
                        let v = pop!(i32);
                        let addr = pop!(u32);
                        self.memory.set(addr as usize, *mem, v as i16)?;
                    }
                    x3c_i64_store8(mem) => {
                        let v = pop!(i64);
                        let addr = pop!(u32);
                        self.memory.set(addr as usize, *mem, v as i8)?;
                    }
                    x3d_i64_store16(mem) => {
                        let v = pop!(i64);
                        let addr = pop!(u32);
                        self.memory.set(addr as usize, *mem, v as i16)?;
                    }
                    x3e_i64_store32(mem) => {
                        let v = pop!(i64);
                        let addr = pop!(u32);
                        self.memory.set(addr as usize, *mem, v as i32)?;
                    }
                    x41_i32_const(i) => f.stack.push(Value::I32(*i)),
                    x42_i64_const(val) => f.stack.push(Value::I64(*val)),
                    x43_f32_const(val) => f.stack.push(Value::F32(*val)),
                    x44_f64_const(val) => f.stack.push(Value::F64(*val)),
                    x45_i32_eqz => {
                        let val = pop!(i32);
                        f.stack.push(Value::I32((val == 0) as i32));
                    }
                    x46_i32_eq => {
                        let y = pop!(i32);
                        let x = pop!(i32);
                        f.stack.push(Value::I32((x == y) as i32))
                    }
                    x47_i32_ne => {
                        let y = pop!(i32);
                        let x = pop!(i32);
                        f.stack.push(Value::I32((x != y) as i32))
                    }
                    x48_i32_lt_s => {
                        let y = pop!(i32);
                        let x = pop!(i32);
                        f.stack.push(Value::I32((x < y) as i32))
                    }
                    x49_i32_lt_u => {
                        let y = pop!(u32);
                        let x = pop!(u32);
                        f.stack.push(Value::I32((x < y) as i32))
                    }
                    x4a_i32_gt_s => {
                        let y = pop!(i32);
                        let x = pop!(i32);
                        f.stack.push(Value::I32((x > y) as i32))
                    }
                    x4b_i32_gt_u => {
                        let y = pop!(u32);
                        let x = pop!(u32);
                        f.stack.push(Value::I32((y > x) as i32))
                    }
                    x4d_i32_le_u => {
                        let y = pop!(u32);
                        let x = pop!(u32);
                        f.stack.push(Value::I32((x <= y) as i32))
                    }
                    x4e_i32_ge_s => {
                        let y = pop!(i32);
                        let x = pop!(i32);
                        f.stack.push(Value::I32((x >= y) as i32))
                    }
                    x4f_i32_ge_u => {
                        let y = pop!(u32);
                        let x = pop!(u32);
                        f.stack.push(Value::I32((x >= y) as i32))
                    }
                    x52_i64_ne => {
                        let y = pop!(i64);
                        let x = pop!(i64);
                        f.stack.push(Value::I64((x != y) as i64))
                    }
                    x6a_i32_add => {
                        let y = pop!(i32);
                        let x = pop!(i32);
                        f.stack.push(Value::I32(x + y))
                    }
                    x6b_i32_sub => {
                        let y = pop!(i32);
                        let x = pop!(i32);
                        f.stack.push(Value::I32(x - y))
                    }
                    x6c_i32_mul => {
                        let y = pop!(i32);
                        let x = pop!(i32);
                        f.stack.push(Value::I32(x * y))
                    }
                    x71_i32_and => {
                        let y = pop!(i32);
                        let x = pop!(i32);
                        f.stack.push(Value::I32(x & y))
                    }
                    x72_i32_or => {
                        let y = pop!(i32);
                        let x = pop!(i32);
                        f.stack.push(Value::I32(x | y))
                    }
                    x73_i32_xor => {
                        let y = pop!(i32);
                        let x = pop!(i32);
                        f.stack.push(Value::I32(x ^ y))
                    }
                    x74_i32_shl => {
                        let y = pop!(i32);
                        let x = pop!(i32);
                        f.stack.push(Value::I32(x << y))
                    }
                    x7e_i64_mul => {
                        let y = pop!(i64);
                        let x = pop!(i64);
                        f.stack.push(Value::I64(x * y))
                    }
                    xad_i64_extend_i32_u => {
                        let x = pop!(u32) as u64;
                        f.stack
                            .push(Value::I64(unsafe { std::mem::transmute::<u64, i64>(x) }))
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
    }
}
