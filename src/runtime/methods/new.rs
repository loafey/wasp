use super::super::{
    clean_model::Model,
    error::{RuntimeError, RuntimeError::*},
    Frame, Runtime, Value,
};
use crate::{
    parser::{ExportDesc, FuncIdx, Global, Instr::*, Module, Parsable},
    runtime::{FuncId, Import},
};
use std::{
    collections::HashMap,
    fs::File,
    io::{Cursor, Read},
    path::Path,
};

impl Runtime {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, RuntimeError> {
        let mut buf = Vec::new();

        let mut f = File::open(path.as_ref()).expect("Failed to open file");
        f.read_to_end(&mut buf).expect("Failed to read file");

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

        let stack = if let Some(ExportDesc::Func(FuncIdx(main_id))) = module
            .exports
            .exports
            .iter()
            .find(|s| matches!(&*s.nm.0, "main" | "_start"))
            .map(|f| f.d)
        {
            vec![Frame {
                func_id: FuncId::Id(main_id),
                pc: 0,
                module: "_$_main_$_".to_string(),
                stack: Vec::new(),
                locals: HashMap::new(),
                // labels: HashMap::new(),
                depth_stack: Vec::new(),
            }]
        } else {
            Vec::new()
        };

        let module = Model::try_from(module)?;

        let mut modules = HashMap::new();
        modules.insert("_$_main_$_".to_string(), Import::WS(module));
        Ok(Self {
            modules,
            stack,
            _path: path.as_ref().to_path_buf(),
        })
    }
}
