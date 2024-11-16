# W.A.S.P

## Latest spec test 
💅: 13\
💩: 134
## Failed: test-suite/test/core/br.wast
```bash
Compiling "test-suite/test/core/br.0.wasm"
Checking (FuncType { input: [], output: [] })
Checking "type-i32"  (FuncType { input: [], output: [] })
Checking "type-i64"  (FuncType { input: [], output: [] })
Checking "type-f32"  (FuncType { input: [], output: [] })
Checking "type-f64"  (FuncType { input: [], output: [] })
Checking "type-i32-i32"  (FuncType { input: [], output: [] })
Checking "type-i64-i64"  (FuncType { input: [], output: [] })
Checking "type-f32-f32"  (FuncType { input: [], output: [] })
Checking "type-f64-f64"  (FuncType { input: [], output: [] })
Checking "type-i32-value"  (FuncType { input: [], output: [Num(I32)] })
Checking "type-i64-value"  (FuncType { input: [], output: [Num(I64)] })
Checking "type-f32-value"  (FuncType { input: [], output: [Num(F32)] })
Checking "type-f64-value"  (FuncType { input: [], output: [Num(F64)] })
Checking "type-f64-f64-value"  (FuncType { input: [], output: [Num(F64), Num(F64)] })
Checking "as-block-first"  (FuncType { input: [], output: [] })
Checking "as-block-mid"  (FuncType { input: [], output: [] })
Checking "as-block-last"  (FuncType { input: [], output: [] })
Checking "as-block-value"  (FuncType { input: [], output: [Num(I32)] })
Checking "as-loop-first"  (FuncType { input: [], output: [Num(I32)] })
Checking "as-loop-mid"  (FuncType { input: [], output: [Num(I32)] })
Checking "as-loop-last"  (FuncType { input: [], output: [Num(I32)] })
Checking "as-br-value"  (FuncType { input: [], output: [Num(I32)] })
Checking "as-br_if-cond"  (FuncType { input: [], output: [] })
Checking "as-br_if-value"  (FuncType { input: [], output: [Num(I32)] })
Checking "as-br_if-value-cond"  (FuncType { input: [], output: [Num(I32)] })
thread 'main' panicked at src/testsuite.rs:302:50:
failed to load module: type error: ReturnTypeMismatch([Num(I32)], [Num(I32), Num(I32)])
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/br_if.wast
```bash
Compiling "test-suite/test/core/br_if.0.wasm"
Checking (FuncType { input: [], output: [] })
Checking "type-i32"  (FuncType { input: [], output: [] })
Checking "type-i64"  (FuncType { input: [], output: [] })
Checking "type-f32"  (FuncType { input: [], output: [] })
thread 'main' panicked at src/runtime/typecheck.rs:325:32:
not yet implemented
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/br_table.wast
```bash
thread 'main' panicked at src/testsuite.rs:273:6:
failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 151, column: 2)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/bulk.wast
```bash
Compiling "test-suite/test/core/bulk.0.wasm"
Compiling "test-suite/test/core/bulk.1.wasm"
Checking (FuncType { input: [], output: [] })
Checking (FuncType { input: [], output: [] })
Compiling "test-suite/test/core/bulk.2.wasm"
Checking "fill"  (FuncType { input: [Num(I32), Num(I32), Num(I32)], output: [] })
thread 'main' panicked at src/runtime/typecheck.rs:447:42:
not yet implemented
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/call.wast
```bash
thread 'main' panicked at src/testsuite.rs:273:6:
failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 46, column: 2)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/call_indirect.wast
```bash
thread 'main' panicked at src/testsuite.rs:273:6:
failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 100, column: 2)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/comments.wast
```bash
Compiling "test-suite/test/core/comments.0.wasm"
Compiling "test-suite/test/core/comments.1.wasm"
Compiling "test-suite/test/core/comments.2.wasm"
Compiling "test-suite/test/core/comments.3.wasm"
Compiling "test-suite/test/core/comments.4.wasm"
Checking "f1"  (FuncType { input: [], output: [Num(I32)] })
thread 'main' panicked at src/testsuite.rs:302:50:
failed to load module: type error: ReturnTypeMismatch([Num(I32)], [Num(I32), Num(I32)])
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/conversions.wast
```bash
Compiling "test-suite/test/core/conversions.0.wasm"
thread 'main' panicked at src/testsuite.rs:302:50:
failed to load module: ParseError(File: "test-suite/test/core/conversions.0.wasm"
UnknownInstruction(<a8>), bin pos: 794, stack: [
    "wasp::parser::instr::Instr",
    "wasp::parser::expr::Expr",
    "wasp::parser::func::Func",
    "wasp::parser::code::Code",
    "alloc::vec::Vec<wasp::parser::code::Code>",
    "wasp::parser::codesec::CodeSection",
    "wasp::parser::module::Module",
])
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/custom.wast
```bash
Compiling "test-suite/test/core/custom.0.wasm"
thread 'main' panicked at src/testsuite.rs:302:50:
failed to load module: ParseError(File: "test-suite/test/core/custom.0.wasm"
SectionSizeMismatch(46, 267), bin pos: 267, stack: [
    "u8",
    "wasp::parser::customsec::CustomSection",
    "wasp::parser::module::Module",
])
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/data.wast
```bash
Compiling "test-suite/test/core/data.0.wasm"
Compiling "test-suite/test/core/data.1.wasm"
Compiling "test-suite/test/core/data.2.wasm"
Compiling "test-suite/test/core/data.3.wasm"
Compiling "test-suite/test/core/data.4.wasm"
Compiling "test-suite/test/core/data.5.wasm"
thread 'main' panicked at src/testsuite.rs:302:50:
failed to load module: ActiveDataWithoutOffset
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/elem.wast
```bash
thread 'main' panicked at src/testsuite.rs:273:6:
failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 71, column: 2)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/endianness.wast
```bash
Compiling "test-suite/test/core/endianness.0.wasm"
thread 'main' panicked at src/testsuite.rs:302:50:
failed to load module: ParseError(File: "test-suite/test/core/endianness.0.wasm"
UnknownInstruction(<bc>), bin pos: 557, stack: [
    "wasp::parser::instr::Instr",
    "wasp::parser::expr::Expr",
    "wasp::parser::func::Func",
    "wasp::parser::code::Code",
    "alloc::vec::Vec<wasp::parser::code::Code>",
    "wasp::parser::codesec::CodeSection",
    "wasp::parser::module::Module",
])
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/exports.wast
```bash
Compiling "test-suite/test/core/exports.0.wasm"
Checking "a"  (FuncType { input: [], output: [] })
Compiling "test-suite/test/core/exports.1.wasm"
Checking "b"  (FuncType { input: [], output: [] })
Compiling "test-suite/test/core/exports.2.wasm"
Checking "a"  (FuncType { input: [], output: [] })
Checking "b"  (FuncType { input: [], output: [] })
Compiling "test-suite/test/core/exports.3.wasm"
Checking "a"  (FuncType { input: [], output: [] })
Compiling "test-suite/test/core/exports.4.wasm"
Checking "c"  (FuncType { input: [], output: [] })
Compiling "test-suite/test/core/exports.5.wasm"
Checking "b"  (FuncType { input: [Num(I32)], output: [] })
Compiling "test-suite/test/core/exports.6.wasm"
Checking "a"  (FuncType { input: [], output: [] })
Compiling "test-suite/test/core/exports.7.wasm"
Checking "a"  (FuncType { input: [], output: [] })
Compiling "test-suite/test/core/exports.8.wasm"
Checking "a"  (FuncType { input: [], output: [] })
Compiling "test-suite/test/core/exports.9.wasm"
Checking "a"  (FuncType { input: [], output: [] })
Compiling "test-suite/test/core/exports.10.wasm"
Checking "a"  (FuncType { input: [], output: [] })
Compiling "test-suite/test/core/exports.11.wasm"
Checking "e"  (FuncType { input: [Num(I32)], output: [Num(I32)] })
thread 'main' panicked at src/testsuite.rs:219:17:
not yet implemented
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/f32.wast
```bash
Compiling "test-suite/test/core/f32.0.wasm"
thread 'main' panicked at src/testsuite.rs:302:50:
failed to load module: ParseError(File: "test-suite/test/core/f32.0.wasm"
UnknownInstruction(<93>), bin pos: 133, stack: [
    "wasp::parser::instr::Instr",
    "wasp::parser::expr::Expr",
    "wasp::parser::func::Func",
    "wasp::parser::code::Code",
    "alloc::vec::Vec<wasp::parser::code::Code>",
    "wasp::parser::codesec::CodeSection",
    "wasp::parser::module::Module",
])
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/f32_bitwise.wast
```bash
Compiling "test-suite/test/core/f32_bitwise.0.wasm"
thread 'main' panicked at src/testsuite.rs:302:50:
failed to load module: ParseError(File: "test-suite/test/core/f32_bitwise.0.wasm"
UnknownInstruction(<8b>), bin pos: 62, stack: [
    "wasp::parser::instr::Instr",
    "wasp::parser::expr::Expr",
    "wasp::parser::func::Func",
    "wasp::parser::code::Code",
    "alloc::vec::Vec<wasp::parser::code::Code>",
    "wasp::parser::codesec::CodeSection",
    "wasp::parser::module::Module",
])
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/f32_cmp.wast
```bash
Compiling "test-suite/test/core/f32_cmp.0.wasm"
thread 'main' panicked at src/testsuite.rs:302:50:
failed to load module: ParseError(File: "test-suite/test/core/f32_cmp.0.wasm"
UnknownInstruction(<5b>), bin pos: 69, stack: [
    "wasp::parser::instr::Instr",
    "wasp::parser::expr::Expr",
    "wasp::parser::func::Func",
    "wasp::parser::code::Code",
    "alloc::vec::Vec<wasp::parser::code::Code>",
    "wasp::parser::codesec::CodeSection",
    "wasp::parser::module::Module",
])
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/f64.wast
```bash
Compiling "test-suite/test/core/f64.0.wasm"
thread 'main' panicked at src/testsuite.rs:302:50:
failed to load module: ParseError(File: "test-suite/test/core/f64.0.wasm"
UnknownInstruction(<a3>), bin pos: 149, stack: [
    "wasp::parser::instr::Instr",
    "wasp::parser::expr::Expr",
    "wasp::parser::func::Func",
    "wasp::parser::code::Code",
    "alloc::vec::Vec<wasp::parser::code::Code>",
    "wasp::parser::codesec::CodeSection",
    "wasp::parser::module::Module",
])
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/f64_bitwise.wast
```bash
Compiling "test-suite/test/core/f64_bitwise.0.wasm"
thread 'main' panicked at src/testsuite.rs:302:50:
failed to load module: ParseError(File: "test-suite/test/core/f64_bitwise.0.wasm"
UnknownInstruction(<a6>), bin pos: 76, stack: [
    "wasp::parser::instr::Instr",
    "wasp::parser::expr::Expr",
    "wasp::parser::func::Func",
    "wasp::parser::code::Code",
    "alloc::vec::Vec<wasp::parser::code::Code>",
    "wasp::parser::codesec::CodeSection",
    "wasp::parser::module::Module",
])
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/f64_cmp.wast
```bash
Compiling "test-suite/test/core/f64_cmp.0.wasm"
thread 'main' panicked at src/testsuite.rs:302:50:
failed to load module: ParseError(File: "test-suite/test/core/f64_cmp.0.wasm"
UnknownInstruction(<64>), bin pos: 101, stack: [
    "wasp::parser::instr::Instr",
    "wasp::parser::expr::Expr",
    "wasp::parser::func::Func",
    "wasp::parser::code::Code",
    "alloc::vec::Vec<wasp::parser::code::Code>",
    "wasp::parser::codesec::CodeSection",
    "wasp::parser::module::Module",
])
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/fac.wast
```bash
thread 'main' panicked at src/testsuite.rs:273:6:
failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 10, column: 208)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/float_exprs.wast
```bash
Compiling "test-suite/test/core/float_exprs.0.wasm"
Checking "f64.no_contraction"  (FuncType { input: [Num(F64), Num(F64), Num(F64)], output: [Num(F64)] })
thread 'main' panicked at src/runtime/typecheck.rs:347:32:
not yet implemented
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/float_literals.wast
```bash
Compiling "test-suite/test/core/float_literals.0.wasm"
thread 'main' panicked at src/testsuite.rs:302:50:
failed to load module: ParseError(File: "test-suite/test/core/float_literals.0.wasm"
UnknownInstruction(<bc>), bin pos: 1960, stack: [
    "wasp::parser::instr::Instr",
    "wasp::parser::expr::Expr",
    "wasp::parser::func::Func",
    "wasp::parser::code::Code",
    "alloc::vec::Vec<wasp::parser::code::Code>",
    "wasp::parser::codesec::CodeSection",
    "wasp::parser::module::Module",
])
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/float_misc.wast
```bash
Compiling "test-suite/test/core/float_misc.0.wasm"
thread 'main' panicked at src/testsuite.rs:302:50:
failed to load module: ParseError(File: "test-suite/test/core/float_misc.0.wasm"
UnknownInstruction(<93>), bin pos: 397, stack: [
    "wasp::parser::instr::Instr",
    "wasp::parser::expr::Expr",
    "wasp::parser::func::Func",
    "wasp::parser::code::Code",
    "alloc::vec::Vec<wasp::parser::code::Code>",
    "wasp::parser::codesec::CodeSection",
    "wasp::parser::module::Module",
])
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/forward.wast
```bash
Compiling "test-suite/test/core/forward.0.wasm"
Checking "even"  (FuncType { input: [Num(I32)], output: [Num(I32)] })
Checking "odd"  (FuncType { input: [Num(I32)], output: [Num(I32)] })
 ERROR wasp::testsuite > test 1/5 failed (module: 0, invoke: "even", error: empty stack: src/runtime/mod.rs:563:55)
