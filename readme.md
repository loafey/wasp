# W.A.S.P

## Latest spec test (typechecking currently disabled)
💅: 50\
💩: 97
## Failed: test-suite/test/core/elem.wast
```bash
2025-03-15T14:18:48.895642Z ERROR src/testsuite.rs:320: oops the test-suite panicked!
Reason:
	panicked at src/runtime/clean_model.rs:699:31:
	not yet implemented
Last test (1):
	Module(Module { _type: MustBe!("module"), _name: None, filename: "elem.0.wasm" })
```

## Failed: test-suite/test/core/endianness.wast
```bash
2025-03-15T14:18:48.902716Z ERROR src/testsuite.rs:424: test 2/69 failed (module: 0, invoke: "i32_load16_s", got [i32(65535)], but expected [i32(-1)])
```

## Failed: test-suite/test/core/exports.wast
```bash
2025-03-15T14:18:48.911568Z ERROR src/testsuite.rs:320: oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:277:17:
	not yet implemented
Last test (14):
	AssertReturn(AssertReturn { _type: MustBe!("assert_return"), action: Invoke { module: Some("$Func"), field: "e", args: [I32 { value: "42" }] }, expected: [I32 { value: "43" }] })
```

## Failed: test-suite/test/core/fac.wast
```bash
2025-03-15T14:18:48.918560Z ERROR src/testsuite.rs:438: test 7/8 failed (module: 0, invoke: "fac-ssa", error: wrong type popped from stack (got BlockLock, expected i64): src/runtime/methods/step.rs:1002:25)
```

## Failed: test-suite/test/core/float_exprs.wast
```bash
2025-03-15T14:18:48.972401Z ERROR src/testsuite.rs:424: test 332/927 failed (module: 39, invoke: "no_demote_mixed_mul", got [i32(0)], but expected [i32(329178166)])
```

## Failed: test-suite/test/core/func.wast
```bash
2025-03-15T14:18:48.989837Z ERROR src/testsuite.rs:438: test 9/172 failed (module: 0, invoke: "local-first-i32", error: a local is missing: src/runtime/methods/step.rs:545:51)
```

## Failed: test-suite/test/core/func_ptrs.wast
```bash
83
2025-03-15T14:18:48.997963Z ERROR src/testsuite.rs:554: test 6/36 did not fail invalidating/parsing, expected error: "unknown table" (module: "test-suite/test/core/func_ptrs.1.wasm")
```

## Failed: test-suite/test/core/global.wast
```bash
2025-03-15T14:18:49.008938Z ERROR src/testsuite.rs:320: oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:396:45:
	failed to load module: GlobalWithoutOffset
Last test (1):
	Module(Module { _type: MustBe!("module"), _name: None, filename: "global.0.wasm" })
```

## Failed: test-suite/test/core/if.wast
```bash
2025-03-15T14:18:49.034904Z ERROR src/testsuite.rs:424: test 98/241 failed (module: 0, invoke: "params", got [i32(1)], but expected [i32(-1)])
```

## Failed: test-suite/test/core/labels.wast
```bash
2025-03-15T14:18:49.044032Z ERROR src/testsuite.rs:424: test 3/29 failed (module: 0, invoke: "loop1", got [i32(1)], but expected [i32(5)])
```

## Failed: test-suite/test/core/linking.wast
```bash
2025-03-15T14:18:49.055049Z ERROR src/testsuite.rs:320: oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:639:21:
	not yet implemented: Register: "Mf" $Mf
Last test (2):
	Register(Register { _type: MustBe!("register"), name: Some("$Mf"), _as: "Mf" })
```

## Failed: test-suite/test/core/local_get.wast
```bash
2025-03-15T14:18:49.061340Z ERROR src/testsuite.rs:438: test 2/36 failed (module: 0, invoke: "type-local-i32", error: a local is missing: src/runtime/methods/step.rs:545:51)
```

## Failed: test-suite/test/core/local_set.wast
```bash
2025-03-15T14:18:49.068195Z ERROR src/testsuite.rs:438: test 20/53 failed (module: 0, invoke: "write", error: a local is missing: src/runtime/methods/step.rs:545:51)
```

## Failed: test-suite/test/core/local_tee.wast
```bash
2025-03-15T14:18:49.077671Z ERROR src/testsuite.rs:438: test 55/97 failed (module: 0, invoke: "write", error: a local is missing: src/runtime/methods/step.rs:545:51)
```

## Failed: test-suite/test/core/loop.wast
```bash
2025-03-15T14:18:49.087688Z ERROR src/testsuite.rs:424: test 40/120 failed (module: 0, invoke: "break-multi-value", got [i32(0), i32(0), i64(0)], but expected [i32(18), i32(-18), i64(18)])
```

