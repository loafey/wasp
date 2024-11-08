use crate::parser::{
    BlockType, Elem, FuncIdx, FuncType, ImportDesc, Instr, LabelIdX, LocalIdX, Module, Name,
    TableIdX, TypeIdX, BT,
};
use std::{collections::HashMap, ops::Deref, u32};

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
}
impl Deref for Table {
    type Target = HashMap<u32, FuncIdx>;

    fn deref(&self) -> &Self::Target {
        &self.table
    }
}

#[derive(Debug)]
pub struct Model {
    pub functions: HashMap<u32, Function>,
    pub tables: Vec<Table>,
    pub function_types: HashMap<u32, FuncType>,
}
impl From<Module> for Model {
    fn from(value: Module) -> Self {
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

        for (k, code) in value.code.code.into_iter().enumerate() {
            let ty = code.code.t;
            let locals = ty.iter().enumerate().map(|(s, _)| s).collect();
            let ty = value.types.function_types[value.funcs.functions[k].0 as usize].clone();
            let mut code = code.code.e.instrs;

            let mut pc = 0;
            while pc < code.len() {
                match &code[pc] {
                    Instr::x02_block(bt, ins) => {
                        match bt {
                            BlockType::Eps => {}
                            BlockType::T(_) => todo!(),
                            BlockType::X(_) => todo!(),
                        }

                        let c = ins.clone();

                        code.remove(pc);
                        code.insert(pc, Instr::block_end(BT::Block, 0));
                        for (_, i) in c.into_iter().enumerate().rev() {
                            code.insert(pc, i);
                        }
                        code.insert(pc, Instr::block_start(BT::Block, 0));
                    }
                    Instr::x03_loop(bt, ins) => {
                        match bt {
                            BlockType::Eps => {}
                            BlockType::T(_) => todo!(),
                            BlockType::X(_) => todo!(),
                        }

                        let c = ins.clone();

                        code.remove(pc);
                        code.insert(pc, Instr::block_end(BT::Loop, 0));
                        for (_, i) in c.into_iter().enumerate().rev() {
                            code.insert(pc, i);
                        }
                        code.insert(pc, Instr::block_start(BT::Loop, 0));
                    }
                    Instr::x04_if_else(bt, then, els) => {
                        match bt {
                            BlockType::Eps => {}
                            BlockType::T(_) => todo!(),
                            BlockType::X(_) => todo!(),
                        };

                        let then = then.clone();
                        let els = els.clone();
                        let els_exists = els.is_some();
                        code.remove(pc);

                        if let Some(els) = els {
                            for i in els.into_iter().rev() {
                                code.insert(pc, i);
                            }
                            code.insert(pc, Instr::block_end(BT::Block, 0)); // els block end
                        }
                        code.insert(pc, Instr::block_end(BT::Block, 0)); // then block end
                        code.insert(pc, Instr::x0c_br(LabelIdX(1))); // then block end
                        for i in then.into_iter().rev() {
                            code.insert(pc, i);
                        }

                        // check
                        code.insert(pc, Instr::x0d_br_if(LabelIdX(0))); // then block end
                        code.insert(pc, Instr::x73_i32_xor);
                        code.insert(pc, Instr::x41_i32_const(1));
                        code.insert(pc, Instr::x20_local_get(LocalIdX(u32::MAX)));
                        // THIS IS SO DISGUSTING

                        if els_exists {
                            code.insert(pc, Instr::block_start(BT::Block, 0)); // then block end
                        }
                        code.insert(pc, Instr::block_start(BT::Block, 0)); // then block end
                        code.insert(pc, Instr::x21_local_set(LocalIdX(u32::MAX)));
                        // THIS IS SO DISGUSTING
                    }
                    _ => {}
                }
                pc += 1;
            }

            let mut pc = 0;
            while pc < code.len() {
                let (sp, ep) = if let Instr::block_start(_, _) = &mut code[pc] {
                    let mut in_pc = pc + 1;
                    let mut bs = 0;
                    loop {
                        match &code[in_pc] {
                            Instr::block_start(_, _) => {
                                bs += 1;
                            }
                            Instr::block_end(_, _) => {
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

                let Instr::block_start(_, ins) = &mut code[sp] else {
                    panic!()
                };
                *ins = ep;
                let Instr::block_end(_, ins) = &mut code[ep] else {
                    panic!()
                };
                *ins = sp;
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
            .map(|_| Table {
                table: HashMap::new(),
            })
            .collect();

        for elem in value.elems.elems.into_iter() {
            match elem {
                Elem::E0(expr, vec) => match &expr.instrs[..] {
                    [Instr::x41_i32_const(off)] => {
                        for (i, v) in vec.into_iter().enumerate() {
                            tables[0].table.insert(*off as u32 + i as u32, v);
                        }
                    }
                    _ => panic!(),
                },
                Elem::E1(_, _) => todo!(),
                Elem::E2(TableIdX(t), expr, _rt, vec) => match &expr.instrs[..] {
                    [Instr::x41_i32_const(off)] => {
                        for (i, v) in vec.into_iter().enumerate() {
                            tables[t as usize].table.insert(*off as u32 + i as u32, v);
                        }
                    }
                    _ => panic!(),
                },
                Elem::E3(_, _) => todo!(),
                Elem::E4(_, _) => todo!(),
                Elem::E5(_, _) => todo!(),
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

        Self {
            functions,
            tables,
            function_types,
        }
    }
}