```

## Failed: test-suite/test/core/func.wast
```bash
Compiling "test-suite/test/core/func.0.wasm"
Checking (FuncType { input: [], output: [] })
Checking (FuncType { input: [], output: [] })
Checking "f"  (FuncType { input: [], output: [] })
Checking (FuncType { input: [], output: [] })
Checking "g"  (FuncType { input: [], output: [] })
Checking (FuncType { input: [], output: [] })
Checking (FuncType { input: [], output: [] })
Checking (FuncType { input: [], output: [] })
Checking (FuncType { input: [], output: [] })
Checking (FuncType { input: [], output: [] })
Checking (FuncType { input: [], output: [] })
Checking (FuncType { input: [], output: [] })
Checking (FuncType { input: [], output: [] })
Checking (FuncType { input: [], output: [] })
Checking (FuncType { input: [Num(I32)], output: [] })
Checking (FuncType { input: [Num(I32)], output: [] })
Checking (FuncType { input: [Num(I32), Num(F64), Num(I64)], output: [] })
Checking (FuncType { input: [Num(I32), Num(F64)], output: [] })
Checking (FuncType { input: [Num(I32), Num(F32), Num(I64), Num(I32), Num(F64)], output: [] })
Checking (FuncType { input: [], output: [] })
Checking (FuncType { input: [], output: [] })
Checking (FuncType { input: [], output: [Num(I32)] })
Checking (FuncType { input: [], output: [Num(I32), Num(F64), Num(F32)] })
Checking (FuncType { input: [], output: [Num(I32), Num(F64)] })
Checking (FuncType { input: [], output: [Num(I32), Num(F32), Num(I64), Num(I32), Num(F64)] })
Checking "type-use-1"  (FuncType { input: [], output: [] })
Checking "type-use-2"  (FuncType { input: [], output: [Num(I32)] })
Checking "type-use-3"  (FuncType { input: [Num(I32)], output: [] })
Checking "type-use-4"  (FuncType { input: [Num(I32), Num(F64), Num(I32)], output: [Num(I32)] })
Checking "type-use-5"  (FuncType { input: [], output: [Num(I32)] })
Checking "type-use-6"  (FuncType { input: [Num(I32)], output: [] })
Checking "type-use-7"  (FuncType { input: [Num(I32), Num(F64), Num(I32)], output: [Num(I32)] })
Checking (FuncType { input: [], output: [] })
Checking (FuncType { input: [], output: [] })
Checking (FuncType { input: [Num(I32), Num(F32), Num(I64), Num(I32)], output: [Num(I32), Num(I64), Num(I32)] })
Checking (FuncType { input: [], output: [] })
Checking "local-first-i32"  (FuncType { input: [], output: [Num(I32)] })
Checking "local-first-i64"  (FuncType { input: [], output: [Num(I64)] })
Checking "local-first-f32"  (FuncType { input: [], output: [Num(F32)] })
Checking "local-first-f64"  (FuncType { input: [], output: [Num(F64)] })
Checking "local-second-i32"  (FuncType { input: [], output: [Num(I32)] })
Checking "local-second-i64"  (FuncType { input: [], output: [Num(I64)] })
Checking "local-second-f32"  (FuncType { input: [], output: [Num(F32)] })
Checking "local-second-f64"  (FuncType { input: [], output: [Num(F64)] })
Checking "local-mixed"  (FuncType { input: [], output: [Num(F64)] })
thread 'main' panicked at src/runtime/typecheck.rs:325:32:
not yet implemented
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/func_ptrs.wast
```bash
Compiling "test-suite/test/core/func_ptrs.0.wasm"
Checking (FuncType { input: [], output: [] })
Checking (FuncType { input: [], output: [] })
Checking (FuncType { input: [], output: [Num(I32)] })
Checking "one"  (FuncType { input: [Num(I32)], output: [Num(I32)] })
Checking "two"  (FuncType { input: [Num(I32)], output: [Num(I32)] })
Checking "three"  (FuncType { input: [Num(I32)], output: [] })
thread 'main' panicked at src/testsuite.rs:302:50:
failed to load module: type error: ReturnTypeMismatch([], [Num(I32)])
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/global.wast
```bash
thread 'main' panicked at src/testsuite.rs:273:6:
failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 7, column: 2)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/i32.wast
```bash
Compiling "test-suite/test/core/i32.0.wasm"
thread 'main' panicked at src/testsuite.rs:302:50:
failed to load module: ParseError(File: "test-suite/test/core/i32.0.wasm"
UnknownInstruction(<6f>), bin pos: 332, stack: [
    "wasp::parser::instr::Instr",
    "wasp::parser::expr::Expr",
    "wasp::parser::func::Func",
    "wasp::parser::code::Code",
    "alloc::vec::Vec<wasp::parser::code::Code>",
    "wasp::parser::codesec::CodeSection",
    "wasp::parser::module::Module",
])
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/i64.wast
```bash
Compiling "test-suite/test/core/i64.0.wasm"
thread 'main' panicked at src/testsuite.rs:302:50:
failed to load module: ParseError(File: "test-suite/test/core/i64.0.wasm"
UnknownInstruction(<81>), bin pos: 357, stack: [
    "wasp::parser::instr::Instr",
    "wasp::parser::expr::Expr",
    "wasp::parser::func::Func",
    "wasp::parser::code::Code",
    "alloc::vec::Vec<wasp::parser::code::Code>",
    "wasp::parser::codesec::CodeSection",
    "wasp::parser::module::Module",
])
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/if.wast
```bash
Compiling "test-suite/test/core/if.0.wasm"
Checking (FuncType { input: [], output: [] })
Checking "empty"  (FuncType { input: [Num(I32)], output: [] })
Checking "singular"  (FuncType { input: [Num(I32)], output: [Num(I32)] })
Checking "multi"  (FuncType { input: [Num(I32)], output: [Num(I32), Num(I32)] })
Checking "nested"  (FuncType { input: [Num(I32), Num(I32)], output: [Num(I32)] })
Checking "as-select-first"  (FuncType { input: [Num(I32)], output: [Num(I32)] })
Checking "as-select-mid"  (FuncType { input: [Num(I32)], output: [Num(I32)] })
Checking "as-select-last"  (FuncType { input: [Num(I32)], output: [Num(I32)] })
Checking "as-loop-first"  (FuncType { input: [Num(I32)], output: [Num(I32)] })
Checking "as-loop-mid"  (FuncType { input: [Num(I32)], output: [Num(I32)] })
Checking "as-loop-last"  (FuncType { input: [Num(I32)], output: [Num(I32)] })
Checking "as-if-condition"  (FuncType { input: [Num(I32)], output: [Num(I32)] })
Checking "as-br_if-first"  (FuncType { input: [Num(I32)], output: [Num(I32)] })
thread 'main' panicked at src/testsuite.rs:302:50:
failed to load module: type error: ReturnTypeMismatch([Num(I32)], [Num(I32), Num(I32)])
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/imports.wast
```bash
thread 'main' panicked at src/testsuite.rs:273:6:
failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 5, column: 2)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/int_exprs.wast
```bash
Compiling "test-suite/test/core/int_exprs.0.wasm"
Checking "i32.no_fold_cmp_s_offset"  (FuncType { input: [Num(I32), Num(I32)], output: [Num(I32)] })
thread 'main' panicked at src/runtime/typecheck.rs:258:33:
not yet implemented
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/int_literals.wast
```bash
Compiling "test-suite/test/core/int_literals.0.wasm"
Checking "i32.test"  (FuncType { input: [], output: [Num(I32)] })
Checking "i32.umax"  (FuncType { input: [], output: [Num(I32)] })
Checking "i32.smax"  (FuncType { input: [], output: [Num(I32)] })
Checking "i32.neg_smax"  (FuncType { input: [], output: [Num(I32)] })
Checking "i32.smin"  (FuncType { input: [], output: [Num(I32)] })
Checking "i32.alt_smin"  (FuncType { input: [], output: [Num(I32)] })
Checking "i32.inc_smin"  (FuncType { input: [], output: [Num(I32)] })
Checking "i32.neg_zero"  (FuncType { input: [], output: [Num(I32)] })
Checking "i32.not_octal"  (FuncType { input: [], output: [Num(I32)] })
Checking "i32.unsigned_decimal"  (FuncType { input: [], output: [Num(I32)] })
Checking "i32.plus_sign"  (FuncType { input: [], output: [Num(I32)] })
Checking "i64.test"  (FuncType { input: [], output: [Num(I64)] })
Checking "i64.umax"  (FuncType { input: [], output: [Num(I64)] })
Checking "i64.smax"  (FuncType { input: [], output: [Num(I64)] })
Checking "i64.neg_smax"  (FuncType { input: [], output: [Num(I64)] })
Checking "i64.smin"  (FuncType { input: [], output: [Num(I64)] })
Checking "i64.alt_smin"  (FuncType { input: [], output: [Num(I64)] })
Checking "i64.inc_smin"  (FuncType { input: [], output: [Num(I64)] })
thread 'main' panicked at src/runtime/typecheck.rs:309:32:
not yet implemented
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/labels.wast
```bash
Compiling "test-suite/test/core/labels.0.wasm"
Checking "block"  (FuncType { input: [], output: [Num(I32)] })
Checking "loop1"  (FuncType { input: [], output: [Num(I32)] })
thread 'main' panicked at src/testsuite.rs:302:50:
failed to load module: type error: ReturnTypeMismatch([Num(I32)], [])
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/left-to-right.wast
```bash
Compiling "test-suite/test/core/left-to-right.0.wasm"
thread 'main' panicked at src/testsuite.rs:302:50:
failed to load module: ParseError(File: "test-suite/test/core/left-to-right.0.wasm"
UnknownInstruction(<6f>), bin pos: 1805, stack: [
    "wasp::parser::instr::Instr",
    "wasp::parser::expr::Expr",
    "wasp::parser::func::Func",
    "wasp::parser::code::Code",
    "alloc::vec::Vec<wasp::parser::code::Code>",
    "wasp::parser::codesec::CodeSection",
    "wasp::parser::module::Module",
])
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/linking.wast
```bash
thread 'main' panicked at src/testsuite.rs:273:6:
failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 5, column: 2)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/load.wast
```bash
Compiling "test-suite/test/core/load.0.wasm"
Checking "as-br-value"  (FuncType { input: [], output: [Num(I32)] })
Checking "as-br_if-cond"  (FuncType { input: [], output: [] })
Checking "as-br_if-value"  (FuncType { input: [], output: [Num(I32)] })
Checking "as-br_if-value-cond"  (FuncType { input: [], output: [Num(I32)] })
Checking "as-br_table-index"  (FuncType { input: [], output: [] })
Checking "as-br_table-value"  (FuncType { input: [], output: [Num(I32)] })
thread 'main' panicked at src/testsuite.rs:302:50:
failed to load module: type error: ReturnTypeMismatch([Num(I32)], [Num(I32), Num(I32)])
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/local_get.wast
```bash
Compiling "test-suite/test/core/local_get.0.wasm"
thread 'main' panicked at src/testsuite.rs:302:50:
failed to load module: ParseError(File: "test-suite/test/core/local_get.0.wasm"
UnknownInstruction(<ba>), bin pos: 526, stack: [
    "wasp::parser::instr::Instr",
    "wasp::parser::expr::Expr",
    "wasp::parser::func::Func",
    "wasp::parser::code::Code",
    "alloc::vec::Vec<wasp::parser::code::Code>",
    "wasp::parser::codesec::CodeSection",
    "wasp::parser::module::Module",
])
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/local_set.wast
```bash
Compiling "test-suite/test/core/local_set.0.wasm"
thread 'main' panicked at src/testsuite.rs:302:50:
failed to load module: ParseError(File: "test-suite/test/core/local_set.0.wasm"
UnknownInstruction(<ba>), bin pos: 581, stack: [
    "wasp::parser::instr::Instr",
    "wasp::parser::expr::Expr",
    "wasp::parser::func::Func",
    "wasp::parser::code::Code",
    "alloc::vec::Vec<wasp::parser::code::Code>",
    "wasp::parser::codesec::CodeSection",
    "wasp::parser::module::Module",
])
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/local_tee.wast
```bash
Compiling "test-suite/test/core/local_tee.0.wasm"
thread 'main' panicked at src/testsuite.rs:302:50:
failed to load module: ParseError(File: "test-suite/test/core/local_tee.0.wasm"
UnknownInstruction(<ba>), bin pos: 1390, stack: [
    "wasp::parser::instr::Instr",
    "wasp::parser::expr::Expr",
    "wasp::parser::func::Func",
    "wasp::parser::code::Code",
    "alloc::vec::Vec<wasp::parser::code::Code>",
    "wasp::parser::codesec::CodeSection",
    "wasp::parser::module::Module",
])
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/loop.wast
```bash
Compiling "test-suite/test/core/loop.0.wasm"
thread 'main' panicked at src/testsuite.rs:302:50:
failed to load module: ParseError(File: "test-suite/test/core/loop.0.wasm"
UnknownInstruction(<5b>), bin pos: 2296, stack: [
    "wasp::parser::instr::Instr",
    "wasp::parser::instr::Instr",
    "wasp::parser::instr::Instr",
    "wasp::parser::expr::Expr",
    "wasp::parser::func::Func",
    "wasp::parser::code::Code",
    "alloc::vec::Vec<wasp::parser::code::Code>",
    "wasp::parser::codesec::CodeSection",
    "wasp::parser::module::Module",
])
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/memory.wast
```bash
Compiling "test-suite/test/core/memory.0.wasm"
Compiling "test-suite/test/core/memory.1.wasm"
Compiling "test-suite/test/core/memory.2.wasm"
Compiling "test-suite/test/core/memory.3.wasm"
Compiling "test-suite/test/core/memory.4.wasm"
Compiling "test-suite/test/core/memory.5.wasm"
 ERROR wasp::testsuite > test 6/88 did not fail invalidating/parsing, expected error: "multiple memories" (module: "test-suite/test/core/memory.6.wasm")
```