## Failed: test-suite/test/core/memory.wast
```bash
2025-03-15T14:18:49.096837Z ERROR src/testsuite.rs:433: test 36/88 failed (module: 9, invoke: "cast", got [--- BLOCK ---, f64(0)], but expected [i64(4611686018427387904)])
```

## Failed: test-suite/test/core/memory_copy.wast
```bash
2025-03-15T14:18:49.277337Z ERROR src/testsuite.rs:320: oops the test-suite panicked!
Reason:
	panicked at src/runtime/memory.rs:157:25:
	attempt to add with overflow
Last test (3419):
	AssertTrap(AssertTrap { _type: MustBe!("assert_trap"), action: Invoke { module: None, field: "run", args: [I32 { value: "0" }, I32 { value: "65516" }, I32 { value: "4294963200" }] }, text: "out of bounds memory access" })
```

## Failed: test-suite/test/core/memory_fill.wast
```bash
2025-03-15T14:18:49.297932Z ERROR src/testsuite.rs:320: oops the test-suite panicked!
Reason:
	panicked at src/runtime/memory.rs:214:27:
	attempt to add with overflow
Last test (8):
	AssertTrap(AssertTrap { _type: MustBe!("assert_trap"), action: Invoke { module: None, field: "test", args: [] }, text: "out of bounds memory access" })
```

## Failed: test-suite/test/core/memory_grow.wast
```bash
2025-03-15T14:18:49.313887Z ERROR src/testsuite.rs:424: test 2/104 failed (module: 0, invoke: "size", got [i32(65535)], but expected [i32(0)])
```

## Failed: test-suite/test/core/memory_init.wast
```bash
2025-03-15T14:18:49.334439Z ERROR src/testsuite.rs:554: test 130/240 did not fail invalidating/parsing, expected error: "unknown data segment" (module: "test-suite/test/core/memory_init.5.wasm")
```

## Failed: test-suite/test/core/memory_size.wast
```bash
2025-03-15T14:18:49.341803Z ERROR src/testsuite.rs:424: test 2/42 failed (module: 0, invoke: "size", got [i32(65535)], but expected [i32(0)])
```

## Failed: test-suite/test/core/memory_trap.wast
```bash
2025-03-15T14:18:49.354610Z ERROR src/testsuite.rs:438: test 2/182 failed (module: 0, invoke: "store", error: out of bounds memory access)
```

## Failed: test-suite/test/core/ref_func.wast
```bash
2025-03-15T14:18:49.360633Z ERROR src/testsuite.rs:320: oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:396:45:
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
Last test (3):
	Module(Module { _type: MustBe!("module"), _name: None, filename: "ref_func.1.wasm" })
```

## Failed: test-suite/test/core/ref_is_null.wast
```bash
2025-03-15T14:18:49.366777Z ERROR src/testsuite.rs:320: oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:351:6:
	failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 5, column: 2)
Last test (1):
	Default
```

## Failed: test-suite/test/core/ref_null.wast
```bash
2025-03-15T14:18:49.371788Z ERROR src/testsuite.rs:320: oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:351:6:
	failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 5, column: 154)
Last test (1):
	Default
```

## Failed: test-suite/test/core/return.wast
```bash
2025-03-15T14:18:49.381296Z ERROR src/testsuite.rs:433: test 8/84 failed (module: 0, invoke: "type-f32-value", got [--- BLOCK ---, f32(3)], but expected [i32(1073741824)])
```

## Failed: test-suite/test/core/select.wast
```bash
2025-03-15T14:18:49.391092Z ERROR src/testsuite.rs:320: oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:351:6:
	failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 33, column: 2)
Last test (1):
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

## Failed: test-suite/test/core/table.wast
```bash
memory allocation of 77309411344 bytes failed
```

## Failed: test-suite/test/core/table_copy.wast
```bash
2025-03-15T14:18:49.961786Z ERROR src/testsuite.rs:458: test 66/1728 failed: out of bounds table access (module: 2, invoke: "test")
```

## Failed: test-suite/test/core/table_fill.wast
```bash
2025-03-15T14:18:49.969046Z ERROR src/testsuite.rs:320: oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:396:45:
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
Last test (1):
	Module(Module { _type: MustBe!("module"), _name: None, filename: "table_fill.0.wasm" })
```

## Failed: test-suite/test/core/table_get.wast
```bash
2025-03-15T14:18:49.976063Z ERROR src/testsuite.rs:320: oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:351:6:
	failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 8, column: 2)
Last test (1):
	Default
```

## Failed: test-suite/test/core/table_grow.wast
```bash
2025-03-15T14:18:49.983680Z ERROR src/testsuite.rs:320: oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:351:6:
	failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 44, column: 2)
Last test (1):
	Default
