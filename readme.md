# W.A.S.P

## Latest spec test (typechecking currently disabled)
💅: 27\
💩: 120
## Failed: test-suite/test/core/custom.wast
```bash
thread 'main' panicked at src/testsuite.rs:325:50:
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
thread 'main' panicked at src/testsuite.rs:325:50:
failed to load module: ActiveDataWithoutOffset
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/elem.wast
```bash
thread 'main' panicked at src/testsuite.rs:293:6:
failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 71, column: 2)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/endianness.wast
```bash
thread 'main' panicked at src/runtime/typecheck.rs:328:34:
not yet implemented
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/exports.wast
```bash
thread 'main' panicked at src/testsuite.rs:239:17:
not yet implemented
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/f32.wast
```bash
thread 'main' panicked at src/testsuite.rs:325:50:
failed to load module: ParseError(File: "test-suite/test/core/f32.0.wasm"
UnknownInstruction(<95>), bin pos: 149, stack: [
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
thread 'main' panicked at src/testsuite.rs:325:50:
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
thread 'main' panicked at src/testsuite.rs:325:50:
failed to load module: ParseError(File: "test-suite/test/core/f32_cmp.0.wasm"
UnknownInstruction(<5d>), bin pos: 85, stack: [
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
thread 'main' panicked at src/testsuite.rs:325:50:
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
thread 'main' panicked at src/testsuite.rs:325:50:
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
thread 'main' panicked at src/testsuite.rs:325:50:
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
thread 'main' panicked at src/runtime/methods/step.rs:1304:25:
not implemented: instruction not supported : x51_i64_eq
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/float_exprs.wast
```bash
 ERROR wasp::testsuite > test 21/927 failed (module: 2, invoke: "f32.no_fold_add_zero", got [i32(2145386496)], but expected [i32(-738197504)])
```

## Failed: test-suite/test/core/float_misc.wast
```bash
thread 'main' panicked at src/testsuite.rs:325:50:
failed to load module: ParseError(File: "test-suite/test/core/float_misc.0.wasm"
UnknownInstruction(<95>), bin pos: 413, stack: [
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
 ERROR wasp::testsuite > test 9/172 failed (module: 0, invoke: "local-first-i32", error: a local is missing: src/runtime/methods/step.rs:533:59)
```

## Failed: test-suite/test/core/func_ptrs.wast
```bash
 ERROR wasp::testsuite > test 5/36 failed: unknown function: spectest::print_i32 (module: 0, invoke: "four")
```

## Failed: test-suite/test/core/global.wast
```bash
thread 'main' panicked at src/testsuite.rs:325:50:
failed to load module: GlobalWithoutOffset
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/i32.wast
```bash
thread 'main' panicked at src/testsuite.rs:325:50:
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
thread 'main' panicked at src/testsuite.rs:325:50:
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
 ERROR wasp::testsuite > test 98/241 failed (module: 0, invoke: "params", got [i32(1)], but expected [i32(-1)])
```

## Failed: test-suite/test/core/imports.wast
```bash
thread 'main' panicked at src/testsuite.rs:293:6:
failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 5, column: 2)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/int_exprs.wast
```bash
thread 'main' panicked at src/runtime/methods/step.rs:1304:25:
not implemented: instruction not supported : x53_i64_lt_s
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/labels.wast
```bash
thread 'main' panicked at src/runtime/typecheck.rs:324:31:
not yet implemented
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/left-to-right.wast
```bash
thread 'main' panicked at src/testsuite.rs:325:50:
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
thread 'main' panicked at src/testsuite.rs:293:6:
failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 5, column: 2)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/load.wast
```bash
thread 'main' panicked at src/runtime/methods/step.rs:1304:25:
not implemented: instruction not supported : x67_i32_clz
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/local_get.wast
```bash
 ERROR wasp::testsuite > test 2/36 failed (module: 0, invoke: "type-local-i32", error: a local is missing: src/runtime/methods/step.rs:533:59)
```

## Failed: test-suite/test/core/local_set.wast
```bash
 ERROR wasp::testsuite > test 20/53 failed (module: 0, invoke: "write", error: a local is missing: src/runtime/methods/step.rs:533:59)
```

## Failed: test-suite/test/core/local_tee.wast
```bash
thread 'main' panicked at src/runtime/methods/step.rs:1304:25:
not implemented: instruction not supported : x8c_f32_neg
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/loop.wast
```bash
thread 'main' panicked at src/testsuite.rs:325:50:
failed to load module: ParseError(File: "test-suite/test/core/loop.0.wasm"
UnknownInstruction(<5d>), bin pos: 2324, stack: [
    "wasp::parser::instr::Instr",
    "wasp::parser::instr::Instr",
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
 ERROR wasp::testsuite > test 7/88 did not fail invalidating/parsing, expected error: "multiple memories" (module: "test-suite/test/core/memory.6.wasm")
```

## Failed: test-suite/test/core/memory_copy.wast
```bash
thread 'main' panicked at src/runtime/memory.rs:136:25:
attempt to add with overflow
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/memory_fill.wast
```bash
thread 'main' panicked at src/runtime/memory.rs:181:27:
attempt to add with overflow
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/memory_grow.wast
```bash
thread 'main' panicked at src/testsuite.rs:293:6:
failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 94, column: 2)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/memory_init.wast
```bash
 ERROR wasp::testsuite > test 130/240 did not fail invalidating/parsing, expected error: "unknown data segment" (module: "test-suite/test/core/memory_init.5.wasm")
```

## Failed: test-suite/test/core/memory_size.wast
```bash
thread 'main' panicked at src/testsuite.rs:325:50:
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
thread 'main' panicked at src/testsuite.rs:325:50:
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
thread 'main' panicked at src/testsuite.rs:325:50:
failed to load module: type error: UnknownFunction
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/nop.wast
```bash
thread 'main' panicked at src/runtime/methods/step.rs:1304:25:
not implemented: instruction not supported : x4c_i32_le_s
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/ref_func.wast
```bash
thread 'main' panicked at src/testsuite.rs:293:6:
failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 5, column: 2)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/ref_is_null.wast
```bash
thread 'main' panicked at src/testsuite.rs:293:6:
failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 5, column: 2)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/ref_null.wast
```bash
thread 'main' panicked at src/testsuite.rs:293:6:
failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 5, column: 154)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/return.wast
```bash
 ERROR wasp::testsuite > test 8/84 failed (module: 0, invoke: "type-f32-value", got [--- BLOCK ---, f32(3)], but expected [i32(1077936128)])
```

