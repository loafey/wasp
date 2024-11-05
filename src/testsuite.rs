use serde::Deserialize;
use std::{fs, path::PathBuf};

/// https://github.com/WebAssembly/wabt/blob/main/docs/wast2json.md#json-format
#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")]
enum Case {
    #[serde(rename = "module")]
    Module {},
    #[serde(rename = "action")]
    Action {},
    #[serde(rename = "assert_return")]
    AssertReturn {},
    #[serde(rename = "assert_exhaustion")]
    AssertExhaustion {},
    #[serde(rename = "assert_trap")]
    AssertTrap {},
    #[serde(rename = "assert_invalid")]
    AssertInvalid {},
    #[serde(rename = "assert_malformed")]
    AssertMalformed {},
    #[serde(rename = "assert_uninstantiable")]
    AssertUninstantiable {},
    #[serde(rename = "assert_unlinkable")]
    AssertUnlinkable {},
    #[serde(rename = "register")]
    Register {},
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
    todo!("{tests:#?}")
    // for test in tests.commands {
    //     let mut p = p.clone();
    //     p.pop();
    //     p.push(&test.filename);
    //     let mut rt = match (runtime(p), test.type_) {
    //         (Ok(rt), TestType::Module) => rt,
    //         (Ok(_) | Err(RuntimeError::NoMain), TestType::AssertMalformed) => {
    //             error!("A malformed test passed parsing: {test:?}");
    //             std::process::exit(1);
    //         }
    //         (Err(RuntimeError::NoMain), TestType::Module) => continue,
    //         (Err(RuntimeError::ParseError(_)), TestType::AssertMalformed) => continue,
    //         (Err(RuntimeError::ParseError(err)), TestType::Module) => {
    //             error!("A valid module failed parsing: {err}");
    //             std::process::exit(1);
    //         }
    //         (_, _) => {
    //             error!("Unhandled test: {test:?}");
    //             std::process::exit(1);
    //         }
    //     };
    //     loop {
    //         if let Err(e) = rt.step() {
    //             match e {
    //                 RuntimeError::Exit(x) => {
    //                     if x != 0 {
    //                         std::process::exit(x);
    //                     } else {
    //                         break;
    //                     }
    //                 }
    //                 e => {
    //                     error!("{e:?}");
    //                     std::process::exit(1);
    //                 }
    //             }
    //         }
    //     }
    // }
}
