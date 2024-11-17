use crate::parser::{
    BlockType, Elem, Expr, FuncIdx, FuncType, ImportDesc, Instr, LabelIdX, LocalIdX, Module, Name,
    TableIdX, TypeIdX, BT,
};
use std::{collections::HashMap, ops::Deref};

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

fn increment_labels(instrs: &mut [Instr], depth: u32) {
    for i in instrs {
        match i {
            Instr::x0c_br(LabelIdX(i)) => *i += depth,
            Instr::x0d_br_if(LabelIdX(i)) => *i += depth,
            Instr::x0e_br_table(ls, LabelIdX(i)) => {
                *i += depth;
                ls.iter_mut().for_each(|LabelIdX(i)| *i += depth);
            }
            Instr::x02_block(_, v) => increment_labels(v, depth),
            Instr::x03_loop(_, v) => increment_labels(v, depth),
            Instr::x04_if_else(_, v1, v2) => {
                increment_labels(v1, depth);
                if let Some(v2) = v2 {
                    increment_labels(v2, depth);
                }
            }
            _ => {}
        }
    }
}

#[derive(Debug)]
pub struct Model {
    pub functions: HashMap<u32, Function>,
    pub tables: Vec<Table>,
    pub _passive_elems: HashMap<u32, Expr>,
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
            .map(|_| Table {
                table: HashMap::new(),
            })
            .collect();

        let mut passive_elems = HashMap::new();
        for (i, elem) in value.elems.elems.into_iter().enumerate() {
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
                Elem::E5(_rt, vec) => {
                    // might be wrong
                    for expr in vec {
                        passive_elems.insert(i as u32, expr);
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

        Self {
            functions,
            tables,
            function_types,
            _passive_elems: passive_elems,
        }
    }
}