## Failed: test-suite/test/core/memory_copy.wast
```bash
Compiling "test-suite/test/core/memory_copy.0.wasm"
Checking "test"  (FuncType { input: [], output: [] })
Checking "load8_u"  (FuncType { input: [Num(I32)], output: [Num(I32)] })
Compiling "test-suite/test/core/memory_copy.1.wasm"
Checking "test"  (FuncType { input: [], output: [] })
thread 'main' panicked at src/runtime/typecheck.rs:446:45:
not yet implemented
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/memory_fill.wast
```bash
Compiling "test-suite/test/core/memory_fill.0.wasm"
Checking "checkRange"  (FuncType { input: [Num(I32), Num(I32), Num(I32)], output: [Num(I32)] })
Checking "test"  (FuncType { input: [], output: [] })
thread 'main' panicked at src/runtime/typecheck.rs:447:42:
not yet implemented
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/memory_grow.wast
```bash
thread 'main' panicked at src/testsuite.rs:273:6:
failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 94, column: 2)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/memory_init.wast
```bash
Compiling "test-suite/test/core/memory_init.0.wasm"
Checking "test"  (FuncType { input: [], output: [] })
Checking "load8_u"  (FuncType { input: [Num(I32)], output: [Num(I32)] })
Compiling "test-suite/test/core/memory_init.1.wasm"
thread 'main' panicked at src/testsuite.rs:302:50:
failed to load module: ParseError(File: "test-suite/test/core/memory_init.1.wasm"
InvalidDataCount, bin pos: 63, stack: [
    "wasp::parser::module::Module",
])
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/memory_size.wast
```bash
Compiling "test-suite/test/core/memory_size.0.wasm"
thread 'main' panicked at src/testsuite.rs:302:50:
failed to load module: ParseError(File: "test-suite/test/core/memory_size.0.wasm"
UnknownInstruction(<3f>), bin pos: 52, stack: [
    "wasp::parser::instr::Instr",
    "wasp::parser::expr::Expr",
    "wasp::parser::func::Func",
    "wasp::parser::code::Code",
    "alloc::vec::Vec<wasp::parser::code::Code>",
    "wasp::parser::codesec::CodeSection",
    "wasp::parser::module::Module",
])
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/memory_trap.wast
```bash
Compiling "test-suite/test/core/memory_trap.0.wasm"
thread 'main' panicked at src/testsuite.rs:302:50:
failed to load module: ParseError(File: "test-suite/test/core/memory_trap.0.wasm"
UnknownInstruction(<3f>), bin pos: 75, stack: [
    "wasp::parser::instr::Instr",
    "wasp::parser::expr::Expr",
    "wasp::parser::func::Func",
    "wasp::parser::code::Code",
    "alloc::vec::Vec<wasp::parser::code::Code>",
    "wasp::parser::codesec::CodeSection",
    "wasp::parser::module::Module",
])
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/names.wast
```bash
Compiling "test-suite/test/core/names.0.wasm"
Checking "foo"  (FuncType { input: [], output: [Num(I32)] })
Compiling "test-suite/test/core/names.1.wasm"
Checking "foo"  (FuncType { input: [], output: [Num(I32)] })
Compiling "test-suite/test/core/names.2.wasm"
Checking ""  (FuncType { input: [], output: [Num(I32)] })
Checking "0"  (FuncType { input: [], output: [Num(I32)] })
Checking "-0"  (FuncType { input: [], output: [Num(I32)] })
Checking "_"  (FuncType { input: [], output: [Num(I32)] })
Checking "$"  (FuncType { input: [], output: [Num(I32)] })
Checking "@"  (FuncType { input: [], output: [Num(I32)] })
Checking "~!@#$%^&*()_+`-={}|[]\\:\";'<>?,./ "  (FuncType { input: [], output: [Num(I32)] })
Checking "NaN"  (FuncType { input: [], output: [Num(I32)] })
Checking "Infinity"  (FuncType { input: [], output: [Num(I32)] })
Checking "if"  (FuncType { input: [], output: [Num(I32)] })
Checking "malloc"  (FuncType { input: [], output: [Num(I32)] })
Checking "_malloc"  (FuncType { input: [], output: [Num(I32)] })
Checking "__malloc"  (FuncType { input: [], output: [Num(I32)] })
Checking "a"  (FuncType { input: [], output: [Num(I32)] })
Checking "A"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{feff}"  (FuncType { input: [], output: [Num(I32)] })
Checking "Å"  (FuncType { input: [], output: [Num(I32)] })
Checking "A\u{30a}"  (FuncType { input: [], output: [Num(I32)] })
Checking "Å"  (FuncType { input: [], output: [Num(I32)] })
Checking "ﬃ"  (FuncType { input: [], output: [Num(I32)] })
Checking "fﬁ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ffi"  (FuncType { input: [], output: [Num(I32)] })
Checking "\0\u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\t\n\u{b}\u{c}\r\u{e}\u{f}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{10}\u{11}\u{12}\u{13}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{1b}\u{1c}\u{1d}\u{1e}\u{1f}"  (FuncType { input: [], output: [Num(I32)] })
Checking " \u{7f}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{80}\u{81}\u{82}\u{83}\u{84}\u{85}\u{86}\u{87}\u{88}\u{89}\u{8a}\u{8b}\u{8c}\u{8d}\u{8e}\u{8f}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{90}\u{91}\u{92}\u{93}\u{94}\u{95}\u{96}\u{97}\u{98}\u{99}\u{9a}\u{9b}\u{9c}\u{9d}\u{9e}\u{9f}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{fff0}\u{fff1}\u{fff2}\u{fff3}\u{fff4}\u{fff5}\u{fff6}\u{fff7}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{fff8}\u{fff9}\u{fffa}\u{fffb}￼�\u{fffe}\u{ffff}"  (FuncType { input: [], output: [Num(I32)] })
Checking "␀␁␂␃␄␅␆␇␈␉␊␋␌␍␎␏"  (FuncType { input: [], output: [Num(I32)] })
Checking "␐␑␒␓␔␕␖␗␘␙␚␛␜␝␞␟"  (FuncType { input: [], output: [Num(I32)] })
Checking "␠␡"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{fff0}\u{fff1}\u{fff2}\u{fff3}\u{fff4}\u{fff5}\u{fff6}\u{fff7}\u{fff8}\u{fff9}\u{fffa}\u{fffb}￼�"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{200d}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{200c}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{34f}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{2060}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{2d7f}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{1107f}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{180e}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{ffef}\u{200b}\u{a0}\u{ad}\u{2060}\u{1680}\u{202e}\u{202d}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{200e}\u{200f}‑\u{2028}\u{2029}\u{202a}\u{202b}\u{202c}\u{202f}\u{2066}\u{2067}\u{2068}\u{2069}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{206a}\u{206b}\u{206c}\u{206d}\u{206e}\u{206f}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{2061}\u{2062}\u{2063}\u{2064}"  (FuncType { input: [], output: [Num(I32)] })
Checking "𐀀\u{dffff}\u{10ffff}"  (FuncType { input: [], output: [Num(I32)] })
Checking "Z\u{30f}\u{346}\u{306}\u{35b}\u{34c}\u{334}\u{358}\u{35e}\u{347}\u{32b}\u{325}\u{32a}\u{353}\u{348}\u{354}\u{34e}\u{317}\u{31e}\u{33a}\u{32f}\u{331}\u{31e}\u{319}\u{331}\u{31c}\u{316}\u{320}a\u{357}\u{368}\u{30e}\u{304}\u{306}\u{357}\u{33f}\u{361}\u{35f}\u{340}\u{336}\u{341}\u{325}\u{330}\u{333}\u{32d}\u{359}\u{332}\u{331}\u{339}\u{31d}\u{34e}\u{33c}l\u{344}\u{34a}\u{31a}\u{357}\u{366}\u{344}\u{36b}\u{307}\u{341}\u{336}\u{337}\u{349}\u{329}\u{339}\u{32b}\u{31d}\u{356}\u{345}\u{319}\u{332}\u{33c}\u{347}\u{35a}\u{34d}\u{32e}\u{34e}\u{325}\u{345}\u{31e}g\u{343}\u{310}\u{305}\u{36e}\u{314}\u{310}\u{30e}\u{302}\u{30f}\u{33e}\u{34a}\u{30d}\u{34b}\u{34a}\u{367}\u{301}\u{306}\u{366}\u{35e}\u{336}\u{355}\u{354}\u{35a}\u{329}o\u{34b}\u{314}\u{350}\u{36a}\u{369}\u{321}\u{34f}\u{322}\u{327}\u{341}\u{32b}\u{319}\u{324}\u{32e}\u{356}\u{359}\u{353}\u{33a}\u{31c}\u{329}\u{33c}\u{318}\u{320}"  (FuncType { input: [], output: [Num(I32)] })
Checking "ᅟᅠㅤﾠ"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{fe00}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{fe04}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{e0100}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{e01ef}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{308}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\n"  (FuncType { input: [], output: [Num(I32)] })
Checking "␤"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{2028}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\r"  (FuncType { input: [], output: [Num(I32)] })
Checking "\r\n"  (FuncType { input: [], output: [Num(I32)] })
Checking "\n\r"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{1e}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{b}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{c}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{85}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{2029}"  (FuncType { input: [], output: [Num(I32)] })
Checking "…"  (FuncType { input: [], output: [Num(I32)] })
Checking "⏎"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{8b}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{8c}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{8d}"  (FuncType { input: [], output: [Num(I32)] })
Checking "↵"  (FuncType { input: [], output: [Num(I32)] })
Checking "↩"  (FuncType { input: [], output: [Num(I32)] })
Checking "⌤"  (FuncType { input: [], output: [Num(I32)] })
Checking "⤶"  (FuncType { input: [], output: [Num(I32)] })
Checking "↲"  (FuncType { input: [], output: [Num(I32)] })
Checking "⮨"  (FuncType { input: [], output: [Num(I32)] })
Checking "⮰"  (FuncType { input: [], output: [Num(I32)] })
Checking "�"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{fdd0}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{fdd1}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{fdd2}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{fdd3}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{fdd4}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{fdd5}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{fdd6}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{fdd7}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{fdd8}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{fdd9}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{fdda}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{fddb}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{fddc}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{fddd}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{fdde}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{fddf}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{fde0}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{fde1}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{fde2}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{fde3}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{fde4}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{fde5}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{fde6}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{fde7}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{fde8}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{fde9}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{fdea}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{fdeb}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{fdec}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{fded}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{fdee}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{fdef}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{fffe}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{ffff}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{1fffe}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{1ffff}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{2fffe}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{2ffff}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{3fffe}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{3ffff}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{4fffe}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{4ffff}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{5fffe}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{5ffff}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{6fffe}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{6ffff}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{7fffe}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{7ffff}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{8fffe}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{8ffff}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{9fffe}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{9ffff}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{afffe}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{affff}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{bfffe}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{bffff}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{cfffe}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{cffff}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{dfffe}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{dffff}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{efffe}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{effff}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{ffffe}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{fffff}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{10fffe}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{10ffff}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{308}‽\u{308}\u{309}"  (FuncType { input: [], output: [Num(I32)] })
Checking "abc"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{202d}abc"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{202e}cba"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{202d}abc\u{202e}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{202e}cba\u{202d}"  (FuncType { input: [], output: [Num(I32)] })
Checking "𝑨"  (FuncType { input: [], output: [Num(I32)] })
Checking "𝐴"  (FuncType { input: [], output: [Num(I32)] })
Checking "𝘈"  (FuncType { input: [], output: [Num(I32)] })
Checking "𝘼"  (FuncType { input: [], output: [Num(I32)] })
Checking "𝐀"  (FuncType { input: [], output: [Num(I32)] })
Checking "𝓐"  (FuncType { input: [], output: [Num(I32)] })
Checking "𝕬"  (FuncType { input: [], output: [Num(I32)] })
Checking "𝗔"  (FuncType { input: [], output: [Num(I32)] })
Checking "𝒜"  (FuncType { input: [], output: [Num(I32)] })
Checking "𝔄"  (FuncType { input: [], output: [Num(I32)] })
Checking "𝔸"  (FuncType { input: [], output: [Num(I32)] })
Checking "𝖠"  (FuncType { input: [], output: [Num(I32)] })
Checking "𝙰"  (FuncType { input: [], output: [Num(I32)] })
Checking "ᴀ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ᴬ"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ⓐ"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ａ"  (FuncType { input: [], output: [Num(I32)] })
Checking "🄐"  (FuncType { input: [], output: [Num(I32)] })
Checking "🄰"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{e0041}"  (FuncType { input: [], output: [Num(I32)] })
Checking "U+0041"  (FuncType { input: [], output: [Num(I32)] })
Checking "A\u{200b}"  (FuncType { input: [], output: [Num(I32)] })
Checking "А"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ꙗ"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{2dfc}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{2df6}"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ɐ"  (FuncType { input: [], output: [Num(I32)] })
Checking "🅐"  (FuncType { input: [], output: [Num(I32)] })
Checking "🅰"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ⱝ"  (FuncType { input: [], output: [Num(I32)] })
Checking "𐐂"  (FuncType { input: [], output: [Num(I32)] })
Checking "𐐈"  (FuncType { input: [], output: [Num(I32)] })
Checking "𐒰"  (FuncType { input: [], output: [Num(I32)] })
Checking "À"  (FuncType { input: [], output: [Num(I32)] })
Checking "Á"  (FuncType { input: [], output: [Num(I32)] })
Checking "Â"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ã"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ä"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ā"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ă"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ą"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ǎ"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ǟ"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ǡ"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ǻ"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ȁ"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ȃ"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ȧ"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ⱥ"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ӑ"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ӓ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ߊ"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{821}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{822}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{823}"  (FuncType { input: [], output: [Num(I32)] })
Checking "ࠤ"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{825}"  (FuncType { input: [], output: [Num(I32)] })
Checking "ऄ"  (FuncType { input: [], output: [Num(I32)] })
Checking "अ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ॲ"  (FuncType { input: [], output: [Num(I32)] })
Checking "অ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ਅ"  (FuncType { input: [], output: [Num(I32)] })
Checking "અ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ଅ"  (FuncType { input: [], output: [Num(I32)] })
Checking "அ"  (FuncType { input: [], output: [Num(I32)] })
Checking "అ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ಅ"  (FuncType { input: [], output: [Num(I32)] })
Checking "അ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ะ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ະ"  (FuncType { input: [], output: [Num(I32)] })
Checking "༁"  (FuncType { input: [], output: [Num(I32)] })
Checking "ཨ"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{fb8}"  (FuncType { input: [], output: [Num(I32)] })
Checking "အ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ဢ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ႜ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ᅡ"  (FuncType { input: [], output: [Num(I32)] })
Checking "አ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ዐ"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ꭰ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ᐊ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ᖳ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ᚨ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ᚪ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ᛆ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ᜀ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ᜠ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ᝀ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ᝠ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ᠠ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ᢇ"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{1920}"  (FuncType { input: [], output: [Num(I32)] })
Checking "ᥣ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ᨕ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ᩋ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ᩡ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ᮃ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ᯀ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ᯁ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ᰣ"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ḁ"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ạ"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ả"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ấ"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ầ"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ẩ"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ẫ"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ậ"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ắ"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ằ"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ẳ"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ẵ"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ặ"  (FuncType { input: [], output: [Num(I32)] })
Checking "あ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ア"  (FuncType { input: [], output: [Num(I32)] })
Checking "ㄚ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ㅏ"  (FuncType { input: [], output: [Num(I32)] })
Checking "㈎"  (FuncType { input: [], output: [Num(I32)] })
Checking "㈏"  (FuncType { input: [], output: [Num(I32)] })
Checking "㈐"  (FuncType { input: [], output: [Num(I32)] })
Checking "㈑"  (FuncType { input: [], output: [Num(I32)] })
Checking "㈒"  (FuncType { input: [], output: [Num(I32)] })
Checking "㈓"  (FuncType { input: [], output: [Num(I32)] })
Checking "㈔"  (FuncType { input: [], output: [Num(I32)] })
Checking "㈕"  (FuncType { input: [], output: [Num(I32)] })
Checking "㈖"  (FuncType { input: [], output: [Num(I32)] })
Checking "㈗"  (FuncType { input: [], output: [Num(I32)] })
Checking "㈘"  (FuncType { input: [], output: [Num(I32)] })
Checking "㈙"  (FuncType { input: [], output: [Num(I32)] })
Checking "㈚"  (FuncType { input: [], output: [Num(I32)] })
Checking "㈛"  (FuncType { input: [], output: [Num(I32)] })
Checking "㉮"  (FuncType { input: [], output: [Num(I32)] })
Checking "㉯"  (FuncType { input: [], output: [Num(I32)] })
Checking "㉰"  (FuncType { input: [], output: [Num(I32)] })
Checking "㉱"  (FuncType { input: [], output: [Num(I32)] })
Checking "㉲"  (FuncType { input: [], output: [Num(I32)] })
Checking "㉳"  (FuncType { input: [], output: [Num(I32)] })
Checking "㉴"  (FuncType { input: [], output: [Num(I32)] })
Checking "㉵"  (FuncType { input: [], output: [Num(I32)] })
Checking "㉶"  (FuncType { input: [], output: [Num(I32)] })
Checking "㉷"  (FuncType { input: [], output: [Num(I32)] })
Checking "㉸"  (FuncType { input: [], output: [Num(I32)] })
Checking "㉹"  (FuncType { input: [], output: [Num(I32)] })
Checking "㉺"  (FuncType { input: [], output: [Num(I32)] })
Checking "㉻"  (FuncType { input: [], output: [Num(I32)] })
Checking "㋐"  (FuncType { input: [], output: [Num(I32)] })
Checking "ꀊ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ꓮ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ꕉ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ꚠ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ꠀ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ꠣ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ꡝ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ꢂ"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{a8ea}"  (FuncType { input: [], output: [Num(I32)] })
Checking "ꤢ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ꥆ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ꦄ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ꨀ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ｱ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ￂ"  (FuncType { input: [], output: [Num(I32)] })
Checking "𐀀"  (FuncType { input: [], output: [Num(I32)] })
Checking "𐊀"  (FuncType { input: [], output: [Num(I32)] })
Checking "𐊠"  (FuncType { input: [], output: [Num(I32)] })
Checking "𐌀"  (FuncType { input: [], output: [Num(I32)] })
Checking "𐎠"  (FuncType { input: [], output: [Num(I32)] })
Checking "𐒖"  (FuncType { input: [], output: [Num(I32)] })
Checking "𐔀"  (FuncType { input: [], output: [Num(I32)] })
Checking "𐝀"  (FuncType { input: [], output: [Num(I32)] })
Checking "𐠀"  (FuncType { input: [], output: [Num(I32)] })
Checking "𐤠"  (FuncType { input: [], output: [Num(I32)] })
Checking "𐦀"  (FuncType { input: [], output: [Num(I32)] })
Checking "𐦠"  (FuncType { input: [], output: [Num(I32)] })
Checking "𐨀"  (FuncType { input: [], output: [Num(I32)] })
Checking "𐬀"  (FuncType { input: [], output: [Num(I32)] })
Checking "𐰀"  (FuncType { input: [], output: [Num(I32)] })
Checking "𐰁"  (FuncType { input: [], output: [Num(I32)] })
Checking "𐲀"  (FuncType { input: [], output: [Num(I32)] })
Checking "𑀅"  (FuncType { input: [], output: [Num(I32)] })
Checking "𑂃"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{11127}"  (FuncType { input: [], output: [Num(I32)] })
Checking "𑅐"  (FuncType { input: [], output: [Num(I32)] })
Checking "𑆃"  (FuncType { input: [], output: [Num(I32)] })
Checking "𑈀"  (FuncType { input: [], output: [Num(I32)] })
Checking "𑊀"  (FuncType { input: [], output: [Num(I32)] })
Checking "𑊰"  (FuncType { input: [], output: [Num(I32)] })
Checking "𑌅"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{11370}"  (FuncType { input: [], output: [Num(I32)] })
Checking "𑐀"  (FuncType { input: [], output: [Num(I32)] })
Checking "𑒁"  (FuncType { input: [], output: [Num(I32)] })
Checking "𑖀"  (FuncType { input: [], output: [Num(I32)] })
Checking "𑘀"  (FuncType { input: [], output: [Num(I32)] })
Checking "𑚀"  (FuncType { input: [], output: [Num(I32)] })
Checking "𑜒"  (FuncType { input: [], output: [Num(I32)] })
Checking "𑜠"  (FuncType { input: [], output: [Num(I32)] })
Checking "𑢡"  (FuncType { input: [], output: [Num(I32)] })
Checking "𑫕"  (FuncType { input: [], output: [Num(I32)] })
Checking "𑰀"  (FuncType { input: [], output: [Num(I32)] })
Checking "𑲏"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{11caf}"  (FuncType { input: [], output: [Num(I32)] })
Checking "𒀀"  (FuncType { input: [], output: [Num(I32)] })
Checking "𖧕"  (FuncType { input: [], output: [Num(I32)] })
Checking "𖩆"  (FuncType { input: [], output: [Num(I32)] })
Checking "𖫧"  (FuncType { input: [], output: [Num(I32)] })
Checking "𖽔"  (FuncType { input: [], output: [Num(I32)] })
Checking "𛱁"  (FuncType { input: [], output: [Num(I32)] })
Checking "𛱤"  (FuncType { input: [], output: [Num(I32)] })
Checking "𞠣"  (FuncType { input: [], output: [Num(I32)] })
Checking "🇦"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ɑ"  (FuncType { input: [], output: [Num(I32)] })
Checking "Λ"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ɒ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ª"  (FuncType { input: [], output: [Num(I32)] })
Checking "∀"  (FuncType { input: [], output: [Num(I32)] })
Checking "₳"  (FuncType { input: [], output: [Num(I32)] })
Checking "𐤀"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ⲁ"  (FuncType { input: [], output: [Num(I32)] })
Checking "𐌰"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ά"  (FuncType { input: [], output: [Num(I32)] })
Checking "Α"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ἀ"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ἁ"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ἂ"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ἃ"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ἄ"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ἅ"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ἆ"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ἇ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ᾈ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ᾉ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ᾊ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ᾋ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ᾌ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ᾍ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ᾎ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ᾏ"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ᾰ"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ᾱ"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ὰ"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ά"  (FuncType { input: [], output: [Num(I32)] })
Checking "ᾼ"  (FuncType { input: [], output: [Num(I32)] })
Checking "𝚨"  (FuncType { input: [], output: [Num(I32)] })
Checking "𝛢"  (FuncType { input: [], output: [Num(I32)] })
Checking "𝜜"  (FuncType { input: [], output: [Num(I32)] })
Checking "𝝖"  (FuncType { input: [], output: [Num(I32)] })
Checking "𝞐"  (FuncType { input: [], output: [Num(I32)] })
Checking "⍶"  (FuncType { input: [], output: [Num(I32)] })
Checking "⍺"  (FuncType { input: [], output: [Num(I32)] })
Checking "⩜"  (FuncType { input: [], output: [Num(I32)] })
Checking "ᗅ"  (FuncType { input: [], output: [Num(I32)] })
Checking "Ꭺ"  (FuncType { input: [], output: [Num(I32)] })
Checking ")˺˼𔗏\u{1d174}\u{1d176}\u{1d178}\u{1d17a}⁾₎❩❫⟯﴿︶﹚）｠\u{e0029}❳❵⟧⟩⟫⟭⦈⦊⦖⸣⸥︘︸︺︼︾﹀﹂﹄﹈﹜﹞］｝｣\u{e005d}\u{e007d}»’”›❯"  (FuncType { input: [], output: [Num(I32)] })
Checking "(˹˻𔗎\u{1d173}\u{1d175}\u{1d177}\u{1d179}⁽₍❨❪⟮﴾︵﹙（｟\u{e0028}❲❴⟦⟨⟪⟬⦇⦉⦕⸢⸤︗︷︹︻︽︿﹁﹃﹇﹛﹝［｛｢\u{e005b}\u{e007b}«‘“‹❮"  (FuncType { input: [], output: [Num(I32)] })
Checking "𝪋\u{1daa4}"  (FuncType { input: [], output: [Num(I32)] })
Checking "𝪋"  (FuncType { input: [], output: [Num(I32)] })
Checking "½"  (FuncType { input: [], output: [Num(I32)] })
Checking "1⁄2"  (FuncType { input: [], output: [Num(I32)] })
Checking "1/2"  (FuncType { input: [], output: [Num(I32)] })
Checking "୳"  (FuncType { input: [], output: [Num(I32)] })
Checking "൴"  (FuncType { input: [], output: [Num(I32)] })
Checking "⳽"  (FuncType { input: [], output: [Num(I32)] })
Checking "꠱"  (FuncType { input: [], output: [Num(I32)] })
Checking "𐅁"  (FuncType { input: [], output: [Num(I32)] })
Checking "𐅵"  (FuncType { input: [], output: [Num(I32)] })
Checking "𐅶"  (FuncType { input: [], output: [Num(I32)] })
Checking "𐦽"  (FuncType { input: [], output: [Num(I32)] })
Checking "𐹻"  (FuncType { input: [], output: [Num(I32)] })
Checking "＂"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{7f}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{8}"  (FuncType { input: [], output: [Num(I32)] })
Checking "⌫"  (FuncType { input: [], output: [Num(I32)] })
Checking "⌦"  (FuncType { input: [], output: [Num(I32)] })
Checking "␈"  (FuncType { input: [], output: [Num(I32)] })
Checking "␡"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{1dfb}"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{f}"  (FuncType { input: [], output: [Num(I32)] })
Checking "←"  (FuncType { input: [], output: [Num(I32)] })
Checking "⌧"  (FuncType { input: [], output: [Num(I32)] })
Checking "⍒"  (FuncType { input: [], output: [Num(I32)] })
Checking "⍔"  (FuncType { input: [], output: [Num(I32)] })
Checking "⍢"  (FuncType { input: [], output: [Num(I32)] })
Checking "⍫"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{1a}"  (FuncType { input: [], output: [Num(I32)] })
Checking "␦"  (FuncType { input: [], output: [Num(I32)] })
Checking "␚"  (FuncType { input: [], output: [Num(I32)] })
Checking "￼"  (FuncType { input: [], output: [Num(I32)] })
Checking "?"  (FuncType { input: [], output: [Num(I32)] })
Checking "¿"  (FuncType { input: [], output: [Num(I32)] })
Checking "᥅"  (FuncType { input: [], output: [Num(I32)] })
Checking ";"  (FuncType { input: [], output: [Num(I32)] })
Checking "՞"  (FuncType { input: [], output: [Num(I32)] })
Checking "؟"  (FuncType { input: [], output: [Num(I32)] })
Checking "፧"  (FuncType { input: [], output: [Num(I32)] })
Checking "⁇"  (FuncType { input: [], output: [Num(I32)] })
Checking "⍰"  (FuncType { input: [], output: [Num(I32)] })
Checking "❓"  (FuncType { input: [], output: [Num(I32)] })
Checking "❔"  (FuncType { input: [], output: [Num(I32)] })
Checking "⳺"  (FuncType { input: [], output: [Num(I32)] })
Checking "⳻"  (FuncType { input: [], output: [Num(I32)] })
Checking "⸮"  (FuncType { input: [], output: [Num(I32)] })
Checking "㉄"  (FuncType { input: [], output: [Num(I32)] })
Checking "꘏"  (FuncType { input: [], output: [Num(I32)] })
Checking "꛷"  (FuncType { input: [], output: [Num(I32)] })
Checking "︖"  (FuncType { input: [], output: [Num(I32)] })
Checking "﹖"  (FuncType { input: [], output: [Num(I32)] })
Checking "？"  (FuncType { input: [], output: [Num(I32)] })
Checking "𑅃"  (FuncType { input: [], output: [Num(I32)] })
Checking "𞥟"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{e003f}"  (FuncType { input: [], output: [Num(I32)] })
Checking "𖡄"  (FuncType { input: [], output: [Num(I32)] })
Checking "⯑"  (FuncType { input: [], output: [Num(I32)] })
Checking "¶"  (FuncType { input: [], output: [Num(I32)] })
Checking "⁋"  (FuncType { input: [], output: [Num(I32)] })
Checking "܀"  (FuncType { input: [], output: [Num(I32)] })
Checking "჻"  (FuncType { input: [], output: [Num(I32)] })
Checking "፨"  (FuncType { input: [], output: [Num(I32)] })
Checking "〷"  (FuncType { input: [], output: [Num(I32)] })
Checking "❡"  (FuncType { input: [], output: [Num(I32)] })
Checking "⸏"  (FuncType { input: [], output: [Num(I32)] })
Checking "⸐"  (FuncType { input: [], output: [Num(I32)] })
Checking "⸑"  (FuncType { input: [], output: [Num(I32)] })
Checking "⸎"  (FuncType { input: [], output: [Num(I32)] })
Checking "\u{14}"  (FuncType { input: [], output: [Num(I32)] })
Checking "☙"  (FuncType { input: [], output: [Num(I32)] })
Checking "⸿"  (FuncType { input: [], output: [Num(I32)] })
Checking "〇"  (FuncType { input: [], output: [Num(I32)] })
Checking "๛"  (FuncType { input: [], output: [Num(I32)] })
Checking "ꙮ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ϓ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ϔ"  (FuncType { input: [], output: [Num(I32)] })
Checking "ẛ"  (FuncType { input: [], output: [Num(I32)] })
Compiling "test-suite/test/core/names.3.wasm"
Checking (FuncType { input: [Num(I32), Num(I32)], output: [] })
thread 'main' panicked at src/testsuite.rs:302:50:
failed to load module: type error: EmptyStack
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/nop.wast
```bash
Compiling "test-suite/test/core/nop.0.wasm"
Checking (FuncType { input: [], output: [] })
Checking (FuncType { input: [Num(I32), Num(I32), Num(I32)], output: [Num(I32)] })
Checking "as-func-first"  (FuncType { input: [], output: [Num(I32)] })
Checking "as-func-mid"  (FuncType { input: [], output: [Num(I32)] })
Checking "as-func-last"  (FuncType { input: [], output: [Num(I32)] })
Checking "as-func-everywhere"  (FuncType { input: [], output: [Num(I32)] })
Checking "as-drop-first"  (FuncType { input: [Num(I32)], output: [] })
Checking "as-drop-last"  (FuncType { input: [Num(I32)], output: [] })
Checking "as-drop-everywhere"  (FuncType { input: [Num(I32)], output: [] })
Checking "as-select-first"  (FuncType { input: [Num(I32)], output: [Num(I32)] })
Checking "as-select-mid1"  (FuncType { input: [Num(I32)], output: [Num(I32)] })
Checking "as-select-mid2"  (FuncType { input: [Num(I32)], output: [Num(I32)] })
Checking "as-select-last"  (FuncType { input: [Num(I32)], output: [Num(I32)] })
Checking "as-select-everywhere"  (FuncType { input: [Num(I32)], output: [Num(I32)] })
Checking "as-block-first"  (FuncType { input: [], output: [Num(I32)] })
Checking "as-block-mid"  (FuncType { input: [], output: [Num(I32)] })
Checking "as-block-last"  (FuncType { input: [], output: [Num(I32)] })
Checking "as-block-everywhere"  (FuncType { input: [], output: [Num(I32)] })
Checking "as-loop-first"  (FuncType { input: [], output: [Num(I32)] })
Checking "as-loop-mid"  (FuncType { input: [], output: [Num(I32)] })
Checking "as-loop-last"  (FuncType { input: [], output: [Num(I32)] })
Checking "as-loop-everywhere"  (FuncType { input: [], output: [Num(I32)] })
Checking "as-if-condition"  (FuncType { input: [Num(I32)], output: [] })
Checking "as-if-then"  (FuncType { input: [Num(I32)], output: [] })
Checking "as-if-else"  (FuncType { input: [Num(I32)], output: [] })
Checking "as-br-first"  (FuncType { input: [Num(I32)], output: [Num(I32)] })
Checking "as-br-last"  (FuncType { input: [Num(I32)], output: [Num(I32)] })
Checking "as-br-everywhere"  (FuncType { input: [Num(I32)], output: [Num(I32)] })
Checking "as-br_if-first"  (FuncType { input: [Num(I32)], output: [Num(I32)] })
Checking "as-br_if-mid"  (FuncType { input: [Num(I32)], output: [Num(I32)] })
Checking "as-br_if-last"  (FuncType { input: [Num(I32)], output: [Num(I32)] })
Checking "as-br_if-everywhere"  (FuncType { input: [Num(I32)], output: [Num(I32)] })
Checking "as-br_table-first"  (FuncType { input: [Num(I32)], output: [Num(I32)] })
Checking "as-br_table-mid"  (FuncType { input: [Num(I32)], output: [Num(I32)] })
Checking "as-br_table-last"  (FuncType { input: [Num(I32)], output: [Num(I32)] })
Checking "as-br_table-everywhere"  (FuncType { input: [Num(I32)], output: [Num(I32)] })
Checking "as-return-first"  (FuncType { input: [Num(I32)], output: [Num(I32)] })
Checking "as-return-last"  (FuncType { input: [Num(I32)], output: [Num(I32)] })
Checking "as-return-everywhere"  (FuncType { input: [Num(I32)], output: [Num(I32)] })
Checking "as-call-first"  (FuncType { input: [Num(I32), Num(I32), Num(I32)], output: [Num(I32)] })
Checking "as-call-mid1"  (FuncType { input: [Num(I32), Num(I32), Num(I32)], output: [Num(I32)] })
Checking "as-call-mid2"  (FuncType { input: [Num(I32), Num(I32), Num(I32)], output: [Num(I32)] })
Checking "as-call-last"  (FuncType { input: [Num(I32), Num(I32), Num(I32)], output: [Num(I32)] })
Checking "as-call-everywhere"  (FuncType { input: [Num(I32), Num(I32), Num(I32)], output: [Num(I32)] })
Checking "as-unary-first"  (FuncType { input: [Num(I32)], output: [Num(I32)] })
Checking "as-unary-last"  (FuncType { input: [Num(I32)], output: [Num(I32)] })
Checking "as-unary-everywhere"  (FuncType { input: [Num(I32)], output: [Num(I32)] })
Checking "as-binary-first"  (FuncType { input: [Num(I32)], output: [Num(I32)] })
Checking "as-binary-mid"  (FuncType { input: [Num(I32)], output: [Num(I32)] })
Checking "as-binary-last"  (FuncType { input: [Num(I32)], output: [Num(I32)] })
Checking "as-binary-everywhere"  (FuncType { input: [Num(I32)], output: [Num(I32)] })
Checking "as-test-first"  (FuncType { input: [Num(I32)], output: [Num(I32)] })
Checking "as-test-last"  (FuncType { input: [Num(I32)], output: [Num(I32)] })
Checking "as-test-everywhere"  (FuncType { input: [Num(I32)], output: [Num(I32)] })
Checking "as-compare-first"  (FuncType { input: [Num(I32)], output: [Num(I32)] })
thread 'main' panicked at src/runtime/typecheck.rs:257:31:
not yet implemented
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/ref_func.wast
```bash
thread 'main' panicked at src/testsuite.rs:273:6:
failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 5, column: 2)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/ref_is_null.wast
```bash
thread 'main' panicked at src/testsuite.rs:273:6:
failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 5, column: 2)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/ref_null.wast
```bash
thread 'main' panicked at src/testsuite.rs:273:6:
failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 5, column: 2)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/return.wast
```bash
Compiling "test-suite/test/core/return.0.wasm"
Checking (FuncType { input: [], output: [] })
Checking "type-i32"  (FuncType { input: [], output: [] })
thread 'main' panicked at src/testsuite.rs:302:50:
failed to load module: type error: EmptyStack
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/select.wast
```bash
thread 'main' panicked at src/testsuite.rs:273:6:
failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 33, column: 2)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/simd_address.wast
```bash
```

