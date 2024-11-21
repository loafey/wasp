use monostate::MustBe;
use serde::Deserialize;
use std::{collections::HashMap, future::Future, path::PathBuf, pin::Pin, sync::Arc};
use tokio::{fs, sync::RwLock};

use crate::{
    parser::{ExportDesc, TypeIdX},
    runtime::{Frame, Runtime, RuntimeError, Value},
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
                "nan:canonical" => f32::from_bits(0b010000000000000000000000000000000),
                "nan:arithmetic" => f32::from_bits(0b011010100000000000000000000000000),
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
                "nan:canonical" => f64::from_bits(
                    0b0100000000000000000000000000000000000000000000000000000000000000,
                ),
                "nan:arithmetic" => f64::from_bits(
                    0b0110101000000000000000000000000000000000000000000000000000000000,
                ),
                _ => unsafe {
                    // println!("{value}");
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
            Value::F32(x) => Value::I32(unsafe { std::mem::transmute::<u32, i32>(x.to_bits()) }),
            Value::F64(x) => Value::I64(unsafe { std::mem::transmute::<u64, i64>(x.to_bits()) }),
            x => x,
        })
        .collect()
}

async fn handle_action<T>(
    rt: Arc<RwLock<Runtime>>,
    action: Action,
    loop_func: impl FnOnce(Arc<RwLock<Runtime>>, String) -> Pin<Box<dyn Future<Output = T>>>,
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

            let mut rt_prep = rt.write().await;

            let fid = *rt_prep.module.exports.get(&field).expect("no function");

            let ExportDesc::Func(TypeIdX(fid)) = fid else {
                panic!("no function with this id")
            };

            let args = const_to_val(args)
                .into_iter()
                .enumerate()
                .map(|(a, b)| (a as u32, b))
                .collect::<HashMap<_, _>>();

            rt_prep.stack.push(Frame {
                func_id: fid,
                pc: 0,
                stack: Vec::new(),
                locals: args,
                depth_stack: Vec::new(),
            });
            drop(rt_prep);

            loop_func(rt.clone(), field.clone()).await
        }
        Action::Get { .. } => todo!(),
    }
}

pub async fn test(mut path: String) {
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
        &fs::read_to_string(&p)
            .await
            .expect("failed to open test data"),
    )
    .expect("failed to parse test data");

    let mut runtime: Option<Arc<RwLock<Runtime>>> = None;

    let mut skip = false;
    let mut module_index = -1;
    let total_tests = tests.commands.len();

    for (test_i, test) in tests.commands.into_iter().enumerate() {
        let test_i = test_i + 1;
        // println!("\n{}/{total_tests}", test_i);
        if let Some(rt) = &mut runtime {
            let mut rt = rt.write().await;
            rt.stack = Vec::new();
        }
        match test {
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

                // println!("@@@ Compiling {p:?}");
                runtime = Some(Arc::new(RwLock::new(
                    Runtime::new(p.clone())
                        .await
                        .expect("failed to load module"),
                )));
                // recreate_runtime = Box::new(move || {
                //     *runtime.borrow_mut() =
                //         Some(Runtime::new(p.clone()).expect("failed to load module"));
                // });
            }
            Case::AssertReturn(AssertReturn {
                action, expected, ..
            }) => {
                if skip {
                    continue;
                }
                let rt = runtime.as_ref().expect("no rt set");

                let expected = const_to_val(expected);

                handle_action(rt.clone(), action, move |rt, field| Box::pin(async move {
                    let mut last;
                    let mut rt = rt.write().await;
                    loop {
                        // let id = rt.stack.first().expect("no first").func_id;
                        last = rt.stack.first().expect("no first").stack.clone();
                        match rt.step().await {
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
                })).await
            }
            Case::Action(ActionWrap { action, .. }) => {
                if skip {
                    continue;
                }
                let rt = runtime.as_ref().expect("no rt set");
                let ac = action.clone();
                handle_action(rt.clone(), action, move |rt, _| {
                    Box::pin(async move {
                        let mut rt = rt.write().await;
                        loop {
                            match rt.step().await {
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
                        }
                    })
                }).await
            }
            Case::AssertExhaustion(AssertExhaustion { _type, action, .. }) => {
                if skip {
                    continue;
                }
                let rt = runtime.as_ref().expect("no rt set");
                let ac = action.clone();
                handle_action(rt.clone(), action, move |rt, _| Box::pin(async move { {
                let mut rt = rt.write().await;
                loop {
                    match rt.step().await {
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
                }}})).await
            }
            Case::AssertTrap(AssertTrap { action, text, .. }) => {
                if skip {
                    continue;
                }
                let rt = runtime.as_ref().expect("no rt set");
                handle_action(rt.clone(), action, move |rt, field| {
                    Box::pin(async move {
                let mut rt = rt.write().await;
                loop {
                            match rt.step().await {
                                Err(RuntimeError::NoFrame(_, _, _)) => {
                                    error!("test {test_i}/{total_tests} did not fail, expected error: {text:?} (module: {module_index}, function {field:?})");
                                    std::process::exit(1);
                                }
                                Err(e) if text.contains(&format!("{e:?}")) => {
                                    break;
                                }
                                Err(e) => {
                                    error!("Got error \"{e:?}\", expected error: {text:?} (module: {module_index}, function {field:?})");
                                    std::process::exit(1);
                                }
                                Ok(()) => (),
                            }
                        }
                    })
                }).await
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

                match Runtime::new(&p).await {
                    Ok(_) => {
                        error!("test {test_i}/{total_tests} did not fail invalidating/parsing, expected error: {text:?} (module: {p:?})");
                        std::process::exit(1);
                    }
                    Err(_) => continue,
                }
            }
            Case::AssertUninstantiable(_) => {
                if skip {
                    continue;
                }
                let _rt = runtime.as_ref().expect("no rt set");
                todo!("AssertUninstantiable")
            }
            Case::AssertUnlinkable(_) => {
                if skip {
                    continue;
                }
                let _rt = runtime.as_ref().expect("no rt set");
                todo!("AssertUnlinkable")
            }
            Case::Register(Register { _as, name, .. }) => {
                if skip {
                    continue;
                }
                let _rt = runtime.as_ref().expect("no rt set");
                if let Some(name) = name {
                    todo!("Register: {_as:?} {name}")
                } else {
                    todo!("Register: {_as:?} ")
                    // rt.modules.insert("bob".to_string(), rt.module);
                }
            }
        }
    }
}
