use super::super::{clean_model::Model, error::RuntimeError, Frame, Runtime};
use crate::{
    parser::{ExportDesc, FuncIdx, Module, Parsable},
    runtime::{FuncId, Import, IO},
};
use std::{
    collections::{BTreeSet, HashMap},
    fs::File,
    io::{Cursor, Read},
    path::{Path, PathBuf},
};

pub enum ToImport {
    IO(IO),
    WS(PathBuf),
}

#[allow(clippy::large_enum_variant)]
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
        let mut non_ordered = HashMap::new();
        self.modules
            .insert("_$_main_$_".to_string(), ToImport::WS(self.path.clone()));
        for (k, v) in self.modules {
            non_ordered.insert(
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
        let mut deps = non_ordered
            .iter()
            .map(|(k, v)| (k.clone(), v.get_dependencies()))
            .collect::<HashMap<_, _>>();

        let mut ordered = Vec::new();
        let mut old = deps.clone();
        while !deps.is_empty() {
            for (k, v) in &deps {
                if v.iter().all(|a| ordered.contains(a)) {
                    ordered.push(k.clone());
                }
            }
            for k in &ordered {
                deps.remove(k);
            }
            if deps == old {
                return Err(RuntimeError::UnknownImport(file!(), line!(), column!()));
                // panic!(
                //     "missing dependency or cycle:\n\tfixed: {ordered:?}\n\tdependencies left: {deps:?}"
                // )
            } else {
                old = deps.clone();
            }
            // println!("Ordered: {ordered:?}\tNon-ordered: {:?}", deps);
        }
        // println!("get_dependencies: {deps:?}");
        // do a topological sort here

        let mut modules = HashMap::new();
        for k in ordered {
            let r = match non_ordered.remove(&k) {
                Some(Intermediate::IO(io)) => Import::IO(io),
                Some(Intermediate::WS(module)) => Import::WS(Model::try_from((&modules, module))?),
                _ => panic!(),
            };
            modules.insert(k, r);
        }

        Runtime::new(modules)
    }
}

impl Runtime {
    pub fn build<P: AsRef<Path>>(path: P) -> RuntimeBuilder {
        RuntimeBuilder {
            path: path.as_ref().to_path_buf(),
            modules: HashMap::new(),
        }
    }

    fn new(modules: HashMap<String, Import>) -> Result<Self, RuntimeError> {
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
