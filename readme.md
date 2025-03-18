# W.A.S.P
This is a highly experimental WebAssembly interpreter.
It is not suitable for usage yet.
## Latest spec test (typechecking currently disabled)


💅: 43\
💩: 104
## Failed: test-suite/test/core/align.wast
```bash
ERROR src/testsuite.rs:445: test 108/162 failed (module: "test-suite/test/core/align.106.wasm", invoke: "f32_align_switch", error: wrong type popped from stack (got f32, expected u32): src/runtime/methods/step.rs:394:29)
```

## Failed: test-suite/test/core/br_if.wast
```bash
ERROR src/testsuite.rs:440: test 10/118 failed (module: "test-suite/test/core/br_if.0.wasm", invoke: "as-block-first", got [--- BLOCK ---, i32(2)], but expected [i32(2)])
```

## Failed: test-suite/test/core/br_table.wast
```bash
ERROR src/testsuite.rs:440: test 23/174 failed (module: "test-suite/test/core/br_table.0.wasm", invoke: "singleton", got [--- BLOCK ---, i32(20)], but expected [i32(20)])
```

## Failed: test-suite/test/core/comments.wast
```bash
ERROR src/testsuite.rs:440: test 6/8 failed (module: "test-suite/test/core/comments.4.wasm", invoke: "f1", got [i32(1), i32(2)], but expected [i32(2)])
```

## Failed: test-suite/test/core/elem.wast
```bash
ERROR src/testsuite.rs:331: oops the test-suite panicked!
Reason:
	panicked at src/runtime/clean_model.rs:699:31:
	not yet implemented
Last test (1):
	Module(Module { _type: MustBe!("module"), _name: None, filename: "elem.0.wasm" })
```

## Failed: test-suite/test/core/endianness.wast
```bash
ERROR src/testsuite.rs:440: test 2/69 failed (module: "test-suite/test/core/endianness.0.wasm", invoke: "i32_load16_s", got [i32(65535)], but expected [i32(-1)])
```

## Failed: test-suite/test/core/exports.wast
```bash
ERROR src/testsuite.rs:331: oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:272:17:
	not yet implemented
Last test (14):
	AssertReturn(AssertReturn { _type: MustBe!("assert_return"), action: Invoke { module: Some("$Func"), field: "e", args: [I32 { value: "42" }] }, expected: [I32 { value: "43" }] })
```

## Failed: test-suite/test/core/fac.wast
```bash
ERROR src/testsuite.rs:440: test 3/8 failed (module: "test-suite/test/core/fac.0.wasm", invoke: "fac-iter", got [i64(1)], but expected [i64(7034535277573963776)])
```

## Failed: test-suite/test/core/float_exprs.wast
```bash
ERROR src/testsuite.rs:440: test 332/927 failed (module: "test-suite/test/core/float_exprs.39.wasm", invoke: "no_demote_mixed_mul", got [i32(0)], but expected [i32(329178166)])
```

## Failed: test-suite/test/core/func.wast
```bash
ERROR src/testsuite.rs:440: test 57/172 failed (module: "test-suite/test/core/func.0.wasm", invoke: "break-br_if-empty", got [i32(2)], but expected [])
```

## Failed: test-suite/test/core/func_ptrs.wast
```bash
83
ERROR src/testsuite.rs:563: test 6/36 did not fail invalidating/parsing, expected error: "unknown table" (module: "test-suite/test/core/func_ptrs.1.wasm")
```

## Failed: test-suite/test/core/global.wast
```bash
ERROR src/testsuite.rs:331: oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:407:45:
	failed to load module: GlobalWithoutOffset
Last test (1):
	Module(Module { _type: MustBe!("module"), _name: None, filename: "global.0.wasm" })
```

## Failed: test-suite/test/core/if.wast
```bash
ERROR src/testsuite.rs:440: test 38/241 failed (module: "test-suite/test/core/if.0.wasm", invoke: "as-br_if-last", got [--- BLOCK ---, i32(2), i32(3)], but expected [i32(3)])
```

## Failed: test-suite/test/core/imports.wast
```bash
ERROR src/testsuite.rs:445: test 4/178 failed (module: "test-suite/test/core/imports.1.wasm", invoke: "print32", error: wrong type popped from stack (got f32, expected i32): src/runtime/methods/step.rs:1393:25)
```

