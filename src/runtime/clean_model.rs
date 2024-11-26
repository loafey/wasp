use super::{
    memory::Memory,
    typecheck::TypeCheckError,
    RuntimeError::{self, *},
    Value,
};
use crate::{
    parser::{
        Data, Elem, ExportDesc, Expr, FuncIdx, FuncType, Global as PGlobal, GlobalIdX, ImportDesc,
        Instr, Limits, Locals, MemArg, MemIdX, Module, Name, TableIdX, TypeIdX, BT,
    },
    ptr::{Ptr, PtrRW},
    runtime::typecheck,
};
use std::collections::HashMap;

#[derive(Debug)]
pub enum Function {
    Foreign {
        ty: FuncType,
        module: Name,
        name: Name,
    },
    Native {
        ty: FuncType,
        _locals: Vec<Locals>,
        code: Vec<Instr>,
        _labels: HashMap<Vec<u32>, u32>,
    },
}

#[derive(Debug, Clone)]
pub enum Global {
    Native(Value),
    Foreign(String, String),
}

#[derive(Debug)]
pub enum Table {
    Native {
        table: HashMap<u32, FuncIdx>,
        table_length: (usize, usize),
    },
    #[allow(unused)]
    Foreign { module: String, name: String },
}

#[derive(Debug, Clone)]
pub struct Model {
    pub functions: Ptr<Vec<Function>>,
    pub tables: PtrRW<Vec<Table>>,
    pub elems: PtrRW<Vec<Expr>>,
    pub function_types: Ptr<Vec<FuncType>>,
    pub globals: PtrRW<Vec<Global>>,
    pub exports: Ptr<HashMap<String, ExportDesc>>,
    pub datas: PtrRW<Vec<Vec<u8>>>,
    pub memory: PtrRW<Memory<{ 65536 + 1 }>>,
}
impl TryFrom<Module> for Model {
    type Error = RuntimeError;
    fn try_from(value: Module) -> Result<Self, Self::Error> {
        let type_len = value.types.function_types.len() as u32;

        let mut sigs = Vec::new();
        for (_, TypeIdX(i)) in value.code.code.iter().zip(&value.funcs.functions) {
            let Some(typ) = value.types.function_types.get(*i as usize) else {
                return Err(RuntimeError::TypeError(
                    typecheck::TypeCheckError::MissingFunction,
                ));
            };
            sigs.push(typ.clone())
        }

        // let globs = value
        //     .globals
        //     .globals
        //     .iter()
        //     .map(|g| g.gt.t)
        //     .collect::<Vec<_>>();

        let mut functions = Vec::new();
        let mut globals = Vec::new();
        let mut tables = Vec::new();

        for import in value.imports.imports.into_iter() {
            match import.desc {
                ImportDesc::Func(TypeIdX(ty_id)) => {
                    let ty = value
                        .types
                        .function_types
                        .get(ty_id as usize)
                        .ok_or(RuntimeError::TypeError(TypeCheckError::MissingType))?
                        .clone();

                    let v = Function::Foreign {
                        ty,
                        module: import.module,
                        name: import.name,
                    };

                    functions.push(v);
                }
                ImportDesc::Global(_) => {
                    globals.push(Global::Foreign(
                        import.module.0.clone(),
                        import.name.0.clone(),
                    ));
                }
                ImportDesc::Table(_) => {
                    tables.push(Table::Foreign {
                        module: import.module.0.clone(),
                        name: import.name.0.clone(),
                    });
                }
                _ => (),
            }
        }

        tables.extend(value.tables.tables.into_iter().map(|t| {
            let table_length = match t.lim {
                Limits::Min(m) => (0, m as usize),
                Limits::MinMax(n, m) => (n as usize, m as usize),
            };
            let table = (table_length.0..table_length.1)
                .map(|a| (a as u32, FuncIdx(u32::MAX)))
                .collect();
            Table::Native {
                table,
                table_length,
            }
        }));

        for (k, code) in value.code.code.into_iter().enumerate() {
            let ty = code.code.t;
            let locals = ty.to_vec();
            // locals.append(
            //     &mut code
            //         .code
            //         .t
            //         .iter()
            //         .flat_map(|l| (0..l.n).map(|_| l.t))
            //         .collect::<Vec<_>>(),
            // );

            let ty = value.types.function_types[value.funcs.functions[k].0 as usize].clone();
            let mut code = code.code.e.instrs;

            let mut pc = 0;
            while pc < code.len() {
                match &code[pc] {
                    Instr::x02_block(bt, ins) => {
                        let c = ins.clone();
                        let bt = *bt;

                        code.remove(pc);
                        code.insert(pc, Instr::block_end(BT::Block, 0, bt));
                        for (_, i) in c.into_iter().enumerate().rev() {
                            code.insert(pc, i);
                        }
                        code.insert(pc, Instr::block_start(BT::Block, 0, bt));
                    }
                    Instr::x03_loop(bt, ins) => {
                        let c = ins.clone();
                        let bt = *bt;

                        code.remove(pc);
                        code.insert(pc, Instr::block_end(BT::Loop, 0, bt));
                        for (_, i) in c.into_iter().enumerate().rev() {
                            code.insert(pc, i);
                        }
                        code.insert(pc, Instr::block_start(BT::Loop, 0, bt));
                    }
                    Instr::x04_if_else(bt, then, els) => {
                        let bt = *bt;

                        let then = then.clone();
                        // increment_labels(&mut then, 1);
                        let els = els.clone();
                        code.remove(pc);

                        code.insert(pc, Instr::block_end(BT::Block, 0, bt)); // then block end
                        for i in then.into_iter().rev() {
                            code.insert(pc, i);
                        }
                        code.insert(pc, Instr::block_start(BT::Block, 0, bt)); // then block end
                        let offset = els.is_some() as usize;

                        if let Some(els) = els {
                            // increment_labels(&mut els, 1);
                            code.insert(pc, Instr::else_jump(0));
                            code.insert(pc, Instr::block_end(BT::Block, 0, bt));
                            for i in els.into_iter().rev() {
                                code.insert(pc, i);
                            }
                            code.insert(pc, Instr::block_start(BT::Block, 0, bt));
                            // els block end
                        }
                        code.insert(pc, Instr::if_then_else(offset));
                        // then block end
                    }
                    _ => {}
                }
                pc += 1;
            }

            let mut pc = 0;
            while pc < code.len() {
                let (sp, ep) = if let Instr::block_start(_, _, _bt) = &mut code[pc] {
                    let mut in_pc = pc + 1;
                    let mut bs = 0;
                    loop {
                        match &code[in_pc] {
                            Instr::block_start(_, _, _bt) => {
                                bs += 1;
                            }
                            Instr::block_end(_, _, _bt) => {
                                if bs == 0 {
                                    break (pc, in_pc);
                                }
                                bs -= 1;
                            }
                            _ => {}
                        }
                        in_pc += 1;
                    }
                } else {
                    pc += 1;
                    continue;
                };

                let Instr::block_start(_, ins, _bt) = &mut code[sp] else {
                    panic!()
                };
                *ins = ep;
                let Instr::block_end(_, ins, _bt) = &mut code[ep] else {
                    panic!()
                };
                *ins = sp;
                pc += 1;
            }

            let mut pc = 0;
            while pc < code.len() {
                if let Instr::if_then_else(_) = &code[pc] {
                    let mut in_pc = pc + 1;

                    let mut bc = 0;
                    loop {
                        match &code[in_pc] {
                            Instr::block_start(_, _, _) => bc += 1,
                            Instr::block_end(_, _, _) => bc -= 1,
                            _ => {}
                        }
                        if bc == 0 {
                            break;
                        }
                        in_pc += 1;
                    }
                    let Instr::if_then_else(offset) = &mut code[pc] else {
                        unreachable!()
                    };
                    *offset += in_pc + 1;
                } else if let Instr::else_jump(_) = &code[pc] {
                    let mut in_pc = pc + 1;

                    let mut bc = 0;
                    loop {
                        match &code[in_pc] {
                            Instr::block_start(_, _, _) => bc += 1,
                            Instr::block_end(_, _, _) => bc -= 1,
                            _ => {}
                        }
                        if bc == 0 {
                            break;
                        }
                        in_pc += 1;
                    }
                    let Instr::else_jump(offset) = &mut code[pc] else {
                        unreachable!()
                    };
                    *offset += in_pc + 1;
                }
                pc += 1;
            }

            functions.push(Function::Native {
                ty,
                _locals: locals,
                _labels: HashMap::new(),
                code,
            });
        }
        let functions: Ptr<_> = functions.into();

        for code in functions.iter() {
            let (_typ, code) = match code {
                Function::Foreign { .. } => continue,
                Function::Native { ty, code, .. } => (ty, code),
            };

            // let locals = typ.input.types.clone();

            // this should be part of the typechecker
            enum Unknown {
                Table,
                Function,
                Type,
            }
            fn valid_calls(
                instrs: &[Instr],
                code_len: u32,
                table_len: u32,
                type_len: u32,
            ) -> Result<(), Unknown> {
                macro_rules! valid_calls {
                    ($p:expr) => {
                        valid_calls($p, code_len, table_len, type_len)
                    };
                }
                for i in instrs {
                    match i {
                        Instr::x02_block(_, instrs) => valid_calls!(instrs)?,
                        Instr::x03_loop(_, instrs) => valid_calls!(instrs)?,
                        Instr::x04_if_else(_, instrs, maybe_instrs) => {
                            valid_calls!(instrs)?;
                            if let Some(instrs) = maybe_instrs {
                                valid_calls!(instrs)?;
                            }
                        }
                        Instr::x10_call(FuncIdx(i)) if *i >= code_len => {
                            return Err(Unknown::Function);
                        }
                        Instr::x11_call_indirect(TypeIdX(_), TableIdX(i)) if *i >= table_len => {
                            return Err(Unknown::Table);
                        }
                        Instr::x11_call_indirect(TypeIdX(t), TableIdX(_)) if *t >= type_len => {
                            return Err(Unknown::Type);
                        }
                        _ => {}
                    }
                }
                Ok(())
            }
            match valid_calls(code, functions.len() as u32, tables.len() as u32, type_len) {
                Ok(_) => {}
                Err(Unknown::Function) => {
                    return Err(RuntimeError::TypeError(TypeCheckError::UnknownFunction));
                }
                Err(Unknown::Table) => {
                    return Err(RuntimeError::TypeError(TypeCheckError::UnknownTable))
                }
                Err(Unknown::Type) => {
                    return Err(RuntimeError::TypeError(TypeCheckError::UnknownType))
                }
            }
        }

        for e in &value.elems.elems {
            let (offset, vec) = match e {
                Elem::E0(expr, vec) | Elem::E2(_, expr, _, vec) => (
                    match &expr.instrs[..] {
                        [Instr::x41_i32_const(offset)] => *offset as u32,
                        _ => todo!(),
                    },
                    vec.clone(),
                ),
                Elem::E4(expr, vec) | Elem::E6(_, expr, _, vec) => (
                    match &expr.instrs[..] {
                        [Instr::x41_i32_const(offset)] => *offset as u32,
                        e => todo!("{e:?}"),
                    },
                    vec.iter()
                        .flat_map(|e| {
                            e.instrs.iter().map(|e| match e {
                                Instr::x41_i32_const(offset) => FuncIdx(*offset as u32),
                                e => todo!("{e:?}"),
                            })
                        })
                        .collect(),
                ),
                Elem::E1(_, vec) | Elem::E3(_, vec) => (0, vec.clone()),
                Elem::E5(_, vec) | Elem::E7(_, vec) => (
                    0,
                    vec.iter()
                        .flat_map(|e| {
                            e.instrs.iter().map(|e| match e {
                                Instr::x41_i32_const(offset) => FuncIdx(*offset as u32),
                                Instr::xd2_ref_func(f) => *f,
                                Instr::xd0_ref_null(_) => FuncIdx(0),
                                e => todo!("{e:?}"),
                            })
                        })
                        .collect(),
                ),
            };
            for FuncIdx(f) in vec {
                let f = f + offset;
                if f >= functions.len() as u32 {
                    println!("here {f} {}", functions.len());
                    return Err(RuntimeError::TypeError(TypeCheckError::UnknownFunction));
                }
            }
        }

        let mut elems = Vec::new();
        for elem in value.elems.elems.into_iter() {
            match elem {
                Elem::E0(expr, vec) => match &expr.instrs[..] {
                    [Instr::x41_i32_const(off)] => {
                        for (i, v) in vec.into_iter().enumerate() {
                            let Table::Native { table, .. } = &mut tables[0] else {
                                unreachable!()
                            };
                            table.insert(*off as u32 + i as u32, v);
                        }
                        elems.push(expr.clone());
                    }
                    _ => panic!(),
                },
                Elem::E1(_fr, funcs) => {
                    elems.push(Expr {
                        instrs: funcs
                            .into_iter()
                            .map(|FuncIdx(i)| Instr::x41_i32_const(i as i32))
                            .collect(),
                    });
                }
                Elem::E2(TableIdX(t), expr, _rt, vec) => match &expr.instrs[..] {
                    [Instr::x41_i32_const(off)] => {
                        elems.push(Expr {
                            instrs: vec
                                .iter()
                                .map(|FuncIdx(i)| Instr::x41_i32_const(*i as i32))
                                .collect(),
                        });
                        for (i, v) in vec.iter().enumerate() {
                            let Table::Native { table, .. } = &mut tables[t as usize] else {
                                unreachable!()
                            };
                            table.insert(*off as u32 + i as u32, *v);
                        }
                    }
                    _ => panic!(),
                },
                Elem::E3(_, _) => todo!(),
                Elem::E4(_, _) => todo!(),
                Elem::E5(_rt, vec) => {
                    // might be wrong
                    for expr in vec {
                        elems.push(expr);
                    }
                }
                Elem::E6(_, _, _, _) => todo!(),
                Elem::E7(_, _) => todo!(),
            }
        }
        let elems = elems.into();
        let tables = tables.into();

        let function_types = value.types.function_types.into();

        let exports = value
            .exports
            .exports
            .iter()
            .map(|exp| (exp.nm.0.clone(), exp.d))
            .collect::<HashMap<_, _>>()
            .into();

        for PGlobal { e, .. } in value.globals.globals.iter() {
            let val = match e.instrs[0] {
                Instr::x41_i32_const(x) => Value::I32(x),
                Instr::x42_i64_const(x) => Value::I64(x),
                Instr::x43_f32_const(x) => Value::F32(x),
                Instr::x44_f64_const(x) => Value::F64(x),
                _ => return Err(GlobalWithoutValue),
            };
            globals.push(Global::Native(val));
        }

        let (mem_cur, mem_max) = value
            .mems
            .mems
            .first()
            .map(|m| match m.limits {
                Limits::Min(i) => (i as usize, usize::MAX),
                Limits::MinMax(i, m) => (i as usize, m as usize),
            })
            .unwrap_or((1, usize::MAX));
        let mut memory = Memory::new(mem_cur, mem_max);

        let mut datas = Vec::new();
        for d in value.datas.data.iter() {
            match d {
                Data::ActiveX(MemIdX(0), e, vec) | Data::Active(e, vec) => {
                    let p = match &e.instrs[..] {
                        [Instr::x41_i32_const(p)] => p,
                        [Instr::x23_global_get(GlobalIdX(i))] => {
                            if let Some(k) = globals.get(*i as usize) {
                                match k {
                                    Global::Native(Value::I32(p)) => p,
                                    _ => todo!(),
                                }
                            } else {
                                return Err(UnknownGlobal);
                            }
                        }
                        _ => {
                            error!("{:?}", e.instrs);
                            return Err(ActiveDataWithoutOffset);
                        }
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
                    datas.push(vec.clone());
                }
                Data::Passive(v) => datas.push(v.clone()),
                Data::ActiveX(_, _, _) => todo!(""),
            }
        }
        let globals = globals.into();
        let datas = datas.into();
        let memory = memory.into();

        Ok(Self {
            functions,
            tables,
            function_types,
            elems,
            globals,
            exports,
            datas,
            memory,
        })
    }
}