## Failed: test-suite/test/core/simd_align.wast
```bash
```

## Failed: test-suite/test/core/simd_bit_shift.wast
```bash
```

## Failed: test-suite/test/core/simd_bitwise.wast
```bash
```

## Failed: test-suite/test/core/simd_boolean.wast
```bash
```

## Failed: test-suite/test/core/simd_const.wast
```bash
```

## Failed: test-suite/test/core/simd_conversions.wast
```bash
```

## Failed: test-suite/test/core/simd_f32x4.wast
```bash
```

## Failed: test-suite/test/core/simd_f32x4_arith.wast
```bash
```

## Failed: test-suite/test/core/simd_f32x4_cmp.wast
```bash
```

## Failed: test-suite/test/core/simd_f32x4_pmin_pmax.wast
```bash
```

## Failed: test-suite/test/core/simd_f32x4_rounding.wast
```bash
```

## Failed: test-suite/test/core/simd_f64x2.wast
```bash
```

## Failed: test-suite/test/core/simd_f64x2_arith.wast
```bash
```

## Failed: test-suite/test/core/simd_f64x2_cmp.wast
```bash
```

## Failed: test-suite/test/core/simd_f64x2_pmin_pmax.wast
```bash
```

## Failed: test-suite/test/core/simd_f64x2_rounding.wast
```bash
```