## Failed: test-suite/test/core/labels.wast
```bash
ERROR src/testsuite.rs:440: test 3/29 failed (module: "test-suite/test/core/labels.0.wasm", invoke: "loop1", got [i32(1)], but expected [i32(5)])
```

## Failed: test-suite/test/core/linking.wast
```bash
ERROR src/testsuite.rs:331: oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:648:21:
	not yet implemented: Register: "Mf" $Mf
Last test (2):
	Register(Register { _type: MustBe!("register"), name: Some("$Mf"), _as: "Mf" })
```

## Failed: test-suite/test/core/local_get.wast
```bash
ERROR src/testsuite.rs:440: test 15/36 failed (module: "test-suite/test/core/local_get.0.wasm", invoke: "as-br_table-value", got [--- BLOCK ---, i32(2)], but expected [i32(2)])
```

## Failed: test-suite/test/core/local_set.wast
```bash
ERROR src/testsuite.rs:445: test 20/53 failed (module: "test-suite/test/core/local_set.0.wasm", invoke: "write", error: wrong type popped from stack (got f32, expected u64): src/runtime/methods/step.rs:1431:25)
```

## Failed: test-suite/test/core/local_tee.wast
```bash
ERROR src/testsuite.rs:445: test 55/97 failed (module: "test-suite/test/core/local_tee.0.wasm", invoke: "write", error: wrong type popped from stack (got f32, expected u64): src/runtime/methods/step.rs:1431:25)
```

## Failed: test-suite/test/core/loop.wast
```bash
ERROR src/testsuite.rs:440: test 40/120 failed (module: "test-suite/test/core/loop.0.wasm", invoke: "break-multi-value", got [i32(0), i32(0), i64(0)], but expected [i32(18), i32(-18), i64(18)])
```

## Failed: test-suite/test/core/memory.wast
```bash
ERROR src/testsuite.rs:440: test 36/88 failed (module: "test-suite/test/core/memory.30.wasm", invoke: "cast", got [--- BLOCK ---, i64(0)], but expected [i64(4611686018427387904)])
```

## Failed: test-suite/test/core/memory_copy.wast
```bash
ERROR src/testsuite.rs:331: oops the test-suite panicked!
Reason:
	panicked at src/runtime/memory.rs:157:25:
	attempt to add with overflow
Last test (3419):
	AssertTrap(AssertTrap { _type: MustBe!("assert_trap"), action: Invoke { module: None, field: "run", args: [I32 { value: "0" }, I32 { value: "65516" }, I32 { value: "4294963200" }] }, text: "out of bounds memory access" })
```

## Failed: test-suite/test/core/memory_fill.wast
```bash
ERROR src/testsuite.rs:440: test 3/100 failed (module: "test-suite/test/core/memory_fill.0.wasm", invoke: "checkRange", got [--- BLOCK ---, --- BLOCK ---, i32(-1)], but expected [i32(-1)])
```

## Failed: test-suite/test/core/memory_grow.wast
```bash
ERROR src/testsuite.rs:440: test 2/104 failed (module: "test-suite/test/core/memory_grow.0.wasm", invoke: "size", got [i32(65535)], but expected [i32(0)])
```

## Failed: test-suite/test/core/memory_init.wast
```bash
ERROR src/testsuite.rs:563: test 130/240 did not fail invalidating/parsing, expected error: "unknown data segment" (module: "test-suite/test/core/memory_init.5.wasm")
```

## Failed: test-suite/test/core/memory_size.wast
```bash
ERROR src/testsuite.rs:440: test 2/42 failed (module: "test-suite/test/core/memory_size.0.wasm", invoke: "size", got [i32(65535)], but expected [i32(0)])
```

## Failed: test-suite/test/core/memory_trap.wast
```bash
ERROR src/testsuite.rs:445: test 2/182 failed (module: "test-suite/test/core/memory_trap.0.wasm", invoke: "store", error: out of bounds memory access)
```

## Failed: test-suite/test/core/ref_func.wast
```bash
ERROR src/testsuite.rs:331: oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:407:45:
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
ERROR src/testsuite.rs:331: oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:362:6:
	failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 5, column: 2)
Last test (1):
	Default
```

## Failed: test-suite/test/core/ref_null.wast
```bash
ERROR src/testsuite.rs:331: oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:362:6:
	failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 5, column: 154)
Last test (1):
	Default
```

