use super::super::{
    clean_model::Model,
    error::{RuntimeError, RuntimeError::*},
    Frame, Runtime, Value,
};
use crate::{
    parser::{ExportDesc, FuncIdx, Global, Instr::*, Module, Parsable},
    runtime::{FuncId, Import, IO},
};
use std::{
    collections::{BTreeSet, HashMap},
    fs::File,
    io::{Cursor, Read},
    path::{Path, PathBuf},
};

enum ToImport {
    IO(IO),
    WS(PathBuf),
}

enum Intermediate {
    IO(IO),
    WS(Module),
}
impl Intermediate {
    pub fn get_dependencies(&self) -> BTreeSet<String> {
        match self {
            Intermediate::IO(_) => BTreeSet::new(),
            Intermediate::WS(module) => module
                .imports
                .imports
                .iter()
                .map(|i| i.module.0.clone())
                .collect(),
        }
    }
}

pub struct RuntimeBuilder {
    path: PathBuf,
    modules: HashMap<String, ToImport>,
}
impl RuntimeBuilder {
    pub fn add_ws<P: AsRef<Path>>(mut self, name: &str, p: P) -> Self {
        self.modules
            .insert(name.to_string(), ToImport::WS(p.as_ref().to_path_buf()));
        self
    }

    pub fn add_io(mut self, name: &str, io: IO) -> Self {
        self.modules.insert(name.to_string(), ToImport::IO(io));
        self
    }

    pub fn build(mut self) -> Result<Runtime, RuntimeError> {
        let mut ordered = HashMap::new();
        self.modules
            .insert("_$_main_$_".to_string(), ToImport::WS(self.path.clone()));
        for (k, v) in self.modules {
            ordered.insert(
                k,
                match v {
                    ToImport::IO(io) => Intermediate::IO(io),
                    ToImport::WS(path) => {
                        let mut buf = Vec::new();

                        let mut f = File::open(&path).expect("Failed to open file");
                        f.read_to_end(&mut buf).expect("Failed to read file");

                        let mut cursor = Cursor::new(&buf[..]);
                        let mut stack = Vec::new();
                        let module = match Module::parse(&mut cursor, &mut stack) {
                            Ok(o) => o,
                            Err(e) => {
                                stack.reverse();
                                return Err(RuntimeError::ParseError(format!(
                                    "File: {:?}\n{e:?}, bin pos: {}, stack: {stack:#?}",
                                    path,
                                    cursor.position()
                                )));
                            }
                        };
                        Intermediate::WS(module)
                    }
                },
            );
        }
        let deps = ordered
            .iter()
            .map(|(k, v)| (k, v.get_dependencies()))
            .collect::<HashMap<_, _>>();
        println!("get_dependencies: {deps:?}");
        // do a topological sort here

        // let rt = Runtime::new(ordered)?;
        todo!()

        // Ok(rt)
    }
}

impl Runtime {
    pub fn build<P: AsRef<Path>>(path: P) -> RuntimeBuilder {
        RuntimeBuilder {
            path: path.as_ref().to_path_buf(),
            modules: HashMap::new(),
        }
    }

    fn new<P: AsRef<Path>>(modules: HashMap<String, Import>) -> Result<Self, RuntimeError> {
        let stack = if let Some(ExportDesc::Func(FuncIdx(main_id))) = match &modules["_$_main_$_"] {
            Import::WS(module) => module,
            Import::IO(_) => unreachable!(),
        }
        .exports
        .iter()
        .find(|s| matches!(&**s.0, "main" | "_start"))
        .map(|f| f.1)
        {
            vec![Frame {
                func_id: FuncId::Id(*main_id),
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

        Ok(Self { modules, stack })
    }
}
