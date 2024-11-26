use std::collections::HashMap;

use super::{clean_model::Model, RuntimeError, RuntimeError::*, Value};

pub type Locals<'t> = &'t HashMap<u32, Value>;
pub type Stack = Vec<Value>;
pub type Function = &'static dyn Fn(Locals) -> Result<Stack, RuntimeError>;

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
pub enum Import {
    WS(Model),
    IO(HashMap<&'static str, Function>),
}
impl Import {
    pub unsafe fn as_ws(&self) -> &Model {
        match &self {
            Import::WS(model) => &model,
            Import::IO(_) => panic!(),
        }
    }

    #[allow(clippy::print_stdout)]
    pub fn spectest() -> Self {
        let map: Vec<(&'static str, Function)> = vec![
            ("print_i32", &|locals| {
                let a = *get!(i32, &0, locals);
                println!("{a}");
                Ok(Vec::new())
            }),
            ("print_i64", &|locals| {
                let a = *get!(i64, &0, locals);
                println!("{a}");
                Ok(Vec::new())
            }),
            #[allow(clippy::print_stdout)]
            ("print_i32_f32", &|locals| {
                let b = *get!(f32, &1, locals);
                let a = *get!(i32, &0, locals);
                println!("{a} {b}");
                Ok(Vec::new())
            }),
            #[allow(clippy::print_stdout)]
            ("print_i64_f64", &|locals| {
                let b = *get!(f64, &1, locals);
                let a = *get!(i64, &0, locals);
                println!("{a} {b}");
                Ok(Vec::new())
            }),
            #[allow(clippy::print_stdout)]
            ("print_f64_f64", &|locals| {
                let b = *get!(f64, &1, locals);
                let a = *get!(f64, &0, locals);
                println!("{a} {b}");
                Ok(Vec::new())
            }),
            #[allow(clippy::print_stdout)]
            ("print_f32", &|locals| {
                let a = *get!(f32, &0, locals);
                println!("{a}");
                Ok(Vec::new())
            }),
            #[allow(clippy::print_stdout)]
            ("print_f64", &|locals| {
                let a = *get!(f64, &0, locals);
                println!("{a}");
                Ok(Vec::new())
            }),
        ];
        let mut res = HashMap::new();
        for (k, v) in map {
            res.insert(k, v);
        }
        Self::IO(res)
    }
}
