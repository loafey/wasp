use crate::parser::{BlockType, FuncType, ImportDesc, Instr, Module, Name, Table, TypeIdX, BT};
use std::collections::HashMap;

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
        labels: HashMap<u32, u32>,
    },
}

#[derive(Debug)]
pub struct Model {
    pub functions: HashMap<u32, Function>,
    pub tables: Vec<Table>,
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
                        code.insert(pc, Instr::block_end(BT::Block));
                        for (_, i) in c.into_iter().enumerate().rev() {
                            code.insert(pc, i);
                        }
                        code.insert(pc, Instr::block_start(BT::Block));
                    }
                    Instr::x03_loop(bt, ins) => {
                        match bt {
                            BlockType::Eps => {}
                            BlockType::T(_) => todo!(),
                            BlockType::X(_) => todo!(),
                        }

                        let c = ins.clone();

                        code.remove(pc);
                        code.insert(pc, Instr::block_end(BT::Loop));
                        for (_, i) in c.into_iter().enumerate().rev() {
                            code.insert(pc, i);
                        }
                        code.insert(pc, Instr::block_start(BT::Loop));
                    }
                    _ => {}
                }
                pc += 1;
            }
            // let code = code
            //     .into_iter()
            //     .map(|i| match i {
            //         Instr::x02_block(bt, ins) => {
            //             match bt {
            //                 BlockType::Eps => {}
            //                 BlockType::T(_) => todo!(),
            //                 BlockType::X(_) => todo!(),
            //             }

            //             let c = ins.clone();
            //             let func = self.module.functions.get_mut(&f.func_id).unwrap();
            //             let Function::Local { code, labels, .. } = func else {
            //                 unreachable!()
            //             };

            //             code.remove(f.pc - 1);
            //             let pos_before = f.pc;
            //             let mut modified = 0;
            //             code.insert(f.pc - 1, Instr::block_end(BT::Block));
            //             for (_, i) in c.into_iter().enumerate().rev() {
            //                 modified += 1;
            //                 code.insert(f.pc - 1, i);
            //             }
            //             labels.iter_mut().for_each(|(_, r)| {
            //                 if (*r as usize) >= f.pc {
            //                     *r += modified as u32 + 1
            //                 }
            //             });
            //             labels.insert(labels.len() as u32, (pos_before + modified) as u32);
            //             code.insert(f.pc - 1, Instr::block_start(BT::Block));
            //             f.depth += 1;
            //         }
            //         Instr::block_end(bt) => todo!(),
            //         i => i,
            //     })
            //     .collect();

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

        Self {
            functions,
            tables: value.tables.tables,
        }
    }
}
