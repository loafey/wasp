# W.A.S.P

## Latest spec test 
💅: 13\
💩: 134
## Failed: test-suite/test/core/block.wast
```bash
@@@ Compiling "test-suite/test/core/block.0.wasm"
Checking
Checking "empty" 
Checking "singular" 
Checking "multi" 
Checking "nested" 
Checking "deep" 
Checking "as-select-first" 
Checking "as-select-mid" 
Checking "as-select-last" 
Checking "as-loop-first" 
Checking "as-loop-mid" 
Checking "as-loop-last" 
Checking "as-if-condition" 
Checking "as-if-then" 
Checking "as-if-else" 
Checking "as-br_if-first" 
Checking "as-br_if-last" 
Checking "as-br_table-first" 
Checking "as-br_table-last" 
Checking
Checking "as-call_indirect-first" 
Checking "as-call_indirect-mid" 
Checking "as-call_indirect-last" 
Checking "as-store-first" 
Checking "as-store-last" 
Checking "as-memory.grow-value" 
Checking
Checking "as-call-value" 
Checking "as-return-value" 
Checking "as-drop-operand" 
Checking "as-br-value" 
Checking "as-local.set-value" 
Checking "as-local.tee-value" 
Checking "as-global.set-value" 
Checking "as-load-operand" 
Checking "as-unary-operand" 
Checking "as-binary-operand" 
Checking "as-test-operand" 
Checking "as-compare-operand" 
Checking "as-binary-operands" 
Checking "as-compare-operands" 
Checking "as-mixed-operands" 
Checking "break-bare" 
Checking "break-value" 
Checking "break-multi-value" 
Checking "break-repeated" 
Checking "break-inner" 
Checking "param" 
Checking "params" 
Checking "params-id" 
Checking "param-break" 
Checking "params-break" 
Checking "params-id-break" 
Checking "effects" 
Checking "type-use" 
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
 ERROR wasp::testsuite > test 114/223 did not fail invalidating/parsing, expected error: "type mismatch" (module: "test-suite/test/core/block.62.wasm")
```