## Failed: test-suite/test/core/simd_i16x8_arith.wast
```bash
```

## Failed: test-suite/test/core/simd_i16x8_arith2.wast
```bash
```

## Failed: test-suite/test/core/simd_i16x8_cmp.wast
```bash
```

## Failed: test-suite/test/core/simd_i16x8_extadd_pairwise_i8x16.wast
```bash
```

## Failed: test-suite/test/core/simd_i16x8_extmul_i8x16.wast
```bash
```

## Failed: test-suite/test/core/simd_i16x8_q15mulr_sat_s.wast
```bash
```

## Failed: test-suite/test/core/simd_i16x8_sat_arith.wast
```bash
```

## Failed: test-suite/test/core/simd_i32x4_arith.wast
```bash
```

## Failed: test-suite/test/core/simd_i32x4_arith2.wast
```bash
```

## Failed: test-suite/test/core/simd_i32x4_cmp.wast
```bash
```

## Failed: test-suite/test/core/simd_i32x4_dot_i16x8.wast
```bash
```

## Failed: test-suite/test/core/simd_i32x4_extadd_pairwise_i16x8.wast
```bash
```

## Failed: test-suite/test/core/simd_i32x4_extmul_i16x8.wast
```bash
```

## Failed: test-suite/test/core/simd_i32x4_trunc_sat_f32x4.wast
```bash
```

