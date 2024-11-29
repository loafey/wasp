# W.A.S.P

## Latest spec test (typechecking currently disabled)
💅: 32\
💩: 115
## Failed: test-suite/test/core/bulk.wast
```bash
 ERROR wasp::testsuite > test 76/117 got error "undefined element", expected error: "uninitialized element 2" (module: 8, function "call")
```

## Failed: test-suite/test/core/data.wast
```bash
 ERROR wasp::testsuite > test 10/61 did not fail invalidating/parsing, expected error: "unknown global" (module: "test-suite/test/core/data.9.wasm")
```

## Failed: test-suite/test/core/elem.wast
```bash
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/runtime/clean_model.rs:578:34:
	not yet implemented: xd2_ref_func(FuncIdx(0))
Last test (0):
	Module(Module { _type: MustBe!("module"), _name: None, filename: "elem.0.wasm" })
```

## Failed: test-suite/test/core/endianness.wast
```bash
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/runtime/methods/step.rs:1379:17:
	not implemented: instruction not supported : x76_i32_shr_u
Last test (1):
	AssertReturn(AssertReturn { _type: MustBe!("assert_return"), action: Invoke { module: None, field: "i32_load16_s", args: [I32 { value: "4294967295" }] }, expected: [I32 { value: "4294967295" }] })
```

## Failed: test-suite/test/core/exports.wast
```bash
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:240:17:
	not yet implemented
Last test (13):
	AssertReturn(AssertReturn { _type: MustBe!("assert_return"), action: Invoke { module: Some("$Func"), field: "e", args: [I32 { value: "42" }] }, expected: [I32 { value: "43" }] })
```

## Failed: test-suite/test/core/f32.wast
```bash
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:355:32:
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
Last test (0):
	Module(Module { _type: MustBe!("module"), _name: None, filename: "f32.0.wasm" })
```

## Failed: test-suite/test/core/f32_bitwise.wast
```bash
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:355:32:
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
Last test (0):
	Module(Module { _type: MustBe!("module"), _name: None, filename: "f32_bitwise.0.wasm" })
```

## Failed: test-suite/test/core/f32_cmp.wast
```bash
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:355:32:
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
Last test (0):
	Module(Module { _type: MustBe!("module"), _name: None, filename: "f32_cmp.0.wasm" })
```

## Failed: test-suite/test/core/f64.wast
```bash
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:355:32:
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
Last test (0):
	Module(Module { _type: MustBe!("module"), _name: None, filename: "f64.0.wasm" })
```

## Failed: test-suite/test/core/f64_bitwise.wast
```bash
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:355:32:
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
Last test (0):
	Module(Module { _type: MustBe!("module"), _name: None, filename: "f64_bitwise.0.wasm" })
```

## Failed: test-suite/test/core/f64_cmp.wast
```bash
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:355:32:
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
Last test (0):
	Module(Module { _type: MustBe!("module"), _name: None, filename: "f64_cmp.0.wasm" })
```

## Failed: test-suite/test/core/fac.wast
```bash
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/runtime/methods/step.rs:1379:17:
	not implemented: instruction not supported : x51_i64_eq
Last test (1):
	AssertReturn(AssertReturn { _type: MustBe!("assert_return"), action: Invoke { module: None, field: "fac-rec", args: [I64 { value: "25" }] }, expected: [I64 { value: "7034535277573963776" }] })
```

## Failed: test-suite/test/core/float_exprs.wast
```bash
 ERROR wasp::testsuite > test 21/927 failed (module: 2, invoke: "f32.no_fold_add_zero", got [i32(2145386496)], but expected [i32(-738197504)])
```

## Failed: test-suite/test/core/float_misc.wast
```bash
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:355:32:
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
Last test (0):
	Module(Module { _type: MustBe!("module"), _name: None, filename: "float_misc.0.wasm" })
```

## Failed: test-suite/test/core/func.wast
```bash
 ERROR wasp::testsuite > test 9/172 failed (module: 0, invoke: "local-first-i32", error: a local is missing: src/runtime/methods/step.rs:536:51)
```