## Failed: test-suite/test/core/br.wast
```bash
@@@ Compiling "test-suite/test/core/br.0.wasm"
Checking
Checking "type-i32" 
Checking "type-i64" 
Checking "type-f32" 
Checking "type-f64" 
Checking "type-i32-i32" 
Checking "type-i64-i64" 
Checking "type-f32-f32" 
Checking "type-f64-f64" 
Checking "type-i32-value" 
Checking "type-i64-value" 
Checking "type-f32-value" 
Checking "type-f64-value" 
Checking "type-f64-f64-value" 
Checking "as-block-first" 
Checking "as-block-mid" 
Checking "as-block-last" 
Checking "as-block-value" 
Checking "as-loop-first" 
Checking "as-loop-mid" 
Checking "as-loop-last" 
Checking "as-br-value" 
Checking "as-br_if-cond" 
Checking "as-br_if-value" 
Checking "as-br_if-value-cond" 
Checking "as-br_table-index" 
Checking "as-br_table-value" 
Checking "as-br_table-value-index" 
Checking "as-return-value" 
Checking "as-return-values" 
Checking "as-if-cond" 
Checking "as-if-then" 
Checking "as-if-else" 
Checking "as-select-first" 
Checking "as-select-second" 
Checking "as-select-cond" 
Checking "as-select-all" 
Checking
Checking "as-call-first" 
Checking "as-call-mid" 
Checking "as-call-last" 
Checking "as-call-all" 
Checking "as-call_indirect-func" 
Checking "as-call_indirect-first" 
Checking "as-call_indirect-mid" 
Checking "as-call_indirect-last" 
Checking "as-call_indirect-all" 
Checking "as-local.set-value" 
Checking "as-local.tee-value" 
Checking "as-global.set-value" 
Checking "as-load-address" 
Checking "as-loadN-address" 
Checking "as-store-address" 
Checking "as-store-value" 
Checking "as-store-both" 
Checking "as-storeN-address" 
Checking "as-storeN-value" 
Checking "as-storeN-both" 
Checking "as-unary-operand" 
Checking "as-binary-left" 
Checking "as-binary-right" 
thread 'main' panicked at src/runtime/typecheck.rs:330:32:
not yet implemented
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/br_if.wast
```bash
@@@ Compiling "test-suite/test/core/br_if.0.wasm"
Checking
Checking "type-i32" 
Checking "type-i64" 
Checking "type-f32" 
Checking "type-f64" 
Checking "type-i32-value" 
Checking "type-i64-value" 
Checking "type-f32-value" 
Checking "type-f64-value" 
Checking "as-block-first" 
thread 'main' panicked at src/testsuite.rs:302:50:
failed to load module: type error: ReturnTypeMismatch([], [Num(I32)])
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
@@@ Compiling "test-suite/test/core/bulk.0.wasm"
@@@ Compiling "test-suite/test/core/bulk.1.wasm"
Checking
Checking
@@@ Compiling "test-suite/test/core/bulk.2.wasm"
Checking "fill" 
thread 'main' panicked at src/runtime/typecheck.rs:467:42:
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
@@@ Compiling "test-suite/test/core/comments.0.wasm"
@@@ Compiling "test-suite/test/core/comments.1.wasm"
@@@ Compiling "test-suite/test/core/comments.2.wasm"
@@@ Compiling "test-suite/test/core/comments.3.wasm"
@@@ Compiling "test-suite/test/core/comments.4.wasm"
Checking "f1" 
thread 'main' panicked at src/testsuite.rs:302:50:
failed to load module: type error: ReturnTypeMismatch([Num(I32)], [Num(I32), Num(I32)])
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/conversions.wast
```bash
@@@ Compiling "test-suite/test/core/conversions.0.wasm"
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
@@@ Compiling "test-suite/test/core/custom.0.wasm"
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
@@@ Compiling "test-suite/test/core/data.0.wasm"
@@@ Compiling "test-suite/test/core/data.1.wasm"
@@@ Compiling "test-suite/test/core/data.2.wasm"
@@@ Compiling "test-suite/test/core/data.3.wasm"
@@@ Compiling "test-suite/test/core/data.4.wasm"
@@@ Compiling "test-suite/test/core/data.5.wasm"
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
@@@ Compiling "test-suite/test/core/endianness.0.wasm"
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
@@@ Compiling "test-suite/test/core/exports.0.wasm"
Checking "a" 
@@@ Compiling "test-suite/test/core/exports.1.wasm"
Checking "b" 
@@@ Compiling "test-suite/test/core/exports.2.wasm"
Checking "a" 
Checking "b" 
@@@ Compiling "test-suite/test/core/exports.3.wasm"
Checking "a" 
@@@ Compiling "test-suite/test/core/exports.4.wasm"
Checking "c" 
@@@ Compiling "test-suite/test/core/exports.5.wasm"
Checking "b" 
@@@ Compiling "test-suite/test/core/exports.6.wasm"
Checking "a" 
@@@ Compiling "test-suite/test/core/exports.7.wasm"
Checking "a" 
@@@ Compiling "test-suite/test/core/exports.8.wasm"
Checking "a" 
@@@ Compiling "test-suite/test/core/exports.9.wasm"
Checking "a" 
@@@ Compiling "test-suite/test/core/exports.10.wasm"
Checking "a" 
@@@ Compiling "test-suite/test/core/exports.11.wasm"
Checking "e" 
thread 'main' panicked at src/testsuite.rs:219:17:
not yet implemented
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/f32.wast
```bash
@@@ Compiling "test-suite/test/core/f32.0.wasm"
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
@@@ Compiling "test-suite/test/core/f32_bitwise.0.wasm"
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
@@@ Compiling "test-suite/test/core/f32_cmp.0.wasm"
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
@@@ Compiling "test-suite/test/core/f64.0.wasm"
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
@@@ Compiling "test-suite/test/core/f64_bitwise.0.wasm"
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
@@@ Compiling "test-suite/test/core/f64_cmp.0.wasm"
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
@@@ Compiling "test-suite/test/core/float_exprs.0.wasm"
Checking "f64.no_contraction" 
thread 'main' panicked at src/runtime/typecheck.rs:367:32:
not yet implemented
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/float_literals.wast
```bash
@@@ Compiling "test-suite/test/core/float_literals.0.wasm"
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
@@@ Compiling "test-suite/test/core/float_misc.0.wasm"
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
@@@ Compiling "test-suite/test/core/forward.0.wasm"
Checking "even" 
Checking "odd" 
 ERROR wasp::testsuite > test 1/5 failed (module: 0, invoke: "even", error: empty stack: src/runtime/mod.rs:564:55)
```

