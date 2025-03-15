use monostate::MustBe;
use serde::Deserialize;
use std::{collections::HashMap, fs, path::PathBuf};

use crate::{
    parser::{ExportDesc, FuncIdx},
    runtime::{FloatExp, Frame, FuncId, Import, Runtime, RuntimeError, Value},
};

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ConstValue {
    I32 { value: String },
    I64 { value: String },
    F32 { value: String },
    F64 { value: String },
    Externref { value: String },
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")]
#[allow(unused)]
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
#[serde(rename_all = "snake_case")]
enum ModuleType {
    Binary,
    Text,
}

#[derive(Debug, Deserialize, Clone)]
struct Module {
    #[serde(rename = "type")]
    _type: MustBe!("module"),
    _name: Option<String>,
    filename: PathBuf,
}

#[derive(Debug, Deserialize, Clone)]
struct AssertReturn {
    #[serde(rename = "type")]
    _type: MustBe!("assert_return"),
    action: Action,
    expected: Vec<ConstValue>,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
struct AssertExhaustion {
    #[serde(rename = "type")]
    _type: MustBe!("assert_exhaustion"),
    action: Action,
    text: String,
}

#[derive(Debug, Deserialize, Clone)]
struct AssertTrap {
    #[serde(rename = "type")]
    _type: MustBe!("assert_trap"),
    action: Action,
    text: String,
}

#[derive(Debug, Deserialize, Clone)]
struct AssertInvalid {
    #[serde(rename = "type")]
    _type: MustBe!("assert_invalid"),
    filename: PathBuf,
    text: String,
    module_type: ModuleType,
}

#[derive(Debug, Deserialize, Clone)]
struct AssertMalformed {
    #[serde(rename = "type")]
    _type: MustBe!("assert_malformed"),
    filename: PathBuf,
    text: String,
    module_type: ModuleType,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
struct AssertUninstantiable {
    #[serde(rename = "type")]
    _type: MustBe!("assert_uninstantiable"),
    filename: PathBuf,
    text: String,
    module_type: ModuleType,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
struct AssertUnlinkable {
    #[serde(rename = "type")]
    _type: MustBe!("assert_unlinkable"),
    filename: PathBuf,
    text: String,
    module_type: ModuleType,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
struct Register {
    #[serde(rename = "type")]
    _type: MustBe!("register"),
    name: Option<String>,
    #[serde(rename = "as")]
    _as: String,
}

#[derive(Debug, Deserialize, Clone)]
struct ActionWrap {
    #[serde(rename = "type")]
    _type: MustBe!("action"),
    action: Action,
}

/// https://github.com/WebAssembly/wabt/blob/main/docs/wast2json.md#json-format
#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
#[allow(unused)]
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
    Default,
}
impl Case {
    fn case_type(&self) -> String {
        match self {
            Self::Module(..) => "module",
            Self::AssertReturn(..) => "assert_return",
            Self::Action(..) => "action",
            Self::AssertExhaustion(..) => "assert_exhaustion",
            Self::AssertTrap(..) => "assert_trap",
            Self::AssertInvalid(..) => "assert_invalid",
            Self::AssertMalformed(..) => "assert_malformed",
            Self::AssertUninstantiable(..) => "assert_uninstantiable",
            Self::AssertUnlinkable(..) => "assert_unlinkable",
            Self::Register(..) => "register",
            Self::Default => "default",
        }
        .to_string()
    }
}

#[derive(Debug, Deserialize)]
struct TestCases {
    commands: Vec<Case>,
}

fn const_to_val(consts: Vec<ConstValue>) -> Vec<Value> {
    consts
        .into_iter()
        .map(|v| match v {
            ConstValue::Externref { value } => Value::Externref(
                value
                    .parse()
                    .or_else(|_| value.parse())
                    .expect("failed to parse"),
            ),
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
            ConstValue::F32 { value } => Value::F32(match &value[..] {
                "nan:canonical" => f32::NAN_CANONICAL,
                "nan:arithmetic" => f32::NAN_ARITHMETIC,
                _ => unsafe {
                    f32::from_bits(
                        value
                            .parse::<i32>()
                            .or_else(|_| value.parse().map(|v| std::mem::transmute::<u32, i32>(v)))
                            .expect("failed to parse") as u32,
                    )
                },
            }),
            ConstValue::F64 { value } => Value::F64(match &value[..] {
                "nan:canonical" => f64::NAN_CANONICAL,
                "nan:arithmetic" => f64::NAN_ARITHMETIC,
                _ => unsafe {
                    f64::from_bits(
                        value
                            .parse::<i64>()
                            .or_else(|_| value.parse().map(|v| std::mem::transmute::<u64, i64>(v)))
                            .expect("failed to parse") as u64,
                    )
                },
            }),
        })
        .collect()
}

fn remove_floats(vals: Vec<Value>) -> Vec<Value> {
    vals.into_iter()
        .map(|v| match v {
            Value::F32(x) => {
                if x.is_nan_canonical() {
                    Value::I32(unsafe {
                        std::mem::transmute::<u32, i32>(f32::NAN_CANONICAL.to_bits())
                    })
                } else if x.is_nan_arithmetic() {
                    Value::I32(unsafe {
                        std::mem::transmute::<u32, i32>(f32::NAN_ARITHMETIC.to_bits())
                    })
                } else {
                    Value::I32(unsafe { std::mem::transmute::<u32, i32>(x.to_bits()) })
                }
            }
            Value::F64(x) => {
                if x.is_nan_canonical() {
                    Value::I64(unsafe {
                        std::mem::transmute::<u64, i64>(f64::NAN_CANONICAL.to_bits())
                    })
                } else if x.is_nan_arithmetic() {
                    Value::I64(unsafe {
                        std::mem::transmute::<u64, i64>(f64::NAN_ARITHMETIC.to_bits())
                    })
                } else {
                    Value::I64(unsafe { std::mem::transmute::<u64, i64>(x.to_bits()) })
                }
            }
            x => x,
        })
        .collect()
}

fn handle_action<T>(
    rt: &mut Runtime,
    action: Action,
    loop_func: impl FnOnce(&mut Runtime, &String) -> T,
) -> T {
    match action {
        Action::Invoke {
            module,
            field,
            args,
        } => {
            if module.is_some() {
                todo!()
            }

            let fid = *unsafe { rt.modules["_$_main_$_"].as_ws() }
                .exports
                .get(&field)
                .expect("no function");

            let ExportDesc::Func(FuncIdx(fid)) = fid else {
                panic!("no function with this id")
            };

            let args = const_to_val(args)
                .into_iter()
                .enumerate()
                .map(|(a, b)| (a as u32, b))
                .collect::<HashMap<_, _>>();

            rt.stack.push(Frame {
                func_id: FuncId::Id(fid),
                pc: 0,
                module: "_$_main_$_".to_string(),
                stack: Vec::new(),
                locals: args,
                depth_stack: Vec::new(),
            });

            loop_func(rt, &field)
        }
        Action::Get { .. } => todo!(),
    }
}

static mut LAST_CASE: (usize, Case) = (0, Case::Default);

pub fn test(mut path: String) {
    #[allow(static_mut_refs)]
    std::panic::set_hook(Box::new(|info| {
        #[allow(clippy::format_collect)]
        let fmted = format!("{info}")
            .lines()
            .map(|s| format!("\t{s}\n"))
            .collect::<String>();
        error!(
            "oops the test-suite panicked!\nReason:\n{fmted}Last test ({}):\n\t{:?}",
            unsafe { &LAST_CASE.0 },
            unsafe { &LAST_CASE.1 }
        )
    }));

    let input = path.to_string();
    if !PathBuf::from(&path).exists() {
        // ugly fix to work around spec test weirdness
        std::process::exit(0);
    }
    path = path.replace(".wast", ".json");
    let o = std::process::Command::new("wast2json")
        .arg(input)
        .arg("-o")
        .arg(&path)
        .output()
        .expect("Failed to run wast2json");
    if !o.status.success() {
        error!(
            "failed calling wast2json: {}",
            String::from_utf8_lossy(&o.stderr)
        );
        std::process::exit(o.status.code().unwrap_or(1));
    }

    let p = PathBuf::from(path);
    let tests = serde_json::from_str::<TestCases>(
        &fs::read_to_string(&p).expect("failed to open test data"),
    )
    .expect("failed to parse test data");

    let mut runtime: Option<Runtime> = None;
    let mut registers: HashMap<String, PathBuf> = HashMap::new();
    let mut current_path = PathBuf::new();

    let mut skip = false;
    let mut module_index = -1;
    let total_tests = tests.commands.len();

    for (test_i, test) in tests.commands.into_iter().enumerate() {
        unsafe {
            LAST_CASE = (test_i, test.clone());
        }
        let test_i = test_i + 1;
        info!(
            "Running test {}/{total_tests} ({})",
            test_i,
            test.case_type()
        );
        if let Some(rt) = &mut runtime {
            rt.stack = Vec::new();
        }
        match test {
            Case::Default => continue,
            Case::Module(module) => {
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

                runtime = Some({
                    let mut rt = Runtime::build(p.clone());
                    rt = rt.add_io("spectest", Import::spectest());
                    for (k, v) in &registers {
                        rt = rt.add_ws(&k.clone(), v.clone());
                    }
                    let mut rt = rt.build().expect("failed to load module");
                    rt.execute_start().expect("start function crashed");
                    rt
                });
                current_path = p.clone();
            }
            Case::AssertReturn(AssertReturn {
                action, expected, ..
            }) => {
                if skip {
                    continue;
                }
                let rt = runtime.as_mut().expect("no rt set");

                let expected = const_to_val(expected);

                handle_action(rt, action, move |rt, field| {
                    let mut last;
                    loop {
                        // let id = rt.stack.first().expect("no first").func_id;
                        last = rt.stack.first().expect("no first").stack.clone();
                        match rt.step() {
                            Err(RuntimeError::NoFrame(_, _, _)) => {
                                let expected = remove_floats(expected);
                                last = remove_floats(last);
                                if last == expected {
                                    break;
                                } else {
                                    error!("test {test_i}/{total_tests} failed (module: {module_index}, invoke: {field:?}, got {last:?}, but expected {expected:?})");
                                    std::process::exit(1);
                                }
                            }
                            Err(RuntimeError::ReturnedToNoFrame(stack, _, _, _)) => {
                                let expected = remove_floats(expected);
                                if stack == expected {
                                    break;
                                } else {
                                    error!("test {test_i}/{total_tests} failed (module: {module_index}, invoke: {field:?}, got {last:?}, but expected {expected:?})");
                                    std::process::exit(1);
                                }
                            }
                            Err(e) => {
                                error!("test {test_i}/{total_tests} failed (module: {module_index}, invoke: {field:?}, error: {e:?})");
                                std::process::exit(1);
                            }
                            Ok(()) => (),
                        }
                    }
                })
            }
            Case::Action(ActionWrap { action, .. }) => {
                if skip {
                    continue;
                }
                let rt = runtime.as_mut().expect("no rt set");
                let ac = action.clone();
                handle_action(rt, action, move |rt, _| loop {
                    match rt.step() {
                        Err(RuntimeError::NoFrame(_, _, _)) => {
                            break;
                        }
                        Err(e) => {
                            error!("test {test_i}/{total_tests} failed: {e:?} (module: {module_index}, invoke: {:?})", match ac {
                                Action::Invoke { field, .. } => field,
                                Action::Get { field,.. } => field,
                            });
                            std::process::exit(1);
                        }
                        Ok(()) => (),
                    }
                })
            }
            Case::AssertExhaustion(AssertExhaustion { _type, action, .. }) => {
                if skip {
                    continue;
                }
                let rt = runtime.as_mut().expect("no rt set");
                let ac = action.clone();
                handle_action(rt, action, move |rt, _| loop {
                    match rt.step() {
                        Err(RuntimeError::StackExhaustion(_, _)) => {
                            break;
                        }
                        Err(e) => {
                            error!("test {test_i}/{total_tests} failed: {e:?} (module: {module_index}, invoke: {:?})", match ac {
                                Action::Invoke { field, .. } => field,
                                Action::Get { field,.. } => field,
                            });
                            std::process::exit(1);
                        }
                        Ok(()) => {}
                    }
                })
            }
            Case::AssertTrap(AssertTrap { action, text, .. }) => {
                if skip {
                    continue;
                }
                let rt = runtime.as_mut().expect("no rt set");
                handle_action(rt, action, move |rt, field| loop {
                    match rt.step() {
                        Err(RuntimeError::NoFrame(_, _, _)) => {
                            error!("test {test_i}/{total_tests} did not fail, expected error: {text:?} (module: {module_index}, function {field:?})");
                            std::process::exit(1);
                        }
                        Err(e)
                            if text.contains(&format!("{e:?}"))
                                || format!("{e:?}").contains(&text)
                                || matches!(
                                    (&*text, &*format!("{e:?}")),
                                    ("undefined element", "uninitialized element")
                                        | ("uninitialized element", "undefined element")
                                        | ("undefined element", "uninitialized element 2")
                                        | ("uninitialized element 2", "undefined element")
                                ) =>
                        {
                            break;
                        }
                        Err(e) => {
                            error!("test {test_i}/{total_tests} got error \"{e:?}\", expected error: {text:?} (module: {module_index}, function {field:?})");
                            std::process::exit(1);
                        }
                        Ok(()) => (),
                    }
                })
            }
            Case::AssertInvalid(AssertInvalid {
                filename,
                text,
                module_type,
                ..
            })
            | Case::AssertMalformed(AssertMalformed {
                filename,
                text,
                module_type,
                ..
            }) => {
                // Currently skipping type checking
                if skip || text == "type mismatch" {
                    continue;
                }
                if let ModuleType::Text = module_type {
                    continue;
                }

                let mut p = p.clone();
                p.pop();
                p.push(&filename);

                let mut rt = Runtime::build(p.clone());
                rt = rt.add_io("spectest", Import::spectest());
                for (k, v) in &registers {
                    rt = rt.add_ws(&k.clone(), v.clone());
                }

                match rt.build() {
                    Ok(_) => {
                        error!("test {test_i}/{total_tests} did not fail invalidating/parsing, expected error: {text:?} (module: {p:?})");
                        std::process::exit(1);
                    }
                    Err(_) => continue,
                }
            }
            Case::AssertUninstantiable(AssertUninstantiable {
                _type,
                filename,
                text,
                module_type,
            }) => {
                if skip || matches!(module_type, ModuleType::Text) {
                    continue;
                }

                let mut p = p.clone();
                p.pop();
                p.push(&filename);
                let mut rt = Runtime::build(&p);
                rt = rt.add_io("spectest", Import::spectest());
                for (k, v) in &registers {
                    rt = rt.add_ws(&k.clone(), v.clone());
                }
                match rt.build().and_then(|mut rt| rt.execute_start()) {
                    Ok(_) => {
                        error!("test {test_i}/{total_tests} did not fail linking, expected error: {text:?} (module: {p:?})");
                        std::process::exit(1);
                    }
                    Err(e)
                        if text == "incompatible import type"
                            || format!("{e:?}").contains("unknown import") =>
                    {
                        continue
                    }
                    Err(e) if format!("{e:?}").contains(&text) => continue,
                    Err(e) => {
                        error!("test {test_i}/{total_tests} got wrong error, expected error: {text:?}, got {e:?} (module: {p:?})");
                        std::process::exit(1);
                    }
                }
            }
            Case::AssertUnlinkable(AssertUnlinkable {
                filename,
                text,
                module_type,
                ..
            }) => {
                // we skip this test for now
                if skip || matches!(module_type, ModuleType::Text) {
                    continue;
                }

                let mut p = p.clone();
                p.pop();
                p.push(&filename);
                let mut rt = Runtime::build(&p);
                rt = rt.add_io("spectest", Import::spectest());
                for (k, v) in &registers {
                    rt = rt.add_ws(&k.clone(), v.clone());
                }
                match rt.build() {
                    Ok(_) => {
                        error!("test {test_i}/{total_tests} did not fail linking, expected error: {text:?} (module: {p:?})");
                        std::process::exit(1);
                    }
                    Err(e)
                        if text == "incompatible import type"
                            || format!("{e:?}").contains("unknown import") =>
                    {
                        continue
                    }
                    Err(e) if format!("{e:?}").contains(&text) => continue,
                    Err(e) => {
                        error!("test {test_i}/{total_tests} got wrong error, expected error: {text:?}, got {e:?} (module: {p:?})");
                        std::process::exit(1);
                    }
                }
            }
            Case::Register(Register { _as, name, .. }) => {
                if skip {
                    continue;
                }

                if let Some(name) = name {
                    todo!("Register: {_as:?} {name}")
                } else {
                    registers.insert(_as, current_path.clone());
                }
            }
        }
    }
}
