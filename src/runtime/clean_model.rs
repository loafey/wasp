use super::{memory::Memory, RuntimeError, RuntimeError::*, Value};
use crate::parser::{
    Data, Elem, ExportDesc, Expr, FuncIdx, FuncType, Global, GlobalIdX, ImportDesc, Instr, Limits,
    MemArg, MemIdX, Module, Name, TableIdX, TypeIdX, BT,
};
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

#[derive(Debug)]
pub enum Function {
    Import {
        ty: FuncType,
        module: Name,
        name: Name,
    },
    Local {
        ty: FuncType,
        locals: Vec<usize>,
        code: Vec<Instr>,
        labels: HashMap<Vec<u32>, u32>,
    },
}

#[derive(Debug)]
pub struct Table {
    table: HashMap<u32, FuncIdx>,
    pub table_length: (usize, usize),
}
impl Deref for Table {
    type Target = HashMap<u32, FuncIdx>;

    fn deref(&self) -> &Self::Target {
        &self.table
    }
}
impl DerefMut for Table {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.table
    }
}

#[derive(Debug)]
pub struct Model {
    pub functions: HashMap<u32, Function>,
    pub tables: Vec<Table>,
    pub elems: HashMap<u32, Expr>,
    pub function_types: HashMap<u32, FuncType>,
    pub globals: HashMap<u32, Value>,
    pub exports: HashMap<String, ExportDesc>,
    pub datas: HashMap<u32, Vec<u8>>,
    pub memory: Memory<{ 65536 + 1 }>,
}
impl TryFrom<Module> for Model {
    type Error = RuntimeError;
    fn try_from(value: Module) -> Result<Self, Self::Error> {
        let mut functions = HashMap::new();
        let mut import_count = 0;
        for (k, import) in value.imports.imports.into_iter().enumerate() {
            let ImportDesc::Func(TypeIdX(ty_id)) = import.desc else {
                continue;
            };
            import_count += 1;

            let ty = value.types.function_types[ty_id as usize].clone();

            let v = Function::Import {
                ty,
                module: import.module,
                name: import.name,
            };

            functions.insert(k as u32, v);
        }

        // println!(
        // "{} {} {}",
        // value.code.code.len(),
        // value.funcs.functions.len(),
        // value.types.function_types.len()
        // );
        // for v in &value.types.function_types {
        // println!("{v:?}")
        // }
        for (k, code) in value.code.code.into_iter().enumerate() {
            let ty = code.code.t;
            let locals = ty.iter().enumerate().map(|(s, _)| s).collect();
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

            functions.insert(
                (k + import_count) as u32,
                Function::Local {
                    ty,
                    locals,
                    labels: HashMap::new(),
                    code,
                },
            );
        }

        let mut tables: Vec<_> = value
            .tables
            .tables
            .into_iter()
            .map(|t| {
                let table_length = match t.lim {
                    Limits::Min(m) => (0, m as usize),
                    Limits::MinMax(n, m) => (n as usize, m as usize),
                };
                let table = (table_length.0..table_length.1)
                    .map(|a| (a as u32, FuncIdx(u32::MAX)))
                    .collect();
                Table {
                    table,
                    table_length,
                }
            })
            .collect();

        let mut elems = HashMap::new();
        for (i, elem) in value.elems.elems.into_iter().enumerate() {
            match elem {
                Elem::E0(expr, vec) => match &expr.instrs[..] {
                    [Instr::x41_i32_const(off)] => {
                        for (i, v) in vec.into_iter().enumerate() {
                            tables[0].table.insert(*off as u32 + i as u32, v);
                        }
                        elems.insert(i as u32, expr.clone());
                    }
                    _ => panic!(),
                },
                Elem::E1(_fr, funcs) => {
                    elems.insert(
                        i as u32,
                        Expr {
                            instrs: funcs
                                .into_iter()
                                .map(|FuncIdx(i)| Instr::x41_i32_const(i as i32))
                                .collect(),
                        },
                    );
                }
                Elem::E2(TableIdX(t), expr, _rt, vec) => match &expr.instrs[..] {
                    [Instr::x41_i32_const(off)] => {
                        elems.insert(
                            i as u32,
                            Expr {
                                instrs: vec
                                    .iter()
                                    .map(|FuncIdx(i)| Instr::x41_i32_const(*i as i32))
                                    .collect(),
                            },
                        );
                        for (i, v) in vec.iter().enumerate() {
                            tables[t as usize].table.insert(*off as u32 + i as u32, *v);
                        }
                    }
                    _ => panic!(),
                },
                Elem::E3(_, _) => todo!(),
                Elem::E4(_, _) => todo!(),
                Elem::E5(_rt, vec) => {
                    // might be wrong
                    for expr in vec {
                        elems.insert(i as u32, expr);
                    }
                }
                Elem::E6(_, _, _, _) => todo!(),
                Elem::E7(_, _) => todo!(),
            }
        }

        let function_types = value
            .types
            .function_types
            .into_iter()
            .enumerate()
            .map(|(i, f)| (i as u32, f))
            .collect();

        let exports = value
            .exports
            .exports
            .iter()
            .map(|exp| (exp.nm.0.clone(), exp.d))
            .collect();

        let mut globals = HashMap::new();
        for (i, Global { e, .. }) in value.globals.globals.iter().enumerate() {
            let val = match e.instrs[0] {
                Instr::x41_i32_const(x) => Value::I32(x),
                Instr::x42_i64_const(x) => Value::I64(x),
                Instr::x43_f32_const(x) => Value::F32(x),
                Instr::x44_f64_const(x) => Value::F64(x),
                _ => return Err(GlobalWithoutValue),
            };
            globals.insert(i as u32, val);
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

        let mut datas = HashMap::new();
        for (i, d) in value.datas.data.iter().enumerate() {
            match d {
                Data::ActiveX(MemIdX(0), e, vec) | Data::Active(e, vec) => {
                    let p = match &e.instrs[..] {
                        [Instr::x41_i32_const(p)] => p,
                        [Instr::x23_global_get(GlobalIdX(i))] => {
                            if let Some(k) = globals.get(i) {
                                match k {
                                    Value::I32(p) => p,
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
                }
                Data::Passive(v) => {
                    datas.insert(i as u32, v.clone());
                }
                Data::ActiveX(_, _, _) => todo!(""),
            }
        }

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