## Failed: test-suite/test/core/func.wast
```bash
@@@ Compiling "test-suite/test/core/func.0.wasm"
Checking
Checking
Checking "f" 
Checking
Checking "g" 
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking
Checking "type-use-1" 
Checking "type-use-2" 
Checking "type-use-3" 
Checking "type-use-4" 
Checking "type-use-5" 
Checking "type-use-6" 
Checking "type-use-7" 
Checking
Checking
Checking
Checking
Checking "local-first-i32" 
Checking "local-first-i64" 
Checking "local-first-f32" 
Checking "local-first-f64" 
Checking "local-second-i32" 
Checking "local-second-i64" 
Checking "local-second-f32" 
Checking "local-second-f64" 
Checking "local-mixed" 
thread 'main' panicked at src/runtime/typecheck.rs:286:32:
not yet implemented
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/func_ptrs.wast
```bash
@@@ Compiling "test-suite/test/core/func_ptrs.0.wasm"
Checking
Checking
Checking
Checking "one" 
Checking "two" 
Checking "three" 
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
@@@ Compiling "test-suite/test/core/i32.0.wasm"
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
@@@ Compiling "test-suite/test/core/i64.0.wasm"
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
@@@ Compiling "test-suite/test/core/if.0.wasm"
Checking
Checking "empty" 
Checking "singular" 
Checking "multi" 
Checking "nested" 
Checking "as-select-first" 
Checking "as-select-mid" 
Checking "as-select-last" 
Checking "as-loop-first" 
Checking "as-loop-mid" 
Checking "as-loop-last" 
Checking "as-if-condition" 
Checking "as-br_if-first" 
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
@@@ Compiling "test-suite/test/core/int_exprs.0.wasm"
Checking "i32.no_fold_cmp_s_offset" 
thread 'main' panicked at src/runtime/typecheck.rs:278:33:
not yet implemented
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/int_literals.wast
```bash
@@@ Compiling "test-suite/test/core/int_literals.0.wasm"
Checking "i32.test" 
Checking "i32.umax" 
Checking "i32.smax" 
Checking "i32.neg_smax" 
Checking "i32.smin" 
Checking "i32.alt_smin" 
Checking "i32.inc_smin" 
Checking "i32.neg_zero" 
Checking "i32.not_octal" 
Checking "i32.unsigned_decimal" 
Checking "i32.plus_sign" 
Checking "i64.test" 
Checking "i64.umax" 
Checking "i64.smax" 
Checking "i64.neg_smax" 
Checking "i64.smin" 
Checking "i64.alt_smin" 
Checking "i64.inc_smin" 
Checking "i64.neg_zero" 
Checking "i64.not_octal" 
Checking "i64.unsigned_decimal" 
Checking "i64.plus_sign" 
Checking "i32-dec-sep1" 
Checking "i32-dec-sep2" 
Checking "i32-hex-sep1" 
Checking "i32-hex-sep2" 
Checking "i64-dec-sep1" 
Checking "i64-dec-sep2" 
Checking "i64-hex-sep1" 
Checking "i64-hex-sep2" 
thread 'main' panicked at src/runtime/mod.rs:996:25:
not implemented: instruction not supported : x7c_i64_add
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/labels.wast
```bash
@@@ Compiling "test-suite/test/core/labels.0.wasm"
Checking "block" 
Checking "loop1" 
Checking "loop2" 
Checking "loop3" 
Checking "loop4" 
thread 'main' panicked at src/runtime/typecheck.rs:281:33:
not yet implemented
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/left-to-right.wast
```bash
@@@ Compiling "test-suite/test/core/left-to-right.0.wasm"
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
@@@ Compiling "test-suite/test/core/load.0.wasm"
Checking "as-br-value" 
Checking "as-br_if-cond" 
Checking "as-br_if-value" 
Checking "as-br_if-value-cond" 
Checking "as-br_table-index" 
Checking "as-br_table-value" 
thread 'main' panicked at src/testsuite.rs:302:50:
failed to load module: type error: ReturnTypeMismatch([Num(I32)], [Num(I32), Num(I32)])
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/local_get.wast
```bash
@@@ Compiling "test-suite/test/core/local_get.0.wasm"
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
@@@ Compiling "test-suite/test/core/local_set.0.wasm"
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
@@@ Compiling "test-suite/test/core/local_tee.0.wasm"
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
@@@ Compiling "test-suite/test/core/loop.0.wasm"
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
@@@ Compiling "test-suite/test/core/memory.0.wasm"
@@@ Compiling "test-suite/test/core/memory.1.wasm"
@@@ Compiling "test-suite/test/core/memory.2.wasm"
@@@ Compiling "test-suite/test/core/memory.3.wasm"
@@@ Compiling "test-suite/test/core/memory.4.wasm"
@@@ Compiling "test-suite/test/core/memory.5.wasm"
 ERROR wasp::testsuite > test 6/88 did not fail invalidating/parsing, expected error: "multiple memories" (module: "test-suite/test/core/memory.6.wasm")