## Failed: test-suite/test/core/select.wast
```bash
thread 'main' panicked at src/testsuite.rs:293:6:
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

## Failed: test-suite/test/core/stack.wast
```bash
thread 'main' panicked at src/runtime/methods/step.rs:1304:25:
not implemented: instruction not supported : x51_i64_eq
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/start.wast
```bash
 ERROR wasp::testsuite > test 1/20 did not fail invalidating/parsing, expected error: "unknown function" (module: "test-suite/test/core/start.0.wasm")
```

## Failed: test-suite/test/core/switch.wast
```bash
thread 'main' panicked at src/runtime/typecheck.rs:323:32:
not yet implemented
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/table.wast
```bash
memory allocation of 77309411344 bytes failed
```

## Failed: test-suite/test/core/table_copy.wast
```bash
thread 'main' panicked at src/testsuite.rs:293:6:
failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 5, column: 2)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/table_fill.wast
```bash
thread 'main' panicked at src/testsuite.rs:325:50:
failed to load module: ParseError(File: "test-suite/test/core/table_fill.0.wasm"
UnknownInstruction(<25>), bin pos: 96, stack: [
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

## Failed: test-suite/test/core/table_get.wast
```bash
thread 'main' panicked at src/testsuite.rs:293:6:
failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 8, column: 2)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/table_grow.wast
```bash
thread 'main' panicked at src/testsuite.rs:293:6:
failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 44, column: 2)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/table_init.wast
```bash
thread 'main' panicked at src/testsuite.rs:293:6:
failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 5, column: 2)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/table_set.wast
```bash
thread 'main' panicked at src/testsuite.rs:293:6:
failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 10, column: 2)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/table_size.wast
```bash
thread 'main' panicked at src/runtime/typecheck.rs:477:41:
not yet implemented
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/traps.wast
```bash
thread 'main' panicked at src/runtime/methods/step.rs:1304:25:
not implemented: instruction not supported : x6d_i32_div_s
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Failed: test-suite/test/core/unreachable.wast
```bash
 ERROR wasp::testsuite > Got error "hit an unreachable code segment: src/runtime/methods/step.rs:281:25", expected error: "unreachable" (module: 0, function "type-i32")
```

## Failed: test-suite/test/core/unreached-invalid.wast
```bash
 ERROR wasp::testsuite > test 1/118 did not fail invalidating/parsing, expected error: "unknown local" (module: "test-suite/test/core/unreached-invalid.0.wasm")
```

## Failed: test-suite/test/core/unreached-valid.wast
```bash
thread 'main' panicked at src/testsuite.rs:325:50:
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
 ERROR wasp::testsuite > Got error "hit an unreachable code segment: src/runtime/methods/step.rs:281:25", expected error: "unreachable" (module: 0, function "func-unwind-by-unreachable")
```

## Failed: test-suite/test/core/utf8-custom-section-id.wast
```bash
 ERROR wasp::testsuite > test 1/176 did not fail invalidating/parsing, expected error: "malformed UTF-8 encoding" (module: "test-suite/test/core/utf8-custom-section-id.0.wasm")
```

## Failed: test-suite/test/core/utf8-import-field.wast
```bash
 ERROR wasp::testsuite > test 1/176 did not fail invalidating/parsing, expected error: "malformed UTF-8 encoding" (module: "test-suite/test/core/utf8-import-field.0.wasm")
```

## Failed: test-suite/test/core/utf8-import-module.wast
```bash
 ERROR wasp::testsuite > test 1/176 did not fail invalidating/parsing, expected error: "malformed UTF-8 encoding" (module: "test-suite/test/core/utf8-import-module.0.wasm")
```