## Failed: test-suite/test/core/func_ptrs.wast
```bash
83
 ERROR wasp::testsuite > test 6/36 did not fail invalidating/parsing, expected error: "unknown table" (module: "test-suite/test/core/func_ptrs.1.wasm")
```

## Failed: test-suite/test/core/global.wast
```bash
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:355:32:
	failed to load module: GlobalWithoutOffset
Last test (0):
	Module(Module { _type: MustBe!("module"), _name: None, filename: "global.0.wasm" })
```

## Failed: test-suite/test/core/i32.wast
```bash
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:355:32:
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
Last test (0):
	Module(Module { _type: MustBe!("module"), _name: None, filename: "i32.0.wasm" })
```

## Failed: test-suite/test/core/i64.wast
```bash
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:355:32:
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
Last test (0):
	Module(Module { _type: MustBe!("module"), _name: None, filename: "i64.0.wasm" })
```

## Failed: test-suite/test/core/if.wast
```bash
 ERROR wasp::testsuite > test 98/241 failed (module: 0, invoke: "params", got [i32(1)], but expected [i32(-1)])
```

## Failed: test-suite/test/core/int_exprs.wast
```bash
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/runtime/methods/step.rs:1379:17:
	not implemented: instruction not supported : x53_i64_lt_s
Last test (3):
	AssertReturn(AssertReturn { _type: MustBe!("assert_return"), action: Invoke { module: None, field: "i64.no_fold_cmp_s_offset", args: [I64 { value: "9223372036854775807" }, I64 { value: "0" }] }, expected: [I32 { value: "1" }] })
```

## Failed: test-suite/test/core/labels.wast
```bash
 ERROR wasp::testsuite > test 3/29 failed (module: 0, invoke: "loop1", got [i32(1)], but expected [i32(5)])
```

## Failed: test-suite/test/core/left-to-right.wast
```bash
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:355:32:
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
Last test (0):
	Module(Module { _type: MustBe!("module"), _name: None, filename: "left-to-right.0.wasm" })
```

## Failed: test-suite/test/core/linking.wast
```bash
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:564:21:
	not yet implemented: Register: "Mf" $Mf
Last test (1):
	Register(Register { _type: MustBe!("register"), name: Some("$Mf"), _as: "Mf" })
```

## Failed: test-suite/test/core/load.wast
```bash
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/runtime/methods/step.rs:1379:17:
	not implemented: instruction not supported : x67_i32_clz
Last test (31):
	AssertReturn(AssertReturn { _type: MustBe!("assert_return"), action: Invoke { module: None, field: "as-unary-operand", args: [] }, expected: [I32 { value: "32" }] })
```

## Failed: test-suite/test/core/local_get.wast
```bash
 ERROR wasp::testsuite > test 2/36 failed (module: 0, invoke: "type-local-i32", error: a local is missing: src/runtime/methods/step.rs:536:51)
```

## Failed: test-suite/test/core/local_set.wast
```bash
 ERROR wasp::testsuite > test 20/53 failed (module: 0, invoke: "write", error: a local is missing: src/runtime/methods/step.rs:536:51)
```

## Failed: test-suite/test/core/local_tee.wast
```bash
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/runtime/methods/step.rs:1379:17:
	not implemented: instruction not supported : x8c_f32_neg
Last test (45):
	AssertReturn(AssertReturn { _type: MustBe!("assert_return"), action: Invoke { module: None, field: "as-unary-operand", args: [F32 { value: "0" }] }, expected: [F32 { value: "4286640610" }] })
```

## Failed: test-suite/test/core/loop.wast
```bash
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:355:32:
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
Last test (0):
	Module(Module { _type: MustBe!("module"), _name: None, filename: "loop.0.wasm" })
```