```

## Failed: test-suite/test/core/memory_copy.wast
```bash
@@@ Compiling "test-suite/test/core/memory_copy.0.wasm"
Checking "test" 
Checking "load8_u" 
@@@ Compiling "test-suite/test/core/memory_copy.1.wasm"
Checking "test" 
thread 'main' panicked at src/runtime/typecheck.rs:466:45:
not yet implemented
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/memory_fill.wast
```bash
@@@ Compiling "test-suite/test/core/memory_fill.0.wasm"
Checking "checkRange" 
Checking "test" 
thread 'main' panicked at src/runtime/typecheck.rs:467:42:
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
@@@ Compiling "test-suite/test/core/memory_init.0.wasm"
Checking "test" 
Checking "load8_u" 
@@@ Compiling "test-suite/test/core/memory_init.1.wasm"
thread 'main' panicked at src/testsuite.rs:302:50:
failed to load module: ParseError(File: "test-suite/test/core/memory_init.1.wasm"
InvalidDataCount, bin pos: 63, stack: [
    "wasp::parser::module::Module",
])
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/memory_size.wast
```bash
@@@ Compiling "test-suite/test/core/memory_size.0.wasm"
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
@@@ Compiling "test-suite/test/core/memory_trap.0.wasm"
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
@@@ Compiling "test-suite/test/core/names.0.wasm"
Checking "foo" 
@@@ Compiling "test-suite/test/core/names.1.wasm"
Checking "foo" 
@@@ Compiling "test-suite/test/core/names.2.wasm"
Checking "" 
Checking "0" 
Checking "-0" 
Checking "_" 
Checking "$" 
Checking "@" 
Checking "~!@#$%^&*()_+`-={}|[]\\:\";'<>?,./ " 
Checking "NaN" 
Checking "Infinity" 
Checking "if" 
Checking "malloc" 
Checking "_malloc" 
Checking "__malloc" 
Checking "a" 
Checking "A" 
Checking "\u{feff}" 
Checking "Å" 
Checking "A\u{30a}" 
Checking "Å" 
Checking "ﬃ" 
Checking "fﬁ" 
Checking "ffi" 
Checking "\0\u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\t\n\u{b}\u{c}\r\u{e}\u{f}" 
Checking "\u{10}\u{11}\u{12}\u{13}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{1b}\u{1c}\u{1d}\u{1e}\u{1f}" 
Checking " \u{7f}" 
Checking "\u{80}\u{81}\u{82}\u{83}\u{84}\u{85}\u{86}\u{87}\u{88}\u{89}\u{8a}\u{8b}\u{8c}\u{8d}\u{8e}\u{8f}" 
Checking "\u{90}\u{91}\u{92}\u{93}\u{94}\u{95}\u{96}\u{97}\u{98}\u{99}\u{9a}\u{9b}\u{9c}\u{9d}\u{9e}\u{9f}" 
Checking "\u{fff0}\u{fff1}\u{fff2}\u{fff3}\u{fff4}\u{fff5}\u{fff6}\u{fff7}" 
Checking "\u{fff8}\u{fff9}\u{fffa}\u{fffb}￼�\u{fffe}\u{ffff}" 
Checking "␀␁␂␃␄␅␆␇␈␉␊␋␌␍␎␏" 
Checking "␐␑␒␓␔␕␖␗␘␙␚␛␜␝␞␟" 
Checking "␠␡" 
Checking "\u{fff0}\u{fff1}\u{fff2}\u{fff3}\u{fff4}\u{fff5}\u{fff6}\u{fff7}\u{fff8}\u{fff9}\u{fffa}\u{fffb}￼�" 
Checking "\u{200d}" 
Checking "\u{200c}" 
Checking "\u{34f}" 
Checking "\u{2060}" 
Checking "\u{2d7f}" 
Checking "\u{1107f}" 
Checking "\u{180e}" 
Checking "\u{ffef}\u{200b}\u{a0}\u{ad}\u{2060}\u{1680}\u{202e}\u{202d}" 
Checking "\u{200e}\u{200f}‑\u{2028}\u{2029}\u{202a}\u{202b}\u{202c}\u{202f}\u{2066}\u{2067}\u{2068}\u{2069}" 
Checking "\u{206a}\u{206b}\u{206c}\u{206d}\u{206e}\u{206f}" 
Checking "\u{2061}\u{2062}\u{2063}\u{2064}" 
Checking "𐀀\u{dffff}\u{10ffff}" 
Checking "Z\u{30f}\u{346}\u{306}\u{35b}\u{34c}\u{334}\u{358}\u{35e}\u{347}\u{32b}\u{325}\u{32a}\u{353}\u{348}\u{354}\u{34e}\u{317}\u{31e}\u{33a}\u{32f}\u{331}\u{31e}\u{319}\u{331}\u{31c}\u{316}\u{320}a\u{357}\u{368}\u{30e}\u{304}\u{306}\u{357}\u{33f}\u{361}\u{35f}\u{340}\u{336}\u{341}\u{325}\u{330}\u{333}\u{32d}\u{359}\u{332}\u{331}\u{339}\u{31d}\u{34e}\u{33c}l\u{344}\u{34a}\u{31a}\u{357}\u{366}\u{344}\u{36b}\u{307}\u{341}\u{336}\u{337}\u{349}\u{329}\u{339}\u{32b}\u{31d}\u{356}\u{345}\u{319}\u{332}\u{33c}\u{347}\u{35a}\u{34d}\u{32e}\u{34e}\u{325}\u{345}\u{31e}g\u{343}\u{310}\u{305}\u{36e}\u{314}\u{310}\u{30e}\u{302}\u{30f}\u{33e}\u{34a}\u{30d}\u{34b}\u{34a}\u{367}\u{301}\u{306}\u{366}\u{35e}\u{336}\u{355}\u{354}\u{35a}\u{329}o\u{34b}\u{314}\u{350}\u{36a}\u{369}\u{321}\u{34f}\u{322}\u{327}\u{341}\u{32b}\u{319}\u{324}\u{32e}\u{356}\u{359}\u{353}\u{33a}\u{31c}\u{329}\u{33c}\u{318}\u{320}" 
Checking "ᅟᅠㅤﾠ" 
Checking "\u{fe00}" 
Checking "\u{fe04}" 
Checking "\u{e0100}" 
Checking "\u{e01ef}" 
Checking "\u{308}" 
Checking "\n" 
Checking "␤" 
Checking "\u{2028}" 
Checking "\r" 
Checking "\r\n" 
Checking "\n\r" 
Checking "\u{1e}" 
Checking "\u{b}" 
Checking "\u{c}" 
Checking "\u{85}" 
Checking "\u{2029}" 
Checking "…" 
Checking "⏎" 
Checking "\u{8b}" 
Checking "\u{8c}" 
Checking "\u{8d}" 
Checking "↵" 
Checking "↩" 
Checking "⌤" 
Checking "⤶" 
Checking "↲" 
Checking "⮨" 
Checking "⮰" 
Checking "�" 
Checking "\u{fdd0}" 
Checking "\u{fdd1}" 
Checking "\u{fdd2}" 
Checking "\u{fdd3}" 
Checking "\u{fdd4}" 
Checking "\u{fdd5}" 
Checking "\u{fdd6}" 
Checking "\u{fdd7}" 
Checking "\u{fdd8}" 
Checking "\u{fdd9}" 
Checking "\u{fdda}" 
Checking "\u{fddb}" 
Checking "\u{fddc}" 
Checking "\u{fddd}" 
Checking "\u{fdde}" 
Checking "\u{fddf}" 
Checking "\u{fde0}" 
Checking "\u{fde1}" 
Checking "\u{fde2}" 
Checking "\u{fde3}" 
Checking "\u{fde4}" 
Checking "\u{fde5}" 
Checking "\u{fde6}" 
Checking "\u{fde7}" 
Checking "\u{fde8}" 
Checking "\u{fde9}" 
Checking "\u{fdea}" 
Checking "\u{fdeb}" 
Checking "\u{fdec}" 
Checking "\u{fded}" 
Checking "\u{fdee}" 
Checking "\u{fdef}" 
Checking "\u{fffe}" 
Checking "\u{ffff}" 
Checking "\u{1fffe}" 
Checking "\u{1ffff}" 
Checking "\u{2fffe}" 
Checking "\u{2ffff}" 
Checking "\u{3fffe}" 
Checking "\u{3ffff}" 
Checking "\u{4fffe}" 
Checking "\u{4ffff}" 
Checking "\u{5fffe}" 
Checking "\u{5ffff}" 
Checking "\u{6fffe}" 
Checking "\u{6ffff}" 
Checking "\u{7fffe}" 
Checking "\u{7ffff}" 
Checking "\u{8fffe}" 
Checking "\u{8ffff}" 
Checking "\u{9fffe}" 
Checking "\u{9ffff}" 
Checking "\u{afffe}" 
Checking "\u{affff}" 
Checking "\u{bfffe}" 
Checking "\u{bffff}" 
Checking "\u{cfffe}" 
Checking "\u{cffff}" 
Checking "\u{dfffe}" 
Checking "\u{dffff}" 
Checking "\u{efffe}" 
Checking "\u{effff}" 
Checking "\u{ffffe}" 
Checking "\u{fffff}" 
Checking "\u{10fffe}" 
Checking "\u{10ffff}" 
Checking "\u{308}‽\u{308}\u{309}" 
Checking "abc" 
Checking "\u{202d}abc" 
Checking "\u{202e}cba" 
Checking "\u{202d}abc\u{202e}" 
Checking "\u{202e}cba\u{202d}" 
Checking "𝑨" 
Checking "𝐴" 
Checking "𝘈" 
Checking "𝘼" 
Checking "𝐀" 
Checking "𝓐" 
Checking "𝕬" 
Checking "𝗔" 
Checking "𝒜" 
Checking "𝔄" 
Checking "𝔸" 
Checking "𝖠" 
Checking "𝙰" 
Checking "ᴀ" 
Checking "ᴬ" 
Checking "Ⓐ" 
Checking "Ａ" 
Checking "🄐" 
Checking "🄰" 
Checking "\u{e0041}" 
Checking "U+0041" 
Checking "A\u{200b}" 
Checking "А" 
Checking "Ꙗ" 
Checking "\u{2dfc}" 
Checking "\u{2df6}" 
Checking "Ɐ" 
Checking "🅐" 
Checking "🅰" 
Checking "Ⱝ" 
Checking "𐐂" 
Checking "𐐈" 
Checking "𐒰" 
Checking "À" 
Checking "Á" 
Checking "Â" 
Checking "Ã" 
Checking "Ä" 
Checking "Ā" 
Checking "Ă" 
Checking "Ą" 
Checking "Ǎ" 
Checking "Ǟ" 
Checking "Ǡ" 
Checking "Ǻ" 
Checking "Ȁ" 
Checking "Ȃ" 
Checking "Ȧ" 
Checking "Ⱥ" 
Checking "Ӑ" 
Checking "Ӓ" 
Checking "ߊ" 
Checking "\u{821}" 
Checking "\u{822}" 
Checking "\u{823}" 
Checking "ࠤ" 
Checking "\u{825}" 
Checking "ऄ" 
Checking "अ" 
Checking "ॲ" 
Checking "অ" 
Checking "ਅ" 
Checking "અ" 
Checking "ଅ" 
Checking "அ" 
Checking "అ" 
Checking "ಅ" 
Checking "അ" 
Checking "ะ" 
Checking "ະ" 
Checking "༁" 
Checking "ཨ" 
Checking "\u{fb8}" 
Checking "အ" 
Checking "ဢ" 
Checking "ႜ" 
Checking "ᅡ" 
Checking "አ" 
Checking "ዐ" 
Checking "Ꭰ" 
Checking "ᐊ" 
Checking "ᖳ" 
Checking "ᚨ" 
Checking "ᚪ" 
Checking "ᛆ" 
Checking "ᜀ" 
Checking "ᜠ" 
Checking "ᝀ" 
Checking "ᝠ" 
Checking "ᠠ" 
Checking "ᢇ" 
Checking "\u{1920}" 
Checking "ᥣ" 
Checking "ᨕ" 
Checking "ᩋ" 
Checking "ᩡ" 
Checking "ᮃ" 
Checking "ᯀ" 
Checking "ᯁ" 
Checking "ᰣ" 
Checking "Ḁ" 
Checking "Ạ" 
Checking "Ả" 
Checking "Ấ" 
Checking "Ầ" 
Checking "Ẩ" 
Checking "Ẫ" 
Checking "Ậ" 
Checking "Ắ" 
Checking "Ằ" 
Checking "Ẳ" 
Checking "Ẵ" 
Checking "Ặ" 
Checking "あ" 
Checking "ア" 
Checking "ㄚ" 
Checking "ㅏ" 
Checking "㈎" 
Checking "㈏" 
Checking "㈐" 
Checking "㈑" 
Checking "㈒" 
Checking "㈓" 
Checking "㈔" 
Checking "㈕" 
Checking "㈖" 
Checking "㈗" 
Checking "㈘" 
Checking "㈙" 
Checking "㈚" 
Checking "㈛" 
Checking "㉮" 
Checking "㉯" 
Checking "㉰" 
Checking "㉱" 
Checking "㉲" 
Checking "㉳" 
Checking "㉴" 
Checking "㉵" 
Checking "㉶" 
Checking "㉷" 
Checking "㉸" 
Checking "㉹" 
Checking "㉺" 
Checking "㉻" 
Checking "㋐" 
Checking "ꀊ" 
Checking "ꓮ" 
Checking "ꕉ" 
Checking "ꚠ" 
Checking "ꠀ" 
Checking "ꠣ" 
Checking "ꡝ" 
Checking "ꢂ" 
Checking "\u{a8ea}" 
Checking "ꤢ" 
Checking "ꥆ" 
Checking "ꦄ" 
Checking "ꨀ" 
Checking "ｱ" 
Checking "ￂ" 
Checking "𐀀" 
Checking "𐊀" 
Checking "𐊠" 
Checking "𐌀" 
Checking "𐎠" 
Checking "𐒖" 
Checking "𐔀" 
Checking "𐝀" 
Checking "𐠀" 
Checking "𐤠" 
Checking "𐦀" 
Checking "𐦠" 
Checking "𐨀" 
Checking "𐬀" 
Checking "𐰀" 
Checking "𐰁" 
Checking "𐲀" 
Checking "𑀅" 
Checking "𑂃" 
Checking "\u{11127}" 
Checking "𑅐" 
Checking "𑆃" 
Checking "𑈀" 
Checking "𑊀" 
Checking "𑊰" 
Checking "𑌅" 
Checking "\u{11370}" 
Checking "𑐀" 
Checking "𑒁" 
Checking "𑖀" 
Checking "𑘀" 
Checking "𑚀" 
Checking "𑜒" 
Checking "𑜠" 
Checking "𑢡" 
Checking "𑫕" 
Checking "𑰀" 
Checking "𑲏" 
Checking "\u{11caf}" 
Checking "𒀀" 
Checking "𖧕" 
Checking "𖩆" 
Checking "𖫧" 
Checking "𖽔" 
Checking "𛱁" 
Checking "𛱤" 
Checking "𞠣" 
Checking "🇦" 
Checking "Ɑ" 
Checking "Λ" 
Checking "Ɒ" 
Checking "ª" 
Checking "∀" 
Checking "₳" 
Checking "𐤀" 
Checking "Ⲁ" 
Checking "𐌰" 
Checking "Ά" 
Checking "Α" 
Checking "Ἀ" 
Checking "Ἁ" 
Checking "Ἂ" 
Checking "Ἃ" 
Checking "Ἄ" 
Checking "Ἅ" 
Checking "Ἆ" 
Checking "Ἇ" 
Checking "ᾈ" 
Checking "ᾉ" 
Checking "ᾊ" 
Checking "ᾋ" 
Checking "ᾌ" 
Checking "ᾍ" 
Checking "ᾎ" 
Checking "ᾏ" 
Checking "Ᾰ" 
Checking "Ᾱ" 
Checking "Ὰ" 
Checking "Ά" 
Checking "ᾼ" 
Checking "𝚨" 
Checking "𝛢" 
Checking "𝜜" 
Checking "𝝖" 
Checking "𝞐" 
Checking "⍶" 
Checking "⍺" 
Checking "⩜" 
Checking "ᗅ" 
Checking "Ꭺ" 
Checking ")˺˼𔗏\u{1d174}\u{1d176}\u{1d178}\u{1d17a}⁾₎❩❫⟯﴿︶﹚）｠\u{e0029}❳❵⟧⟩⟫⟭⦈⦊⦖⸣⸥︘︸︺︼︾﹀﹂﹄﹈﹜﹞］｝｣\u{e005d}\u{e007d}»’”›❯" 
Checking "(˹˻𔗎\u{1d173}\u{1d175}\u{1d177}\u{1d179}⁽₍❨❪⟮﴾︵﹙（｟\u{e0028}❲❴⟦⟨⟪⟬⦇⦉⦕⸢⸤︗︷︹︻︽︿﹁﹃﹇﹛﹝［｛｢\u{e005b}\u{e007b}«‘“‹❮" 
Checking "𝪋\u{1daa4}" 
Checking "𝪋" 
Checking "½" 
Checking "1⁄2" 
Checking "1/2" 
Checking "୳" 
Checking "൴" 
Checking "⳽" 
Checking "꠱" 
Checking "𐅁" 
Checking "𐅵" 
Checking "𐅶" 
Checking "𐦽" 
Checking "𐹻" 
Checking "＂" 
Checking "\u{7f}" 
Checking "\u{8}" 
Checking "⌫" 
Checking "⌦" 
Checking "␈" 
Checking "␡" 
Checking "\u{1dfb}" 
Checking "\u{f}" 
Checking "←" 
Checking "⌧" 
Checking "⍒" 
Checking "⍔" 
Checking "⍢" 
Checking "⍫" 
Checking "\u{1a}" 
Checking "␦" 
Checking "␚" 
Checking "￼" 
Checking "?" 
Checking "¿" 
Checking "᥅" 
Checking ";" 
Checking "՞" 
Checking "؟" 
Checking "፧" 
Checking "⁇" 
Checking "⍰" 
Checking "❓" 
Checking "❔" 
Checking "⳺" 
Checking "⳻" 
Checking "⸮" 
Checking "㉄" 
Checking "꘏" 
Checking "꛷" 
Checking "︖" 
Checking "﹖" 
Checking "？" 
Checking "𑅃" 
Checking "𞥟" 
Checking "\u{e003f}" 
Checking "𖡄" 
Checking "⯑" 
Checking "¶" 
Checking "⁋" 
Checking "܀" 
Checking "჻" 
Checking "፨" 
Checking "〷" 
Checking "❡" 
Checking "⸏" 
Checking "⸐" 
Checking "⸑" 
Checking "⸎" 
Checking "\u{14}" 
Checking "☙" 
Checking "⸿" 
Checking "〇" 
Checking "๛" 
Checking "ꙮ" 
Checking "ϓ" 
Checking "ϔ" 
Checking "ẛ" 
@@@ Compiling "test-suite/test/core/names.3.wasm"
Checking
thread 'main' panicked at src/testsuite.rs:302:50:
failed to load module: type error: EmptyStack
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/nop.wast
```bash
@@@ Compiling "test-suite/test/core/nop.0.wasm"
Checking
Checking
Checking "as-func-first" 
Checking "as-func-mid" 
Checking "as-func-last" 
Checking "as-func-everywhere" 
Checking "as-drop-first" 
Checking "as-drop-last" 
Checking "as-drop-everywhere" 
Checking "as-select-first" 
Checking "as-select-mid1" 
Checking "as-select-mid2" 
Checking "as-select-last" 
Checking "as-select-everywhere" 
Checking "as-block-first" 
Checking "as-block-mid" 
Checking "as-block-last" 
Checking "as-block-everywhere" 
Checking "as-loop-first" 
Checking "as-loop-mid" 
Checking "as-loop-last" 
Checking "as-loop-everywhere" 
Checking "as-if-condition" 
Checking "as-if-then" 
Checking "as-if-else" 
Checking "as-br-first" 
Checking "as-br-last" 
Checking "as-br-everywhere" 
Checking "as-br_if-first" 
Checking "as-br_if-mid" 
Checking "as-br_if-last" 
Checking "as-br_if-everywhere" 
Checking "as-br_table-first" 
Checking "as-br_table-mid" 
Checking "as-br_table-last" 
Checking "as-br_table-everywhere" 
Checking "as-return-first" 
Checking "as-return-last" 
Checking "as-return-everywhere" 
Checking "as-call-first" 
Checking "as-call-mid1" 
Checking "as-call-mid2" 
Checking "as-call-last" 
Checking "as-call-everywhere" 
Checking "as-unary-first" 
Checking "as-unary-last" 
Checking "as-unary-everywhere" 
Checking "as-binary-first" 
Checking "as-binary-mid" 
Checking "as-binary-last" 
Checking "as-binary-everywhere" 
Checking "as-test-first" 
Checking "as-test-last" 
Checking "as-test-everywhere" 
Checking "as-compare-first" 
thread 'main' panicked at src/runtime/typecheck.rs:277:31:
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
@@@ Compiling "test-suite/test/core/return.0.wasm"
Checking
Checking "type-i32" 
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
@@@ Compiling "test-suite/test/core/stack.0.wasm"
Checking "fac-expr" 
thread 'main' panicked at src/runtime/typecheck.rs:287:31:
not yet implemented
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/start.wast
```bash
Checking
 ERROR wasp::testsuite > test 0/20 did not fail invalidating/parsing, expected error: "unknown function" (module: "test-suite/test/core/start.0.wasm")
