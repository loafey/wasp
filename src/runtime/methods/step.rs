use super::super::{
    clean_model::Function,
    error::{RuntimeError, RuntimeError::*},
    DepthValue, Frame, Runtime, Value,
};
use crate::parser::{
    BlockType, DataIdx, ElemIdx, Expr, FuncIdx, GlobalIdX, Instr::*, LabelIdX, LocalIdX, MemArg,
    TableIdX, TypeIdX, BT,
};
use std::collections::HashMap;

macro_rules! gen_macros {
    ($f:expr) => {
        let f = $f;
        macro_rules! set_local {
            ($index:expr, $v:expr) => {{
                f.locals.insert($index, $v);
            }};
        }

        macro_rules! local {
            ($index:expr) => {
                unwrap!(f.locals.get($index), MissingLocal)
            };
            (i32, $index:expr) => {{
                let val = match unwrap!(f.locals.get($index), MissingLocal) {
                    Value::I32(val) => val,
                    x => throw!(|a, b, c| WrongType(a, "i32", x.as_str(), b, c)),
                };
                val
            }};
            (i64, $index:expr) => {{
                let val = match unwrap!(f.locals.get($index), MissingLocal) {
                    Value::I64(val) => val,
                    x => throw!(|a, b, c| WrongType(a, "i64", x.as_str(), b, c)),
                };
                val
            }};
            (u32, $index:expr) => {{
                let val = match unwrap!(f.locals.get($index), MissingLocal) {
                    Value::I32(val) => val,
                    x => throw!(|a, b, c| WrongType(a, "u32", x.as_str(), b, c)),
                };
                unsafe { std::mem::transmute::<i32, u32>(val) }
            }};
            (u64, $index:expr) => {{
                let val = match unwrap!(f.locals.get($index), MissingLocal) {
                    Value::I64(val) => val,
                    x => throw!(|a, b, c| WrongType(a, "u64", x.as_str(), b, c)),
                };
                unsafe { std::mem::transmute::<i64, u64>(val) }
            }};
            (f32, $index:expr) => {{
                let val = match unwrap!(f.locals.get($index), MissingLocal) {
                    Value::F32(val) => val,
                    x => throw!(|a, b, c| WrongType(a, "f32", x.as_str(), b, c)),
                };
                val
            }};
            (f64, $index:expr) => {{
                let val = match unwrap!(f.locals.get($index), MissingLocal) {
                    Value::F64(val) => val,
                    x => throw!(|a, b, c| WrongType(a, "f64", x.as_str(), b, c)),
                };
                val
            }};
        }

        macro_rules! peek {
            () => {
                f.stack.last()
            };
        }

        macro_rules! pop {
            (i32) => {{
                let val = match unwrap!(f.stack.pop(), EmptyStack) {
                    Value::I32(val) => val,
                    x => throw!(|a, b, c| WrongType(a, "i32", x.as_str(), b, c)),
                };
                val
            }};
            (i64) => {{
                let val = match unwrap!(f.stack.pop(), EmptyStack) {
                    Value::I64(val) => val,
                    x => throw!(|a, b, c| WrongType(a, "i64", x.as_str(), b, c)),
                };
                val
            }};
            (u32) => {{
                let val = match unwrap!(f.stack.pop(), EmptyStack) {
                    Value::I32(val) => val,
                    x => throw!(|a, b, c| WrongType(a, "u32", x.as_str(), b, c)),
                };
                unsafe { std::mem::transmute::<i32, u32>(val) }
            }};
            (u64) => {{
                let val = match unwrap!(f.stack.pop(), EmptyStack) {
                    Value::I64(val) => val,
                    x => throw!(|a, b, c| WrongType(a, "u64", x.as_str(), b, c)),
                };
                unsafe { std::mem::transmute::<i64, u64>(val) }
            }};
            (f32) => {{
                let val = match unwrap!(f.stack.pop(), EmptyStack) {
                    Value::F32(val) => val,
                    x => throw!(|a, b, c| WrongType(a, "f32", x.as_str(), b, c)),
                };
                val
            }};
            (f64) => {{
                let val = match unwrap!(f.stack.pop(), EmptyStack) {
                    Value::F64(val) => val,
                    x => throw!(|a, b, c| WrongType(a, "f64", x.as_str(), b, c)),
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

        macro_rules! get {
            (stack) => {
                &f.stack
            };

            (depth_stack) => {
                &f.depth_stack
            };

            (func_id) => {
                &f.func_id
            };

            (pc) => {
                &f.pc
            };
        }

        macro_rules! set {
            (pc) => {
                f.pc
            };
        }

        macro_rules! push {
            ($v:expr) => {
                f.stack.push($v)
            };
            (i32,$v:expr) => {
                f.stack.push(Value::I32($v))
            };
            (i64,$v:expr) => {
                f.stack.push(Value::I64($v))
            };
            (u32,$v:expr) => {
                f.stack
                    .push(Value::I32(unsafe { std::mem::transmute::<u32, i32>($v) }))
            };
            (u64,$v:expr) => {
                f.stack
                    .push(Value::I64(unsafe { std::mem::transmute::<u64, i64>($v) }))
            };
            (f32,$v:expr) => {
                f.stack.push(Value::F32($v))
            };
            (f64,$v:expr) => {
                f.stack.push(Value::F64($v))
            };
        }

        macro_rules! pop_depth {
            () => {
                f.depth_stack.pop()
            };
        }

        macro_rules! push_depth {
            ($v:expr) => {
                f.depth_stack.push($v)
            };
        }
    };
}

impl Runtime {
    pub fn step(&mut self) -> Result<(), RuntimeError> {
        // print!("{} ", self.stack.len());
        macro_rules! unwrap {
            ($expr:expr, $err:expr) => {
                $expr.ok_or($err(file!(), line!(), column!()))?
            };
        }
        gen_macros!(unwrap!(self.stack.last_mut(), NoFrame));
        // println!("{:?}", get!(stack));
        // println!("{:?}", get!(depth_stack));
        // println!();

        match &self.module.functions[get!(func_id)] {
            Function::Import { module, name, .. } => {
                // println!();
                // println!("calling {module:?}::{name:?}");
                match (&*module.0, &*name.0) {
                    #[allow(clippy::print_stdout)]
                    ("console", "log") => {
                        let y = *local!(i32, &0) as usize;
                        let x = *local!(i32, &1) as usize;
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
                        push!(i32, std::env::args().count() as i32);
                        let s = std::env::args_os().map(|s| s.len() + 1).sum::<usize>() as i32;
                        push!(i32, s);
                        push!(i32, 0);
                    }
                    ("wasi_snapshot_preview1", "proc_exit") => {
                        let x = *local!(i32, &0);
                        return Err(Exit(x));
                    }
                    ("wasi_snapshot_preview1", "args_get") => {
                        let _ = *local!(i32, &0);
                        let _ = *local!(i32, &0);
                        push!(i32, 0);
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
                // println!("{ty:?}");
                if *get!(pc) >= code.len() {
                    let mut frame = unwrap!(self.stack.pop(), NoFrame);
                    let last = unwrap!(self.stack.last_mut(), NoFrame);
                    let mut col = Vec::new();
                    for _ in 0..ty.output.types.len() {
                        col.push(unwrap!(frame.stack.pop(), EmptyStack));
                    }
                    col.reverse();
                    last.stack.append(&mut col);
                    return Ok(());
                }
                let mut instr = &code[*get!(pc)];
                instr = if let comment(_, r) = instr { r } else { instr };
                let instr = instr;
                // println!("{instr:?}");
                set!(pc) += 1;
                match instr {
                    x00_unreachable => {
                        throw!(Unreachable)
                    }
                    x01_nop => (),
                    x02_block(_, _) => throw!(Impossible),
                    x03_loop(_, _) => throw!(Impossible),
                    x04_if_else(_, _, _) => throw!(Impossible),
                    x0c_br(LabelIdX(label)) => {
                        let mut last = None;
                        // println!("{label}");
                        for _ in 0..=*label {
                            last = pop_depth!();
                        }
                        let bt = unwrap!(last, MissingJumpLabel);
                        match bt.bt {
                            BT::Loop => {
                                set!(pc) = bt.pos;
                            }
                            BT::Block => {
                                set!(pc) = bt.pos + 1;
                            }
                        }
                        for _ in 0..=*label {
                            let mut collect = Vec::new();
                            loop {
                                let p = pop!();
                                if matches!(p, Value::BlockLock) {
                                    match bt.vt {
                                        BlockType::Eps => {}
                                        BlockType::T(_) => {
                                            collect.reverse();
                                            push!(unwrap!(collect.pop(), EmptyStack));
                                        }
                                        BlockType::TypIdx(i) => {
                                            let ft = unwrap!(
                                                self.module.function_types.get(&(i as u32)),
                                                MissingFunction
                                            );
                                            for _ in &ft.output.types {
                                                push!(unwrap!(collect.pop(), EmptyStack))
                                            }
                                        }
                                    }
                                    break;
                                } else {
                                    collect.push(p);
                                }
                            }
                        }
                    }
                    x0d_br_if(LabelIdX(label)) => {
                        let val = pop!(i32);

                        if val != 0 {
                            let mut last = None;
                            if get!(depth_stack).len() as u32 == *label {
                                let mut res = Vec::new();
                                for _ in &ty.output.types {
                                    res.push(pop!());
                                }
                                res.reverse();
                                match self.stack.last_mut() {
                                    Some(s) => s.stack.append(&mut res),
                                    None => {
                                        throw!(|a, b, c| ReturnedToNoFrame(res, a, b, c))
                                    }
                                }
                            } else {
                                for _ in 0..=*label {
                                    last = pop_depth!();
                                }
                                let bt = unwrap!(last, MissingJumpLabel);
                                match bt.bt {
                                    BT::Loop => {
                                        set!(pc) = bt.pos;
                                    }
                                    BT::Block => {
                                        set!(pc) = bt.pos + 1;
                                    }
                                }
                                for _ in 0..=*label {
                                    let mut collect = Vec::new();
                                    loop {
                                        let p = pop!();
                                        if matches!(p, Value::BlockLock) {
                                            match bt.vt {
                                                BlockType::Eps => {
                                                    break;
                                                }
                                                BlockType::T(_) => {
                                                    collect.reverse();
                                                    push!(unwrap!(collect.pop(), EmptyStack));
                                                    break;
                                                }
                                                BlockType::TypIdx(_) => todo!(),
                                            }
                                        } else {
                                            collect.push(p);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    x0e_br_table(labels, def) => {
                        let index = pop!(u32) as usize;
                        let label = if index >= labels.len() {
                            *def
                        } else {
                            labels[index]
                        };

                        if get!(depth_stack).len() as u32 == *label {
                            let mut res = Vec::new();
                            for _ in &ty.output.types {
                                res.push(pop!());
                            }
                            res.reverse();
                            match self.stack.last_mut() {
                                Some(s) => s.stack.append(&mut res),
                                None => throw!(|a, b, c| ReturnedToNoFrame(res, a, b, c)),
                            }
                        } else {
                            let mut last = None;
                            for _ in 0..=*label {
                                last = pop_depth!();
                            }
                            let bt = unwrap!(last, MissingJumpLabel);
                            match bt.bt {
                                BT::Loop => {
                                    set!(pc) = bt.pos;
                                }
                                BT::Block => {
                                    set!(pc) = bt.pos + 1;
                                }
                            }
                            for _ in 0..=*label {
                                let mut collect = Vec::new();
                                loop {
                                    let p = pop!();
                                    if matches!(p, Value::BlockLock) {
                                        match bt.vt {
                                            BlockType::Eps => {}
                                            BlockType::T(_) => {
                                                push!(collect.remove(0))
                                            }
                                            BlockType::TypIdx(_) => todo!(),
                                        }
                                        break;
                                    } else {
                                        collect.push(p);
                                    }
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
                            let value = unwrap!(last_f.stack.pop(), EmptyStack);
                            res.push(value)
                        }
                        res.reverse();
                        match self.stack.last_mut() {
                            Some(s) => s.stack.append(&mut res),
                            None => {
                                throw!(|a, b, c| ReturnedToNoFrame(res, a, b, c))
                            }
                        }
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
                        for (i, _) in ty.input.types.iter().enumerate().rev() {
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
                        for (i, _) in ty.input.types.iter().enumerate().rev() {
                            locals.insert(i as u32, pop!());
                        }

                        let table = &self.module.tables[*table_index as usize];
                        // println!(
                        //     "Call info ({}): \n\tinputs: {locals:?}\n\tfunction_index: {function_index}",
                        //     f.func_id
                        // );

                        let FuncIdx(id) =
                            unwrap!(table.get(&(function_index as u32)), UninitializedElement);
                        if *id == u32::MAX {
                            throw!(UninitializedElement)
                        }

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
                        match cond == 0 {
                            true => push!(y),
                            false => push!(x),
                        }
                    }
                    x20_local_get(LocalIdX(id)) => push!(*local!(id)),
                    x21_local_set(LocalIdX(id)) => {
                        let val = pop!();
                        set_local!(*id, val);
                    }
                    x22_local_tee(LocalIdX(id)) => {
                        let last = pop!();
                        set_local!(*id, last);
                        push!(last);
                    }
                    x23_global_get(GlobalIdX(id)) => {
                        push!(self.globals.get(id).copied().unwrap_or(Value::I32(0)))
                    }
                    x24_global_set(GlobalIdX(id)) => {
                        let pop = pop!();
                        self.globals.insert(*id, pop);
                    }
                    x28_i32_load(mem) => {
                        let addr = pop!(u32);
                        push!(i32, self.memory.get(addr as usize, *mem)?);
                    }
                    x29_i64_load(mem) => {
                        let addr = pop!(u32);
                        push!(i64, self.memory.get(addr as usize, *mem)?);
                    }
                    x2a_f32_load(mem) => {
                        let addr = pop!(u32);
                        push!(f32, self.memory.get(addr as usize, *mem)?);
                    }
                    x2b_f64_load(mem) => {
                        let addr = pop!(u32);
                        push!(f64, self.memory.get(addr as usize, *mem)?);
                    }
                    x2c_i32_load8_s(mem) => {
                        let addr = pop!(u32);
                        push!(i32, self.memory.get::<i8>(addr as usize, *mem)? as i32);
                    }
                    x2d_i32_load8_u(mem) => {
                        let addr = pop!(u32);
                        push!(u32, self.memory.get::<u8>(addr as usize, *mem)? as u32);
                    }
                    x2e_i32_load16_s(mem) => {
                        let addr = pop!(u32);
                        push!(i32, self.memory.get::<u16>(addr as usize, *mem)? as i32);
                    }
                    x2f_i32_load16_u(mem) => {
                        let addr = pop!(u32);
                        push!(u32, self.memory.get::<u16>(addr as usize, *mem)? as u32);
                    }
                    x30_i64_load8_s(mem) => {
                        let addr = pop!(u32);
                        push!(i64, self.memory.get::<i8>(addr as usize, *mem)? as i64);
                    }
                    x31_i64_load8_u(mem) => {
                        let addr = pop!(u32);
                        push!(u64, self.memory.get::<u8>(addr as usize, *mem)? as u64);
                    }
                    x32_i64_load16_s(mem) => {
                        let addr = pop!(u32);
                        push!(i64, self.memory.get::<i16>(addr as usize, *mem)? as i64);
                    }
                    x33_i64_load16_u(mem) => {
                        let addr = pop!(u32);
                        push!(u64, self.memory.get::<u16>(addr as usize, *mem)? as u64);
                    }
                    x34_i64_load32_s(mem) => {
                        let addr = pop!(u32);
                        push!(i64, self.memory.get::<i32>(addr as usize, *mem)? as i64);
                    }
                    x35_i64_load32_u(mem) => {
                        let addr = pop!(u32);
                        push!(u64, self.memory.get::<u32>(addr as usize, *mem)? as u64);
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
                    x40_grow => {
                        let amount = pop!(i32);
                        push!(i32, self.memory.grow(amount as usize))
                    }
                    x41_i32_const(val) => push!(i32, *val),
                    x42_i64_const(val) => push!(i64, *val),
                    x43_f32_const(val) => push!(f32, *val),
                    x44_f64_const(val) => push!(f64, *val),
                    x45_i32_eqz => {
                        let val = pop!(i32);
                        push!(i32, (val == 0) as i32);
                    }
                    x46_i32_eq => {
                        let y = pop!(i32);
                        let x = pop!(i32);
                        push!(i32, (x == y) as i32)
                    }
                    x47_i32_ne => {
                        let y = pop!(i32);
                        let x = pop!(i32);
                        push!(i32, (x != y) as i32)
                    }
                    x48_i32_lt_s => {
                        let y = pop!(i32);
                        let x = pop!(i32);
                        push!(i32, (x < y) as i32)
                    }
                    x49_i32_lt_u => {
                        let y = pop!(u32);
                        let x = pop!(u32);
                        push!(i32, (x < y) as i32)
                    }
                    x4a_i32_gt_s => {
                        let y = pop!(i32);
                        let x = pop!(i32);
                        push!(i32, (x > y) as i32)
                    }
                    x4b_i32_gt_u => {
                        let y = pop!(u32);
                        let x = pop!(u32);
                        push!(i32, (y > x) as i32)
                    }
                    x4d_i32_le_u => {
                        let y = pop!(u32);
                        let x = pop!(u32);
                        push!(i32, (x <= y) as i32)
                    }
                    x4e_i32_ge_s => {
                        let y = pop!(i32);
                        let x = pop!(i32);
                        push!(i32, (x >= y) as i32)
                    }
                    x4f_i32_ge_u => {
                        let y = pop!(u32);
                        let x = pop!(u32);
                        push!(i32, (x >= y) as i32)
                    }
                    x50_i64_eqz => {
                        let val = pop!(i64);
                        push!(i32, (val == 0) as i32);
                    }
                    x52_i64_ne => {
                        let y = pop!(i64);
                        let x = pop!(i64);
                        push!(i64, (x != y) as i64)
                    }
                    x5e_f32_gt => {
                        let y = pop!(f32);
                        let x = pop!(f32);
                        push!(i32, (x < y) as i32)
                    }
                    x68_i32_ctz => {
                        let x = pop!(i32);
                        push!(i32, x.trailing_zeros() as i32)
                    }
                    x6a_i32_add => {
                        let y = pop!(i32);
                        let x = pop!(i32);
                        push!(i32, x + y)
                    }
                    x6b_i32_sub => {
                        let y = pop!(i32);
                        let x = pop!(i32);
                        push!(i32, x - y)
                    }
                    x6c_i32_mul => {
                        let y = pop!(i32);
                        let x = pop!(i32);
                        push!(i32, x * y)
                    }
                    x71_i32_and => {
                        let y = pop!(i32);
                        let x = pop!(i32);
                        push!(i32, x & y)
                    }
                    x72_i32_or => {
                        let y = pop!(i32);
                        let x = pop!(i32);
                        push!(i32, x | y)
                    }
                    x73_i32_xor => {
                        let y = pop!(i32);
                        let x = pop!(i32);
                        push!(i32, x ^ y)
                    }
                    x74_i32_shl => {
                        let y = pop!(i32);
                        let x = pop!(i32);
                        push!(i32, x << y)
                    }
                    x7d_i64_sub => {
                        let y = pop!(i64);
                        let x = pop!(i64);
                        push!(i64, x - y)
                    }
                    x7e_i64_mul => {
                        let y = pop!(i64);
                        let x = pop!(i64);
                        push!(i64, x * y)
                    }
                    xad_i64_extend_i32_u => {
                        let x = pop!(u32) as u64;
                        push!(u64, x)
                    }
                    xfc_8_memory_init(DataIdx(i), _) => {
                        let amount = pop!(i32) as usize;
                        let source = pop!(i32) as usize;
                        let destination = pop!(i32) as usize;
                        let val = unwrap!(self.datas.get(i), MissingData);
                        if source + amount > val.len() {
                            throw!(DataInitOutOfRange)
                        }
                        self.memory
                            .slice_write(destination, &val[source..source + amount])?
                    }
                    xfc_9_data_drop(DataIdx(i)) => {
                        self.datas.insert(*i, Vec::new());
                    }
                    xfc_10_memory_copy(_, _) => {
                        let amount = pop!(i32) as usize;
                        let source = pop!(i32) as usize;
                        let destination = pop!(i32) as usize;
                        // println!("amount: {amount}, source: {source}, dest: {destination}");
                        self.memory.copy(source, amount, destination)?;
                    }
                    xfc_11_memory_fill(_) => {
                        let amount = pop!(i32) as usize;
                        let val = pop!(i32) as u8;
                        let ptr = pop!(i32) as usize;
                        self.memory.bulk_write(ptr, amount, val)?;
                    }
                    xfc_12_table_init(ElemIdx(e), TableIdX(t)) => {
                        let amount = pop!(i32) as u32;
                        let source = pop!(i32) as u32;
                        let destination = pop!(i32) as u32;
                        let elems = unwrap!(self.module.elems.get(e), MissingElementIndex);
                        let table =
                            unwrap!(self.module.tables.get_mut(*t as usize), MissingTableIndex);

                        let check_1 = source + amount > elems.instrs.len() as u32;
                        let check_2 = destination < table.table_length.0 as u32;
                        let check_3 = destination + amount > table.table_length.1 as u32;
                        if check_1 || check_2 || check_3 {
                            throw!(OutOfBoundsTableAccess)
                        }

                        for i in 0..amount {
                            // should crash here
                            let e = match elems.instrs[(i + source) as usize] {
                                x41_i32_const(i) => FuncIdx(i as u32),
                                _ => todo!(),
                            };
                            table.insert(i + destination, e);
                        }
                    }
                    xfc_13_elem_drop(ElemIdx(i)) => {
                        self.module.elems.insert(*i, Expr { instrs: Vec::new() });
                    }
                    xfc_14_table_copy(TableIdX(a), TableIdX(b)) => {
                        let amount = pop!(i32) as u32;
                        let source = pop!(i32) as u32;
                        let destination = pop!(i32) as u32;

                        let a = unwrap!(self.module.tables.get(*a as usize), MissingTableIndex);
                        let check_1 = source + amount > a.len() as u32;
                        let mut clones = Vec::new();
                        for i in 0..amount {
                            let index = i + source;
                            let f = unwrap!(a.get(&index), OutOfBoundsTableAccess);
                            clones.push(*f);
                        }

                        let b = unwrap!(self.module.tables.get_mut(*b as usize), MissingTableIndex);
                        let check_2 = destination < b.table_length.0 as u32;
                        let check_3 = destination + amount > b.table_length.1 as u32;

                        if check_1 || check_2 || check_3 {
                            throw!(OutOfBoundsTableAccess)
                        }

                        for (i, v) in clones.into_iter().enumerate() {
                            let index = i as u32 + destination;
                            b.insert(index, v);
                        }
                    }
                    block_start(bt, be, vt) => {
                        // println!("block_start: {vt:?}");
                        let mut to_push = Vec::new();
                        if matches!(bt, BT::Block) {
                            match vt {
                                BlockType::Eps => {}
                                BlockType::T(_) => {}
                                BlockType::TypIdx(t) => {
                                    for _ in unwrap!(
                                        self.module.function_types.get(&(*t as u32)),
                                        MissingFunction
                                    )
                                    .input
                                    .types
                                    .iter()
                                    {
                                        to_push.push(pop!());
                                    }
                                }
                            };
                        }
                        push!(Value::BlockLock);
                        match bt {
                            BT::Block => push_depth!(DepthValue {
                                bt: *bt,
                                pos: *be,
                                vt: *vt,
                            }),
                            BT::Loop => push_depth!(DepthValue {
                                bt: *bt,
                                pos: get!(pc) - 1,
                                vt: *vt,
                            }),
                        }
                        for p in to_push {
                            push!(p);
                        }
                    }
                    block_end(_, _, bt) => {
                        // println!("block_end: {bt:?}");
                        let mut last = Vec::new();
                        loop {
                            if let Some(Value::BlockLock) = peek!() {
                                pop!();
                                break;
                            }
                            last.push(pop!());
                        }
                        match bt {
                            BlockType::Eps => {}
                            BlockType::T(_) => match last.last() {
                                Some(p) => push!(*p),
                                None => throw!(EmptyStack),
                            },
                            BlockType::TypIdx(t) => {
                                let func = unwrap!(
                                    self.module.function_types.get(&(*t as u32)),
                                    MissingFunction
                                );
                                for _ in 0..func.output.types.len() {
                                    push!(unwrap!(last.pop(), EmptyStack));
                                }
                            }
                        }
                        pop_depth!();
                    }
                    if_then_else(jump_index) => {
                        let val = pop!(i32);

                        if val != 0 {
                            set!(pc) = *jump_index;
                        }
                        // for (i, c) in code.iter().enumerate() {
                        //     println!("{i}:\t{c:?}");
                        // }
                    }
                    else_jump(jump_index) => {
                        // for (i, c) in code.iter().enumerate() {
                        //     println!("{i}:\t{c:?}");
                        // }
                        set!(pc) = *jump_index;
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
