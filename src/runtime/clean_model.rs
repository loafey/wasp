use super::{
    memory::Memory,
    typecheck::TypeCheckError,
    IOFunction, Import, Mem,
    RuntimeError::{self, *},
    Stack, Value, IO,
};
use crate::{
    parser::{
        self, Code, Data, Elem, ExportDesc, Expr, FuncIdx, FuncType, Global as PGlobal, GlobalIdX,
        ImportDesc, Instr, LabelIdX, Limits, Locals, MemArg, MemIdX, Module, TableIdX, TypeIdX, BT,
    },
    ptr::{Ptr, PtrRW},
};
use std::collections::HashMap;

pub enum Function {
    WS {
        ty: FuncType,
        _locals: Vec<Locals>,
        code: Vec<Instr>,
        _labels: HashMap<Vec<u32>, u32>,
    },
    IO {
        ty: FuncType,
        func: IOFunction,
    },
}

impl std::fmt::Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WS {
                ty,
                _locals,
                code,
                _labels,
            } => f
                .debug_struct("WS")
                .field("ty", ty)
                .field("_locals", _locals)
                .field("code", code)
                .field("_labels", _labels)
                .finish(),
            Self::IO { .. } => write!(f, "IO"),
        }
    }
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
    Foreign {
        module: String,
        name: String,
    },
}

#[allow(clippy::type_complexity)]
fn setup_imports(
    other: &HashMap<String, Import>,
    value: &Module,
) -> Result<(Vec<Ptr<Function>>, Vec<PtrRW<Global>>, Vec<PtrRW<Table>>), RuntimeError> {
    let mut functions = Vec::new();
    let mut globals = Vec::new();
    let mut tables = Vec::new();

    for import in &value.imports.imports {
        match import.desc {
            ImportDesc::Func(TypeIdX(tid)) => {
                let ty = value
                    .types
                    .function_types
                    .get(tid as usize)
                    .ok_or(MissingType(file!(), line!(), column!()))?
                    .clone();

                match other.get(&import.module.0).expect("impossible!") {
                    Import::WS(other) => {
                        let exp = other
                            .exports
                            .get(&import.name.0)
                            .ok_or(RuntimeError::UnknownImport(file!(), line!(), column!()))?;
                        let ExportDesc::Func(FuncIdx(id)) = exp else {
                            return Err(RuntimeError::IncompatibleImportType(
                                file!(),
                                line!(),
                                column!(),
                            ));
                        };
                        let c = other
                            .functions
                            .get(*id as usize)
                            .ok_or(RuntimeError::UnknownImport(file!(), line!(), column!()))?;
                        let ty2 = match c.as_ref() {
                            Function::WS { ty, .. } | Function::IO { ty, .. } => ty,
                        };

                        if &ty != ty2 {
                            return Err(RuntimeError::IncompatibleImportType(
                                file!(),
                                line!(),
                                column!(),
                            ));
                        }

                        functions.push(c.clone());
                    }
                    Import::IO(IO {
                        functions: funcs, ..
                    }) => {
                        let func = funcs
                            .get(&*import.name.0)
                            .ok_or(RuntimeError::UnknownImport(file!(), line!(), column!()))?;

                        functions.push(Function::IO { func: *func, ty }.into())
                    }
                }
            }
            ImportDesc::Global(_) => {
                globals
                    .push(Global::Foreign(import.module.0.clone(), import.name.0.clone()).into());
            }
            ImportDesc::Table(_) => {
                tables.push(
                    Table::Foreign {
                        module: import.module.0.clone(),
                        name: import.name.0.clone(),
                    }
                    .into(),
                );
            }
            _ => (),
        }
    }

    Ok((functions, globals, tables))
}

fn get_tables(value: &Module, tables: &mut Vec<PtrRW<Table>>) -> Result<(), RuntimeError> {
    tables.extend(value.tables.tables.iter().map(|t| {
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
        .into()
    }));
    Ok(())
}