## Failed: test-suite/test/core/simd_i32x4_trunc_sat_f64x2.wast
```bash
```

## Failed: test-suite/test/core/simd_i64x2_arith.wast
```bash
```

## Failed: test-suite/test/core/simd_i64x2_arith2.wast
```bash
```

## Failed: test-suite/test/core/simd_i64x2_cmp.wast
```bash
```

## Failed: test-suite/test/core/simd_i64x2_extmul_i32x4.wast
```bash
```

## Failed: test-suite/test/core/simd_i8x16_arith.wast
```bash
```

## Failed: test-suite/test/core/simd_i8x16_arith2.wast
```bash
```

## Failed: test-suite/test/core/simd_i8x16_cmp.wast
```bash
```

## Failed: test-suite/test/core/simd_i8x16_sat_arith.wast
```bash
```

## Failed: test-suite/test/core/simd_int_to_int_extend.wast
```bash
```

## Failed: test-suite/test/core/simd_lane.wast
```bash
```

## Failed: test-suite/test/core/simd_linking.wast
```bash
```

## Failed: test-suite/test/core/simd_load.wast
```bash
```

## Failed: test-suite/test/core/simd_load16_lane.wast
```bash
```

## Failed: test-suite/test/core/simd_load32_lane.wast
```bash
```

