use serde::Deserialize;
use serde_json::Value;
use std::{fs, path::PathBuf};

use crate::runtime;

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
    source_filename: String,
    commands: Vec<Case>,
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
    let mut runtime = None;
    for test in tests.commands {
        match test {
            Case::Module(module) => {
                let mut p = p.clone();
                p.pop();
                p.push(&module.filename);
                runtime = Some(crate::runtime(p).expect("failed to load module"));
            }
            Case::AssertReturn(assert_return) => todo!("AssertReturn"),
            Case::Action(action_wrap) => todo!("Action"),
            Case::AssertExhaustion(assert_exhaustion) => todo!("AssertExhaustion"),
            Case::AssertTrap(assert_trap) => todo!("AssertTrap"),
            Case::AssertInvalid(assert_invalid) => todo!("AssertInvalid"),
            Case::AssertMalformed(assert_malformed) => todo!("AssertMalformed"),
            Case::AssertUninstantiable(assert_uninstantiable) => todo!("AssertUninstantiable"),
            Case::AssertUnlinkable(assert_unlinkable) => todo!("AssertUnlinkable"),
            Case::Register(register) => todo!("Register"),
        }
    }
}