```

## Failed: test-suite/test/core/table_init.wast
```bash
2025-03-15T14:18:50.021655Z ERROR src/testsuite.rs:458: test 68/780 failed: out of bounds table access (module: 3, invoke: "test")
```

## Failed: test-suite/test/core/table_set.wast
```bash
2025-03-15T14:18:50.027241Z ERROR src/testsuite.rs:320: oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:351:6:
	failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 10, column: 2)
Last test (1):
	Default
```

## Failed: test-suite/test/core/table_size.wast
```bash
2025-03-15T14:18:50.035198Z ERROR src/testsuite.rs:320: oops the test-suite panicked!
Reason:
	panicked at src/runtime/methods/step.rs:1757:17:
	not implemented: instruction not supported : xfc_16_table_size(TableIdX(0))
Last test (2):
	AssertReturn(AssertReturn { _type: MustBe!("assert_return"), action: Invoke { module: None, field: "size-t0", args: [] }, expected: [I32 { value: "0" }] })
```

## Failed: test-suite/test/core/unreachable.wast
```bash
2025-03-15T14:18:50.045148Z ERROR src/testsuite.rs:498: test 34/64 did not fail, expected error: "unreachable" (module: 0, function "as-if-then-no-else")
```

## Failed: test-suite/test/core/unreached-invalid.wast
```bash
2025-03-15T14:18:50.056890Z ERROR src/testsuite.rs:554: test 1/118 did not fail invalidating/parsing, expected error: "unknown local" (module: "test-suite/test/core/unreached-invalid.0.wasm")
```

## Failed: test-suite/test/core/unreached-valid.wast
```bash
2025-03-15T14:18:50.062467Z ERROR src/testsuite.rs:320: oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:396:45:
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
Last test (1):
	Module(Module { _type: MustBe!("module"), _name: None, filename: "unreached-valid.0.wasm" })
```

## Failed: test-suite/test/core/unwind.wast
```bash
2025-03-15T14:18:50.070270Z ERROR src/testsuite.rs:438: test 3/50 failed (module: 0, invoke: "func-unwind-by-br", error: missing jump label: src/runtime/methods/step.rs:301:26)
```

## Failed: test-suite/test/core/utf8-custom-section-id.wast
```bash
2025-03-15T14:18:50.086827Z ERROR src/testsuite.rs:554: test 1/176 did not fail invalidating/parsing, expected error: "malformed UTF-8 encoding" (module: "test-suite/test/core/utf8-custom-section-id.0.wasm")
```

# Somewhat opinionated order of tests
1. ✅ binary
2. ✅ binary-leb128
3. ✅ address
4. ✅ align
5. ✅ i32
6. ✅ i64
7. ✅ f32
8. ✅ f64
9. ✅ const
10. ✅ int_exprs
11. ❌ float_exprs
12. ✅ load
13. ✅ store
14. ❌ memory
15. ❌ local_get
16. ❌ local_set
17. ❌ local_tee
18. ❌ func
19. ✅ call
20. ❌ return
21. ✅ block
22. ❌ loop
23. ❌ if
24. ✅ br
25. ✅ br_if
26. ✅ br_table
27. ❌ select
28. ✅ traps
29. ❌ unreachable
30. ❌ memory_init
31. ❌ memory_grow
32. ❌ memory_copy
33. ❌ memory_fill
34. ❌ memory_size
35. ❌ memory_trap
36. ✅ memory_redundancy
37. ✅ stack
38. ✅ imports
39. ❌ exports
40. ❌ linking
41. ✅ start
42. ✅ inline-module
43. ✅ call_indirect
44. ❌ func_ptrs
45. ❌ ref_func
46. ❌ ref_is_null
47. ❌ ref_null
48. ❌ unwind
49. ❌ unreached-invalid
50. ❌ unreached-valid
51. ❌ table
52. ❌ table_get
53. ❌ table_set
54. ❌ table_size
55. ❌ table_grow
56. ❌ table_copy
57. ❌ table_fill
58. ❌ select
59. ✅ type
60. ✅ align
61. ❌ func_ptrs
62. ✅ data
63. ❌ elem
64. ✅ names
65. ✅ comments
66. ❌ utf8-custom-section-id
67. ✅ utf8-import-field
68. ✅ utf8-import-module
69. ✅ utf8-invalid-encoding
70. ✅ obsolete-keywords
71. ✅ skip-stack-guard-page
72. ❌ endianness
73. ✅ float_bitwise
74. ✅ float_cmp
75. ✅ int_literals
76. ✅ float_literals
77. ❌ fac
78. ❌ global
79. ✅ forward
80. ❌ labels
81. ✅ left-to-right
82. ✅ nop
83. ❌ ref_is_null
84. ✅ memory_redundancy
