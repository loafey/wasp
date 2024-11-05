use serde::Deserialize;
use std::{cell::RefCell, collections::HashMap, fs, path::PathBuf, rc::Rc};

use crate::{
    parser::{ExportDesc, FuncIdx, TypeIdX},
    runtime::{self, Frame, RuntimeError, Value},
};

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ConstValue {
    I32 { value: String },
    I64 { value: String },
    F32 { value: String },
    F64 { value: String },
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")]
enum Action {
    #[serde(rename = "invoke")]
    Invoke {
        module: Option<String>,
        field: String,
        args: Vec<ConstValue>,
    },
    #[serde(rename = "get")]
    Get {
        module: Option<String>,
        field: String,
    },
}

#[derive(Debug, Deserialize, Clone)]
enum ModuleType {}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
struct Module {
    name: Option<String>,
    filename: PathBuf,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
struct AssertReturn {
    action: Action,
    expected: Vec<ConstValue>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
struct AssertExhaustion {
    action: Action,
    text: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
struct AssertTrap {
    action: Action,
    text: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
struct AssertInvalid {
    filename: PathBuf,
    text: String,
    module_type: ModuleType,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
struct AssertMalformed {
    filename: PathBuf,
    text: String,
    module_type: ModuleType,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
struct AssertUninstantiable {
    filename: PathBuf,
    text: String,
    module_type: ModuleType,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
struct AssertUnlinkable {
    filename: PathBuf,
    text: String,
    module_type: ModuleType,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
struct Register {
    name: String,
    #[serde(rename = "as")]
    _as: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
struct ActionWrap {
    action: Action,
}

/// https://github.com/WebAssembly/wabt/blob/main/docs/wast2json.md#json-format
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case", untagged)]
enum Case {
    Module(Module),
    AssertReturn(AssertReturn),
    Action(ActionWrap),
    AssertExhaustion(AssertExhaustion),
    AssertTrap(AssertTrap),
    AssertInvalid(AssertInvalid),
    AssertMalformed(AssertMalformed),
    AssertUninstantiable(AssertUninstantiable),
    AssertUnlinkable(AssertUnlinkable),
    Register(Register),
}

#[derive(Debug, Deserialize)]
struct TestCases {
    commands: Vec<Case>,
}

fn const_to_val(consts: Vec<ConstValue>) -> Vec<Value> {
    consts
        .into_iter()
        .map(|v| match v {
            ConstValue::I32 { value } => Value::I32(
                value
                    .parse()
                    .or_else(|_| {
                        value
                            .parse()
                            .map(|v| unsafe { std::mem::transmute::<u32, i32>(v) })
                    })
                    .expect("failed to parse"),
            ),
            ConstValue::I64 { value } => Value::I64(
                value
                    .parse()
                    .or_else(|_| {
                        value
                            .parse()
                            .map(|v| unsafe { std::mem::transmute::<u64, i64>(v) })
                    })
                    .expect("failed to parse"),
            ),
            ConstValue::F32 { value } => Value::F32(value.parse().expect("failed to parse")),
            ConstValue::F64 { value } => Value::F64(value.parse().expect("failed to parse")),
        })
        .collect()
}

pub fn test(mut path: String) {
    let input = path.to_string();
    path = path.replace(".wast", ".wasm");
    std::process::Command::new("wast2json")
        .arg(input)
        .arg("-o")
        .arg(&path)
        .output()
        .expect("Failed to run wast2json");

    let p = PathBuf::from(path);
    let tests = serde_json::from_str::<TestCases>(
        &fs::read_to_string(&p).expect("failed to open test data"),
    )
    .expect("failed to parse test data");

    let runtime = Rc::new(RefCell::new(None));

    let mut recreate_runtime: Box<dyn Fn()> = Box::new(|| {});
    let mut skip = false;
    let mut module_index = -1;
    let total_tests = tests.commands.len();

    for (test_i, test) in tests.commands.into_iter().enumerate() {
        recreate_runtime();
        match test {
            Case::Module(module) => {
                let runtime = runtime.clone();
                let mut p = p.clone();
                p.pop();
                p.push(&module.filename);
                skip = module
                    .filename
                    .extension()
                    .map(|s| s == "wat")
                    .unwrap_or_default();
                module_index += 1;
                if skip {
                    continue;
                }
                recreate_runtime = Box::new(move || {
                    *runtime.borrow_mut() =
                        Some(crate::runtime(p.clone()).expect("failed to load module"));
                });
            }
            Case::AssertReturn(AssertReturn { action, expected }) => {
                if skip {
                    continue;
                }
                let mut rt = runtime.borrow_mut();
                let rt = rt.as_mut().expect("no rt set");
                match action {
                    Action::Invoke {
                        module,
                        field,
                        args,
                    } => {
                        if module.is_some() {
                            todo!()
                        }

                        let fid = rt.exports.get(&field).expect("no function");

                        let ExportDesc::Func(TypeIdX(fid)) = fid else {
                            panic!("no function with this id")
                        };

                        let args = const_to_val(args)
                            .into_iter()
                            .enumerate()
                            .map(|(a, b)| (a as u32, b))
                            .collect::<HashMap<_, _>>();

                        let expected = const_to_val(expected);

                        rt.stack.push(Frame {
                            func_id: *fid,
                            pc: 0,
                            stack: Vec::new(),
                            locals: args,
                            depth_stack: Vec::new(),
                        });

                        let mut last;
                        loop {
                            last = rt.stack.first().expect("no first").stack.clone();
                            match rt.step() {
                                Err(RuntimeError::NoFrame(_, _, _)) => {
                                    if last == expected {
                                        break;
                                    } else {
                                        let Value::F32(x) = last[0] else { panic!() };
                                        let x = x.to_bits();
                                        let Value::F32(y) = expected[0] else { panic!() };
                                        let y = y.to_bits();
                                        println!("Gotten:   {x:032b}");
                                        println!("Expected: {y:032b}");

                                        error!("test {test_i}/{total_tests} failed (module: {module_index}, invoke: {field:?}, got {last:?}, but expected {expected:?})");
                                        std::process::exit(1);
                                    }
                                }
                                Err(e) => {
                                    error!("{e:?}");
                                    std::process::exit(1);
                                }
                                Ok(()) => (),
                            }
                        }
                    }
                    Action::Get { module, field } => todo!("ActionGet"),
                }
            }
            Case::Action(ActionWrap { action }) => {
                if skip {
                    continue;
                }
                let mut rt = runtime.borrow_mut();
                let rt = rt.as_mut().expect("no rt set");
                match action {
                    Action::Invoke {
                        module,
                        field,
                        args,
                    } => {
                        if module.is_some() {
                            todo!()
                        }

                        let fid = rt.exports.get(&field).expect("no function");

                        let ExportDesc::Func(TypeIdX(fid)) = fid else {
                            panic!("no function with this id")
                        };

                        let args = const_to_val(args)
                            .into_iter()
                            .enumerate()
                            .map(|(a, b)| (a as u32, b))
                            .collect::<HashMap<_, _>>();

                        rt.stack.push(Frame {
                            func_id: *fid,
                            pc: 0,
                            stack: Vec::new(),
                            locals: args,
                            depth_stack: Vec::new(),
                        });

                        loop {
                            match rt.step() {
                                Err(RuntimeError::NoFrame(_, _, _)) => {
                                    break;
                                }
                                Err(e) => {
                                    error!("{e:?}");
                                    std::process::exit(1);
                                }
                                Ok(()) => (),
                            }
                        }
                    }
                    Action::Get { module, field } => todo!(),
                }
            }
            Case::AssertExhaustion(assert_exhaustion) => {
                if skip {
                    continue;
                }
                let mut rt = runtime.borrow_mut();
                let rt = rt.as_mut().expect("no rt set");
                todo!("AssertExhaustion")
            }
            Case::AssertTrap(assert_trap) => {
                if skip {
                    continue;
                }
                let mut rt = runtime.borrow_mut();
                let rt = rt.as_mut().expect("no rt set");
                todo!("AssertTrap")
            }
            Case::AssertInvalid(assert_invalid) => {
                if skip {
                    continue;
                }
                let mut rt = runtime.borrow_mut();
                let rt = rt.as_mut().expect("no rt set");
                todo!("AssertInvalid")
            }
            Case::AssertMalformed(assert_malformed) => {
                if skip {
                    continue;
                }
                let mut rt = runtime.borrow_mut();
                let rt = rt.as_mut().expect("no rt set");
                todo!("AssertMalformed")
            }
            Case::AssertUninstantiable(assert_uninstantiable) => {
                if skip {
                    continue;
                }
                let mut rt = runtime.borrow_mut();
                let rt = rt.as_mut().expect("no rt set");
                todo!("AssertUninstantiable")
            }
            Case::AssertUnlinkable(assert_unlinkable) => {
                if skip {
                    continue;
                }
                let mut rt = runtime.borrow_mut();
                let rt = rt.as_mut().expect("no rt set");
                todo!("AssertUnlinkable")
            }
            Case::Register(register) => {
                if skip {
                    continue;
                }
                let mut rt = runtime.borrow_mut();
                let rt = rt.as_mut().expect("no rt set");
                todo!("Register")
            }
        }
    }
}
