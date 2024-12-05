use super::super::{
    clean_model::Function,
    error::{RuntimeError, RuntimeError::*},
    DepthValue, Frame, Runtime, Value,
};
use crate::{
    parser::{
        BlockType, DataIdx, ElemIdx, Expr, FuncIdx, GlobalIdX, Instr::*, LabelIdX, LocalIdX,
        RefTyp, TableIdX, TypeIdX, BT,
    },
    runtime::{clean_model::Table, FloatExp, FuncId},
};
use core::f64;
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

            (module) => {
                &f.module
            };

            (locals) => {
                &f.locals
            };

            (module) => {
                &f.module
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
            (stack) => {
                f.stack
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
        if self.stack.len() > u16::MAX as usize {
            return Err(StackExhaustion(self.stack.len(), u16::MAX as usize));
        }

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

        let (code, ty, module) = {
            let func_id = get!(func_id).clone();
            let (module, function) = match func_id {
                FuncId::Id(id) => (
                    unsafe {
                        unwrap!(self.modules.get(get!(module)), |a, b, c| NoModule(
                            get!(module).clone(),
                            a,
                            b,
                            c
                        ))
                        .as_ws()
                    },
                    id,
                ),
                FuncId::Foreign { module, id } => todo!("foreign call {module}::({id})"),
            };
            let ptr = unwrap!(module.functions.get(function as usize), MissingFunction);
            match ptr.as_ref() {
                Function::WS { ty, code, .. } => (code, ty, module),
                Function::IO { func, .. } => {
                    let Frame { locals, .. } = unwrap!(self.stack.pop(), NoFrame);
                    let res = func(&locals, &mut module.memory.write())?;
                    let frame = unwrap!(self.stack.last_mut(), NoFrame);
                    for v in res {
                        frame.stack.push(v);
                    }
                    return Ok(());
                }
            }
        };

        // Execute
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
                                        module.function_types.get(i as usize),
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
                let func_id = match &last_f.func_id {
                    FuncId::Id(id) => id,
                    FuncId::Foreign { .. } => todo!(),
                };
                let ty = unwrap!(module.functions.get(*func_id as usize), MissingFunction);
                let Function::WS { ty, .. } = ty.as_ref() else {
                    panic!("return in IO :(")
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
                let fun = &module.functions[*id as usize];
                let ty = match fun.as_ref() {
                    Function::WS { ty, .. } => ty,
                    Function::IO { ty, .. } => ty,
                };
                let (ty, module, func_id) = (ty, get!(module).clone(), FuncId::Id(*id));

                let mut locals = HashMap::new();
                for (i, _) in ty.input.types.iter().enumerate().rev() {
                    locals.insert(i as u32, pop!());
                }

                self.stack.push(Frame {
                    func_id,
                    pc: 0,
                    module,
                    stack: Vec::new(),
                    locals,
                    depth_stack: Vec::new(),
                });
            }
            x11_call_indirect(TypeIdX(type_index), TableIdX(table_index)) => {
                let function_index = pop!(i32);

                let ty = unwrap!(
                    module.function_types.get(*type_index as usize),
                    MissingFunction
                );

                let mut locals = HashMap::new();
                for (i, _) in ty.input.types.iter().enumerate().rev() {
                    locals.insert(i as u32, pop!());
                }

                let table = module.tables[*table_index as usize].read();
                // println!(
                //     "Call info ({}): \n\tinputs: {locals:?}\n\tfunction_index: {function_index}",
                //     f.func_id
                // );
                let Table { table, .. } = &*table;
                let FuncIdx(id) = *unwrap!(table.get(&(function_index as u32)), UndefinedElement);
                if id == u32::MAX {
                    throw!(UninitializedElement)
                }

                if let Some(func) = module.functions.get(id as usize) {
                    let ty2 = match func.as_ref() {
                        Function::WS { ty, .. } => ty,
                        Function::IO { ty, .. } => ty,
                    };
                    if ty.as_ref() != ty2 {
                        throw!(IndirectCallTypeMismatch)
                    }
                }

                let module = get!(module).clone();
                self.stack.push(Frame {
                    func_id: FuncId::Id(id),
                    pc: 0,
                    stack: Vec::new(),
                    module,
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
                let global = unwrap!(module.globals.get(*id as usize), MissingGlobal).read();
                push!(global.1);
            }
            x24_global_set(GlobalIdX(id)) => {
                let pop = pop!();
                let Some(r) = module.globals.get(*id as usize) else {
                    unreachable!()
                };
                r.write().1 = pop;
            }
            x28_i32_load(mem) => {
                let addr = pop!(u32);
                push!(i32, module.memory.read().get(addr as usize, *mem)?);
            }
            x29_i64_load(mem) => {
                let addr = pop!(u32);
                push!(i64, module.memory.read().get(addr as usize, *mem)?);
            }
            x2a_f32_load(mem) => {
                let addr = pop!(u32);
                push!(f32, module.memory.read().get(addr as usize, *mem)?);
            }
            x2b_f64_load(mem) => {
                let addr = pop!(u32);
                push!(f64, module.memory.read().get(addr as usize, *mem)?);
            }
            x2c_i32_load8_s(mem) => {
                let addr = pop!(u32);
                push!(
                    i32,
                    module.memory.read().get::<i8>(addr as usize, *mem)? as i32
                );
            }
            x2d_i32_load8_u(mem) => {
                let addr = pop!(u32);
                push!(
                    u32,
                    module.memory.read().get::<u8>(addr as usize, *mem)? as u32
                );
            }
            x2e_i32_load16_s(mem) => {
                let addr = pop!(u32);
                push!(
                    i32,
                    module.memory.read().get::<u16>(addr as usize, *mem)? as i32
                );
            }
            x2f_i32_load16_u(mem) => {
                let addr = pop!(u32);
                push!(
                    u32,
                    module.memory.read().get::<u16>(addr as usize, *mem)? as u32
                );
            }
            x30_i64_load8_s(mem) => {
                let addr = pop!(u32);
                push!(
                    i64,
                    module.memory.read().get::<i8>(addr as usize, *mem)? as i64
                );
            }
            x31_i64_load8_u(mem) => {
                let addr = pop!(u32);
                push!(
                    u64,
                    module.memory.read().get::<u8>(addr as usize, *mem)? as u64
                );
            }
            x32_i64_load16_s(mem) => {
                let addr = pop!(u32);
                push!(
                    i64,
                    module.memory.read().get::<i16>(addr as usize, *mem)? as i64
                );
            }
            x33_i64_load16_u(mem) => {
                let addr = pop!(u32);
                push!(
                    u64,
                    module.memory.read().get::<u16>(addr as usize, *mem)? as u64
                );
            }
            x34_i64_load32_s(mem) => {
                let addr = pop!(u32);
                push!(
                    i64,
                    module.memory.read().get::<i32>(addr as usize, *mem)? as i64
                );
            }
            x35_i64_load32_u(mem) => {
                let addr = pop!(u32);
                push!(
                    u64,
                    module.memory.read().get::<u32>(addr as usize, *mem)? as u64
                );
            }
            x36_i32_store(mem) => {
                let v = pop!(i32);
                let addr = pop!(u32);
                module.memory.write().set(addr as usize, *mem, v)?;
            }
            x37_i64_store(mem) => {
                let v = pop!(i64);
                let addr = pop!(u32);
                module.memory.write().set(addr as usize, *mem, v)?;
            }
            x38_f32_store(mem) => {
                let v = pop!(f32);
                let addr = pop!(u32);
                module.memory.write().set(addr as usize, *mem, v)?;
            }
            x39_f64_store(mem) => {
                let v = pop!(f64);
                let addr = pop!(u32);
                module.memory.write().set(addr as usize, *mem, v)?;
            }
            x3a_i32_store8(mem) => {
                let v = pop!(i32);
                let addr = pop!(u32);
                module.memory.write().set(addr as usize, *mem, v as i8)?;
            }
            x3b_i32_store16(mem) => {
                let v = pop!(i32);
                let addr = pop!(u32);
                module.memory.write().set(addr as usize, *mem, v as i16)?;
            }
            x3c_i64_store8(mem) => {
                let v = pop!(i64);
                let addr = pop!(u32);
                module.memory.write().set(addr as usize, *mem, v as i8)?;
            }
            x3d_i64_store16(mem) => {
                let v = pop!(i64);
                let addr = pop!(u32);
                module.memory.write().set(addr as usize, *mem, v as i16)?;
            }
            x3e_i64_store32(mem) => {
                let v = pop!(i64);
                let addr = pop!(u32);
                module.memory.write().set(addr as usize, *mem, v as i32)?;
            }
            x40_grow => {
                let amount = pop!(i32);
                push!(i32, module.memory.write().grow(amount as usize))
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
                push!(i32, (x > y) as i32)
            }
            x4c_i32_le_s => {
                let y = pop!(i32);
                let x = pop!(i32);
                push!(i32, (x <= y) as i32)
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
            x51_i64_eq => {
                let y = pop!(i64);
                let x = pop!(i64);
                push!(i32, (x == y) as i32)
            }
            x52_i64_ne => {
                let y = pop!(i64);
                let x = pop!(i64);
                push!(i32, (x != y) as i32)
            }
            x53_i64_lt_s => {
                let y = pop!(i64);
                let x = pop!(i64);
                push!(i32, (x < y) as i32)
            }
            x54_i64_lt_u => {
                let y = pop!(u64);
                let x = pop!(u64);
                push!(i32, (x < y) as i32)
            }
            x55_i64_gt_s => {
                let y = pop!(i64);
                let x = pop!(i64);
                push!(i32, (x > y) as i32)
            }
            x56_i64_gt_u => {
                let y = pop!(u64);
                let x = pop!(u64);
                push!(i32, (x > y) as i32)
            }
            x57_i64_le_s => {
                let y = pop!(i64);
                let x = pop!(i64);
                push!(i32, (x <= y) as i32)
            }
            x58_i64_le_u => {
                let y = pop!(u64);
                let x = pop!(u64);
                push!(i32, (x <= y) as i32)
            }
            x59_i64_ge_s => {
                let y = pop!(i64);
                let x = pop!(i64);
                push!(i32, (x >= y) as i32)
            }
            x5a_i64_ge_u => {
                let y = pop!(u64);
                let x = pop!(u64);
                push!(i32, (x >= y) as i32)
            }
            x5b_f32_eq => {
                let y = pop!(f32);
                let x = pop!(f32);
                push!(i32, (x == y) as i32)
            }
            x5c_f32_ne => {
                let y = pop!(f32);
                let x = pop!(f32);
                push!(i32, (x != y) as i32)
            }
            x5d_f32_lt => {
                let y = pop!(f32);
                let x = pop!(f32);
                push!(i32, (x < y) as i32)
            }
            x5e_f32_gt => {
                let y = pop!(f32);
                let x = pop!(f32);
                push!(i32, (x > y) as i32)
            }
            x5f_f32_le => {
                let y = pop!(f32);
                let x = pop!(f32);
                push!(i32, (x <= y) as i32)
            }
            x60_f32_ge => {
                let y = pop!(f32);
                let x = pop!(f32);
                push!(i32, (x >= y) as i32)
            }
            x61_f64_eq => {
                let y = pop!(f64);
                let x = pop!(f64);
                push!(i32, (x == y) as i32)
            }
            x62_f64_ne => {
                let y = pop!(f64);
                let x = pop!(f64);
                push!(i32, (x != y) as i32)
            }
            x63_f64_lt => {
                let y = pop!(f64);
                let x = pop!(f64);
                push!(i32, (x < y) as i32)
            }
            x64_f64_gt => {
                let y = pop!(f64);
                let x = pop!(f64);
                push!(i32, (x > y) as i32)
            }
            x65_f64_le => {
                let y = pop!(f64);
                let x = pop!(f64);
                push!(i32, (x <= y) as i32)
            }
            x66_f64_ge => {
                let y = pop!(f64);
                let x = pop!(f64);
                push!(i32, (x >= y) as i32)
            }
            x67_i32_clz => {
                let x = pop!(i32);
                push!(u32, x.leading_zeros())
            }
            x68_i32_ctz => {
                let x = pop!(i32);
                push!(u32, x.trailing_zeros())
            }
            x69_i32_popcnt => {
                let x = pop!(i32);
                push!(u32, x.count_ones())
            }
            x6a_i32_add => {
                let y = pop!(i32);
                let x = pop!(i32);
                push!(i32, x.wrapping_add(y))
            }
            x6b_i32_sub => {
                let y = pop!(i32);
                let x = pop!(i32);
                push!(i32, x.wrapping_sub(y))
            }
            x6c_i32_mul => {
                let y = pop!(i32);
                let x = pop!(i32);
                push!(i32, x.wrapping_mul(y))
            }
            x6d_i32_div_s => {
                let y = pop!(i32);
                let x = pop!(i32);
                if y == 0 {
                    throw!(IntegerDivideByZero)
                }
                match x.checked_div(y) {
                    Some(res) => push!(i32, res),
                    None => throw!(IntegerOverflow),
                }
            }
            x6e_i32_div_u => {
                let y = pop!(u32);
                let x = pop!(u32);
                if y == 0 {
                    throw!(IntegerDivideByZero)
                }
                match x.checked_div(y) {
                    Some(res) => push!(u32, res),
                    None => throw!(IntegerOverflow),
                }
            }
            x6f_i32_rem_s => {
                let y = pop!(i32);
                let x = pop!(i32);
                if y == 0 {
                    throw!(IntegerDivideByZero)
                }
                push!(i32, x.wrapping_rem(y))
            }
            x70_i32_rem_u => {
                let y = pop!(u32);
                let x = pop!(u32);
                if y == 0 {
                    throw!(IntegerDivideByZero)
                }
                push!(u32, x.wrapping_rem(y))
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
                push!(i32, x.wrapping_shl(y as u32))
            }
            x75_i32_shr_s => {
                let y = pop!(i32);
                let x = pop!(i32);
                push!(i32, x.wrapping_shr(y as u32))
            }
            x76_i32_shr_u => {
                let y = pop!(u32);
                let x = pop!(u32);
                push!(u32, x.wrapping_shr(y))
            }
            x77_i32_rotl => {
                let y = pop!(u32);
                let x = pop!(u32);
                push!(u32, x.rotate_left(y))
            }
            x78_i32_rotr => {
                let y = pop!(u32);
                let x = pop!(u32);
                push!(u32, x.rotate_right(y))
            }
            x79_i64_clz => {
                let x = pop!(i64);
                push!(u64, x.leading_zeros() as u64)
            }
            x7a_i64_ctz => {
                let x = pop!(i64);
                push!(u64, x.trailing_zeros() as u64)
            }
            x7b_i64_popcnt => {
                let x = pop!(i64);
                push!(u64, x.count_ones() as u64)
            }
            x7c_i64_add => {
                let y = pop!(i64);
                let x = pop!(i64);
                push!(i64, x.wrapping_add(y))
            }
            x7d_i64_sub => {
                let y = pop!(i64);
                let x = pop!(i64);
                push!(i64, x.wrapping_sub(y))
            }
            x7e_i64_mul => {
                let y = pop!(i64);
                let x = pop!(i64);
                push!(i64, x.wrapping_mul(y))
            }
            x7f_i64_div_s => {
                let y = pop!(i64);
                let x = pop!(i64);
                if y == 0 {
                    throw!(IntegerDivideByZero)
                }
                match x.checked_div(y) {
                    Some(res) => push!(i64, res),
                    None => throw!(IntegerOverflow),
                }
            }
            x80_i64_div_u => {
                let y = pop!(u64);
                let x = pop!(u64);
                if y == 0 {
                    throw!(IntegerDivideByZero)
                }
                match x.checked_div(y) {
                    Some(res) => push!(u64, res),
                    None => throw!(IntegerOverflow),
                }
            }
            x81_i64_rem_s => {
                let y = pop!(i64);
                let x = pop!(i64);
                if y == 0 {
                    throw!(IntegerDivideByZero)
                }
                push!(i64, x.wrapping_rem(y))
            }
            x82_i64_rem_u => {
                let y = pop!(u64);
                let x = pop!(u64);
                if y == 0 {
                    throw!(IntegerDivideByZero)
                }
                push!(u64, x.wrapping_rem(y))
            }
            x83_i64_and => {
                let y = pop!(i64);
                let x = pop!(i64);
                push!(i64, x & y)
            }
            x84_i64_or => {
                let y = pop!(i64);
                let x = pop!(i64);
                push!(i64, x | y)
            }
            x85_i64_xor => {
                let y = pop!(i64);
                let x = pop!(i64);
                push!(i64, x ^ y)
            }
            x86_i64_shl => {
                let y = pop!(i64);
                let x = pop!(i64);
                push!(i64, x.wrapping_shl(y as u32))
            }
            x87_i64_shr_s => {
                let y = pop!(i64);
                let x = pop!(i64);
                push!(i64, x.wrapping_shr(y as u32))
            }
            x88_i64_shr_u => {
                let y = pop!(u64);
                let x = pop!(u64);
                push!(u64, x.wrapping_shr(y as u32))
            }
            x89_i64_rotl => {
                let y = pop!(u64);
                let x = pop!(u64);
                push!(u64, x.rotate_left(y as u32))
            }
            x8a_i64_rotr => {
                let y = pop!(u64);
                let x = pop!(u64);
                push!(u64, x.rotate_right(y as u32))
            }
            x8b_f32_abs => {
                let x = pop!(f32);
                push!(f32, x.abs())
            }
            x8c_f32_neg => {
                let x = pop!(f32);
                push!(f32, -x)
            }
            x8d_f32_ceil => {
                let x = pop!(f32);
                push!(f32, x.ceil())
            }
            x8e_f32_floor => {
                let x = pop!(f32);
                push!(f32, x.floor())
            }
            x8f_f32_trunc => {
                let x = pop!(f32);
                push!(f32, x.trunc())
            }
            x90_f32_nearest => {
                let x = pop!(f32);
                push!(f32, x.round_ties_even())
            }
            x91_f32_sqrt => {
                let x = pop!(f32);
                push!(f32, x.sqrt())
            }
            x92_f32_add => {
                let y = pop!(f32);
                let x = pop!(f32);
                push!(f32, x + y)
            }
            x93_f32_sub => {
                let y = pop!(f32);
                let x = pop!(f32);
                push!(f32, x - y)
            }
            x94_f32_mul => {
                let y = pop!(f32);
                let x = pop!(f32);
                push!(f32, x * y)
            }
            x95_f32_div => {
                let y = pop!(f32);
                let x = pop!(f32);
                push!(f32, x / y)
            }
            x96_f32_min => {
                let y = pop!(f32);
                let x = pop!(f32);

                if (y.to_bits() == 1 << 31 && x == 0.0) || (x.to_bits() == 1 << 31 && y == 0.0) {
                    push!(f32, f32::from_bits(1 << 31));
                } else if y.is_nan() || x.is_nan() {
                    push!(f32, f32::NAN_ARITHMETIC);
                } else if y.is_infinite() && y.is_sign_positive() {
                    push!(f32, x)
                } else if x.is_infinite() && x.is_sign_positive() {
                    push!(f32, y)
                } else if (y.is_infinite() && y.is_sign_negative())
                    || (x.is_infinite() && x.is_sign_negative())
                {
                    push!(f32, f32::NEG_INFINITY)
                } else {
                    push!(f32, x.min(y))
                }
            }
            x97_f32_max => {
                let y = pop!(f32);
                let x = pop!(f32);

                if (y.to_bits() == (1 << 31)) && (x.to_bits() == (1 << 31)) {
                    push!(f32, f32::from_bits(1 << 31));
                } else if (y.to_bits() == (1 << 31) && x == 0.0)
                    || (x.to_bits() == (1 << 31) && y == 0.0)
                {
                    push!(f32, 0.0);
                } else if y.is_nan() || x.is_nan() {
                    push!(f32, f32::NAN_ARITHMETIC);
                } else if y.is_infinite() && y.is_sign_negative() {
                    push!(f32, x)
                } else if x.is_infinite() && x.is_sign_negative() {
                    push!(f32, y)
                } else if (y.is_infinite() && y.is_sign_positive())
                    || (x.is_infinite() && x.is_sign_positive())
                {
                    push!(f32, f32::INFINITY)
                } else {
                    push!(f32, x.max(y))
                }
            }
            x98_f32_copysign => {
                let y = pop!(f32);
                let x = pop!(f32);
                if x.nan_sign() == y.nan_sign() {
                    push!(f32, x);
                } else {
                    push!(f32, f32::from_bits(x.to_bits() ^ (1 << 31)))
                }
            }
            x99_f64_abs => {
                let x = pop!(f64);
                push!(f64, x.abs())
            }
            x9a_f64_neg => {
                let x = pop!(f64);
                push!(f64, -x)
            }
            x9b_f64_ceil => {
                let x = pop!(f64);
                push!(f64, x.ceil())
            }
            x9c_f64_floor => {
                let x = pop!(f64);
                push!(f64, x.floor())
            }
            x9d_f64_trunc => {
                let x = pop!(f64);
                push!(f64, x.trunc())
            }
            x9e_f64_nearest => {
                let x = pop!(f64);
                push!(f64, x.round_ties_even())
            }
            x9f_f64_sqrt => {
                let x = pop!(f64);
                push!(f64, x.sqrt())
            }
            xa0_f64_add => {
                let y = pop!(f64);
                let x = pop!(f64);
                push!(f64, x + y)
            }
            xa1_f64_sub => {
                let y = pop!(f64);
                let x = pop!(f64);
                push!(f64, x - y)
            }
            xa2_f64_mul => {
                let y = pop!(f64);
                let x = pop!(f64);
                push!(f64, x * y)
            }
            xa3_f64_div => {
                let y = pop!(f64);
                let x = pop!(f64);
                push!(f64, x / y)
            }
            xa4_f64_min => {
                let y = pop!(f64);
                let x = pop!(f64);

                if (y.to_bits() == 1 << 63 && x == 0.0) || (x.to_bits() == 1 << 63 && y == 0.0) {
                    push!(f64, f64::from_bits(1 << 63));
                } else if y.is_nan() || x.is_nan() {
                    push!(f64, f64::NAN_ARITHMETIC);
                } else if y.is_infinite() && y.is_sign_positive() {
                    push!(f64, x)
                } else if x.is_infinite() && x.is_sign_positive() {
                    push!(f64, y)
                } else if (y.is_infinite() && y.is_sign_negative())
                    || (x.is_infinite() && x.is_sign_negative())
                {
                    push!(f64, f64::NEG_INFINITY)
                } else {
                    push!(f64, x.min(y))
                }
            }
            xa5_f64_max => {
                let y = pop!(f64);
                let x = pop!(f64);

                if (y.to_bits() == (1 << 63)) && (x.to_bits() == (1 << 63)) {
                    push!(f64, f64::from_bits(1 << 63));
                } else if (y.to_bits() == (1 << 63) && x == 0.0)
                    || (x.to_bits() == (1 << 63) && y == 0.0)
                {
                    push!(f64, 0.0);
                } else if y.is_nan() || x.is_nan() {
                    push!(f64, f64::NAN_ARITHMETIC);
                } else if y.is_infinite() && y.is_sign_negative() {
                    push!(f64, x)
                } else if x.is_infinite() && x.is_sign_negative() {
                    push!(f64, y)
                } else if (y.is_infinite() && y.is_sign_positive())
                    || (x.is_infinite() && x.is_sign_positive())
                {
                    push!(f64, f64::INFINITY)
                } else {
                    push!(f64, x.max(y))
                }
            }
            xa6_f64_copysign => {
                let y = pop!(f64);
                let x = pop!(f64);
                if x.nan_sign() == y.nan_sign() {
                    push!(f64, x);
                } else {
                    push!(f64, f64::from_bits(x.to_bits() ^ (1 << 63)))
                }
            }
            xa7_i32_wrap_i64 => {
                let x = pop!(i64);
                push!(i32, x.rem_euclid(2i64.pow(32)) as i32);
            }
            xa8_i32_trunc_f32_s => {
                let x = pop!(f32);
                if x.is_nan() {
                    throw!(InvalidConversionToInteger)
                }
                if x >= i32::MAX as f32 || x < i32::MIN as f32 {
                    throw!(IntegerOverflow);
                }
                let y = x.trunc() as i32;
                push!(i32, y)
            }
            xa9_i32_trunc_f32_u => {
                let x = pop!(f32).trunc();
                if x.is_nan() {
                    throw!(InvalidConversionToInteger)
                }
                if x >= u32::MAX as f32 || x < 0.0 {
                    throw!(IntegerOverflow);
                }
                let y = x as u32;
                push!(u32, y)
            }
            xaa_i32_trunc_f64_s => {
                let x = pop!(f64).trunc();
                if x.is_nan() {
                    throw!(InvalidConversionToInteger)
                }
                if x > i32::MAX as f64 || x < i32::MIN as f64 {
                    throw!(IntegerOverflow);
                }
                let y = x as i32;
                push!(i32, y)
            }
            xab_i32_trunc_f64_u => {
                let x = pop!(f64).trunc();
                if x.is_nan() {
                    throw!(InvalidConversionToInteger)
                }
                if x > u32::MAX as f64 || x < 0.0 {
                    throw!(IntegerOverflow);
                }
                let y = x as u32;
                push!(u32, y)
            }
            xac_i64_extend_i32_s => {
                let x = pop!(i32) as i64;
                push!(i64, x)
            }
            xad_i64_extend_i32_u => {
                let x = pop!(u32) as u64;
                push!(u64, x)
            }
            xae_i64_trunc_f32_s => {
                let x = pop!(f32);
                if x.is_nan() {
                    throw!(InvalidConversionToInteger)
                }
                if x >= i64::MAX as f32 || x < i64::MIN as f32 {
                    throw!(IntegerOverflow);
                }
                let y = x.trunc() as i64;
                push!(i64, y)
            }
            xaf_i64_trunc_f32_u => {
                let x = pop!(f32).trunc();
                if x.is_nan() {
                    throw!(InvalidConversionToInteger)
                }
                if x >= u64::MAX as f32 || x < 0.0 {
                    throw!(IntegerOverflow);
                }
                let y = x as u64;
                push!(u64, y)
            }
            xb0_i64_trunc_f64_s => {
                let x = pop!(f64);
                if x.is_nan() {
                    throw!(InvalidConversionToInteger)
                }
                if x >= i64::MAX as f64 || x < i64::MIN as f64 {
                    throw!(IntegerOverflow);
                }
                let y = x.trunc() as i64;
                push!(i64, y)
            }
            xb1_i64_trunc_f64_u => {
                let x = pop!(f64).trunc();
                if x.is_nan() {
                    throw!(InvalidConversionToInteger)
                }
                if x >= u64::MAX as f64 || x < 0.0 {
                    throw!(IntegerOverflow);
                }
                let y = x as u64;
                push!(u64, y)
            }
            xb2_f32_convert_i32_s => {
                let x = pop!(i32) as f32;
                push!(f32, x)
            }
            xb3_f32_convert_i32_u => {
                let x = pop!(u32) as f32;
                push!(f32, x)
            }
            xb4_f32_convert_i64_s => {
                let x = pop!(i64) as f32;
                push!(f32, x)
            }
            xb5_f32_convert_i64_u => {
                let x = pop!(u64) as f32;
                push!(f32, x)
            }
            xb6_f32_demote_f64 => {
                let x = pop!(f64);
                if x.is_nan() {
                    // very ugly
                    if x.to_bits()
                        & 0b0111111111110100000000000000000000000000000000000000000000000000
                        == 0b0111111111110100000000000000000000000000000000000000000000000000
                    {
                        push!(f32, f32::from_bits(0b11010100000000000000000000000000))
                    } else if x.to_bits()
                        & 0b0011111111100000000000000000000000000000000000000000000000000000
                        == 0b0011111111100000000000000000000000000000000000000000000000000000
                        && x.to_bits() & 0b1 == 0
                    {
                        push!(f32, f32::from_bits(0b10000000000000000000000000000000))
                    } else {
                        push!(f32, f32::from_bits(0b11010100000000000000000000000000))
                    }
                } else {
                    let y = x as f32;
                    push!(f32, y)
                }
            }

            xb7_f64_convert_i32_s => {
                let x = pop!(i32) as f64;
                push!(f64, x)
            }
            xb8_f64_convert_i32_u => {
                let x = pop!(u32) as f64;
                push!(f64, x)
            }
            xb9_f64_convert_i64_s => {
                let x = pop!(i64) as f64;
                push!(f64, x)
            }
            xba_f64_convert_i64_u => {
                let x = pop!(u64) as f64;
                push!(f64, x)
            }
            xbb_f64_promote_f32 => {
                let x = pop!(f32);
                if x.is_nan_canonical() {
                    push!(f64, f64::NAN_CANONICAL)
                } else if x.is_nan_arithmetic() {
                    push!(f64, f64::NAN_ARITHMETIC)
                } else {
                    let y = x as f64;
                    push!(f64, y)
                }
            }
            xbc_i32_reinterpret_f32 => {
                let x = pop!(f32);
                push!(u32, x.to_bits());
            }
            xbd_i64_reinterpret_f64 => {
                let x = pop!(f64);
                push!(u64, x.to_bits());
            }
            xbe_f32_reinterpret_i32 => {
                let x = pop!(u32);
                push!(f32, f32::from_bits(x));
            }
            xbf_f64_reinterpret_i64 => {
                let x = pop!(u64);
                push!(f64, f64::from_bits(x));
            }
            xc0_i32_extend8_s => {
                let x = pop!(i32) as i8;
                push!(i32, x as i32)
            }
            xc1_i32_extend16_s => {
                let x = pop!(i32) as i16;
                push!(i32, x as i32)
            }
            xc2_i64_extend8_s => {
                let x = pop!(i64) as i8;
                push!(i64, x as i64)
            }
            xc3_i64_extend16_s => {
                let x = pop!(i64) as i16;
                push!(i64, x as i64)
            }
            xc4_i64_extend32_s => {
                let x = pop!(i64) as i32;
                push!(i64, x as i64)
            }
            xd0_ref_null(x) => match x {
                RefTyp::FuncRef => todo!(),
                RefTyp::ExternRef => todo!(),
            },
            xfc_0_i32_trunc_sat_f32_s => {
                let x = pop!(f32);
                if x.is_nan() {
                    push!(i32, 0);
                } else if x >= i32::MAX as f32 {
                    push!(i32, i32::MAX)
                } else if x < i32::MIN as f32 {
                    push!(i32, i32::MIN)
                } else {
                    let y = x.trunc() as i32;
                    push!(i32, y)
                }
            }
            xfc_1_i32_trunc_sat_f32_u => {
                let x = pop!(f32);
                if x.is_nan() {
                    push!(u32, 0);
                } else if x >= u32::MAX as f32 {
                    push!(u32, u32::MAX)
                } else if x < u32::MIN as f32 {
                    push!(u32, u32::MIN)
                } else {
                    let y = x.trunc() as u32;
                    push!(u32, y)
                }
            }
            xfc_2_i32_trunc_sat_f64_s => {
                let x = pop!(f64);
                if x.is_nan() {
                    push!(i32, 0);
                } else if x >= i32::MAX as f64 {
                    push!(i32, i32::MAX)
                } else if x < i32::MIN as f64 {
                    push!(i32, i32::MIN)
                } else {
                    let y = x.trunc() as i32;
                    push!(i32, y)
                }
            }
            xfc_3_i32_trunc_sat_f64_u => {
                let x = pop!(f64);
                if x.is_nan() {
                    push!(u32, 0);
                } else if x >= u32::MAX as f64 {
                    push!(u32, u32::MAX)
                } else if x < u32::MIN as f64 {
                    push!(u32, u32::MIN)
                } else {
                    let y = x.trunc() as u32;
                    push!(u32, y)
                }
            }
            xfc_4_i64_trunc_sat_f32_s => {
                let x = pop!(f32);
                if x.is_nan() {
                    push!(i64, 0);
                } else if x >= i64::MAX as f32 {
                    push!(i64, i64::MAX)
                } else if x < i64::MIN as f32 {
                    push!(i64, i64::MIN)
                } else {
                    let y = x.trunc() as i64;
                    push!(i64, y)
                }
            }
            xfc_5_i64_trunc_sat_f32_u => {
                let x = pop!(f32);
                if x.is_nan() {
                    push!(u64, 0);
                } else if x >= u64::MAX as f32 {
                    push!(u64, u64::MAX)
                } else if x < u64::MIN as f32 {
                    push!(u64, u64::MIN)
                } else {
                    let y = x.trunc() as u64;
                    push!(u64, y)
                }
            }
            xfc_6_i64_trunc_sat_f64_s => {
                let x = pop!(f64);
                if x.is_nan() {
                    push!(i64, 0);
                } else if x >= i64::MAX as f64 {
                    push!(i64, i64::MAX)
                } else if x < i64::MIN as f64 {
                    push!(i64, i64::MIN)
                } else {
                    let y = x.trunc() as i64;
                    push!(i64, y)
                }
            }
            xfc_7_i64_trunc_sat_f64_u => {
                let x = pop!(f64);
                if x.is_nan() {
                    push!(u64, 0);
                } else if x >= u64::MAX as f64 {
                    push!(u64, u64::MAX)
                } else if x < u64::MIN as f64 {
                    push!(u64, u64::MIN)
                } else {
                    let y = x.trunc() as u64;
                    push!(u64, y)
                }
            }
            xfc_8_memory_init(DataIdx(i), _) => {
                let amount = pop!(i32) as usize;
                let source = pop!(i32) as usize;
                let destination = pop!(i32) as usize;
                let val = unwrap!(module.datas.get(*i as usize), MissingData).read();
                if source + amount > val.len() {
                    throw!(DataInitOutOfRange)
                }
                module
                    .memory
                    .write()
                    .slice_write(destination, &val[source..source + amount])?
            }
            xfc_9_data_drop(DataIdx(i)) => {
                if let Some(r) = module.datas.get(*i as usize) {
                    *r.write() = Vec::new();
                }
            }
            xfc_10_memory_copy(_, _) => {
                let amount = pop!(i32) as usize;
                let source = pop!(i32) as usize;
                let destination = pop!(i32) as usize;
                // println!("amount: {amount}, source: {source}, dest: {destination}");
                module.memory.write().copy(source, amount, destination)?;
            }
            xfc_11_memory_fill(_) => {
                let amount = pop!(i32) as usize;
                let val = pop!(i32) as u8;
                let ptr = pop!(i32) as usize;
                module.memory.write().bulk_write(ptr, amount, val)?;
            }
            xfc_12_table_init(ElemIdx(e), TableIdX(t)) => {
                let amount = pop!(i32) as u32;
                let source = pop!(i32) as u32;
                let destination = pop!(i32) as u32;
                let elems = unwrap!(module.elems.get(*e as usize), MissingElementIndex);
                let mut table = unwrap!(module.tables.get(*t as usize), MissingTableIndex).write();
                let Table {
                    table,
                    table_length,
                    ..
                } = &mut *table;

                let check_1 = source + amount > elems.read().instrs.len() as u32;
                // let check_2 = destination < table_length.0 as u32;
                let check_3 = destination + amount > table_length.1 as u32;
                if check_1 /*|| dbg!(check_2)*/ || check_3 {
                    throw!(OutOfBoundsTableAccess)
                }

                for i in 0..amount {
                    // should crash here
                    let e = match elems.read().instrs[(i + source) as usize] {
                        x41_i32_const(i) => FuncIdx(i as u32),
                        _ => todo!(),
                    };
                    table.insert(i + destination, e);
                }
            }
            xfc_13_elem_drop(ElemIdx(i)) => {
                let Some(r) = module.elems.get(*i as usize) else {
                    unreachable!()
                };
                *r.write() = Expr { instrs: Vec::new() };
            }
            xfc_14_table_copy(TableIdX(i_a), TableIdX(i_b)) => {
                let amount = pop!(i32) as u32;
                let source = pop!(i32) as u32;
                let destination = pop!(i32) as u32;

                let mut a_source =
                    unwrap!(module.tables.get(*i_a as usize), MissingTableIndex).write();
                let a_len = a_source.table_length;
                let a = &mut a_source.table;

                let check_1 = source + amount > a_len.0 as u32;
                let mut clones = Vec::new();
                for i in 0..amount {
                    let index = i + source;
                    let f = a.get(&index).unwrap_or(&FuncIdx(u32::MAX));
                    clones.push(*f);
                }
                drop(a_source);
                let mut b = unwrap!(module.tables.get(*i_b as usize), MissingTableIndex).write();
                let check_2 = destination > b.table.len() as u32;
                let check_3 = destination + amount > b.table_length.1 as u32;

                if check_1 || check_2 || check_3 {
                    throw!(OutOfBoundsTableAccess)
                }

                for (i, v) in clones.into_iter().enumerate() {
                    let index = i as u32 + destination;
                    b.table.insert(index, v);
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
                            for _ in
                                unwrap!(module.function_types.get(*t as usize), MissingFunction)
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
                        let func = unwrap!(module.function_types.get(*t as usize), MissingFunction);
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