## Failed: test-suite/test/core/simd_load64_lane.wast
```bash
```

## Failed: test-suite/test/core/simd_load8_lane.wast
```bash
```

## Failed: test-suite/test/core/simd_load_extend.wast
```bash
```

## Failed: test-suite/test/core/simd_load_splat.wast
```bash
```

## Failed: test-suite/test/core/simd_load_zero.wast
```bash
```

## Failed: test-suite/test/core/simd_splat.wast
```bash
```

## Failed: test-suite/test/core/simd_store.wast
```bash
```

## Failed: test-suite/test/core/simd_store16_lane.wast
```bash
```

## Failed: test-suite/test/core/simd_store32_lane.wast
```bash
```

## Failed: test-suite/test/core/simd_store64_lane.wast
```bash
```

## Failed: test-suite/test/core/simd_store8_lane.wast
```bash
```

## Failed: test-suite/test/core/skip-stack-guard-page.wast
```bash
thread 'main' panicked at src/testsuite.rs:273:6:
failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 5, column: 2)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/stack.wast
```bash
Compiling "test-suite/test/core/stack.0.wasm"
Checking "fac-expr"  (FuncType { input: [Num(I64)], output: [Num(I64)] })
thread 'main' panicked at src/runtime/typecheck.rs:267:31:
not yet implemented
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/start.wast
```bash
Checking (FuncType { input: [], output: [] })
 ERROR wasp::testsuite > test 0/20 did not fail invalidating/parsing, expected error: "unknown function" (module: "test-suite/test/core/start.0.wasm")
```