## Failed: test-suite/test/core/memory.wast
```bash
 ERROR wasp::testsuite > test 8/88 did not fail invalidating/parsing, expected error: "multiple memories" (module: "test-suite/test/core/memory.7.wasm")
```

## Failed: test-suite/test/core/memory_copy.wast
```bash
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/runtime/memory.rs:146:25:
	attempt to add with overflow
Last test (3418):
	AssertTrap(AssertTrap { _type: MustBe!("assert_trap"), action: Invoke { module: None, field: "run", args: [I32 { value: "0" }, I32 { value: "65516" }, I32 { value: "4294963200" }] }, text: "out of bounds memory access" })
```

## Failed: test-suite/test/core/memory_fill.wast
```bash
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/runtime/memory.rs:191:27:
	attempt to add with overflow
Last test (7):
	AssertTrap(AssertTrap { _type: MustBe!("assert_trap"), action: Invoke { module: None, field: "test", args: [] }, text: "out of bounds memory access" })
```

## Failed: test-suite/test/core/memory_grow.wast
```bash
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:355:32:
	failed to load module: ParseError(File: "test-suite/test/core/memory_grow.0.wasm"
	UnknownInstruction(<3f>), bin pos: 179, stack: [
	    "wasp::parser::instr::Instr",
	    "wasp::parser::expr::Expr",
	    "wasp::parser::func::Func",
	    "wasp::parser::code::Code",
	    "alloc::vec::Vec<wasp::parser::code::Code>",
	    "wasp::parser::codesec::CodeSection",
	    "wasp::parser::module::Module",
	])
Last test (0):
	Module(Module { _type: MustBe!("module"), _name: None, filename: "memory_grow.0.wasm" })
```

## Failed: test-suite/test/core/memory_init.wast
```bash
 ERROR wasp::testsuite > test 130/240 did not fail invalidating/parsing, expected error: "unknown data segment" (module: "test-suite/test/core/memory_init.5.wasm")
```

## Failed: test-suite/test/core/memory_size.wast
```bash
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:355:32:
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
Last test (0):
	Module(Module { _type: MustBe!("module"), _name: None, filename: "memory_size.0.wasm" })
```

## Failed: test-suite/test/core/memory_trap.wast
```bash
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:355:32:
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
Last test (0):
	Module(Module { _type: MustBe!("module"), _name: None, filename: "memory_trap.0.wasm" })
```

## Failed: test-suite/test/core/ref_func.wast
```bash
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:355:32:
	failed to load module: ParseError(File: "test-suite/test/core/ref_func.1.wasm"
	UnknownInstruction(<d1>), bin pos: 220, stack: [
	    "wasp::parser::instr::Instr",
	    "wasp::parser::expr::Expr",
	    "wasp::parser::func::Func",
	    "wasp::parser::code::Code",
	    "alloc::vec::Vec<wasp::parser::code::Code>",
	    "wasp::parser::codesec::CodeSection",
	    "wasp::parser::module::Module",
	])
Last test (2):
	Module(Module { _type: MustBe!("module"), _name: None, filename: "ref_func.1.wasm" })
```

## Failed: test-suite/test/core/ref_is_null.wast
```bash
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:314:6:
	failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 5, column: 2)
Last test (0):
	Default
```

## Failed: test-suite/test/core/ref_null.wast
```bash
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:314:6:
	failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 5, column: 154)
Last test (0):
	Default
```

## Failed: test-suite/test/core/return.wast
```bash
 ERROR wasp::testsuite > test 8/84 failed (module: 0, invoke: "type-f32-value", got [--- BLOCK ---, f32(3)], but expected [i32(1077936128)])
```

## Failed: test-suite/test/core/select.wast
```bash
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:314:6:
	failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 33, column: 2)
Last test (0):
	Default
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
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/runtime/methods/step.rs:1379:17:
	not implemented: instruction not supported : x51_i64_eq
Last test (1):
	AssertReturn(AssertReturn { _type: MustBe!("assert_return"), action: Invoke { module: None, field: "fac-expr", args: [I64 { value: "25" }] }, expected: [I64 { value: "7034535277573963776" }] })
```