fn get_functions(
    code: Vec<Code>,
    function_types: &[FuncType],
    function_idx: &[TypeIdX],
    functions: &mut Vec<Ptr<Function>>,
) -> Result<(), RuntimeError> {
    for (k, code) in code.into_iter().enumerate() {
        let ty = code.code.t;
        let locals = ty.to_vec();

        let ty = function_types[function_idx[k].0 as usize].clone();
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

        functions.push(
            Function::WS {
                ty,
                _locals: locals,
                _labels: HashMap::new(),
                code,
            }
            .into(),
        );
    }

    Ok(())
}

fn validate_calls(
    functions: &[Ptr<Function>],
    tables: &[PtrRW<Table>],
    type_len: u32,
) -> Result<(), RuntimeError> {
    for code in functions {
        let Function::WS { code, .. } = code.as_ref() else {
            continue;
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
            Err(Unknown::Type) => return Err(RuntimeError::TypeError(TypeCheckError::UnknownType)),
        }
    }
    Ok(())
}

fn validate_elems(elems: &[Elem], functions: &[Ptr<Function>]) -> Result<(), RuntimeError> {
    for e in elems {
        let (_, vec) = match e {
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
            if f >= functions.len() as u32 {
                return Err(RuntimeError::TypeError(TypeCheckError::UnknownFunction));
            }
        }
    }
    Ok(())
}