## Failed: test-suite/test/core/return.wast
```bash
ERROR src/testsuite.rs:440: test 6/84 failed (module: "test-suite/test/core/return.0.wasm", invoke: "type-i32-value", got [--- BLOCK ---, i32(1)], but expected [i32(1)])
```

## Failed: test-suite/test/core/select.wast
```bash
ERROR src/testsuite.rs:331: oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:362:6:
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

## Failed: test-suite/test/core/stack.wast
```bash
ERROR src/testsuite.rs:440: test 2/7 failed (module: "test-suite/test/core/stack.0.wasm", invoke: "fac-expr", got [i64(1)], but expected [i64(7034535277573963776)])
```

## Failed: test-suite/test/core/switch.wast
```bash
ERROR src/testsuite.rs:440: test 2/28 failed (module: "test-suite/test/core/switch.0.wasm", invoke: "stmt", got [--- BLOCK ---, --- BLOCK ---, --- BLOCK ---, --- BLOCK ---, --- BLOCK ---, --- BLOCK ---, --- BLOCK ---, --- BLOCK ---, --- BLOCK ---, i32(0)], but expected [i32(0)])
```

## Failed: test-suite/test/core/table.wast
```bash
memory allocation of 77309411344 bytes failed
```

## Failed: test-suite/test/core/table_copy.wast
```bash
ERROR src/testsuite.rs:466: test 66/1728 failed: out of bounds table access (module: "test-suite/test/core/table_copy.2.wasm", invoke: "test")
```

## Failed: test-suite/test/core/table_fill.wast
```bash
ERROR src/testsuite.rs:331: oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:407:45:
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
ERROR src/testsuite.rs:331: oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:362:6:
	failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 8, column: 2)
Last test (1):
	Default
```

## Failed: test-suite/test/core/table_grow.wast
```bash
ERROR src/testsuite.rs:331: oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:362:6:
	failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 44, column: 2)
Last test (1):
	Default
```

## Failed: test-suite/test/core/table_init.wast
```bash
ERROR src/testsuite.rs:466: test 68/780 failed: out of bounds table access (module: "test-suite/test/core/table_init.3.wasm", invoke: "test")
```

## Failed: test-suite/test/core/table_set.wast
```bash
ERROR src/testsuite.rs:331: oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:362:6:
	failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 10, column: 2)
Last test (1):
	Default
```

## Failed: test-suite/test/core/table_size.wast
```bash
ERROR src/testsuite.rs:331: oops the test-suite panicked!
Reason:
	panicked at src/runtime/methods/step.rs:1763:17:
	not implemented: instruction not supported : xfc_16_table_size(TableIdX(0))
Last test (2):
	AssertReturn(AssertReturn { _type: MustBe!("assert_return"), action: Invoke { module: None, field: "size-t0", args: [] }, expected: [I32 { value: "0" }] })
