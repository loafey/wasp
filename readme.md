# W.A.S.P

## Latest spec test (typechecking currently disabled)
💅: 20\
💩: 127
## Failed: test-suite/test/core/bulk.wast
```bash

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = Some(
    (
        1,
        1,
    ),
)

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = Some(
    (
        2,
        2,
    ),
)

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = Some(
    (
        65,
        65,
    ),
)

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = Some(
    (
        1,
        1,
    ),
)

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None
 ERROR wasp::testsuite > test 77/117 failed: empty stack: src/runtime/methods/step.rs:257:41 (module: 8, invoke: "init")
```

## Failed: test-suite/test/core/call.wast
```bash
thread 'main' panicked at src/testsuite.rs:284:6:
failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 46, column: 2)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/call_indirect.wast
```bash
thread 'main' panicked at src/testsuite.rs:284:6:
failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 100, column: 2)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/conversions.wast
```bash

thread 'main' panicked at src/testsuite.rs:313:50:
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

thread 'main' panicked at src/testsuite.rs:313:50:
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

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None
thread 'main' panicked at src/testsuite.rs:313:50:
failed to load module: ActiveDataWithoutOffset
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/elem.wast
```bash
thread 'main' panicked at src/testsuite.rs:284:6:
failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 71, column: 2)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/endianness.wast
```bash

thread 'main' panicked at src/testsuite.rs:313:50:
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

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None
thread 'main' panicked at src/testsuite.rs:230:17:
not yet implemented
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/f32.wast
```bash

thread 'main' panicked at src/testsuite.rs:313:50:
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

thread 'main' panicked at src/testsuite.rs:313:50:
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

thread 'main' panicked at src/testsuite.rs:313:50:
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

thread 'main' panicked at src/testsuite.rs:313:50:
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

thread 'main' panicked at src/testsuite.rs:313:50:
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

thread 'main' panicked at src/testsuite.rs:313:50:
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
thread 'main' panicked at src/testsuite.rs:284:6:
failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 10, column: 208)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/float_exprs.wast
```bash

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None
thread 'main' panicked at src/runtime/methods/step.rs:873:25:
not implemented: instruction not supported : xa2_f64_mul
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/float_literals.wast
```bash

thread 'main' panicked at src/testsuite.rs:313:50:
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

thread 'main' panicked at src/testsuite.rs:313:50:
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

## Failed: test-suite/test/core/func.wast
```bash

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None
 ERROR wasp::testsuite > test 9/172 failed (module: 0, invoke: "local-first-i32", error: a local is missing: src/runtime/methods/step.rs:507:59)
```

## Failed: test-suite/test/core/func_ptrs.wast
```bash

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None
 ERROR wasp::testsuite > test 5/36 failed: unknown function: spectest::print_i32 (module: 0, invoke: "four")
```

## Failed: test-suite/test/core/global.wast
```bash

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None
thread 'main' panicked at src/testsuite.rs:313:50:
failed to load module: GlobalWithoutOffset
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/i32.wast
```bash

thread 'main' panicked at src/testsuite.rs:313:50:
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

thread 'main' panicked at src/testsuite.rs:313:50:
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

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None
thread 'main' panicked at src/runtime/typecheck.rs:379:41:
not yet implemented
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/imports.wast
```bash
thread 'main' panicked at src/testsuite.rs:284:6:
failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 5, column: 2)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/int_exprs.wast
```bash

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None
thread 'main' panicked at src/runtime/methods/step.rs:699:36:
attempt to add with overflow
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/int_literals.wast
```bash

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None
thread 'main' panicked at src/runtime/methods/step.rs:873:25:
not implemented: instruction not supported : x7c_i64_add
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/labels.wast
```bash

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None
thread 'main' panicked at src/runtime/typecheck.rs:321:31:
not yet implemented
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/left-to-right.wast
```bash

thread 'main' panicked at src/testsuite.rs:313:50:
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
thread 'main' panicked at src/testsuite.rs:284:6:
failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 5, column: 2)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/load.wast
```bash

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None
thread 'main' panicked at src/runtime/methods/step.rs:873:25:
not implemented: instruction not supported : x67_i32_clz
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/local_get.wast
```bash

thread 'main' panicked at src/testsuite.rs:313:50:
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

thread 'main' panicked at src/testsuite.rs:313:50:
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

thread 'main' panicked at src/testsuite.rs:313:50:
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

thread 'main' panicked at src/testsuite.rs:313:50:
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

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None
 ERROR wasp::testsuite > test 7/88 did not fail invalidating/parsing, expected error: "multiple memories" (module: "test-suite/test/core/memory.6.wasm")
```

## Failed: test-suite/test/core/memory_copy.wast
```bash

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None
thread 'main' panicked at src/runtime/memory.rs:136:25:
attempt to add with overflow
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/memory_fill.wast
```bash

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None
thread 'main' panicked at src/runtime/memory.rs:181:27:
attempt to add with overflow
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/memory_grow.wast
```bash
thread 'main' panicked at src/testsuite.rs:284:6:
failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 94, column: 2)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/memory_init.wast
```bash

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = Some(
    (
        4,
        4,
    ),
)

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = Some(
    (
        4,
        4,
    ),
)

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = Some(
    (
        4,
        4,
    ),
)

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = None

[src/parser/module.rs:120:13] found_data_count.zip(found_data_size) = Some(
    (
        1,
        1,
    ),
)
 ERROR wasp::testsuite > test 130/240 did not fail invalidating/parsing, expected error: "unknown data segment" (module: "test-suite/test/core/memory_init.5.wasm")
```

## Failed: test-suite/test/core/memory_size.wast
```bash

thread 'main' panicked at src/testsuite.rs:313:50:
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
