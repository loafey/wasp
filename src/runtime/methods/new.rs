use super::super::{
    clean_model::Model,
    error::{RuntimeError, RuntimeError::*},
    typecheck, Frame, Runtime, Value,
};
use crate::{
    parser::{
        Elem, ExportDesc, FuncIdx, Global,
        Instr::{self, *},
        LabelIdX, Module, Parsable, TableIdX, TypeIdX,
    },
    runtime::{clean_model::Function, typecheck::TypeCheckError},
};
use std::{collections::HashMap, io::Cursor, path::Path};
use tokio::{fs::File, io::AsyncReadExt};

impl Runtime {
    pub async fn new<P: AsRef<Path>>(path: P) -> Result<Self, RuntimeError> {
        let mut buf = Vec::new();

        let mut f = File::open(path.as_ref())
            .await
            .expect("Failed to open file");
        f.read_to_end(&mut buf).await.expect("Failed to read file");

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

        let mut sigs = Vec::new();
        for (_, TypeIdX(i)) in module.code.code.iter().zip(&module.funcs.functions) {
            let Some(typ) = module.types.function_types.get(*i as usize) else {
                return Err(RuntimeError::TypeError(
                    typecheck::TypeCheckError::MissingFunction,
                ));
            };
            sigs.push(typ.clone())
        }
        let globs = module
            .globals
            .globals
            .iter()
            .map(|g| g.gt.t)
            .collect::<Vec<_>>();
        // println!("{:?}", module.imports);

        let code_len = module.code.code.len() as u32;
        let table_len = module.tables.tables.len() as u32;
        let type_len = module.types.function_types.len() as u32;

        for e in &module.elems.elems {
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
                if f >= code_len {
                    return Err(RuntimeError::TypeError(TypeCheckError::UnknownFunction));
                }
            }
        }

        for ((_code_i, code), TypeIdX(i)) in module
            .code
            .code
            .iter()
            .enumerate()
            .zip(&module.funcs.functions)
        {
            let Some(typ) = module.types.function_types.get(*i as usize) else {
                return Err(RuntimeError::TypeError(
                    typecheck::TypeCheckError::MissingFunction,
                ));
            };

            let mut locals = typ.input.types.clone();
            locals.extend(code.code.t.iter().flat_map(|l| (0..l.n).map(|_| l.t)));

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
                        x02_block(_, instrs) => valid_calls!(instrs)?,
                        x03_loop(_, instrs) => valid_calls!(instrs)?,
                        x04_if_else(_, instrs, maybe_instrs) => {
                            valid_calls!(instrs)?;
                            if let Some(instrs) = maybe_instrs {
                                valid_calls!(instrs)?;
                            }
                        }
                        x10_call(FuncIdx(i)) if *i >= code_len => {
                            return Err(Unknown::Function);
                        }
                        x11_call_indirect(TypeIdX(_), TableIdX(i)) if *i >= table_len => {
                            return Err(Unknown::Table);
                        }
                        x11_call_indirect(TypeIdX(t), TableIdX(_)) if *t >= type_len => {
                            return Err(Unknown::Type);
                        }
                        _ => {}
                    }
                }
                Ok(())
            }
            match valid_calls(&code.code.e.instrs, code_len, table_len, type_len) {
                Ok(_) => {}
                Err(Unknown::Function) => {
                    return Err(RuntimeError::TypeError(TypeCheckError::UnknownFunction))
                }
                Err(Unknown::Table) => {
                    return Err(RuntimeError::TypeError(TypeCheckError::UnknownTable))
                }
                Err(Unknown::Type) => {
                    return Err(RuntimeError::TypeError(TypeCheckError::UnknownType))
                }
            }
            // print!("Checking");
            // for k in &module.exports.exports {
            // match k.d {
            // ExportDesc::Func(TypeIdX(i)) if i as usize == _code_i => {
            // print!(" {:?} ", k.nm.0);
            // break;
            // }
            // _ => {}
            // };
            // }
            // println!();
            // println!(" ({typ:?})");
            // ignore the result of type checking

            let _ = typecheck::check(
                Vec::new(),
                &locals,
                &code.code.e.instrs,
                &sigs,
                &module.types.function_types,
                &globs,
                Some(typ.output.types.clone()),
            );
        }

        let module = Model::try_from(module)?;
        for f in module.functions.values() {
            match f {
                Function::Import { .. } => continue,
                Function::Local { code, .. } => {
                    let mut depth = 0;
                    for c in code {
                        match c {
                            x0c_br(LabelIdX(i)) | x0d_br_if(LabelIdX(i)) => {
                                if *i > depth {
                                    return Err(RuntimeError::UnknownLabel);
                                }
                            }
                            x0e_br_table(ls, LabelIdX(i)) => {
                                for LabelIdX(i) in ls {
                                    if *i > depth {
                                        return Err(RuntimeError::UnknownLabel);
                                    }
                                }
                                if *i > depth {
                                    return Err(RuntimeError::UnknownLabel);
                                }
                            }
                            block_start(_, _, _) => depth += 1,
                            block_end(_, _, _) => depth -= 1,
                            _ => {}
                        }
                    }
                }
            }
        }
        Ok(Self {
            module,
            modules: HashMap::new(),
            stack,
            _path: path.as_ref().to_path_buf(),
        })
    }
}