```

## Failed: test-suite/test/core/switch.wast
```bash
@@@ Compiling "test-suite/test/core/switch.0.wasm"
Checking "stmt" 
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
@@@ Compiling "test-suite/test/core/table.0.wasm"
@@@ Compiling "test-suite/test/core/table.1.wasm"
@@@ Compiling "test-suite/test/core/table.2.wasm"
@@@ Compiling "test-suite/test/core/table.3.wasm"
@@@ Compiling "test-suite/test/core/table.4.wasm"
@@@ Compiling "test-suite/test/core/table.5.wasm"
@@@ Compiling "test-suite/test/core/table.6.wasm"
@@@ Compiling "test-suite/test/core/table.7.wasm"
@@@ Compiling "test-suite/test/core/table.8.wasm"
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
@@@ Compiling "test-suite/test/core/table_size.0.wasm"
thread 'main' panicked at src/parser/instr.rs:645:24:
not yet implemented: 0xfc 16
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/traps.wast
```bash
@@@ Compiling "test-suite/test/core/traps.0.wasm"
Checking "no_dce.i32.div_s" 
thread 'main' panicked at src/runtime/typecheck.rs:315:34:
not yet implemented
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/unreachable.wast
```bash
@@@ Compiling "test-suite/test/core/unreachable.0.wasm"
Checking
Checking
Checking "type-i32" 
Checking "type-i64" 
Checking "type-f32" 
Checking "type-f64" 
Checking "as-func-first" 
Checking "as-func-mid" 
Checking "as-func-last" 
Checking "as-func-value" 
Checking "as-block-first" 
Checking "as-block-mid" 
Checking "as-block-last" 
Checking "as-block-value" 
Checking "as-block-broke" 
Checking "as-loop-first" 
Checking "as-loop-mid" 
Checking "as-loop-last" 
Checking "as-loop-broke" 
Checking "as-br-value" 
Checking "as-br_if-cond" 
Checking "as-br_if-value" 
Checking "as-br_if-value-cond" 
Checking "as-br_table-index" 
Checking "as-br_table-value" 
Checking "as-br_table-value-2" 
Checking "as-br_table-value-index" 
Checking "as-br_table-value-and-index" 
Checking "as-return-value" 
Checking "as-if-cond" 
Checking "as-if-then" 
thread 'main' panicked at src/testsuite.rs:302:50:
failed to load module: type error: IfElseTypeMismatch([], [Num(I32)])
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/unreached-invalid.wast
```bash
Checking
 ERROR wasp::testsuite > test 0/118 did not fail invalidating/parsing, expected error: "unknown local" (module: "test-suite/test/core/unreached-invalid.0.wasm")
```

## Failed: test-suite/test/core/unreached-valid.wast
```bash
@@@ Compiling "test-suite/test/core/unreached-valid.0.wasm"
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
@@@ Compiling "test-suite/test/core/unwind.0.wasm"
Checking "func-unwind-by-unreachable" 
Checking "func-unwind-by-br" 
Checking "func-unwind-by-br-value" 
Checking "func-unwind-by-br_if" 
Checking "func-unwind-by-br_if-value" 
Checking "func-unwind-by-br_table" 
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

