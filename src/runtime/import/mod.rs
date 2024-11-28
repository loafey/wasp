use std::collections::HashMap;

use crate::{
    parser::{ExportDesc, GlobalIdX},
    ptr::PtrRW,
};

use super::{
    clean_model::{Global, Model},
    memory::Memory,
    RuntimeError::{self, *},
    Value,
};

pub type Locals<'t> = &'t HashMap<u32, Value>;
pub type Mem<'t> = &'t mut Memory<{ 65536 + 1 }>;
pub type Stack = Vec<Value>;

pub type IOFunction = &'static dyn Fn(Locals, Mem) -> Result<Stack, RuntimeError>;

macro_rules! unwrap {
    ($expr:expr, $err:expr) => {
        $expr.ok_or($err(file!(), line!(), column!()))?
    };
}

macro_rules! throw {
    ($expr:expr) => {
        unwrap!(None, $expr)
    };
}

#[macro_export]
macro_rules! get {
    ($index:expr, $locals:expr) => {
        unwrap!($locals.get($index), MissingLocal)
    };
    (i32, $index:expr, $locals:expr) => {{
        let val = match unwrap!($locals.get($index), MissingLocal) {
            Value::I32(val) => val,
            x => throw!(|a, b, c| WrongType(a, "i32", x.as_str(), b, c)),
        };
        val
    }};
    (i64, $index:expr, $locals:expr) => {{
        let val = match unwrap!($locals.get($index), MissingLocal) {
            Value::I64(val) => val,
            x => throw!(|a, b, c| WrongType(a, "i64", x.as_str(), b, c)),
        };
        val
    }};
    (u32, $index:expr, $locals:expr) => {{
        let val = match unwrap!($locals.get($index), MissingLocal) {
            Value::I32(val) => val,
            x => throw!(|a, b, c| WrongType(a, "u32", x.as_str(), b, c)),
        };
        unsafe { std::mem::transmute::<i32, u32>(val) }
    }};
    (u64, $index:expr, $locals:expr) => {{
        let val = match unwrap!($locals.get($index), MissingLocal) {
            Value::I64(val) => val,
            x => throw!(|a, b, c| WrongType(a, "u64", x.as_str(), b, c)),
        };
        unsafe { std::mem::transmute::<i64, u64>(val) }
    }};
    (f32, $index:expr, $locals:expr) => {{
        let val = match unwrap!($locals.get($index), MissingLocal) {
            Value::F32(val) => val,
            x => throw!(|a, b, c| WrongType(a, "f32", x.as_str(), b, c)),
        };
        val
    }};
    (f64, $index:expr, $locals:expr) => {{
        let val = match unwrap!($locals.get($index), MissingLocal) {
            Value::F64(val) => val,
            x => throw!(|a, b, c| WrongType(a, "f64", x.as_str(), b, c)),
        };
        val
    }};
}

pub struct IO {
    pub functions: HashMap<&'static str, IOFunction>,
    pub globals: HashMap<&'static str, Value>,
    pub memory: PtrRW<Memory<{ 65536 + 1 }>>,
}
pub enum Import {
    WS(Model),
    IO(IO),
}
impl Import {
    pub fn get_global(&self, name: &str) -> Value {
        match self {
            Import::WS(model) => {
                let Some(ExportDesc::Global(GlobalIdX(i))) = model.exports.get(name) else {
                    unreachable!()
                };
                let global = model
                    .globals
                    .get(*i as usize)
                    .expect("missing global")
                    .read();
                match &*global {
                    Global::Native(value) => *value,
                    Global::Foreign(..) => todo!("re-export of global"),
                }
            }
            Import::IO(IO { globals, .. }) => *globals
                .get(name)
                .unwrap_or_else(|| panic!("missing global: {name}")),
        }
    }

    pub unsafe fn as_ws(&self) -> &Model {
        match &self {
            Import::WS(model) => model,
            Import::IO { .. } => panic!(),
        }
    }

    #[allow(clippy::print_stdout)]
    pub fn spectest() -> IO {
        let map: Vec<(&'static str, IOFunction)> = vec![
            ("print_i32", &|locals, _| {
                let a = *get!(i32, &0, locals);
                println!("{a}");
                Ok(Vec::new())
            }),
            ("print_i64", &|locals, _| {
                let a = *get!(i64, &0, locals);
                println!("{a}");
                Ok(Vec::new())
            }),
            #[allow(clippy::print_stdout)]
            ("print_i32_f32", &|locals, _| {
                let b = *get!(f32, &1, locals);
                let a = *get!(i32, &0, locals);
                println!("{a} {b}");
                Ok(Vec::new())
            }),
            #[allow(clippy::print_stdout)]
            ("print_i64_f64", &|locals, _| {
                let b = *get!(f64, &1, locals);
                let a = *get!(i64, &0, locals);
                println!("{a} {b}");
                Ok(Vec::new())
            }),
            #[allow(clippy::print_stdout)]
            ("print_f64_f64", &|locals, _| {
                let b = *get!(f64, &1, locals);
                let a = *get!(f64, &0, locals);
                println!("{a} {b}");
                Ok(Vec::new())
            }),
            #[allow(clippy::print_stdout)]
            ("print_f32", &|locals, _| {
                let a = *get!(f32, &0, locals);
                println!("{a}");
                Ok(Vec::new())
            }),
            #[allow(clippy::print_stdout)]
            ("print_f64", &|locals, _| {
                let a = *get!(f64, &0, locals);
                println!("{a}");
                Ok(Vec::new())
            }),
        ];
        let mut res = HashMap::new();
        for (k, v) in map {
            res.insert(k, v);
        }

        let mut globals = HashMap::new();
        globals.insert("global_i32", Value::I32(666));
        globals.insert("global_i64", Value::I64(666));
        globals.insert("global_f32", Value::F32(f32::from_bits(1143383654)));
        globals.insert(
            "global_f64",
            Value::F64(f64::from_bits(4649074691427585229)),
        );

        IO {
            functions: res,
            globals,
            memory: Memory::new(1, 1).into(),
        }
    }
}