fn setup_elems(
    elems: Vec<Elem>,
    tables: &mut [PtrRW<Table>],
) -> Result<Vec<PtrRW<Expr>>, RuntimeError> {
    let mut result = Vec::new();
    for elem in elems.into_iter() {
        match elem {
            Elem::E0(expr, vec) => match &expr.instrs[..] {
                [Instr::x41_i32_const(off)] => {
                    for (i, v) in vec.into_iter().enumerate() {
                        let mut table = tables[0].write();
                        let table = match &mut *table {
                            Table::Native { table, .. } => table,
                            Table::Foreign { module, name } => {
                                todo!("{module}::{name}")
                            }
                        };
                        table.insert(*off as u32 + i as u32, v);
                    }
                    result.push(expr.clone().into());
                }
                _ => panic!(),
            },
            Elem::E1(_fr, funcs) => {
                result.push(
                    Expr {
                        instrs: funcs
                            .into_iter()
                            .map(|FuncIdx(i)| Instr::x41_i32_const(i as i32))
                            .collect(),
                    }
                    .into(),
                );
            }
            Elem::E2(TableIdX(t), expr, _rt, vec) => match &expr.instrs[..] {
                [Instr::x41_i32_const(off)] => {
                    result.push(
                        Expr {
                            instrs: vec
                                .iter()
                                .map(|FuncIdx(i)| Instr::x41_i32_const(*i as i32))
                                .collect(),
                        }
                        .into(),
                    );
                    for (i, v) in vec.iter().enumerate() {
                        let mut table = tables[t as usize].write();
                        let Table::Native { table, .. } = &mut *table else {
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
                    result.push(expr.into());
                }
            }
            Elem::E6(_, _, _, _) => todo!(),
            Elem::E7(_, _) => todo!(),
        }
    }
    Ok(result)
}

fn get_globals(
    globals: &mut Vec<PtrRW<Global>>,
    p_globals: Vec<parser::Global>,
) -> Result<(), RuntimeError> {
    for PGlobal { e, .. } in p_globals {
        let val = match e.instrs[0] {
            Instr::x41_i32_const(x) => Value::I32(x),
            Instr::x42_i64_const(x) => Value::I64(x),
            Instr::x43_f32_const(x) => Value::F32(x),
            Instr::x44_f64_const(x) => Value::F64(x),
            _ => return Err(GlobalWithoutValue),
        };
        globals.push(Global::Native(val).into());
    }
    Ok(())
}

fn setup_memory<const N: usize>(mems: Vec<parser::Mem>) -> Result<Memory<N>, RuntimeError> {
    let (mem_cur, mem_max) = mems
        .first()
        .map(|m| match m.limits {
            Limits::Min(i) => (i as usize, usize::MAX),
            Limits::MinMax(i, m) => (i as usize, m as usize),
        })
        .unwrap_or((1, usize::MAX));
    Ok(Memory::new(mem_cur, mem_max))
}

fn setup_data<const N: usize>(
    data: Vec<parser::Data>,
    memory: &mut Memory<N>,
    globals: &[PtrRW<Global>],
) -> Result<Vec<PtrRW<Vec<u8>>>, RuntimeError> {
    let mut datas = Vec::new();
    for d in data {
        match d {
            Data::ActiveX(MemIdX(0), e, vec) | Data::Active(e, vec) => {
                let p = match &e.instrs[..] {
                    [Instr::x41_i32_const(p)] => *p,
                    [Instr::x23_global_get(GlobalIdX(i))] => {
                        if let Some(k) = globals.get(*i as usize) {
                            let k = k.read();
                            match &*k {
                                Global::Native(Value::I32(p)) => *p,
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
                        p as usize + i,
                        MemArg {
                            align: 0,
                            offset: 0,
                        },
                        *v,
                    )?;
                }
                datas.push(vec.clone().into());
            }
            Data::Passive(v) => datas.push(v.clone().into()),
            Data::ActiveX(_, _, _) => todo!(""),
        }
    }
    Ok(datas)
}

fn validate_label_depth(functions: &[Ptr<Function>]) -> Result<(), RuntimeError> {
    for f in functions {
        let Function::WS { code, .. } = f.as_ref() else {
            continue;
        };
        let mut depth = 0;
        for c in code {
            match c {
                Instr::x0c_br(LabelIdX(i)) | Instr::x0d_br_if(LabelIdX(i)) => {
                    if *i > depth {
                        return Err(RuntimeError::UnknownLabel);
                    }
                }
                Instr::x0e_br_table(ls, LabelIdX(i)) => {
                    for LabelIdX(i) in ls {
                        if *i > depth {
                            return Err(RuntimeError::UnknownLabel);
                        }
                    }
                    if *i > depth {
                        return Err(RuntimeError::UnknownLabel);
                    }
                }
                Instr::block_start(_, _, _) => depth += 1,
                Instr::block_end(_, _, _) => depth -= 1,
                _ => {}
            }
        }
    }
    Ok(())
}

#[derive(Debug, Clone)]
pub struct Model {
    pub functions: Vec<Ptr<Function>>,
    pub tables: Vec<PtrRW<Table>>,
    pub elems: Vec<PtrRW<Expr>>,
    pub function_types: Vec<Ptr<FuncType>>,
    pub globals: Vec<PtrRW<Global>>,
    pub exports: HashMap<String, ExportDesc>,
    pub datas: Vec<PtrRW<Vec<u8>>>,
    pub memory: PtrRW<Memory<{ 65536 + 1 }>>,
}
impl TryFrom<(&HashMap<String, Import>, Module)> for Model {
    type Error = RuntimeError;
    fn try_from((other, value): (&HashMap<String, Import>, Module)) -> Result<Self, Self::Error> {
        let type_len = value.types.function_types.len() as u32;

        let (mut functions, mut globals, mut tables) = setup_imports(other, &value)?;
        get_tables(&value, &mut tables)?;
        get_functions(
            value.code.code,
            &value.types.function_types,
            &value.funcs.functions,
            &mut functions,
        )?;
        validate_calls(&functions, &tables, type_len)?;
        validate_elems(&value.elems.elems, &functions)?;
        let elems = setup_elems(value.elems.elems, &mut tables)?;
        get_globals(&mut globals, value.globals.globals)?;
        let mut memory = setup_memory(value.mems.mems)?;
        let datas = setup_data(value.datas.data, &mut memory, &globals)?;
        validate_label_depth(&functions)?;

        Ok(Self {
            functions,
            tables,
            function_types: value
                .types
                .function_types
                .into_iter()
                .map(Into::into)
                .collect(),
            elems,
            globals,
            exports: value.exports.exports.into_iter().collect::<HashMap<_, _>>(),
            datas,
            memory: memory.into(),
        })
    }
}