## Failed: test-suite/test/core/store.wast
```bash
Compiling "test-suite/test/core/store.0.wasm"
Checking "as-block-value"  (FuncType { input: [], output: [] })
Checking "as-loop-value"  (FuncType { input: [], output: [] })
Checking "as-br-value"  (FuncType { input: [], output: [] })
Checking "as-br_if-value"  (FuncType { input: [], output: [] })
Checking "as-br_if-value-cond"  (FuncType { input: [], output: [] })
Checking "as-br_table-value"  (FuncType { input: [], output: [] })
Checking "as-return-value"  (FuncType { input: [], output: [] })
Checking "as-if-then"  (FuncType { input: [], output: [] })
Checking "as-if-else"  (FuncType { input: [], output: [] })
Checking (FuncType { input: [Num(I32)], output: [Num(I32)] })
Checking (FuncType { input: [Num(I64)], output: [Num(I64)] })
Checking (FuncType { input: [Num(F32)], output: [Num(F32)] })
Checking (FuncType { input: [Num(F64)], output: [Num(F64)] })
Checking (FuncType { input: [Num(I32)], output: [Num(I32)] })
Checking (FuncType { input: [Num(I32)], output: [Num(I32)] })
Checking (FuncType { input: [Num(I64)], output: [Num(I64)] })
Checking (FuncType { input: [Num(I64)], output: [Num(I64)] })
Checking (FuncType { input: [Num(I64)], output: [Num(I64)] })
Checking (FuncType { input: [], output: [] })
Checking (FuncType { input: [], output: [] })
Checking (FuncType { input: [], output: [] })
Checking (FuncType { input: [], output: [] })
Checking (FuncType { input: [], output: [] })
Checking (FuncType { input: [], output: [] })
Checking (FuncType { input: [], output: [] })
Checking (FuncType { input: [], output: [] })
Checking (FuncType { input: [], output: [] })
Checking (FuncType { input: [], output: [] })
Checking (FuncType { input: [], output: [] })
Checking (FuncType { input: [], output: [] })
Checking (FuncType { input: [], output: [] })
Checking (FuncType { input: [], output: [] })
Checking (FuncType { input: [], output: [] })
Checking (FuncType { input: [], output: [] })
Checking (FuncType { input: [], output: [] })
Checking (FuncType { input: [], output: [] })
Checking (FuncType { input: [], output: [] })
Checking (FuncType { input: [], output: [] })
Checking (FuncType { input: [], output: [] })
Checking (FuncType { input: [], output: [] })
Checking (FuncType { input: [Num(I32)], output: [Num(I32)] })
Checking (FuncType { input: [], output: [] })
Checking (FuncType { input: [Num(I32)], output: [Num(I32)] })
Checking (FuncType { input: [], output: [] })
Checking (FuncType { input: [], output: [] })
 ERROR wasp::testsuite > test 50/68 did not fail invalidating/parsing, expected error: "type mismatch" (module: "test-suite/test/core/store.41.wasm")
```

## Failed: test-suite/test/core/switch.wast
```bash
Compiling "test-suite/test/core/switch.0.wasm"
Checking "stmt"  (FuncType { input: [Num(I32)], output: [Num(I32)] })
thread 'main' panicked at src/testsuite.rs:302:50:
failed to load module: type error: ReturnTypeMismatch([], [Num(I32)])
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/table-sub.wast
```bash
thread 'main' panicked at src/parser/instr.rs:645:24:
not yet implemented: 0xfc 14
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/table.wast
```bash
Compiling "test-suite/test/core/table.0.wasm"
Compiling "test-suite/test/core/table.1.wasm"
Compiling "test-suite/test/core/table.2.wasm"
Compiling "test-suite/test/core/table.3.wasm"
Compiling "test-suite/test/core/table.4.wasm"
Compiling "test-suite/test/core/table.5.wasm"
Compiling "test-suite/test/core/table.6.wasm"
Compiling "test-suite/test/core/table.7.wasm"
Compiling "test-suite/test/core/table.8.wasm"
thread 'main' panicked at src/parser/tabletype.rs:13:9:
not yet implemented
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/table_copy.wast
```bash
thread 'main' panicked at src/testsuite.rs:273:6:
failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 5, column: 2)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/table_fill.wast
```bash
thread 'main' panicked at src/testsuite.rs:273:6:
failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 5, column: 2)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/table_get.wast
```bash
thread 'main' panicked at src/testsuite.rs:273:6:
failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 5, column: 2)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/table_grow.wast
```bash
thread 'main' panicked at src/testsuite.rs:273:6:
failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 6, column: 2)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/table_init.wast
```bash
thread 'main' panicked at src/testsuite.rs:273:6:
failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 5, column: 2)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/table_set.wast
```bash
thread 'main' panicked at src/testsuite.rs:273:6:
failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 5, column: 2)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/table_size.wast
```bash
Compiling "test-suite/test/core/table_size.0.wasm"
thread 'main' panicked at src/parser/instr.rs:645:24:
not yet implemented: 0xfc 16
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/traps.wast
```bash
Compiling "test-suite/test/core/traps.0.wasm"
Checking "no_dce.i32.div_s"  (FuncType { input: [Num(I32), Num(I32)], output: [] })
thread 'main' panicked at src/runtime/typecheck.rs:295:34:
not yet implemented
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/unreachable.wast
```bash
Compiling "test-suite/test/core/unreachable.0.wasm"
Checking (FuncType { input: [], output: [] })
Checking (FuncType { input: [Num(I32), Num(I32), Num(I32)], output: [] })
Checking "type-i32"  (FuncType { input: [], output: [Num(I32)] })
Checking "type-i64"  (FuncType { input: [], output: [Num(I64)] })
Checking "type-f32"  (FuncType { input: [], output: [Num(F32)] })
Checking "type-f64"  (FuncType { input: [], output: [Num(F64)] })
Checking "as-func-first"  (FuncType { input: [], output: [Num(I32)] })
Checking "as-func-mid"  (FuncType { input: [], output: [Num(I32)] })
Checking "as-func-last"  (FuncType { input: [], output: [] })
Checking "as-func-value"  (FuncType { input: [], output: [Num(I32)] })
Checking "as-block-first"  (FuncType { input: [], output: [Num(I32)] })
Checking "as-block-mid"  (FuncType { input: [], output: [Num(I32)] })
Checking "as-block-last"  (FuncType { input: [], output: [] })
Checking "as-block-value"  (FuncType { input: [], output: [Num(I32)] })
Checking "as-block-broke"  (FuncType { input: [], output: [Num(I32)] })
Checking "as-loop-first"  (FuncType { input: [], output: [Num(I32)] })
Checking "as-loop-mid"  (FuncType { input: [], output: [Num(I32)] })
Checking "as-loop-last"  (FuncType { input: [], output: [] })
Checking "as-loop-broke"  (FuncType { input: [], output: [Num(I32)] })
Checking "as-br-value"  (FuncType { input: [], output: [Num(I32)] })
Checking "as-br_if-cond"  (FuncType { input: [], output: [] })
Checking "as-br_if-value"  (FuncType { input: [], output: [Num(I32)] })
Checking "as-br_if-value-cond"  (FuncType { input: [], output: [Num(I32)] })
Checking "as-br_table-index"  (FuncType { input: [], output: [] })
Checking "as-br_table-value"  (FuncType { input: [], output: [Num(I32)] })
Checking "as-br_table-value-2"  (FuncType { input: [], output: [Num(I32)] })
Checking "as-br_table-value-index"  (FuncType { input: [], output: [Num(I32)] })
Checking "as-br_table-value-and-index"  (FuncType { input: [], output: [Num(I32)] })
Checking "as-return-value"  (FuncType { input: [], output: [Num(I64)] })
Checking "as-if-cond"  (FuncType { input: [], output: [Num(I32)] })
Checking "as-if-then"  (FuncType { input: [Num(I32), Num(I32)], output: [Num(I32)] })
thread 'main' panicked at src/testsuite.rs:302:50:
failed to load module: type error: IfElseTypeMismatch([], [Num(I32)])
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/unreached-invalid.wast
```bash
Checking (FuncType { input: [], output: [] })
 ERROR wasp::testsuite > test 0/118 did not fail invalidating/parsing, expected error: "unknown local" (module: "test-suite/test/core/unreached-invalid.0.wasm")
```

## Failed: test-suite/test/core/unreached-valid.wast
```bash
Compiling "test-suite/test/core/unreached-valid.0.wasm"
thread 'main' panicked at src/testsuite.rs:302:50:
failed to load module: ParseError(File: "test-suite/test/core/unreached-valid.0.wasm"
UnknownInstruction(<d1>), bin pos: 273, stack: [
    "wasp::parser::instr::Instr",
    "wasp::parser::expr::Expr",
    "wasp::parser::func::Func",
    "wasp::parser::code::Code",
    "alloc::vec::Vec<wasp::parser::code::Code>",
    "wasp::parser::codesec::CodeSection",
    "wasp::parser::module::Module",
])
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/unwind.wast
```bash
Compiling "test-suite/test/core/unwind.0.wasm"
Checking "func-unwind-by-unreachable"  (FuncType { input: [], output: [] })
Checking "func-unwind-by-br"  (FuncType { input: [], output: [] })
thread 'main' panicked at src/testsuite.rs:302:50:
failed to load module: type error: ReturnTypeMismatch([], [Num(I32), Num(I64)])
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/utf8-custom-section-id.wast
```bash
 ERROR wasp::testsuite > test 0/176 did not fail invalidating/parsing, expected error: "malformed UTF-8 encoding" (module: "test-suite/test/core/utf8-custom-section-id.0.wasm")
```

## Failed: test-suite/test/core/utf8-import-field.wast
```bash
 ERROR wasp::testsuite > test 0/176 did not fail invalidating/parsing, expected error: "malformed UTF-8 encoding" (module: "test-suite/test/core/utf8-import-field.0.wasm")
```

## Failed: test-suite/test/core/utf8-import-module.wast
```bash
 ERROR wasp::testsuite > test 0/176 did not fail invalidating/parsing, expected error: "malformed UTF-8 encoding" (module: "test-suite/test/core/utf8-import-module.0.wasm")
```