## Failed: test-suite/test/core/start.wast
```bash
 ERROR wasp::testsuite > test 1/20 did not fail invalidating/parsing, expected error: "unknown function" (module: "test-suite/test/core/start.0.wasm")
```

## Failed: test-suite/test/core/table.wast
```bash
memory allocation of 77309411344 bytes failed
```

## Failed: test-suite/test/core/table_copy.wast
```bash
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/ptr.rs:18:19:
	already borrowed: BorrowMutError
Last test (65):
	Action(ActionWrap { _type: MustBe!("action"), action: Invoke { module: None, field: "test", args: [] } })
```

## Failed: test-suite/test/core/table_fill.wast
```bash
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:355:32:
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
Last test (0):
	Module(Module { _type: MustBe!("module"), _name: None, filename: "table_fill.0.wasm" })
```

## Failed: test-suite/test/core/table_get.wast
```bash
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:314:6:
	failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 8, column: 2)
Last test (0):
	Default
```

## Failed: test-suite/test/core/table_grow.wast
```bash
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:314:6:
	failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 44, column: 2)
Last test (0):
	Default
```

## Failed: test-suite/test/core/table_init.wast
```bash
 ERROR wasp::testsuite > test 4/780 failed: out of bounds table access (module: 1, invoke: "test")
```

## Failed: test-suite/test/core/table_set.wast
```bash
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:314:6:
	failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 10, column: 2)
Last test (0):
	Default
```

## Failed: test-suite/test/core/table_size.wast
```bash
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/runtime/methods/step.rs:1379:17:
	not implemented: instruction not supported : xfc_16_table_size(TableIdX(0))
Last test (1):
	AssertReturn(AssertReturn { _type: MustBe!("assert_return"), action: Invoke { module: None, field: "size-t0", args: [] }, expected: [I32 { value: "0" }] })
```

## Failed: test-suite/test/core/token.wast
```bash
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:355:32:
	failed to load module: unknown import: src/runtime/clean_model.rs:122:82
Last test (11):
	Module(Module { _type: MustBe!("module"), _name: None, filename: "token.11.wasm" })
```

## Failed: test-suite/test/core/traps.wast
```bash
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/runtime/methods/step.rs:1379:17:
	not implemented: instruction not supported : x6d_i32_div_s
Last test (1):
	AssertTrap(AssertTrap { _type: MustBe!("assert_trap"), action: Invoke { module: None, field: "no_dce.i32.div_s", args: [I32 { value: "1" }, I32 { value: "0" }] }, text: "integer divide by zero" })
```

## Failed: test-suite/test/core/unreachable.wast
```bash
 ERROR wasp::testsuite > test 2/64 got error "hit an unreachable code segment: src/runtime/methods/step.rs:280:17", expected error: "unreachable" (module: 0, function "type-i32")
```

## Failed: test-suite/test/core/unreached-invalid.wast
```bash
 ERROR wasp::testsuite > test 1/118 did not fail invalidating/parsing, expected error: "unknown local" (module: "test-suite/test/core/unreached-invalid.0.wasm")
```

## Failed: test-suite/test/core/unreached-valid.wast
```bash
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:355:32:
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
Last test (0):
	Module(Module { _type: MustBe!("module"), _name: None, filename: "unreached-valid.0.wasm" })
```

## Failed: test-suite/test/core/unwind.wast
```bash
 ERROR wasp::testsuite > test 2/50 got error "hit an unreachable code segment: src/runtime/methods/step.rs:280:17", expected error: "unreachable" (module: 0, function "func-unwind-by-unreachable")
```

## Failed: test-suite/test/core/utf8-custom-section-id.wast
```bash
 ERROR wasp::testsuite > test 1/176 did not fail invalidating/parsing, expected error: "malformed UTF-8 encoding" (module: "test-suite/test/core/utf8-custom-section-id.0.wasm")
```