```

## Failed: test-suite/test/core/unreachable.wast
```bash
ERROR src/testsuite.rs:507: test 34/64 did not fail, expected error: "unreachable" (module: "test-suite/test/core/unreachable.0.wasm", function "as-if-then-no-else")
```

## Failed: test-suite/test/core/unreached-invalid.wast
```bash
ERROR src/testsuite.rs:563: test 1/118 did not fail invalidating/parsing, expected error: "unknown local" (module: "test-suite/test/core/unreached-invalid.0.wasm")
```

## Failed: test-suite/test/core/unreached-valid.wast
```bash
ERROR src/testsuite.rs:331: oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:407:45:
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
ERROR src/testsuite.rs:440: test 3/50 failed (module: "test-suite/test/core/unwind.0.wasm", invoke: "func-unwind-by-br", got [i32(3), i64(1)], but expected [])
```

## Failed: test-suite/test/core/utf8-custom-section-id.wast
```bash
ERROR src/testsuite.rs:563: test 1/176 did not fail invalidating/parsing, expected error: "malformed UTF-8 encoding" (module: "test-suite/test/core/utf8-custom-section-id.0.wasm")
```

# Opinionated order of tests
Beware that this list might miss a test or two
1. ✅ binary
2. ❌ func
3. ❌ memory
4. ❌ table
5. ✅ i32
6. ✅ i64
7. ✅ f32
8. ✅ f64
9. ✅ call
10. ✅ call_indirect
11. ✅ block
12. ❌ if
13. ❌ loop
14. ✅ br
15. ❌ br_if
16. ❌ br_table
17. ❌ global
18. ❌ local_get
19. ❌ local_set
20. ❌ local_tee
21. ❌ stack
22. ❌ imports
23. ❌ exports
24. ✅ start
25. ✅ data
26. ❌ elem
27. ✅ type
28. ✅ address
29. ✅ load
30. ✅ store
31. ✅ const
32. ✅ conversions
33. ❌ return
34. ❌ select
35. ✅ traps
36. ❌ ref_func
37. ❌ ref_null
38. ❌ ref_is_null
39. ❌ memory_size
40. ❌ memory_grow
41. ❌ memory_init
42. ❌ memory_copy
43. ❌ memory_fill
44. ❌ table_get
45. ❌ table_set
46. ❌ table_size
47. ❌ table_grow
48. ❌ table_init
49. ❌ table_copy
50. ❌ table_fill
51. ❌ align
52. ✅ bulk
53. ✅ int_exprs
54. ✅ int_literals
55. ❌ float_exprs
56. ✅ float_literals
57. ✅ float_memory
58. ✅ float_misc
59. ✅ f32_cmp
60. ✅ f64_cmp
61. ✅ f32_bitwise
62. ✅ f64_bitwise
63. ❌ labels
64. ❌ unreachable
65. ✅ nop
66. ❌ comments
67. ✅ names
68. ❌ linking
69. ✅ custom
70. ❌ endianness
71. ✅ forward
72. ✅ binary-leb128
73. ❌ func_ptrs
74. ✅ left-to-right
75. ✅ memory_redundancy
76. ❌ memory_trap
77. ✅ token
78. ❌ unwind
79. ✅ skip-stack-guard-page
80. ✅ obsolete-keywords
81. ❌ fac
82. ❌ unreached-invalid
83. ❌ unreached-valid
84. ❌ utf8-custom-section-id
85. ✅ utf8-import-field
86. ✅ utf8-import-module
87. ✅ utf8-invalid-encoding
88. ❌ switch
89. ✅ inline-module
90. ❌ simd_load
91. ❌ simd_store
92. ❌ simd_const
93. ❌ simd_lane
94. ❌ simd_conversions
95. ❌ simd_bitwise
96. ❌ simd_boolean
97. ❌ simd_i8x16_arith
98. ❌ simd_i16x8_arith
99. ❌ simd_i32x4_arith
100. ❌ simd_i64x2_arith
101. ❌ simd_f32x4_arith
102. ❌ simd_f64x2_arith
103. ❌ simd_f32x4_cmp
104. ❌ simd_f64x2_cmp
105. ❌ simd_i8x16_cmp
106. ❌ simd_i16x8_cmp
107. ❌ simd_i32x4_cmp
108. ❌ simd_i64x2_cmp
109. ❌ simd_splat
110. ❌ simd_bit_shift
111. ❌ simd_address
112. ❌ simd_align
113. ❌ simd_i8x16_sat_arith
114. ❌ simd_i16x8_sat_arith
115. ❌ simd_i8x16_arith2
116. ❌ simd_i16x8_arith2
117. ❌ simd_i32x4_arith2
118. ❌ simd_i64x2_arith2
119. ❌ simd_f32x4_rounding
120. ❌ simd_f64x2_rounding
121. ❌ simd_f32x4_pmin_pmax
122. ❌ simd_f64x2_pmin_pmax
123. ❌ simd_load_splat
124. ❌ simd_load_extend
125. ❌ simd_load_zero
126. ❌ simd_load8_lane
127. ❌ simd_load16_lane
128. ❌ simd_load32_lane
129. ❌ simd_load64_lane
130. ❌ simd_store8_lane
131. ❌ simd_store16_lane
132. ❌ simd_store32_lane
133. ❌ simd_store64_lane
134. ❌ simd_int_to_int_extend
135. ❌ simd_i16x8_extmul_i8x16
136. ❌ simd_i32x4_extmul_i16x8
137. ❌ simd_i64x2_extmul_i32x4
138. ❌ simd_i16x8_extadd_pairwise_i8x16
139. ❌ simd_i32x4_extadd_pairwise_i16x8
140. ❌ simd_i32x4_dot_i16x8
141. ❌ simd_i16x8_q15mulr_sat_s
142. ❌ simd_i32x4_trunc_sat_f32x4
143. ❌ simd_i32x4_trunc_sat_f64x2
144. ❌ simd_linking
