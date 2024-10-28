use crate::parser::{FuncType, ImportDesc, Instr, Module, Name, TypeIdX};
use std::collections::HashMap;

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
    },
}

impl std::fmt::Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Import { ty, module, name } => f
                .debug_struct("Import")
                .field("ty", ty)
                .field("module", module)
                .field("name", name)
                .finish(),
            Self::Local {
                ty,
                locals,
                code: _,
            } => f
                .debug_struct("Local")
                .field("ty", ty)
                .field("locals", locals)
                .field("code", &"...")
                .finish(),
        }
    }
}

#[derive(Debug)]
pub struct Model {
    functions: HashMap<u32, Function>,
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
            let k = k + import_count;
            let ty = code.code.t;
            let locals = ty.iter().enumerate().map(|(s, _)| s).collect();
            let ty = value.types.function_types[k].clone();

            functions.insert(
                k as u32,
                Function::Local {
                    ty,
                    locals,
                    code: code.code.e.instrs,
                },
            );
        }

        Self { functions }
    }
}
